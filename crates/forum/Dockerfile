FROM rust:1 AS builder
COPY shared-net /shared-net
COPY shared-data /shared-data
COPY forum /forum
WORKDIR /forum
RUN cargo build --release --bin forum

# We do not need the Rust toolchain to run the binary!
FROM debian:stable-slim AS runtime
WORKDIR /opt/forum
COPY --from=builder /forum/target/release/forum .
ENTRYPOINT ["./forum"]
