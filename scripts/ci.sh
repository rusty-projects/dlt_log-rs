#!/bin/bash
set -eu

cargo build --verbose

# test without running dlt-daemon
killall --wait dlt-daemon || true
cargo test --verbose

# test with running dlt-daemon
dlt-daemon -d
cargo test --verbose

cargo clippy -- -D warnings
cargo fmt -- --check
