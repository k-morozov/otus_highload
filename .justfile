fmt: 
  cargo +nightly fmt --all -- --verbose

fmt-check: 
  cargo +nightly fmt --all -- --check --verbose

clippy-check:
  cargo clippy

clippy:
  cargo clippy --fix