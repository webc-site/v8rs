#!/usr/bin/env bash
#MISE description="编译全 workspace"
#MISE dir="{{ config_root }}"

set -euo pipefail
exec cargo build --workspace "$@"
