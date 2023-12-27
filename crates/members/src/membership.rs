use std::{pin::Pin, sync::Arc, time::Duration};

use crate::protos::v1::membership::{members_request::ToEpoch, *};

use moka::future::Cache;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tokio_util::sync::CancellationToken;
use tonic::{Request, Response};
use tracing::{debug, warn};

use self::member::{default_cache, ClientCache};

// 5m
const MIN_INTERVAL: i64 = 300;
// 30m
const MAX_INTERVAL: i64 = 1800;

pub(crate) fn get_next_eviction(now: i64, total_members: i64, last_seen: i64) -> i64 {
    let ofs = now - last_seen;
    ((((total_members as f64) / (ofs as f64)) * (MIN_INTERVAL as f64)).round() as i64)
        .clamp(MIN_INTERVAL, MAX_INTERVAL)
        - ofs
}

#[cfg(test)]
mod test {
    use std::time::{Duration, SystemTime};

    use crate::membership::MIN_INTERVAL;

    use super::get_next_eviction;

    fn test_it(total_members: i64, last_seen: i64) -> f64 {
        let re = get_next_eviction(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("it")
                .as_secs() as i64,
            total_members,
            last_seen,
        );
        println!("{}", re);
        re as f64
    }

    fn test_expect_60s_ago(members: i64, expected: f64) {
        let re = test_it(
            members,
            SystemTime::now()
                .checked_sub(Duration::from_secs(60))
                .expect("it")
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("it")
                .as_secs() as i64,
        );
        assert_eq!(re, expected);
    }

    #[test]
    fn test_min_at_50() {
        test_expect_60s_ago(50, (MIN_INTERVAL - 60) as f64);
    }

    #[test]
    fn test_min_at_250() {
        test_expect_60s_ago(250, 1190.0);
    }

    #[test]
    fn test_min_at_1250() {
        test_expect_60s_ago(1250, 1740.0);
    }
}

pub struct MembershipService {
    allow: member::MemberCache,
    deny: member::MemberCache,
    incarnations: i64,
    watcher: Arc<member::MembershipWatcher>,
}

// impl tonic::client::

mod member {
    use crate::protos::{
        self,
        v1::membership::{
            membership_service_client::MembershipServiceClient, Discoverable, Empty, Readiness,
            Service,
        },
    };

    use super::get_next_eviction;
    use anyhow::bail;

    use std::{
        convert::Infallible,
        str::FromStr,
        sync::{
            atomic::{AtomicI32, AtomicU8, Ordering},
            Arc,
        },
        time::SystemTime,
    };
    use tokio_util::sync::CancellationToken;
    use tonic::transport::Channel;
    use tracing::debug;

    #[derive(thiserror::Error, Debug, Clone)]
    pub enum MemberError {
        #[error("No available peers")]
        NoAvailablePeers,
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Member {
        url: url::Url,

        pub discovery: super::Discoverable,
        pub joined_at: i64,
        pub last_seen: i64,
        pub services: Vec<protos::v1::membership::Service>,

        pub member: MemberClient,
    }

    impl Into<protos::v1::membership::Member> for Member {
        fn into(self) -> protos::v1::membership::Member {
            protos::v1::membership::Member {
                discovery: Some(self.discovery),
                joined_at: self.joined_at,
                last_seen: self.last_seen,
                services: self.services.iter().map(|it| (*it).into()).collect(),
            }
        }
    }

    #[repr(u8)]
    enum ClientStatus {
        Ready = 0,
        InUse = 1,
        Failing = 2,
    }

    impl TryFrom<u8> for ClientStatus {
        type Error = Infallible;
        fn try_from(v: u8) -> Result<Self, Self::Error> {
            Ok(match v {
                0 => ClientStatus::Ready,
                1 => ClientStatus::InUse,
                2 => ClientStatus::Failing,
                _ => unreachable!(),
            })
        }
    }

    #[derive(Clone)]
    pub struct MemberClient {
        // TODO: do we want this to:
        // 1) maintain load balanced connections via a manager
        // 2) create & maintain connections via a raw connection cache
        // 3) maintain address references and re-create clients as needed

        // 1 is simple, yet will not allow for fine-grained control down the road as to
        // the potential configurations with intra-cluster networking

        // 2 is expensive in terms of the size of each client, and we need guaranteed mutability
        // for the client cache.

