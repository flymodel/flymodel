use std::{fmt::Display, sync::Arc};

use actix_multipart::form::tempfile::TempFileConfig;
// use crate::schema::{build_schema, };
use actix_web::{
    dev::Service,
    guard,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Result,
};

use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql_actix_web::{GraphQL, GraphQLSubscription};
use flymodel::tls::TlsConf;
use flymodel_entities::{db::DbLoader, entities};
use flymodel_registry::storage::StorageOrchestrator;
use flymodel_tracing::tracer::{OtlpTracer, OtlpTracerConfig};
use opentelemetry::{
    trace::{FutureExt, SpanKind, TraceContextExt, Tracer},
    Context,
};
use sea_orm::DbConn;
use tracing::info;

use crate::{
    apply_data,
    artifacts::{
        experiments::{download_experiment_artifact, upload_experiment_artifact},
        model_version::{download_model_version_artifact, upload_model_version_artifact},
    },
    schema::{build_schema, FlymodelSchema},
};
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

pub async fn start_server<
    A,
    P: std::convert::AsRef<std::path::Path> + Clone + Send + Sync + 'static,
>(
    db: DbConn,
    bind: A,
    temp_dir: P,
    tracer: Option<OtlpTracerConfig>,
    store: Arc<StorageOrchestrator>,
    tls: Option<TlsConf>,
    dry: bool,
) -> anyhow::Result<()>
where
    A: std::net::ToSocketAddrs + Display,
{
    std::fs::create_dir_all(temp_dir.clone())?;

    info!("starting on http://{}", bind);
    let schema = build_schema(db.clone(), None, None, tracer.clone())?;
    let service_tracer = if let Some(tracer) = tracer.clone() {
        Some(tracer.new_tracer_provider("flymodel-graphql")?)
    } else {
        None
    };
    let store = store.clone();
    let server = HttpServer::new(move || {
        let temp_dir = temp_dir.clone();
        let store = store.clone();
        let base = App::new()
            .wrap(TracingLogger::default())
            .app_data(TempFileConfig::default().directory(temp_dir))
            .app_data(Data::new(store));

        apply_data! {
            base,
            app_data,
            |e| Data::new(e),
            db,
            tracer
        }

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

        base.service(upload_model_version_artifact)
            .service(upload_experiment_artifact)
            .service(download_model_version_artifact)
            .service(download_experiment_artifact)
            .service(
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
    });

    if let Some(tls) = tls {
        let tls = tls.server_config()?;
        if !dry {
            server
                .bind_rustls_0_22(bind, tls)?
                .run()
                .await
                .map_err(|e| anyhow::anyhow!(e))
        } else {
            Ok(())
        }
    } else {
        if !dry {
            server
                .bind(bind)?
                .run()
                .await
                .map_err(|e| anyhow::anyhow!(e))
        } else {
            Ok(())
        }
    }
}
