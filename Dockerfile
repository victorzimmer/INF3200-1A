# Using official latest Rust image as base
FROM rust:latest

# Copy all files to Docker image
COPY ./ ./

# Build for release
RUN cargo build --release

# Run the binary
CMD ["./target/release/INF3200-1A"]

# Set address and port for Rocket.rs
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

# Expose the port
EXPOSE 8000/tcp
