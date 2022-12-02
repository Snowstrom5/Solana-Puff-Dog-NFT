use db::models::news_twitter_user::NewsTwitterUser;

pub fn calc_user_score(
    news_twitter_user: &NewsTwitterUser,
    user_referenced_tweets_count: i32,
) -> i32 {
    let followers_count = news_twitter_user.followers_count;

    // Followers
    let followers_score = if followers_count > 1000000 {
        300
    } else if followers_count > 100000 {
        250
    } else if followers_count > 10000 {
        200
    } else if followers_count > 1000 {
        150
    } else {
        100
    };
    // Listed
    let listed_count = news_twitter_user.listed_count;
    let listed_score = if listed_count > 5000 {
        250
    } else if listed_count > 1000 {
        200
    } else if listed_count > 500 {
        150
    } else {
        100
    };

    // Tweet References
    let user_referenced_tweets_score = if user_referenced_tweets_count > 100 {
        300
    } else if user_referenced_tweets_count > 75 {
        250
    } else if user_referenced_tweets_count > 50 {
        200
    } else if user_referenced_tweets_count > 25 {
        150
    } else {
        100
    };
    followers_score + listed_score + user_referenced_tweets_score
}
