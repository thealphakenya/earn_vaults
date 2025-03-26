# Use an official Rust image as the builder
FROM rust:latest AS builder

# Set the working directory for the Rust project
WORKDIR /app

# Install dependencies required for build
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy Cargo files and fetch dependencies separately to leverage Docker caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch && cargo build --release

# Copy the actual source code and build the project
COPY ./src ./src
RUN cargo build --release

# Use a smaller image for deployment
FROM debian:buster-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    libpq-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/earn_vault /usr/local/bin/earn_vault

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/earn_vault"]

# Expose necessary ports
EXPOSE 8080