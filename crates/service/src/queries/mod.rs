use async_graphql::MergedObject;
pub mod bucket;
use self::bucket::BucketQueries;
use flymodel_entities::entities;

#[derive(Clone, Default, MergedObject)]
pub struct Query(BucketQueries);
