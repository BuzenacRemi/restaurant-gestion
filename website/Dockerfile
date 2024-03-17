FROM rust:latest as builder
WORKDIR /usr/src/website
COPY . .
RUN cargo install --path .

FROM debian:latest
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/website /usr/local/bin/website
EXPOSE 8000
ENV ROCKET_ADDRESS="0.0.0.0"
CMD ["website"]