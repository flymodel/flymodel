use async_graphql::MergedObject;
pub mod bucket;
pub mod namespace;
use self::{bucket::BucketQueries, namespace::NamespaceQueries};
use flymodel_entities::entities;

#[derive(Clone, Default, MergedObject)]
pub struct Query(BucketQueries, NamespaceQueries);
