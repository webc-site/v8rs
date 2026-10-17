//! v8/src/base/enum-set.h 的 1:1 转写
//! 位集形态的枚举集合: 存储为整型位图, 枚举的底层序号即位下标。
//! 登记差异:
//! - C++ `class T = int` → Rust 无符号存储 (默认 u32; 位补语义等价)。
//! - std::initializer_list<E> → 切片构造 from_elements。
//! - operator<=> = default → bits 上的 PartialEq/Eq/PartialOrd/Ord。
//! - ostream << element 输出枚举底层序号 → Display 以 Debug 呈现同一信息。
//! - operator|=&-(E) 及对应复赋值形态: 与 add/remove/add_set/remove_set 及
//!   集合级 |  & - 算子完全重合, 且 Rust coherence 无法按元素类型重载 →
//!   仅提供具名方法 (零 panic 原则下的显式收敛)。

use std::{
  cmp::Ordering,
  fmt::{self, Debug, Display, Formatter},
  marker::PhantomData,
  ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Sub, SubAssign},
};

use crate::bits::{BitOps, BitOpsUnsigned};

/// 枚举序号契约 (对应原版 static_cast<underlying_type_t<E>> 与 static_cast<E>)
pub trait EnumOrdinal: Copy + Eq + Debug {
  /// 底层整型序号 (原版 element 直用枚举值; 无显式赋值枚举即声明序)
  fn ordinal(self) -> usize;
  /// 序号 → 枚举 (原版 static_cast<E>; 越界序号返回 None, 禁 panic)
  fn from_ordinal(ordinal: usize) -> Option<Self>;
}

/// 原版 `EnumSet<E, T>`
pub struct EnumSet<E, T = u32> {
  bits: T,
  _marker: PhantomData<fn(E)>,
}

impl<E, T: Default> Default for EnumSet<E, T> {
  /// constexpr EnumSet() = default
  fn default() -> Self {
    Self {
      bits: T::default(),
      _marker: PhantomData,
    }
  }
}

impl<E, T: Copy> Clone for EnumSet<E, T> {
  fn clone(&self) -> Self {
    *self
  }
}
impl<E, T: Copy> Copy for EnumSet<E, T> {}

/// 位操作族 (要求 E 具备序号契约, T 具备位运算契约)
impl<E: EnumOrdinal, T> EnumSet<E, T>
where
  T: BitOps
    + BitOpsUnsigned
    + BitOr<Output = T>
    + BitAnd<Output = T>
    + Not<Output = T>
    + Default
    + Copy
    + Eq,
{
  /// constexpr EnumSet(std::initializer_list<E> init)
  pub fn from_elements(init: &[E]) -> Self {
    let mut bits = T::default();
    for e in init {
      bits = bits | Self::mask(*e);
    }
    Self {
      bits,
      _marker: PhantomData,
    }
  }

  /// static constexpr T Mask(E element) — DCHECK_GT(sizeof(T)*8, element)
  fn mask(element: E) -> T {
    let ord = element.ordinal();
    debug_assert!(ord < T::BITS as usize, "元素序号超出位宽 (原版 DCHECK_GT)");
    T::bit_at(ord as u32)
  }

  /// constexpr bool empty()
  #[inline]
  pub fn empty(&self) -> bool {
    self.bits == T::default()
  }

  /// constexpr bool contains(E)
  #[inline]
  pub fn contains(&self, element: E) -> bool {
    (self.bits & Self::mask(element)) != T::default()
  }

  /// constexpr bool contains_all(EnumSet)
  #[inline]
  pub fn contains_all(&self, set: Self) -> bool {
    (self.bits & set.bits) == set.bits
  }

  /// constexpr bool contains_any(EnumSet)
  #[inline]
  pub fn contains_any(&self, set: Self) -> bool {
    (self.bits & set.bits) != T::default()
  }

  /// constexpr bool contains_only(E)
  #[inline]
  pub fn contains_only(&self, element: E) -> bool {
    self.bits == Self::mask(element)
  }

  /// constexpr bool is_subset_of(EnumSet)
  #[inline]
  pub fn is_subset_of(&self, set: Self) -> bool {
    (self.bits & set.bits) == self.bits
  }

  /// constexpr void Add(E)
  #[inline]
  pub fn add(&mut self, element: E) {
    self.bits = self.bits | Self::mask(element);
  }

  /// constexpr void Add(EnumSet)
  #[inline]
  pub fn add_set(&mut self, set: Self) {
    self.bits = self.bits | set.bits;
  }

  /// constexpr void Remove(E)
  #[inline]
  pub fn remove(&mut self, element: E) {
    self.bits = self.bits & !Self::mask(element);
  }

  /// constexpr void Remove(EnumSet)
  #[inline]
  pub fn remove_set(&mut self, set: Self) {
    self.bits = self.bits & !set.bits;
  }

  /// constexpr void RemoveAll()
  #[inline]
  pub fn remove_all(&mut self) {
    self.bits = T::default();
  }

  /// constexpr void Intersect(EnumSet)
  #[inline]
  pub fn intersect(&mut self, set: Self) {
    self.bits = self.bits & set.bits;
  }

  /// constexpr T ToIntegral()
  #[inline]
  pub fn to_integral(&self) -> T {
    self.bits
  }

  /// static constexpr EnumSet FromIntegral(T bits)
  #[inline]
  pub fn from_integral(bits: T) -> Self {
    Self {
      bits,
      _marker: PhantomData,
    }
  }

  /// 取最低位 1 对应元素并清位 (原版 ostream 循环的 CountTrailingZerosNonZero)
  fn pop_lowest(&mut self) -> Option<E> {
    if self.empty() {
      return None;
    }
    let ord = self.bits.count_trailing_zeros_nonzero() as usize;
    let bits = self.bits;
    // 原版 static_cast<E>(zeros) 不校验; Rust 越界序号回传 None 且仍清位
    self.bits = bits & !T::bit_at(ord as u32);
    E::from_ordinal(ord)
  }

  /// 低位在前取出全部元素 (Display 与测试共用)
  pub fn elements(&self) -> Vec<E> {
    let mut rest = *self;
    let mut out = Vec::new();
    while let Some(e) = rest.pop_lowest() {
      out.push(e);
    }
    out
  }

  /// 置位个数 (CountPopulation 位图形态; 对应集合基数)
  #[inline]
  pub fn count(&self) -> u32 {
    self.bits.count_population()
  }
}

