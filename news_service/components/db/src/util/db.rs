use crate::init_db_pool;
use sqlx::PgPool;

pub async fn init_db() -> PgPool {
    init_db_pool().await.unwrap()
}
