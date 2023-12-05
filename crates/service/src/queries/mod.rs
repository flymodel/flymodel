use async_graphql::MergedObject;
pub mod bucket;
pub mod model;
pub mod namespace;
use self::{bucket::BucketQueries, model::ModelQueries, namespace::NamespaceQueries};

#[derive(Clone, Default, MergedObject)]
pub struct Query(BucketQueries, NamespaceQueries, ModelQueries);
