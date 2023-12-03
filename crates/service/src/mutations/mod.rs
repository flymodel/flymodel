use async_graphql::MergedObject;

use self::root::RootMutations;
pub mod root;

#[derive(MergedObject, Clone, Default)]
pub struct Mutation(RootMutations);
