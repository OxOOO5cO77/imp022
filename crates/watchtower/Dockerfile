FROM rust:1 AS builder
COPY shared-net /shared-net
COPY shared-data /shared-data
COPY watchtower /watchtower
WORKDIR /watchtower
RUN cargo build --release --bin watchtower

# We do not need the Rust toolchain to run the binary!
FROM debian:stable-slim AS runtime
WORKDIR /watchtower
COPY --from=builder /watchtower/target/release/watchtower /usr/local/bin
ENTRYPOINT ["/usr/local/bin/watchtower"]
