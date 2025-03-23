# Use an official Rust image
FROM rust:latest

# Set the working directory
WORKDIR /app

# Copy the Cargo files and dependencies separately for better caching
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

# Copy the rest of the app
COPY . .

# Build the application
RUN cargo build --release

# Expose the port (make sure it matches the port your app runs on)
EXPOSE 8000

# Run the binary
CMD ["./target/release/earn_vault"]