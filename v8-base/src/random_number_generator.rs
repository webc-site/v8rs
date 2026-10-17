//! v8/src/base/utils/random-number-generator.{h,cc} 的 1:1 转写
//!
//! 64 位 seed 经 MurmurHash3 派生 xorshift128+ 双状态, 周期 2^128-1。
//! 同 seed 同调用序列 → 同输出流 (引擎快照/复现语义基石)。
//! 差异登记:
//! - 熵源默认路径取 POSIX 分支 (/dev/urandom), 失败回退系统时钟混合;
//!   Windows/Starboard 分支不适用 (仅覆盖 macOS/Linux)。
//! - `CHECK_LE(n, max)` 等入参校验以 `Result` 承接 (禁 panic), 不再 abort。

use std::{
  collections::HashSet,
  fs::File,
  io::Read,
  process::id,
  sync::{LazyLock, Mutex},
  time::{SystemTime, UNIX_EPOCH},
};

use crate::{Error, Result, bits::BitOps};

/// EntropySource: 嵌入方注入的熵回调 (原版 bool (*)(unsigned char*, size_t))
pub type EntropySource = fn(buf: &mut [u8]) -> bool;

static ENTROPY_SOURCE: LazyLock<Mutex<Option<EntropySource>>> = LazyLock::new(|| Mutex::new(None));

/// SetEntropySource (进程级, 原版走 LazyMutex)
pub fn set_entropy_source(source: Option<EntropySource>) {
  if let Ok(mut slot) = ENTROPY_SOURCE.lock() {
    *slot = source;
  }
}

/// 从注册熵源取 8 字节熵 (原版首次构造时的嵌入方熵分支)
fn seed_from_entropy_source() -> Option<i64> {
  let source = { *ENTROPY_SOURCE.lock().ok()? }?;
  let mut buf = [0_u8; 8];
  source(&mut buf).then(|| i64::from_le_bytes(buf))
}

/// 从 /dev/urandom 取 8 字节熵 (原版 POSIX 分支)
fn seed_from_urandom() -> Option<i64> {
  let mut f = File::open("/dev/urandom").ok()?;
  let mut buf = [0_u8; 8];
  f.read_exact(&mut buf).ok()?;
  Some(i64::from_le_bytes(buf))
}

/// 时钟混合回退熵 (原版 Time::NowFromSystemTime << 24 ^ TimeTicks::Now)
fn seed_from_clock() -> i64 {
  let sys = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .map_or(0_u64, |d| d.as_nanos() as u64);
  // Instant 无稳定数值表达, 以 栈地址 + 进程号 组合噪声承接原版弱熵意图
  // (原版 TimeTicks::Now 提供单调时钟异或项)
  let noise = (&sys as *const u64 as usize) ^ (id() as usize);
  ((sys << 24) ^ noise as u64) as i64
}

/// 伪随机数发生器 (非重入、非线程安全, 与原版同款约束)
#[derive(Debug, Clone, Copy)]
pub struct RandomNumberGenerator {
  initial_seed: i64,
  state0: u64,
  state1: u64,
}

/// 原版 kMultiplier: 0x5'deec'e66d
const MULTIPLIER: i64 = 0x5dee_ce66d;
/// 原版 kAddend: 0xb
const ADDEND: i64 = 0xb;
/// 原版 kMask: 0xffff'ffff'ffff
const MASK: i64 = 0xffff_ffff_ffff;

impl RandomNumberGenerator {
  /// RandomNumberGenerator(): 嵌入方熵 → /dev/urandom → 时钟回退
  #[must_use]
  pub fn new() -> Self {
    if let Some(seed) = seed_from_entropy_source() {
      return Self::seeded(seed);
    }
    if let Some(seed) = seed_from_urandom() {
      return Self::seeded(seed);
    }
    Self::seeded(seed_from_clock())
  }

  /// RandomNumberGenerator(int64_t seed) — 不触熵源, 直接种子 (原版同款)
  #[must_use]
  pub fn with_seed(seed: i64) -> Self {
    Self::seeded(seed)
  }

  fn seeded(seed: i64) -> Self {
    // MurmurHash3 是双射: state0/state1 不可能同时为 0。
    // 原版 rng.cc:222 为 CHECK (release 恒执行), 此处保持 assert 同强度。
    let state0 = murmur_hash3(seed as u64);
    let state1 = murmur_hash3(!state0);
    assert!(state0 != 0 || state1 != 0, "种子状态全零");
    Self {
      initial_seed: seed,
      state0,
      state1,
    }
  }

  /// NextInt(): 全宽 32 位
  #[inline]
  pub fn next_int(&mut self) -> i32 {
    self.next(32)
  }

  /// NextInt(max): [0, max), 2 的幂走乘法快路径, 否则拒绝采样
  pub fn next_bounded_int(&mut self, max: i32) -> i32 {
    debug_assert!(0 < max);
    // 2 的幂快路径
    if max.is_power_of_two() {
      return ((max as i64 * self.next(31) as i64) >> 31) as i32;
    }
    // 拒绝采样: 避免模偏差
    loop {
      let rnd = self.next(31);
      let val = rnd % max;
      if i32::MAX - (rnd - val) >= max - 1 {
        return val;
      }
    }
  }

  /// NextBool()
  #[inline]
  pub fn next_bool(&mut self) -> bool {
    self.next(1) != 0
  }

  /// NextDouble(): [0, 1)
  #[inline]
  pub fn next_double(&mut self) -> f64 {
    Self::to_double(Self::xor_shift128(&mut self.state0, &mut self.state1))
  }

  /// NextInt64()
  #[inline]
  pub fn next_int64(&mut self) -> i64 {
    Self::xor_shift128(&mut self.state0, &mut self.state1) as i64
  }

