use crate::graphql::errors::GqlError;
use async_graphql::{ErrorExtensions, FieldResult};
use db::{
    models::news_cron_job::NewsCronJob, sql::news_cron_job::get_last_completed_news_cron_job,
};
use sqlx::postgres::PgPool;

pub async fn news_feed_status_query<'a>(db_pool: &PgPool) -> FieldResult<NewsCronJob> {
    match get_last_completed_news_cron_job(db_pool).await {
        Ok(news_cron_job) => Ok(news_cron_job),
        Err(_) => Err(GqlError::NotFound.extend()),
    }
}

#[cfg(test)]
mod tests {

    use async_graphql::value;
    use db::{
        init_env, init_test_db_pool,
        util::{convert::now_utc_datetime, test::test_util::create_fake_news_cron_job},
    };

    use crate::graphql::test_util::create_fake_schema;

    #[tokio::test]
    async fn news_feed_status_query_test() {
        init_env();
        let db_pool = init_test_db_pool().await.unwrap();
        let test_datetime = now_utc_datetime();
        create_fake_news_cron_job(&db_pool, test_datetime).await;

        let schema = create_fake_schema(db_pool);

        let resp = schema
            .execute(
                r#"
                query {
                    newsFeedStatus {
                        completedAt                       
                    }
                }
                "#,
            )
            .await;
        assert_eq!(
            resp.data,
            value!({
                "newsFeedStatus":
                    {
                        "completedAt": test_datetime.unix_timestamp(),
                    }
                ,
            })
        );
    }
}
