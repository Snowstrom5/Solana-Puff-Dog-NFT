CREATE TABLE news_cron_job (
    id                          SERIAL,
    started_at                  BIGINT          NOT NULL,
    started_at_str              TEXT            NOT NULL,
    completed_at                BIGINT,
    completed_at_str            TEXT,
    error                       TEXT
);



