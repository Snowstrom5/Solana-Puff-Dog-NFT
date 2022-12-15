use crate::twitter::api::post_tweet;
use crate::twitter::oauth::get_api_user_ctx;
use anyhow::Result;
use chrono::{Local, Utc};
use db::constants::{
    NEWS_FEED_MIN_NUM_SHARES_BEFORE_TWEETING, NEWS_FEED_URLS_LIMIT, NEWS_FEED_URLS_NUM_DAYS,
};
use db::models::news_cron_job::{CronType, NewsCronJob};
use db::queries::news_feed_url_query::NewsFeedUrlQuery;
use db::sql::news_cron_job::{
    insert_news_cron_job, update_news_cron_job_completed_at, update_news_cron_job_error,
};
use db::sql::news_feed_url::update_news_feed_url_tweeted_at;
use db::sql::news_feed_url_query::get_news_feed_urls;
use db::util::convert::{datetime_to_str, now_utc_datetime};
use db::util::db::init_db;
use db::util::time::past_days;
use log::{debug, error, info, warn};
use sqlx::PgPool;
use tokio_schedule::{every, Job};

pub async fn start_tweet_scheduler() {
    info!("start_tweet_scheduler - {:?}", Local::now());
    let db_pool = init_db().await;
    let tweet_scheduler = every(2).hours().in_timezone(&Utc).perform(|| async {
        if let Err(err) = start_tweet_cron_job(&db_pool).await {
            println!("start_tweet_cron_job failed: {:?}", err);
        }
    });
    tweet_scheduler.await;
}

pub async fn start_tweet_cron_job(db_pool: &PgPool) -> anyhow::Result<()> {
    let start_datetime = now_utc_datetime();
    let news_cron_job = NewsCronJob {
        cron_type: CronType::Tweet.to_string(),
        started_at: start_datetime.unix_timestamp(),
        started_at_str: datetime_to_str(start_datetime),
        completed_at: None,
        completed_at_str: None,
        error: None,
    };

    let news_cron_job_db = insert_news_cron_job(db_pool, news_cron_job).await?;
    match tweet_cron_job(&db_pool).await {
        Ok(_) => {
            let completed_datetime = now_utc_datetime();
            update_news_cron_job_completed_at(
                &db_pool,
                news_cron_job_db.id,
                completed_datetime.unix_timestamp(),
                datetime_to_str(completed_datetime),
            )
            .await?;
        }
        Err(err) => {
            update_news_cron_job_error(&db_pool, news_cron_job_db.id, err.to_string()).await?;
            error!("tweet_cron_job failed: {:?}", err);
        }
    }
    Ok(())
}

pub async fn tweet_cron_job(db_pool: &PgPool) -> Result<()> {
    info!("tweet_cron_job started - {:?}", Local::now());
    let recent_timestamp = past_days(NEWS_FEED_URLS_NUM_DAYS).unix_timestamp();

    match get_news_feed_urls(db_pool, recent_timestamp, NEWS_FEED_URLS_LIMIT).await {
        Ok(news_feed_urls) => {
            // The NewsFeedUrls not shared on Twitter yet
            let news_feed_urls_not_tweeted: Vec<NewsFeedUrlQuery> = news_feed_urls
                .into_iter()
                .filter(|nfu| {
                    nfu.tweeted_at.is_none()
                        && nfu.num_references >= NEWS_FEED_MIN_NUM_SHARES_BEFORE_TWEETING
                })
                .collect();
            match news_feed_urls_not_tweeted.first() {
                Some(news_feed_url) => {
                    info!(
                        "news_feed_url found not shared on Twitter- {:?}",
                        news_feed_url
                    );

                    let tweet_text = get_tweet_text(news_feed_url);
                    if cfg!(debug_assertions) {
                        debug!("tweet_text - {}", tweet_text);
                    } else {
                        // Only post tweets in release mode
                        let api_user_ctx = get_api_user_ctx().await;
                        post_tweet(&api_user_ctx, tweet_text).await?;
                    }

                    //Update tweeted_at value
                    let now_utc_datetime = now_utc_datetime();
                    update_news_feed_url_tweeted_at(
                        db_pool,
                        news_feed_url.url_id,
                        now_utc_datetime.unix_timestamp(),
                        datetime_to_str(now_utc_datetime),
                    )
                    .await?;
                }
                None => {
                    warn!("all news_feed_urls have been shared on Twitter");
                }
            }
        }
        Err(e) => {
            info!("no news_feed_urls found - {:?}", e);
        }
    }

    Ok(())
}

pub fn get_tweet_text(news_feed_url: &NewsFeedUrlQuery) -> String {
    format!(
        r#"{}

More info: https://climatenews.app/news_feed/{}

{}

Article link: {}
#ClimateNews"#,
        news_feed_url.title,
        news_feed_url.url_slug,
        tweet_shared_by_text(news_feed_url),
        news_feed_url.expanded_url_parsed
    )
}

// TODO avoid duplicating this logic on web and backend
pub fn tweet_shared_by_text(news_feed_url: &NewsFeedUrlQuery) -> String {
    let shared_by_text = format!("Shared by @{}", news_feed_url.first_referenced_by_username);
    let mut num_references_text = String::from("");
    if news_feed_url.num_references > 2 {
        num_references_text = format!("and {} others", news_feed_url.num_references - 1);
    } else if news_feed_url.num_references == 2 {
        num_references_text = format!("and 1 other");
    }
    format!("{} {}", shared_by_text, num_references_text)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_tweet_text() {
        let news_feed_url_query = NewsFeedUrlQuery {
            url_slug: String::from("example-slug"),
            url_id: 1,
            url_score: 100,
            num_references: 2,
            tweeted_at: None,
            first_referenced_by_username: String::from("climatenews_app"),
            created_at: 0,
            title: String::from("Example Title"),
            description: String::from("example description"),
            expanded_url_parsed: String::from("https://www.theguardian.com/environment/2022/dec/12/brazil-goldminers-carve-road-to-chaos-amazon-reserve"),
            expanded_url_host: String::from("https://www.theguardian.com"),
            display_url: String::from("https://www.theguardian.com"),
             preview_image_thumbnail_url: None,
             preview_image_url: None,
        };

        assert_eq!(
            get_tweet_text(&news_feed_url_query),
            String::from("Example Title\n\nMore info: https://climatenews.app/news_feed/example-slug\n\nShared by @climatenews_app and 1 other\n\nArticle link: https://www.theguardian.com/environment/2022/dec/12/brazil-goldminers-carve-road-to-chaos-amazon-reserve\n#ClimateNews")
    );
    }
}
