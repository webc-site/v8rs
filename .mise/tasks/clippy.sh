#!/usr/bin/env bash
#MISE description="rustfmt + clippy 0 警告门禁"
#MISE dir="{{ config_root }}"

set -euo pipefail
exec ./sh/clippy.sh "$@"