        // 3 is a bit more complex, and we need to maintain a reference to the address cache.
        // This is a bit of a pain, it seems better for our use cases to destroy and re-create clients based on the
        // address cache. a few things to consider before committing:
        //     - each client will need to use shared tls pools for each endpoint. as such it may be worthwhile to double-down and create a secondary wrapper struct
        client: MembershipServiceClient<Channel>,

        status: Arc<AtomicU8>,

        current_uses: Arc<AtomicI32>,
    }

    impl MemberClient {
        pub fn new(client: MembershipServiceClient<Channel>) -> Self {
            Self {
                client,
                status: Arc::new(AtomicU8::new(ClientStatus::Ready as u8)),
                current_uses: Arc::new(AtomicI32::new(0)),
            }
        }

        pub fn ready(&self) -> bool {
            self.status.load(std::sync::atomic::Ordering::SeqCst) == ClientStatus::Ready as u8
        }

        fn uses(&self) -> i32 {
            self.current_uses.load(std::sync::atomic::Ordering::SeqCst)
        }

        fn exit_usage(&self) -> ClientStatus {
            let uses = self.current_uses.fetch_sub(1, Ordering::SeqCst);
            if !self.ready() && uses == 1 {
                let last = ClientStatus::try_from(self.status.load(Ordering::SeqCst))
                    .expect("internal cast");
                last
            } else {
                ClientStatus::Ready
            }
        }

        pub fn client(&self) -> (MembershipServiceClient<Channel>, fn(&Self)) {
            self.current_uses.fetch_add(1, Ordering::SeqCst);
            (self.client.clone(), |this: &Self| {
                this.exit_usage();
            })
        }
    }

    pub(crate) type MemberCache = super::Cache<String, Member>;
    pub(crate) type ClientCache = super::Cache<Service, MemberClient>;

    pub(crate) fn default_cache() -> MemberCache {
        MemberCache::builder().max_capacity(2048).build()
    }

    pub struct MembershipWatcher {
        allow: MemberCache,
        deny: MemberCache,
        incarnations: i64,
        cancel: CancellationToken,
        clients: ClientCache,
    }
    impl MembershipWatcher {
        pub fn new(
            allow: MemberCache,
            deny: MemberCache,
            cancel: CancellationToken,
            clients: ClientCache,
        ) -> Arc<Self> {
            let this = Arc::new(Self {
                allow,
                deny,
                cancel,
                clients,
                incarnations: 0,
            });
            let rf = this.clone();
            tokio::spawn(rf.check());
            this.clone()
        }

        pub fn replicate(self: Arc<Self>) -> Arc<Self> {
            let incarnations = self.incarnations + 1;
            debug!("incarnation: {}", incarnations);
            Arc::new(Self {
                allow: self.allow.clone(),
                deny: self.deny.clone(),
                clients: self.clients.clone(),
                cancel: self.cancel.clone(),
                incarnations,
            })
        }

        async fn check(self: Arc<Self>) {
            loop {
                let tok = self.as_ref().cancel.clone();
                if tok.is_cancelled() {
                    break;
                }
                let now = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("it")
                    .as_secs() as i64;
                let sz = self.as_ref().allow.weighted_size();
                let mut remove = vec![];
                for (k, v) in self.as_ref().allow.iter() {
                    if get_next_eviction(now, sz as i64, v.last_seen) <= 0
                        || self.as_ref().deny.contains_key(k.as_ref())
                    {
                        remove.push(k.as_ref().to_string());
                    }
                }
            }
        }

        pub async fn with_member(self: Arc<Self>, discovery: Discoverable) -> anyhow::Result<()> {
            if self.deny.contains_key(&discovery.address) {
                bail!("is denied member");
            }
            let endpoint = tonic::transport::Endpoint::from_str(&discovery.address)?;
            let mut client = MembershipServiceClient::connect(endpoint).await?;
            let cli = client.ping(Empty {}).await?;
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs() as i64;
            if cli.get_ref().readiness == <Readiness as Into<i32>>::into(Readiness::Ready) {
                let url = url::Url::parse(&discovery.address)?;
                self.allow
                    .insert(
                        discovery.address.clone(),
                        Member {
                            url,
                            discovery,
                            last_seen: now,
                            joined_at: now,
                            services: cli
                                .get_ref()
                                .services
                                .clone()
                                .iter()
                                .map(|it| Service::try_from(*it).expect("ok"))
                                .collect(),
                            member: MemberClient::new(client),
                        },
                    )
                    .await;

                // self.clients.insert()
            }
            Ok(())
        }

