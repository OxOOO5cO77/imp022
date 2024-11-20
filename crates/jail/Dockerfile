FROM rust:1 AS builder
COPY shared-net /shared-net
COPY jail /jail
WORKDIR /jail
RUN cargo build --release --bin jail

# We do not need the Rust toolchain to run the binary!
FROM debian:stable-slim AS runtime
WORKDIR /opt/jail
COPY --from=builder /jail/target/release/jail .
ENTRYPOINT ["./jail"]
