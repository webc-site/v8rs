#!/usr/bin/env bash
set -euo pipefail

# 自动化执行 cargo-pgo 优化与跑分全流程

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$DIR"

echo "清理历史 PGO 数据"
cargo pgo clean

echo "阶段一：插桩构建"
cargo pgo build -- -p v8-benchmarks --release

echo "阶段二：运行代表性基准用例采集执行轨迹"
# 根据 target-directory 获取二进制实际生成路径
TARGET_DIR="$(cargo metadata --format-version 1 --no-deps | grep -o '"target_directory":"[^"]*"' | cut -d'"' -f4)"
"$TARGET_DIR/release/v8-benchmarks"

echo "阶段三：应用画像执行终极优化编译"
cargo pgo optimize -- -p v8-benchmarks --release

echo "阶段四：运行 PGO 优化后的基准评估"
"$TARGET_DIR/release/v8-benchmarks"
