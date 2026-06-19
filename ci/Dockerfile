FROM rust:1.94.0-alpine3.23

# openssl needed for cargo-udeps install
RUN apk update && apk add --no-cache nodejs openssl-dev openssl-libs-static

RUN rustup component add clippy rustfmt

RUN rustup toolchain install nightly

RUN cargo +nightly install cargo-udeps --locked
