#!/bin/bash
set -eu
export CARGO_TERM_COLOR=always

cargo build

# linting
cargo clippy -- -D warnings
cargo fmt -- --check

# test without running dlt-daemon
killall --wait dlt-daemon || true
cargo test --test init_ok

# test with running dlt-daemon
dlt-daemon -d
cargo test

# code coverage
cargo llvm-cov --lcov --output-path lcov.info
