# v8-codegen-core 开发日志

## 轮7 (W7CG) 进展记录

### 1. 测试面编译修复
- 清理 `tests/codegen_semantic.rs` 中上一轮遗留的 `_keep` 幽灵引用与模块私有导出问题，统一改为 crate 根公有导出引用。
- 修复 `CodeDesc`、`CodeReference`、`CodeReferenceKind`、`SignatureBuilder`、`MachineRepresentation` 等导出与方法契约。
- 修复 C-ABI 函数指针签名与 `ExternalReference` 构造。

### 2. 定制脚本与核心语义填体
- **handler-table**:
  - 更新 `tool/src/codegen/handler-table.h.js`，严格对齐 `v8/src/codegen/handler-table.{h,cc}`。
  - 填体 `handler_table.rs`：实现 RangeBased 与 ReturnAddressBased 双模式完整逻辑，包括位域编解码（Prediction / WasUsed / Offset）、越界检查、`lookup_handler_index_for_range` 最内层区间匹配、`lookup_return` 二分查找，以及 `LengthForRange` 字节大小计算。
- **code-factory**:
  - 更新 `tool/src/codegen/code-factory.h.js`，严格对齐 `v8/src/codegen/code-factory.{h,cc}`。
  - 填体 `code_factory.rs`：实现 `AllocationSiteOverrideMode` 枚举、`BuiltinId` 映射以及 22 项静态工厂方法分派逻辑，并于 `lib.rs` 正式导出。
- **machine-type / code-comments / jump-table-info / register-config**:
  - 补全 `MachineRepresentation` 枚举及其底层判别值。
  - 为 `CodeCommentsWriter` 与 `JumpTableInfoWriter` 增加 `emit_to_vec` 序列化方法。
  - 为 `RegisterConfiguration` 增加测试配置构造与 Combine 模式 FP 别名逻辑验证。

### 3. 门禁与规范验证
- `CARGO_TARGET_DIR=target/agent_w7cg cargo nextest run -p v8-codegen-core`: 19/19 测试全部通过 (100% 绿)。
- `rg "\[allow" v8-codegen-core`: 0 处 allow，严格遵循规范零绕过。
- `CARGO_TARGET_DIR=target/agent_w7cg cargo clippy -p v8-codegen-core --all-targets -- -D warnings`: 0 警告 0 报错。

## 轮7 Reviewer 审查与优化结论 (W7CG-Review)

### 1. 深度比对与缺陷修复 (handler-table & code-factory)
- **handler_table**:
  - **修复 `handler_offset_is_valid` 边界偏差**: 原 `< K_LAZY_DEOPT` 修正为 `<= K_LAZY_DEOPT`，对齐 V8 原版 `HandlerOffsetField::kMax` 合法 28 位最大值的位域编码约定。
  - **内存对齐防 UB**: 将 `read_i32` / `write_i32` 升级为 `std::ptr::read_unaligned` / `std::ptr::write_unaligned`，杜绝未对齐切片解引用 UB 隐患。
  - **热路径查询迭代与边界优化**: 在 `lookup_handler_index_for_range` 与 `lookup_return` 中前置单次模式断言，消除循环内重复的 `mode_` 校验与多余冗余边界检查。
  - **消除 panic 骨架**: 为 `emit_return_table_start` 与 `emit_return_entry` 接入 `AssemblerBase` 具体实现，并使用现代 Rust `.is_multiple_of(4)` 进行 4 字节元数据对齐，消灭 `todo!()` panic。
  - **完整反汇编与显示支持**: 新增 `format_range` / `format_return` 以及 `CatchPrediction::Display`，对齐 V8 `HandlerTableRangePrint` / `HandlerTableReturnPrint`。
- **code_factory**:
  - **补齐缺失全局枚举**: 新增 `ScopeType`、`ConvertReceiverMode`、`TypeofMode`、`StringAddFlags`、`ArgvMode`，彻底消除魔法数字。
  - **修复派发语义偏差**: 修正 `call_function`（根据 `ConvertReceiverMode` 派发）、`load_global_ic` / `load_global_ic_in_optimized_code`（根据 `TypeofMode` 派发）、`string_add`（根据 `StringAddFlags` 派发）、`fast_new_function_context`（根据 `ScopeType::Eval = 2` 严格映射）、`c_entry`（完整映射 `switch_to_central_stack` / `argv_mode` / `builtin_exit_frame`）。
  - **扩展 `BuiltinId` 枚举**: 补全上述派发对应的具体 builtin 标识（如 `WasmCEntry`、`LoadGlobalICInsideTypeof` 等）。

