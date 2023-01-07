# Climate News - Services

[![web_push](https://github.com/climatenews/services/actions/workflows/news_service_web_push.yml/badge.svg)](https://github.com/climatenews/services/actions/workflows/news_service_web_push.yml) [![cron_push](https://github.com/climatenews/services/actions/workflows/news_service_cron_push.yml/badge.svg)](https://github.com/climatenews/services/actions/workflows/news_service_cron_push.yml) [![api_push](https://github.com/climatenews/services/actions/workflows/news_service_api_push.yml/badge.svg)](https://github.com/climatenews/services/actions/workflows/news_service_api_push.yml)


## Overview
`devops` - Terraform & Ansible deployment scripts

`news_service` - Rust Cron and API services

`web` - Next.js frontend

## Running locally
### Prerequisites
- Docker & Docker Compose
- [Twitter API key](https://developer.twitter.com/en/docs/authentication/oauth-2-0/bearer-tokens)
- [OpenAI API key](https://openai.com/api/)

### Setting up the .env.dev file
```bash
# copy the sample .env file 
cp .env.sample .env.dev
```
Set the `OPENAI_API_KEY` & `TWITTER_BEARER_TOKEN` variables in `.env.dev`

### Test the app with Docker Compose
```bash
# start docker
service docker start

# Start the app
docker-compose --env-file ".env.dev" up -d --build 

# tail the logs
docker-compose logs --tail="all" -f
```

## Deploying
### Deploy the stack with Docker Swarm
```bash
# Initialize a docker swarm
docker swarm init 

# Deploy the stack
env $(cat .env.dev | xargs) docker stack deploy --compose-file docker-compose.yaml climate_news_stack 

# Display stack info
docker stack ps climate_news_stack

# Display service info
docker service ps climate_news_stack_news_cron

# Inspect a service
docker service inspect --pretty climate_news_stack_news_cron

# Restart a service
docker service update --force climate_news_stack_news_cron

# Run a command in a container
docker container ls
docker exec -it f855a1118d35 /bin/bash

# Logs for a service
docker service logs -f --since 1h climate_news_stack_db
docker service logs -f --since 1h climate_news_stack_news_api
docker service logs -f --since 1h climate_news_stack_news_cron
docker service logs -f --since 1h climate_news_stack_web
docker service logs -f --since 1h climate_news_stack_caddy

# Remove the stack
docker stack rm climate_news_stack

```

# Triggering a new Docker image build
```bash

git tag -a v0.0.38 -m "ui fixes" && git push origin v0.0.38

```