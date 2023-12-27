use async_graphql::MergedObject;

use self::{
    bucket::BucketMutations, experiment::ExperimentMutations, model::ModelMutations,
    model_version::ModelVersionMutations, namespace::NamespaceMutations,
};
pub mod bucket;
pub mod experiment;
pub mod model;
pub mod model_version;
pub mod namespace;

#[derive(MergedObject, Clone, Default)]
pub struct Mutation(
    NamespaceMutations,
    BucketMutations,
    ModelMutations,
    ModelVersionMutations,
    ExperimentMutations,
);
