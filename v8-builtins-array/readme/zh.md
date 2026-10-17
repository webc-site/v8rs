# v8-builtins-array

V8 数组（Array）内建与可扩展定长数组（GrowableFixedArray）实现。

## 模块架构

1. **ArrayBuiltin 内建表 (`array.rs`)**：
   - 包含 33 种数组核心内建枚举（`ArrayConcat`, `ArrayConstructor`, `ArrayPop`, `ArrayPrototypePush` 等）；
   - 支持 `strum` 双向序列化；
   - 语义分类谓词：
     - `is_prototype_method`: Array.prototype 原型方法判定；
     - `is_search_method`: IndexOf / Includes 族查找方法判定；
     - `is_fast_clone_method`: CloneFastJSArray / ExtractFastJSArray 快速数组克隆判定；
     - `is_constructor`: 构造函数判定；
   - 与 `v8-builtins-core` 全局调度表双向关联（`global_id`, `descriptor_name`）；
   - 包含 67 项来自原版 `array*.tq` 的宏与类型声明元数据（`ARRAY_TQ_DECLS`）。
2. **GrowableFixedArray (`growable.rs`)**：
   - 100% Safe Rust 复刻原版 V8 `GrowableFixedArray`（`growable-fixed-array-gen.h/cc`）；
   - 精确实现原版扩容公式：`new_capacity = current_capacity + (current_capacity >> 1) + 16`；
   - 安全的 `reserve`、`push`、`shrink_to_fit`、越界与溢出安全防护（`MAX_FIXED_ARRAY_CAPACITY = 512M`）。
3. **安全与质量保障**：
   - 顶层 `#![forbid(unsafe_code)]`；
   - 零 panic，零裸 unwrap / expect，零 dead code。