FROM rust:1 AS builder
COPY shared-net /shared-net
COPY shared-data /shared-data
COPY warehouse /warehouse
WORKDIR /warehouse
RUN cargo build --release --bin warehouse

# We do not need the Rust toolchain to run the binary!
FROM debian:stable-slim AS runtime
WORKDIR /opt/warehouse
COPY --from=builder /warehouse/target/release/warehouse .
COPY --from=builder /warehouse/assets/data ./assets/data
ENTRYPOINT ["./warehouse"]
