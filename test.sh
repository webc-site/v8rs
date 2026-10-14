#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -x
# --all-features
cargo nextest run --failure-output immediate --status-level slow --final-status-level slow --show-progress=none
