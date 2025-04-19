#!/bin/bash
set -eu
VERSION=$1

FEATURE=$(jq -r '.features | keys[] | select(startswith("ghcr.io/devcontainers/features/rust"))' .devcontainer/devcontainer.json)

# check that feature starts with ghcr.io/devcontainers/features/rust
if [[ $FEATURE != ghcr.io/devcontainers/features/rust* ]]; then
  echo "Feature does not start with ghcr.io/devcontainers/features/rust"
  exit 1
fi

jq --arg FEATURE "$FEATURE" --arg VERSION "$VERSION" '.features.[$FEATURE].version = $VERSION' .devcontainer/devcontainer.json > temp.json
mv temp.json .devcontainer/devcontainer.json
