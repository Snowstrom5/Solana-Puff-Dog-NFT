use crate::models::news_referenced_tweet_url::NewsReferencedTweetUrl;
use sqlx::PgPool;

pub async fn insert_news_referenced_tweet_url(
    pool: &PgPool,
    news_referenced_tweet_url: NewsReferencedTweetUrl,
) -> anyhow::Result<()> {
    sqlx::query_as!(
        NewsReferencedTweetUrl,
        r#"
            INSERT INTO news_referenced_tweet_url ( 
                tweet_id, url_id
             )
            VALUES ( $1, $2)
            RETURNING 
                tweet_id, url_id
            "#,
        news_referenced_tweet_url.tweet_id,
        news_referenced_tweet_url.url_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(())
}

// TODO: Log sqlx errors + ignore RowNotFound errors
pub async fn find_news_referenced_tweet_url_by_tweet_id_and_url_id(
    pool: &PgPool,
    tweet_id: i64,
    url_id: i32,
) -> Option<NewsReferencedTweetUrl> {
    let news_referenced_tweet_url_result = sqlx::query_as!(
        NewsReferencedTweetUrl,
        r#"
            SELECT 
                tweet_id, url_id  
            FROM news_referenced_tweet_url 
            WHERE tweet_id = $1 AND url_id = $2                     
            "#,
        tweet_id,
        url_id
    )
    .fetch_one(pool)
    .await;
    match news_referenced_tweet_url_result {
        Ok(news_referenced_tweet_url_vec) => Some(news_referenced_tweet_url_vec),
        Err(_) => None,
    }
}

pub async fn truncate_news_referenced_tweet_url(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("TRUNCATE news_referenced_tweet_url RESTART IDENTITY")
        .execute(pool)
        .await?;
    Ok(())
}
