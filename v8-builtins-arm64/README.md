[English](#en) | [中文](#zh)

---

<a name="en"></a>

#

---

<a name="zh"></a>

# v8-builtins-arm64

arm64 汇编内建的 **entry 名面** 转写 (名表/枚举/登记骨架)。本 crate 不含任何汇编函数体, 日志中 "clean 3/3 冻结" 仅指名面忠实性, 不表示汇编面完成。推迟面与测试口径台账如下 (轮7 W7BAM 整改登记)。

- [本席已覆盖 (名面)](#本席已覆盖-名面)
- [推迟面台账 (整体推迟, 现波零覆盖)](#推迟面台账-整体推迟-现波零覆盖)
- [语义口径声明](#语义口径声明)
- [测试映射口径](#测试映射口径)
- [依赖面登记](#依赖面登记)

## 本席已覆盖 (名面)

- `Builtin` 枚举 71 项与原版 `void Builtins::Generate_*` (v8/src/builtins/arm64/builtins-arm64.cc) 双射, 与 `ASM_GENERATE_ENTRIES` 同名同序;
- 71 项零成本 `const fn` 工厂函数; `OsrSourceTier`/`DebugBreakKind` 枚举直译; `CLASSES` 4 项类名骨架;
- `root_register()` 绑定 x26 (`v8_codegen_arm64::K_ROOT_REGISTER`, code()==26);
- `global_id` 经 v8-builtins-core 全局表: 可解析 37 项 / 不可解析 34 项 (全 Wasm* + Call/Construct Impl 变体);
- 顶层 `#![forbid(unsafe_code)]`, 纯 Safe Rust。

## 推迟面台账 (整体推迟, 现波零覆盖)

1. **汇编函数体**: `Generate_*` 共 71 个的 MacroAssembler 指令序列 (x9-x28 寄存器操作 / sp-fp 帧建立 / 分支对齐 / PC 相对常量) 全部不展开, 随汇编体填波转写;
2. **帧常量**: 原版 `v8/src/execution/arm64/frame-constants-arm64.h/.cc` 未转写 (注意: 规程曾误引 `frame-inl-arm64.h`, 该文件不存在);
3. **MOPs 宏**: baseline/maglev  bytecode 宏汇编面未转写;
4. **寄存器分配**: `RegisterAllocator`/`Scoped` 现仅类名骨架 (`CLASSES` 登记), 真分配策略与约束随汇编体波;
5. **真码执行 / 仿真器口径**: arm64 真码需 V8 仿真器或非仿真 arm64 主机才可执行核验, 口径挂 v8-execution-simulator 席 (另见 target/wave6/W6BA.txt 登记), 本 crate 不承诺运行时行为;
6. **cc 登记面**: `CcBuiltin`/`CC_BUILTINS` 为汇编体推迟面占位 (空表是设计态非漏填), 原版 cc 宏登记数据随汇编体填波派生。

## 语义口径声明

`expected_frame_type` 与 6 个分类谓词 (`is_js_entry_or_trampoline` 等, 分区 8+10+15+11+6+21=71 互斥完备) 为**启发式归组, 非 V8 真帧型/真分类** (如 Maglev 栈检查归 Interpreted、Resume/Restart 蹦床归 Entry 均不契合真实帧型); 真帧型面随汇编体填波替换, 下游不得当作权威语义引用。

## 测试映射口径

原版 **无** `cctest/test-builtins-arm64*` 专测 — V8 汇编内建经全量 mjsunit/cctest 间接覆盖, 非 arm64 主机需仿真器。故 "原版测试 1:1" 前提不成立, 本席测试为名面忠实性自建回归:

- `tests/main.rs` + `tests/builtins_arm64_regression.rs` (7/7): 冻结原版 71 `Generate_` 名集断双射 (防增删/改名漂移)、global_id 可解析 37 项精确钉 + 不可解析 34 项、谓词分区互斥完备;
- 功能面验证走间接 mjsunit 口径, 挂 **rs8 里程碑** (todo.md:131 "测试 随 M4 闭环 + cctest" 口径的同源结论), 不在本 crate 现波承诺。

## 依赖面登记

- `thiserror` 依赖已冗余清理: `Error`/`Result` 死面 (src 内零构造零消费) 已删, `Cargo.toml` 中 `thiserror` 依赖已移除;
- clippy 门禁: `cargo clippy -p v8-builtins-arm64 --all-targets` 0 警告通过。

