FROM rust:1 AS builder
COPY shared-net /shared-net
COPY archive-lib /archive-lib
COPY forum-lib /forum-lib
COPY gate-lib /gate-lib
COPY hall-lib /hall-lib
COPY gate /gate
WORKDIR /gate
RUN cargo build --release --bin gate

# We do not need the Rust toolchain to run the binary!
FROM debian:stable-slim AS runtime
WORKDIR /opt/gate
COPY --from=builder /gate/target/release/gate .
ENTRYPOINT ["./gate"]
