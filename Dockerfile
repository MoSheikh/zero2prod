FROM rust:1.87.0

WORKDIR /app

COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release
RUN mv ./target/release/zero2prod run

COPY conf ./conf

ENV APP_ENV=prod

ENTRYPOINT ["./run"]
