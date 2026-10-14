//! 原版 v8/src/numbers/math-random.cc/h — Math.random 状态机
//!
//! 语义红线 (todo 10.3): XorShift128+ 1:1 转写, 禁 fastrand/rand。
//! Context/PodArray/FixedDoubleArray 绑定推迟到对象层; 数值状态机
//! (种子派生 MurmurHash3, 缓存重填) 以纯函数落地。

/// kCacheSize — 每次重填生成的随机数个数
pub const CACHE_SIZE: usize = 64;

/// kStateSize — 状态字节数 (2 × i64)
pub const STATE_SIZE: usize = 2 * 8;

/// MathRandom::State — xorshift128+ 双字状态
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct State {
  /// 状态字 0
  pub s0: u64,
  /// 状态字 1
  pub s1: u64,
}

impl State {
  /// 全零判定 (未初始化哨兵)
  #[must_use]
  pub const fn is_zero(&self) -> bool {
    self.s0 == 0 && self.s1 == 0
  }
}

/// RandomNumberGenerator::XorShift128 — xorshift128+ 步进
/// (random-number-generator.h 123-132, 与 builtins/math.tq 同源)
#[must_use]
#[inline]
pub fn xor_shift128(state0: &mut u64, state1: &mut u64) -> u64 {
  let mut s1 = *state0;
  let s0 = *state1;
  *state0 = s0;
  s1 ^= s1 << 23;
  s1 ^= s1 >> 17;
  s1 ^= s0;
  s1 ^= s0 >> 26;
  *state1 = s1;
  s0.wrapping_add(s1)
}

/// RandomNumberGenerator::ToDouble — u64 → [0, 1): 丢 11 位后除 2^53
#[must_use]
#[inline]
pub fn random_to_double(random: u64) -> f64 {
  // (random >> 11) ∈ [0, 2^53), 除 2^53 映射到 [0, 1)
  (random >> 11) as f64 / 9_007_199_254_740_992.0
}

/// RandomNumberGenerator::MurmurHash3 — u64 终结化散列
/// (random-number-generator.cc 226-232)
#[must_use]
#[inline]
pub fn murmur_hash3(mut h: u64) -> u64 {
  h ^= h >> 33;
  h = h.wrapping_mul(0xFF51_AFD7_ED55_8CCD);
  h ^= h >> 33;
  h = h.wrapping_mul(0xC4CE_B9FE_1A85_EC53);
  h ^= h >> 33;
  h
}

/// InitializeAndMaybeRefillCache 的数值核心
///
/// * 全零状态时按 seed 派生: s0 = MurmurHash3(seed), s1 = MurmurHash3(~seed)
///   (与原版一致要求派生后非全零, 违反即输入错误 → debug 断言)
/// * 生成 CACHE_SIZE 个随机数, 倒序填入 cache 并返回终态
///
/// 返回 (终态, cache); Context/PodArray 写回推迟到对象层
#[must_use]
pub fn initialize_and_refill(mut state: State, seed: u64, cache: &mut [f64; CACHE_SIZE]) -> State {
  if state.is_zero() {
    state.s0 = murmur_hash3(seed);
    state.s1 = murmur_hash3(!seed);
    debug_assert!(!state.is_zero());
  }
  let State { s0, s1 } = &mut state;
  for slot in cache.iter_mut().rev() {
    // xorshift128+ 生成, 倒序写缓存
    *slot = random_to_double(xor_shift128(s0, s1));
  }
  state
}

/// 默认种子随机源 — 原版取 isolate RNG 的 NextInt64;
/// 由调用方注入 (fastrand/getrandom 在 v8-base 层), 此处仅契约
#[must_use]
pub fn default_state_from_seed(seed: u64) -> State {
  State {
    s0: murmur_hash3(seed),
    s1: murmur_hash3(!seed),
  }
}
