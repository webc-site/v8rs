//! 内部条目索引 InternalIndex (对应 v8/src/objects/internal-index.h)

/// 条目索引包装类型，区分普通数组下标与哈希表桶条目
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InternalIndex(pub usize);

impl InternalIndex {
  pub const NOT_FOUND: Self = Self(usize::MAX);

  #[inline]
  pub const fn new(raw: usize) -> Self {
    Self(raw)
  }

  #[inline]
  pub const fn not_found() -> Self {
    Self::NOT_FOUND
  }

  #[inline]
  pub const fn is_found(&self) -> bool {
    self.0 != usize::MAX
  }

  #[inline]
  pub const fn is_not_found(&self) -> bool {
    self.0 == usize::MAX
  }

  #[inline]
  pub const fn as_usize(&self) -> usize {
    self.0
  }

  #[inline]
  pub const fn as_uint32(&self) -> u32 {
    debug_assert!(self.0 <= u32::MAX as usize);
    self.0 as u32
  }

  #[inline]
  pub const fn as_int(&self) -> i32 {
    debug_assert!(self.0 <= i32::MAX as usize);
    self.0 as i32
  }

  #[inline]
  pub const fn raw_value(&self) -> usize {
    self.0
  }

  #[inline]
  pub const fn adjust_down(&self, subtract: usize) -> Self {
    debug_assert!(self.0 >= subtract);
    Self(self.0 - subtract)
  }

  #[inline]
  pub const fn adjust_up(&self, add: usize) -> Self {
    debug_assert!(self.0 <= usize::MAX - add);
    Self(self.0 + add)
  }
}

/// 条目迭代范围
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InternalIndexRange {
  current: usize,
  max: usize,
}

impl InternalIndexRange {
  #[inline]
  pub const fn new(max: usize) -> Self {
    Self { current: 0, max }
  }

  #[inline]
  pub const fn with_bounds(min: usize, max: usize) -> Self {
    Self { current: min, max }
  }
}

impl Iterator for InternalIndexRange {
  type Item = InternalIndex;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    if self.current < self.max {
      let idx = InternalIndex::new(self.current);
      self.current += 1;
      Some(idx)
    } else {
      None
    }
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    let rem = self.max.saturating_sub(self.current);
    (rem, Some(rem))
  }
}

impl ExactSizeIterator for InternalIndexRange {}
