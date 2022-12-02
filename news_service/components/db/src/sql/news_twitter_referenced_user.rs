use crate::models::news_twitter_referenced_user::NewsTwitterReferencedUser;
use sqlx::PgPool;

pub async fn insert_news_twitter_referenced_user(
    pool: &PgPool,
    news_twitter_referenced_user: NewsTwitterReferencedUser,
) -> Result<NewsTwitterReferencedUser, sqlx::Error> {
    sqlx::query_as!(
        NewsTwitterReferencedUser,
        r#"
            INSERT INTO news_twitter_referenced_user ( 
                user_id, username
             )
            VALUES ( $1, $2)
            RETURNING 
            user_id, username
            "#,
        news_twitter_referenced_user.user_id,
        news_twitter_referenced_user.username,
    )
    .fetch_one(pool)
    .await
}

pub async fn find_news_twitter_referenced_user_by_user_id(
    pool: &PgPool,
    user_id: i64,
) -> Result<NewsTwitterReferencedUser, sqlx::Error> {
    sqlx::query_as!(
        NewsTwitterReferencedUser,
        r#"
            SELECT user_id, username
            FROM news_twitter_referenced_user
            WHERE user_id = $1;
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn truncate_news_twitter_referenced_user(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("TRUNCATE news_twitter_referenced_user RESTART IDENTITY")
        .execute(pool)
        .await?;
    Ok(())
}
