FROM rust:1.94.0-alpine3.23

RUN apk update && apk add --no-cache nodejs

RUN rustup component add clippy rustfmt

RUN rustup toolchain install nightly

RUN cargo +nightly install cargo-udeps --locked
