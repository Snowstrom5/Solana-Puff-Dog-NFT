use crate::{
    news_feed::constants::REQUEST_SLEEP_DURATION,
    openai::models::{Completion, CompletionArgs},
    slack::send_main_cron_message,
};
use anyhow::{bail, Error, Result};
use db::models::news_tweet_url::NewsTweetUrlWithId;
use log::info;
use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    StatusCode,
};
use tokio::time::{sleep, Duration};

const BASE_URL: &str = "https://api.openai.com/v1";
const MODEL_NAME: &str = "curie:ft-personal-2022-10-14-18-16-36";
const PROMPT_END: &str = " \n\n###\n\n";

pub async fn fetch_news_tweet_url_climate_classification(
    news_tweet_url: NewsTweetUrlWithId,
) -> Result<bool> {
    let title_and_description =
        format!("{} - {}", news_tweet_url.title, news_tweet_url.description);

    let result = fetch_text_climate_classification(title_and_description.clone()).await;
    info!(
        "OpenAI classification - result: {:?} - text: {}",
        result,
        title_and_description.clone()
    );
    result
}

async fn fetch_text_climate_classification(text: String) -> Result<bool> {
    let prompt = format!("{}{}", text, PROMPT_END);
    let completion = openai_completion_request(prompt).await;
    return match completion?.as_str() {
        " 0" => Ok(false),
        " 1" => Ok(true),
        _ => Err(Error::msg("OpenAI completion - invalid response error")),
    };
}

pub async fn openai_completion_request(prompt: String) -> Result<String> {
    let args = CompletionArgs {
        prompt,
        temperature: 1.0,
        max_tokens: 1,
        model: String::from(MODEL_NAME),
    };
    let url = format!("{}/completions", BASE_URL);
    let client = reqwest::Client::new();
    let body = serde_json::to_string(&args)?;
    let api_token = std::env::var("OPENAI_API_KEY")?;
    let response = client
        .post(url)
        .body(body.clone())
        .header(AUTHORIZATION, format!("Bearer {}", api_token))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await;
    sleep(Duration::from_millis(REQUEST_SLEEP_DURATION)).await;

    match response {
        Err(e) => Err(Error::new(e).context("OpenAI completion API error".to_string())),
        Ok(response) => {
            if response.status() == StatusCode::OK {
                let mut result: Completion = response.json().await?;
                let choice = result.choices.remove(0);
                Ok(choice.text)
            } else {
                let result = response.text().await?;
                send_main_cron_message(format!("openai_completion_request failed: {:?}", result));
                bail!("openai - server error - {}", result)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use db::init_env;

    #[tokio::test]
    async fn fetch_climate_classification_test_1() {
        init_env();
        let text = "Italy: Floods and rain kill at least 10 overnight - officials - Rescuers are searching for four others missing after torrential rainfall hit the Marche region overnight.";
        let is_climate_related = fetch_text_climate_classification(text.to_string())
            .await
            .unwrap();
        assert!(is_climate_related);
    }

    #[tokio::test]
    async fn fetch_climate_classification_test_2() {
        init_env();
        let text = "Former President Donald Trump invoked his Fifth Amendment right more than 440 times on Wednesday during a deposition by lawyers from New York Attorney General Letitia James’ office, according to multiple sources. - Former President Donald Trump invoked his Fifth Amendment right more than 440 times on Wednesday during a deposition by lawyers from New York Attorney General Letitia James’ office, according to multiple sources.";
        let is_climate_related = fetch_text_climate_classification(text.to_string())
            .await
            .unwrap();
        assert!(!is_climate_related);
    }
}
