FROM rust:1.72.0 as builder

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./
COPY src src
COPY static static
COPY templates templates
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.20

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/public-webpage /usr/local/bin/public-webpage
COPY --from=builder /app/static /app/static
COPY --from=builder /app/templates /app/templates

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
ENV ROCKET_LOG_LEVEL="debug"


CMD ["public-webpage"]