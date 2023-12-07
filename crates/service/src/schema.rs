use crate::{db::DbLoader, mutations::Mutation, queries::Query};
use async_graphql::{
    extensions::{OpenTelemetry, Tracing},
    EmptySubscription, Schema,
};
use flymodel_entities::entities::{self};
use flymodel_tracing::tracer::OtlpTracerConfig;
use sea_orm::DbConn;
use tracing::debug;

pub type FlymodelSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema(
    db: DbConn,
    depth: Option<usize>,
    complexity: Option<usize>,
    tracer: Option<OtlpTracerConfig>,
) -> anyhow::Result<FlymodelSchema> {
    let builder = Schema::build(Default::default(), Default::default(), EmptySubscription)
        .extension(Tracing)
        .enable_federation()
        .enable_subscription_in_federation()
        .data(DbLoader::<entities::bucket::Model>::new(
            db.clone(),
            tracer.clone(),
        ))
        .data(DbLoader::<entities::namespace::Model>::new(
            db.clone(),
            tracer.clone(),
        ))
        .data(DbLoader::<entities::model::Model>::new(
            db.clone(),
            tracer.clone(),
        ))
        .data(DbLoader::<entities::model_artifact::Model>::new(
            db.clone(),
            tracer.clone(),
        ))
        .data(DbLoader::<entities::model_state::Model>::new(
            db.clone(),
            tracer.clone(),
        ))
        .data(DbLoader::<entities::experiment::Model>::new(
            db.clone(),
            tracer.clone(),
        ))
        .data(DbLoader::<entities::experiment_artifact::Model>::new(
            db.clone(),
            tracer.clone(),
        ))
        .data(DbLoader::<entities::model_version::Model>::new(
            db.clone(),
            tracer.clone(),
        ))
        .data(DbLoader::<entities::object_blob::Model>::new(
            db.clone(),
            tracer.clone(),
        ))
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
    let builder = if let Some(tracer) = tracer {
        debug!("tracer");
        let tracer = tracer.new_tracer_provider("flymodel-gql-query")?.tracer;
        builder.extension(OpenTelemetry::new(tracer))
    } else {
        builder
    };
    Ok(builder.finish())
}
