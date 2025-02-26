# ---- Stage 1: Build ----
FROM rust:1.76 as builder

# Install MUSL tools for static linking
RUN apt-get update && apt-get install -y musl-tools

# Set working directory
WORKDIR /app

# Set Rust target to MUSL (static binary)
RUN rustup target add x86_64-unknown-linux-musl

# Copy source code and dependencies
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the Rust binary with static linking
RUN cargo build --release --target=x86_64-unknown-linux-musl

# ---- Stage 2: Minimal Runtime ----
FROM scratch  # Completely empty base image

# Set working directory
WORKDIR /app

# Copy the compiled static binary from builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/fibbot /app/fibbot

# Set the default command to execute the binary
CMD ["/app/fibbot"]
