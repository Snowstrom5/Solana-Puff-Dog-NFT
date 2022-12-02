use crate::news_feed::cron_job::cron_job;
use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use chrono::Local;
use db::init_env;
use db::models::news_cron_job::NewsCronJob;
use db::sql::news_cron_job::insert_news_cron_job;
use db::sql::news_cron_job::update_news_cron_job_completed_at;
use db::sql::news_cron_job::update_news_cron_job_error;
use db::util::convert::datetime_to_str;
use db::util::convert::now_utc_datetime;
use db::util::db::init_db;
use log::error;
use log::info;
use sqlx::PgPool;
use sqlx::Pool;
use sqlx::Postgres;
use std::env;

pub mod language;
pub mod news_feed;
pub mod openai;
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
    // Start scheduler on a new thread
    actix_rt::spawn(async move {
        start_scheduler().await;
    });

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

pub async fn start_scheduler() {
    info!("start_scheduler - {:?}", Local::now());
    let db_pool = init_db().await;
    loop {
        // cron job continuous loop
        if let Err(err) = start_cron_job(&db_pool).await {
            error!("start_cron_job failed: {:?}", err);
        }
    }
}

pub async fn start_cron_job(db_pool: &PgPool) -> anyhow::Result<()> {
    let start_datetime = now_utc_datetime();
    let news_cron_job = NewsCronJob {
        started_at: start_datetime.unix_timestamp(),
        started_at_str: datetime_to_str(start_datetime),
        completed_at: None,
        completed_at_str: None,
        error: None,
    };

    let news_cron_job_db = insert_news_cron_job(db_pool, news_cron_job).await?;
    match cron_job(&db_pool).await {
        Ok(_) => {
            let completed_datetime = now_utc_datetime();
            update_news_cron_job_completed_at(
                &db_pool,
                news_cron_job_db.id,
                completed_datetime.unix_timestamp(),
                datetime_to_str(completed_datetime),
            )
            .await?;
        }
        Err(err) => {
            update_news_cron_job_error(&db_pool, news_cron_job_db.id, err.to_string()).await?;
            error!("cron_job failed: {:?}", err);
        }
    }
    Ok(())
}
