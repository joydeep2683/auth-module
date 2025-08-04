# ----------- Stage 1: Build -----------
FROM rust:1.85-bookworm AS builder
WORKDIR /app

# Copy manifests and build dependencies first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -r src

# Copy source and build final binary
COPY . .
RUN cargo build --release

# ----------- Stage 2: Runtime -----------
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
RUN useradd -m appuser

# 👇 IMPORTANT: Binary name must match package name
COPY --from=builder /app/target/release/auth-module /usr/local/bin/auth-module

USER appuser
EXPOSE 8080

CMD ["auth-module"]
