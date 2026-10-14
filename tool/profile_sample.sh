#!/usr/bin/env bash
set -euo pipefail

# 自动化基准测试性能采样与热点分析脚本 (针对 macOS sample)

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$DIR"

TARGET_DIR="$(cargo metadata --format-version 1 --no-deps | grep -o '"target_directory":"[^"]*"' | cut -d'"' -f4)"
BIN="$TARGET_DIR/release/v8-benchmarks"

echo "正在使用保留符号配置进行优化构建"
cargo build -p v8-benchmarks --release --config 'profile.release.strip=false' --config 'profile.release.debug=1'

SAMPLE_FILE="/tmp/v8_benchmarks_profile.sample"
rm -f "$SAMPLE_FILE"

echo "启动基准并执行 2 秒高频采样"
"$BIN" & PID=$!
sample "$PID" 2 -file "$SAMPLE_FILE" > /dev/null 2>&1 || true
wait "$PID" || true

if [ ! -f "$SAMPLE_FILE" ]; then
  echo "采样产物未生成，请检查权限或系统环境"
  exit 1
fi

echo "采样完成，正在分析并过滤前 30 个叶子热点函数"
if command -v rustfilt > /dev/null 2>&1; then
  cat "$SAMPLE_FILE" | rustfilt | sed -E 's/^[ \t\+\!\:\*\|]+//' | grep '^[0-9]' | awk '{print $1, $2}' | sort -k2 | awk '{count[$2]+=$1} END {for (fn in count) print count[fn], fn}' | sort -n -r | head -n 30
else
  cat "$SAMPLE_FILE" | sed -E 's/^[ \t\+\!\:\*\|]+//' | grep '^[0-9]' | awk '{print $1, $2}' | sort -k2 | awk '{count[$2]+=$1} END {for (fn in count) print count[fn], fn}' | sort -n -r | head -n 30
fi
