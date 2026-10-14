//! v8/src/base/flags.h 的 1:1 转写 — 类型安全的枚举位标志集
//!
//! 原版 `Flags<EnumT, BitfieldT = int>` 类模板: 位域存储 + 按位运算符集。
//! Rust 无枚举位运算, 以 `Flags<E, T>` 泛型结构体 + 标准位运算符重载承接;
//! `DEFINE_OPERATORS_FOR_FLAGS` 的 enum|enum 语法在 Rust 调用方写
//! `Flags::new(E::A) | Flags::new(E::B)` 等价。
//! `hash_value(Flags)` 走 mask 的 usize 直投 (与原版一致)。

use core::{
  fmt,
  hash::{Hash, Hasher},
  marker::PhantomData,
  ops,
};

/// 位域存储类型 (原版 BitfieldT 约束: 整数, storage >= bitfield)
pub trait Bitfield: Copy + Default + PartialEq + Eq + fmt::Debug {
  const ZERO: Self;
  fn and(self, rhs: Self) -> Self;
  fn or(self, rhs: Self) -> Self;
  fn xor(self, rhs: Self) -> Self;
  fn not(self) -> Self;
  fn as_usize(self) -> usize;
  fn eq_mask(self, other: Self) -> bool;
}

macro_rules! impl_bitfield {
  ($($t:ty),*) => {$(
    impl Bitfield for $t {
      const ZERO: Self = 0;
      #[inline] fn and(self, rhs: Self) -> Self { self & rhs }
      #[inline] fn or(self, rhs: Self) -> Self { self | rhs }
      #[inline] fn xor(self, rhs: Self) -> Self { self ^ rhs }
      #[inline] fn not(self) -> Self { !self }
      #[inline] fn as_usize(self) -> usize { self as usize }
      #[inline] fn eq_mask(self, other: Self) -> bool { self == other }
    }
  )*};
}

impl_bitfield!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

/// 可作标志的枚举 (原版 flag_type: 可静态转换到 BitfieldT 的枚举/整数)
pub trait FlagEnum<T: Bitfield>: Copy {
  fn to_mask(self) -> T;
}

macro_rules! impl_flag_enum_same {
  ($($t:ty),*) => {$(
    impl FlagEnum<$t> for $t {
      #[inline] fn to_mask(self) -> $t { self }
    }
  )*};
}

impl_flag_enum_same!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

/// Flags<EnumT, BitfieldT>: OR 组合的枚举值集合
#[derive(Debug, Clone, Copy, Default)]
pub struct Flags<E, T = i32> {
  mask: T,
  _flag: PhantomData<fn() -> E>,
}

impl<E: FlagEnum<T>, T: Bitfield> Flags<E, T> {
  /// Flags(): 空集
  #[inline]
  pub const fn new() -> Self {
    Self {
      mask: T::ZERO,
      _flag: PhantomData,
    }
  }

  /// Flags(flag_type)
  #[inline]
  pub fn flag(flag: E) -> Self {
    Self {
      mask: flag.to_mask(),
      _flag: PhantomData,
    }
  }

  /// Flags(std::initializer_list<flag_type>)
  #[inline]
  pub fn from_flags(flags: impl IntoIterator<Item = E>) -> Self {
    let mut r = Self::new();
    for f in flags {
      r.mask = r.mask.or(f.to_mask());
    }
    r
  }

  /// Flags(mask_type) 显式掩码构造
  #[inline]
  pub const fn from_mask(mask: T) -> Self {
    Self {
      mask,
      _flag: PhantomData,
    }
  }

  /// set(flag, value): 置/清给定标志
  #[inline]
  pub fn set_flag(&mut self, flag: E, value: bool) -> &mut Self {
    self.set_flags(Self::flag(flag), value)
  }

