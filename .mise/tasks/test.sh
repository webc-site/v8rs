#!/usr/bin/env bash
#MISE description="nextest 全量测试 (--all-features)"
#MISE dir="{{ config_root }}"

set -euo pipefail
exec ./test.sh "$@"
