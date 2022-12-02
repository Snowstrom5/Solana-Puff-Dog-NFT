// Combined News feed URL references
#[derive(Debug, Clone, async_graphql::SimpleObject)]
pub struct NewsFeedUrlReference {
    pub url_id: i32,
    pub tweet_id: String,
    pub tweet_text: String,
    pub tweet_created_at_str: String,
    pub author_username: String,
    pub retweeted_by_usernames: Vec<String>,
}
