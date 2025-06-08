FROM rust:latest AS builder

WORKDIR /app

RUN cargo init
COPY Cargo.toml Cargo.lock /app/

RUN cargo new --lib /app/plugin_api
COPY plugin_api/Cargo.toml /app/plugin_api/

RUN cargo new --lib /app/src/crates/macros
COPY src/crates/macros /app/src/crates/

RUN cargo new --lib /app/src/crates/plugin_system
COPY src/crates/plugin_system/Cargo.toml /app/src/crates/plugin_system

RUN cargo new --lib /app/src/crates/server
COPY src/crates/server/Cargo.toml /app/src/crates/server

RUN cargo new --lib /app/src/crates/services
COPY src/crates/services/Cargo.toml /app/src/crates/services

RUN cargo new --lib /app/src/crates/shared
COPY src/crates/shared/Cargo.toml /app/src/crates/shared

RUN cargo new --lib /app/src/crates/usecases
COPY src/crates/usecases/Cargo.toml /app/src/crates/usecases

RUN apt-get update && apt-get install -y mingw-w64 \
    && rustup target add x86_64-pc-windows-gnu

RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --target x86_64-pc-windows-gnu

COPY ./plugin_api /app/plugin_api
COPY ./src /app/src

RUN --mount=type=cache,target=/usr/local/cargo/registry /bin/bash -c 'set -e; \
  touch /app/plugin_api/src/lib.rs; \
  touch /app/src/crates/macros/src/lib.rs; \
  touch /app/src/crates/plugin_system/src/lib.rs; \
  touch /app/src/crates/server/src/lib.rs; \
  touch /app/src/crates/services/src/lib.rs; \
  touch /app/src/crates/shared/src/lib.rs; \
  touch /app/src/crates/usecases/src/lib.rs; \
  touch /app/src/main.rs; \
  cargo build --target x86_64-pc-windows-gnu'

FROM debian:latest AS exporter
COPY --from=builder /app/target/x86_64-pc-windows-gnu/debug/asya.exe /asya.exe
CMD ["cp", "/asya.exe", "/output/asya.exe"]