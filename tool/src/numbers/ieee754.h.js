#!/usr/bin/env bun
// tool/src/numbers/ieee754.h.js
// 机械提取 v8/src/numbers/ieee754.{h,cc}: math::pow 分派骨架与决策常量
// (libm 级原语在 src/base/ieee754.h, 由 v8-base 承接, 不重复)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/numbers/ieee754.h");
const ccSrc = readV8("src/numbers/ieee754.cc");

// 对数守卫
if (!/V8_EXPORT_PRIVATE double pow\(double x, double y\);/.test(hSrc)) {
  throw new Error("ieee754.h 未找到 pow 声明");
}
const branches = [
  { k: "std::isnan(y)", d: "指数 NaN → NaN" },
  { k: "std::isinf(y) && (x == 1 || x == -1)", d: "±1 的 ±∞ 次幂 → NaN (ECMAScript 历史行为)" },
  { k: "std::isnan(x)", d: "底数 NaN → 归一为 quiet NaN" },
  { k: "y == 2", d: "x**2 → x*x (对齐优化编译器强度削减)" },
  { k: "y == 0.5", d: "x**0.5 → sqrt(x+0); x=-∞ → +∞" },
  { k: "y == 0", d: "fuzzer 抑制分支: y==0 → 1" },
];
for (const b of branches) {
  if (!ccSrc.includes(b.k)) throw new Error(`ieee754.cc 未找到分支 ${b.k}`);
}
if (!ccSrc.includes("return 42;")) throw new Error("未找到 fuzzer 哑值 42");
if (!/v8_flags\.use_std_math_pow/.test(ccSrc)) throw new Error("未找到 use_std_math_pow 分派");
if (!ccSrc.includes("base::ieee754::legacy::pow(x, y)")) throw new Error("未找到 legacy::pow 兜底");

let rust = banner("src/numbers/ieee754.h.js", "src/numbers/ieee754.{h,cc}", "math::pow 分派层骨架");

rust += `
/// math::pow 决策分支登记 (原版 ieee754.cc, 按求值序)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowBranch {
  /// 指数 NaN → quiet NaN
  ExponentNaN,
  /// |base|==1 且指数 ±∞ → NaN (ECMAScript 历史; IEEE 754-2008 规定为 1)
  BaseOneInfiniteExponent,
  /// 底数 NaN → 归一 quiet NaN (libm 区分 signaling, JS 不区分)
  BaseNaN,
  /// y==2 → x*x
  Square,
  /// y==0.5 → sqrt(x+0); x 为 -∞ 时 → +∞
  Sqrt,
  /// use_std_math_pow: 走 std::pow
  StdPow,
  /// 兜底: base::ieee754::legacy::pow
  LegacyPow,
}

/// pow 分派骨架 (语义实现见 crate 内手写 ieee754.rs)
pub mod pow_skel {
  /// 求值序分支数对数守卫
  pub const BRANCH_COUNT: usize = ${branches.length};

  /// 原版: double math::pow(double x, double y)
  /// 差异要点: (±1, ±∞) = NaN; y==2 → x*x; y==0.5 → sqrt(x+0) (+0 保证 -0**0.5 = +0)
  pub fn pow(_x: f64, _y: f64) -> f64 {
    unimplemented!("骨架")
  }

  /// correctness fuzzer 跨架构抑制分支: y==0→1, x NaN→x, 其余哑值 42
  pub const FUZZER_DUMMY: f64 = 42.0;
}
`;

const p = writeSrc("v8-numbers", "ieee754_generated.rs", rust);
console.log(`written ${p}; 分支=${branches.length}`);
