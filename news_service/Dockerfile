# Examples:
# https://www.lpalmieri.com/posts/fast-rust-docker-builds
# https://windsoilder.github.io/writing_dockerfile_in_rust_project.html

# Leveraging the pre-built Docker images with 
# cargo-chef and the Rust toolchain

FROM lukemathwalker/cargo-chef:latest-rust-1.59.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
ARG SERVICE_NAME
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN SQLX_OFFLINE=true cargo build --release --bin ${SERVICE_NAME}

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS runtime

ARG SERVICE_NAME
ENV SERVICE_NAME=${SERVICE_NAME}

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates
RUN update-ca-certificates
# curl is used for docker health checks
RUN apt-get install -y curl 

WORKDIR /app
COPY --from=builder /app/target/release/${SERVICE_NAME} /usr/local/bin
ENTRYPOINT /usr/local/bin/$SERVICE_NAME


