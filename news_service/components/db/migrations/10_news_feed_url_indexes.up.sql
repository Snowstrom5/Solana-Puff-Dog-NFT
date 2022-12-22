create index news_feed_url_is_climate_related_index on news_feed_url (is_climate_related);
create index news_feed_url_created_at_index on news_feed_url (created_at);
create index news_feed_url_url_score_index on news_feed_url (url_score);
create index news_feed_url_first_referenced_by_index on news_feed_url (first_referenced_by);
