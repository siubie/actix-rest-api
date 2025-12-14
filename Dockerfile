# Multi-stage build for optimized production image

# Build stage
FROM rust:latest AS builder

# Create a new empty shell project
WORKDIR /app

# Copy manifests
COPY Cargo.toml ./
COPY Cargo.lock* ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY src ./src

# Build the actual application
RUN touch src/main.rs && \
    cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    curl && \
    rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1000 app

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/hello-actix /app/hello-actix

# Copy migrations
COPY migrations ./migrations

# Change ownership
RUN chown -R app:app /app

# Switch to app user
USER app

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the binary
CMD ["/app/hello-actix"]