### 2. 测试面扩展
- 在 `tests/codegen_semantic.rs` 中新增：
  - `handler_table_lazy_deopt_and_formatting`: 验证 `K_LAZY_DEOPT` 编解码及格式化。
  - `handler_table_emit_with_assembler`: 验证 `emit_return_table_start` 与 `emit_return_entry` 在汇编缓冲区上的小端序列化。
  - `code_factory_dispatch`: 完整验证上述 6 组派发分支的不同 BuiltinId 映射行为。
- 测试总数提升至 21 项。

### 3. 门禁验证 (100% 通过)
- `CARGO_TARGET_DIR=target/agent_w7cgr cargo nextest run -p v8-codegen-core`: 21/21 测试全部通过 (100% 绿)。
- `rg "\[allow" v8-codegen-core`: 0 处 allow 绕过。
- `CARGO_TARGET_DIR=target/agent_w7cgr cargo clippy -p v8-codegen-core --all-targets -- -D warnings`: 0 警告 0 报错。
- 结论: clean，达到发布标准。

## 轮7 Round 2 Reviewer 交叉审查与优化结论 (W7CG-Review2)

### 1. 深度交叉审查与鲁棒性加固 (handler-table & code-factory)
- **handler_table**:
  - **加固模式安全断言**: 为 `set_range_start`、`set_range_end`、`set_range_data`、`set_range_handler` 补齐 `assert!(self.mode_ == EncodingMode::RangeBased)`，杜绝在 `ReturnAddressBased` 模式下误写导致的缓冲区溢出隐患。
  - **补齐 Return 编码越界检查**: 在 `set_return_handler` 中增加 `handler_offset_is_valid` 断言校验，防止超限 handler offset 发生静默截断。
  - **汇编输出防错升级**: `emit_return_entry` 在汇编写入前校验 `handler_offset_is_valid`，超限直接返回类型化错误 `Error::InvalidHandlerTable`，拒绝无效指令生成。
  - **表对齐填充校验**: `from_raw` 补齐与 V8 原版 `DCHECK_GE` 严格对齐的 padding debug_assert。
- **code_factory**:
  - **补齐 `ElementsKind` 全局枚举**: 严格对齐 `v8/src/objects/elements-kind.h`，定义 `PackedSmi`、`HoleySmi`、`Packed`、`Holey`、`PackedDouble`、`HoleyDouble` 判别值并导出，消除构造工厂中的裸数字魔法常量。
  - **提供 C++ 默认参数便捷重载**: 新增 `call_default`、`call_function_default`、`string_add_default`、`c_entry_default`、`runtime_c_entry_default`，精确匹配 V8 源码头文件的默认实参约定。
- **tool 脚本同步**:
  - 同步更新 `tool/src/codegen/code-factory.h.js` 与 `tool/src/codegen/handler-table.h.js`，保证重跑覆盖后规范与优化成果持久一致。

### 2. 测试面扩展
- 在 `tests/codegen_semantic.rs` 中新增：
  - `handler_table_safety_and_mode_checks`: 覆盖 ReturnAddressBased 误写 panic、handler offset 超限断言拦截、`emit_return_entry` 错误返回、`ElementsKind` 类型化分派及 5 组默认实参辅助方法。
- 测试总数提升至 22 项 (100% 通过)。

### 3. 门禁验证 (100% 通过)
- `CARGO_TARGET_DIR=target/agent_w7cgr2 cargo nextest run -p v8-codegen-core`: 22/22 测试全部通过 (100% 绿)。
- `rg "\[allow" v8-codegen-core`: 0 处 allow 绕过。
- `CARGO_TARGET_DIR=target/agent_w7cgr2 cargo clippy -p v8-codegen-core --all-targets -- -D warnings`: 0 警告 0 报错。
- 结论: clean，达到发布标准。


