use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Direct & Indirect referenced tweet URLS
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
pub struct NewsReferencedUrlQuery {
    pub author_id: i64,
    pub url_id: i32,
    pub user_score: Option<i32>,
    pub created_at: i64,
    pub created_at_str: String,
}
