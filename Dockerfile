# Use a slim Rust image to reduce memory usage during the build process
FROM rust:slim AS builder

# Set working directory
WORKDIR /app

# Install required dependencies for building and Railway CLI
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    curl \
    sudo

# Install Railway CLI via Cargo
RUN cargo install railwayapp --locked

# Verify Railway CLI installation
RUN /root/.cargo/bin/railway --version

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
    apt-get install -y --no-install-recommends \
    libssl-dev \
    libpq-dev \
    curl \
    sudo && \
    rm -rf /var/lib/apt/lists/*

# Install Railway CLI via Cargo in the final stage
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    source $HOME/.cargo/env && \
    cargo install railwayapp --locked

# Verify Railway CLI installation in the final stage
RUN /root/.cargo/bin/railway --version

# Set working directory in the runtime container
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/earn_vault /usr/local/bin/earn_vault

# Check if .env file exists, if not, create it with default values
COPY .env .env

# Ensure .env file exists and add default variables if missing
RUN if [ ! -f .env ]; then \
        echo "DATABASE_URL=postgres://user:password@host:5432/dbname" > .env && \
        echo "API_KEY=your-default-api-key" >> .env && \
        echo "SECRET_KEY=your-secret" >> .env && \
        echo "PORT=8080" >> .env; \
    fi

# Load environment variables from .env file
ENV $(cat .env | xargs)

# Expose the application's port
EXPOSE $PORT

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/earn_vault"]