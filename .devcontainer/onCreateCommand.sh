#!/bin/bash
set -eu

sudo apt-get update

# dlt-daemon and library
sudo apt-get install -y dlt-daemon libdlt-dev

# install bindgen requirements
sudo apt-get install -y libclang-dev
