use async_graphql::Object;
use flymodel_entities::{entities, prelude::*};

#[derive(Clone, Default)]
pub struct BucketQueries;

#[Object]
impl BucketQueries {
    async fn list_buckets(&self, namespace: u32) -> Vec<entities::bucket::Model> {
        vec![]
    }

    async fn bucket(&self, id: Option<String>) -> entities::bucket::Model {
        entities::bucket::Model {
            id: 0,
            namespace: 0,
            name: "".to_string(),
            region: "".to_string(),
            role: entities::enums::Lifecycle::Test,
            shard: 0,
            created_at: Default::default(),
            last_modified: Default::default(),
        }
    }
}
