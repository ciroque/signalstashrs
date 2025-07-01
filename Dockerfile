# ---- Build Stage ----
FROM rust:1.88 AS builder

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# Install protoc for prost-build
RUN apt-get update && apt-get install -y --no-install-recommends protobuf-compiler ca-certificates && rm -rf /var/lib/apt/lists/*

# Build actual source
COPY . .
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:bookworm-slim

# Install only necessary system dependencies (ca-certificates for HTTPS, openssl if needed)
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/signalstashrs /app/signalstashrs

# Set environment variables (edit as needed)
ENV RUST_LOG=info

# Expose the port your app listens on (change if not 20120)
EXPOSE 20120

# Run the binary
CMD ["./signalstashrs"]
