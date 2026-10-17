#!/usr/bin/env bash
set -euo pipefail

# 自动化二进制符号核验脚本：检查关键优化模块是否真实链接入二进制

TARGET="${1:-}"
if [ -z "$TARGET" ]; then
  # 自动寻找 release 产物
  TARGET_DIR="$(cargo metadata --format-version 1 --no-deps | grep -o '"target_directory":"[^"]*"' | cut -d'"' -f4)"
  TARGET="$TARGET_DIR/release/v8-benchmarks"
fi

if [ ! -f "$TARGET" ]; then
  echo "目标二进制不存在: $TARGET"
  echo "请先执行 cargo build -p v8-benchmarks --release"
  exit 1
fi

echo "正在反编译并核查二进制符号: $TARGET"

check_symbol() {
  local pattern="$1"
  local desc="$2"
  local count
  count="$( (nm -C "$TARGET" 2>/dev/null || true) | (grep -E "$pattern" || true) | wc -l | tr -d ' ')"
  if [ "$count" -gt 0 ]; then
    echo "已链接 [$count 处符号]: $desc ($pattern)"
  else
    echo "未链接 [0 处符号]: $desc ($pattern)"
  fi
}

check_symbol "v8_ic" "v8-ic 内联缓存"
check_symbol "v8_objects_maps" "v8-objects-maps 隐藏类与形状"
check_symbol "v8_interpreter" "v8-interpreter 字节码解释器"
check_symbol "BytecodeDispatcher" "字节码虚拟机调度器"
check_symbol "v8_compiler" "v8-compiler 编译器"
check_symbol "v8_maglev" "v8-maglev JIT 前后端"
check_symbol "v8_turboshaft" "v8-turboshaft 编译管线"
check_symbol "v8_builtins" "v8-builtins 官方内置函数"
