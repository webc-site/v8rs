// 由 tool/src/utils/consts.js 从 v8/src/utils/bit-vector.h;src/utils/bit-vector.cc 生成; 修订就地改, 复跑覆盖

//! 定长位向量, 移植自 v8/src/utils/bit-vector。
//!
//! 上游以 Zone 分配 + 单字 inline 存储; Rust 版收敛为 `Vec<usize>` 承载
//! (data 长度恒 >= 1, 与上游 inline 槽位约定一致), Zone 参数从 API 中移除。

/// 单个机器字位数 (kDataBits = kBitsPerSystemPointer)
pub const DATA_BITS: u32 = usize::BITS;
/// 位数以 2 为底的对数 (kDataBitShift)
pub const DATA_BIT_SHIFT: u32 = DATA_BITS.trailing_zeros();
/// 全 1 字掩码。
const WORD_MASK: usize = usize::MAX;

/// `BitVector` (bit-vector.h:30): 定长位集, 迭代序升序。
#[derive(Debug, Clone, Default)]
pub struct BitVector {
  length: usize,
  data: Vec<usize>,
}

/// 正向迭代器 (bit-vector.h:84 Iterator)。
pub struct Iter<'a> {
  /// 下一候选字下标。
  word: usize,
  /// 当前字内下一候选位。
  bit: u32,
  data: &'a [usize],
}

/// 反向迭代器 (bit-vector.h:103 ReverseIterator)。
pub struct IterRev<'a> {
  word: usize,
  bit: u32,
  data: &'a [usize],
}

impl BitVector {
  /// `BitVector(length, zone)` (bit-vector.h:139): data 长度恒 >= 1。
  #[must_use]
  pub fn new(length: usize) -> Self {
    let data_length = (length + DATA_BITS as usize - 1) >> DATA_BIT_SHIFT;
    Self {
      length,
      data: vec![0; data_length.max(1)],
    }
  }

  /// `CopyFrom(other)` (bit-vector.h:206): 等长逐字覆盖。
  pub fn copy_from(&mut self, other: &Self) {
    debug_assert_eq!(other.length, self.length);
    self.data.clone_from(&other.data);
  }

  /// `BitVector(other, zone)` 拷贝构造等价。
  #[must_use]
  pub fn clone_from(other: &Self) -> Self {
    other.clone()
  }

  /// `CopyFrom(other, index, count)` (bit-vector.h:212): 区间掩码拷贝。
  pub fn copy_from_range(&mut self, other: &Self, index: usize, count: usize) {
    debug_assert!(other.length <= self.length);
    debug_assert!(index + count <= other.length);
    self.for_range(index, count, other, |dst, src, mask| {
      *dst = (*dst & !mask) | (src & mask);
    });
  }

  /// `Resize` (bit-vector.h:232): 只增不缩 (DCHECK_GT)。
  pub fn resize(&mut self, new_length: usize) {
    debug_assert!(new_length > self.length);
    let new_data_length = (new_length + DATA_BITS as usize - 1) >> DATA_BIT_SHIFT;
    self.data.resize(new_data_length.max(1), 0);
    self.length = new_length;
  }

  /// `Contains` (bit-vector.h:250)。
  #[must_use]
  pub fn contains(&self, i: usize) -> bool {
    debug_assert!(i < self.length);
    self.data[i >> DATA_BIT_SHIFT] & Self::bit(i) != 0
  }

  /// `Add(i)` (bit-vector.h:254)。
  pub fn add(&mut self, i: usize) {
    debug_assert!(i < self.length);
    self.data[i >> DATA_BIT_SHIFT] |= Self::bit(i);
  }

  /// `Add(index, count)` (bit-vector.h:258): 区间置位。
  pub fn add_range(&mut self, index: usize, count: usize) {
    debug_assert!(index + count <= self.length);
    self.for_range_inplace(index, count, WORD_MASK, |w, mask| *w |= mask);
  }

  /// `AddAll` (bit-vector.h:266): 尾部不完整字按 length 截断。
  pub fn add_all(&mut self) {
    if self.length == 0 {
      return;
    }
    let partial_size = self.length % DATA_BITS as usize;
    let bulk_size = self.data.len() - usize::from(partial_size != 0);
    self.data[..bulk_size].fill(WORD_MASK);
    if partial_size != 0 {
      self.data[bulk_size] = WORD_MASK >> (DATA_BITS - partial_size as u32);
    }
  }

  /// `Remove(i)` (bit-vector.h:276)。
  pub fn remove(&mut self, i: usize) {
    debug_assert!(i < self.length);
    self.data[i >> DATA_BIT_SHIFT] &= !Self::bit(i);
  }

  /// `Remove(index, count)` (bit-vector.h:280): 区间清位。
  pub fn remove_range(&mut self, index: usize, count: usize) {
    debug_assert!(index + count <= self.length);
    self.for_range_inplace(index, count, 0, |w, mask| *w &= !mask);
  }

  /// `Union` (bit-vector.h:290)。
  pub fn union(&mut self, other: &Self) {
    debug_assert!(other.length <= self.length);
    for (dst, src) in self.data.iter_mut().zip(other.data.iter()) {
      *dst |= src;
    }
  }

