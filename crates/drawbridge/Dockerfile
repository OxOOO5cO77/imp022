FROM rust:1 AS builder
COPY shared-net /shared-net
COPY drawbridge /drawbridge
WORKDIR /drawbridge
RUN cargo build --release --bin drawbridge

# We do not need the Rust toolchain to run the binary!
FROM debian:stable-slim AS runtime
WORKDIR /opt/drawbridge
COPY --from=builder /drawbridge/target/release/drawbridge .
ENTRYPOINT ["./drawbridge"]
