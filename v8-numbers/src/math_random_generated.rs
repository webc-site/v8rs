// 由 tool/src/numbers/math-random.h.js 从 v8/src/numbers/math-random.{h,cc} 生成; 修订就地改, 复跑覆盖
// MathRandom 常量与状态机实现

pub use crate::math_random::State;
use crate::math_random::murmur_hash3;

/// MathRandom 缓存常量 (原版 class MathRandom : AllStatic)
pub mod math_random_const {
  /// 随机数缓存条数 (原版 kCacheSize)
  pub const CACHE_SIZE: i32 = 64;

  /// 状态字节宽 = 2 * kInt64Size (原版 kStateSize)
  pub const STATE_SIZE: i32 = 16;

  /// kInt64Size (src/common/globals.h)
  pub const INT64_SIZE: i32 = 8;

  /// 默认未传 Isolate 时的降级回退种子 (原版 InitializeAndMaybeRefillCache 默认随机源)
  pub const DEFAULT_MATH_RANDOM_SEED: u64 = 0x2545_F491_4F6C_DD1D;
}

/// 平台分支 (原版 const bool kUseRefillCache = kSystemPointerSize == 4):
/// x64/arm64 目标 kSystemPointerSize==8 → 不用 RefillCache, 直接 xorshift128+ 即取
pub const USE_REFILL_CACHE: bool = false;

/// 原版 native_context 上挂载的随机状态与缓存
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NativeContextRandomState {
  pub state: State,
  pub index: i32,
  pub cache: [f64; math_random_const::CACHE_SIZE as usize],
}

impl Default for NativeContextRandomState {
  fn default() -> Self {
    Self::new()
  }
}

impl NativeContextRandomState {
  #[inline]
  #[must_use]
  pub const fn new() -> Self {
    Self {
      state: State { s0: 0, s1: 0 },
      index: 0,
      cache: [0.0; math_random_const::CACHE_SIZE as usize],
    }
  }

  #[inline]
  pub fn reset(&mut self) {
    self.index = 0;
    self.state = State { s0: 0, s1: 0 };
    self.cache.fill(0.0);
  }
}

/// MathRandom 状态机与运算骨架实现
pub mod math_random_skel {
  use std::mem::align_of;

  use super::{NativeContextRandomState, State, USE_REFILL_CACHE, math_random_const};
  use crate::math_random::{murmur_hash3, random_to_double, xor_shift128};

  /// 初始化上下文随机状态 Pod (原版 InitializeContext)
  pub fn initialize_context(_isolate_key: usize, native_context: usize) {
    if native_context != 0 {
      debug_assert_eq!(
        0,
        native_context % align_of::<NativeContextRandomState>(),
        "native_context 必须按 alignof(NativeContextRandomState) 对齐"
      );
      // SAFETY: native_context 非空且指向已对齐的 NativeContextRandomState 结构体
      let ctx = unsafe { &mut *(native_context as *mut NativeContextRandomState) };
      ctx.state = State { s0: 0, s1: 0 };
      ctx.index = 0;
      ctx.cache.fill(0.0);
    }
  }

  /// 复位: index=Smi::zero(), state={0,0} (原版 ResetContext)
  pub fn reset_context(native_context: usize) {
    if native_context != 0 {
      debug_assert_eq!(
        0,
        native_context % align_of::<NativeContextRandomState>(),
        "native_context 必须按 alignof(NativeContextRandomState) 对齐"
      );
      // SAFETY: native_context 非空且指向已对齐的 NativeContextRandomState 结构体
      let ctx = unsafe { &mut *(native_context as *mut NativeContextRandomState) };
      ctx.reset();
    }
  }

