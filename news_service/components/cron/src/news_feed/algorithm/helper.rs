use std::collections::HashMap;

use crate::news_feed::models::tweet_info::TweetInfo;
use db::queries::news_referenced_url_query::NewsReferencedUrlQuery;

// Populate a map of author_id to score
// author_id -> user_score
pub fn populate_author_score_map(
    news_referenced_urls: &Option<Vec<NewsReferencedUrlQuery>>,
) -> HashMap<i64, i32> {
    let mut author_score_map: HashMap<i64, i32> = HashMap::new();
    if let Some(news_referenced_urls) = news_referenced_urls {
        for news_referenced_url in news_referenced_urls {
            let author_id = news_referenced_url.author_id;
            let user_score = news_referenced_url.user_score.map_or_else(|| 0, |us| us);

            author_score_map.insert(author_id, user_score);
        }
    }
    author_score_map
}

// Populate a map Urls shared in tweets.
// url_id -> [TweetInfo, TweetInfo]
pub fn populate_url_to_tweet_map(
    news_referenced_urls: &Option<Vec<NewsReferencedUrlQuery>>,
) -> HashMap<i32, Vec<TweetInfo>> {
    let mut url_to_tweet_map: HashMap<i32, Vec<TweetInfo>> = HashMap::new();
    if let Some(news_referenced_urls) = news_referenced_urls {
        for news_referenced_url in news_referenced_urls {
            // Populate TweetInfo
            let url_id = news_referenced_url.url_id;
            let author_id = news_referenced_url.author_id;
            let created_at = news_referenced_url.created_at;
            let tweet_info = TweetInfo {
                author_id,
                created_at,
            };
            // check if url exists in the map
            if url_to_tweet_map.contains_key(&url_id) {
                if let Some(tweet_info_vec) = url_to_tweet_map.get(&url_id) {
                    let mut tweet_info_vec = tweet_info_vec.clone();
                    // Ensures URL is not already added by same author
                    if !tweet_info_vec.iter().any(|ti| ti.author_id == author_id) {
                        tweet_info_vec.push(tweet_info);
                    }
                    url_to_tweet_map.insert(url_id, tweet_info_vec);
                }
            } else {
                url_to_tweet_map.insert(url_id, vec![tweet_info]);
            }
        }
    }
    url_to_tweet_map
}
