# v8-flags 开发日志

> 2026-10-12 收敛: 自 readme/{zh,en}.md 台账迁入 (仅中文)。看板 (todo.md) 只留状态。

## readme 台账 (2026-10-12 自 readme/{zh,en}.md 收敛)

### 审查环回归记录 (自 readme/zh.md「测试覆盖」节迁入)

- round2 回归: `implication_no_change_registers_no_reset_edge` (蕴含等值不登记重置边,
  对齐原版 implied_by_map_ 仅改值时 push) / `resolve_wasm_code_space_clamp`
  (矛盾处理前置块) / `split_argument_name_too_long` (原版 SplitArgument 1KB 缓冲 FlagError)
- round3 回归 (`tests/review_round3_unittest.rs`): 规范配置修正 (V8_ENABLE_MAGLEV_GRAPH_PRINTER
  按 BUILD.gn:487 standalone 默认开; V8_CAN_CREATE_SHARED_HEAP_BOOL 按 globals.h:137 默认开 —
  原 7 flag 误只读, 18 蕴含缺失, 已按 clang -E 预处理地面真值补齐 848/848 全序零差) /
  无符号溢出拒收 (≥2^63 对应原版 errno!=0, flags.cc:716-728) / 数值前缀对齐 strtol·strtod
  (前导空白跳过 + inf/infinity/nan/十六进制浮点, flags.cc:801/823) / 边表以
  EnforceFlagImplications 调用为界 (flags.cc:1508 每调用新建 processor) /
  implied_by 残留语义 (flags.cc:238-241 非蕴含设置不清除) / 哈希串段 std::quoted 转义
  (flags.cc:534) / 50 例命令行边界对拍表

en.md 无日志内容 (映射 / 计数门禁 / 偏差 / 推迟均为结构性文档), 留 readme 不动。
