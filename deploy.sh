#!/bin/bash

# set -o errexit
# set -o nounset
# set -o pipefail
# set -o xtrace

# readonly TARGET_HOST=wren@daemon
# readonly TARGET_PATH=/home/wren/cyber
# readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
# readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/hello-world

# cargo build --release --target=${TARGET_ARCH}
# rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
# ssh -t ${TARGET_HOST} ${TARGET_PATH}
../.cargo/bin/cross build --release --target=aarch64-unknown-linux-gnu
