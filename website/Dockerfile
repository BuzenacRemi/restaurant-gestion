FROM rust:latest as builder
WORKDIR /usr/src/website
COPY . .
RUN cargo install --path .

FROM debian:latest
RUN apt-get update && apt install -y openssl && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/website /usr/local/bin/website

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/website/templates ./templates
COPY --from=builder /usr/src/website/assets ./assets

RUN chmod -R 755 ./templates
RUN chmod -R 755 ./assets

RUN ls -la /usr/local/bin

EXPOSE 8000

ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_SECRET_KEY="dtn2nzC/5BKC4PJz/c14R8v0BBsU3B1ji2NPJTlDxrM="

CMD ["./website"]