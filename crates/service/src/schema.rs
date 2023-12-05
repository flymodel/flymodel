use crate::{db::DbLoader, mutations::Mutation, queries::Query};
use async_graphql::{extensions::Tracing, EmptySubscription, Schema};
use flymodel_entities::entities::{self};
use sea_orm::DbConn;

pub type FlymodelSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema(db: DbConn, depth: Option<usize>, complexity: Option<usize>) -> FlymodelSchema {
    let builder = Schema::build(Default::default(), Default::default(), EmptySubscription)
        .extension(Tracing)
        .enable_federation()
        .enable_subscription_in_federation()
        // TODO: otel
        // .extension(OpenTelemetry::new())
        .data(DbLoader::<entities::bucket::Model>::new(db.clone()))
        .data(DbLoader::<entities::namespace::Model>::new(db.clone()))
        .data(DbLoader::<entities::model::Model>::new(db.clone()))
        .data(DbLoader::<entities::model_artifact::Model>::new(db.clone()))
        .data(DbLoader::<entities::model_state::Model>::new(db.clone()))
        .data(DbLoader::<entities::model_version::Model>::new(db.clone()))
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
