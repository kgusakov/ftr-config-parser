pre-commit: check fmt-check clippy test

fmt-check:
     cargo fmt --check

check:
    cargo check --all-targets

clippy:
    cargo clippy --all-targets -- -D warnings

test:
    RUST_LOG="info" cargo test