#!/bin/bash
set -eu

# the tests need the DLT daemon to cover the logging paths, fail if it is not running
export DLT_LOG_REQUIRE_DAEMON=1

mkdir -p target/coverage

cargo llvm-cov \
    --lcov \
    --output-path target/coverage/lcov.info \
    --branch

cargo llvm-cov report \
    --html \
    --output-dir target/coverage \
    --branch

cargo llvm-cov report \
    --branch \
    --fail-under-functions 100 \
    --fail-under-lines 100 \
    --fail-under-regions 100 \
    --fail-uncovered-lines 100 \
    --fail-uncovered-regions 100 \
    --fail-uncovered-functions 100
