use crate::queries::news_feed_url_query::NewsFeedUrlQuery;
use anyhow::Result;
use sqlx::PgPool;

pub async fn get_news_feed_urls(
    pool: &PgPool,
    timestamp: i64,
    limit: i64,
) -> Result<Vec<NewsFeedUrlQuery>, sqlx::Error> {
    sqlx::query_as!(
        NewsFeedUrlQuery,
        r#"
        SELECT
            nfu.url_slug, 
            nfu.url_id, 
            nfu.url_score,
            nfu.num_references,
            u.username as first_referenced_by_username,
            nfu.created_at,
            tu.title,
            tu.description,
            tu.expanded_url_parsed,
            tu.expanded_url_host,
            tu.display_url,
            tu.preview_image_thumbnail_url,
            tu.preview_image_url
            
        FROM
            news_feed_url as nfu
            JOIN news_tweet_url as tu ON tu.id = nfu.url_id
            JOIN news_twitter_user as u ON u.user_id = nfu.first_referenced_by
        WHERE
            nfu.created_at > $1  
            AND nfu.is_climate_related = True  
        ORDER BY
            url_score DESC
            -- num_references DESC
        LIMIT $2 
     "#,
        timestamp,
        limit
    )
    .fetch_all(pool)
    .await
}

pub async fn get_news_feed_url(
    pool: &PgPool,
    url_slug: String,
) -> Result<NewsFeedUrlQuery, sqlx::Error> {
    sqlx::query_as!(
        NewsFeedUrlQuery,
        r#"
        SELECT
            nfu.url_slug, 
            nfu.url_id, 
            nfu.url_score,
            nfu.num_references,
            u.username as first_referenced_by_username,
            nfu.created_at,
            tu.title,
            tu.description,
            tu.expanded_url_parsed,
            tu.expanded_url_host,
            tu.display_url,
            tu.preview_image_thumbnail_url,
            tu.preview_image_url
            
        FROM
            news_feed_url as nfu
            JOIN news_tweet_url as tu ON tu.id = nfu.url_id
            JOIN news_twitter_user as u ON u.user_id = nfu.first_referenced_by
        WHERE
            nfu.url_slug = $1
     "#,
     url_slug
    )
    .fetch_one(pool)
    .await
}
