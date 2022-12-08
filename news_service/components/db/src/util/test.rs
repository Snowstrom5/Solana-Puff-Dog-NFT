// #[cfg(test)]
pub mod test_util {
    use crate::models::news_cron_job::NewsCronJob;
    use crate::models::news_feed_url::NewsFeedUrl;
    use crate::models::news_referenced_tweet::NewsReferencedTweet;
    use crate::models::news_referenced_tweet_url::NewsReferencedTweetUrl;
    use crate::models::news_tweet::NewsTweet;
    use crate::models::news_tweet_url::NewsTweetUrl;
    use crate::models::news_twitter_referenced_user::NewsTwitterReferencedUser;
    use crate::models::news_twitter_user::NewsTwitterUser;
    use crate::sql::news_cron_job::{insert_news_cron_job, truncate_news_cron_job};
    use crate::sql::news_feed_url::{insert_news_feed_url, truncate_news_feed_url};
    use crate::sql::news_referenced_tweet::{
        insert_news_referenced_tweet, truncate_news_referenced_tweet,
    };
    use crate::sql::news_referenced_tweet_url::{
        insert_news_referenced_tweet_url, truncate_news_referenced_tweet_url,
    };
    use crate::sql::news_tweet::{insert_news_tweet, truncate_news_tweet};
    use crate::sql::news_tweet_url::{insert_news_tweet_url, truncate_news_tweet_url};
    use crate::sql::news_twitter_referenced_user::{
        insert_news_twitter_referenced_user, truncate_news_twitter_referenced_user,
    };
    use crate::sql::news_twitter_user::{insert_news_twitter_user, truncate_news_twitter_user};
    use crate::util::convert::datetime_to_str;
    use sqlx::PgPool;
    use time::OffsetDateTime;

    pub async fn create_fake_news_tweet_url(db_pool: &PgPool, created_at_timestamp: i64) {
        truncate_news_tweet_url(db_pool).await.unwrap();
        let news_tweet_url = NewsTweetUrl {
            url: String::from("https://t.co/4HPNrqOnZj"),
            expanded_url: String::from("expanded_url"),
            expanded_url_parsed: String::from("expanded_url_parsed"),
            expanded_url_host: String::from("expanded_url_host"),
            display_url: String::from("display_url"),
            is_twitter_url: false,
            is_english: true,
            preview_image_url: Some(String::from("preview_image_url")),
            preview_image_thumbnail_url: Some(String::from("preview_image_thumbnail_url")),
            title: String::from("example title"),
            description: String::from("description"),
            created_at: created_at_timestamp,
            created_at_str: String::from("created_at_str"),
        };
        insert_news_tweet_url(db_pool, news_tweet_url)
            .await
            .unwrap();
    }

    pub async fn create_fake_news_tweet(db_pool: &PgPool, created_at_timestamp: i64) {
        truncate_news_tweet(db_pool).await.unwrap();
        let news_tweet = NewsTweet {
            tweet_id: 1,
            text: String::from("tweet_text"),
            author_id: 1,
            conversation_id: Some(1),
            in_reply_to_user_id: None,
            created_at: created_at_timestamp,
            created_at_str: String::from("created_at_str"),
        };
        insert_news_tweet(db_pool, news_tweet).await.unwrap();
        let news_tweet_retweeted = NewsTweet {
            tweet_id: 2,
            text: String::from("RT tweet_text"),
            author_id: 2,
            conversation_id: Some(2),
            in_reply_to_user_id: None,
            created_at: created_at_timestamp,
            created_at_str: String::from("created_at_str"),
        };
        insert_news_tweet(db_pool, news_tweet_retweeted)
            .await
            .unwrap();

        let news_tweet_quoted = NewsTweet {
            tweet_id: 3,
            text: String::from("quoted_tweet_text"),
            author_id: 3,
            conversation_id: Some(3),
            in_reply_to_user_id: None,
            created_at: created_at_timestamp,
            created_at_str: String::from("created_at_str"),
        };
        insert_news_tweet(db_pool, news_tweet_quoted).await.unwrap();
    }

