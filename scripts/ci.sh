#!/bin/bash
set -eu
export CARGO_TERM_COLOR=always

echo "::group::building"
cargo build
echo "::endgroup::"

# linting
echo "::group::linting"
cargo clippy -- -D warnings
cargo fmt -- --check
echo "::endgroup::"

# test without running dlt-daemon
echo "::group::test without running dlt-daemon"
killall --wait dlt-daemon || true
cargo test --test init_ok
echo "::endgroup::"

# test with running dlt-daemon
echo "::group::test with running dlt-daemon"
dlt-daemon -d
cargo test
echo "::endgroup::"

# code coverage
echo "::group::code coverage, required: 80% line coverage"
cargo llvm-cov --fail-under-lines 80
echo "::endgroup::"
