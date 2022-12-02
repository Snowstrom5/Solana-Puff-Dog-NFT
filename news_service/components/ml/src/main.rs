use anyhow::Result;
use db::constants::NEWS_FEED_URLS_NUM_DAYS;
use db::init_env;
use db::queries::news_feed_url_query::NewsFeedUrlQuery;
use db::sql::news_feed_url_query::get_news_feed_urls;
use db::util::db::init_db;
use db::util::time::past_days;
use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

pub const NEWS_FEED_URLS_LIMIT_ML: i64 = 700;
pub const FILE_NAME: &str = "news_feed_urls.jsonl";
pub const OPENAI_PROMPT_END: &str = " \n\n###\n\n";

#[derive(Debug, Serialize)]
struct Record {
    completion: String,
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    init_env();
    export_news_feed_urls().await?;
    Ok(())
}

async fn export_news_feed_urls() -> Result<()> {
    let db_pool = init_db().await;
    let recent_timestamp = past_days(NEWS_FEED_URLS_NUM_DAYS).unix_timestamp();
    let news_feed_urls: Vec<NewsFeedUrlQuery> =
        get_news_feed_urls(&db_pool, recent_timestamp, NEWS_FEED_URLS_LIMIT_ML)
            .await
            .unwrap();

    if Path::new(FILE_NAME).exists() {
        fs::remove_file(FILE_NAME)?;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(FILE_NAME)?;

    let mut news_feed_url_hashset: HashSet<String> = HashSet::new();
    for news_feed_url in news_feed_urls {
        let title_and_description = remove_unicode(format!(
            "{} - {}",
            news_feed_url.title, news_feed_url.description
        ));
        let prompt = remove_unicode(format!("{}{}", title_and_description, OPENAI_PROMPT_END));

        if !news_feed_url_hashset.contains(&prompt.clone()) {
            let record = Record {
                completion: String::from(" 1\n"),
                prompt: prompt.clone(),
            };
            jsonl::write(&mut file, &record)?;
            news_feed_url_hashset.insert(prompt.clone());
        }
    }
    Ok(())
}

fn remove_unicode(s: String) -> String {
    s.replace(|c: char| !c.is_ascii(), "").replace('"', "'")
}
