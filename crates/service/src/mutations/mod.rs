use async_graphql::MergedObject;

use self::{
    bucket::BucketMutations, model::ModelMutations, model_version::ModelVersionMutations,
    namespace::NamespaceMutations,
};
pub mod bucket;
pub mod model;
pub mod model_version;
pub mod namespace;

#[derive(MergedObject, Clone, Default)]
pub struct Mutation(
    NamespaceMutations,
    BucketMutations,
    ModelMutations,
    ModelVersionMutations,
);
