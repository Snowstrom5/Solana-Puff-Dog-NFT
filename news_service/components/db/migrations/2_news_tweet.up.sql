CREATE TABLE news_tweet (
    id                          SERIAL,
    tweet_id                    BIGINT          NOT NULL UNIQUE,
    text                        TEXT            NOT NULL,
    author_id                   BIGINT          NOT NULL,
    conversation_id             BIGINT,
    in_reply_to_user_id         BIGINT,
    created_at                  BIGINT          NOT NULL,
    created_at_str              TEXT            NOT NULL
);


