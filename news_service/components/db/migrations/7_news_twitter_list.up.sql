CREATE TABLE news_twitter_list (
    id                              SERIAL,
    list_id                         BIGINT      NOT NULL UNIQUE,
    last_checked_at                 BIGINT      NOT NULL
);



