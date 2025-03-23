# Use an official Rust image as the base
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo files separately to leverage Docker's caching mechanism
COPY Cargo.toml Cargo.lock ./

# Create a dummy src directory to allow dependencies to be fetched
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Fetch dependencies and compile them to optimize caching
RUN cargo build --release

# Copy the actual source code
COPY . .

# Rebuild the application with the actual source code
RUN cargo build --release

# Use a smaller base image for deployment (reducing image size)
FROM debian:buster-slim

# Set the working directory inside the container
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/earn_vault .

# Expose the port (update this if your app runs on a different port)
EXPOSE 8000

# Run the compiled binary
CMD ["./earn_vault"]