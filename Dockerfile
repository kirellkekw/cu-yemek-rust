FROM rust:1.79.0 as builder
WORKDIR /app/
COPY . .
RUN cargo install --path .

FROM debian:bookworm
RUN apt-get update && apt install -y openssl ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/cukurova-yemekhane /usr/local/bin/cukurova-yemekhane
CMD ["cukurova-yemekhane"]