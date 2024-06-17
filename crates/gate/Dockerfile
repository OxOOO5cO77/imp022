FROM rust:1 AS builder
COPY shared-net /shared-net
COPY gate /gate
WORKDIR /gate
RUN cargo build --release --bin gate

# We do not need the Rust toolchain to run the binary!
FROM debian:stable-slim AS runtime
WORKDIR /gate
COPY --from=builder /crates/gate/target/release/gate /usr/local/bin
ENTRYPOINT ["/usr/local/bin/gate"]
