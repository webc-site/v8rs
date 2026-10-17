// 由 tool/src/objects/heap-number.h.js 从 v8/src/objects/heap-number.{h,tq} 生成; 修订就地改, 复跑覆盖
// HeapNumber/UninitializedHeapNumber 布局表
//! HeapNumber 布局 (生成基线; 语义实现归 crate 已有 heap_number 模块)。

/// float64 载荷大小 (原版 value 字段)。
pub const VALUE_SIZE: usize = 8;

/// HeapNumber Torque 布局字段表 (heap-number.tq, 顺序 1:1)。
pub const HEAP_NUMBER_FIELDS: [(&str, &str); 1] = [
  ("value", "float64"),
];

/// UninitializedHeapNumber 与 HeapNumber 布局等宽 (原版 kSize = sizeof(HeapNumber);
/// 用于可变 double 字段的未初始化占位, 便于就地转换)。
pub const UNINITIALIZED_HEAP_NUMBER_FIELDS: [(&str, &str); 1] = [
  ("value", "float64"),
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(VALUE_SIZE == 8);
  assert!(HEAP_NUMBER_FIELDS.len() == 1);
  assert!(UNINITIALIZED_HEAP_NUMBER_FIELDS.len() == 1);
};