  /// `UnionIsChanged` (bit-vector.h:297)。
  pub fn union_is_changed(&mut self, other: &Self) -> bool {
    debug_assert!(other.length <= self.length);
    let mut changed = false;
    for (dst, src) in self.data.iter_mut().zip(other.data.iter()) {
      let old = *dst;
      *dst |= src;
      changed |= *dst != old;
    }
    changed
  }

  /// `Intersect` (bit-vector.h:307)。
  pub fn intersect(&mut self, other: &Self) {
    debug_assert_eq!(other.length, self.length);
    for (dst, src) in self.data.iter_mut().zip(other.data.iter()) {
      *dst &= src;
    }
  }

  /// `IntersectIsChanged` (bit-vector.h:314)。
  pub fn intersect_is_changed(&mut self, other: &Self) -> bool {
    debug_assert_eq!(other.length, self.length);
    let mut changed = false;
    for (dst, src) in self.data.iter_mut().zip(other.data.iter()) {
      let old = *dst;
      *dst &= src;
      changed |= *dst != old;
    }
    changed
  }

  /// `Subtract` (bit-vector.h:325)。
  pub fn subtract(&mut self, other: &Self) {
    debug_assert_eq!(other.length, self.length);
    for (dst, src) in self.data.iter_mut().zip(other.data.iter()) {
      *dst &= !src;
    }
  }

  /// `IsSubsetOf` (bit-vector.h:333)。
  #[must_use]
  pub fn is_subset_of(&self, other: &Self) -> bool {
    debug_assert_eq!(other.length, self.length);
    self
      .data
      .iter()
      .zip(other.data.iter())
      .all(|(&a, &b)| a & !b == 0)
  }

  /// `Clear` (bit-vector.h:339)。
  pub fn clear(&mut self) {
    self.data.fill(0);
  }

  /// `IsEmpty` (bit-vector.h:341)。
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.data.iter().all(|&w| w == 0)
  }

  /// `Equals` (bit-vector.h:343)。
  #[must_use]
  pub fn equals(&self, other: &Self) -> bool {
    self.data == other.data
  }

  /// `Count` (bit-vector.cc:25): 置位总数。
  #[must_use]
  pub fn count(&self) -> usize {
    self.data.iter().map(|w| w.count_ones() as usize).sum()
  }

  /// `length` (bit-vector.h:345)。
  #[must_use]
  pub const fn length(&self) -> usize {
    self.length
  }

  /// `UsedLength` (bit-vector.h:348): 最高置位位 + 1。
  #[must_use]
  pub fn used_length(&self) -> usize {
    self.iter_rev().next().map_or(0, |last| last + 1)
  }

  /// `begin/end` (bit-vector.h:355)。
  pub fn iter(&self) -> Iter<'_> {
    Iter {
      word: 0,
      bit: 0,
      data: &self.data,
    }
  }

  /// `rbegin/rend` (bit-vector.h:359)。
  pub fn iter_rev(&self) -> IterRev<'_> {
    IterRev {
      word: self.data.len(),
      bit: 0,
      data: &self.data,
    }
  }

  const fn bit(index: usize) -> usize {
    1 << (index & (DATA_BITS as usize - 1))
  }

  /// `ForRange` (bit-vector.h:374): 头/尾字掩码 + 中段整字回调; count==0 早退。
  fn for_range(
    &mut self,
    index: usize,
    count: usize,
    other: &Self,
    mut word_op: impl FnMut(&mut usize, &usize, usize),
  ) {
    if count == 0 {
      return;
    }
    let start_word = index >> DATA_BIT_SHIFT;
    let start_bit = index & (DATA_BITS as usize - 1);
    if start_bit + count <= DATA_BITS as usize {
      let mask = (WORD_MASK >> (DATA_BITS as usize - count)) << start_bit;
      word_op(&mut self.data[start_word], &other.data[start_word], mask);
      return;
    }
    let first_mask = WORD_MASK << start_bit;
    word_op(
      &mut self.data[start_word],
      &other.data[start_word],
      first_mask,
    );

    let mut remaining = count - (DATA_BITS as usize - start_bit);
    let mut current_word = start_word + 1;
    let full_words = remaining >> DATA_BIT_SHIFT;
    // 中段整字直拷 (上游 CopyImpl)
    self.data[current_word..current_word + full_words]
      .copy_from_slice(&other.data[current_word..current_word + full_words]);

    remaining &= DATA_BITS as usize - 1;
    if remaining > 0 {
      current_word += full_words;
      let last_mask = WORD_MASK >> (DATA_BITS as usize - remaining);
      word_op(
        &mut self.data[current_word],
        &other.data[current_word],
        last_mask,
      );
    }
  }

  /// ForRange 的就地变体 (Add/Remove 区间): 中段整字写 full_word
  /// (Add → 全 1, Remove → 0, 对应原版 full_words_op 的两个 fill 实参)。
  fn for_range_inplace(
    &mut self,
    index: usize,
    count: usize,
    full_word: usize,
    mut op: impl FnMut(&mut usize, usize),
  ) {
    if count == 0 {
      return;
    }
    let start_word = index >> DATA_BIT_SHIFT;
    let start_bit = index & (DATA_BITS as usize - 1);
    if start_bit + count <= DATA_BITS as usize {
      let mask = (WORD_MASK >> (DATA_BITS as usize - count)) << start_bit;
      op(&mut self.data[start_word], mask);
      return;
    }
    op(&mut self.data[start_word], WORD_MASK << start_bit);

    let mut remaining = count - (DATA_BITS as usize - start_bit);
    let mut current_word = start_word + 1;
    let full_words = remaining >> DATA_BIT_SHIFT;
    self.data[current_word..current_word + full_words].fill(full_word);

    remaining &= DATA_BITS as usize - 1;
    if remaining > 0 {
      current_word += full_words;
      op(
        &mut self.data[current_word],
        WORD_MASK >> (DATA_BITS as usize - remaining),
      );
    }
  }
}

