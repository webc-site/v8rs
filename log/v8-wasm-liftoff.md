# v8-wasm-liftoff 开发日志

> 2026-10-12 启动: Liftoff Wasm 基线编译器 (x64 / arm64 架构寄存器分配与常量表)

## 一、架构设计与模块组织

- `v8-wasm-liftoff/src/lib.rs`: 顶层 `#![forbid(unsafe_code)]`，扁平导出所有子模块类型。
- `liftoff_assembler_consts.rs`: 栈槽大小（`STACK_SLOT_SIZE = 8`）、内联局部类型容量（`INLINE_LOCAL_KINDS = 16`）、溢出目标、分支局部处理（`repr(u8)`）、跳转方向、字节序。
- `liftoff_assembler_defs.rs`: x64 与 arm64 目标平台下的通用与浮点缓存寄存器表（GP_CACHE_REGS, FP_CACHE_REGS）及不变式编译期断言。
- `liftoff_bailout_reasons.rs`: Liftoff 编译回退原因枚举（`LiftoffBailoutReason` 0..=21），对齐 Chromium 监控直方图并提供字符串映射。
- `liftoff_register.rs`: 寄存器分类（`RegClass`）、半字配对、以及 `REG_CLASS_FOR` 类型到寄存器分类的映射表。
- `liftoff_varstate.rs`: 变量状态栈槽位置枚举（`VarStateLocation`: Stack, Register, IntConst）。
- `parallel_move.rs`: 寄存器并行搬移装载类别（`LoadKind`: Nop, Constant, Stack, LowHalfStack, HighHalfStack）。
- `tests/liftoff.rs`: 覆盖回退原因字符串、类型映射、寄存器表长度、状态机枚举与常量的不变式单测（5/5 passed）。

## 二、复审环日志

- **Round 1 (全新只读席, 目标 clean 1/3)**:
  - 顶层 `#![forbid(unsafe_code)]`，扁平导出架构无二次导出；
  - 1:1 比对 `LiftoffBailoutReason`（0..=21）、`REG_CLASS_FOR`、x64/arm64 缓存寄存器表及限制；
  - 状态枚举与并行装载类别完备；
  - 隔离目录 `target/agent_liftoff_rev1` 门禁实测 5/5 全绿，Clippy 0 警告；
  - 结论：无反馈意见 / 0 代码缺陷 / 测试全部覆盖通过（**达成 clean 1/3**）。
- **Round 2 (全新独立 Reviewer, 目标 clean 2/3)**:
  - 顶层安全契约核验：`src/lib.rs:1` 声明 `#![forbid(unsafe_code)]`，全库 100% Safe Rust；
  - 扁平导出架构与私有子模块，杜绝二次导出；
  - 1:1 比对 `LiftoffBailoutReason`（0..=21）及 `to_str()` 描述字符串；
  - `REG_CLASS_FOR` 11 种 ValueKind 映射精确对齐；
  - x64 与 arm64 缓存寄存器表、位宽编码常数及不变式断言完备；
  - 汇编常量、状态枚举及并行装载类别与原版逐一吻合；
  - 零 panic（零裸 unwrap / expect）、零 dead code、零 allow 绕过；
  - 隔离目录 `target/agent_liftoff_rev2` 门禁实测 5/5 全绿，Clippy 0 警告；
  - 结论：无反馈意见 / 0 代码缺陷 / 测试全部覆盖通过（**达成 clean 2/3**）。
- **Round 3 (全新独立终验 Reviewer, 目标 clean 3/3 终检验收冻结)**:
  - 顶层安全契约核验：`src/lib.rs:1` 声明 `#![forbid(unsafe_code)]`，全库 100% Safe Rust；
  - 扁平导出架构与私有子模块，杜绝二次导出；
  - 核心枚举与映射表 1:1 比对原版 C++：
    - `LiftoffBailoutReason`（0..=21）及 `to_str()` 与 `liftoff-bailout-reasons.h` 逐一吻合；
    - `REG_CLASS_FOR` 11 种 ValueKind 映射、`RegClass`、x64/arm64 寄存器限制常数与位宽编码完全一致；
    - 汇编布局常数（`STACK_SLOT_SIZE`, `INLINE_LOCAL_KINDS`, `NO_CACHED_MEM_INDEX`）、控制流与状态枚举（`SpillLocation`, `IgnoreLocals`, `JumpDirection`, `Endianness`, `SmiCheckMode`）完全对齐；
    - `VarStateLocation`、`LoadKind` 逐一吻合；
  - 零 panic（零裸 unwrap / expect）、零 dead code、零 allow 绕过；
  - 隔离目录 `target/agent_liftoff_rev3` 门禁实测 5/5 全绿，Clippy 0 警告；
  - 结论：无反馈意见 / 0 代码缺陷 / 测试全部覆盖通过（**达成 clean 3/3 终验冻结**）。
