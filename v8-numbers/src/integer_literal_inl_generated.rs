// 由 tool/src/numbers/integer-literal-inl.h.js 从 v8/src/numbers/integer-literal-inl.h 生成; 修订就地改, 复跑覆盖
// IntegerLiteral 运算骨架与真实实现

use std::{
  cmp::Ordering,
  fmt,
  ops::{Add, BitOr, Shl},
};

use itoa::Buffer;

/// 整数字面量结构体 (字段与 V8 1:1, 零恒非负)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegerLiteralSkel {
  pub negative: bool,
  pub absolute_value: u64,
}

impl IntegerLiteralSkel {
  /// 规范构造: 绝对值为 0 时负号归一为 false
  #[inline]
  #[must_use]
  pub const fn new(negative: bool, absolute_value: u64) -> Self {
    Self {
      negative: if absolute_value == 0 { false } else { negative },
      absolute_value,
    }
  }

  #[inline]
  #[must_use]
  pub const fn is_negative(&self) -> bool {
    self.negative
  }

  #[inline]
  #[must_use]
  pub const fn absolute_value(&self) -> u64 {
    self.absolute_value
  }

  /// 整数三态比较 (原版 Compare)
  #[inline]
  #[must_use]
  pub const fn compare(&self, other: &Self) -> Ordering {
    use std::cmp::Ordering::{Equal, Greater, Less};
    if self.absolute_value == other.absolute_value {
      if self.absolute_value == 0 || self.negative == other.negative {
        Equal
      } else if self.negative {
        Less
      } else {
        Greater
      }
    } else if self.absolute_value < other.absolute_value {
      if other.negative { Greater } else { Less }
    } else if self.negative {
      Less
    } else {
      Greater
    }
  }

  /// 是否可安全表示为 i32 (直接常量比较, 避免两次 compare 对象构建)
  #[inline]
  #[must_use]
  pub const fn is_representable_as_i32(&self) -> bool {
    if self.negative {
      self.absolute_value <= (1u64 << 31)
    } else {
      self.absolute_value <= (i32::MAX as u64)
    }
  }

  /// 是否可安全表示为 u32
  #[inline]
  #[must_use]
  pub const fn is_representable_as_u32(&self) -> bool {
    !self.negative && self.absolute_value <= u32::MAX as u64
  }

  /// 是否可安全表示为 i64 (直接常量比较, 避免两次 compare 对象构建)
  #[inline]
  #[must_use]
  pub const fn is_representable_as_i64(&self) -> bool {
    if self.negative {
      self.absolute_value <= (1u64 << 63)
    } else {
      self.absolute_value <= (i64::MAX as u64)
    }
  }

  /// 是否可安全表示为 u64
  #[inline]
  #[must_use]
  pub const fn is_representable_as_u64(&self) -> bool {
    !self.negative
  }

  /// 尝试转换为 i32
  #[inline]
  #[must_use]
  pub const fn try_to_i32(&self) -> Option<i32> {
    if self.is_representable_as_i32() {
      let v = if self.negative {
        self.absolute_value.wrapping_neg() as i32
      } else {
        self.absolute_value as i32
      };
      Some(v)
    } else {
      None
    }
  }

  /// 尝试转换为 u32
  #[inline]
  #[must_use]
  pub const fn try_to_u32(&self) -> Option<u32> {
    if self.is_representable_as_u32() {
      Some(self.absolute_value as u32)
    } else {
      None
    }
  }

  /// 尝试转换为 i64
  #[inline]
  #[must_use]
  pub const fn try_to_i64(&self) -> Option<i64> {
    if self.is_representable_as_i64() {
      let v = if self.negative {
        self.absolute_value.wrapping_neg() as i64
      } else {
        self.absolute_value as i64
      };
      Some(v)
    } else {
      None
    }
  }

  /// 尝试转换为 u64
  #[inline]
  #[must_use]
  pub const fn try_to_u64(&self) -> Option<u64> {
    if self.is_representable_as_u64() {
      Some(self.absolute_value)
    } else {
      None
    }
  }
}

impl PartialOrd for IntegerLiteralSkel {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for IntegerLiteralSkel {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.compare(other)
  }
}