impl Iterator for Iter<'_> {
  type Item = usize;

  fn next(&mut self) -> Option<usize> {
    // 逐字扫描剩余位: 首个非零字取最低置位
    while self.word < self.data.len() {
      let word = self.data[self.word];
      let rest = word >> self.bit;
      if rest != 0 {
        let tz = rest.trailing_zeros();
        let index = self.word * DATA_BITS as usize + self.bit as usize + tz as usize;
        self.bit += tz + 1;
        return Some(index);
      }
      self.word += 1;
      self.bit = 0;
    }
    None
  }
}

impl Iterator for IterRev<'_> {
  type Item = usize;

  fn next(&mut self) -> Option<usize> {
    while self.word > 0 {
      let word = self.data[self.word - 1];
      // bit 记 "已消费位 + 1": 排除 bit-1 及以上, 找剩余部分的最高位
      let lead = if self.bit == 0 {
        word
      } else {
        word & ((1 << (self.bit - 1)) - 1)
      };
      if lead != 0 {
        let top = usize::BITS - lead.leading_zeros() - 1; // 最高置位位号
        let index = (self.word - 1) * DATA_BITS as usize + top as usize;
        self.bit = top + 1;
        return Some(index);
      }
      self.word -= 1;
      self.bit = 0;
    }
    None
  }
}

/// `GrowableBitVector` (bit-vector.h:439): 按需倍增扩容的位集。
#[derive(Debug, Clone, Default)]
pub struct GrowableBitVector {
  bits: BitVector,
}

impl GrowableBitVector {
  /// `GrowableBitVector(length, zone)` (bit-vector.h:442)。
  #[must_use]
  pub fn new(length: usize) -> Self {
    Self {
      bits: BitVector::new(length),
    }
  }

  /// `Contains` (bit-vector.h:444): 越界即 false。
  #[must_use]
  pub fn contains(&self, value: usize) -> bool {
    self.in_bits_range(value) && self.bits.contains(value)
  }

  /// `Add(value, zone)` (bit-vector.h:448): 越界先 Grow。
  pub fn add(&mut self, value: usize) {
    if !self.in_bits_range(value) {
      self.grow(value);
    }
    self.bits.add(value);
  }

  /// `Add(other, zone)` (bit-vector.h:454): 并入另一向量全部位。
  pub fn add_all(&mut self, other: &Self) {
    if other.bits.length() == 0 {
      return;
    }
    let max = other.bits.length() - 1;
    if !self.in_bits_range(max) {
      self.grow(max);
    }
    self.bits.union(&other.bits);
  }

  /// `IsEmpty` (bit-vector.h:460)。
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.bits.is_empty()
  }

  /// `Clear` (bit-vector.h:462)。
  pub fn clear(&mut self) {
    self.bits.clear();
  }

  /// `length` (bit-vector.h:464)。
  #[must_use]
  pub fn length(&self) -> usize {
    self.bits.length()
  }

  /// `Equals` (bit-vector.h:466)。
  #[must_use]
  pub fn equals(&self, other: &Self) -> bool {
    self.length() == other.length() && self.bits.equals(&other.bits)
  }

  /// `begin/end` (bit-vector.h:470)。
  pub fn iter(&self) -> Iter<'_> {
    self.bits.iter()
  }

  fn in_bits_range(&self, value: usize) -> bool {
    self.bits.length() > value
  }

  /// `Grow` (bit-vector.h:497): bit_ceil(needed+1), 下限 kInitialLength=1024;
  /// 分配尺寸恒为 2 的幂且严格大于最大值。
  fn grow(&mut self, needed_value: usize) {
    const INITIAL_LENGTH: usize = 1024;
    const MAX_SUPPORTED_VALUE: usize = (1 << 30) - 1;
    debug_assert!(!self.in_bits_range(needed_value));
    assert!(
      needed_value <= MAX_SUPPORTED_VALUE,
      "growable bit vector 越上界"
    );
    let new_length = INITIAL_LENGTH.max((needed_value + 1).next_power_of_two());
    self.bits.resize(new_length);
  }
}
