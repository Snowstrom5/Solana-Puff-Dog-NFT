CREATE TABLE news_referenced_tweet (
    id                          SERIAL,
    tweet_id                    BIGINT      NOT NULL,
    referenced_tweet_id         BIGINT      NOT NULL,
    referenced_tweet_kind       TEXT        NOT NULL
);
create index news_referenced_tweet_tweet_id_index on news_referenced_tweet (tweet_id);
create index news_referenced_tweet_referenced_tweet_id_index on news_referenced_tweet (referenced_tweet_id);
create index news_referenced_tweet_referenced_tweet_kind_index on news_referenced_tweet (referenced_tweet_kind);

