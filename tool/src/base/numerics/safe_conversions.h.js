#!/usr/bin/env bun
// tool/src/base/numerics/safe_conversions.h.js — 覆盖 v8/src/base/numerics/safe_conversions.h + safe_conversions_impl.h + safe_conversions_arm_impl.h
// 解析 RangeCheck 位标志类、strict_cast/saturated_cast/checked_cast 三转换、StrictNumeric 类、
// impl 侧枚举 (IntegerRepresentation/NumericRangeRepresentation) 与类型提升结构
// → v8-base/src/numerics/safe_conversions.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/numerics/safe_conversions.h");
const impl = readV8("src/base/numerics/safe_conversions_impl.h");
const arm = readV8("src/base/numerics/safe_conversions_arm_impl.h");

for (const anchor of [
  [h, "V8_BASE_NUMERICS_SAFE_CONVERSIONS_H_"],
  [h, "class StrictNumeric"],
  [h, "constexpr Dst strict_cast(Src value)"],
  [h, "constexpr Dst saturated_cast(Src value)"],
  [impl, "V8_BASE_NUMERICS_SAFE_CONVERSIONS_IMPL_H_"],
  [impl, "class RangeCheck"],
  [arm, "V8_BASE_NUMERICS_SAFE_CONVERSIONS_ARM_IMPL_H_"],
]) {
  if (!anchor[0].includes(anchor[1])) throw new Error(`anchor missing: ${anchor[1]}`);
}

// 真解析: impl 枚举
const enums = [...impl.matchAll(/enum class (\w+) \{ ([^}]+) \};/g)].map((m) => ({
  name: m[1],
  variants: m[2].split(",").map((s) => s.trim()).filter(Boolean),
}));
// 真解析: 三 cast 自由函数 + StrictNumeric 方法
const casts = [...h.matchAll(/^constexpr Dst (\w+_cast)\(Src value\) \{/gm)].map((m) => m[1]);
// 真解析: 邻域取整族 (ClampFloor/ClampCeil/ClampRound)
const rounders = [...h.matchAll(/^Dst (Clamp\w+)\(Src value\) \{/gm)].map((m) => m[1]);
// 真解析: RangeCheck 方法
const rcMethods = [...impl.matchAll(/^  constexpr bool (Is\w+)\(\) const/gm)].map((m) => m[1]);
// 真解析: impl 侧结构名
const implStructs = [...impl.matchAll(/^struct (\w+) \{/gm)].map((m) => m[1]);
// arm 特化: ssat/usat 饱和指令
const armAsm = [...arm.matchAll(/"(ssat|usat) /g)].map((m) => m[1]);

if (casts.length < 2 || enums.length < 2) throw new Error(`casts=${casts.length} enums=${enums.length}`);

const body = `${banner("src/base/numerics/safe_conversions.h.js", "src/base/numerics/safe_conversions.h + _impl.h + _arm_impl.h", "数值安全转换: strict/checked/saturated 三族 cast 与 StrictNumeric")}
//! v8 侧: cast ${casts.length} 种 ${rounders.length ? `+ 取整族 ${rounders.length}` : ""}; 枚举 ${enums.length} 个; RangeCheck 方法 ${rcMethods.length}; impl 结构 ${implStructs.length}; arm 饱和指令 ${armAsm.length} 处

/// v8: src/base/numerics/safe_conversions_impl.h enum class IntegerRepresentation
${enums.map((e) => `/// v8: enum class ${e.name} { ${e.variants.join(", ")} }
enum ${e.name.replace(/([A-Z])/g, "_$1").toLowerCase().slice(1)} { ${e.variants.map((v) => v.replace(/^k(\w)/, (_, c) => c.toUpperCase()).replace(/([A-Z])/g, "_$1").toUpperCase()).join(", ")} }`).join("\n")}

/// v8: src/base/numerics/safe_conversions_impl.h class RangeCheck
/// 双 bool 位标志 (is_underflow_, is_overflow_; 成员顺序是整型转换优化的依据, 勿动)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RangeCheck {
    /// v8: is_underflow_ (低位)
    pub is_underflow: bool,
    /// v8: is_overflow_ (高位)
    pub is_overflow: bool,
}

impl RangeCheck {
${rcMethods.map((m0) => `    /// v8: RangeCheck::${m0}()
    pub fn ${m0.replace(/^Is/, "is_").replace(/([A-Z])/g, (c) => c.toLowerCase())}(&self) -> bool { todo!() }`).join("\n")}
}

// ---- 三 cast 族 ----
/// v8: src/base/numerics/safe_conversions.h strict_cast — 溢出即 CHECK 崩溃
pub fn strict_cast<Dst, Src: Copy + Into<Dst> + PartialOrd>(value: Src) -> Dst { /* 语义: TryInto + expect */ todo!() }
/// v8: src/base/numerics/safe_conversions.h checked_cast — 溢出返回 0
pub fn checked_cast<Dst, Src: Copy>(value: Src) -> Dst { todo!() }
/// v8: src/base/numerics/safe_conversions.h saturated_cast — 溢出钳到域端点/NaN
/// SaturationDefaultLimits: NaN→quiet_NaN, Overflow→+inf, Underflow→-inf (浮点); 整型钳 MAX/MIN
pub fn saturated_cast<Dst, Src: Copy>(value: Src) -> Dst { todo!() }

// ---- 取整族 (SafeFloor/Round/Ceil 等) ----
${rounders.map((r) => `/// v8: src/base/numerics/safe_conversions.h ${r}
pub fn ${r.replace(/([a-z])([A-Z])/g, "$1_$2").toLowerCase()}<Dst>(value: f64) -> Dst { /* 语义: floor/ceil/round 后饱和转换 */ todo!() }`).join("\n")}

/// v8: src/base/numerics/safe_conversions.h class StrictNumeric<T>
/// newtype: 构造时 strict_cast 验证; operator Dst() 隐式转换 → Rust 显式 .get()
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StrictNumeric<T>(T);

impl<T> StrictNumeric<T> {
    pub fn new(value: T) -> Self { Self(value) }
    /// v8: operator Dst() — Rust 侧显式取值
    pub fn get(self) -> T { self.0 }
}

// ---- impl 侧类型提升/域关系结构 (${implStructs.length} 个) ----
${implStructs.map((s) => `/// v8: src/base/numerics/safe_conversions_impl.h struct ${s}`).join("\n")}
// 语义: DstRangeRelationToSrcRange (kMaxExponent 域包含判定); BigEnoughPromotion /
// FastIntegerArithmeticPromotion (双操作数提升选型); UnderlyingType (剥 Checked/Clamped/Strict 包装);
// IsLess/IsLessOrEqual/IsGreater/IsGreaterOrEqual (异符号安全比较) — Developer 补

// ---- arm 侧 (ssat/usat 饱和指令内联, ${armAsm.length} 处) ----
// ${[...new Set(armAsm)].join(", ")} — SaturateFastAsmOp::Do
// Rust: i32→iN 的 clamp 即一条 ssat/usat, 无需 asm 特化
`;

writeSrc("v8-base", "numerics/safe_conversions.rs.skel", body);
console.log(`safe_conversions.h.js: casts=${casts.length} rounders=${rounders.length} enums=${enums.length} rcMethods=${rcMethods.length} implStructs=${implStructs.length} armAsm=${new Set(armAsm).size}`);
