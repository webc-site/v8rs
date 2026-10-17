[English](#en) | [中文](#zh)

---

<a name="en"></a>

# v8-builtins-x64

Transpiled bindings for V8 x64 assembly builtins **entry surface** (names, enumerations, registration scaffolding). This crate does not contain assembly function bodies; "clean 3/3 freeze" in logs refers strictly to entry surface fidelity, not full assembly implementation. Deferral ledger and test mappings are recorded below (Wave 7 W7BXM remediation).

- [Covered Surface (Entry Surface)](#covered-surface-entry-surface)
- [Deferral Ledger (Fully Deferred to Subsequent Waves)](#deferral-ledger-fully-deferred-to-subsequent-waves)
- [Semantic Grouping Statement](#semantic-grouping-statement)
- [Test Mapping](#test-mapping)
- [Dependency Ledger](#dependency-ledger)

## Covered Surface (Entry Surface)

- `Builtin` enum with 73 items bijectively mapping to original `void Builtins::Generate_*` in `v8/src/builtins/x64/builtins-x64.cc:44..5741`, aligned identically with `ASM_GENERATE_ENTRIES`;
- 73 zero-cost `const fn` factory functions; `ArgumentsElementType`, `OsrSourceTier`, and `DebugBreakKind` direct enums; `CLASSES` with 3 class skeleton entries;
- `root_register()` bound to r13 (`v8_codegen_x64::R13`, code == 13, per `v8/src/codegen/x64/register-x64.h:387`);
- `global_id` linked via `v8-builtins-core` global table: 37 resolvable / 36 non-resolvable (identical resolvable subset with arm64 seat);
- Platform-specific entries `is_x64_specific` covering `{AdaptShadowStackForDeopt, DumpFrame}`;
- Top-level `#![forbid(unsafe_code)]`, 100% Safe Rust.

## Deferral Ledger (Fully Deferred to Subsequent Waves)

1. **Assembly Function Bodies**: 73 `Generate_*` MacroAssembler instruction sequences (rax-r15 register operations, rbp-rsp frame prologue/epilogue, branch alignment, RIP-relative constants) are not expanded;
2. **Frame Constants**: `v8/src/execution/x64/frame-constants-x64.h/.cc` deferred (note: `frame-inl-x64.h` does not exist in V8 tree);
3. **Execution / Simulator Semantics**: x64 execution requires x86-64 hardware or emulator; execution semantics belong to simulator wave;
4. **Register Allocation**: `RegisterConfiguration` traversal deferred to assembly body wave;
5. **Cc Registration Surface**: `CcBuiltin` and `CC_BUILTINS` are placeholders for deferred assembly registration;
6. **Boundary Statement**: This crate covers only entry names and dispatch metadata; assembly instruction bodies are deferred.

## Semantic Grouping Statement

- 6 categorization predicates (`is_js_entry_or_trampoline`, etc., partitioning 8+10+15+8+11+21=73 mutually exclusive and exhaustive) are heuristic classifications; ContinueTo* entries are grouped into `is_runtime_or_aux`;
- `expected_frame_type` is refined for 21 entries based on original frame macros; unrefined members use heuristic fallbacks.

## Test Mapping

Original V8 has no standalone `test/cctest/test-builtins-x64.cc`; builtins are indirectly verified via full test suites. Tests here provide regression fidelity:

- `tests/main.rs` (5) + `tests/builtins_x64_regression.rs` (4) (9 total): Zero heap allocation streaming iterators, compile-time constant slices, bijective mapping verification, exact 37/36 membership, refined frame types, root register verification;
- Functional execution belongs to milestone rs8.

## Dependency Ledger

- `thiserror` dependency removed following deletion of dead `error.rs`;
- Clippy: `cargo clippy -p v8-builtins-x64 --all-targets` passes with 0 warnings;
- Generation scripts: `tool/src/builtins/x64/builtins-x64.cc.js`, `tool/src/builtins/x64/builtins-x64.js`, and `tool/src/builtins/builtins-x64.js` updated and 100% idempotent.

---

<a name="zh"></a>

# v8-builtins-x64

x64 汇编内建的 **entry 名面** 转写 (名表/枚举/登记骨架)。本 crate 不含任何汇编函数体, 日志中 "clean 3/3 冻结" 仅指名面忠实性, 不表示汇编面完成。推迟面与测试口径台账如下 (轮7 W7BXM 整改登记)。

- [本席已覆盖 (名面)](#本席已覆盖-名面)
- [推迟面台账 (整体推迟, 现波零覆盖)](#推迟面台账-整体推迟-现波零覆盖)
- [语义口径声明](#语义口径声明)
- [测试映射口径](#测试映射口径)
- [依赖面登记](#依赖面登记)

## 本席已覆盖 (名面)

- `Builtin` 枚举 73 项与原版 `void Builtins::Generate_*` (v8/src/builtins/x64/builtins-x64.cc:44..5741) 双射, 与 `ASM_GENERATE_ENTRIES` 同名同序;
- 73 项零成本 `const fn` 工厂函数; `ArgumentsElementType`/`OsrSourceTier`/`DebugBreakKind` 枚举直译 (对证 cc:58/2943/3364); `CLASSES` 3 项类名骨架;
- `root_register()` 绑定 r13 (`v8_codegen_x64::R13`, code()==13, 对证 v8/src/codegen/x64/register-x64.h:387 `kRootRegister = r13`);
- `global_id` 经 v8-builtins-core 全局表: 可解析 37 项 / 不可解析 36 项 (与 arm64 席可解析集完全同集);
- 平台独有面 `is_x64_specific` 恰为 {AdaptShadowStackForDeopt (cc:3168), DumpFrame (cc:5388)} (builtins-arm64.cc 无同名 Generate_, 73=71+2 对时一致);
- 顶层 `#![forbid(unsafe_code)]`, 纯 Safe Rust。

## 推迟面台账 (整体推迟, 现波零覆盖)

1. **汇编函数体**: `Generate_*` 共 73 个的 MacroAssembler 指令序列 (rax-r15 寄存器操作 / rbp-rsp 帧建立序 / 分支对齐 / RIP 相对常量) 全部不展开, 随汇编体填波转写;
2. **帧常量**: 原版 `v8/src/execution/x64/frame-constants-x64.h/.cc` 未转写 (注意: 规程曾误引 `frame-inl-x64.h`, 该文件在本树不存在, 帧型枚举面实为 `v8/src/execution/frames.h:119-133` 的 STACK_FRAME_TYPE_LIST);
3. **真码执行 / 仿真器口径**: x64 真码需 x86-64 真机或非仿真配置才可执行核验, 仿真口径挂 v8-execution-simulator 席, 本 crate 不承诺运行时行为;
4. **寄存器分配面**: 汇编内建内 `RegisterConfiguration` 可分配寄存器遍历 (如 BaselineOutOfLinePrologue cc:1870/1961 与 DumpFrame cc:5397 现场保存) 现仅名面登记, 真分配策略随汇编体波;
5. **cc 登记面**: `CcBuiltin`/`CC_BUILTINS` 为汇编体推迟面占位 (空表是设计态非漏填) — 原版 builtins-x64.cc 无独立登记宏, 汇编面真值即 `Generate_*` 定义本身, 逐条登记数据随函数体填波派生;
6. **名面/汇编面边界声明**: 本 crate 只覆盖 entry 名面 + 调度元数据 (名表/谓词分区/帧型提示); 日志票面 "JSEntry/… 汇编内建" 措辞仅指名面登记, 逐指令面 (Assembler 序列/帧建立序/栈操作) 零覆盖, 二者边界以上述 1-5 为准。

## 语义口径声明

- 6 个分类谓词 (`is_js_entry_or_trampoline` 等, 分区 8+10+15+8+11+21=73 互斥完备) 为自研**启发式归组, 非 V8 真分类**; 成员归属已按原版 `Generate_` 帧宏面订正: ContinueTo* 四项 (CEntry 续执行桩, cc:2009-2085 按 BuiltinContinuationFrame 弹出续跑) 自 deopt 组迁出, `is_c_entry_or_api` 随之订正组名为 `is_runtime_or_aux` (运行时调用/回调/辅助桩, 涵盖 DoubleToI 转换桩 cc:4753、DumpFrame dump 桩 cc:5388);
- `expected_frame_type` 已按原版帧宏逐项精化 21 条 (出处见 entry.rs 臂注释与 tests/builtins_x64_regression.rs 钉死用例: JSConstructEntry→CONSTRUCT_ENTRY cc:561-563、双蹦床实开 INTERNAL cc:620、RunMicrotasksTrampoline 无帧尾调 cc:702-706、ResumeGeneratorTrampoline INTERNAL cc:894/908/919、RestartFrameTrampoline 处理 INTERPRETED 帧 cc:5764、Baseline 族→BASELINE cc:1899/2003/3060-3067、Maglev 栈检查→MAGLEV cc:3073-3080 等); 其余成员 (如 PushArgsThen{Call,Construct}Impl/ConstructForwardAllArgsImpl) 仍为启发式兜底, **真帧型随汇编体填波替换**, 下游不得当权威语义引用。

## 测试映射口径

原版 **无** `test/cctest/test-builtins-x64.cc` 专测 — cctest 全域无直接引用这些蹦床/入口名的用例 (V8 汇编内建经全量 mjsunit/cctest 间接覆盖), 故规程 "原版测试 1:1" 前提不成立, 本席测试为名面忠实性自建回归:

- `tests/main.rs` (5) + `tests/builtins_x64_regression.rs` (4) 共 9 例: 消除单测运行时堆分配 (流式遍历、编译期常量切片直接比对、windows(2) 零分配去重检验); 冻结原版 73 `Generate_` 名集断双射 (防增删/改名漂移)、global_id 可解析 37 项 + 不可解析 36 项双集精确钉死、谓词分区互斥完备、精化帧型面、root_register code==13;
- 功能面验证走间接 mjsunit 口径, 挂 **rs8 里程碑** (todo.md:131 "测试 随 M4 闭环 + cctest" 口径的同源结论), 不在本 crate 现波承诺。

## 依赖面登记

- `thiserror` 冗余依赖已清理: `Error`/`Result` 死面 (src 内零构造零消费) 已删, `Cargo.toml` 中 `thiserror` 依赖已移除;
- clippy 门禁: `cargo clippy -p v8-builtins-x64 --all-targets` 0 警告 0 诊断全绿通过;
- 生成脚本: `tool/src/builtins/x64/builtins-x64.cc.js`、`tool/src/builtins/x64/builtins-x64.js` 与 `tool/src/builtins/builtins-x64.js` 已同步更新, 包含 strum 派生、CcBuiltin 推迟注记、精化帧型与 `is_runtime_or_aux` 谓词, 复跑 100% 幂等。

