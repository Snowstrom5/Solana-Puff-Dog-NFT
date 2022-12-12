use crate::news_feed::constants::{MAX_TWEET_RESULTS, REQUEST_SLEEP_DURATION};
use crate::util::convert::i64_to_numeric_id;
use anyhow::{bail, Result};
use db::util::time::lookup_period;
use log::info;
use tokio::time::{sleep, Duration};
use twitter_v2::authorization::{BearerToken, Oauth2Token};
use twitter_v2::id::NumericId;
use twitter_v2::prelude::PaginableApiResponse;
use twitter_v2::query::{Exclude, TweetExpansion, TweetField, UserField};
use twitter_v2::{Error, Tweet, TwitterApi, User};

static TWEET_FIELDS: [TweetField; 6] = [
    TweetField::AuthorId,
    TweetField::CreatedAt,
    TweetField::InReplyToUserId,
    TweetField::ConversationId,
    TweetField::Entities,
    TweetField::ReferencedTweets,
];

static TWEET_EXPANSIONS: [TweetExpansion; 1] = [TweetExpansion::AuthorId];

static TWEET_USER_FIELDS: [UserField; 1] = [UserField::Username];

static USER_FIELDS: [UserField; 5] = [
    UserField::PublicMetrics,
    UserField::ProfileImageUrl,
    UserField::Description,
    UserField::Name,
    UserField::Verified,
];

pub async fn get_list_members(
    twitter_api: &TwitterApi<BearerToken>,
    list_id: NumericId,
) -> Result<Vec<User>> {
    info!("Twitter API - get_list_members: {}", list_id);
    let mut list_users: Vec<User> = vec![];
    let list_users_response = twitter_api
        .get_list_members(list_id)
        .user_fields(USER_FIELDS)
        .send()
        .await;

    parse_error_response(&list_users_response).await?;

    let list_users_response = list_users_response.unwrap();

    if let Some(new_list_users) = list_users_response.clone().into_data() {
        list_users = [list_users, new_list_users].concat();
    }
    // TODO make pagination generic
    let mut next_page_response = list_users_response.next_page().await;
    parse_error_response(&next_page_response).await?;
    loop {
        match next_page_response {
            Err(e) => {
                bail!("get_list_members - next_page_response error: {}", e);
            } //return Err(anyhow!(e)),
            Ok(None) => {
                break;
            }
            Ok(Some(ref next_page_result)) => {
                info!("get_list_members - next_page_response");
                let new_list_users: Option<Vec<User>> = next_page_result.clone().into_data();
                if let Some(new_list_users) = new_list_users {
                    list_users = [list_users, new_list_users].concat();
                }
                let new_response = next_page_result.next_page().await;
                parse_error_response(&new_response).await?;
                next_page_response = new_response;
            }
        }
    }

    Ok(list_users)
}

pub async fn get_users_by_author_id(
    twitter_api: &TwitterApi<BearerToken>,
    author_ids: Vec<i64>,
) -> Result<Vec<User>> {
    info!("Twitter API - get_users_by_author_id: {}", author_ids.len());
    let mut user_vec: Vec<User> = vec![];
    let split_author_ids_vec =
        split_requests_into_max_amount(author_ids.iter().map(|i| i64_to_numeric_id(*i)).collect());
    for split_author_ids in split_author_ids_vec {
        let users_response = twitter_api
            .get_users(split_author_ids)
            .user_fields(USER_FIELDS)
            .send()
            .await;

        parse_error_response(&users_response).await?;
        let users = users_response.unwrap().into_data().unwrap();
        user_vec = [user_vec, users].concat();
    }

    Ok(user_vec)
}

pub async fn post_tweet(
    twitter_api: &TwitterApi<Oauth2Token>,
    text: String,
) -> Result<Option<Tweet>> {
    info!("Twitter API - post_tweet: {}", text);
    let tweet_response = twitter_api.post_tweet().text(text).send().await;

    parse_error_response(&tweet_response).await?;
    Ok(tweet_response?.into_data())
}

