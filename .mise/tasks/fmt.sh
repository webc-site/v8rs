#!/usr/bin/env bash
#MISE description="rustfmt (nightly)"
#MISE dir="{{ config_root }}"

set -euo pipefail
exec cargo +nightly fmt --all "$@"
