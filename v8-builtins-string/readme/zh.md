# v8-builtins-string

V8 字符串（String）内建调度与元数据实现。

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