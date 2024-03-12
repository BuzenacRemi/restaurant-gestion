FROM rust:latest as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src \
    && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs \
    && cargo build --release

RUN rm -f target/release/deps/website*

COPY . .

CMD ["apt-get", "update"]
CMD ["apt-get", "upgrade", "-y"]
CMD ["apt-get", "install", "libssl-dev", "-y"]

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /app/target/release/website /usr/local/bin/website

CMD ["/usr/local/bin/website"]