  /// NextBytes(buffer): 逐字节 Next(8) (与原版一致的流消耗)
  pub fn next_bytes(&mut self, buffer: &mut [u8]) {
    for b in buffer.iter_mut() {
      *b = self.next(8) as u8;
    }
  }

  /// NextSample(max, n): n 个 < max 的互异 u64; n > max 时 Err (原版 CHECK_LE)
  pub fn next_sample(&mut self, max: u64, n: usize) -> Result<Vec<u64>> {
    if n as u64 > max {
      return Err(Error::InvalidArg("next_sample: n > max"));
    }
    if n == 0 {
      return Ok(Vec::new());
    }
    // 选/弃择小边, 省发生器调用次数
    let smaller_part = (max - n as u64).min(n as u64) as usize;
    let mut selected: HashSet<u64> = HashSet::with_capacity(smaller_part);
    let mut counter = 0_usize;
    while selected.len() != smaller_part && counter / 3 < smaller_part {
      let x = (self.next_double() * max as f64) as u64;
      // 原版 rng.cc:158 CHECK_LT(x, max): release 亦执行, 防越界值混入样本
      assert!(x < max, "next_sample: 采样值越界");
      selected.insert(x);
      counter += 1;
    }
    if selected.len() == smaller_part {
      if smaller_part != n {
        return Ok(complement_sample(&selected, max));
      }
      // 原版 unordered_set 迭代序本就不确定, 顺序语义等价
      return Ok(selected.into_iter().collect());
    }
    // 小边 3 倍步数内未凑齐, 走慢路径 (把已选的当排除集)
    self.next_sample_slow(max, n, &selected)
  }

  /// NextSampleSlow(max, n, excluded): 全量候选剔排除集; max-|excluded| < n 时 Err
  pub fn next_sample_slow(
    &mut self,
    max: u64,
    n: usize,
    excluded: &HashSet<u64>,
  ) -> Result<Vec<u64>> {
    if excluded.len() as u64 > max || (max - excluded.len() as u64) < n as u64 {
      return Err(Error::InvalidArg("next_sample_slow: max - |excluded| < n"));
    }
    let mut result: Vec<u64> = Vec::with_capacity((max - excluded.len() as u64) as usize);
    for i in 0..max {
      if !excluded.contains(&i) {
        result.push(i);
      }
    }
    // 大边为准继续收缩, 同样省调用
    let larger_part = (max - n as u64).max(n as u64) as usize;
    // 排除集可能已使初始候选小于 larger_part
    while result.len() != larger_part && result.len() > n {
      let x = (self.next_double() * result.len() as f64) as usize;
      // 原版 rng.cc:197 CHECK_LT(x, result.size()): release 亦执行
      assert!(x < result.len(), "next_sample_slow: 交换下标越界");
      let last = result.len() - 1;
      result.swap(x, last);
      result.pop();
    }
    if result.len() != n {
      let set: HashSet<u64> = result.iter().copied().collect();
      return Ok(complement_sample(&set, max));
    }
    Ok(result)
  }

  /// SetSeed(seed): 覆盖当前 seed 并重置双状态
  pub fn set_seed(&mut self, seed: i64) {
    *self = Self::seeded(seed);
  }

  /// initial_seed()
  #[inline]
  #[must_use]
  pub const fn initial_seed(&self) -> i64 {
    self.initial_seed
  }

  /// ToDouble(random): [0, 2^53) → [0, 1) (丢低 11 位, 除以 2^53)
  #[inline]
  #[must_use]
  pub fn to_double(random: u64) -> f64 {
    let random_0_to_2_53 = (random >> 11) as f64;
    const K2_53: f64 = (1_u64 << 53) as f64;
    random_0_to_2_53 / K2_53
  }

  /// XorShift128(state0, state1): xorshift128+ 步进, 返回 s0+s1
  /// (与 builtins/math.tq 的 xorshift128 镜像保持一致)
  #[inline]
  pub fn xor_shift128(state0: &mut u64, state1: &mut u64) -> u64 {
    let s1 = *state0;
    let s0 = *state1;
    *state0 = s0;
    let mut x = s1;
    x ^= x << 23;
    x ^= x >> 17;
    x ^= s0;
    x ^= s0 >> 26;
    *state1 = x;
    s0.wrapping_add(x)
  }

  /// Next(bits): 取状态高 bits 位 (0 < bits <= 32)
  fn next(&mut self, bits: u32) -> i32 {
    debug_assert!(0 < bits && bits <= 32);
    let random = Self::xor_shift128(&mut self.state0, &mut self.state1);
    (random >> (64 - bits)) as u32 as i32
  }
}

impl Default for RandomNumberGenerator {
  fn default() -> Self {
    Self::new()
  }
}

/// MurmurHash3 的 64 位 finalizer (原版 MurmurHash3)
#[inline]
#[must_use]
pub const fn murmur_hash3(mut h: u64) -> u64 {
  h ^= h >> 33;
  h = h.wrapping_mul(0xFF51_AFD7_ED55_8CCD);
  h ^= h >> 33;
  h = h.wrapping_mul(0xC4CE_B9FE_1A85_EC53);
  h ^= h >> 33;
  h
}

/// 原版匿名 ComplementSample: 补集抽样 (0..max 排除 set)
fn complement_sample(set: &HashSet<u64>, max: u64) -> Vec<u64> {
  (0..max).filter(|i| !set.contains(i)).collect()
}

// 常数自检: 与原版字面量一致 (编译期锁定)
const _: () = {
  assert!(MULTIPLIER == 0x5dee_ce66d);
  assert!(ADDEND == 0xb);
  assert!(MASK == 0xffff_ffff_ffff);
};
