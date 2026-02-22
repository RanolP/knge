FROM rust:1.89-bookworm AS base
WORKDIR /workspace

COPY Cargo.toml ./
COPY apps/api/Cargo.toml apps/api/Cargo.toml
COPY apps/api/src apps/api/src

RUN cargo build --release -p api

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=base /workspace/target/release/api /usr/local/bin/api
EXPOSE 3000
CMD ["api"]
