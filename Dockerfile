# Use an official Rust image as the builder
FROM rust:latest AS builder

# Set the working directory for the Rust project
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files into the container
COPY ["Cargo.toml", "Cargo.lock", "./"]

# Create a temporary src directory to allow dependency caching
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies to cache them
RUN cargo build --release

# Now copy the rest of the source code
COPY ./src ./src

# Build the actual project
RUN cargo build --release

# Use a smaller image for the final image
FROM debian:buster-slim

# Install necessary libraries (if any)
RUN apt-get update && apt-get install -y \
    libssl-dev \
    libpq-dev

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/earn_vault /usr/local/bin/earn_vault

# Set the entrypoint for the container
ENTRYPOINT ["/usr/local/bin/earn_vault"]

# Expose any required ports (if necessary)
EXPOSE 8080
