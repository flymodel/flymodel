use async_graphql::MergedObject;
pub mod bucket;
pub mod namespace;
use self::{bucket::BucketQueries, namespace::NamespaceQueries};

#[derive(Clone, Default, MergedObject)]
pub struct Query(BucketQueries, NamespaceQueries);
