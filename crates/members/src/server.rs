use std::net::ToSocketAddrs;

use anyhow::Context;
use tracing::info;

use crate::protos::v1::membership::membership_service_server::MembershipServiceServer;

fn default_membership_address() -> String {
    "127.0.0.1:50051".to_string()
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct MembershipConfig {
    #[serde(default = "default_membership_address")]
    pub address: String,
    pub initial_members: Vec<String>,
}

impl Default for MembershipConfig {
    fn default() -> Self {
        Self {
            address: default_membership_address(),
            initial_members: vec![],
        }
    }
}
pub struct Server {
    conf: MembershipConfig,
}

impl Server {
    pub fn new(conf: MembershipConfig) -> Self {
        Server { conf }
    }

    pub async fn serve(&self) -> anyhow::Result<()> {
        info!("starting on http://{}", self.conf.address);
        let service = crate::membership::MembershipService::new(None, None, None, None);
        Ok(tonic::transport::Server::builder()
            .add_service(MembershipServiceServer::new(service))
            .serve(
                self.conf
                    .address
                    .to_socket_addrs()?
                    .next()
                    .context("must have an address")?,
            )
            .await?)
    }
}
