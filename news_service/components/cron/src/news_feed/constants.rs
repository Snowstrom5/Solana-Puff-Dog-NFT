pub static MAX_TWEET_RESULTS: usize = 100;

// User Tweet timeline limit: 1500/per 15m
pub static REQUEST_SLEEP_DURATION: u64 = 1500;

pub fn twitter_lists() -> Vec<i64> {
    if cfg!(debug_assertions) {
        // 1 list in debug mode
        vec![
            1586920047964205057, // Climate News - @climatenews_app - https://twitter.com/i/lists/1586920047964205057 - 59 Members
        ]
    } else {
        // 3 lists in release mode
        vec![
            1586920047964205057, // Climate News - @climatenews_app - https://twitter.com/i/lists/1586920047964205057 - 59 Members
            1053067173961326594, // scientists who do climate - @KHayhoe - https://twitter.com/i/lists/1053067173961326594 - 3.1K Members
            1308140854524162059, // Tweets about climate change from journalists, policy specialists, and institutions. https://twitter.com/i/lists/1308140854524162059 - 231 Members
        ]
    }
}
