#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if ! [ -d "v8" ]; then
  git clone --depth=1 ssh://git@ssh.github.com:443/webc-fork/v8.git
fi

if [ ! -d "sh" ]; then
  ln -s "$HOME/.local/share/cargo_sh" sh
  ./sh/init.sh
fi

mkdir -p git
cd git
dump() {
  local dir=$(basename $1)
  local repo=https://github.com/$1
  local url

  if [ -d "$dir" ]; then
    git -C "$dir" pull
  else
    git clone --depth=1 "$repo"
  fi
}

dump ChuyueSun/VeriStruct
dump 2akouwu/reverify
