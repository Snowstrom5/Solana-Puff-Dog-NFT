use crate::twitter::api::post_tweet;
use crate::twitter::oauth::get_api_user_ctx;
use anyhow::Result;
use chrono::Local;
use db::constants::{NEWS_FEED_URLS_LIMIT, NEWS_FEED_URLS_NUM_DAYS};
use db::models::news_cron_job::{CronType, NewsCronJob};
use db::queries::news_feed_url_query::NewsFeedUrlQuery;
use db::sql::news_cron_job::{
    insert_news_cron_job, update_news_cron_job_completed_at, update_news_cron_job_error,
};
use db::sql::news_feed_url_query::get_news_feed_urls;
use db::util::convert::{datetime_to_str, now_utc_datetime};
use db::util::db::init_db;
use db::util::time::past_days;
use log::{error, info, warn};
use sqlx::PgPool;
// use tokio_schedule::{every, Job};

pub async fn start_tweet_scheduler() {
    info!("start_tweet_scheduler - {:?}", Local::now());
    let db_pool = init_db().await;
    if let Err(err) = start_tweet_cron_job(&db_pool).await {
        println!("start_tweet_cron_job failed: {:?}", err);
    }
    // let tweet_scheduler = every(1).minutes().in_timezone(&Utc).perform(|| async {
    //     if let Err(err) = start_tweet_cron_job(&db_pool).await {
    //         println!("start_tweet_cron_job failed: {:?}", err);
    //     }
    // });
    // tweet_scheduler.await;
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
            // NewsFeedUrls not shared on Twitter yet
            let news_feed_urls_not_tweeted: Vec<NewsFeedUrlQuery> = news_feed_urls
                .into_iter()
                .filter(|nfu| nfu.tweeted_at.is_none())
                .collect();
            match news_feed_urls_not_tweeted.first() {
                Some(news_feed_url) => {
                    info!(
                        "news_feed_url found not shared on Twitter- {:?}",
                        news_feed_url
                    );

                    let tweet_text = get_tweet_text(news_feed_url);
                    info!("tweet_text- {}", tweet_text);
                    let api_user_ctx = get_api_user_ctx().await;
                    post_tweet(&api_user_ctx, String::from("test tweet")).await?;
                    //Update tweeted_at value
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
    // TODO update tweet format
    // TODO copy shared logic from the frontend
    format!(
        r#"
        {}
        https://climatenews.app/news_feed/{}
        Shared by: {} and {} others
        Article link: {}
        #ClimateNews 
        "#,
        news_feed_url.title,
        news_feed_url.url_slug,
        news_feed_url.first_referenced_by_username,
        news_feed_url.num_references - 1,
        news_feed_url.expanded_url_parsed
    )
}
