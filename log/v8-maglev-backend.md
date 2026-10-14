# v8-maglev-backend 开发日志

> 2026-10-12: 自 todo.md 看板行与 Wave 9 派发收敛 (仅中文)。

## 门禁实测

- 单元测试: `cargo nextest run -p v8-maglev-backend` (4/4 全部通过)
- Clippy: `cargo clippy -p v8-maglev-backend --all-targets -- -D warnings` (0 警告)
- 格式化: `cargo +nightly fmt --check -p v8-maglev-backend` (净)

## 模块与实现面

1. **x64 架构后端**: `maglev_assembler_x64_inl`，涵盖条件码映射表（`CONDITION_FOR_FLOAT64`）、无序比较奇偶标志（`CONDITION_FOR_NAN`）、缩放因子表（`SCALE_FACTOR` / `scale_factor_from_int`）以及 151 处汇编方法签名清单。
2. **arm64 架构后端**: `maglev_assembler_arm64`，涵盖 NaN 条件码（`CONDITION_FOR_NAN`）、LSL 移位缩放换算（`shift_from_scale`）、循环展开尺寸（`LOOP_UNROLL_SIZE`）、比较临时寄存器计数（`MAP_COMPARE_TEMPORARY_COUNT`）以及 185 处内联方法签名清单（`INL_METHODS`）。
3. **多架构登记**: `arm`、`loong64`、`ppc`、`riscv`、`s390` 完整登记非目标架构代码基线与行数/锚点。
4. **测试覆盖**: 验证 x64 与 arm64 缩放计算、条件码映射、方法表大小与条件编译守卫统计。
