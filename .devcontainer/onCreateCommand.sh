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
  rustup toolchain install "$CHANNEL"
  rustup default "$CHANNEL"
  echo "Rust $CHANNEL channel installed and set as default"
fi

# for coverage measurement (install after setting up the correct toolchain)
rustup component add llvm-tools
LLVM_COV_VERSION=0.9.1
LLVM_COV_SHA256=b3f68e625481fed9b16444174f3fa5ebcdbde4a1878803a35eabe2dcefcdc41a
LLVM_COV_TARBALL=$(mktemp)
curl -LsSf -o "$LLVM_COV_TARBALL" "https://github.com/taiki-e/cargo-llvm-cov/releases/download/v${LLVM_COV_VERSION}/cargo-llvm-cov-x86_64-unknown-linux-gnu.tar.gz"
echo "${LLVM_COV_SHA256}  ${LLVM_COV_TARBALL}" | sha256sum --check --strict
tar xzf "$LLVM_COV_TARBALL" -C /usr/local/cargo/bin
rm -f "$LLVM_COV_TARBALL"

# pre-commit
sudo apt-get install -y python3-pip
pip3 install --break-system-packages pre-commit==4.6.2
pre-commit install