pub async fn get_user_tweets(
    twitter_api: &TwitterApi<BearerToken>,
    user_id: NumericId,
    last_tweet_id: Option<NumericId>,
) -> Result<Vec<Tweet>> {
    let start_time = lookup_period();

    info!("Twitter API - get_user_tweets: {}", user_id);
    let mut tweets: Vec<Tweet> = vec![];
    let tweets_api_response = if let Some(last_tweet_id) = last_tweet_id {
        // Use last_tweet_id
        twitter_api
            .get_user_tweets(user_id)
            .max_results(MAX_TWEET_RESULTS)
            .tweet_fields(TWEET_FIELDS)
            .start_time(start_time)
            .since_id(last_tweet_id)
            .exclude([Exclude::Replies])
            .send()
            .await
    } else {
        twitter_api
            .get_user_tweets(user_id)
            .max_results(MAX_TWEET_RESULTS)
            .tweet_fields(TWEET_FIELDS)
            .start_time(start_time)
            .exclude([Exclude::Replies])
            .send()
            .await
    };
    parse_error_response(&tweets_api_response).await?;

    let tweets_api_response = tweets_api_response?;

    if let Some(new_tweets) = tweets_api_response.clone().into_data() {
        tweets = [tweets, new_tweets].concat();
    }

    let mut next_page_response = tweets_api_response.next_page().await;

    parse_error_response(&next_page_response).await?;
    loop {
        match next_page_response {
            Err(e) => {
                bail!("next_page_response error: {}", e);
            }
            Ok(None) => {
                break;
            }
            Ok(Some(ref next_page_result)) => {
                let new_tweets: Option<Vec<Tweet>> = next_page_result.clone().into_data();
                if let Some(new_tweets) = new_tweets {
                    tweets = [tweets, new_tweets].concat();
                }
                let new_response = next_page_result.next_page().await;
                parse_error_response(&new_response).await?;
                next_page_response = new_response;
            }
        }
    }
    Ok(tweets)
}

#[derive(Debug)]
pub struct TweetsWithUsers {
    pub tweets: Vec<Tweet>,
    pub users: Vec<User>,
}

pub async fn get_tweets_with_users(
    twitter_api: &TwitterApi<BearerToken>,
    tweet_ids: Vec<NumericId>,
) -> Result<TweetsWithUsers> {
    info!(
        "Twitter API - get_tweets_with_users - num_tweet_ids: {:?} ",
        tweet_ids.len()
    );
    let tweets_response = twitter_api
        .get_tweets(tweet_ids)
        .tweet_fields(TWEET_FIELDS)
        .expansions(TWEET_EXPANSIONS)
        .user_fields(TWEET_USER_FIELDS)
        .send()
        .await;
    parse_error_response(&tweets_response).await?;
    let tweets_response = tweets_response?;

    let includes = tweets_response.includes();
    let tweets = tweets_response.clone().into_data();

    if let (Some(tweets), Some(includes)) = (tweets, includes) {
        if let Some(users) = includes.users.clone() {
            return Ok(TweetsWithUsers { tweets, users });
        }
    }
    bail!("get_tweets - unable to parse TweetsWithUsers from response")
}

//split items into max 100 elements
pub fn split_requests_into_max_amount<T>(items: Vec<T>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut count = 0usize;
    let items_len = items.len();

    let mut split_items_vec: Vec<Vec<T>> = vec![];
    loop {
        if count + MAX_TWEET_RESULTS >= items_len {
            // last elements in vec
            split_items_vec.push(items[count..items_len].to_vec());
            break;
        }
        split_items_vec.push(items[count..count + MAX_TWEET_RESULTS].to_vec());
        count += MAX_TWEET_RESULTS;
    }
    split_items_vec
}

// Retry logic:

pub async fn parse_error_response<T>(response: &Result<T, Error>) -> Result<&T, anyhow::Error> {
    sleep(Duration::from_millis(REQUEST_SLEEP_DURATION)).await;
    match response {
        Err(twitter_v2::Error::Request(ref e)) => {
            if e.is_timeout() {
                bail!("Request error timeout: {}", e);
            } else if e.is_connect() {
                bail!("Request error connect: {}", e);
            } else if e.status() == Some(reqwest::StatusCode::TOO_MANY_REQUESTS) {
                bail!("Request error too many requests: {}", e);
            } else {
                bail!("Request error unhandled: {}", e);
            }
        }
        Err(twitter_v2::Error::Api(ref e)) => {
            if e.status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                bail!("Api error due to 429 response {}", e);
            } else {
                bail!("Api error unhandled: {}", e);
            }
        }
        Err(e) => {
            bail!("generic error unhandled {}", e);
        }
        Ok(response) => Ok(response),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::twitter::init_twitter_api;
    use crate::util::convert::i64_to_numeric_id;
    use db::init_env;

    #[tokio::test]
    async fn get_tweets_test() {
        init_env();
        let twitter_api = init_twitter_api();
        let tweet_ids = vec![
            i64_to_numeric_id(1587558325050560513),
            i64_to_numeric_id(1587605401037512705),
        ];
        let tweets_with_users: TweetsWithUsers = get_tweets_with_users(&twitter_api, tweet_ids)
            .await
            .unwrap();
        assert_eq!(tweets_with_users.tweets.len(), 2);
        assert_eq!(
            tweets_with_users.users.first().unwrap().username,
            "natmaslowski"
        );
    }
}
