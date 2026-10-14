use v8_common::K_HOLE_NAN_INT64;

use crate::{heap_object::HeapObject, map::Map, tagged::Tagged};

/// 堆分配的双精度浮点数对象 (HeapNumber)
///
/// 用于表示超出 Smi 范围的数字或带小数点的浮点数。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HeapNumber {
  /// 继承自 HeapObject
  pub header: HeapObject,
  /// 存储的 64 位双精度浮点数值
  pub value: f64,
}

impl HeapNumber {
  /// Hole NaN 专属 64 位位模式常量 (原版 kHoleNanInt64, globals.h 承载)
  pub const HOLE_NAN_INT64: u64 = K_HOLE_NAN_INT64;

  // IEEE 754 字段位掩码常量 (heap-number.h:44-54)
  pub const SIGN_MASK: u32 = 0x8000_0000;
  pub const EXPONENT_MASK: u32 = 0x7FF0_0000;
  pub const MANTISSA_MASK: u32 = 0x000F_FFFF;
  pub const MANTISSA_BITS: usize = 52;
  pub const EXPONENT_BITS: usize = 11;
  pub const EXPONENT_BIAS: usize = 1023;
  /// 高 32 位中指数域的右移位数。
  pub const EXPONENT_SHIFT: usize = 20;
  /// 指数全 1 (无穷或 NaN) 的无偏指数值 (0x7FF - 1023)。
  pub const INFINITY_OR_NAN_EXPONENT: usize = (0x7FF0_0000 >> 20) - 1023;
  /// 高 32 位中的尾数位数。
  pub const MANTISSA_BITS_IN_TOP_WORD: usize = 20;
  /// 高 32 位中的非尾数位数 (符号 1 + 指数 11)。
  pub const NON_MANTISSA_BITS_IN_TOP_WORD: usize = 12;

  /// 以给定的 Map 和浮点数构造 HeapNumber
  #[inline]
  pub const fn new(map: Tagged<Map>, value: f64) -> Self {
    Self {
      header: HeapObject::new(map),
      value,
    }
  }

  /// 以 64 位整数比特位模式构造 HeapNumber
  #[inline]
  pub const fn from_bits(map: Tagged<Map>, bits: u64) -> Self {
    Self {
      header: HeapObject::new(map),
      value: f64::from_bits(bits),
    }
  }

  /// 读取浮点数值
  #[inline]
  pub const fn value(&self) -> f64 {
    self.value
  }

  /// 更新浮点数值
  #[inline]
  pub fn set_value(&mut self, value: f64) {
    self.value = value;
  }

  /// 读取浮点数的原始 64 位比特表示
  #[inline]
  pub const fn value_as_bits(&self) -> u64 {
    self.value.to_bits()
  }

  /// 设置浮点数的原始 64 位比特表示
  #[inline]
  pub fn set_value_as_bits(&mut self, bits: u64) {
    self.value = f64::from_bits(bits);
  }

  /// 判断当前 HeapNumber 是否为 Hole 特殊标记
  #[inline]
  pub const fn is_the_hole(&self) -> bool {
    self.value_as_bits() == Self::HOLE_NAN_INT64
  }

  /// 判断当前值是否为 NaN
  #[inline]
  pub fn is_nan(&self) -> bool {
    self.value.is_nan()
  }
}
