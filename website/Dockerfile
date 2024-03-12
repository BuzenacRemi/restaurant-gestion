FROM rust:latest as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src \
    && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs \
    && cargo build --release

RUN rm -f target/release/deps/website*

COPY . .

RUN cargo build --release

FROM debian:latest

COPY --from=builder /app/target/release/website /usr/local/bin/website

CMD ["/usr/local/bin/website"]
