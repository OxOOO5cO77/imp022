FROM rust:1 AS builder
COPY shared-net /shared-net
COPY hall /hall
WORKDIR /hall
RUN cargo build --release --bin hall

# We do not need the Rust toolchain to run the binary!
FROM debian:stable-slim AS runtime
WORKDIR /hall
COPY --from=builder /hall/target/release/hall /usr/local/bin
ENTRYPOINT ["/usr/local/bin/hall"]
