use crate::news_feed::algorithm::news_feed_v1::populate_news_feed_v1;
use crate::news_feed::user_tweets::get_all_user_tweets;
use crate::slack::send_main_cron_message;
use crate::twitter::init_twitter_api;
use anyhow::Result;
use db::models::news_cron_job::{CronType, NewsCronJob};
use db::sql::news_cron_job::{
    insert_news_cron_job, update_news_cron_job_completed_at, update_news_cron_job_error,
};
use db::util::convert::{datetime_to_str, now_utc_datetime};
use db::util::db::init_db;
use db::util::time::now_formated;
use log::{error, info};
use sqlx::PgPool;
use tokio::time::{sleep, Duration};

pub async fn start_main_scheduler() {
    let db_pool = init_db().await;
    loop {
        // cron job continuous loop
        send_main_cron_message(format!("main_cron_job started - {:?}", now_formated()));
        match start_main_cron_job(&db_pool).await {
            Ok(_) => {
                send_main_cron_message(format!("main_cron_job ended - {:?}", now_formated()));
            }
            Err(err) => {
                error!("main_cron_job failed: {:?}", err);
            }
        }
        // Sleep for 10 minutes
        sleep(Duration::from_secs(10 * 60)).await;
    }
}

pub async fn start_main_cron_job(db_pool: &PgPool) -> anyhow::Result<()> {
    let start_datetime = now_utc_datetime();
    let news_cron_job = NewsCronJob {
        cron_type: CronType::Main.to_string(),
        started_at: start_datetime.unix_timestamp(),
        started_at_str: datetime_to_str(start_datetime),
        completed_at: None,
        completed_at_str: None,
        error: None,
    };

    let news_cron_job_db = insert_news_cron_job(db_pool, news_cron_job).await?;
    match main_cron_job(&db_pool).await {
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
            error!("main_cron_job failed: {:?}", err);
            update_news_cron_job_error(&db_pool, news_cron_job_db.id, err.to_string()).await?;
            send_main_cron_message(format!("main_cron_job failed: {:?}", err));
        }
    }
    Ok(())
}

pub async fn main_cron_job(db_pool: &PgPool) -> Result<()> {
    info!("main_cron_job started - {:?}", now_formated());
    let twitter_api = init_twitter_api();
    get_all_user_tweets(db_pool, &twitter_api).await?;
    populate_news_feed_v1(db_pool).await?;
    Ok(())
}
