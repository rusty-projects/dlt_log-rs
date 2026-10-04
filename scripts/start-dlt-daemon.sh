#!/bin/bash
set -eu

# start the DLT daemon in the background, accepting connections from localhost only
dlt-daemon -d -c "$(realpath "$(dirname "$0")/../.devcontainer/dlt.conf")"
