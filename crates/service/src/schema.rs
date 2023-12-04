use async_graphql::{extensions::Tracing, EmptySubscription, Schema};
use flymodel_entities::entities::{self, namespace};
use sea_orm::DbConn;

use crate::{db::DbLoader, mutations::Mutation, queries::Query};

pub type FlymodelSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema(db: DbConn, depth: Option<usize>, complexity: Option<usize>) -> FlymodelSchema {
    let builder = Schema::build(Default::default(), Default::default(), EmptySubscription)
        .extension(Tracing)
        .data(DbLoader::<entities::bucket::Model>::new(db.clone()))
        .data(DbLoader::<namespace::Entity>::new(db.clone()))
        .data(db);
    let builder = if let Some(depth) = depth {
        builder.limit_depth(depth)
    } else {
        builder
    };
    let builder = if let Some(complexity) = complexity {
        builder.limit_complexity(complexity)
    } else {
        builder
    };
    builder.finish()
}
