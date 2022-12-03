FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /rust-hex
WORKDIR /rust-hex/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /rust-hex/fuzz/target/x86_64-unknown-linux-gnu/release/rusthex-fuzz /