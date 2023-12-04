use std::{collections::HashMap, sync::Arc};

use bytes::Bytes;
use flymodel::storage::StorageRole;

use crate::minio::{S3Configuration, S3Storage};

type StorageMap = HashMap<String, Box<dyn StorageProvider>>;

#[derive(serde::Deserialize, Debug)]
pub struct StorageConfig {
    s3: Option<Vec<S3Configuration>>,
}

pub struct StorageOrchestrator {
    storage: Arc<StorageMap>,
}

impl StorageOrchestrator {
    pub fn get(&self, store: &String) -> Option<&Box<dyn StorageProvider>> {
        self.storage.get(store)
    }

    pub async fn setup(&self) -> anyhow::Result<()> {
        for store in self.storage.as_ref().values() {
            store.setup().await?;
        }
        Ok(())
    }
}
impl StorageConfig {
    pub async fn build(self) -> anyhow::Result<StorageOrchestrator> {
        let mut storage = StorageMap::new();
        if let Some(confmap) = self.s3 {
            for config in confmap {
                storage.insert(
                    config.bucket.clone(),
                    Box::new(S3Storage::new(config).await?),
                );
            }
        } else {
            anyhow::bail!("no storage provided")
        }
        Ok(StorageOrchestrator {
            storage: Arc::new(storage),
        })
    }
}

#[async_trait::async_trait]
pub trait StorageProvider {
    fn role(&self) -> StorageRole;

    async fn setup(&self) -> anyhow::Result<()>;

    fn prefix(&self) -> String;
    fn resolve_path(&self, path: String) -> String {
        let pre = self.prefix();
        if pre.ends_with('/') {
            return pre + &path;
        }
        pre + "/" + &path
    }

    async fn put(&self, path: String, bs: bytes::Bytes) -> anyhow::Result<Option<String>>;
    async fn get(&self, path: String, version_id: Option<String>) -> anyhow::Result<Bytes>;
}
