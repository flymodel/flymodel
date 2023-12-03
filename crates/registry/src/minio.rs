use crate::storage::StorageProvider;
use anyhow::bail;
use aws_config::{environment::EnvironmentVariableCredentialsProvider, AppName, Region};
use aws_sdk_s3::{
    primitives::ByteStream,
    types::{BucketVersioningStatus, VersioningConfiguration},
    Client,
};
use bytes::Bytes;
use flymodel::storage::StorageRole;
use tracing::trace;

fn default_path() -> String {
    "/".to_string()
}
fn default_pathstyle() -> bool {
    true
}
fn default_public() -> bool {
    false
}

#[derive(serde::Deserialize, Debug)]
pub struct S3Configuration {
    endpoint: Option<String>,
    #[serde(default = "default_public")]
    public: bool,
    region: Option<String>,
    #[serde(default = "default_path")]
    prefix: String,
    pub bucket: String,
    role: StorageRole,
    #[serde(default = "default_pathstyle")]
    path_style: bool,
}

pub struct S3Storage {
    cli: Client,
    prefix: String,
    role: StorageRole,
    bucket: String,
}

impl S3Storage {
    pub async fn new<'a>(conf: S3Configuration) -> anyhow::Result<Self> {
        let mut builder = aws_sdk_s3::config::Builder::new().force_path_style(conf.path_style);
        if !conf.public {
            builder = builder.credentials_provider(EnvironmentVariableCredentialsProvider::new())
        }
        if let Some(endpoint) = conf.endpoint {
            builder = builder.endpoint_url(endpoint)
        }
        if let Some(region) = conf.region {
            builder = builder.region(Region::new(region))
        }
        builder = builder.app_name(AppName::new(format!("FlyModel-v{}", "0.1.0"))?);
        let cli = Client::from_conf(builder.build());
        return Ok(Self {
            cli,
            prefix: conf.prefix,
            role: conf.role,
            bucket: conf.bucket,
        });
    }

    pub async fn setup_bucket(&self) -> anyhow::Result<()> {
        match self
            .cli
            .head_bucket()
            .bucket(self.bucket.clone())
            .send()
            .await
        {
            Ok(_) => {}
            Err(_) => {
                let _ = self
                    .cli
                    .create_bucket()
                    .bucket(self.bucket.clone())
                    .send()
                    .await;
            }
        }
        self.cli
            .head_bucket()
            .bucket(self.bucket.clone())
            .send()
            .await?;

        match self
            .cli
            .get_bucket_versioning()
            .bucket(self.bucket.clone())
            .send()
            .await?
            .status
        {
            None | Some(BucketVersioningStatus::Suspended) => {
                self.cli
                    .put_bucket_versioning()
                    .bucket(self.bucket.clone())
                    .versioning_configuration(
                        VersioningConfiguration::builder()
                            .status(BucketVersioningStatus::Enabled)
                            .build(),
                    )
                    .send()
                    .await?;
            }
            Some(..) => {}
        }

        // let tag = Tag::builder()
        //     .key("role")
        //     .value(self.role.to_string())
        //     .build()?;
        // if !self
        //     .cli
        //     .get_bucket_tagging()
        //     .bucket(self.bucket.clone())
        //     .send()
        //     .await?
        //     .tag_set
        //     .contains(&tag)
        // {
        //     self.cli
        //         .put_bucket_tagging()
        //         .bucket(self.bucket.clone())
        //         .tagging(aws_sdk_s3::types::Tagging::builder().tag_set(tag).build()?);
        // };
        Ok(())
    }
}

#[async_trait::async_trait]
impl StorageProvider for S3Storage {
    fn prefix(&self) -> String {
        self.prefix.clone()
    }

    async fn setup(&self) -> anyhow::Result<()> {
        self.setup_bucket().await
    }

    async fn put(&self, path: String, bs: bytes::Bytes) -> anyhow::Result<()> {
        self.cli
            .put_object()
            .bucket(self.bucket.clone())
            .key(self.resolve_path(path))
            .body(ByteStream::from(bs))
            .send()
            .await?;
        Ok(())
    }
    async fn get(&self, path: String, version: Option<usize>) -> anyhow::Result<Bytes> {
        let key = self.resolve_path(path);
        let base = self
            .cli
            .get_object()
            .bucket(self.bucket.clone())
            .key(key.clone());
        Ok(if let Some(version) = version {
            let ver = self
                .cli
                .list_object_versions()
                .bucket(self.bucket.clone())
                .key_marker(key.clone())
                .send()
                .await?;

            let mut vers = ver.versions.unwrap();
            if vers.len() < version {
                bail!("does not have the version")
            }
            vers.sort_by(|a, b| {
                a.last_modified
                    .expect("datetimes")
                    .cmp(&b.last_modified.expect("dates"))
            });
            trace!("{:#?}", vers);
            base.set_version_id(vers.get(version).expect("it").version_id.clone())
                .send()
                .await?
                .body
                .collect()
                .await?
                .into_bytes()
        } else {
            base.send().await?.body.collect().await?.into_bytes()
        })
    }
}
