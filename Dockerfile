# Use an official Rust image for building
FROM rust:latest AS builder

# Set the working directory
WORKDIR /app

# Ensure the Docker build context includes Cargo.lock
COPY ./Cargo.toml ./Cargo.lock ./

# Create a temporary src directory to allow dependency caching
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Fetch dependencies and compile them for caching
RUN cargo build --release

# Remove the dummy source code and copy the actual source code
RUN rm -r src
COPY . .

# Rebuild the application with the actual source code
RUN cargo build --release

# Use a smaller base image for deployment
FROM debian:buster-slim

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/earn_vault .

# Ensure the binary has execution permissions
RUN chmod +x /app/earn_vault

# Expose the application port
EXPOSE 8000

# Run the compiled binary
CMD ["./earn_vault"]