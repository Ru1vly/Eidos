# Multi-stage build for optimized Eidos image
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY lib_core/Cargo.toml lib_core/
COPY lib_chat/Cargo.toml lib_chat/
COPY lib_translate/Cargo.toml lib_translate/
COPY lib_bridge/Cargo.toml lib_bridge/

# Create dummy source files to cache dependencies
RUN mkdir -p src lib_core/src lib_chat/src lib_translate/src lib_bridge/src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > lib_core/src/lib.rs && \
    echo "pub fn dummy() {}" > lib_chat/src/lib.rs && \
    echo "pub fn dummy() {}" > lib_translate/src/lib.rs && \
    echo "pub fn dummy() {}" > lib_bridge/src/lib.rs

# Build dependencies (cached layer)
RUN cargo build --release && \
    rm -rf src lib_core/src lib_chat/src lib_translate/src lib_bridge/src target/release/deps/eidos* target/release/deps/lib_*

# Copy actual source code
COPY src ./src
COPY lib_core ./lib_core
COPY lib_chat ./lib_chat
COPY lib_translate ./lib_translate
COPY lib_bridge ./lib_bridge
COPY benches ./benches
COPY tests ./tests

# Build the application
RUN cargo build --release --bin eidos

# Runtime stage - minimal image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1000 eidos

WORKDIR /home/eidos

# Copy binary from builder
COPY --from=builder /app/target/release/eidos /usr/local/bin/eidos

# Copy example configuration
COPY eidos.toml.example ./eidos.toml.example

# Set ownership
RUN chown -R eidos:eidos /home/eidos

USER eidos

# Set environment variables
ENV EIDOS_MODEL_PATH=/home/eidos/model.onnx
ENV EIDOS_TOKENIZER_PATH=/home/eidos/tokenizer.json

# Create volume mount points for models
VOLUME ["/home/eidos/models"]

ENTRYPOINT ["eidos"]
CMD ["--help"]

# Labels
LABEL org.opencontainers.image.title="Eidos" \
      org.opencontainers.image.description="AI-powered CLI for Linux command generation" \
      org.opencontainers.image.version="0.1.0" \
      org.opencontainers.image.authors="EIDOS Team" \
      org.opencontainers.image.source="https://github.com/yourusername/eidos"
