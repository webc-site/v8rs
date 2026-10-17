[English](#en) | [中文](#zh)

---

<a name="en"></a>

# v8-codegen-arm64 — 1:1 port of V8 arm64 code generation

- [Module mapping](#module-mapping)
- [Deviations](#deviations)
- [Deferred (wave2: gated on v8-codegen-core settling)](#deferred-wave2-gated-on-v8-codegen-core-settling)
- [Encode kernel assessment: hand-written 1:1 vs asmkit (conclusion, for sign-off)](#encode-kernel-assessment-hand-written-11-vs-asmkit-conclusion-for-sign-off)
- [Gates](#gates)

## Module mapping

| v8 original | generator script (tool/src/codegen/arm64/) | crate src |
| --- | --- | --- |
| src/codegen/arm64/constants-arm64.h | constants-arm64.h.js | src/constants.rs |
| src/codegen/arm64/register-arm64.h (+ src/codegen/register-base.h DEFINE_REGISTER_NAMES) | register-arm64.h.js | src/registers.rs |

Naming: C++ `kFoo` → Rust `K_FOO` (aligned with v8-common globals.rs); enum variant `kFoo` → `Foo`; NEON/SVE acronyms protected. Re-running a script overwrites manual edits; run `cargo fmt -p v8-codegen-arm64` after regeneration.

## Deviations

1. Ticket says `registers-arm64.cc`: this fork has no such file; it is `register-arm64.h` (data plane, ported) and `register-arm64.cc` (VectorFormat transforms, logic plane, deferred).
2. `TrampolineFlags` / `StaticLinks` / `kFlagsOffset`: removed from current upstream constants-arm64.h; nothing to align — recorded as deviation, not omission.
3. Legacy print name `zeroreg`: code 31 stringifies to `x31` here (DEFINE_REGISTER_NAMES); the simulator alias table spells it `xzr` (execution/arm64/simulator, different layer).
4. `RegisterName(sp)` internal code 63 is out of bounds: upstream reads Names[63] (UB); we add the same bounds guard as v8-codegen-x64 and return "invalid".
5. High-bit literals (e.g. 0x8000000000000000) are UB in C++; kept as exact two's-complement via `0x..._u64 as i64`.
6. al/nv exclusion in NegateCondition preserved as debug_assert (mirrors upstream DCHECK).
7. Dependencies: the wave1 data plane (constants/registers) is itself zero-coupled; the crate's current v8-codegen-core dependency was introduced by the concurrent assembler wave — wave1 added no third-party dependency.

## Deferred (wave2: gated on v8-codegen-core settling)

- macro-assembler-arm64 family (MacroAssembler / full Operand semantics / MemOperand / RelocInfo coupling)
- decoder-arm64 + diagnostics/arm64 disassembly goldens (host is arm64, native per-instruction comparison)
- register-arm64.cc VectorFormat transform family
- instruction classes Fixed/FMask/Mask enums and assembler-arm64.cc emission
- CPURegList and AreAliased/AreSameFormat/AreConsecutive predicates
- constant-pool-arm64 / simulator register name tables (belong to v8-execution-simulator)

## Encode kernel assessment: hand-written 1:1 vs asmkit (conclusion, for sign-off)

**Conclusion: hand-written 1:1; do not adopt asmkit.**

1. Audit chain: v8rs requires every value to be mechanically countable against upstream; asmkit's field descriptors are a non-1:1 remapping of INSTRUCTION_FIELDS_LIST, which would break the audit chain.
2. Dependency risk: asmkit is a third-party package; any drift invalidates the whole ledger. The codegen layer currently keeps a zero third-party dependency policy.
3. Cost: the encode kernel is just offset/width/mask triples plus insert/extract helpers; the data plane is already machine-generated, so hand-writing is cheap and guarded by 88 field-identity tests.
4. Landing: encode kernel stays hand-written in-crate, delivered with the decoder golden wave (wave2).

## Gates

- `bun tool/src/codegen/arm64/constants-arm64.h.js && bun tool/src/codegen/arm64/register-arm64.h.js && cargo fmt -p v8-codegen-arm64` — idempotent (product hashes unchanged)
- `cargo nextest run -p v8-codegen-arm64` — all green (10 tests: constants golden, name↔code roundtrip, aliases, calling convention + concurrent assembler tests)
- `cargo clippy -p v8-codegen-arm64 --all-targets -- -D warnings` RC=0; `cargo fmt --check` clean


---

<a name="zh"></a>

# v8-codegen-arm64 — V8 arm64 代码生成 1:1 转写

- [模块映射](#模块映射)
- [偏差登记](#偏差登记)
- [推迟清单 (二梯队: 须待 v8-codegen-core 收敛)](#推迟清单-二梯队-须待-v8-codegen-core-收敛)
- [encode 内核评估: 自写 1:1 vs asmkit (结论, 供拍板)](#encode-内核评估-自写-11-vs-asmkit-结论-供拍板)
- [门禁](#门禁)

## 模块映射

| v8 原版 | 生成脚本 (tool/src/codegen/arm64/) | crate src |
| --- | --- | --- |
| src/codegen/arm64/constants-arm64.h | constants-arm64.h.js | src/constants.rs |
| src/codegen/arm64/register-arm64.h (+ src/codegen/register-base.h DEFINE_REGISTER_NAMES) | register-arm64.h.js | src/registers.rs |

约定: C++ `kFoo` → Rust `K_FOO` (对齐 v8-common globals.rs); 枚举变体 `kFoo` → `Foo`; NEON/SVE 缩略词保护; 复跑脚本覆盖修订, 复跑后须 `cargo fmt -p v8-codegen-arm64` 收敛。

## 偏差登记

1. 票面 `registers-arm64.cc`: 本 fork 无此文件; 对应 `register-arm64.h`(数据面已转写) 与 `register-arm64.cc`(VectorFormat 变换函数, 属逻辑面, 推迟)。
2. `TrampolineFlags` / `StaticLinks` / `kFlagsOffset`: 已从当前上游 constants-arm64.h 移除 (重构删除), 无值可对, 记偏差非漏转。
3. 旧上游打印名 `zeroreg`: 本 fork code 31 串化名为 `x31` (DEFINE_REGISTER_NAMES); 模拟器别名表显示 `xzr` (execution/arm64/simulator, 非本层)。
4. `RegisterName(sp)` 内部编码 63 越界: 原版 Names[63] 为未定义行为, 本实现按 v8-codegen-x64 同款边界守护返回 "invalid"。
5. 高位字面量 (如 0x8000000000000000) 在 C++ 为 UB/实现定义, Rust 以 `0x..._u64 as i64` 保真补码值。
6. kNegateCondition 的 al/nv 排除在 Rust 侧以 debug_assert 保留原 DCHECK 语义。
7. 依赖说明: wave1 数据面 (constants/registers) 本身零耦合; 本 crate 现依赖 v8-codegen-core 系并发席 assembler 波次引入, wave1 未新增第三方依赖。

## 推迟清单 (二梯队: 须待 v8-codegen-core 收敛)

- macro-assembler-arm64 全族 (MacroAssembler/Operand 全量/MemOperand 语义/RelocInfo 联动)
- decoder-arm64 + diagnostics/arm64 反汇编金标准 (host 即 arm64, 可原生逐指令比对)
- register-arm64.cc VectorFormat 变换族 (kFormatHalfWidth/DoubleWidth/FillQ 等)
- 指令类 Fixed/FMask/Mask 枚举与 assembler-arm64.cc 指令发射族
- CPURegList 与 AreAliased/AreSameFormat/AreConsecutive 谓词族
- constant-pool-arm64 / simulator 名表 (归 v8-execution-simulator 票)

## encode 内核评估: 自写 1:1 vs asmkit (结论, 供拍板)

**结论: 自写 1:1, 不引入 asmkit。**

1. 审计口径: v8rs 要求逐值可对数 (本票 102+88+32+22+23 全走真跑 golden); asmkit 的 field 描述符语义是上游 INSTRUCTION_FIELDS_LIST 的非 1:1 重映射, 引入即断审计链。
2. 依赖风险: asmkit 为第三方 npm 包 (V8 fork 自用), 版本漂移即全表失配; v8rs codegen 层现策略零第三方依赖。
3. 成本: 位域 encode 内核 = offset/width/mask 三件套 + 插入/提取函数, 数据面已由脚本机械化保证, 自写边际成本低且可用 88 字段恒等式测试守护。
4. 结论落点: encode 内核留 crate 内自写, 与 decoder 金标准同波次 (二梯队) 交付。

## 门禁

- `bun tool/src/codegen/arm64/constants-arm64.h.js && bun tool/src/codegen/arm64/register-arm64.h.js && cargo fmt -p v8-codegen-arm64` 复跑幂等 (产物哈希不变)
- `cargo nextest run -p v8-codegen-arm64` 全绿 (10 测试: 常量/名表互逆/别名/调用约定金标准 + 并发席 assembler 面)
- `cargo clippy -p v8-codegen-arm64 --all-targets -- -D warnings` RC=0; `cargo fmt --check` 净

