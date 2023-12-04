use crate::storage::StorageProvider;
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
    pub role: StorageRole,
    bucket: String,
}

impl S3Storage {
    pub fn new<'a>(conf: S3Configuration) -> anyhow::Result<Self> {
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
    fn role(&self) -> StorageRole {
        self.role.clone()
    }

    fn prefix(&self) -> String {
        self.prefix.clone()
    }

    async fn setup(&self) -> anyhow::Result<()> {
        self.setup_bucket().await
    }

    async fn put(&self, path: String, bs: bytes::Bytes) -> anyhow::Result<Option<String>> {
        let key = self.resolve_path(path);
        trace!("putting object: {}", key);
        Ok(self
            .cli
            .put_object()
            .bucket(self.bucket.clone())
            .key(key)
            .body(ByteStream::from(bs))
            .send()
            .await?
            .version_id)
    }

    async fn get(&self, path: String, version_id: Option<String>) -> anyhow::Result<Bytes> {
        let key = self.resolve_path(path);
        trace!("getting object: {}", key);
        let base = self
            .cli
            .get_object()
            .bucket(self.bucket.clone())
            .key(key)
            .set_version_id(version_id);
        Ok(base.send().await?.body.collect().await?.into_bytes())
    }
}

#[cfg(test)]
mod test {
    use bytes::Bytes;
    use flymodel::storage::StorageRole;

    use super::S3Storage;
    use crate::storage::StorageProvider;

    fn new_minio_test_client() -> S3Storage {
        dotenv::dotenv().ok();
        S3Storage::new(super::S3Configuration {
            endpoint: Some("http://localhost:9000".into()),
            public: false,
            region: Some("ca-local".into()),
            prefix: "".into(),
            bucket: "ml-dev".into(),
            role: StorageRole::Test,
            path_style: true,
        })
        .expect("storage")
    }

    #[tokio::test]
    async fn test_minio_integration() -> anyhow::Result<()> {
        let client = new_minio_test_client();
        client.setup().await?;

        let expect = Bytes::from_static(b"abc");
        client.put("test.txt".into(), expect.clone()).await?;
        let resp = client.get("test.txt".into(), None).await?;
        assert_eq!(resp, expect);

        Ok(())
    }
}
