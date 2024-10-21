FROM rust:1 AS builder
COPY shared-net /shared-net
COPY shared-data /shared-data
COPY gate /gate
COPY archive /archive
WORKDIR /archive
RUN cargo build --release --bin archive

# We do not need the Rust toolchain to run the binary!
FROM debian:stable-slim AS runtime
WORKDIR /opt/archive
COPY --from=builder /archive/target/release/archive .
ENTRYPOINT ["./archive"]
