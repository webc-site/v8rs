#!/usr/bin/env bash
set -euo pipefail

# 自动化审查门禁总控脚本：串联格式化、静态检查、全量测试、符号与依赖完整性审查

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$DIR"

echo "步骤一：代码格式检查"
cargo +nightly fmt --all --check

echo "步骤二：静态 Clippy 检查"
./sh/clippy.sh

echo "步骤三：全量单元与集成测试"
./test.sh

echo "步骤四：依赖拓扑扫描"
./tool/reverify_deps.sh

echo "步骤五：二进制符号链接状态核查"
./tool/reverify_symbols.sh

echo "全流程门禁核验通过"