        pub async fn client_for(
            &self,
            service: Service,
        ) -> Result<
            (
                MembershipServiceClient<Channel>,
                for<'a> fn(&'a MemberClient),
            ),
            MemberError,
        > {
            // TODO: we could use an iterator here where we only compare min until finding one at zero, falling back to the next
            // non-zero client.
            if let Some(ref min) = self
                .allow
                .iter()
                .filter_map(|this| {
                    if this.1.services.contains(&service) {
                        Some(this.1)
                    } else {
                        None
                    }
                })
                .min_by_key(|it| it.member.uses())
            {
                Ok(min.member.client())
            } else {
                Err(MemberError::NoAvailablePeers)
            }
        }
    }
}

impl MembershipService {
    pub fn new(
        allow: Option<member::MemberCache>,
        deny: Option<member::MemberCache>,
        clients: Option<member::ClientCache>,
        tok: Option<CancellationToken>,
    ) -> Self {
        let allow = allow.unwrap_or_else(default_cache);
        let deny = deny.unwrap_or_else(default_cache);
        let watcher = member::MembershipWatcher::new(
            allow.clone(),
            deny.clone(),
            tok.unwrap_or_else(CancellationToken::new),
            clients.unwrap_or_else(|| {
                ClientCache::builder()
                    .time_to_live(Duration::from_secs(MAX_INTERVAL as u64))
                    .build()
            }),
        );
        Self {
            allow,
            deny,
            watcher,
            incarnations: 0,
        }
    }

    pub fn replicate(&self) -> Self {
        let incarnations = self.incarnations + 1;
        debug!("incarnation: {}", incarnations);
        Self {
            allow: self.allow.clone(),
            deny: self.deny.clone(),
            watcher: self.watcher.clone().replicate(),
            incarnations,
        }
    }
}

#[tonic::async_trait]
impl membership_service_server::MembershipService for MembershipService {
    async fn ping(&self, _request: Request<Empty>) -> Result<Response<Status>, tonic::Status> {
        Ok(tonic::Response::new(Status {
            readiness: Readiness::Ready.into(),
            services: vec![],
        }))
    }

    async fn check(
        &self,
        _request: Request<Discoverable>,
    ) -> Result<Response<Status>, tonic::Status> {
        Ok(tonic::Response::new(Status {
            readiness: Readiness::Ready.into(),
            services: vec![],
        }))
    }

    type MembersStream = Pin<Box<dyn Stream<Item = Result<Member, tonic::Status>> + Send>>;

    async fn members(
        &self,
        request: Request<MembersRequest>,
    ) -> Result<Response<Self::MembersStream>, tonic::Status> {
        let (tx, rx) = mpsc::channel(128);
        let allow = self.allow.clone();
        tokio::spawn(async move {
            let filt = &request.get_ref().to_epoch;
            for (_, member) in &allow {
                let mut cont: bool = true;
                match filt {
                    Some(ref f) => match f {
                        ToEpoch::After(epoch) => {
                            cont = member.last_seen >= *epoch;
                        }
                        ToEpoch::Before(epoch) => {
                            cont = member.last_seen <= *epoch;
                        }
                    },
                    None => (),
                };
                if cont {
                    tx.send(Ok(member.into())).await.unwrap();
                }
            }
        });
        Ok(Response::new(
            Box::pin(ReceiverStream::new(rx)) as Self::MembersStream
        ))
    }

    async fn allow(
        &self,
        request: Request<DiscoveryList>,
    ) -> Result<Response<Empty>, tonic::Status> {
        let mut todo = vec![];
        for member in &request.get_ref().discovery {
            todo.push(tokio::spawn(
                self.watcher.clone().with_member(member.to_owned()),
            ))
        }
        for fut in futures::future::join_all(todo).await {
            match fut {
                Ok(v) => v.unwrap(),
                Err(e) => {
                    warn!("join error: {}", e);
                    return Err(tonic::Status::new(
                        tonic::Code::Internal,
                        "Unable to process clients",
                    ));
                }
            }
        }
        Ok(tonic::Response::new(Empty {}))
    }

    async fn deny(
        &self,
        _request: Request<DiscoveryList>,
    ) -> Result<Response<Empty>, tonic::Status> {
        Ok(tonic::Response::new(Empty {}))
    }
}
