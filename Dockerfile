# Use Rust 1.77.2 as the builder base to compile the application
FROM rust:1.77.2 AS builder

# Copy all local files into the container
COPY ./ .

# Compile the Rust application in release mode to optimize binary
RUN cargo build --release

# List the contents of the build directory to verify successful build
RUN ls -l /target/release

# Start a new build stage using Ubuntu as the base to create a smaller final image
FROM ubuntu:latest

# Metadata as label, maintainers or authors of the image
LABEL authors="ebyrdeu"

# Copy the built binary from the builder stage to the /app directory
COPY --from=builder /target/release/password_hash_salt /app/password_hash_salt

# Install ca-certificates for SSL verification, necessary if the app makes HTTPS requests
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set environment variables for the application
ARG PORT=8810
ARG ADDRESS=0.0.0.0
ENV PORT=${PORT}
ENV ADDRESS=${ADDRESS}

# Set the working directory to /app
WORKDIR /app

# Ensure the application binary is executable
RUN chmod +x /app/password_hash_salt

# Define the entrypoint for running the application
ENTRYPOINT ["/app/password_hash_salt"]
