# Use a slim Rust image to reduce memory usage during the build process
FROM rust:slim AS builder

# Set working directory
WORKDIR /app

# Install required dependencies for building
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev libpq-dev

# Ensure Cargo files exist before copying
COPY Cargo.toml .  
RUN [ -f Cargo.lock ] || touch Cargo.lock  
COPY Cargo.lock .  

# Fetch dependencies (caching step)
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

# Copy source code and build the project
COPY . .  
RUN cargo build --release --verbose  

# Reduce binary size
RUN strip target/release/earn_vault

# Use a minimal Debian image for deployment
FROM debian:bullseye-slim

# Optimize APT mirrors and install runtime dependencies using a Docker mirror
RUN mkdir -p /etc/docker && \
    echo '{ "registry-mirrors": ["https://mirror.gcr.io"] }' > /etc/docker/daemon.json && \
    sed -i 's|http://deb.debian.org|http://mirror.gcr.io/debian-security|' /etc/apt/sources.list && \
    apt-get update && \
    apt-get install -y --no-install-recommends libssl-dev libpq-dev && \
    rm -rf /var/lib/apt/lists/*

# Set working directory in the runtime container
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/earn_vault /usr/local/bin/earn_vault

# Expose the service port
EXPOSE 8080

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/earn_vault"]