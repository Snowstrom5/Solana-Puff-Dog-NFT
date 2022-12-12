use super::helper::slugify;
use super::time_decay::time_decayed_url_score;
use crate::news_feed::algorithm::helper::{populate_author_score_map, populate_url_to_tweet_map};
use crate::news_feed::models::tweet_info::TweetInfo;
use crate::openai::api::fetch_news_tweet_url_climate_classification;
use anyhow::Result;
use chrono::Local;
use db::constants::NEWS_FEED_URLS_NUM_DAYS;
use db::models::news_feed_url::NewsFeedUrl;
use db::sql::news_feed_url::{
    find_news_feed_url_by_url_id, find_news_feed_url_by_url_slug,
    find_top_news_feed_urls_without_is_climate_related_set, insert_news_feed_url,
    reset_news_feed_url_url_scores, update_news_feed_url_url_is_climate_related,
    update_news_feed_url_url_score_and_num_references,
};
use db::sql::news_referenced_url_query::get_news_referenced_urls;
use db::sql::news_tweet_url::find_news_tweet_url_by_id;
use db::util::convert::{
    datetime_from_unix_timestamp, datetime_to_str, now_utc_timestamp, seconds_in_hour,
};
use db::util::time::past_days;
use log::{error, info};
use rust_decimal_macros::dec;
use sqlx::PgPool;
use std::collections::HashMap;

pub async fn populate_news_feed_v1(db_pool: &PgPool) -> Result<()> {
    info!("populate_news_feed_v1 - {:?}", Local::now());
    let recent_timestamp = past_days(NEWS_FEED_URLS_NUM_DAYS).unix_timestamp();

    // Direct & indirect references
    let news_referenced_urls = get_news_referenced_urls(db_pool, recent_timestamp).await;

    let author_score_map: HashMap<i64, i32> = populate_author_score_map(&news_referenced_urls);

    let url_to_tweet_map: HashMap<i32, Vec<TweetInfo>> =
        populate_url_to_tweet_map(&news_referenced_urls);

    // Insert News feed urls
    populate_news_feed_urls_v1(db_pool, author_score_map, url_to_tweet_map).await?;
    info!("populate_news_feed_v1 complete - {:?}", Local::now());
    Ok(())
}

async fn populate_news_feed_urls_v1(
    db_pool: &PgPool,
    author_score_map: HashMap<i64, i32>,
    url_to_tweet_map: HashMap<i32, Vec<TweetInfo>>,
) -> Result<()> {
    // Reset news feed URL scores
    reset_news_feed_url_url_scores(db_pool).await.unwrap();
    // URLS with shared by author count + score
    for url_id in url_to_tweet_map.keys() {
        let tweet_info_vec = url_to_tweet_map.get(url_id).unwrap();
        // Sum total score for url based on author scores
        let url_score: i32 = tweet_info_vec
            .iter()
            .map(|tweet_info| author_score_map.get(&tweet_info.author_id).unwrap())
            .sum();

        // Find the first tweet that shared the url
        let first_tweet: &TweetInfo = tweet_info_vec
            .iter()
            .min_by_key(|tweet_info| tweet_info.created_at)
            .unwrap();

        let hours_since_first_created =
            (now_utc_timestamp() - first_tweet.created_at) / seconds_in_hour();

        // Calculate url score factoring in time decay
        // A gravity of 0.45 is used in the ranking algorithm
        // Set it to 0.5 for newer results or 0.3 to older results
        let gravity = dec!(0.45);
        let time_decayed_url_score =
            time_decayed_url_score(gravity, url_score, hours_since_first_created);

        // Number of references/shares
        let num_references = tweet_info_vec.len() as i32;

        let url_slug = create_news_feed_url_slug(db_pool, *url_id).await?;

        let news_feed_url_db = find_news_feed_url_by_url_id(db_pool, *url_id).await;
        if news_feed_url_db.is_none() && time_decayed_url_score > 0 {
            let news_feed_url = NewsFeedUrl {
                url_slug,
                url_id: *url_id,
                url_score: time_decayed_url_score,
                num_references,
                first_referenced_by: first_tweet.author_id,
                is_climate_related: None,
                created_at: first_tweet.created_at,
                created_at_str: datetime_to_str(datetime_from_unix_timestamp(
                    first_tweet.created_at,
                )),
                tweeted_at: None,
                tweeted_at_str: None,
            };
            insert_news_feed_url(db_pool, news_feed_url).await?;
        } else {
            update_news_feed_url_url_score_and_num_references(
                db_pool,
                *url_id,
                url_score,
                num_references,
            )
            .await?;
        }
    }
    update_is_climate_related_fields(db_pool).await?;
    Ok(())
}

// Create news feed url slug
async fn create_news_feed_url_slug(db_pool: &PgPool, url_id: i32) -> Result<String> {
    let news_tweet_url = find_news_tweet_url_by_id(db_pool, url_id).await?;
    let mut url_slug = slugify(&news_tweet_url.title);
    let mut count = 1u32;
    loop {
        let news_feed_url_db = find_news_feed_url_by_url_slug(db_pool, url_slug.clone()).await;
        if news_feed_url_db.is_ok() {
            // existing url_slug in use, try a new url_slug
            url_slug = format!("{}-{}", url_slug, count);
            count += 1;
        } else {
            // url_slug not in use;
            break;
        }
    }
    Ok(url_slug)
}

// Set the is_climate_related field for the top scoring news_feed_urls
// that have not had the classification field set yet
// Only call OpenAI API for 100 top news feed links to reduce API fees
async fn update_is_climate_related_fields(db_pool: &PgPool) -> Result<()> {
    if let Ok(news_feed_urls) =
        find_top_news_feed_urls_without_is_climate_related_set(db_pool).await
    {
        for news_feed_url in news_feed_urls {
            let news_tweet_url = find_news_tweet_url_by_id(db_pool, news_feed_url.url_id).await?;

            match fetch_news_tweet_url_climate_classification(news_tweet_url).await {
                Ok(is_climate_related) => {
                    update_news_feed_url_url_is_climate_related(
                        db_pool,
                        news_feed_url.url_id,
                        is_climate_related,
                    )
                    .await?;
                }
                Err(err) => {
                    error!(
                        "fetch_news_tweet_url_climate_classification failed: {:?}",
                        err
                    );
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use db::{
        init_env, init_test_db_pool,
        sql::news_feed_url::truncate_news_feed_url,
        util::{
            convert::now_utc_timestamp,
            test::test_util::{create_fake_news_feed_url, create_fake_news_tweet_url},
        },
    };

    #[tokio::test]
    async fn create_news_feed_url_slug_test() {
        init_env();
        let db_pool = init_test_db_pool().await.unwrap();
        truncate_news_feed_url(&db_pool).await.unwrap();
        let created_at_timestamp = now_utc_timestamp();

        create_fake_news_tweet_url(&db_pool, created_at_timestamp).await;

        // url_slug not in use
        let url_slug_new = create_news_feed_url_slug(&db_pool, 1).await.unwrap();
        assert_eq!(url_slug_new, String::from("example-title"));

        create_fake_news_feed_url(&db_pool, created_at_timestamp).await;

        // url_slug in use
        let url_slug_existing = create_news_feed_url_slug(&db_pool, 1).await.unwrap();
        assert_eq!(url_slug_existing, String::from("example-title-1"));
    }
}
