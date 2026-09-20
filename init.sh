#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if ! [ -d "cpp2rust" ]; then
  git clone ssh://git@ssh.github.com:443/webc-fork/cpp2rust.git
fi

if ! [ -d "v8" ]; then
  git clone ssh://git@ssh.github.com:443/webc-fork/v8.git
fi

if [ ! -d "sh" ]; then
  ln -s "$HOME/.local/share/cargo_sh" sh
fi

./sh/init.sh
