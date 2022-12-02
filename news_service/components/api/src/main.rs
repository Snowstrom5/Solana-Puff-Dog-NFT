use crate::graphql::{init_graphql_schema, ClimateActionSchema};
use actix_cors::Cors;
use actix_web::{get, guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use db::{init_db_pool, init_env};
use log::info;
use std::env;

pub mod graphql;

pub const N_WORKERS: usize = 4;

async fn index(schema: web::Data<ClimateActionSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[get("/health")]
pub async fn health() -> Result<HttpResponse> {
    //TODO add db check: https://tjmaynes.com/posts/implementing-the-health-check-api-pattern-with-rust/
    Ok(HttpResponse::Ok().body("success".to_string()))
}

async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

fn main() {
    init_env();
    actix_web::rt::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
    })
    .block_on(async { async_main().await })
}

async fn async_main() {
    let db_pool = init_db_pool().await.unwrap();
    let schema = init_graphql_schema(db_pool);

    let host = env::var("API_HOST").expect("HOST is not set");
    let port = env::var("API_PORT").expect("PORT is not set");

    info!("Playground: http://{}:{}/playground", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive()) //TODO: remove
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(schema.clone()))
            .service(health)
            .service(web::resource("/graphql").guard(guard::Post()).to(index))
            .service(
                web::resource("/playground")
                    .guard(guard::Get())
                    .to(index_playground),
            )
    })
    .workers(N_WORKERS)
    .bind(format!("{}:{}", host, port))
    .unwrap_or_else(|_| panic!("Couldn't bind to port {}", port))
    .run()
    .await
    .unwrap();
}
