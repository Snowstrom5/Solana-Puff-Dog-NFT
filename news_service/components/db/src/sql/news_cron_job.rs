use crate::models::news_cron_job::{NewsCronJob, NewsCronJobWithId};
use anyhow::Result;
use sqlx::PgPool;

pub async fn insert_news_cron_job(
    pool: &PgPool,
    news_cron_job: NewsCronJob,
) -> Result<NewsCronJobWithId, sqlx::Error> {
    sqlx::query_as!(
        NewsCronJobWithId,
        r#"
            INSERT INTO news_cron_job ( 
                started_at, started_at_str, completed_at, completed_at_str, error
             )
            VALUES ( $1, $2, $3, $4, $5)
            RETURNING 
                id, started_at, started_at_str, completed_at, completed_at_str, error
            "#,
        news_cron_job.started_at,
        news_cron_job.started_at_str,
        news_cron_job.completed_at,
        news_cron_job.completed_at_str,
        news_cron_job.error,
    )
    .fetch_one(pool)
    .await
}

pub async fn update_news_cron_job_completed_at(
    pool: &PgPool,
    cron_job_id: i32,
    completed_at: i64,
    completed_at_str: String,
) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
            UPDATE news_cron_job 
            SET completed_at = $1, completed_at_str = $2
            WHERE id = $3
        "#,
        completed_at,
        completed_at_str,
        cron_job_id
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn update_news_cron_job_error(
    pool: &PgPool,
    cron_job_id: i32,
    error: String,
) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
            UPDATE news_cron_job 
            SET error = $1
            WHERE id = $2
        "#,
        error,
        cron_job_id
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn get_last_completed_news_cron_job(pool: &PgPool) -> Result<NewsCronJob, sqlx::Error> {
    sqlx::query_as!(
        NewsCronJob,
        r#"
        SELECT started_at, started_at_str, completed_at, completed_at_str, error            
        FROM news_cron_job
        WHERE completed_at IS NOT NULL
        ORDER BY completed_at DESC
        LIMIT 1
     "#
    )
    .fetch_one(pool)
    .await
}

pub async fn truncate_news_cron_job(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("TRUNCATE news_cron_job RESTART IDENTITY")
        .execute(pool)
        .await?;
    Ok(())
}
