use crate::graphql::errors::GqlError;
use async_graphql::{ErrorExtensions, FieldResult};
use db::{
    models::news_feed_url_reference::NewsFeedUrlReference,
    queries::news_feed_url_references_query::NewsFeedUrlReferencesQuery,
    sql::news_feed_url_references_query::get_news_feed_url_references,
};
use sqlx::postgres::PgPool;

pub async fn news_feed_url_references_query<'a>(
    pool: &PgPool,
    url_id: i32,
) -> FieldResult<Vec<NewsFeedUrlReference>> {
    // TODO catch errors and return GQL error
    let news_feed_url_references_result: Option<Vec<NewsFeedUrlReferencesQuery>> =
        get_news_feed_url_references(pool, url_id).await;

    if let Some(news_feed_url_references_query_list) = news_feed_url_references_result {
        let mut news_feed_url_references: Vec<NewsFeedUrlReference> = vec![];
        for news_feed_url_reference_query in news_feed_url_references_query_list.clone() {
            if news_feed_url_reference_query.referenced_tweet_kind
                != Some(String::from("retweeted"))
            {
                // TODO make const for "retweeted"
                let retweeted_by_usernames: Vec<String> = news_feed_url_references_query_list
                    .clone()
                    .into_iter()
                    .filter(|nfu| {
                        nfu.referenced_tweet_id == Some(news_feed_url_reference_query.tweet_id)
                            && nfu.referenced_tweet_kind == Some(String::from("retweeted"))
                            && get_author_username(nfu).is_some()
                    })
                    .map(|nfu| get_author_username(&nfu))
                    .map(|username| format!("@{}", username.unwrap()))
                    .collect();

                let author_username = get_author_username(&news_feed_url_reference_query.clone())
                    .unwrap_or_else(|| String::from(""));

                let news_feed_url_reference = NewsFeedUrlReference {
                    url_id: news_feed_url_reference_query.url_id,
                    tweet_id: news_feed_url_reference_query.tweet_id.to_string(),
                    tweet_text: news_feed_url_reference_query.text.clone(),
                    tweet_created_at_str: news_feed_url_reference_query.created_at_str.clone(),
                    author_username,
                    retweeted_by_usernames,
                };
                news_feed_url_references.push(news_feed_url_reference);
            }
        }
        Ok(news_feed_url_references)
    } else {
        Err(GqlError::NotFound.extend())
    }
}

fn get_author_username(
    news_feed_url_reference_query: &NewsFeedUrlReferencesQuery,
) -> Option<String> {
    match news_feed_url_reference_query.username.clone() {
        Some(author_username) => Some(author_username),
        None => news_feed_url_reference_query.referenced_username.clone(),
    }
}

#[cfg(test)]
mod tests {

    use crate::graphql::test_util::create_fake_schema;
    use async_graphql::value;
    use db::{
        init_env, init_test_db_pool,
        util::{
            convert::now_utc_timestamp,
            test::test_util::{
                create_fake_news_referenced_tweet_url, create_fake_news_referenced_tweets,
                create_fake_news_tweet, create_fake_news_tweet_url,
                create_fake_news_twitter_referenced_user, create_fake_news_twitter_user,
            },
        },
    };

    #[tokio::test]
    async fn get_news_feed_url_references_test() {
        init_env();
        let db_pool = init_test_db_pool().await.unwrap();
        let created_at_timestamp = now_utc_timestamp();

        create_fake_news_twitter_user(&db_pool).await;
        create_fake_news_tweet(&db_pool, created_at_timestamp).await;
        create_fake_news_tweet_url(&db_pool, created_at_timestamp).await;
        create_fake_news_referenced_tweet_url(&db_pool).await;
        create_fake_news_referenced_tweets(&db_pool).await;
        create_fake_news_twitter_referenced_user(&db_pool).await;

        let schema = create_fake_schema(db_pool);

        let resp = schema
            .execute(
                r#"
                query {
                    newsFeedUrlReferences(urlId: 1) {
                        tweetId
                        tweetText
                        tweetCreatedAtStr
                        authorUsername
                        retweetedByUsernames 
                      }
                }
                "#,
            )
            .await;
        assert_eq!(
            resp.data,
            value!({
                "newsFeedUrlReferences": [
                    {
                        "tweetId": String::from("1"),
                        "tweetText": String::from("tweet_text"),
                        "tweetCreatedAtStr": String::from("created_at_str"),
                        "authorUsername": String::from("username"),
                        "retweetedByUsernames": [String::from("retweeted_username")],
                    },
                    {
                        "tweetId": String::from("3"),
                        "tweetText": String::from("quoted_tweet_text"),
                        "tweetCreatedAtStr": String::from("created_at_str"),
                        "authorUsername": String::from("quoted_username"),
                        "retweetedByUsernames": [],
                    }
                ],
            })
        );
    }
}
