use crate::models::news_feed_url::{NewsFeedUrl, NewsFeedUrlSlug};
use anyhow::Result;
use sqlx::{postgres::PgQueryResult, PgPool};

pub async fn insert_news_feed_url(
    pool: &PgPool,
    news_feed_url: NewsFeedUrl,
) -> Result<NewsFeedUrl, sqlx::Error> {
    sqlx::query_as!(
        NewsFeedUrl,
        r#"
            INSERT INTO news_feed_url ( 
                url_slug, url_id, url_score, num_references, first_referenced_by, is_climate_related, created_at, created_at_str, tweeted_at, tweeted_at_str
             )
            VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING 
                url_slug, url_id, url_score, num_references, first_referenced_by, is_climate_related, created_at, created_at_str, tweeted_at, tweeted_at_str
            "#,
        news_feed_url.url_slug,
        news_feed_url.url_id,
        news_feed_url.url_score,
        news_feed_url.num_references,
        news_feed_url.first_referenced_by,
        news_feed_url.is_climate_related,
        news_feed_url.created_at,
        news_feed_url.created_at_str,
        news_feed_url.tweeted_at,
        news_feed_url.tweeted_at_str,
    )
    .fetch_one(pool)
    .await
}

pub async fn find_news_feed_url_by_url_id(pool: &PgPool, url_id: i32) -> Option<NewsFeedUrl> {
    let query = sqlx::query_as!(
        NewsFeedUrl,
        r#"
            SELECT url_slug, url_id, url_score, num_references, first_referenced_by, is_climate_related, created_at, created_at_str, tweeted_at, tweeted_at_str
            FROM news_feed_url
            WHERE url_id = $1;
        "#,
        url_id
    );

    let news_feed_url_result = query.fetch_one(pool).await;
    match news_feed_url_result {
        Ok(news_feed_url) => Some(news_feed_url),
        Err(_) => None,
    }
}

pub async fn reset_news_feed_url_url_scores(pool: &PgPool) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
        UPDATE news_feed_url 
        SET url_score = 0
        "#,
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn update_news_feed_url_url_score_and_num_references(
    pool: &PgPool,
    url_id: i32,
    url_score: i32,
    num_references: i32,
) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
        UPDATE news_feed_url 
        SET url_score = $1, num_references = $2
        WHERE url_id = $3
            "#,
        url_score,
        num_references,
        url_id
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn update_news_feed_url_url_is_climate_related(
    pool: &PgPool,
    url_id: i32,
    is_climate_related: bool,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE news_feed_url 
        SET is_climate_related = $1
        WHERE url_id = $2
        "#,
        is_climate_related,
        url_id
    )
    .execute(pool)
    .await
}

pub async fn update_news_feed_url_tweeted_at(
    pool: &PgPool,
    url_id: i32,
    tweeted_at: i64,
    tweeted_at_str: String,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE news_feed_url 
        SET tweeted_at = $1, tweeted_at_str = $2
        WHERE url_id = $3
        "#,
        tweeted_at,
        tweeted_at_str,
        url_id
    )
    .execute(pool)
    .await
}

pub async fn find_news_feed_url_by_url_slug(
    pool: &PgPool,
    url_slug: String,
) -> Result<NewsFeedUrl, sqlx::Error> {
    let query = sqlx::query_as!(
        NewsFeedUrl,
        r#"
            SELECT url_slug, url_id, url_score, num_references, first_referenced_by, is_climate_related, created_at, created_at_str, tweeted_at, tweeted_at_str
            FROM news_feed_url
            WHERE url_slug = $1        
        "#,
        url_slug
    );

    query.fetch_one(pool).await
}

pub async fn find_top_news_feed_urls_without_is_climate_related_set(
    pool: &PgPool,
) -> Result<Vec<NewsFeedUrl>, sqlx::Error> {
    // Limiting results to 20 to reduce OpenAI API fees
    let query = sqlx::query_as!(
        NewsFeedUrl,
        r#"
            SELECT url_slug, url_id, url_score, num_references, first_referenced_by, is_climate_related, created_at, created_at_str, tweeted_at, tweeted_at_str
            FROM news_feed_url
            WHERE 
                is_climate_related IS NULL 
            ORDER BY url_score DESC
            LIMIT 20
            
        "#
    );

    query.fetch_all(pool).await
}

pub async fn find_news_feed_url_slugs_within_date_range(
    pool: &PgPool,
    from_timestamp: i64,
    to_timestamp: i64,
) -> Result<Vec<NewsFeedUrlSlug>, sqlx::Error> {
    let query = sqlx::query_as!(
        NewsFeedUrlSlug,
        r#"
            SELECT url_slug
            FROM news_feed_url
            WHERE
                created_at >= $1  
                AND created_at < $2  
                AND is_climate_related = True
        "#,
        from_timestamp,
        to_timestamp
    );

    query.fetch_all(pool).await
}

pub async fn truncate_news_feed_url(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("TRUNCATE news_feed_url RESTART IDENTITY")
        .execute(pool)
        .await?;
    Ok(())
}
