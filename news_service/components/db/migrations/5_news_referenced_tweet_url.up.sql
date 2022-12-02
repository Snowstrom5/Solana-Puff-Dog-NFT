CREATE TABLE news_referenced_tweet_url (
    id                SERIAL,
    tweet_id          BIGINT        NOT NULL,
    url_id            INTEGER       NOT NULL
);
create index news_referenced_tweet_url_tweet_id_index on news_referenced_tweet_url (tweet_id);

