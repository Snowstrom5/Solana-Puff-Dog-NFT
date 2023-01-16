use crate::init_db_pool;
use sqlx::PgPool;
use anyhow::Result;

pub async fn init_db() -> PgPool {
    init_db_pool().await.unwrap()
}

pub async fn init_db_result() -> Result<PgPool> {
    init_db_pool().await
}

