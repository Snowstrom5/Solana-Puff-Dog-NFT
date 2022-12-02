use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// News feed URLs
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
#[graphql(name = "NewsFeedUrl")]
pub struct NewsFeedUrlQuery {
    pub url_id: i32,
    pub url_score: i32,
    pub num_references: i32,
    pub first_referenced_by_username: String,
    pub created_at: i64,
    pub title: String,
    pub description: String,
    pub expanded_url_parsed: String,
    pub expanded_url_host: String,
    pub display_url: String,
    pub preview_image_thumbnail_url: Option<String>,
    pub preview_image_url: Option<String>,
}
