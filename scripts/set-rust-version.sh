#!/bin/bash
set -eu
VERSION=$1

FEATURE=$(jq -r '.features | keys[] | select(startswith("ghcr.io/devcontainers/features/rust"))' .devcontainer/devcontainer.json)

# check that feature starts with ghcr.io/devcontainers/features/rust
if [[ $FEATURE != ghcr.io/devcontainers/features/rust* ]]; then
  echo "Feature does not start with ghcr.io/devcontainers/features/rust"
  exit 1
fi

# For beta and nightly, use latest as the base and install the channel via rustup
if [[ "$VERSION" == "beta" || "$VERSION" == "nightly" ]]; then
  # Use latest for the devcontainer feature, we'll install beta/nightly via rustup in onCreateCommand
  jq --arg FEATURE "$FEATURE" --arg VERSION "latest" '.features.[$FEATURE].version = $VERSION' .devcontainer/devcontainer.json > temp.json
  mv temp.json .devcontainer/devcontainer.json
  
  # Create a marker file to indicate which channel to install
  echo "$VERSION" > .devcontainer/.rust-channel
else
  # For specific versions, use them directly
  jq --arg FEATURE "$FEATURE" --arg VERSION "$VERSION" '.features.[$FEATURE].version = $VERSION' .devcontainer/devcontainer.json > temp.json
  mv temp.json .devcontainer/devcontainer.json
  
  # Remove any existing channel marker
  rm -f .devcontainer/.rust-channel
fi
