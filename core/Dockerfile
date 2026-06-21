# Build stage
FROM rust:1-bookworm AS builder
WORKDIR /app

COPY . .
RUN cargo build --release

# Production stage
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/quantaura /app/quantaura

RUN mkdir -p /app/data

EXPOSE 8000
CMD ["./quantaura"]