/// constexpr EnumSet operator~()
impl<E, T: Not<Output = T> + Copy> Not for EnumSet<E, T> {
  type Output = Self;
  fn not(self) -> Self {
    Self {
      bits: !self.bits,
      _marker: PhantomData,
    }
  }
}

/// constexpr EnumSet operator|(EnumSet)
impl<E, T: BitOr<Output = T> + Copy> BitOr for EnumSet<E, T> {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self {
    Self {
      bits: self.bits | rhs.bits,
      _marker: PhantomData,
    }
  }
}

/// constexpr EnumSet operator&(EnumSet)
impl<E, T: BitAnd<Output = T> + Copy> BitAnd for EnumSet<E, T> {
  type Output = Self;
  fn bitand(self, rhs: Self) -> Self {
    Self {
      bits: self.bits & rhs.bits,
      _marker: PhantomData,
    }
  }
}

/// constexpr EnumSet operator-(EnumSet) — bits & ~set.bits
impl<E, T: BitAnd<Output = T> + Not<Output = T> + Copy> Sub for EnumSet<E, T> {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self {
    Self {
      bits: self.bits & !rhs.bits,
      _marker: PhantomData,
    }
  }
}

/// EnumSet& operator|=(EnumSet)
impl<E, T: BitOr<Output = T> + Copy> BitOrAssign for EnumSet<E, T> {
  fn bitor_assign(&mut self, rhs: Self) {
    self.bits = self.bits | rhs.bits;
  }
}

/// EnumSet& operator&=(EnumSet)
impl<E, T: BitAnd<Output = T> + Copy> BitAndAssign for EnumSet<E, T> {
  fn bitand_assign(&mut self, rhs: Self) {
    self.bits = self.bits & rhs.bits;
  }
}

/// EnumSet& operator-=(EnumSet)
impl<E, T: BitAnd<Output = T> + Not<Output = T> + Copy> SubAssign for EnumSet<E, T> {
  fn sub_assign(&mut self, rhs: Self) {
    self.bits = self.bits & !rhs.bits;
  }
}

/// constexpr auto operator<=>(const EnumSet&) = default — bits 字典序
impl<E, T: PartialEq> PartialEq for EnumSet<E, T> {
  fn eq(&self, other: &Self) -> bool {
    self.bits == other.bits
  }
}
impl<E, T: Eq> Eq for EnumSet<E, T> {}
impl<E, T: PartialOrd> PartialOrd for EnumSet<E, T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.bits.partial_cmp(&other.bits)
  }
}
impl<E, T: Ord> Ord for EnumSet<E, T> {
  fn cmp(&self, other: &Self) -> Ordering {
    self.bits.cmp(&other.bits)
  }
}

/// 原版 std::ostream& operator<<(ostream&, EnumSet): "{e0, e1, ...}" 低位在前
impl<E: EnumOrdinal, T> Display for EnumSet<E, T>
where
  T: BitOps
    + BitOpsUnsigned
    + BitOr<Output = T>
    + BitAnd<Output = T>
    + Not<Output = T>
    + Default
    + Copy
    + Eq,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str("{")?;
    for (i, e) in self.elements().iter().enumerate() {
      if i > 0 {
        f.write_str(", ")?;
      }
      // 原版 ostream << 枚举呈现底层序号; Debug 输出同一信息 (登记差异)
      write!(f, "{:?}", e)?;
    }
    f.write_str("}")
  }
}
