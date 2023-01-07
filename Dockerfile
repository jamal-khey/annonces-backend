FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner

COPY --from=builder /usr/local/cargo/bin/rust-server-bin /usr/local/bin/rust-server-bin
COPY --from=builder /app/.env .env
COPY --from=builder /app/Rocket.toml Rocket.toml

EXPOSE 8080

CMD ["rust-server-bin"]