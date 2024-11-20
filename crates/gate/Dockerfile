FROM rust:1 AS builder
COPY shared-net /shared-net
COPY gate /gate
WORKDIR /gate
RUN cargo build --release --bin gate

# We do not need the Rust toolchain to run the binary!
FROM debian:stable-slim AS runtime
WORKDIR /opt/gate
COPY --from=builder /gate/target/release/gate .
ENTRYPOINT ["./gate"]
