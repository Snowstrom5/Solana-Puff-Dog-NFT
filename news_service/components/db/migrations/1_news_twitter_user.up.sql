CREATE TABLE news_twitter_user (
    id                              SERIAL,
    user_id                         BIGINT      NOT NULL UNIQUE,
    username                        TEXT        NOT NULL UNIQUE,
    profile_image_url               TEXT,
    description                     TEXT,
    verified                        BOOLEAN,
    followers_count                 INTEGER     NOT NULL,
    listed_count                    INTEGER     NOT NULL,
    user_referenced_tweets_count    INTEGER,
    user_score                      INTEGER,
    last_tweet_id                   BIGINT,  
    last_updated_at                 BIGINT      NOT NULL,
    last_checked_at                 BIGINT      NOT NULL
);



