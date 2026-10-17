#!/usr/bin/env bun
// tool/src/base/numerics/checked_math.h.js — 覆盖 v8/src/base/numerics/checked_math.h + checked_math_impl.h
// 解析 CheckedNumeric 类方法面、12 个 Checked*Op 结构、BASE_NUMERIC_ARITHMETIC_* X-macro 展开、using 导出表
// → v8-base/src/numerics/checked_math.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/numerics/checked_math.h");
const impl = readV8("src/base/numerics/checked_math_impl.h");

for (const anchor of [
  [h, "V8_BASE_NUMERICS_CHECKED_MATH_H_"],
  [h, "class CheckedNumeric"],
  [h, "BASE_NUMERIC_ARITHMETIC_OPERATORS(Checked, Check, Add, +, +=)"],
  [h, "BASE_NUMERIC_ARITHMETIC_VARIADIC(Checked, Check, Max)"],
  [impl, "V8_BASE_NUMERICS_CHECKED_MATH_IMPL_H_"],
  [impl, "struct CheckedAddOp"],
]) {
  if (!anchor[0].includes(anchor[1])) throw new Error(`anchor missing: ${anchor[1]}`);
}

// 真解析: 12 个 Checked*Op 结构 (impl)
const ops = [...impl.matchAll(/^struct (Checked\w+Op) \{\};$/gm)].map((m) => m[1]);
if (ops.length !== 12) throw new Error(`expect 12 ops, got ${ops.length}`);

// 真解析: X-macro 行 → (op 名, 符号)
const xOps = [...h.matchAll(/BASE_NUMERIC_ARITHMETIC_OPERATORS\(Checked, Check, (\w+), (\S+?),/g)].map((m) => ({ name: m[1], sym: m[2] }));
const xVariadic = [...h.matchAll(/BASE_NUMERIC_ARITHMETIC_VARIADIC\(Checked, Check, (\w+)\)/g)].map((m) => m[1]);
if (xOps.length !== 10 || xVariadic.length !== 2) throw new Error(`xmacro ${xOps.length}/${xVariadic.length}`);

// 真解析: 类方法/自由函数签名 (checked_math.h)
const methods = [...h.matchAll(/^  (?:template <[\s\S]*?>\n\s*)?(?:friend |constexpr |V8_INLINE )*(?:[\w:<>&*]+ )?(IsValid|AssignIfValid|ValueOrDie|ValueOrDefault|Cast|Abs|Max|Min|UnsignedAbs|FastRuntimeNegate|MathOp|operator[-~+]\d?|operator\w*=)\(/gm)].map((m) => m[1]);
const freeFns = [...h.matchAll(/^constexpr (?:bool|StrictNumeric<Dst>|CheckedNumeric<[\w:<>*]+>|auto) (\w+)\(/gm)].map((m) => m[1]);

// 真解析: using 导出表
const usings = [...h.matchAll(/^using internal::(\w+);$/gm)].map((m) => m[1]);

const body = `${banner("src/base/numerics/checked_math.h.js", "src/base/numerics/checked_math.h + checked_math_impl.h", "溢出检查算术: CheckedNumeric 类型面")}
//! v8 侧: Checked*Op 结构 ${ops.length} 个; X-macro 算子 ${xOps.length + xVariadic.length} 个; using 导出 ${usings.length} 条
//! Rust 策略: 核心语义 → i*.checked_add 等原生方法; CheckedNumeric<T> 保留 Option<T> 式包装

/// v8: src/base/numerics/checked_math.h class CheckedNumeric<T>
/// state_ = (value, is_valid); Rust: (T, bool)
pub struct CheckedNumeric<T> {
    /// v8: state_ (CheckedNumericState: value_ + is_valid_)
    pub value: T,
    pub is_valid: bool,
}

impl<T> CheckedNumeric<T> {
    /// v8: IsValid() / AssignIfValid(Dst*) / ValueOrDie() / ValueOrDefault()
    /// v8 方法面 (解析): ${[...new Set(methods)].join(", ")}
    pub fn is_valid(&self) -> bool { self.is_valid }

    /// v8: ValueOrDie — 无效则 CHECK 崩溃 (Rust: expect)
    pub fn value_or_die(&self) -> T where T: Copy { /* 语义: expect 有效 */ todo!() }

    /// v8: ValueOrDefault(default)
    pub fn value_or_default(&self, default: T) -> T where T: Copy { todo!() }

    /// v8: Cast<Dst>() — 目标类型域检查后转换
    pub fn cast<Dst>(&self) -> CheckedNumeric<Dst> { todo!() }

    /// v8: Abs() / UnsignedAbs() / Max() / Min() / FastRuntimeNegate()
    /// 语义面就地修订
}

// ---- 12 个 Checked*Op (impl 侧规则结构) ----
${ops.map((op) => `/// v8: src/base/numerics/checked_math_impl.h struct ${op}
/// v8: result_type = MaxExponentPromotion<T, U>; DoSafeAdd 等分派 — Rust 用 checked_* 原生替换`).join("\n")}
// 语义: Add/Sub/Mul → checked_add/checked_sub/checked_mul; Div/Mod 除零检查;
// Lsh/Rsh 移位量界检查; And/Or/Xor 位运算仅域检查; Max/Min 比较后域检查

// ---- X-macro 算子展开 (checked_math.h 尾部) ----
${xOps.map((o) => `/// v8: BASE_NUMERIC_ARITHMETIC_OPERATORS(Checked, Check, ${o.name}, ${o.sym}, ${o.sym}=)
pub fn check_${o.name.toLowerCase()}<T>(lhs: T, rhs: T) -> CheckedNumeric<T> { todo!() }`).join("\n")}
${xVariadic.map((v) => `/// v8: BASE_NUMERIC_ARITHMETIC_VARIADIC(Checked, Check, ${v})
pub fn check_${v.toLowerCase()}<T>(lhs: T, rhs: T) -> CheckedNumeric<T> { todo!() }`).join("\n")}

// ---- 自由函数 ----
${[...new Set(freeFns)].map((f) => `/// v8: src/base/numerics/checked_math.h ${f}
pub fn ${f.replace(/^Check/, "check_").replace(/([A-Z])/g, (c, _i) => c.toLowerCase())}() { todo!() }`).join("\n")}

// ---- using 导出对账表 (v8 侧 ${usings.length} 条) ----
// ${usings.join(", ")}
`;

writeSrc("v8-base", "numerics/checked_math.rs.skel", body);
console.log(`checked_math.h.js: ops=${ops.length} xmacro=${xOps.length + xVariadic.length} methods=${new Set(methods).size} freeFns=${new Set(freeFns).size} usings=${usings.length}`);
