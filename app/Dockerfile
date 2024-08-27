FROM rust:latest as builder

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml ./
COPY src src
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/public-webpage /usr/local/bin/public-webpage
COPY static static
COPY templates templates
COPY Rocket.toml Rocket.toml

#ENV ROCKET_ADDRESS=0.0.0.0
#ENV ROCKET_PORT=8080
#ENV ROCKET_LOG_LEVEL="debug"

CMD ["public-webpage"]
