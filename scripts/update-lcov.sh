#!/bin/bash
set -eu
cargo llvm-cov --lcov --output-path lcov.info
