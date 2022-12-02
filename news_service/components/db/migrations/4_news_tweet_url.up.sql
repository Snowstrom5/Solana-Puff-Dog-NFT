CREATE TABLE news_tweet_url (
    id                          SERIAL,
    url                         TEXT        NOT NULL,
    expanded_url                TEXT        NOT NULL,
    expanded_url_parsed         TEXT        NOT NULL UNIQUE,
    expanded_url_host           TEXT        NOT NULL,
    display_url                 TEXT        NOT NULL,
    is_twitter_url              BOOLEAN     NOT NULL,
    is_english                  BOOLEAN     NOT NULL,
    title                       TEXT        NOT NULL,
    description                 TEXT        NOT NULL,
    preview_image_thumbnail_url TEXT,
    preview_image_url           TEXT,    
    created_at                  BIGINT      NOT NULL,
    created_at_str              TEXT        NOT NULL
);


