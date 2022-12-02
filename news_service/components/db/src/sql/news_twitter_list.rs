use crate::models::news_twitter_list::NewsTwitterList;
use anyhow::Result;
use sqlx::PgPool;

pub async fn insert_news_twitter_list(
    pool: &PgPool,
    news_twitter_list: NewsTwitterList,
) -> Result<NewsTwitterList, sqlx::Error> {
    sqlx::query_as!(
        NewsTwitterList,
        r#"
            INSERT INTO news_twitter_list ( 
                list_id, last_checked_at
             )
            VALUES ( $1, $2)
            RETURNING 
                list_id, last_checked_at
            "#,
        news_twitter_list.list_id,
        news_twitter_list.last_checked_at,
    )
    .fetch_one(pool)
    .await
}

pub async fn find_news_twitter_list_by_list_id(
    pool: &PgPool,
    list_id: i64,
) -> Result<NewsTwitterList, sqlx::Error> {
    sqlx::query_as!(
        NewsTwitterList,
        r#"
            SELECT list_id, last_checked_at
            FROM news_twitter_list
            WHERE list_id = $1;
        "#,
        list_id
    )
    .fetch_one(pool)
    .await
}

pub async fn update_news_twitter_list_last_checked_at(
    pool: &PgPool,
    list_id: i64,
    last_checked_at: i64,
) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
            UPDATE news_twitter_list 
            SET last_checked_at = $1
            WHERE list_id = $2
            "#,
        last_checked_at,
        list_id
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn truncate_news_twitter_list(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("TRUNCATE news_twitter_list RESTART IDENTITY")
        .execute(pool)
        .await?;
    Ok(())
}
