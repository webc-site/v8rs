[English](#en) | [中文](#zh)

---

<a name="en"></a>

# v8-codegen-x64 · wave1 ledger

Rust 1:1 replica of `v8/src/codegen/x64/`. wave1 = zero-coupling pure data plane.

- [wave1 delivered](#wave1-delivered)
- [Script counts](#script-counts)
- [Deferred (wave2 / second tier)](#deferred-wave2-second-tier)

## wave1 delivered

| Unit | C++ source | Product | Script |
|---|---|---|---|
| Full constant table | `src/codegen/x64/constants-x64.h` | `src/constants.rs` | `tool/src/codegen/x64/constants-x64.h.js` |
| Register data plane | `src/codegen/x64/register-x64.h` + `src/codegen/register-base.h` | `src/registers.rs` | `tool/src/codegen/x64/register-x64.h.js` |
| Golden tests | same headers (expected values transcribed independently from C++) | `tests/x64_data.rs` (13 cases; crate total 21/21 — the other 8 belong to a concurrent seat's `tests/main.rs`) | — |

Ticket-vs-snapshot drift notes:

- `constants-x64.h` in this snapshot holds exactly 2 constants (`kRootRegisterBias=128`, `kMaxPCRelativeCodeRangeInMB=2048`).
- `OperandSize` / `TrampolineFlags` named in the ticket do not exist anywhere in this V8 tree for x64; nothing invented.
- `kNoReg` is really `RegisterBase::kCode_no_reg = -1` (register-base.h) → mirrored as `K_CODE_NO_REG`.
- `registers-x64.cc/.h` is actually `register-x64.h` here; name tables live in-header via the `DEFINE_REGISTER_NAMES` X-macro.

## Script counts

`constants-x64.h.js` — constants 2/2, per-value + per-type golden check, naming-map asserts, idempotent reruns (output passes through nightly rustfmt fixed point).

`register-x64.h.js` — 34 anchors plus full sequence checks:

- GENERAL_REGISTERS 16; ALWAYS_ALLOCATABLE 11; MAYBE_ALLOCATABLE both branches 0/1 (r14)
- XMM base/ext 16/16; YMM base/ext 16/16; ALLOCATABLE_DOUBLE 15
- Register calling-convention aliases 24; Double/YMM aliases 3; kPtrComprCageBaseRegister branches 2
- kCArgRegs WIN/SYSV = 4/6; callee-save GPR WIN/SYSV = 7/5; callee-save FP WIN/SYSV = 10/0
- Scalars kNumRegs=16 / kWindowsHomeStackSlots=4 / kSimdMaskRegisters=false / kCode_no_reg=-1
- 7 header-aligned static assertions (1:1 with header `static_assert`s: sizeof<=int ×3, kAfterLast<=int8, kDoubleAfterLast==kYMMAfterLast both branches, cross-table length invariants); full product contains 19 `const _:` in total (the other 12 are script-added naming-map/inverse invariants)

## Deferred (wave2 / second tier)

- **macro-assembler-x64 family** (`macro-assembler-x64.{h,cc}`, `assembler-x64.{h,cc}`, `assembler-x64-inl.h`): blocked on green codegen-core + second-tier plan. `src/{assembler,operand}.rs`, the `v8_codegen_core` dep in `error.rs`/`Cargo.toml`, and `deferred/wave2_assembler_tests.rs` are concurrent-seat in-flight work, not wave1 scope.
- **diagnostics/x64 disassembler plane**: wave2.
- **asmkit kernel evaluation items**: `reglist-x64.h` (MacroAssembler-coupled methods, FormatValue/Print), `interface-descriptors-x64-inl.h`, `fma-instr.h`/`sse-instr.h` opcode enums, `cpu-x64.cc` — decision left to wave2.
- **Config switches**: APX_F `high_bit` masked branch already taken (numerically equal to the cc:72 non-masked branch on legal domain 0-15, see review round 1 D4); AVX10 (`K_*_AFTER_LAST_AVX10` materialized as consts, `from_code` bounds follow default build); `ArgumentPaddingSlots` build flag taken as function parameter (`V8_X64_16BYTE_STACK_ALIGNMENT_BOOL`); `kFPAliasing` (AliasingKind belongs to platform-neutral base, hook up after codegen-core lands); `ReassignRegister` generic helper (needs trait design).
- **tests/**: dynamic halves of `v8/test/unittests` register/disasm cases deferred; wave1 keeps only statically verifiable golden tables.


---

<a name="zh"></a>

# v8-codegen-x64 · wave1 台账 (v8-codegen-x64)

1:1 复刻 `v8/src/codegen/x64/` 的 Rust crate。wave1 = 零耦合纯数据面。

- [wave1 完成清单](#wave1-完成清单)
- [脚本对数](#脚本对数)
- [推迟清单 (wave2 / 二梯队)](#推迟清单-wave2-二梯队)
- [门禁](#门禁)

## wave1 完成清单

| 单元 | C++ 源 | 产物 | 脚本 |
|---|---|---|---|
| 全量常量表 | `src/codegen/x64/constants-x64.h` | `src/constants.rs` | `tool/src/codegen/x64/constants-x64.h.js` |
| 寄存器数据面 | `src/codegen/x64/register-x64.h` + `src/codegen/register-base.h` + `src/codegen/x64/reglist-x64.h:15` (RegList=RegListBase<Register> 别名) | `src/registers.rs` | `tool/src/codegen/x64/register-x64.h.js` |
| 金标准测试 | 同上 (期望值独立抄自 C++ 头) | `tests/x64_data.rs` (13 项; 全 crate 21/21, 另 8 项属并发席 `tests/main.rs`) | — |

注 (票面 vs 快照版本漂移):

- 本快照 `constants-x64.h` 全量即 2 个常量 (`kRootRegisterBias=128`、`kMaxPCRelativeCodeRangeInMB=2048`)。
- 票面提到的 `OperandSize` / `TrampolineFlags` 在本快照 **codegen/x64 面零命中** (v8/src/codegen 全域 grep 为 0)，未凭空造表;注: `enum class OperandSize` 存在于 interpreter 面 (v8/src/interpreter/bytecode-operands.h:84), 归 v8-interpreter 票, 与本 crate 无关 (round8 D1 措辞收窄)。
- `kNoReg` 实名 `RegisterBase::kCode_no_reg = -1` (register-base.h)，已 1:1 落 `K_CODE_NO_REG`。
- `registers-x64.cc/.h` 在本快照实名 `register-x64.h`；寄存器名表经 `DEFINE_REGISTER_NAMES` X-macro 内联于头内。

## 脚本对数

`constants-x64.h.js` — 常量 2/2，值+类型逐条对表，命名映射断言，复跑幂等 (产物过 nightly rustfmt 固定点)。

`register-x64.h.js` — 锚点/结构断言 34 条 + 列表逐序列对表，对数：

- GENERAL_REGISTERS 16；ALWAYS_ALLOCATABLE 11；MAYBE_ALLOCATABLE 双支 0/1 (r14)
- XMM base/ext 16/16；YMM base/ext 16/16；ALLOCATABLE_DOUBLE 15
- Register 调用约定别名 24；Double/YMM 别名 3；kPtrComprCageBaseRegister 双支 2
- kCArgRegs WIN/SYSV = 4/6；callee-save GPR WIN/SYSV = 7/5；callee-save FP WIN/SYSV = 10/0
- 标量 kNumRegs=16 / kWindowsHomeStackSlots=4 / kSimdMaskRegisters=false / kCode_no_reg=-1
- 头对齐静态断言 7 条 (逐条对齐头内 `static_assert`: sizeof<=int ×3、kAfterLast<=int8、kDoubleAfterLast==kYMMAfterLast 双支、跨表 len 不变式)；生成物内 `const _:` 全部共 19 条 (7 条之外为脚本自加的命名映射/互逆不变式)

## 推迟清单 (wave2 / 二梯队)

- **macro-assembler-x64 全族** (`macro-assembler-x64.{h,cc}`、`assembler-x64.{h,cc}`、`assembler-x64-inl.h`)：依赖 codegen-core 绿 + 二梯队规划。`deferred/wave2_assembler_tests.rs` 与 `src/{assembler,operand}.rs`、`error.rs` 对 `v8-codegen-core` 的依赖为并发席位在飞内容，非 wave1 交付面。
- **diagnostics/x64 反汇编面** (decoder / disassembler 对接)：留 wave2。
- **asmkit 内核评估项**：`reglist-x64.h` (RegList 与 MacroAssembler 耦合方法、FormatValue/Print 面)、`interface-descriptors-x64-inl.h`、`fma-instr.h` / `sse-instr.h` 指令枚举表、`cpu-x64.cc`：标注留 wave2 决策。
- **RegList 别名族** (round2 补登)：`DoubleRegList`/`Simd128RegList` (reglist-x64.h:17-18)、`kJSCallerSaved`/`kCallerSaved` 常量表 (reglist-x64.h:22-34) 未落数据面；注意并发席 `src/reglist.rs` 死码 (u64 位宽同错) 待其自行收敛。
- **配置开关面**：APX_F `high_bit` 掩码支已采 (合法域 0-15 与 cc:72 非掩码支数值等价，见复审 round1 D4)；AVX10 (`K_*_AFTER_LAST_AVX10` 已由常量物化，但 from_code 校验按默认构建)、`ArgumentPaddingSlots` 的构建开关以函数入参形态承接 (`V8_X64_16BYTE_STACK_ALIGNMENT_BOOL`)；`kFPAliasing` (AliasingKind 属平台无关基座，待 codegen-core 落位后对接)；`ReassignRegister` 泛型 helper (需 trait 设计)。
- **tests/ 静态验证面**：对齐 `v8/test/unittests` 中 register/disasm 相关用例的动态面留 wave2，wave1 仅收录可静态验证的金标准对表。

## 门禁

- `bun tool/src/codegen/x64/{constants-x64,register-x64}.h.js` 连跑两次产物字节一致
- `cargo nextest run -p v8-codegen-x64 --all-features` 全绿
- `cargo clippy -p v8-codegen-x64 --all-targets --all-features -- -D warnings` RC=0
- `cargo +nightly fmt -p v8-codegen-x64 -- --check` 净 (脚本内置 rustfmt 固定点)

