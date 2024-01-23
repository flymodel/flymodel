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

#[macro_export]
macro_rules! with_dbs {
    (
        $builder: ident,
        $fn: ident,
        $wrap: expr,
        $db: ident,
        $tracer: ident,
         $($typ: path), + $(,)?) => {
        let $builder = $builder
        $(
            .$fn(
                ($wrap)(DbLoader::<$typ>::new(
                    $db.clone(),
                    $tracer.clone(),
                ))
            )
        )*;
    };
}

#[macro_export]
macro_rules! apply_data {
    ($builder: ident, $fn: ident,
        $wrap: expr,
        $db: ident, $tracer: ident) => {
        $crate::with_dbs! {
            $builder,
            $fn,
            $wrap,
            $db,
            $tracer,
            entities::bucket::Model,
            entities::namespace::Model,
            entities::model::Model,
            entities::model_artifact::Model,
            entities::model_state::Model,
            entities::model_version::Model,
            entities::model_tag::Model,
            entities::model_version_tag::Model,
            entities::experiment::Model,
            entities::experiment_artifact::Model,
            entities::experiment_tag::Model,
            entities::object_blob::Model,
        }
    };
}

pub fn build_schema(
    db: DbConn,
    depth: Option<usize>,
    complexity: Option<usize>,
    tracer: Option<OtlpTracerConfig>,
) -> anyhow::Result<FlymodelSchema> {
    let builder = Schema::build(Default::default(), Default::default(), Default::default())
        .extension(Tracing)
        .enable_federation()
        .enable_subscription_in_federation()
        .data(db.clone());

    apply_data! {
        builder,
        data,
        |e| e,
        db,
        tracer
    }

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
