use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Tweet data
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
pub struct NewsTweet {
    pub tweet_id: i64,
    pub text: String,
    pub author_id: i64,
    pub conversation_id: Option<i64>,
    pub in_reply_to_user_id: Option<i64>,
    pub created_at: i64,
    pub created_at_str: String,
}
