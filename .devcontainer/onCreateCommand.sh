#!/bin/bash
set -eu

sudo apt-get update

# dlt-daemon and library
sudo apt-get install -y dlt-daemon libdlt-dev dlt-tools

# install bindgen requirements
sudo apt-get install -y libclang-dev

# Check if we need to install a specific Rust channel (beta or nightly)
if [ -f .devcontainer/.rust-channel ]; then
  CHANNEL=$(cat .devcontainer/.rust-channel)
  echo "Installing Rust $CHANNEL channel..."
  rustup toolchain install $CHANNEL
  rustup default $CHANNEL
  echo "Rust $CHANNEL channel installed and set as default"
fi

# for coverage measurement (install after setting up the correct toolchain)
rustup component add llvm-tools
curl -LsSf https://github.com/taiki-e/cargo-llvm-cov/releases/download/v0.6.21/cargo-llvm-cov-x86_64-unknown-linux-gnu.tar.gz | tar xzf - -C /usr/local/cargo/bin

# pre-commit
sudo apt-get install -y python3-pip
pip3 install --break-system-packages pre-commit
pre-commit install
