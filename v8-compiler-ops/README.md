[English](#en) | [中文](#zh)

---

<a name="en"></a>

# v8-compiler-ops

Rust transcription of V8 `src/compiler` operator system (Common / Machine / Simplified / JS operators + Verifier). Depends on v8-compiler-graph and v8-zone.

- [Generator scripts (tool/src/compiler/, re-run = regenerate baseline, then cargo fmt)](#generator-scripts-toolsrccompiler-re-run-regenerate-baseline-then-cargo-fmt)
- [Module map](#module-map)
- [Tests (16/16 passing)](#tests-1616-passing)
- [Ledger](#ledger)

## Generator scripts (tool/src/compiler/, re-run = regenerate baseline, then cargo fmt)

| Script | Upstream source | Output | In-script assertions |
|---|---|---|---|
| `common-operator.h.js` | `common-operator.h/.cc` | `common_operator.rs` | static op table + parameterized op counts |
| `machine-operator.h.js` | `machine-operator.h/.cc` + `write-barrier-kind.h` | `machine_operator.rs` | 56 pure ops from PURE_BINARY_OP_LIST_32/64, field by field |
| `machine-operator.cc.js` | `machine-operator.cc:775-884` (Pair tail of MACHINE_PURE_OP_LIST) + `machine-operator.h:573-578` | `machine_pair_operator.rs` | cc entries 6 == h no-arg getters 6 == expected 6; props kNoProperties and counts (4/3, 0, 2) asserted per field, throws on mismatch |
| `simplified-operator.h.js` | `simplified-operator.h/.cc` | `simplified_operator.rs` | simplified op table |
| `js-operator.h.js` | `js-operator.h/.cc` | `js_operator.rs` | JS op macro counts (incl. FeedbackVector value input) |

## Module map

| Upstream | Rust module | Notes |
|---|---|---|
| `machine-operator.*` (PURE_BINARY_OP_LIST) | `machine_operator.rs` | global pure-op cache + Load/Store/Unaligned parameterized ops via zone |
| `machine-operator.*` (Pair binary tail entries, cc:879-884) | `machine_pair_operator.rs` | Int32Pair{Add,Sub,Mul} (4 value-in / 2 value-out), Word32Pair{Shl,Shr,Sar} (3/2); kPure; separate cache with `impl MachineOperatorBuilder` getters (module-split adaptation of the single C++ cache) |
| `verifier.h/.cc` (core subset) | `verifier.rs` | count-driven node/graph contracts; new table entries verified automatically |

## Tests (16/16 passing)

`machine_operator_unittest.rs` covers the 6 Pair table entries field by field, entry count == 6 and Pair-vs-single opcode discriminant mutual exclusion; `verifier_unittest.rs` covers Pair nodes with 4/3 inputs against total_input_count plus missing/extra edge rejection; other files cover common/simplified/js families.

## Ledger

- Deferred (evidence and ownership):
  - `StorePairRepresentation` + `TryStorePair` (h:192-212/1257, cc:971-1035/1487-1517/2207): the pair-representation table switches at compile time across V8_TARGET_ARCH_64_BIT x V8_COMPRESS_POINTERS (37/17/0 entries); this crate's builder has no Flags/arch input yet — landing a fixed table would be a fake fix. Blocked on MachineOperatorBuilder::Flags (kLoadStorePairs) infra.
  - mips64 `Lsa/Dlsa` instruction bodies and `kLsaSaBits`/`LsaSaValue` bitfields belong to arch crates (v8-codegen-mips64 / v8-compiler-backend in todo.md), not the operator tables.
  - Overlapping string copy length logic (e.g. `CodeStubAssembler::MoveElements`, codegen/code-stub-assembler.h:2490) belongs to v8-codegen-csa.


---

<a name="zh"></a>

# v8-compiler-ops

V8 `src/compiler` 算子体系 (Common / Machine / Simplified / JS 算子 + OperatorVerifier) 的 Rust 转写。依赖 v8-compiler-graph (IrOpcode/Operator/Graph)、v8-zone。

- [转换脚本 (tool/src/compiler/, 复跑 = 重生成基线 → cargo fmt)](#转换脚本-toolsrccompiler-复跑-重生成基线-cargo-fmt)
- [模块映射](#模块映射)
- [测试映射 (16/16 通过)](#测试映射-1616-通过)
- [台账](#台账)

## 转换脚本 (tool/src/compiler/, 复跑 = 重生成基线 → cargo fmt)

| 脚本 | 原版来源 | 产出 | 内置对数 |
|---|---|---|---|
| `common-operator.h.js` | `src/compiler/common-operator.h/.cc` | `src/common_operator.rs` | 静态算子表 + 参数化算子计数 |
| `machine-operator.h.js` | `src/compiler/machine-operator.h/.cc` + `write-barrier-kind.h` | `src/machine_operator.rs` | PURE_BINARY_OP_LIST_32/64 共 56 纯算子表项逐字段 |
| `machine-operator.cc.js` | `src/compiler/machine-operator.cc:775-884` (MACHINE_PURE_OP_LIST 尾部 Pair 表项) + `machine-operator.h:573-578` | `src/machine_pair_operator.rs` | cc 提取 6 == h 无参 getter 声明 6 == 预期 6; 属性 kNoProperties、计数 (4/3,0,2) 逐字段断言, 任一不符即抛错 |
| `simplified-operator.h.js` | `src/compiler/simplified-operator.h/.cc` | `src/simplified_operator.rs` | simplified 算子表 |
| `js-operator.h.js` | `src/compiler/js-operator.h/.cc` | `src/js_operator.rs` | JS 算子宏族计数 (含 FeedbackVector 值输入) |

## 模块映射

| 原版 (v8/src/compiler) | Rust 模块 | 说明 |
|---|---|---|
| `machine-operator.h/.cc` (PURE_BINARY_OP_LIST_32/64) | `src/machine_operator.rs` | MachineOperatorGlobalCache + 纯算子 getter; Load/Store/Unaligned* 参数化算子走 zone |
| `machine-operator.h/.cc` (Pair 二元表项 cc:879-884) | `src/machine_pair_operator.rs` | Int32Pair{Add,Sub,Mul} (4 值入→2 值出)、Word32Pair{Shl,Shr,Sar} (3 值入→2 值出); kPure; 独立缓存 MachinePairOperatorGlobalCache, getter 以 `impl MachineOperatorBuilder` 分片扩展 (原版单缓存的模块切分适配) |
| `common-operator.*` / `simplified-operator.*` / `js-operator.*` | 对应同名 rs | 各算子族 |
| `verifier.h/.cc` (核心子集) | `src/verifier.rs` | 节点输入计数 / Effect-Control-FrameState 拓扑契约; 计数驱动, 新表项自动纳入校验 |

## 测试映射 (16/16 通过)

| Rust 测试 | 覆盖 |
|---|---|
| `tests/machine_operator_unittest.rs` | 纯算子属性/计数、Load-Store、**Pair 二元表项 6 项逐字段 + 表项计数 6 + opcode 枚举互斥 (Pair vs 单值)** |
| `tests/verifier_unittest.rs` | 图契约用例, **含 Pair 节点 4/3 输入与 total_input_count 对齐、缺边/多边拒绝** |
| 其余 unittest | common/simplified/js 算子族 |

## 台账

- 遗留登记 (本 crate 无法 1:1 落地部分, 证据与归属):
  - `StorePairRepresentation` + `MachineOperatorBuilder::TryStorePair` (machine-operator.h:192-212/1257, cc:971-1035/1487-1517/2207): 表按 `V8_TARGET_ARCH_64_BIT`×`V8_COMPRESS_POINTERS` 三态 (37/17/0 项) 编译期切换, 本 crate builder 尚无 Flags/架构入参, 强行落表即伪装修复; 待 MachineOperatorBuilder::Flags (kLoadStorePairs) 基座落地后补。
  - mips64 `Lsa/Dlsa` 指令本体与 `kLsaSaBits`/`LsaSaValue` 位域 (codegen/mips64, diagnostics): 属架构 crate (todo: v8-codegen-mips64 / v8-compiler-backend) 域, 非算子表。
  - 字符串重叠拷贝长度 (如 `CodeStubAssembler::MoveElements` 区间重叠处理, codegen/code-stub-assembler.h:2490): 属 v8-codegen-csa 域。

