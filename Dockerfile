# Use an official Rust image as the builder
FROM rust:latest AS builder

# Set working directory
WORKDIR /app

# Install dependencies required for build
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy Cargo files and fetch dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch && cargo build --release

# Copy source code and build the project
COPY ./src ./src
RUN cargo build --release

# Use a smaller image for deployment
FROM debian:buster-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y libssl-dev libpq-dev && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=builder /app/target/release/earn_vault /usr/local/bin/earn_vault

# Railway automatically assigns a port
EXPOSE 8080

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/earn_vault"]
