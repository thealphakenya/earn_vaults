# Use an official Rust image as the builder
FROM rust:latest AS builder

# Set working directory
WORKDIR /app

# Install dependencies required for the build
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy Cargo files and fetch dependencies
COPY Cargo.toml ./
COPY Cargo.lock ./ || true  # Continue even if Cargo.lock is missing
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch && cargo build --release

# Copy source code and build the project
COPY ./src ./src
RUN cargo build --release

# Use a more stable Debian version for deployment
FROM debian:bullseye-slim

# Fix network issues: Change APT mirror and retry package installation if needed
RUN sed -i 's|http://deb.debian.org|http://ftp.us.debian.org|' /etc/apt/sources.list && \
    apt-get update && apt-get install -y --no-install-recommends libssl-dev libpq-dev || \
    (sleep 5 && apt-get install -y libssl-dev libpq-dev) && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=builder /app/target/release/earn_vault /usr/local/bin/earn_vault

# Expose the application's port
EXPOSE 8080

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/earn_vault"]