[轮7 W7CG] tests/codegen_semantic.rs 31 编译错清零 (接力并行修复+本席复核门禁验证); lib todo! 69→45 (code_factory 22 处 1:1 填体并补齐原版 UNREACHABLE/DCHECK 语义, handler_table 族 emit 面填体含 DataAlign 0xcc 填充与 from_raw padding 断言); 深水面 45 todo! 已入 README 推迟台账 (v8-codegen-core/README.md); 导出面变更记档 (HandlerTableType/CodeReferenceKind 别名, MachineRepresentation 精确序号, num_entries/emit_to_vec/new_for_test 等别名 API); 门禁 CARGO_TARGET_DIR=target/agent_w7cg cargo nextest run -p v8-codegen-core: 22/22 通过。

## 轮7 Round 3 Reviewer 终验审查与冻结结论 (W7CG-Review3, 3/3 终验达成)

### 1. 终验深度比对与对齐加固
- **handler_table**:
  - **哨兵统一消灭裸数字**: `lookup_return` 未命中分支由裸 `-1` 替换为命名常量 `K_NO_HANDLER_FOUND`，与 `lookup_handler_index_for_range` 及 V8 原版头文件契约完全对齐。
  - **断言与对齐严格保证**: 逐行验证 `from_raw` 的 padding debug_assert、`get_*` / `set_*` 的双模式单次断言、`handler_offset_is_valid` [0, kLazyDeopt] 边界，以及 `read_unaligned` / `write_unaligned` 零对齐 UB。
- **code_factory**:
  - 逐行对照 `v8/src/codegen/code-factory.{h,cc}` 的 22 项静态工厂分派与 5 组默认实参辅助方法（`c_entry_default`、`runtime_c_entry_default`、`string_add_default`、`call_default`、`call_function_default`），100% 严格一致。
- **jump_table_info**:
  - 为 `JumpTableInfoWriter::emit` 完整接入 `AssemblerBase` 字节发射，并在 `tests/codegen_semantic.rs` 中补充了针对汇编缓冲与 `emit_to_vec` 的等价性断言测试。
- **tool 脚本双向 100% 幂等同步**:
  - 同步更新并实测验证 `tool/src/codegen/code-factory.h.js`、`tool/src/codegen/handler-table.h.js` 和 `tool/src/codegen/jump-table-info.h.js`，保证重跑覆盖后产物与优化规范完全一致且 0 回退。

### 2. 测试面与代码规范严格核验
- **测试通过率**: `CARGO_TARGET_DIR=target/agent_w7cgr3 cargo nextest run -p v8-codegen-core`: 22/22 测试全部通过 (100% 绿)。
- **Allow 绕过检测**: `rg "\[allow" v8-codegen-core`: 0 处绕过。
- **Clippy 静态检查**: `CARGO_TARGET_DIR=target/agent_w7cgr3 cargo clippy -p v8-codegen-core --all-targets -- -D warnings`: 0 警告 0 报错。
- **代码格式化**: `cargo +nightly fmt --check -p v8-codegen-core`: 100% 对齐规范。
- **最终结论**: clean (3/3 终验达成，准予封板冻结)。

## W11CGb 接力波 (2026-10-13)

接力撞线席 W11CG (停在「建基线快照，扫描各脚本输出与规范化基线的真实差异」中途)。

### 1. 现场复测与差集盘点判定表 (18 处增量)

复测确认: `cargo check` 红 5×E0015 (全在 src/code_reference.rs:65/:72/:79/:142/:149 一带); todo! 计数 56 (01:0x 普查 38 + 18)。三方对比 (git HEAD vs `/tmp/W11CG_src_baseline` 基线快照 vs 工作区) 判定: **基线快照 38 处 todo 与 HEAD 一致, 工作区 10 个 DIFF 文件全部为「基线快照后脚本产物就地回写」导致的回退, 18 处增量 0 处属新增挂载草案**。逐条判定:

