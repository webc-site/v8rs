#!/usr/bin/env bun
// tool/src/bigint/bigint-internal.h.js
// 机械提取 v8/src/bigint/bigint-internal.{h,cc}: 处理器中断协议/上限常量/公开算子面

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/bigint/bigint-internal.h");
const ccSrc = readV8("src/bigint/bigint-internal.cc");

// 1. 工作量阈值 (中断检查粒度, 约 10-100ms)
const workM = hSrc.match(/static const uintptr_t kWorkEstimateThreshold = (\d+);/);
if (!workM || workM[1] !== "5000000") throw new Error("kWorkEstimateThreshold 漂移");
const workThreshold = workM[1];

// 2. 位数上限 kMaxNumDigits = UINT32_MAX / kDigitBits (64 位 kDigitBits=64)
const maxM = hSrc.match(/constexpr uint32_t kMaxNumDigits = UINT32_MAX \/ kDigitBits;/);
if (!maxM) throw new Error("kMaxNumDigits 公式漂移");
const kDigitBits = 64; // 64 位目标
const maxNumDigits = Math.floor(0xffffffff / kDigitBits);

// 3. FromString 入口族 (头侧声明)
const fromStringFns = [...hSrc.matchAll(/void (FromString\w*)\(RWDigits/g)].map((m) => m[1]);
if (fromStringFns.join() !== "FromString,FromStringClassic,FromStringLarge,FromStringBasePowerOfTwo") {
  throw new Error(`FromString 族漂移: ${fromStringFns.join()}`);
}

// 4. 中断状态与 cc 侧分派面
if (!hSrc.includes("Status::kInterrupted")) throw new Error("kInterrupted 漂移");
const ccOps = [...ccSrc.matchAll(/^void ProcessorImpl::(\w+)\(/gm)].map((m) => m[1]);
if (ccOps.join() !== "Multiply,MultiplyLarge,CachedMod_MakeInverse") {
  throw new Error(`cc 分派面漂移: ${ccOps.join()}`);
}

// 5. 公开算子面 (头侧 25 项声明机械登记)
const decls = [...hSrc.matchAll(/\n  (?:void|bool|int|uint32_t|digit_t) (\w+)\(/g)].map((m) => m[1]);
if (decls.length !== 25) throw new Error(`头侧声明应 25, 实得 ${decls.length}`);

let rust = banner("src/bigint/bigint-internal.h.js", "src/bigint/bigint-internal.{h,cc}", "BigInt 处理器内部协议");

rust += `
/// BigInt 处理器内部常量 (原版 bigint-internal.h)
pub mod bigint_internal_const {
  /// 工作量阈值: 累计约一次 mul 指令计数的上限, 达到即查中断
  /// (目标粒度 10-100ms; 原版 kWorkEstimateThreshold)
  pub const WORK_ESTIMATE_THRESHOLD: usize = ${workThreshold};

  /// 位数上限 = UINT32_MAX / kDigitBits, 64 位目标 kDigitBits=64 (原版 kMaxNumDigits)
  pub const MAX_NUM_DIGITS: u32 = ${maxNumDigits};

  /// kDigitBits (64 位目标)
  pub const DIGIT_BITS: u32 = ${kDigitBits};

  /// FromString 分派族 (对数: ${fromStringFns.length})
  pub const FROM_STRING_FAMILY: [&str; ${fromStringFns.length}] = [${fromStringFns.map((f) => `"${f}"`).join(", ")}];
}

/// 处理器中断状态 (原版 Status)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
  Ok = 0,
  Interrupted = 1,
}

/// ProcessorImpl 公开算子面 (原版 bigint-internal.h 头侧声明, ${decls.length} 项, 顺序登记)
pub const PROCESSOR_DECL_METHODS: [&str; ${decls.length}] = [${decls.map((f) => `"${f}"`).join(", ")}];

/// 中断协议签名骨架 (语义见 crate 内手写 bigint_internal.rs)
pub mod bigint_internal_skel {
  /// 工作量累计: 达 WORK_ESTIMATE_THRESHOLD 清零并查平台中断 (原版 AddWorkEstimate)
  pub fn add_work_estimate(_estimate: usize) -> bool {
    unimplemented!("骨架")
  }

  /// 字符串转 BigInt 总分派 (原版 FromString; 按 radix/长度选 Classic/Large/PowerOfTwo)
  pub fn from_string(_z: &mut [u64], _accumulator: &mut usize) {
    unimplemented!("骨架")
  }
}
`;

const p = writeSrc("v8-bigint", "bigint_internal_generated.rs", rust);
console.log(`written ${p}; 阈值=${workThreshold}, 算子=${decls.length}, cc分派=${ccOps.length}`);
