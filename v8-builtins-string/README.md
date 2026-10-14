[English](#en) | [中文](#zh)

---

<a name="en"></a>

# v8-builtins-string

V8 String builtins dispatch and metadata implementation.

- [Architecture](#architecture)

## Architecture

1. **StringBuiltin Dispatch Table (`string.rs`)**:
   - Contains 26 core string builtin variants (`StringAddNoMapCheck`, `StringCompare`, `StringFromCharCode`, `StringPrototypeReplace`, etc.);
   - Strum two-way serialization support;
   - Semantic classification predicates:
     - `is_prototype_method`: String.prototype method classification;
     - `is_comparison`: Comparison builtins (including WASM);
     - `is_wasm_string`: WASM dedicated string fast path builtins;
     - `is_case_conversion`: Case conversion method builtins;
   - Bidirectional linkage to `v8-builtins-core` global dispatch table (`global_id`, `descriptor_name`).
2. **Safety & Quality Guarantees**:
   - `#![forbid(unsafe_code)]`;
   - Zero panic, zero bare unwrap/expect, zero dead code.

---

<a name="zh"></a>

# v8-builtins-string

V8 字符串（String）内建调度与元数据实现。

- [模块架构](#模块架构)

## 模块架构

1. **StringBuiltin 内建表 (`string.rs`)**：
   - 包含 26 种字符串核心内建枚举（`StringAddNoMapCheck`, `StringCompare`, `StringFromCharCode`, `StringPrototypeReplace` 等）；
   - 支持 `strum` 双向序列化；
   - 语义分类谓词：
     - `is_prototype_method`: String.prototype 原型方法判定；
     - `is_comparison`: 字符串比较方法判定（含 WASM 比较）；
     - `is_wasm_string`: WASM 专用字符串快速路径判定；
     - `is_case_conversion`: 大小写转换方法族判定；
   - 与 `v8-builtins-core` 全局调度表双向关联（`global_id`, `descriptor_name`）；
2. **安全与质量保障**：
   - 顶层 `#![forbid(unsafe_code)]`；
   - 零 panic，零裸 unwrap / expect，零 dead code。