| # | 文件 | 判定 | 回退内容 |
|---|------|------|---------|
| 1-2 | assembler.rs +2 | 回退 | DefaultAssemblerBuffer(new/grow) 与 ExternalAssemblerBufferImpl 已填体被替换为 `external_assembler_buffer` 骨架 todo; AssemblerOptions::Default 规范深水格式被重置为「待 Developer 修订」 |
| 3-10 | code_comments.rs +8 | 回退 | Writer::{Add, Emit, section_size, emit_to_vec} 与 Reader 侧已填体全部被脚本草案 todo 覆盖 (含 emit 签名退化为 `*mut ()`) |
| 11 | code_desc.rs +1 | 回退 | CodeDesc::initialize 已填布局体 (CodeDescOffsets 参数集) 被静态函数骨架覆盖 |
| 12 | linkage.rs +1 | 回退 | LinkageLocation::size_in_pointers 已填体 (from_code+element_size_in_pointers) 被 todo 覆盖 |
| 13 | macro_assembler.rs +1 | 回退 | comment_for_off_heap_trampoline 已填体 (基线 flags.code_comments=false 恒空串) 被 todo 覆盖; 另 4 处规范深水格式回退 |
| 14-15 | register_config.rs +2 | 回退 | new_for_test/new_for_test_with_aliasing 测试构造族被删, AliasingKind 变体名回退为 kOverlap/kCombine/kIndependent |
| 16-18 | signature.rs +3 | 回退 | SignatureBuilder::{add_return, add_param, get} 已填体被 todo 覆盖 |

另查出 **todo 计数以外的 2 处静默回退**: machine_type.rs 整体丢失 MachineRepresentation 枚举 + from_code + element_size_in_pointers (tests/ 依赖面), aligned_slot.rs 丢失消 unused_assignments 的 match 返回重构。同波恢复。

### 2. E0015 修法

5×E0015 源于脚本草案将 Kind 谓词写成 const fn 内 `self.kind_ == Kind::X` (derived PartialEq 非 const)。基线实现本就以 **const 安全的显式判别值比较 `self.kind_ as u8 == Kind::X as u8`** 承载 (方案一「const 手写比较替代重载调用」), 恢复 code_reference.rs 即全数消除, 未删任何断言。附带发现: 未挂载文件不参与编译掩盖了 cpu_features.rs 枚举 `= 19,,` / `= 10,,` 双逗号脚本产物, 挂载时一并修复。

### 3. 恢复与 todo 消账

- **恢复件数: 10 文件**整体从基线快照恢复 (恢复前核 diff 确认 current 侧无 baseline 缺失的有效新增); todo 56→38, 门禁即刻转绿。
- **续消账 todo 38→32** (浅水 1:1 填体 6 处):
  - tick_counter.rs AttachLocalHeap/DetachLocalHeap (tick-counter.cc:15-21 纯 1:1, -2);
  - cpu_features.rs supported_ EnumSet 位集载体 + IsSupported/SetSupported/SetUnsupported/icache_line_size/dcache_line_size 填体 (cpu-features.h:149-168, -4); Probe/SupportsSimd128/SupportsOptimizer/FlushICache 留深水规范格式。
  - register_config.rs 移植原版构造函数 `RegisterConfiguration::new` (register-configuration.cc:228-323, Combine/Overlap/Independent 三支全量, 无 todo 增量)。
- **深水规范化**: code_reference 11 / handler_table 4 / register_config 3 / source_position 2 / flush 1 / script_details 1 / assembler 1 / macro_assembler 4 / cpu_features 5 共 32 处统一为 `todo!("原函数名: 前置; 解锁 crate")` 格式, 台账登记 `v8-codegen-core/readme/zh.md` (重建, 原为空)。
- **lib.rs 最小挂载 diff**: +mod/pub use cpu_features、tick_counter 两面 (v8-maglev-frontend 复验依赖的本 crate 绿前提已恢复)。

### 4. 新增测试 (v8/test/unittests codegen 族移植)

- `tests/register_configuration_unittest.rs`: 原版 register-configuration-unittest.cc BasicProperties + CombineAliasing 全断言面 1:1 移植 (2 项)。
- `tests/cpu_features_tick_counter_unittest.rs`: 位集往返 + Attach/Detach DCHECK 面回归 (2 项; 原版无专属 unittest, 按填体语义)。

### 5. 门禁 (CARGO_TARGET_DIR=target/agent_W11CGb, 均 -p v8-codegen-core)

- `cargo check`: RC=0 (恢复前 E0015×5 红)。
- `cargo nextest run`: RC=0, 41/41 通过 100% 绿 (接力起点账面 36 → 41, 含本席新增 4 项)。
- `cargo clippy --all-targets -- -D warnings`: RC=0 (touch lib.rs 强制重验一次, 非缓存绿)。

### 6. 清理

删除 W11CG 中间产物: `/tmp/W11CG_src_baseline` (基线快照, 使命完成)、`target/agent_W11CG` (撞线席构建缓存 174M); crate 内无 `*.baseline*` 残留。未触其他席在飞目录与 crate。


