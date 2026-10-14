#!/usr/bin/env bun
// tool/src/bigint/fromstring.cc.js
// 机械提取 v8/src/bigint/fromstring.cc: 字符串解析四入口与超大数阈值分支

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/bigint/fromstring.cc");
const inlH = readV8("src/bigint/bigint-inl.h");

// 1. 入口面 (对数: 4 个 ProcessorImpl::FromString*)
const entries = [...ccSrc.matchAll(/^void ProcessorImpl::(FromString\w*)\(/gm)].map((m) => m[1]);
if (entries.join() !== "FromStringClassic,FromStringLarge,FromStringBasePowerOfTwo,FromString") {
  throw new Error(`FromString 入口漂移: ${entries.join()}`);
}

// 2. 超大数阈值分支 (< kFromStringLargeThreshold → Classic, 否则 Large)
const branchM = ccSrc.match(/accumulator->ResultLength\(\) < config::kFromStringLargeThreshold/);
if (!branchM) throw new Error("FromStringLarge 阈值分支漂移");
const largeAll = [...inlH.matchAll(/constexpr uint32_t kFromStringLargeThreshold = (\d+);/g)].map((x) => x[1]);
if (largeAll.length !== 2 || largeAll[1] !== "25") throw Error("kFromStringLargeThreshold 64 位漂移");

// 3. 幂次表 (radix 2^k 快路径) 与十进制 chunk 化
const hasPowTwo = ccSrc.includes("FromStringBasePowerOfTwo");
const hasChunk = /kRadixMaxVectorLength|chunk|Chunk/i.test(ccSrc);
if (!hasPowTwo) throw new Error("缺 2^k 快路径");

let rust = banner("src/bigint/fromstring.cc.js", "src/bigint/fromstring.cc (+bigint-inl.h config)", "字符串解析骨架");

rust += `
/// 字符串解析阈值 (原版 bigint-inl.h config; 64 位为当前目标)
pub mod from_string_config {
  /// Classic → Large 门槛 (32 位 165 / 64 位 25)
  pub const FROM_STRING_LARGE_THRESHOLD_32: u32 = ${largeAll[0]};
  pub const FROM_STRING_LARGE_THRESHOLD_64: u32 = ${largeAll[1]};

  /// 2^k 底数快路径标记 (原版 FromStringBasePowerOfTwo)
  pub const POWER_OF_TWO_FAST_PATH: bool = ${hasPowTwo};
}

/// 解析入口骨架 (语义见手写 from_string.rs)
pub mod from_string_skel {
  /// 逐步累加经典解析 (原版 FromStringClassic)
  pub fn from_string_classic(_z: &mut [u64], _acc: &mut usize) {
    unimplemented!("骨架")
  }
  /// 超大数分块解析 (原版 FromStringLarge; 结果长度 ≥ LARGE_THRESHOLD)
  pub fn from_string_large(_z: &mut [u64], _acc: &mut usize) {
    unimplemented!("骨架")
  }
  /// 2^k 底数直接按位移重排 (原版 FromStringBasePowerOfTwo)
  pub fn from_string_base_power_of_two(_z: &mut [u64], _acc: &mut usize) {
    unimplemented!("骨架")
  }
  /// 总分派 (原版 FromString; 底数是否 2^k 与长度二选一)
  pub fn from_string(_z: &mut [u64], _acc: &mut usize) {
    unimplemented!("骨架")
  }

  /// 入口对数守卫
  pub const ENTRY_POINTS: [&str; 4] = [${entries.map((e) => `"${e}"`).join(", ")}];
}
`;

const p = writeSrc("v8-bigint", "from_string_generated.rs", rust);
console.log(`written ${p}; 入口=${entries.length}, 大数阈值=${largeAll[1]}`);
