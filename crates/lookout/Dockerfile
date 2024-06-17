FROM rust:1 AS builder
COPY shared-net /shared-net
COPY lookout /lookout
WORKDIR /lookout
RUN cargo build --release --bin lookout

# We do not need the Rust toolchain to run the binary!
FROM debian:stable-slim AS runtime
WORKDIR /lookout
COPY --from=builder /crates/lookout/target/release/lookout /usr/local/bin
ENTRYPOINT ["/usr/local/bin/lookout"]
