FROM rust:latest AS builder

WORKDIR /usr/src/fibbot
RUN mkdir -p /usr/src/fibbot

COPY . .

RUN apt-get update && apt-get install -y build-essential
RUN cargo build --release
RUN ls -l /usr/src/fibbot/target/release/
