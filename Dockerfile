# Use an official Rust image as the builder
FROM rust:latest AS builder

# Set the working directory for the Rust project
WORKDIR /app

# Copy Cargo files and fetch dependencies
COPY ["Cargo.toml", "Cargo.lock", "./"]
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy the actual source code
COPY ./src ./src
RUN cargo build --release

# Use a smaller image for deployment
FROM debian:buster-slim

# Install necessary libraries
RUN apt-get update && apt-get install -y \
    libssl-dev \
    libpq-dev

# Copy the compiled binary
COPY --from=builder /app/target/release/earn_vault /usr/local/bin/earn_vault

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/earn_vault"]

# Expose necessary ports
EXPOSE 8080