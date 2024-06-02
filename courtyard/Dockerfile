FROM rust:1 AS builder
COPY shared-net /shared-net
COPY courtyard /courtyard
WORKDIR /courtyard
RUN cargo build --release --bin courtyard

# We do not need the Rust toolchain to run the binary!
FROM debian:stable-slim AS runtime
WORKDIR /courtyard
COPY --from=builder /courtyard/target/release/courtyard /usr/local/bin
ENTRYPOINT ["/usr/local/bin/courtyard"]
