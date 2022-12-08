use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use sqlx::ConnectOptions;
use std::env;
use std::str::FromStr;
use std::time::Duration;

pub mod constants;
pub mod models;
pub mod queries;
pub mod sql;
pub mod util;

pub const NUM_DB_CONNECTIONS: u32 = 4;

pub fn init_env() {
    // Enabled in debug mode
    if cfg!(debug_assertions) {
        dotenv::from_filename("../.env.localhost").ok();
    }
    // ignore the logger error after its first invocation
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init();
}

pub async fn init_db_pool() -> anyhow::Result<PgPool> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    println!("{}", database_url);
    let mut connection_options = PgConnectOptions::from_str(&database_url)?;
    connection_options
        .disable_statement_logging()
        .log_slow_statements(log::LevelFilter::Error, Duration::from_millis(500));

    let db_pool = PgPoolOptions::new()
        .min_connections(NUM_DB_CONNECTIONS)
        .max_connections(NUM_DB_CONNECTIONS)
        .connect_with(connection_options)
        .await?;

    // Auto-migrate db
    sqlx::migrate!("./migrations").run(&db_pool).await?;
    Ok(db_pool)
}

pub async fn init_test_db_pool() -> anyhow::Result<PgPool> {
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL is not set");
    let db_pool = PgPool::connect(&database_url).await?;

    Ok(db_pool)
}
