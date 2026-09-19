#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
if [ -f sh/env.sh ]; then
  . sh/env.sh
fi
set -x

exec cargo run --package example "$@"
