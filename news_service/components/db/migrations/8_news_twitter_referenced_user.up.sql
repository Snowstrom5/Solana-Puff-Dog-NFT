CREATE TABLE news_twitter_referenced_user (
    id                              SERIAL,
    user_id                         BIGINT      NOT NULL UNIQUE,
    username                        TEXT        NOT NULL UNIQUE
);



