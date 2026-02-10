FROM rust:1.92-slim AS chef
RUN cargo install cargo-chef
WORKDIR /app

# Dependency analysis, only rebuilds when manifests change.
FROM chef as planner
COPY services/backend/ .
RUN cargo chef prepare --recipe-path recipe.json --bin portfolio-service

# Compile first the dependencies (cached unless manifests change), then the app.
FROM chef as builder
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --locked

COPY services/backend/ .
RUN cargo build --release --bin portfolio-service --locked

# Minimal runtime image
FROM debian:bookworm-slim

# Needed for HTTPS/TLS
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a system user, so the app doesnt run as root.
RUN groupadd -r appuser && useradd -r -g appuser appuser

WORKDIR /app

COPY --from=builder /app/target/release/portfolio-service .

RUN chown -R appuser:appuser /app

USER appuser

EXPOSE 8080

CMD ["./portfolio-service"]