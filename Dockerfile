FROM rustlang/rust:nightly-bookworm AS builder

WORKDIR /app

ARG APP_DEVELOPMENT=false
ENV APP_DEVELOPMENT=${APP_DEVELOPMENT}

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
COPY .sqlx .sqlx

RUN if [ "$APP_DEVELOPMENT" = "true" ] || [ "$APP_DEVELOPMENT" = "1" ] || [ "$APP_DEVELOPMENT" = "yes" ]; then \
        cargo build && cp target/debug/backend-v1 /app/backend-v1; \
    else \
        cargo build --release && cp target/release/backend-v1 /app/backend-v1; \
    fi

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/backend-v1 /app/backend-v1
COPY --from=builder /app/migrations /app/migrations

EXPOSE 3000

CMD ["/app/backend-v1"]
