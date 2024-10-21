FROM rust:1 AS builder
COPY shared-net /shared-net
COPY bazaar /bazaar
WORKDIR /bazaar
RUN cargo build --release --bin bazaar

# We do not need the Rust toolchain to run the binary!
FROM debian:stable-slim AS runtime
WORKDIR /opt/bazaar
COPY --from=builder /bazaar/target/release/bazaar .
ENTRYPOINT ["./bazaar"]