  /// set(flags, value)
  #[inline]
  pub fn set_flags(&mut self, flags: Self, value: bool) -> &mut Self {
    if value {
      *self |= flags;
    } else {
      *self &= !flags;
    }
    self
  }

  /// contains: 全含给定集合
  #[inline]
  #[must_use]
  pub fn contains(&self, flags: Self) -> bool {
    self.mask.and(flags.mask).eq_mask(flags.mask)
  }

  /// contains 单标志形态
  #[inline]
  #[must_use]
  pub fn contains_flag(&self, flag: E) -> bool {
    self.contains(Self::flag(flag))
  }

  /// with: 追加集合的新集
  #[inline]
  #[must_use]
  pub fn with(&self, flags: Self) -> Self {
    *self | flags
  }

  /// without: 剔除集合的新集
  #[inline]
  #[must_use]
  pub fn without(&self, flags: Self) -> Self {
    *self & !flags
  }

  /// 与裸掩码相等比较 (原版 operator==(flag_type) 的同宽形态)
  #[inline]
  #[must_use]
  pub fn mask_eq(&self, other: &Self) -> bool {
    self.mask.eq_mask(other.mask)
  }

  /// 当前掩码 (原版 operator mask_type())
  #[inline]
  pub const fn mask(&self) -> T {
    self.mask
  }

  /// 原版 operator!(): 空集判定
  #[inline]
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.mask.eq_mask(T::ZERO)
  }
}

impl<T: Bitfield, E: FlagEnum<T>> PartialEq for Flags<E, T> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.mask.eq_mask(other.mask)
  }
}
impl<T: Bitfield, E: FlagEnum<T>> Eq for Flags<E, T> {}

impl<T: Bitfield, E: FlagEnum<T>> ops::BitOr for Flags<E, T> {
  type Output = Self;
  #[inline]
  fn bitor(self, rhs: Self) -> Self {
    Self::from_mask(self.mask.or(rhs.mask))
  }
}

impl<T: Bitfield, E: FlagEnum<T>> ops::BitAnd for Flags<E, T> {
  type Output = Self;
  #[inline]
  fn bitand(self, rhs: Self) -> Self {
    Self::from_mask(self.mask.and(rhs.mask))
  }
}

impl<T: Bitfield, E: FlagEnum<T>> ops::BitXor for Flags<E, T> {
  type Output = Self;
  #[inline]
  fn bitxor(self, rhs: Self) -> Self {
    Self::from_mask(self.mask.xor(rhs.mask))
  }
}

impl<T: Bitfield, E: FlagEnum<T>> ops::Not for Flags<E, T> {
  type Output = Self;
  #[inline]
  fn not(self) -> Self {
    Self::from_mask(self.mask.not())
  }
}

impl<T: Bitfield, E: FlagEnum<T>> ops::BitOrAssign for Flags<E, T> {
  #[inline]
  fn bitor_assign(&mut self, rhs: Self) {
    self.mask = self.mask.or(rhs.mask);
  }
}

impl<T: Bitfield, E: FlagEnum<T>> ops::BitAndAssign for Flags<E, T> {
  #[inline]
  fn bitand_assign(&mut self, rhs: Self) {
    self.mask = self.mask.and(rhs.mask);
  }
}

impl<T: Bitfield, E: FlagEnum<T>> ops::BitXorAssign for Flags<E, T> {
  #[inline]
  fn bitxor_assign(&mut self, rhs: Self) {
    self.mask = self.mask.xor(rhs.mask);
  }
}

/// 原版 friend hash_value(Flags): mask 直投 usize
#[inline]
#[must_use]
pub fn hash_value<E, T: Bitfield>(flags: Flags<E, T>) -> usize {
  flags.mask.as_usize()
}

/// 让 `Hasher` 场景可用 (hash_value 走本模块自由函数, 此处仅供 std hash 桥接)
impl<E, T: Bitfield> Hash for Flags<E, T> {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    state.write_usize(self.mask.as_usize());
  }
}
