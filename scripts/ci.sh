#!/bin/bash
set -eu

cargo build

# test without running dlt-daemon
killall --wait dlt-daemon || true
cargo test

# test with running dlt-daemon
dlt-daemon -d
cargo test

# linting
cargo clippy -- -D warnings
cargo fmt -- --check

# code coverage
cargo llvm-cov
