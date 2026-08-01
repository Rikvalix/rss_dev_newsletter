FROM rust:1.97-alpine as builder

RUN apk add --no-cache musl-dev ca-certificates

WORKDIR /app

COPY ./src ./src
COPY ./Cargo.lock .
COPY ./Cargo.toml .

RUN cargo build --release

FROM scratch as runner

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

COPY --from=builder /app/target/release/rss_dev_newsletter /server

EXPOSE 8080

ENTRYPOINT ["/server"]