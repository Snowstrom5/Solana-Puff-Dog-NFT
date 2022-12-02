use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Tweet url
#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
pub struct NewsTweetUrlWithId {
    pub id: i32,
    pub url: String,
    pub expanded_url: String,
    pub expanded_url_parsed: String,
    pub expanded_url_host: String,
    pub display_url: String,
    pub is_twitter_url: bool,
    pub is_english: bool,
    pub title: String,
    pub description: String,
    pub preview_image_thumbnail_url: Option<String>,
    pub preview_image_url: Option<String>,
    pub created_at: i64,
    pub created_at_str: String,
}

#[derive(FromRow, Deserialize, Serialize, Debug, Clone, async_graphql::SimpleObject)]
pub struct NewsTweetUrl {
    pub url: String,
    pub expanded_url: String,
    pub expanded_url_parsed: String,
    pub expanded_url_host: String,
    pub display_url: String,
    pub is_twitter_url: bool,
    pub is_english: bool,
    pub title: String,
    pub description: String,
    pub preview_image_thumbnail_url: Option<String>,
    pub preview_image_url: Option<String>,
    pub created_at: i64,
    pub created_at_str: String,
}