impl From<i32> for IntegerLiteralSkel {
  #[inline]
  fn from(v: i32) -> Self {
    Self::new(v < 0, v.unsigned_abs() as u64)
  }
}

impl From<i64> for IntegerLiteralSkel {
  #[inline]
  fn from(v: i64) -> Self {
    Self::new(v < 0, v.unsigned_abs())
  }
}

impl From<u64> for IntegerLiteralSkel {
  #[inline]
  fn from(v: u64) -> Self {
    Self::new(false, v)
  }
}

impl fmt::Display for IntegerLiteralSkel {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.negative && self.absolute_value != 0 {
      f.write_str("-")?;
    }
    let mut buf = Buffer::new();
    f.write_str(buf.format(self.absolute_value))
  }
}

impl Add for IntegerLiteralSkel {
  type Output = Self;
  #[inline]
  fn add(self, rhs: Self) -> Self {
    integer_literal_inl_skel::add(&self, &rhs)
  }
}

impl Shl for IntegerLiteralSkel {
  type Output = Self;
  #[inline]
  fn shl(self, rhs: Self) -> Self {
    integer_literal_inl_skel::shl(&self, &rhs)
  }
}

impl BitOr for IntegerLiteralSkel {
  type Output = Self;
  #[inline]
  fn bitor(self, rhs: Self) -> Self {
    integer_literal_inl_skel::bit_or(&self, &rhs)
  }
}

/// IntegerLiteral 内联运算实现 (原版 integer-literal-inl.h)。
pub mod integer_literal_inl_skel {
  use itoa::Buffer;

  use super::IntegerLiteralSkel;

  /// 十进制串, 负值带 '-' (原版 ToString)
  #[inline]
  #[must_use]
  pub fn to_string(x: &IntegerLiteralSkel) -> String {
    let mut buf = Buffer::new();
    let formatted = buf.format(x.absolute_value);
    if x.negative && x.absolute_value != 0 {
      let mut s = String::with_capacity(1 + formatted.len());
      s.push('-');
      s.push_str(formatted);
      s
    } else {
      formatted.to_string()
    }
  }

  /// 左移: y 须非负且 < 64 (原版 operator<<, DCHECK 两个前置)
  #[inline]
  #[must_use]
  pub const fn shl(x: &IntegerLiteralSkel, y: &IntegerLiteralSkel) -> IntegerLiteralSkel {
    debug_assert!(!y.negative, "shift amount must not be negative");
    debug_assert!(y.absolute_value < 64, "shift amount must be < 64");
    let shift = if y.absolute_value < 64 {
      y.absolute_value as u32
    } else {
      63
    };
    let shifted = x.absolute_value.wrapping_shl(shift);
    IntegerLiteralSkel::new(x.negative, shifted)
  }

  /// 加法: 同号相加 / 异号大减小取大者号 (原版 operator+)
  #[inline]
  #[must_use]
  pub const fn add(x: &IntegerLiteralSkel, y: &IntegerLiteralSkel) -> IntegerLiteralSkel {
    if x.negative == y.negative {
      debug_assert!(
        x.absolute_value <= u64::MAX - y.absolute_value,
        "overflow on same sign addition"
      );
      IntegerLiteralSkel::new(x.negative, x.absolute_value.wrapping_add(y.absolute_value))
    } else if x.absolute_value >= y.absolute_value {
      IntegerLiteralSkel::new(x.negative, x.absolute_value - y.absolute_value)
    } else {
      IntegerLiteralSkel::new(!x.negative, y.absolute_value - x.absolute_value)
    }
  }

  /// 相等: 逐字段判定 (原版 operator==, integer-literal.h)
  #[inline]
  #[must_use]
  pub const fn eq(x: &IntegerLiteralSkel, y: &IntegerLiteralSkel) -> bool {
    x.negative == y.negative && x.absolute_value == y.absolute_value
  }

  /// 按位或: 两侧均须非负 (原版 operator|, integer-literal.h)
  #[inline]
  #[must_use]
  pub const fn bit_or(x: &IntegerLiteralSkel, y: &IntegerLiteralSkel) -> IntegerLiteralSkel {
    debug_assert!(
      !x.negative && !y.negative,
      "bit_or operands must be non-negative"
    );
    IntegerLiteralSkel::new(false, x.absolute_value | y.absolute_value)
  }
}
