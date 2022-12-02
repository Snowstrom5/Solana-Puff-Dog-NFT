use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// User referenced tweets join query
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
pub struct NewsUserReferencedTweetQuery {
    pub author_id: i64,
    pub tweet_id: i64,
    pub referenced_author_id: i64,
    pub referenced_tweet_id: i64,
    pub referenced_tweet_kind: String,
}
