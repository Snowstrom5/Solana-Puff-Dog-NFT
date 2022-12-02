use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Tweet reference
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
pub struct NewsReferencedTweet {
    pub tweet_id: i64,
    pub referenced_tweet_id: i64,
    pub referenced_tweet_kind: String,
}
