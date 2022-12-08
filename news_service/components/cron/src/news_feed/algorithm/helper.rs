use crate::news_feed::models::tweet_info::TweetInfo;
use db::queries::news_referenced_url_query::NewsReferencedUrlQuery;
use deunicode::deunicode_char;
use std::{cmp, collections::HashMap};

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

// Source: https://github.com/Stebalien/slug-rs/blob/master/src/lib.rs
pub fn slugify(s: &str) -> String {
    // Split the sentence into a vector of words
    let words: Vec<&str> = s.split(" ").collect();
    // Get the first eight words from the vector and create a new string
    let s = words[..cmp::min(10, words.len())].to_vec().join(" ");
    let mut slug: Vec<u8> = Vec::with_capacity(s.len());

    // Starts with true to avoid leading -
    let mut prev_is_dash = true;
    {
        let mut push_char = |x: u8| {
            match x {
                b'a'..=b'z' | b'0'..=b'9' => {
                    prev_is_dash = false;
                    slug.push(x);
                }
                b'A'..=b'Z' => {
                    prev_is_dash = false;
                    // Manual lowercasing as Rust to_lowercase() is unicode
                    // aware and therefore much slower
                    slug.push(x - b'A' + b'a');
                }
                _ => {
                    if !prev_is_dash {
                        slug.push(b'-');
                        prev_is_dash = true;
                    }
                }
            }
        };

        for c in s.chars() {
            if c.is_ascii() {
                (push_char)(c as u8);
            } else {
                for &cx in deunicode_char(c).unwrap_or("-").as_bytes() {
                    (push_char)(cx);
                }
            }
        }
    }

    // It's not really unsafe in practice, we know we have ASCII
    let mut string = unsafe { String::from_utf8_unchecked(slug) };
    if string.ends_with('-') {
        string.pop();
    }
    // We likely reserved more space than needed.
    string.shrink_to_fit();
    string
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(
            slugify("The Texas Group Waging a National Crusade Against Climate Action"),
            "the-texas-group-waging-a-national-crusade-against-climate-action"
        );
        assert_eq!(
            slugify("Whistleblower: Enviva claim of ‘being good for the planet… all nonsense’"),
            "whistleblower-enviva-claim-of-being-good-for-the-planet-all"
        );
    }
}
