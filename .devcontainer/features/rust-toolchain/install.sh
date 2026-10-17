#!/bin/bash
set -eu

TOOLCHAIN="${TOOLCHAIN:-}"

if [ -z "$TOOLCHAIN" ]; then
  echo "No toolchain requested, keeping the toolchain of the rust feature"
  exit 0
fi

export CARGO_HOME="${CARGO_HOME:-/usr/local/cargo}"
export RUSTUP_HOME="${RUSTUP_HOME:-/usr/local/rustup}"
export PATH="$CARGO_HOME/bin:$PATH"

# keep files writable for the rustlang group, as the rust feature does
umask 0002

echo "Installing Rust $TOOLCHAIN toolchain..."
rustup toolchain install "$TOOLCHAIN" --profile default \
  --component rust-analyzer,rust-src,llvm-tools
rustup default "$TOOLCHAIN"
