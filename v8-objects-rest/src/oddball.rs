// 由 tool/src/objects/oddball.h.js 从 v8/src/objects/oddball.{h,tq} 生成; 修订就地改, 复跑覆盖
// Oddball kind 常量 + Torque 布局字段表
//! Oddball: true/false/null/undefined/boolean 等单例基类。

/// kind 值: false (原版 kFalse)。
pub const K_FALSE: u8 = 0;
/// kind 值: true (原版 kTrue)。
pub const K_TRUE: u8 = 1;
/// 非布尔掩码 (原版 kNotBooleanMask = ~1 截断 u8)。
pub const K_NOT_BOOLEAN_MASK: u8 = 0xfe;
/// kind 值: null (原版 kNull; == Internals::kNullOddballKind)。
pub const K_NULL: u8 = 3;
/// kind 值: undefined (原版 kUndefined; == Internals::kUndefinedOddballKind)。
pub const K_UNDEFINED: u8 = 4;

/// 是否布尔 kind (低 1 位可分辨)。
#[inline]
#[must_use]
pub const fn is_boolean_kind(kind: u8) -> bool {
  (kind & K_NOT_BOOLEAN_MASK) != K_NOT_BOOLEAN_MASK
}

/// Oddball Torque 布局字段表 (oddball.tq `extern class Oddball extends PrimitiveHeapObject`, 顺序 1:1)。
pub const ODDBALL_FIELDS: [(&str, &str); 5] = [
  ("to_number_raw", "float64"),
  ("to_string", "String"),
  ("to_number", "Number"),
  ("type_of", "String"),
  ("kind", "Smi"),
];

/// 子类层级 (oddball.tq; 全部空体继承, 实例类型复用父段)。
/// Null/Undefined/Boolean/True/False → 各自 h 侧 V8_IT_REUSE_PARENT。
pub const SUBCLASS_HIERARCHY: [(&str, &str); 5] = [
  ("Null", "Oddball"),
  ("Undefined", "Oddball"),
  ("Boolean", "Oddball"),
  ("True", "Boolean"),
  ("False", "Boolean"),
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_FALSE == 0 && K_TRUE == 1);
  assert!(K_NULL == 3 && K_UNDEFINED == 4);
  assert!(K_NOT_BOOLEAN_MASK == 0xfe);
  assert!(ODDBALL_FIELDS.len() == 5);
  assert!(ODDBALL_FIELDS[0] == ("to_number_raw", "float64"));
  assert!(ODDBALL_FIELDS[4] == ("kind", "Smi"));
};
