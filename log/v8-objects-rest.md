# v8-objects-rest 席位日志

[轮7 W7O3c] 接力 W7O3 (撞轮次上限熔断态: todo! 57→47, check 绿, 停在 abstract_code.rs 尾部 stub 半删除态)。

## 接力实况 (差集盘点)

- 起点 todo! 47 → 终点 todo! 11 (填体/删净 36 处); `cargo check -p v8-objects-rest` 0 error; `cargo nextest` 5/5 PASS (每组改完即跑, 共三轮门禁全绿)。
- 半写态核验: abstract_code.rs 尾部 9 个被 AbstractCodeExt impl 取代的 free-function stub 残留确认 → 删净 (无外部引用)。

## 各组明细

1. abstract_code.rs: 47→36 中删 9 stub; 余 2 为 ledger 项 (contains Code 支 isolate / SourcePosition 两支位置表)。
2. deoptimization_data_inl.rs 11→1:
   - DeoptimizationDataExt 填体 ulength/literal_array/size_including_header + 新增 deopt_count/bytecode_offset_or_builtin_continuation_id/set_bytecode_offset (6 个 free stub 挂载为 trait 方法, 逐法对照 deoptimization-data-inl.h);
   - DeoptimizationLiteralArrayExt 填体 ulength/get_raw + 新增 get (CHECK 转 Invariant, 禁 panic 红线)/set (三分支换装, 基线沙箱关两支: BytecodeArray→wrapper / MakeWeak);
   - 元素区/长度槽经 v8-objects-arrays TaggedArrayExt 解锁 (fixed-array 席已交付, 原推迟判定过期);
   - 余 1: GetSharedFunctionInfo (SharedFunctionInfoWrapper 类本体缺) → 台账 #11。
3. turboshaft_types.rs 6→0: 10 类挂载 — 具体类 repr(C) 布局 + 6 ext trait (from/to/set/from_high..to_low/elements_high/low/special_values/min/max, 非对齐 f64 走 read_unaligned/write_unaligned), 32 个 inl 访问器 1:1; 柔性尾经 address()+DATA_START 索引。
4. turbofan_types_inl.rs 6→0: 5 布局类 + 5 ext trait (bitset_low/high, union type1/2 set 带 WriteBarrierMode 形参, range min/max 非对齐, heap constant Cast 裸转, other-number constant), 16 访问器 1:1。
5. trusted_object_tq.rs 1→0: self_indirect_pointer 仅沙箱开存在, 基线存根删净, 语义由 K_SELF_INDIRECT_POINTER_OFFSET_SANDBOX_ON 常量登记。
6. instruction_stream.rs 6→3: relocation_start/end/size 填体 (TrustedByteArray begin/end/length 解锁, 台账 #1/#2/#3 中 byte-array 项过期); 余 Relocate/Initialize/Finalize (WritableJitAllocation/CodeDesc 深水)。
7. bytecode_array.rs 3→2: SizeIncludingMetadata 填体 (constant_pool→TrustedFixedArray::size_for, handler_table→TrustedByteArray::allocated_size, source_position_table→ByteArray::allocated_size 三支 1:1, 原版 L228 判 ByteArray 支保留); 余 SourcePositionTable 空表哨兵 (isolate/heap) + MarkVerified (sandbox)。
8. code.rs 3→3: 三处均深水 ledger 项 (EmbeddedData/Builtins/SourcePositionTableIterator); 顺手清未用 marker import (DeoptimizationLiteralArray)。

## 推迟台账 (readme/zh.md, 本席建立, 11 条)

原函数名/前置/解锁 crate 全表见 `v8-objects-rest/readme/zh.md`:
execution 席 1 (AbstractCode::contains Code 支) · codegen-core 席 5 (SourcePosition×2 + Relocate/Initialize/Finalize) · isolate/heap 席 1 (SourcePositionTable 哨兵) · sandbox 席 1 (MarkVerified) · snapshot 席 1 (metadata_start 嵌入支) · v8-init/builtins 席 1 (sandboxing_mode 内建支) · shared-function-info 席 1 (GetSharedFunctionInfo)。
并登记「已解锁回填」区 (fixed/byte-array 面交付使 11 处原推迟就地填体)。

## 红线合规

只改 v8-objects-rest/; 未动 Cargo.toml; 无 git 操作; 无 panic! (原版 CHECK 转 Error::Invariant); 锚点 /// v8: 注释齐; 注释中文。

---

## [round1 审查] 独立交叉比对与深度审查报告 (达成 clean 1/3)

### 1. 缺陷修复与语义补齐
- **BytecodeArray 内存安全修复**: `clear_padding` 原实现误将 `data_size = K_HEADER_SIZE + len` 传入 `size_for(data_size)`，导致重复累加 `K_HEADER_SIZE` 并越界多写 64 字节 0 值造成内存破坏。已就地修正为 `size_for(len) - data_size`。
- **BytecodeArrayBuf / CodeBuf 封装完善**: 新增 `BytecodeArrayBuf` 并为 `CodeBuf` / `CodeWrapperBuf` / `BytecodeArrayBuf` 实现 `Deref`，统一测试与脱堆构造契约。
- **CodeKind 兼容函数补齐**: 补齐 `to_code_kind_flag`、`From<CodeKind> for CodeKindFlag`、`contains_kind` 及 `code_kind_to_code_kind_flag` 顶层转换。
- **AbstractCode / Code 方法对齐**: 补齐 `source_statement_position` 语义面与 baseline 码判定（CHECK_NE 转 Invariant 错误）。
- **DeoptimizationData 边界防御**: `deopt_count` 增加空去优化数据（`len == 0`）返回 `Ok(0)` 的安全分支，避免无效 OutOfBounds 报错。

### 2. 门禁实测
- `rg "\[allow"`: 0 处绕过。
- `CARGO_TARGET_DIR=target/agent_w9or1 cargo clippy -p v8-objects-rest --all-targets -- -D warnings`: 0 error, 0 warning。
- `CARGO_TARGET_DIR=target/agent_w9or1 cargo nextest run -p v8-objects-rest`: 10/10 PASS。

### 3. 结论
代码规范 12 项原则全量合规，0 缺陷 / 0 遗漏 / 测试全过，round1 达成 **clean 1/3**。