  /// ExternalReference 入口: 状态未初始化 (s0==s1==0) 时按 random_seed 或
  /// RNG 播种 — s0=MurmurHash3(seed), s1=MurmurHash3(~seed);
  /// 32 位路径回填 cache; 64 位路径返回 Smi::zero (原版 InitializeAndMaybeRefillCache)
  pub fn initialize_and_maybe_refill_cache(isolate_key: usize, raw_native_context: usize) -> usize {
    if raw_native_context == 0 {
      return 0;
    }
    debug_assert_eq!(
      0,
      raw_native_context % align_of::<NativeContextRandomState>(),
      "raw_native_context 必须按 alignof(NativeContextRandomState) 对齐"
    );
    // SAFETY: raw_native_context 非空且指向已对齐的 NativeContextRandomState 结构体
    let ctx = unsafe { &mut *(raw_native_context as *mut NativeContextRandomState) };
    if ctx.state.is_zero() {
      let seed = if isolate_key != 0 {
        isolate_key as u64
      } else {
        math_random_const::DEFAULT_MATH_RANDOM_SEED
      };
      ctx.state.s0 = murmur_hash3(seed);
      ctx.state.s1 = murmur_hash3(!seed);
      if ctx.state.is_zero() {
        ctx.state.s0 = 1;
      }
    }

    if USE_REFILL_CACHE {
      for slot in ctx.cache.iter_mut().rev() {
        let rand = xor_shift128(&mut ctx.state.s0, &mut ctx.state.s1);
        *slot = random_to_double(rand);
      }
      ctx.index = math_random_const::CACHE_SIZE;
      math_random_const::CACHE_SIZE as usize
    } else {
      0
    }
  }

  /// xorshift128+ 单步 (原版 base::RandomNumberGenerator::XorShift128)
  #[inline]
  #[must_use]
  pub fn xorshift128(s0: &mut u64, s1: &mut u64) -> u64 {
    xor_shift128(s0, s1)
  }

  /// 生成 [0, 1) 之间的双精度浮点数 (原版 NextDouble)
  #[inline]
  #[must_use]
  pub fn next_double(s0: &mut u64, s1: &mut u64) -> f64 {
    random_to_double(xor_shift128(s0, s1))
  }
}

/// 独立的 Math.random 伪随机状态机包装
#[derive(Debug, Clone, PartialEq)]
pub struct MathRandomMachine {
  pub state: State,
  pub cache: [f64; math_random_const::CACHE_SIZE as usize],
  pub index: i32,
}

impl Default for MathRandomMachine {
  fn default() -> Self {
    Self::new()
  }
}

impl MathRandomMachine {
  #[inline]
  #[must_use]
  pub const fn new() -> Self {
    Self {
      state: State { s0: 0, s1: 0 },
      cache: [0.0; math_random_const::CACHE_SIZE as usize],
      index: 0,
    }
  }

  /// 状态重置
  #[inline]
  pub fn reset(&mut self) {
    self.index = 0;
    self.state = State { s0: 0, s1: 0 };
    self.cache.fill(0.0);
  }

  /// 播种初始化
  #[inline]
  pub fn init_seed(&mut self, seed: u64) {
    self.state.s0 = murmur_hash3(seed);
    self.state.s1 = murmur_hash3(!seed);
    if self.state.is_zero() {
      self.state.s0 = 1;
    }
  }

  /// 产生下一个随机双精度数 [0, 1) (直取路径)
  #[inline]
  pub fn next_double(&mut self) -> f64 {
    if self.state.is_zero() {
      self.init_seed(math_random_const::DEFAULT_MATH_RANDOM_SEED);
    }
    math_random_skel::next_double(&mut self.state.s0, &mut self.state.s1)
  }

  /// 缓存重填: 按 xorshift128+ 倒序填满 cache 缓冲区 (原版 MathRandom::RefillCache)
  pub fn refill_cache(&mut self) {
    if self.state.is_zero() {
      self.init_seed(math_random_const::DEFAULT_MATH_RANDOM_SEED);
    }
    for slot in self.cache.iter_mut().rev() {
      *slot = math_random_skel::next_double(&mut self.state.s0, &mut self.state.s1);
    }
    self.index = math_random_const::CACHE_SIZE;
  }

  /// 从缓存获取下一个随机数 (若耗尽则触发 refill_cache 重新装填)
  #[inline]
  pub fn next_cached_double(&mut self) -> f64 {
    if self.index <= 0 {
      self.refill_cache();
    }
    self.index -= 1;
    self.cache[self.index as usize]
  }
}
