FROM rust:1.85-bookworm AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY apps/api ./apps/api
RUN cargo build --release -p kloudlab-api
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build /app/target/release/kloudlab-api /usr/local/bin/kloudlab-api
EXPOSE 8080
CMD ["/usr/local/bin/kloudlab-api"]
