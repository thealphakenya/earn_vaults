# Use an official Rust image for building
FROM rust:latest AS builder

# Set working directory
WORKDIR /app

# Install required dependencies with retries
RUN apt-get update && \
    apt-get install -y --no-install-recommends pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy Cargo files first to cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Fetch dependencies separately (reduces unnecessary builds)
RUN cargo fetch && cargo build --release

# Copy actual source code and build the project
COPY ./src ./src
RUN cargo build --release && strip target/release/earn_vault

# Use a smaller base image for deployment
FROM debian:buster-slim

# Set non-interactive mode for installation (prevents hangs)
ARG DEBIAN_FRONTEND=noninteractive

# Use a faster mirror to fix package fetching issues
RUN sed -i 's|http://deb.debian.org|http://ftp.us.debian.org|' /etc/apt/sources.list

# Install runtime dependencies with retry logic
RUN apt-get update && \
    apt-get install -y --no-install-recommends libssl-dev libpq-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary and check if it exists
COPY --from=builder /app/target/release/earn_vault /usr/local/bin/earn_vault
RUN if [ ! -f /usr/local/bin/earn_vault ]; then echo "ERROR: Binary not found!" && exit 1; fi

# Expose port (Railway auto-assigns)
EXPOSE 8080

# Ensure proper execution permissions
RUN chmod +x /usr/local/bin/earn_vault

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/earn_vault"]