FROM rust:1 AS builder
COPY shared-net /shared-net
COPY gate-lib /gate-lib
COPY archive-lib /archive-lib
COPY archive /archive
WORKDIR /archive
RUN cargo build --release --bin archive

# We do not need the Rust toolchain to run the binary!
FROM debian:stable-slim AS runtime
WORKDIR /opt/archive
COPY --from=builder /archive/target/release/archive .
ENTRYPOINT ["./archive"]
