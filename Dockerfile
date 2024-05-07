# Use Rust 1.77.2 as the builder base to compile the application
FROM rust:1.77.2 AS builder

# Set the working directory in the builder container
WORKDIR /usr/src/myapp

# Copy all local files into the container
COPY . .

# Compile the Rust application in release mode to optimize binary
RUN cargo build --release

# Start a new build stage using a smaller base image
FROM ubuntu:latest

# Metadata as label, maintainers or authors of the image
LABEL authors="ebyrdeu"

# Install ca-certificates for SSL verification, necessary if the app makes HTTPS requests
RUN apt-get update && apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Set environment variables for the application
ENV ADDRESS=0.0.0.0


# Set the working directory to /app
WORKDIR /app

# Copy the built binary from the builder stage to the /app directory
COPY --from=builder /usr/src/myapp/target/release/password_hash_salt .

# Define the entrypoint for running the application
ENTRYPOINT ["./password_hash_salt"]
