FROM rust:1.87.0 as builder

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY Cargo.toml .
COPY src src

RUN cargo build --release

FROM debian:bookworm-slim as runtime

WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates libpq-dev \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zero2prod zero2prod
COPY conf conf

ENV APP_ENV prod

ENTRYPOINT ["./zero2prod"]
