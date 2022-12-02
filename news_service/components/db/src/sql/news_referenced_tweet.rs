use crate::models::news_referenced_tweet::NewsReferencedTweet;
use sqlx::PgPool;

pub async fn insert_news_referenced_tweet(
    pool: &PgPool,
    news_referenced_tweet: NewsReferencedTweet,
) -> anyhow::Result<()> {
    sqlx::query_as!(
        NewsReferencedTweet,
        r#"
            INSERT INTO news_referenced_tweet ( 
                tweet_id, referenced_tweet_id, referenced_tweet_kind
             )
            VALUES ( $1, $2, $3)
            RETURNING 
                tweet_id, referenced_tweet_id, referenced_tweet_kind
            "#,
        news_referenced_tweet.tweet_id,
        news_referenced_tweet.referenced_tweet_id,
        news_referenced_tweet.referenced_tweet_kind
    )
    .fetch_one(pool)
    .await?;

    Ok(())
}

// TODO: Log sqlx errors + ignore RowNotFound errors
pub async fn find_all_news_referenced_tweets(pool: &PgPool) -> Option<Vec<NewsReferencedTweet>> {
    let news_referenced_tweet_result = sqlx::query_as!(
        NewsReferencedTweet,
        r#"
            SELECT 
                tweet_id, referenced_tweet_id, referenced_tweet_kind      
            FROM news_referenced_tweet            
            "#
    )
    .fetch_all(pool)
    .await;
    match news_referenced_tweet_result {
        Ok(news_referenced_tweet_vec) => Some(news_referenced_tweet_vec),
        Err(_) => None,
    }
}

pub async fn truncate_news_referenced_tweet(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("TRUNCATE news_referenced_tweet RESTART IDENTITY")
        .execute(pool)
        .await?;
    Ok(())
}
