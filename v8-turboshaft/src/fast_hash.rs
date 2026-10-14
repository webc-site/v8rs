// 由 tool/src/compiler/turboshaft/fast-hash.h.js 从 v8/src/compiler/turboshaft/fast-hash.h 生成; 修订就地改, 复跑覆盖
// 快哈希: 特化分发表 + 17 倍混合 (1:1 实现)

// 原版 fast_hash 分发表 (主模板 + 8 特化):
// - 主模板: enum → static_cast<size_t> 直转, 其它走 base::hash
// - fast_hash<std::optional<T>>: has_value ? (fast_hash<T>(*v) << 1) + 1 : 0
// - fast_hash<std::pair<T1, T2>>: fast_hash_combine(first, second)
// - fast_hash<std::tuple<Ts...>>: 逐元素 fast_hash_combine
// - fast_hash<base::Vector<T>>: size 混入 + 逐元素
// - fast_hash<std::span<T, N>>: size 混入 + 逐元素
// - fast_hash<IndirectHandle<T>>: 地址 (deref)
// - fast_hash<MaybeIndirectHandle<T>>: 地址
// - fast_hash<std::array<T, N>>: 逐元素 (无 size 混入)
/// v8: src/compiler/turboshaft/fast-hash.h fast_hash
pub trait FastHash {
  fn fast_hash(&self) -> u64;
}

/// size_t 直传路径 (base::hash<size_t> 即恒等)
impl FastHash for u64 {
  fn fast_hash(&self) -> u64 {
    *self
  }
}

impl FastHash for u32 {
  fn fast_hash(&self) -> u64 {
    *self as u64
  }
}

impl FastHash for usize {
  fn fast_hash(&self) -> u64 {
    *self as u64
  }
}

/// 原版: has_value ? (fast_hash<T>()(*v) << 1) + 1 : 0
impl FastHash for Option<u64> {
  fn fast_hash(&self) -> u64 {
    match self {
      Some(v) => (v.fast_hash() << 1) + 1,
      None => 0,
    }
  }
}

/// 原版 fast_hash_combine(size_t acc, size_t value) = 17 * acc + value
/// v8: src/compiler/turboshaft/fast-hash.h fast_hash_combine(acc, value)
#[inline]
#[must_use]
pub const fn fast_hash_combine2(acc: u64, value: u64) -> u64 {
  17 * acc + value
}

/// 原版变参 fast_hash_combine(v, vs...) =
///   combine(combine(vs...), hash(v)) — 展开为 Σ 17^(i-1) * hash(v_i)
/// (首元素权重 1, 逐位后移)。空集 = 0, 单元素 = hash(v)。
/// v8: src/compiler/turboshaft/fast-hash.h fast_hash_combine(T, Ts...)
#[must_use]
pub fn fast_hash_combine(values: &[u64]) -> u64 {
  let mut acc = 0u64;
  let mut pow = 1u64;
  for v in values {
    acc = acc.wrapping_add(pow.wrapping_mul(*v));
    pow = pow.wrapping_mul(17);
  }
  acc
}

/// 原版 fast_hash_range(first, last): acc = combine(acc, *first) 左折叠。
/// size_t 元素走非模板重载 → acc = 17 * acc + value。
/// v8: src/compiler/turboshaft/fast-hash.h fast_hash_range
#[must_use]
pub fn fast_hash_range(values: &[u64]) -> u64 {
  let mut acc = 0u64;
  for v in values {
    acc = fast_hash_combine2(acc, *v);
  }
  acc
}

/// 原版 fast_hash<std::pair<T1, T2>>: combine(first, second)
/// (两参变参展开 = 17 * hash(second) + hash(first), 即 acc=second 权重高)
#[must_use]
pub fn fast_hash_pair(first: u64, second: u64) -> u64 {
  fast_hash_combine(&[first, second])
}
