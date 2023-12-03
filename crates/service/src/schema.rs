use async_graphql::{dataloader::DataLoader, extensions::Tracing, EmptySubscription, Schema};
use flymodel_entities::queries::{self};
use sea_orm::{DatabaseConnection, DbConn};

use crate::{mutations::Mutation, queries::Query};

pub type FlymodelSchema = Schema<Query, Mutation, EmptySubscription>;
pub struct OrmDataloader {
    pub db: DatabaseConnection,
}

pub fn build_schema(db: DbConn, depth: Option<usize>, complexity: Option<usize>) -> FlymodelSchema {
    let ormloader: DataLoader<OrmDataloader> =
        DataLoader::new(OrmDataloader { db: db.clone() }, tokio::spawn);
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
