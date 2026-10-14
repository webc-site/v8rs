#!/usr/bin/env bun
// tool/src/numbers/math-random.h.js
// 机械提取 v8/src/numbers/math-random.{h,cc}: MathRandom 常量/State 布局/xorshift128+ 接口骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/numbers/math-random.h");
const ccSrc = readV8("src/numbers/math-random.cc");

// 常量提取
const cacheM = hSrc.match(/static const int kCacheSize = (\d+);/);
if (!cacheM) throw new Error("未找到 kCacheSize");
const cacheSize = parseInt(cacheM[1], 10);

const stateM = hSrc.match(/static const int kStateSize = 2 \* kInt64Size;/);
if (!stateM) throw new Error("未找到 kStateSize 定义");
const int64Size = 8; // kInt64Size (src/common/globals.h)
const stateSize = 2 * int64Size;

// cc 侧分支对数
const mustCc = [
  "InitializeContext",
  "ResetContext",
  "InitializeAndMaybeRefillCache",
  "MurmurHash3(seed)",
  "MurmurHash3(~seed)",
  "XorShift128(&state.s0, &state.s1)",
  "ToDouble(random)",
];
for (const k of mustCc) {
  if (!ccSrc.includes(k)) throw new Error(`math-random.cc 未找到 ${k}`);
}
// kUseRefillCache: 64 位目标 (kSystemPointerSize==8) → false
if (!/const bool kUseRefillCache = kSystemPointerSize == 4;/.test(ccSrc)) {
  throw new Error("未找到 kUseRefillCache 定义");
}

let rust = banner("src/numbers/math-random.h.js", "src/numbers/math-random.{h,cc}", "MathRandom 常量与骨架");

rust += `
/// MathRandom 缓存常量 (原版 class MathRandom : AllStatic)
pub mod math_random_const {
  /// 随机数缓存条数 (原版 kCacheSize)
  pub const CACHE_SIZE: i32 = ${cacheSize};

  /// 状态字节宽 = 2 * kInt64Size (原版 kStateSize)
  pub const STATE_SIZE: i32 = ${stateSize};

  /// kInt64Size (src/common/globals.h)
  pub const INT64_SIZE: i32 = ${int64Size};
}

/// xorshift128+ 状态 (原版 struct MathRandom::State)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct State {
  pub s0: u64,
  pub s1: u64,
}

/// 平台分支 (原版 const bool kUseRefillCache = kSystemPointerSize == 4):
/// x64/arm64 目标 kSystemPointerSize==8 → 不用 RefillCache, 直接 xorshift128+ 即取
pub const USE_REFILL_CACHE: bool = false;

/// MathRandom 签名骨架 (语义实现见 crate 内手写 math_random.rs)
pub mod math_random_skel {
  use super::State;

  /// 初始化上下文随机状态 Pod (原版 InitializeContext)
  pub fn initialize_context(_isolate_key: usize, _native_context: usize) {
    unimplemented!("骨架")
  }

  /// 复位: index=Smi::zero(), state={0,0} (原版 ResetContext)
  pub fn reset_context(_native_context: usize) {
    unimplemented!("骨架")
  }

  /// ExternalReference 入口: 状态未初始化 (s0==s1==0) 时按 random_seed 或
  /// RNG 播种 — s0=MurmurHash3(seed), s1=MurmurHash3(~seed);
  /// 32 位路径回填 cache; 64 位路径返回 Smi::zero (原版 InitializeAndMaybeRefillCache)
  pub fn initialize_and_maybe_refill_cache(_isolate_key: usize, _raw_native_context: usize) -> usize {
    unimplemented!("骨架")
  }

  /// xorshift128+ 单步 (原版 base::RandomNumberGenerator::XorShift128)
  pub fn xorshift128(_s0: &mut u64, _s1: &mut u64) -> u64 {
    unimplemented!("骨架")
  }
}
`;

const p = writeSrc("v8-numbers", "math_random_generated.rs", rust);
console.log(`written ${p}; CACHE=${cacheSize}, STATE=${stateSize}, 接口=${mustCc.length}`);
