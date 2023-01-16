use log::{error, info};
use slack_hook::{PayloadBuilder, Slack};
use std::env;

static TWEET_CRON_CHANNEL: &str = "#tweet-cron";
static MAIN_CRON_CHANNEL: &str = "#main-cron";

pub fn send_tweet_cron_message(message: String) {
    match env::var("TWEET_CRON_WEBHOOK_URL") {
        Ok(webhook_url) => {
            send_message(webhook_url, TWEET_CRON_CHANNEL.to_string(), message);
        }
        Err(err) => {
            error!(
                "Unable to parse TWEET_CRON_WEBHOOK_URL env variable {}",
                err
            )
        }
    }
}

pub fn send_main_cron_message(message: String) {
    match env::var("MAIN_CRON_WEBHOOK_URL") {
        Ok(webhook_url) => {
            send_message(webhook_url, MAIN_CRON_CHANNEL.to_string(), message);
        }
        Err(err) => {
            error!("Unable to parse MAIN_CRON_WEBHOOK_URL env variable {}", err)
        }
    }
}

fn send_message(webhook_url: String, channel: String, message: String) {
    if cfg!(debug_assertions) {
        info!("[{}]", message);
    } else {
        info!("{}", message);
        match Slack::new(webhook_url.as_str()) {
            Ok(slack) => {
                let payload = PayloadBuilder::new()
                    .text(message)
                    .channel(channel)
                    .username("Slack Bot")
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
}
