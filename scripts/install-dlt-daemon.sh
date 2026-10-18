#!/bin/bash
set -eu

# Replace the packaged DLT daemon, library and tools with a build of the
# given git revision (commit hash or branch) of
# https://github.com/COVESA/dlt-daemon. Used by CI to test against other DLT
# versions than the one packaged in Ubuntu, see dlt-versions.yml.
# Optional further arguments are patch files applied before building.
REF=$1
shift
PATCHES=()
for patch in "$@"; do
  PATCHES+=("$(realpath "$patch")")
done
REPO=https://github.com/COVESA/dlt-daemon.git
SRC_DIR=$(mktemp -d)

SUDO=""
if [ "$(id -u)" -ne 0 ]; then
  SUDO=sudo
fi

# stop a running daemon of the packaged version
killall --wait dlt-daemon || true

$SUDO apt-get update
$SUDO DEBIAN_FRONTEND=noninteractive apt-get remove -y --purge \
  dlt-daemon dlt-tools libdlt-dev libdlt2
$SUDO DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
  cmake zlib1g-dev

git init --quiet "$SRC_DIR"
git -C "$SRC_DIR" fetch --quiet --depth 1 "$REPO" "$REF"
git -C "$SRC_DIR" checkout --quiet FETCH_HEAD
echo "Building dlt-daemon $REF ($(git -C "$SRC_DIR" rev-parse HEAD))"
for patch in "${PATCHES[@]}"; do
  echo "Applying $patch"
  git -C "$SRC_DIR" apply "$patch"
done

# install to /usr like the removed packages: rust-lld does not search
# /usr/local/lib
cmake -S "$SRC_DIR" -B "$SRC_DIR/build" \
  -DCMAKE_BUILD_TYPE=Release \
  -DCMAKE_INSTALL_PREFIX=/usr \
  -DWITH_DLT_TESTS=OFF \
  -DWITH_DLT_EXAMPLES=OFF
cmake --build "$SRC_DIR/build" --parallel "$(nproc)"
$SUDO cmake --install "$SRC_DIR/build"
$SUDO ldconfig
rm -rf "$SRC_DIR"

dlt-daemon -h 2>&1 | grep -m1 "Package Version" || true
