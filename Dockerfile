FROM rust:1.98.1-alpine AS builder

RUN apk add --no-cache musl-dev lld clang

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
COPY .cargo/config.toml .cargo/config.toml
COPY src ./src
COPY ./README.md ./README.md
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release --target x86_64-unknown-linux-musl && cp target/x86_64-unknown-linux-musl/release/seed /tmp/seed

FROM alpine:3.24

RUN addgroup -S seed && \
    adduser -S uni -G seed && \
    mkdir -p /server && \
    chown -R uni:seed /server

COPY --from=builder /tmp/seed /usr/local/bin/seed

USER uni

WORKDIR /server

ENTRYPOINT ["/usr/local/bin/seed"]
