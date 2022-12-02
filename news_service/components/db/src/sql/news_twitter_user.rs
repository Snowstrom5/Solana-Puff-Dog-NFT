use crate::models::news_twitter_user::NewsTwitterUser;
use sqlx::{postgres::PgQueryResult, PgPool};

pub async fn insert_news_twitter_user(
    pool: &PgPool,
    news_twitter_user: NewsTwitterUser,
) -> Result<NewsTwitterUser, sqlx::Error> {
    sqlx::query_as!(
        NewsTwitterUser,
        r#"
            INSERT INTO news_twitter_user ( 
                user_id, username, profile_image_url, description, verified, followers_count, listed_count, user_referenced_tweets_count, user_score, last_tweet_id, last_updated_at, last_checked_at
             )
            VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING 
            user_id, username, profile_image_url, description, verified, followers_count, listed_count, user_referenced_tweets_count, user_score, last_tweet_id, last_updated_at, last_checked_at
            "#,
        news_twitter_user.user_id,
        news_twitter_user.username,
        news_twitter_user.profile_image_url,
        news_twitter_user.description,
        news_twitter_user.verified,
        news_twitter_user.followers_count,
        news_twitter_user.listed_count,
        news_twitter_user.user_referenced_tweets_count,
        news_twitter_user.user_score,
        news_twitter_user.last_tweet_id,
        news_twitter_user.last_updated_at,
        news_twitter_user.last_checked_at,
    )
    .fetch_one(pool)
    .await
}

pub async fn find_all_news_twitter_users(
    pool: &PgPool,
) -> Result<Vec<NewsTwitterUser>, sqlx::Error> {
    sqlx::query_as!(
        NewsTwitterUser,
        r#"
            SELECT user_id, username, profile_image_url, description, verified, followers_count, listed_count, user_referenced_tweets_count, user_score, last_tweet_id, last_updated_at, last_checked_at
            FROM news_twitter_user
            ORDER BY username ASC
        "#
    ).fetch_all(pool).await
}

pub async fn find_news_twitter_user_by_user_id(
    pool: &PgPool,
    user_id: i64,
) -> Result<NewsTwitterUser, sqlx::Error> {
    sqlx::query_as!(
        NewsTwitterUser,
        r#"
            SELECT user_id, username, profile_image_url, description, verified, followers_count, listed_count, user_referenced_tweets_count, user_score, last_tweet_id, last_updated_at, last_checked_at
            FROM news_twitter_user
            WHERE user_id = $1;
        "#,
        user_id
    ).fetch_one(pool).await
}

pub async fn update_news_twitter_user_last_checked_at(
    pool: &PgPool,
    user_id: i64,
    last_checked_at: i64,
) -> anyhow::Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
            UPDATE news_twitter_user 
            SET last_checked_at = $1
            WHERE user_id = $2
            "#,
        last_checked_at,
        user_id
    )
    .execute(pool)
    .await
}

pub async fn update_news_twitter_user_last_updated_at(
    pool: &PgPool,
    user_id: i64,
    last_tweet_id: i64,
    last_updated_at: i64,
) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
            UPDATE news_twitter_user 
            SET last_tweet_id = $1, last_updated_at = $2
            WHERE user_id = $3
            "#,
        last_tweet_id,
        last_updated_at,
        user_id
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn update_news_twitter_user_stats(
    pool: &PgPool,
    user_id: i64,
    user_referenced_tweets_count: i32,
    user_score: i32,
) -> anyhow::Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
            UPDATE news_twitter_user 
            SET 
                user_referenced_tweets_count = $1,
                user_score = $2 
            WHERE user_id = $3
            "#,
        user_referenced_tweets_count,
        user_score,
        user_id
    )
    .execute(pool)
    .await
}

pub async fn truncate_news_twitter_user(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("TRUNCATE news_twitter_user RESTART IDENTITY")
        .execute(pool)
        .await?;
    Ok(())
}
