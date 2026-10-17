// 由 tool/src/objects/heap-number-inl.h.js 从 v8/src/objects/heap-number-inl.h 生成; 修订就地改, 复跑覆盖
// 席位表差集: 构造/访问/hole 判定 (.h/.tq 布局归既有 heap-number.h.js)
//! HeapNumber -inl 差集: 双精度盒的构造与位级访问。
//!
//! 布局 (value: float64) 归既有 heap-number.h.js; 本文件登记构造器
//! 双形态、位访问与 hole 判定 (位模式比较, NaN 语义不可用)。

use v8_objects_core::heap_number::HeapNumber;

/// hole NaN 的位模式 (原版 kHoleNanInt64 = 0xFFF7FFFF_FFF7FFFF, 非 MIPS; 不可用 NaN 语义比较)。
pub const K_HOLE_NAN_INT64: u64 = 0xFFF7FFFFFFF7FFFF;

// ── 访问器 (heap-number-inl.h 1:1; 载体 = v8-objects-core HeapNumber 布局) ──
/// 原版 value() / set_value() (double 语义读写)。
///
/// v8: src/objects/heap-number-inl.h HeapNumber::value / HeapNumber::set_value
#[inline]
#[must_use]
pub fn value(heap_number: &HeapNumber) -> f64 {
  heap_number.value()
}

/// v8: src/objects/heap-number-inl.h HeapNumber::set_value
#[inline]
pub fn set_value(heap_number: &mut HeapNumber, value: f64) {
  heap_number.set_value(value);
}

/// 原版 value_as_bits() / set_value_as_bits() (位模式读写, 免 NaN 比较陷阱)。
///
/// v8: src/objects/heap-number-inl.h HeapNumber::value_as_bits / set_value_as_bits
#[inline]
#[must_use]
pub fn value_as_bits(heap_number: &HeapNumber) -> u64 {
  heap_number.value_as_bits()
}

/// v8: src/objects/heap-number-inl.h HeapNumber::set_value_as_bits
#[inline]
pub fn set_value_as_bits(heap_number: &mut HeapNumber, bits: u64) {
  heap_number.set_value_as_bits(bits);
}

/// 原版 is_the_hole() — hole 判定 = 位模式等于 K_HOLE_NAN_INT64。
#[inline]
#[must_use]
pub const fn is_the_hole(bits: u64) -> bool {
  bits == K_HOLE_NAN_INT64
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(is_the_hole(K_HOLE_NAN_INT64));
  assert!(!is_the_hole(0));
  assert!(!is_the_hole(f64::INFINITY.to_bits()));
};