    pub async fn create_fake_news_referenced_tweet_url(db_pool: &PgPool) {
        truncate_news_referenced_tweet_url(db_pool).await.unwrap();

        let news_referenced_tweet_url = NewsReferencedTweetUrl {
            tweet_id: 1,
            url_id: 1,
        };
        insert_news_referenced_tweet_url(db_pool, news_referenced_tweet_url)
            .await
            .unwrap();

        let news_referenced_tweet_url_retweeted = NewsReferencedTweetUrl {
            tweet_id: 2,
            url_id: 1,
        };
        insert_news_referenced_tweet_url(db_pool, news_referenced_tweet_url_retweeted)
            .await
            .unwrap();

        let news_referenced_tweet_url_quoted = NewsReferencedTweetUrl {
            tweet_id: 3,
            url_id: 1,
        };
        insert_news_referenced_tweet_url(db_pool, news_referenced_tweet_url_quoted)
            .await
            .unwrap();
    }

    pub async fn create_fake_news_referenced_tweets(db_pool: &PgPool) {
        truncate_news_referenced_tweet(db_pool).await.unwrap();

        let news_referenced_tweet_retweeted = NewsReferencedTweet {
            tweet_id: 2,
            referenced_tweet_id: 1,
            referenced_tweet_kind: String::from("retweeted"),
        };
        insert_news_referenced_tweet(db_pool, news_referenced_tweet_retweeted)
            .await
            .unwrap();

        let news_referenced_tweet_quoted = NewsReferencedTweet {
            tweet_id: 3,
            referenced_tweet_id: 1,
            referenced_tweet_kind: String::from("quoted"),
        };
        insert_news_referenced_tweet(db_pool, news_referenced_tweet_quoted)
            .await
            .unwrap();
    }

    pub async fn create_fake_news_feed_url(db_pool: &PgPool, created_at_timestamp: i64) {
        truncate_news_feed_url(db_pool).await.unwrap();
        let news_feed_url = NewsFeedUrl {
            url_slug: String::from("example-title"),
            url_id: 1,
            url_score: 90,
            num_references: 2,
            first_referenced_by: 1,
            is_climate_related: Some(true),
            created_at: created_at_timestamp,
            created_at_str: String::from("created_at_str"),
        };
        insert_news_feed_url(db_pool, news_feed_url).await.unwrap();
    }

    pub async fn create_fake_news_twitter_user(db_pool: &PgPool) {
        truncate_news_twitter_user(db_pool).await.unwrap();
        let news_twitter_user = NewsTwitterUser {
            user_id: 1,
            username: String::from("username"),
            profile_image_url: Some(String::from("profile_image_url")),
            description: Some(String::from("description")),
            verified: Some(true),
            followers_count: 100,
            listed_count: 100,
            user_referenced_tweets_count: None,
            user_score: None,
            last_tweet_id: None,
            last_updated_at: 0,
            last_checked_at: 0,
        };
        insert_news_twitter_user(db_pool, news_twitter_user)
            .await
            .unwrap();
    }

    pub async fn create_fake_news_twitter_referenced_user(db_pool: &PgPool) {
        truncate_news_twitter_referenced_user(db_pool)
            .await
            .unwrap();
        let news_twitter_referenced_user_retweeted = NewsTwitterReferencedUser {
            user_id: 2,
            username: String::from("retweeted_username"),
        };
        insert_news_twitter_referenced_user(db_pool, news_twitter_referenced_user_retweeted)
            .await
            .unwrap();
        let news_twitter_referenced_user_quoted = NewsTwitterReferencedUser {
            user_id: 3,
            username: String::from("quoted_username"),
        };
        insert_news_twitter_referenced_user(db_pool, news_twitter_referenced_user_quoted)
            .await
            .unwrap();
    }

    pub async fn create_fake_news_cron_job(db_pool: &PgPool, start_datetime: OffsetDateTime) {
        truncate_news_cron_job(db_pool).await.unwrap();

        let news_cron_job = NewsCronJob {
            started_at: start_datetime.unix_timestamp(),
            started_at_str: datetime_to_str(start_datetime),
            completed_at: Some(start_datetime.unix_timestamp()),
            completed_at_str: Some(datetime_to_str(start_datetime)),
            error: None,
        };

        insert_news_cron_job(db_pool, news_cron_job).await.unwrap();
    }
}
