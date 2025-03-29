# Use a slim Rust image to reduce memory usage during the build process
FROM rust:slim AS builder

# Set working directory
WORKDIR /app

# Install required dependencies for building
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev

# Copy Cargo files and fetch dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

# Copy source code and build the project
COPY . .
RUN cargo build --release

# Use a smaller, more stable Debian version for deployment
FROM debian:bullseye-slim

# Fix network issues, optimize APT mirrors, and install required runtime dependencies
RUN sed -i 's|http://deb.debian.org|http://ftp.us.debian.org|' /etc/apt/sources.list && \
    apt-get update && \
    apt-get install -y --no-install-recommends libssl-dev libpq-dev && \
    rm -rf /var/lib/apt/lists/*

# Set working directory in the runtime container
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/earn_vault /usr/local/bin/earn_vault

# Ensure .env file exists and add default variables if missing
# If .env is present, copy it; otherwise, create it with default values
COPY .env .env || echo "DATABASE_URL=postgres://user:password@host:5432/dbname" > .env

# Alternatively, ensure that the .env file is created with default values
RUN touch .env && \
    echo "DATABASE_URL=${DATABASE_URL:-postgres://user:password@host:5432/dbname}" >> .env && \
    echo "API_KEY=${API_KEY:-your-default-api-key}" >> .env && \
    echo "SECRET_KEY=${SECRET_KEY:-your-secret}" >> .env && \
    echo "PORT=${PORT:-8080}" >> .env

# Load environment variables from .env file
ENV $(cat .env | xargs)

# Expose the application's port
EXPOSE $PORT

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/earn_vault"]