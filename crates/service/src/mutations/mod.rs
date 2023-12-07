use async_graphql::MergedObject;

use self::namespace::NamespaceMutations;
pub mod namespace;

#[derive(MergedObject, Clone, Default)]
pub struct Mutation(NamespaceMutations);
