use log::{error, info};
use slack_hook::{PayloadBuilder, Slack};
use std::env;

static TWEET_CRON_ENV_STR: &str = "TWEET_CRON_WEBHOOK_URL";
static MAIN_CRON_ENV_STR: &str = "MAIN_CRON_WEBHOOK_URL";
static TWEET_CRON_CHANNEL: &str = "#tweet-cron";
static MAIN_CRON_CHANNEL: &str = "#main-cron";

pub fn send_tweet_cron_message(message: String) {
    send_message(TWEET_CRON_ENV_STR, TWEET_CRON_CHANNEL.to_string(), message);
}

pub fn send_main_cron_message(message: String) {
    send_message(MAIN_CRON_ENV_STR, MAIN_CRON_CHANNEL.to_string(), message);
}

fn send_message(webhook_env_str: &str, channel: String, message: String) {
    if cfg!(debug_assertions) {
        info!("{}", message);
    } else {
        match env::var(webhook_env_str) {
            Ok(webhook_url) => {
                match Slack::new(webhook_url.as_str()) {
                    Ok(slack) => {
                        let payload = PayloadBuilder::new()
                            .text(message)
                            .channel(channel)
                            .username("Hooty Bot")
                            .icon_emoji(":chart_with_upwards_trend:")
                            .build()
                            .unwrap();

                        if let Err(err) = slack.send(&payload) {
                            log::error!("unable to send slack message: {}", err);
                        }
                    }
                    Err(err) => {
                        log::error!("unable to init slack: {}", err);
                    }
                };
            }
            Err(err) => {
                error!("Unable to parse {} env {}", webhook_env_str, err)
            }
        }
    }
}
