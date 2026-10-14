#!/usr/bin/env bun
// tool/src/base/numerics/clamped_math.h.js — 覆盖 v8/src/base/numerics/clamped_math.h + clamped_math_impl.h
// 解析 ClampedNumeric 类、12 个 Clamped*Op、X-macro 算子 + 比较算子展开、using 导出表
// → v8-base/src/numerics/clamped_math.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/numerics/clamped_math.h");
const impl = readV8("src/base/numerics/clamped_math_impl.h");

for (const anchor of [
  [h, "V8_BASE_NUMERICS_CLAMPED_MATH_H_"],
  [h, "class ClampedNumeric"],
  [h, "BASE_NUMERIC_ARITHMETIC_OPERATORS(Clamped, Clamp, Add, +, +=)"],
  [h, "BASE_NUMERIC_COMPARISON_OPERATORS(Clamped, IsLess, <)"],
  [impl, "V8_BASE_NUMERICS_CLAMPED_MATH_IMPL_H_"],
  [impl, "struct ClampedAddOp"],
]) {
  if (!anchor[0].includes(anchor[1])) throw new Error(`anchor missing: ${anchor[1]}`);
}

// 真解析: 12 个 Clamped*Op
const ops = [...impl.matchAll(/^struct (Clamped\w+Op) \{\};$/gm)].map((m) => m[1]);
if (ops.length !== 12) throw new Error(`expect 12 ops, got ${ops.length}`);

// 真解析: X-macro 三组
const arith = [...h.matchAll(/BASE_NUMERIC_ARITHMETIC_OPERATORS\(Clamped, Clamp, (\w+), (\S+?),/g)].map((m) => ({ name: m[1], sym: m[2] }));
const variadic = [...h.matchAll(/BASE_NUMERIC_ARITHMETIC_VARIADIC\(Clamped, Clamp, (\w+)\)/g)].map((m) => m[1]);
const comparisons = [...h.matchAll(/BASE_NUMERIC_COMPARISON_OPERATORS\(Clamped, (\w+), (\S+?)\)/g)].map((m) => ({ name: m[1], sym: m[2] }));
if (arith.length !== 10 || variadic.length !== 2 || comparisons.length !== 6) {
  throw new Error(`xmacro ${arith.length}/${variadic.length}/${comparisons.length}`);
}

// 真解析: using 导出
const usings = [...h.matchAll(/^using internal::(\w+);$/gm)].map((m) => m[1]);

const body = `${banner("src/base/numerics/clamped_math.h.js", "src/base/numerics/clamped_math.h + clamped_math_impl.h", "饱和 (钳位) 算术: ClampedNumeric 类型面")}
//! v8 侧: Clamped*Op ${ops.length} 个; 算子 X-macro ${arith.length + variadic.length}; 比较 X-macro ${comparisons.length}; using ${usings.length} 条
//! Rust 策略: 语义对应 i*.saturating_add 等; ClampedNumeric<T> 保留 newtype 包装

/// v8: src/base/numerics/clamped_math.h class ClampedNumeric<T>
/// Rust: 透明 newtype, 运算后钳位到 T 域
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClampedNumeric<T>(pub T);

impl<T> ClampedNumeric<T> {
    /// v8: RawValue() / Clamp* 系列; 语义面就地修订
    pub fn raw_value(&self) -> T where T: Copy { self.0 }
}

// ---- 12 个 Clamped*Op (impl 侧规则结构) ----
${ops.map((op) => `/// v8: src/base/numerics/clamped_math_impl.h struct ${op}
/// SaturationOp 分派 — Rust 用 saturating_* / clamp 原生替换`).join("\n")}
// 语义: Add/Sub/Mul → saturating_add/sub/mul; Div 除零与 MIN/-1 钳位;
// Mod/Lsh/Rsh/And/Or/Xor 溢出时钳到域端点; Max/Min 比较取值

// ---- 算子展开 ----
${arith.map((o) => `/// v8: BASE_NUMERIC_ARITHMETIC_OPERATORS(Clamped, Clamp, ${o.name}, ${o.sym}, ${o.sym}=)
pub fn clamp_${o.name.toLowerCase()}<T>(lhs: T, rhs: T) -> ClampedNumeric<T> { todo!() }`).join("\n")}
${variadic.map((v) => `/// v8: BASE_NUMERIC_ARITHMETIC_VARIADIC(Clamped, Clamp, ${v})
pub fn clamp_${v.toLowerCase()}<T>(lhs: T, rhs: T) -> ClampedNumeric<T> { todo!() }`).join("\n")}

// ---- 比较算子展开 (返回 ClampedNumeric<bool>, 非裸 bool) ----
${comparisons.map((c) => `/// v8: BASE_NUMERIC_COMPARISON_OPERATORS(Clamped, ${c.name}, ${c.sym})
impl PartialOrd for ClampedNumeric<T> { /* 语义: ${c.name} — 就地修订, 保留 ClampedNumeric<bool> 返回形 */ }`).join("\n")}

// ---- using 导出对账表 (v8 侧 ${usings.length} 条) ----
// ${usings.join(", ")}
`;

writeSrc("v8-base", "numerics/clamped_math.rs.skel", body);
console.log(`clamped_math.h.js: ops=${ops.length} arith=${arith.length} variadic=${variadic.length} cmp=${comparisons.length} usings=${usings.length}`);
