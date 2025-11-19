# TileMania - Multi-stage Docker build
# Builds a minimal Docker image for TileMania

# Stage 1: Builder
FROM rust:1.70 as builder

# Install system dependencies for Bevy
RUN apt-get update && apt-get install -y \
    libasound2-dev \
    libudev-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY assets ./assets

# Build release binary
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libasound2 \
    libudev1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 tilemania

# Create app directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/tilemania /app/tilemania

# Copy assets
COPY --from=builder /app/assets /app/assets

# Set permissions
RUN chown -R tilemania:tilemania /app

# Switch to non-root user
USER tilemania

# Expose port (if needed for future web features)
EXPOSE 8080

# Health check (optional)
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD test -f /app/tilemania || exit 1

# Run the game
CMD ["./tilemania"]
