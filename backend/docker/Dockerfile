# ============================================
# CFP Backend - Multi-stage Docker Build
# ============================================
# Builds a minimal production Docker image

# ============ STAGE 1: Builder ============
FROM rust:1.75-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    musl-tools \
    && rm -rf /var/lib/apt/lists/*

# Create musl target for static linking
RUN rustup target add x86_64-unknown-linux-musl

# Set working directory
WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./

# Create dummy source to cache dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    echo "// dummy lib" > src/lib.rs

# Build dependencies (cached layer)
RUN cargo build --release --target x86_64-unknown-linux-musl

# Remove dummy source
RUN rm -rf src

# Copy actual source code
COPY src ./src
COPY migrations ./migrations
COPY templates ./templates
COPY static ./static

# Build the application
RUN touch src/main.rs && \
    cargo build --release --target x86_64-unknown-linux-musl

# ============ STAGE 2: Runtime ============
FROM alpine:3.18 AS runtime

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    tzdata \
    libgcc \
    && update-ca-certificates

# Create non-root user
RUN addgroup -g 1000 -S cfp && \
    adduser -u 1000 -S cfp -G cfp

# Set timezone
ENV TZ=UTC

# Create necessary directories
RUN mkdir -p /app /data/uploads /data/logs /data/db && \
    chown -R cfp:cfp /app /data

# Switch to non-root user
USER cfp

# Set working directory
WORKDIR /app

# Copy built binary from builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/cfp-backend ./cfp-backend

# Copy configuration files
COPY --chown=cfp:cfp config/production.toml ./config/
COPY --chown=cfp:cfp .env.production ./.env

# Copy migrations
COPY --chown=cfp:cfp migrations ./migrations

# Copy SSL certificates (if any)
COPY --chown=cfp:cfp certs ./certs

# Create health check script
RUN echo '#!/bin/sh\n\
curl -f http://localhost:8080/health || exit 1' > /app/healthcheck.sh && \
    chmod +x /app/healthcheck.sh

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/app/healthcheck.sh"]

# Runtime environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=0
ENV APP_ENVIRONMENT=production
ENV DATABASE_URL=sqlite:///data/db/cfp.db
ENV UPLOAD_DIR=/data/uploads
ENV LOG_FILE=/data/logs/cfp.log

# Run migrations and start application
CMD ["sh", "-c", "./cfp-backend migrate && ./cfp-backend"]
