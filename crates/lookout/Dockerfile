FROM rust:1 AS builder
COPY shared-net /shared-net
COPY shared-data /shared-data
COPY lookout /lookout
WORKDIR /lookout
RUN cargo build --release --bin lookout

# We do not need the Rust toolchain to run the binary!
FROM debian:stable-slim AS runtime
WORKDIR /opt/lookout
COPY --from=builder /lookout/target/release/lookout .
ENTRYPOINT ["./lookout"]
