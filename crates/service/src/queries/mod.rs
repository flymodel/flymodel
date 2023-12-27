use async_graphql::MergedObject;
pub mod bucket;
pub mod experiment;
pub mod model;
pub mod namespace;

use self::{
    bucket::BucketQueries, experiment::ExperimentQueries, model::ModelQueries,
    namespace::NamespaceQueries,
};

#[derive(Clone, Default, MergedObject)]
pub struct Query(
    BucketQueries,
    NamespaceQueries,
    ModelQueries,
    ExperimentQueries,
);
