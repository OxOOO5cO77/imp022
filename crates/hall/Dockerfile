FROM rust:1 AS builder
COPY shared-net /shared-net
COPY gate-lib /gate-lib
COPY hall-lib /hall-lib
COPY hall /hall
WORKDIR /hall
RUN cargo build --release --bin hall

# We do not need the Rust toolchain to run the binary!
FROM debian:stable-slim AS runtime
WORKDIR /opt/hall
COPY --from=builder /hall/target/release/hall .
COPY --from=builder /hall/assets/data ./assets/data
ENTRYPOINT ["./hall"]
