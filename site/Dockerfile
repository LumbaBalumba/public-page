FROM rust:latest

WORKDIR /sites/public-page

COPY . .

RUN cargo install --path .
RUN cargo build

CMD ["cargo", "run", "--release"]
