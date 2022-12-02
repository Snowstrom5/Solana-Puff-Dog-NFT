use sqlx::PgPool;

use crate::queries::news_feed_url_references_query::NewsFeedUrlReferencesQuery;

// TODO add user_score
pub async fn get_news_feed_url_references(
    pool: &PgPool,
    url_id: i32,
) -> Option<Vec<NewsFeedUrlReferencesQuery>> {
    let news_feed_url_references_query_result = sqlx::query_as!(
        NewsFeedUrlReferencesQuery,
        r#"
        SELECT 
            rtu.url_id,
            t.text,
            t.tweet_id,
            t.author_id,
            t.created_at_str,
            u.username as "username?",
            ru.username as "referenced_username?",
            rt.referenced_tweet_id as "referenced_tweet_id?",
            rt.referenced_tweet_kind as "referenced_tweet_kind?"
        FROM
            news_referenced_tweet_url as rtu 
            JOIN news_tweet_url as tu ON tu.id = rtu.url_id
            JOIN news_tweet as t ON t.tweet_id = rtu.tweet_id
            LEFT JOIN news_twitter_user as u ON t.author_id = u.user_id	
            LEFT JOIN news_twitter_referenced_user as ru ON t.author_id = ru.user_id
            LEFT JOIN news_referenced_tweet as rt ON t.tweet_id = rt.tweet_id
        WHERE
            rtu.url_id = $1
            AND tu.is_twitter_url = False
            AND tu.title IS NOT NULL
            AND t.in_reply_to_user_id IS NULL
        "#,
        url_id
    )
    .fetch_all(pool)
    .await;
    match news_feed_url_references_query_result {
        Ok(news_feed_url_references) => Some(news_feed_url_references),
        Err(_) => None,
    }
}
