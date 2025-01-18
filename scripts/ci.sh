#!/bin/bash
set -eu

cargo build --verbose
cargo test --verbose
cargo clippy -- -D warnings
cargo fmt -- --check
