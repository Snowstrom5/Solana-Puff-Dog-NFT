use crate::graphql::errors::GqlError;
use async_graphql::{ErrorExtensions, FieldResult};

use db::sql::news_feed_url::find_news_feed_url_slugs_within_date_range;
use db::util::time::timestamp_from_month_year;
use sqlx::postgres::PgPool;

pub async fn sitemap_news_feed_url_slugs_query<'a>(
    db_pool: &PgPool,
    month: i32,
    year: i32,
) -> FieldResult<Vec<String>> {
    let from_timestamp = timestamp_from_month_year(month, year)?;
    let to_timestamp = if month == 12 {
        timestamp_from_month_year(1, year + 1)?
    } else {
        timestamp_from_month_year(month + 1, year)?
    };

    match find_news_feed_url_slugs_within_date_range(db_pool, from_timestamp, to_timestamp).await {
        Ok(news_feed_urls) => {
            let url_slugs: Vec<String> = news_feed_urls
                .iter()
                .map(|nfu| nfu.url_slug.to_string())
                .collect();
            Ok(url_slugs)
        }
        Err(_) => Err(GqlError::NotFound.extend()),
    }
}

#[cfg(test)]
mod tests {

    use crate::graphql::test_util::create_fake_schema;
    use async_graphql::value;
    use db::{init_env, init_test_db_pool, util::test::test_util::create_fake_news_feed_url};
    use time::macros::datetime;

    #[tokio::test]
    async fn get_sitemap_news_feed_url_slugs_test() {
        init_env();
        let db_pool = init_test_db_pool().await.unwrap();
        let created_at_datetime = datetime!(2022 - 01 - 02  0:00).assume_utc();
        let created_at_timestamp = created_at_datetime.unix_timestamp();

        create_fake_news_feed_url(&db_pool, created_at_timestamp).await;

        let schema = create_fake_schema(db_pool);

        let resp = schema
            .execute(
                r#"
                query {
                    query {
                        sitemapNewsFeedUrlSlugs(month: 1, year: 2022) 
                    }(month: 1, year: 2022) 
                }
                "#,
            )
            .await;
        assert_eq!(
            resp.data,
            value!({
                "sitemapNewsFeedUrlSlugs": [
                    String::from("example-title")
                ],
            })
        );
    }
}
