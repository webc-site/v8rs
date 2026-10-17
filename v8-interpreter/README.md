[English](#en) | [中文](#zh)

---

<a name="en"></a>

# v8-interpreter

Rust port of V8 Ignition bytecode interpreter core infrastructure.

- [Key Features](#key-features)

## Key Features

- **Complete Bytecode Table (217 bytecodes)**: Mechanically aligned with V8 `bytecodes.{h,cc}`, including ShortStar (Star0..Star15), Wide / ExtraWide prefixes, DebugBreak series, and full operand metadata.
- **Register System**: Virtual registers (`Register`) and register lists (`RegisterList`), strictly matching frame layouts and parameter mappings (receiver / a0..an).
- **Zero-Copy Stream Decoding**: `BytecodeArrayIterator` for high-performance bytecode streaming with multi-level operand scaling (Single / Double / Quadruple).
- **Literal and Control Flags**: `CreateArrayLiteralFlags`, `CreateObjectLiteralFlags`, `CreateClosureFlags`, `TestTypeOfFlags`, `StoreLookupSlotFlags`.
- **100% Safe Rust**: Zero raw pointers, zero panics/unwraps, zero dead code, zero warning suppressions.

---

<a name="zh"></a>

# v8-interpreter

Ignition 解释器核心基础设施的 Rust 1:1 端口。

- [核心特性](#核心特性)

## 核心特性

- **217 条 Bytecode 全表**：机械对齐 V8 原版 `bytecodes.{h,cc}`，含 ShortStar (Star0..Star15)、宽前缀 (Wide, ExtraWide)、DebugBreak 系列以及所有操作数元表定义。
- **寄存器系统**：虚拟寄存器（`Register`）、寄存器段（`RegisterList`），严格匹配解释器栈帧布局与参数映射（receiver / a0..an）。
- **零拷贝流式解码**：`BytecodeArrayIterator` 高效解析字节码流，支持多级操作数缩放（Single / Double / Quadruple）。
- **字面量与控制流旗标**：`CreateArrayLiteralFlags`, `CreateObjectLiteralFlags`, `CreateClosureFlags`, `TestTypeOfFlags`, `StoreLookupSlotFlags`。
- **100% Safe Rust**：无裸指针、零 unwrap/expect、零 panic、0 dead code、无 allow 警告绕过。
