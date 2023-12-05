use std::fmt::Display;

// use crate::schema::{build_schema, };
use actix_web::{
    dev::Service,
    guard,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Result,
};

use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql_actix_web::{GraphQL, GraphQLSubscription};
use flymodel_tracing::tracer::{OtlpTracer, OtlpTracerConfig};
use opentelemetry::{
    trace::{FutureExt, SpanKind, TraceContextExt, Tracer},
    Context,
};
use sea_orm::DbConn;
use tracing::info;

use crate::schema::{build_schema, FlymodelSchema};
use tracing_actix_web::TracingLogger;

const SUBSCRIPTION: &str = "/graphql";

async fn graphql_ws(
    schema: web::Data<FlymodelSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

async fn graphql_idx() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint(SUBSCRIPTION)
                .subscription_endpoint(SUBSCRIPTION)
                .finish(),
        ))
}

pub async fn start_server<A>(
    db: DbConn,
    bind: A,
    tracer: Option<OtlpTracerConfig>,
) -> anyhow::Result<()>
where
    A: std::net::ToSocketAddrs + Display,
{
    info!("starting on http://{}", bind);
    let schema = build_schema(db.clone(), None, None, tracer.clone())?;
    let service_tracer = if let Some(tracer) = tracer {
        Some(tracer.new_tracer_provider("flymodel-graphql")?)
    } else {
        None
    };

    HttpServer::new(move || {
        let base = App::new().wrap(TracingLogger::default());

        let base = if let Some(tracer) = service_tracer.clone() {
            base.app_data(Data::new(tracer))
        } else {
            base
        };

        let base = base.wrap_fn(|req, srv| {
            let tracer: Option<&Data<OtlpTracer>> = req.app_data();
            if let Some(tracer) = tracer {
                let tracer = tracer.tracer.clone();
                srv.call(req)
                    .with_context(opentelemetry::Context::current_with_span(
                        tracer
                            .span_builder("request")
                            .with_kind(SpanKind::Server)
                            .start(&tracer),
                    ))
            } else {
                srv.call(req).with_context(Context::current())
            }
        });

        base.service(
            web::resource(SUBSCRIPTION)
                .guard(guard::Post())
                .app_data(Data::new(schema.clone()))
                .to(GraphQL::new(schema.clone())),
        )
        .service(
            web::resource(SUBSCRIPTION)
                .guard(guard::Get())
                .guard(guard::Header("upgrade", "websocket"))
                .app_data(Data::new(schema.clone()))
                .to(graphql_ws),
        )
        .service(web::resource("/").guard(guard::Get()).to(graphql_idx))
    })
    .bind(bind)?
    .run()
    .await
    .map_err(|e| anyhow::anyhow!(e))
}
