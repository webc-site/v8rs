//! v8/src/regexp/regexp-compiler.cc 的 DynamicBitSet 数据面 1:1 转写
//!
//! 对应 C++ `regexp::(anonymous)::DynamicBitSet` (regexp-compiler.cc:429-464)。
//! 小整数 (< kFirstLimit=32) 走位图, 其余落 ZoneList 去重表; 原版 Zone 分配
//! 以 Vec 承接 (行为等价: 仅集合语义, 无分配地址可观测面)。

/// kFirstLimit (regexp-compiler.cc:460)。
pub const K_FIRST_LIMIT: u32 = 32;

/// DynamicBitSet (regexp-compiler.cc:429-464)。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DynamicBitSet {
  /// uint32_t first_ = 0
  first: u32,
  /// ZoneList<unsigned>* remaining_ = nullptr → Vec (None ≡ nullptr)
  remaining: Option<Vec<u32>>,
}

impl DynamicBitSet {
  pub const fn new() -> Self {
    Self {
      first: 0,
      remaining: None,
    }
  }

  /// bool Get(unsigned value) const (regexp-compiler.cc:431-439)。
  pub fn get(&self, value: u32) -> bool {
    if value < K_FIRST_LIMIT {
      (self.first & (1u32 << value)) != 0
    } else if let Some(remaining) = &self.remaining {
      remaining.contains(&value)
    } else {
      false
    }
  }

  /// bool IsEmpty() const (regexp-compiler.cc:441-443)。
  pub fn is_empty(&self) -> bool {
    self.first == 0 && self.remaining.as_deref().is_none_or(<[u32]>::is_empty)
  }

  /// void Set(unsigned value, Zone* zone) (regexp-compiler.cc:446-457)。
  ///
  /// 原版惰性创建 remaining_ 并对已有元素去重 (Contains 检查), Vec 承接同语义。
  pub fn set(&mut self, value: u32) {
    if value < K_FIRST_LIMIT {
      self.first |= 1u32 << value;
    } else {
      let remaining = self.remaining.get_or_insert_with(|| Vec::with_capacity(1));
      if remaining.is_empty() || !remaining.contains(&value) {
        remaining.push(value);
      }
    }
  }

  /// 测试/对数用: 惰性块元素只读视图 (原版 private, 无公开访问面)。
  pub fn remaining(&self) -> Option<&[u32]> {
    self.remaining.as_deref()
  }

  /// 测试/对数用: first_ 位图原始值。
  pub const fn first_bits(&self) -> u32 {
    self.first
  }
}
