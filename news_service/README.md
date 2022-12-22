# Backend services

### Running the services
```bash
cargo run --bin api
cargo run --bin cron
```
### running tests
```bash
# Setup a test database
export DATABASE_URL=postgres://climate_news:climate_news@localhost:5432/climate_news_test 
sqlx database drop -y && sqlx database create &&  sqlx migrate run
# all tests
export DATABASE_URL=postgres://climate_news:climate_news@localhost:5432/climate_news_test
cargo test -- --nocapture
# individual test
 cargo test --package cron --bin cron -- graphql::queries::news_feed_urls::tests::get_news_feed_urls_test --exact --nocapture 

```

## Pre build steps
### SQLX offline mode

```bash
# Generate a sqlx-data.json file
cargo clean && DATABASE_URL=postgres://climate_news:climate_news@localhost:5432/climate_news cargo sqlx prepare --merged
```
#### Setting up the database
```sh
# Sqlx CLI
cargo install sqlx-cli 

# create database & run migrations
cd components/db
sqlx database create --database-url postgres://climate_news:climate_news@localhost:5432/climate_news
sqlx migrate run --database-url postgres://climate_news:climate_news@localhost:5432/climate_news
sqlx migrate revert --database-url postgres://climate_news:climate_news@localhost:5432/climate_news
sqlx database drop --database-url postgres://climate_news:climate_news@localhost:5432/climate_news

```
