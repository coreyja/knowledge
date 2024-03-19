FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
RUN rustc --version; cargo --version; rustup --version

COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!

RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

RUN cargo build --release --locked

# Start building the final image
FROM debian:stable-slim as final
WORKDIR /app

COPY --from=builder /app/target/release/knowledge .

EXPOSE 3000

ENTRYPOINT ["./knowledge"]
