FROM rust:1.90-bookworm AS builder
WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/teamy-github-readme-stats /usr/local/bin/teamy-github-readme-stats

EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/teamy-github-readme-stats"]
CMD ["serve", "--listen", "0.0.0.0:8080"]
