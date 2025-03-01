# Stage 1: Build the Rust binary
FROM rust:latest AS builder

WORKDIR /fibbot

COPY . .

RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /fibbot/target/release/fibbot /fibbot

ENTRYPOINT ["/fibbot"]
