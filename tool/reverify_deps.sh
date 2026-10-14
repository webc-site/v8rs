#!/usr/bin/env bash
set -euo pipefail

# 自动化依赖拓扑核验脚本：扫描工作区内未被主引擎接入的孤儿优化 crate

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$DIR"

echo "正在检查运行时 rs8 与基准测试 v8-benchmarks 的核心优化依赖"

OPT_CRATES=(
  "v8-ic"
  "v8-objects-maps"
  "v8-interpreter"
  "v8-compiler-frontend"
  "v8-compiler-backend"
  "v8-maglev-frontend"
  "v8-maglev-backend"
  "v8-turboshaft"
  "v8-builtins-core"
  "v8-builtins-array"
)

for crate_name in "${OPT_CRATES[@]}"; do
  in_bench="$( (cargo tree -p v8-benchmarks 2>/dev/null || true) | (grep "$crate_name" || true) | wc -l | tr -d ' ')"
  in_rs8="$( (cargo tree -p rs8 2>/dev/null || true) | (grep "$crate_name" || true) | wc -l | tr -d ' ')"
  rev_count="$( (cargo tree --workspace -i "$crate_name" 2>/dev/null || true) | (grep -v "$crate_name " || true) | wc -l | tr -d ' ')"
  
  status="孤儿未连通"
  if [ "$in_rs8" -gt 0 ]; then
    status="已接入 rs8"
  elif [ "$in_bench" -gt 0 ]; then
    status="已接入 benchmarks"
  fi

  echo "模块: $crate_name | 状态: $status | 外部反向引用数: $rev_count"
done
