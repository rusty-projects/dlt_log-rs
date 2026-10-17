#!/bin/bash
set -eu
VERSION=$1
CONFIG=.devcontainer/devcontainer.json
TOOLCHAIN_FEATURE=./features/rust-toolchain

RUST_FEATURE=$(jq -r '.features | keys[] | select(startswith("ghcr.io/devcontainers/features/rust:"))' "$CONFIG")

if [ -z "$RUST_FEATURE" ]; then
  echo "Feature ghcr.io/devcontainers/features/rust not found in $CONFIG"
  exit 1
fi

# The rust feature only supports versions and dated nightlies. For the beta
# and nightly channels, install latest stable with the rust feature and let
# the local rust-toolchain feature install the channel on top.
if [[ "$VERSION" == "beta" || "$VERSION" == "nightly" ]]; then
  RUST_VERSION=latest
  TOOLCHAIN=$VERSION
else
  RUST_VERSION=$VERSION
  TOOLCHAIN=""
fi

jq --arg RUST_FEATURE "$RUST_FEATURE" --arg RUST_VERSION "$RUST_VERSION" \
  --arg TOOLCHAIN_FEATURE "$TOOLCHAIN_FEATURE" --arg TOOLCHAIN "$TOOLCHAIN" \
  '.features[$RUST_FEATURE].version = $RUST_VERSION
   | .features[$TOOLCHAIN_FEATURE].toolchain = $TOOLCHAIN' \
  "$CONFIG" > temp.json
mv temp.json "$CONFIG"
