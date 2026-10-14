[English](#en) | [中文](#zh)

---

<a name="en"></a>

# v8-deoptimizer

Rust port of V8 deoptimization subsystem and translated state infrastructure.

- [Key Features](#key-features)

## Key Features

- **Deoptimization Reasons & Kinds**:
  - `DeoptimizeReason` (68 entries): Full coverage of deopt reasons including bounds check, division by zero, missing feedback, migration failures.
  - `LazyDeoptimizeReason` (17 entries): Dedicated lazy deoptimization reasons.
  - `DeoptimizeKind`: `Eager`, `LazyAfterFastCall`, `Lazy`.
- **Translation Bytecode Opcodes (`TranslationOpcode`)**:
  - 47 entries covering JS frames (4), general frames (11), and data translation items (36).
  - Operand capacity up to 6 with frame/begin predicates.
- **Deoptimized Frame & Materialized Store**:
  - `DeoptimizedFrameInfo`: Snapshot parameters and expression stack information.
  - `MaterializedObjectStore`: Frame materialized objects store.
  - Architecture exit constants: x64 and arm64 eager/lazy exit sequence sizes.
- **100% Safe Rust**:
  - Zero unsafe, zero panics, zero naked unwraps/expects, zero dead code, zero warning suppressions.

---

<a name="zh"></a>

# v8-deoptimizer

反优化（Deoptimization）子系统与物化状态基础设施的 Rust 1:1 端口。

- [核心特性](#核心特性)

## 核心特性

- **反优化原因与种类**：
  - `DeoptimizeReason`（68 项）：精确覆盖越界、除零、未命中类型反馈、类型演进失败等全量触发原因，包含消息描述与名称映射。
  - `LazyDeoptimizeReason`（17 项）：惰性反优化独立枚举。
  - `DeoptimizeKind`：`Eager`、`LazyAfterFastCall`、`Lazy`。
- **翻译字节码操作码（TranslationOpcode）**：
  - 47 项完整操作码表，区分 JS 帧（4 项）、通用帧（11 项）与数据项（36 项）。
  - 操作数容量上限 6，提供 `is_frame_opcode`, `is_js_frame_opcode`, `is_interpreter_frame_opcode`, `is_begin` 判定。
- **反优化帧与物化存储**：
  - `DeoptimizedFrameInfo`：反优化帧快照参数与表达式栈信息。
  - `MaterializedObjectStore`：物化对象栈存储骨架。
  - 架构分派常量：x64 与 arm64 架构下 eager / lazy 反优化退出指令序列尺寸常量。
- **100% Safe Rust**：
  - 零 unsafe、零 panic、零裸 unwrap/expect、零 dead code、零 allow 警告绕过。
