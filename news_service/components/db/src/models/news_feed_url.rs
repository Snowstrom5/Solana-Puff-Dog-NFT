use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// News feed URL
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
pub struct NewsFeedUrl {
    pub url_slug: String,
    pub url_id: i32,
    pub url_score: i32,
    pub num_references: i32,
    pub first_referenced_by: i64,
    pub is_climate_related: Option<bool>,
    pub created_at: i64,
    pub created_at_str: String,
}
