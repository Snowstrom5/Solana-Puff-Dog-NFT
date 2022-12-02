use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Tweet url reference
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
pub struct NewsReferencedTweetUrl {
    pub tweet_id: i64,
    pub url_id: i32,
}
