use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use db::init_env;
use db::util::db::init_db;
use scheduler::main_scheduler::start_main_scheduler;
use scheduler::tweet_scheduler::start_tweet_scheduler;
use sqlx::Pool;
use sqlx::Postgres;
use std::env;

pub mod language;
pub mod news_feed;
pub mod openai;
pub mod scheduler;
pub mod slack;
pub mod twitter;
pub mod util;

pub struct AppState {
    pub db_pool: Pool<Postgres>,
}

#[get("/health")]
pub async fn health(data: web::Data<AppState>) -> Result<HttpResponse> {
    // TODO move to db component
    let is_database_connected = sqlx::query("SELECT 1")
        .fetch_one(&data.db_pool)
        .await
        .is_ok();
    if is_database_connected {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::json!({ "database_connected": is_database_connected }).to_string()))
    } else {
        Ok(HttpResponse::ServiceUnavailable()
            .content_type("application/json")
            .body(serde_json::json!({ "database_connected": is_database_connected }).to_string()))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_env();
    // Start mainscheduler on a new thread
    actix_rt::spawn(async move {
        start_main_scheduler().await;
    });

    // // Start tweet scheduler on a new thread
    // actix_rt::spawn(async move {
    //     start_tweet_scheduler().await;
    // });

    let host = env::var("CRON_HOST").expect("HOST is not set");
    let port = env::var("CRON_PORT").expect("PORT is not set");

    let db_pool = init_db().await;
    let app_state = web::Data::new(AppState { db_pool: db_pool });

    // Start Web server
    HttpServer::new(move || App::new().app_data(app_state.clone()).service(health))
        .bind(format!("{}:{}", host, port))
        .unwrap_or_else(|_| panic!("Couldn't bind to port {}", port))
        .run()
        .await
}
