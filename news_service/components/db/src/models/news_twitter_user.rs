use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Twitter User data
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
pub struct NewsTwitterUser {
    pub user_id: i64,
    pub username: String,
    pub profile_image_url: Option<String>,
    pub description: Option<String>,
    pub verified: Option<bool>,
    pub followers_count: i32,
    pub listed_count: i32,
    pub user_referenced_tweets_count: Option<i32>,
    pub user_score: Option<i32>,
    pub last_tweet_id: Option<i64>,
    pub last_updated_at: i64,
    pub last_checked_at: i64,
}
