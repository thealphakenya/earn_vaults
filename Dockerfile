# Use an official Rust image as the base for building
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy Cargo files first to leverage Docker's caching mechanism
COPY Cargo.toml Cargo.lock ./

# Ensure Cargo.lock exists before proceeding
RUN test -f Cargo.lock || (echo "Cargo.lock not found!" && exit 1)

# Create a dummy src directory to allow dependencies to be fetched
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Fetch dependencies and build to optimize caching
RUN cargo build --release

# Copy the actual source code
COPY . .

# Rebuild the application with actual source
RUN cargo build --release

# Use a smaller base image for deployment
FROM debian:buster-slim

# Set working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/earn_vault .

# Expose the necessary port
EXPOSE 8000

# Command to run the application
CMD ["./earn_vault"]