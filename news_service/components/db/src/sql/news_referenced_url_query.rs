use crate::queries::news_referenced_url_query::NewsReferencedUrlQuery;
use sqlx::PgPool;

// Referenced tweet URLS since timestamp
// Used by the news feed
// e.g
//   1) GretaThunberg shared a link to theguardian.com
//   2) DrShepherd2013 shared a link to nature.com
//   3) GretaThunberg retweeted a tweet that contained a link to theguardian.com
//   4) DrShepherd2013 quoted a tweet that contained a link to nature.com
pub async fn get_news_referenced_urls(
    pool: &PgPool,
    timestamp: i64,
) -> Option<Vec<NewsReferencedUrlQuery>> {
    let news_referenced_url_query_result = sqlx::query_as!(
        NewsReferencedUrlQuery,
        r#"
        SELECT 
            rtu.url_id,
            t.author_id,
            u.user_score,
            t.created_at,
            t.created_at_str
        FROM
            news_referenced_tweet_url as rtu 
            JOIN news_tweet_url as tu ON tu.id = rtu.url_id
            JOIN news_tweet as t ON t.tweet_id = rtu.tweet_id
            LEFT JOIN news_twitter_user as u ON t.author_id = u.user_id	
            
        WHERE
            tu.is_twitter_url = False
            AND tu.is_english = True
            AND tu.title IS NOT NULL
            AND t.in_reply_to_user_id IS NULL
        AND t.created_at > $1
        ORDER BY  
            t.created_at DESC
     "#,
        timestamp
    )
    .fetch_all(pool)
    .await;
    match news_referenced_url_query_result {
        Ok(news_referenced_urls) => Some(news_referenced_urls),
        Err(_) => None,
    }
}
