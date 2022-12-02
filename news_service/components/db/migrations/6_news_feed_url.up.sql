CREATE TABLE news_feed_url (
    id                      SERIAL,
    url_id                  INTEGER     NOT NULL UNIQUE,  
    url_score               INTEGER     NOT NULL,  
    num_references          INTEGER     NOT NULL,
    first_referenced_by     BIGINT      NOT NULL,
    is_climate_related      BOOLEAN,
    created_at              BIGINT      NOT NULL,
    created_at_str          TEXT        NOT NULL
);



