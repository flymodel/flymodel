use std::fmt::{Debug, Display};

// use crate::schema::{build_schema, };
use actix_web::{
    guard,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Result,
};
use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql_actix_web::{GraphQL, GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use sea_orm::DbConn;
use tracing::info;

use crate::schema::{build_schema, FlymodelSchema};

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

pub async fn start_server<A>(db: DbConn, bind: A) -> anyhow::Result<()>
where
    A: std::net::ToSocketAddrs + Display,
{
    info!("starting on http://{}", bind);
    let schema = build_schema(db.clone(), None, None);

    HttpServer::new(move || {
        App::new()
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
    })
    .bind(bind)?
    .run()
    .await
    .map_err(|e| anyhow::anyhow!(e))
}
