use bytes::Bytes;

use crate::{errs::FlymodelResult, lifecycle::Lifecycle};

#[async_trait::async_trait]
pub trait StorageProvider {
    fn role(&self) -> Lifecycle;

    async fn setup(&self) -> FlymodelResult<()>;

    fn prefix(&self) -> String;
    fn resolve_path(&self, path: String) -> String {
        let pre = self.prefix();
        if pre.ends_with('/') {
            return pre + &path;
        }
        pre + "/" + &path
    }

    async fn put(&self, path: String, bs: Bytes) -> FlymodelResult<Option<String>>;
    async fn del(&self, path: String, version_id: Option<String>) -> FlymodelResult<()>;
    async fn get(&self, path: String, version_id: Option<String>) -> FlymodelResult<Bytes>;
}
