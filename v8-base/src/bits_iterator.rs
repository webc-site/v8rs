//! v8/src/base/bits-iterator.h 的 1:1 转写 — 置位位位置的前向迭代器
//! 原版 BitsIterator<T, kMSBFirst> (forward_iterator, value_type=int):
//! - operator*  → CountLeadingZeros/CountTrailingZeros 定位
//! - operator++ → bits &= ~(T{1} << pos) (bits.rs::clear_bit)
//! - end() 为 BitsIterator{0} → Rust Iterator 以 bits==0 自然终止, 语义等价
//!
//! make_iterator_range 容器形态 → iterate_bits/iterate_bits_backwards 直接返回迭代器

use crate::bits::BitOpsUnsigned;

/// 置位位迭代器: MSB_FIRST=false 从低到高, true 从高到低
/// (原版模板参数 bool kMSBFirst → Rust const generic)
pub struct BitsIterator<T, const MSB_FIRST: bool> {
  bits: T,
}

impl<T: BitOpsUnsigned, const MSB_FIRST: bool> BitsIterator<T, MSB_FIRST> {
  /// explicit BitsIterator(T bits)
  #[inline]
  pub fn new(bits: T) -> Self {
    Self { bits }
  }

  /// int operator*() const — 当前置位位下标 (迭代结束时语义上不可调用,
  /// Rust Iterator 不暴露解引用, 由 next 内部使用)
  #[inline]
  fn deref_pos(bits: T) -> u32 {
    if MSB_FIRST {
      T::BITS - 1 - bits.count_leading_zeros()
    } else {
      bits.count_trailing_zeros()
    }
  }
}

impl<T: BitOpsUnsigned, const MSB_FIRST: bool> Iterator for BitsIterator<T, MSB_FIRST> {
  type Item = u32;

  #[inline]
  fn next(&mut self) -> Option<u32> {
    // end 哨兵 BitsIterator{0}: 无置位位即结束 (原版 operator== 语义)
    if self.bits.count_population() == 0 {
      return None;
    }
    let pos = Self::deref_pos(self.bits);
    // BitsIterator& operator++(): bits_ &= ~(T{1} << **this)
    self.bits = self.bits.clear_bit(pos);
    Some(pos)
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    let left = self.bits.count_population() as usize;
    (left, Some(left))
  }
}

impl<T: BitOpsUnsigned, const MSB_FIRST: bool> ExactSizeIterator for BitsIterator<T, MSB_FIRST> {}

/// Returns an iterable over the bits in {bits}, from LSB to MSB.
#[inline]
pub fn iterate_bits<T: BitOpsUnsigned>(bits: T) -> BitsIterator<T, false> {
  BitsIterator::new(bits)
}

/// Returns an iterable over the bits in {bits}, from MSB to LSB.
#[inline]
pub fn iterate_bits_backwards<T: BitOpsUnsigned>(bits: T) -> BitsIterator<T, true> {
  BitsIterator::new(bits)
}
