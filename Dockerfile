FROM rust:1.76.0 as builder
WORKDIR /usr/src/doc-status
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src
RUN cargo install --path . --bin doc-status --target-dir .
FROM debian:bookworm-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/doc-status /usr/local/bin/doc-status
WORKDIR /usr/src/doc-status
COPY entrypoint.sh entrypoint.sh
ENTRYPOINT ["./entrypoint.sh"]
