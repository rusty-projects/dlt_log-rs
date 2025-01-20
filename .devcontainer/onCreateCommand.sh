#!/bin/bash
set -eu

sudo apt-get update

# dlt-daemon and library
sudo apt-get install -y dlt-daemon libdlt-dev

# install bindgen requirements
sudo apt-get install -y libclang-dev

# for coverage measurement
rustup component add llvm-tools-preview
curl -LsSf https://github.com/taiki-e/cargo-llvm-cov/releases/download/v0.6.16/cargo-llvm-cov-x86_64-unknown-linux-gnu.tar.gz | tar xzf - -C /usr/local/cargo/bin
