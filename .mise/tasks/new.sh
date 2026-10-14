#!/usr/bin/env bash
#MISE description="创建微 crate: mise run new -- <crate名>"
#MISE dir="{{ config_root }}"

set -euo pipefail
if [ $# -lt 1 ]; then
  echo "用法: mise run new -- <crate名>" >&2
  exit 1
fi
exec ./sh/new.sh "$1"
