use std::{collections::HashMap, sync::Arc};

use flymodel::{errs::FlymodelResult, storage::StorageProvider};

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

    pub async fn setup(&self) -> FlymodelResult<()> {
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
                storage.insert(config.bucket.clone(), Box::new(S3Storage::new(config)?));
            }
        } else {
            anyhow::bail!("no storage provided")
        }
        Ok(StorageOrchestrator {
            storage: Arc::new(storage),
        })
    }
}
