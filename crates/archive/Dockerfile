FROM rust:1 AS builder
COPY shared-net /shared-net
COPY archive /archive
WORKDIR /archive
RUN cargo build --release --bin archive

# We do not need the Rust toolchain to run the binary!
FROM debian:stable-slim AS runtime
WORKDIR /archive
COPY --from=builder /crates/archive/target/release/archive /usr/local/bin
ENTRYPOINT ["/usr/local/bin/archive"]
