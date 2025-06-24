ARG IMAGE=rust:1.87.0

FROM $IMAGE as builder

WORKDIR /app

COPY Cargo.toml .
COPY src src

RUN cargo build --release

FROM $IMAGE as runtime

WORKDIR /app

COPY --from=builder /app/target/release/zero2prod zero2prod
COPY conf conf

ENV APP_ENV prod

ENTRYPOINT ["./zero2prod"]
