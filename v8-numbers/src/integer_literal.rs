//! 原版 v8/src/numbers/integer-literal.h + integer-literal-inl.h
//! IntegerLiteral — 64 位带符号绝对值表示的整数字面量 (诊断/错误信息用)

use std::{
  fmt,
  ops::{Add, BitOr, Shl},
};

use itoa::Buffer;
/// 整数字面量 — (负号, 绝对值); 零恒非负
#[derive(Clone, Copy, Debug)]
pub struct IntegerLiteral {
  negative: bool,
  absolute_value: u64,
}

impl IntegerLiteral {
  /// 从 (符号, 绝对值) 构造; 0 归一为非负
  #[must_use]
  pub const fn from_sign_abs(negative: bool, absolute_value: u64) -> Self {
    Self {
      negative: if absolute_value == 0 { false } else { negative },
      absolute_value,
    }
  }

  /// is_negative
  #[must_use]
  pub const fn is_negative(&self) -> bool {
    self.negative
  }

  /// absolute_value
  #[must_use]
  pub const fn absolute_value(&self) -> u64 {
    self.absolute_value
  }

  /// IsRepresentableAs::<T>() — 值域内可精确转换
  #[must_use]
  pub fn is_representable_as<T: IntBound>(&self) -> bool {
    let min = IntegerLiteral::from_sign_abs(true, T::MIN_ABS);
    let max = IntegerLiteral::from_sign_abs(false, T::MAX_ABS);
    self >= &min && self <= &max
  }

  /// To::<T>() — 前置 is_representable_as (debug 断言), 二进制直转
  #[must_use]
  pub fn to<T: IntBound>(&self) -> T {
    debug_assert!(self.is_representable_as::<T>());
    let v = if self.negative {
      self.absolute_value.wrapping_neg()
    } else {
      self.absolute_value
    };
    T::from_u64(v)
  }

  /// TryTo::<T>() — 值域检查 + 转换
  #[must_use]
  pub fn try_to<T: IntBound>(&self) -> Option<T> {
    if self.is_representable_as::<T>() {
      Some(self.to::<T>())
    } else {
      None
    }
  }

  /// Compare — 三态: 小/等/大
  #[must_use]
  pub fn compare(&self, other: &Self) -> Ordering {
    use Ordering::{Equal, Greater, Less};
    if self.absolute_value == other.absolute_value {
      if self.absolute_value == 0 || self.negative == other.negative {
        return Equal;
      }
      return if self.negative { Less } else { Greater };
    }
    if self.absolute_value < other.absolute_value {
      return if other.negative { Greater } else { Less };
    }
    if self.negative { Less } else { Greater }
  }

  /// ToString — 十进制 (|v| 补 '-')
  #[must_use]
  pub fn to_string_value(&self) -> String {
    self.to_string()
  }

  /// 按位或 (两侧非负, debug 断言)
  #[must_use]
  pub fn bit_or(&self, y: &Self) -> Self {
    debug_assert!(!self.negative && !y.negative);
    Self::from_sign_abs(false, self.absolute_value | y.absolute_value)
  }

  /// 左移 (y ∈ [0, 64); 违规移位量夹取到 63, 原版为 DCHECK + UB)
  #[must_use]
  pub fn shift_left(&self, y: &Self) -> Self {
    debug_assert!(!y.negative);
    debug_assert!(y.absolute_value < 64);
    Self::from_sign_abs(
      self.negative,
      self
        .absolute_value
        .wrapping_shl(y.absolute_value.min(63) as u32),
    )
  }

  /// 加法 (无符号加减收敛; 溢出 debug 断言)
  #[must_use]
  pub fn add(&self, y: &Self) -> Self {
    if self.negative == y.negative {
      debug_assert!(self.absolute_value <= u64::MAX - y.absolute_value);
      return Self::from_sign_abs(
        self.negative,
        self.absolute_value.wrapping_add(y.absolute_value),
      );
    }
    if self.absolute_value >= y.absolute_value {
      return Self::from_sign_abs(self.negative, self.absolute_value - y.absolute_value);
    }
    Self::from_sign_abs(!self.negative, y.absolute_value - self.absolute_value)
  }
}

/// 三态比较结果
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ordering {
  Less,
  Equal,
  Greater,
}

use std::cmp::Ordering as StdOrdering;

use Ordering::{Equal, Greater, Less};

impl From<Ordering> for StdOrdering {
  fn from(o: Ordering) -> Self {
    match o {
      Less => StdOrdering::Less,
      Equal => StdOrdering::Equal,
      Greater => StdOrdering::Greater,
    }
  }
}

impl PartialEq for IntegerLiteral {
  fn eq(&self, other: &Self) -> bool {
    self.compare(other) == Equal
  }
}

impl Eq for IntegerLiteral {}

impl PartialOrd for IntegerLiteral {
  fn partial_cmp(&self, other: &Self) -> Option<StdOrdering> {
    Some(self.cmp(other))
  }
}

impl Ord for IntegerLiteral {
  fn cmp(&self, other: &Self) -> StdOrdering {
    self.compare(other).into()
  }
}

impl From<i32> for IntegerLiteral {
  fn from(v: i32) -> Self {
    Self::from_sign_abs(v < 0, v.unsigned_abs() as u64)
  }
}

impl From<i64> for IntegerLiteral {
  fn from(v: i64) -> Self {
    Self::from_sign_abs(v < 0, v.unsigned_abs())
  }
}

impl From<u64> for IntegerLiteral {
  fn from(v: u64) -> Self {
    Self::from_sign_abs(false, v)
  }
}

/// IntBound — 值域边界 (IsRepresentableAs/To 的类型参数)
pub trait IntBound: Copy {
  /// 类型最小值的绝对值 (u64 位型)
  const MIN_ABS: u64;
  /// 类型最大值的绝对值
  const MAX_ABS: u64;
  /// u64 位型直转
  fn from_u64(v: u64) -> Self;
}

impl IntBound for i32 {
  const MIN_ABS: u64 = 1u64 << 31;
  const MAX_ABS: u64 = (1u64 << 31) - 1;
  fn from_u64(v: u64) -> Self {
    v as i32
  }
}

impl IntBound for i64 {
  const MIN_ABS: u64 = 1u64 << 63;
  const MAX_ABS: u64 = (1u64 << 63) - 1;
  fn from_u64(v: u64) -> Self {
    v as i64
  }
}

impl IntBound for u32 {
  const MIN_ABS: u64 = 0;
  const MAX_ABS: u64 = u32::MAX as u64;
  fn from_u64(v: u64) -> Self {
    v as u32
  }
}

impl IntBound for u64 {
  const MIN_ABS: u64 = 0;
  const MAX_ABS: u64 = u64::MAX;
  fn from_u64(v: u64) -> Self {
    v
  }
}

impl fmt::Display for IntegerLiteral {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.negative {
      f.write_str("-")?;
    }
    let mut buf = Buffer::new();
    f.write_str(buf.format(self.absolute_value))
  }
}

impl BitOr for IntegerLiteral {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self {
    self.bit_or(&rhs)
  }
}

impl Shl for IntegerLiteral {
  type Output = Self;
  fn shl(self, rhs: Self) -> Self {
    self.shift_left(&rhs)
  }
}

impl Add for IntegerLiteral {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    IntegerLiteral::add(&self, &rhs)
  }
}

impl From<IntegerLiteral> for String {
  fn from(v: IntegerLiteral) -> Self {
    v.to_string_value()
  }
}
