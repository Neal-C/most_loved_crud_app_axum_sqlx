## Following this documentation : https://docs.fl0.com/docs/builds/dockerfile/rust
##! Change the Rust version to the lastest !
## ! cargo build in release mode ! for much better performance
## ! Change app-name 'template-rust' to yours

# Leveraging the pre-built Docker images with
# cargo-chef and the Rust toolchain
FROM lukemathwalker/cargo-chef:latest-rust-1.72.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM rust:1.72-slim AS most_loved_crud_app
COPY --from=builder /app/target/release/most_loved_crud_app /usr/local/bin
ENTRYPOINT ["/usr/local/bin/most_loved_crud_app"]