//! v8/src/base/ring-buffer.h 的 1:1 转写
//! template <typename T, uint8_t _SIZE = 10> class RingBuffer final
//! 原版为定长数组 + pos/is_full 环形语义 (Push/Size/Empty/Clear/Reduce),
//! 无 Head/Tail 指针族 (该版本已在上游移除), 逐条对齐以下断言:
//! - Push: 写入 elements_[pos_++]; pos_ == kSize 时回绕 0 并置 is_full_ = true
//! - Size: is_full_ ? kSize : pos_
//! - Empty: Size() == 0
//! - Clear: pos_ = 0; is_full_ = false (元素本体保留, 与原版一致)
//! - Reduce: 自最新元素向最旧折叠 (pos_-1..0, 再 kSize-1..pos_ 仅当 full)

/// constexpr RingBuffer() = default 的 Default 面
impl<T: Copy + Default, const SIZE: usize> Default for RingBuffer<T, SIZE> {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

/// RingBuffer<T, _SIZE>: 定长环形缓冲 (T 需可按原版 memset/copy 语义处理 → Copy + Default)
#[derive(Debug, Clone, Copy)]
pub struct RingBuffer<T: Copy, const SIZE: usize = 10> {
  /// T elements_[kSize]
  elements_: [T; SIZE],
  /// uint8_t pos_ = 0 (下一个写入位置)
  pos_: usize,
  /// bool is_full_ = false
  is_full_: bool,
}

/// kSize (原版 static constexpr uint8_t kSize = _SIZE)
pub const K_RING_BUFFER_DEFAULT_SIZE: usize = 10;

impl<T: Copy + Default, const SIZE: usize> RingBuffer<T, SIZE> {
  /// constexpr RingBuffer() = default
  #[inline]
  pub fn new() -> Self {
    Self {
      elements_: [T::default(); SIZE],
      pos_: 0,
      is_full_: false,
    }
  }

  /// constexpr void Push(const T& value)
  #[inline]
  pub fn push(&mut self, value: &T) {
    self.elements_[self.pos_] = *value;
    self.pos_ += 1;
    if self.pos_ == SIZE {
      self.pos_ = 0;
      self.is_full_ = true;
    }
  }

  /// constexpr uint8_t Size() const
  #[inline]
  pub fn size(&self) -> usize {
    if self.is_full_ { SIZE } else { self.pos_ }
  }

  /// constexpr bool Empty() const
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.size() == 0
  }

  /// Full 判定 (由 is_full_ 直接暴露, 语义即 Size() == kSize)
  #[inline]
  pub fn is_full(&self) -> bool {
    self.is_full_
  }

  /// constexpr void Clear()
  #[inline]
  pub fn clear(&mut self) {
    self.pos_ = 0;
    self.is_full_ = false;
  }

  /// constexpr T Reduce(Callback callback, const T& initial) const
  /// callback 以 (累加值, 元素) 折叠, 遍历顺序: 新 → 旧
  #[inline]
  pub fn reduce(&self, mut callback: impl FnMut(T, &T) -> T, initial: &T) -> T {
    let mut result = *initial;
    // for (uint8_t i = pos_; i > 0; --i) result = callback(result, elements_[i - 1]);
    for i in (1..=self.pos_).rev() {
      result = callback(result, &self.elements_[i - 1]);
    }
    if !self.is_full_ {
      return result;
    }
    // for (uint8_t i = kSize; i > pos_; --i) result = callback(result, elements_[i - 1]);
    for i in (self.pos_ + 1..=SIZE).rev() {
      result = callback(result, &self.elements_[i - 1]);
    }
    result
  }

  /// 当前有效元素快照 (最新在前) — 原版无对应 public API, 供测试/调试用
  pub fn snapshot_newest_first(&self) -> Vec<T> {
    let mut out = Vec::with_capacity(self.size());
    let n = self.size();
    for k in 0..n {
      // 最新元素位于 pos_-1, 依次回退 (环形)
      let idx = (self.pos_ + SIZE - 1 - k) % SIZE;
      out.push(self.elements_[idx]);
    }
    out
  }
}
