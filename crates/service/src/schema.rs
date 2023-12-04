use async_graphql::{dataloader::DataLoader, extensions::Tracing, EmptySubscription, Schema};
use sea_orm::{DatabaseConnection, DbConn};

use crate::{
    db::{Database, OrmDataloader},
    mutations::Mutation,
    queries::Query,
};

pub type FlymodelSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema(db: DbConn, depth: Option<usize>, complexity: Option<usize>) -> FlymodelSchema {
    let ormloader: Database = OrmDataloader::new(db.clone());
    let builder = Schema::build(Default::default(), Default::default(), EmptySubscription)
        .extension(Tracing)
        .data(db)
        .data(ormloader);
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
