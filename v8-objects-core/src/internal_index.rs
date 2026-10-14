// 由 tool/src/objects/internal-index.h.js 从 v8/src/objects/internal-index.h 生成; 修订就地改, 复跑覆盖
// InternalIndex: 字典条目索引包装

use core::ops::Range;

/// 字典条目索引包装 (原版 class InternalIndex)
/// entry 与 index 语义不同: entry 指向存储槽, index 为逻辑键序
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct InternalIndex(usize);

impl InternalIndex {
  /// 原版 kNotFound = size_t::MAX
  pub const NOT_FOUND: usize = usize::MAX;

  /// 原版 explicit constexpr InternalIndex(size_t)
  #[inline]
  #[must_use]
  pub const fn new(raw: usize) -> Self {
    Self(raw)
  }

  /// 原版 InternalIndex::NotFound()
  #[inline]
  #[must_use]
  pub const fn not_found() -> Self {
    Self(Self::NOT_FOUND)
  }

  /// 原版 adjust_down (DCHECK 由调用方保证)
  #[inline]
  #[must_use]
  pub const fn adjust_down(self, subtract: usize) -> Self {
    Self(self.0 - subtract)
  }

  /// 原版 adjust_up
  #[inline]
  #[must_use]
  pub const fn adjust_up(self, add: usize) -> Self {
    Self(self.0 + add)
  }

  /// 原版 is_found
  #[inline]
  #[must_use]
  pub const fn is_found(self) -> bool {
    self.0 != Self::NOT_FOUND
  }

  /// 原版 is_not_found
  #[inline]
  #[must_use]
  pub const fn is_not_found(self) -> bool {
    self.0 == Self::NOT_FOUND
  }

  /// 原版 raw_value
  #[inline]
  #[must_use]
  pub const fn raw_value(self) -> usize {
    self.0
  }

  /// 原版 as_uint32
  #[inline]
  #[must_use]
  pub const fn as_uint32(self) -> u32 {
    self.0 as u32
  }

  /// 原版 as_int
  #[inline]
  #[must_use]
  pub const fn as_int(self) -> i32 {
    self.0 as i32
  }

  /// 原版 Range::begin 的迭代起点
  #[inline]
  #[must_use]
  pub const fn range(min: usize, max: usize) -> Range<usize> {
    min..max
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(InternalIndex::NOT_FOUND == usize::MAX);
  assert!(InternalIndex::not_found().is_not_found());
  assert!(InternalIndex::new(3).adjust_up(2).raw_value() == 5);
  assert!(InternalIndex::new(5).adjust_down(2).raw_value() == 3);
};
