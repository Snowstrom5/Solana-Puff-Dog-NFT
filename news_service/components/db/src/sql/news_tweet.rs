use crate::models::news_tweet::NewsTweet;
use sqlx::PgPool;

pub async fn insert_news_tweet(pool: &PgPool, news_tweet: NewsTweet) -> anyhow::Result<()> {
    sqlx::query_as!(
        NewsTweet,
        r#"
            INSERT INTO news_tweet ( 
                tweet_id, text, author_id, conversation_id, in_reply_to_user_id, created_at, created_at_str
             )
            VALUES ( $1, $2, $3, $4, $5, $6, $7)
            RETURNING 
            tweet_id, text, author_id, conversation_id, in_reply_to_user_id, created_at, created_at_str
            "#,
        news_tweet.tweet_id,
        news_tweet.text,
        news_tweet.author_id,
        news_tweet.conversation_id,
        news_tweet.in_reply_to_user_id,
        news_tweet.created_at,
        news_tweet.created_at_str,
    )
    .fetch_one(pool)
    .await?;

    Ok(())
}


pub async fn find_news_tweet_by_tweet_id(pool: &PgPool, tweet_id: i64) -> Option<NewsTweet> {
    let news_tweet_result = sqlx::query_as!(
        NewsTweet,
        r#"
            SELECT 
                tweet_id, text, author_id, conversation_id, in_reply_to_user_id, created_at, created_at_str  
            FROM news_tweet 
            WHERE tweet_id = $1              
            "#,
        tweet_id
    )
    .fetch_one(pool)
    .await;
    match news_tweet_result {
        Ok(news_tweet) => Some(news_tweet),
        Err(_) => None,
    }
}

pub async fn truncate_news_tweet(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("TRUNCATE news_tweet RESTART IDENTITY")
        .execute(pool)
        .await?;
    Ok(())
}
