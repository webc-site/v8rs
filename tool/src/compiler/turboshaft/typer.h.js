#!/usr/bin/env bun
// tool/src/compiler/turboshaft/typer.h.js
// 机械提取 v8/src/compiler/turboshaft/typer.{h,cc}
// → v8-turboshaft/src/typer.rs.skel
//
// 提取面: WordOperationTyper<Bits>/FloatOperationTyper 方法面;
// Typer::TypeForRepresentation; BranchRefinements; 数组 min/max 工具。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/typer.h");
const cc = readV8("src/compiler/turboshaft/typer.cc");

// 1. 结构哨兵
for (const [re, name] of [
  [/template <size_t Bits>\s*\nstruct WordOperationTyper \{/, "WordOperationTyper<Bits>"],
  [/static_assert\(Bits == 32 \|\| Bits == 64\)/, "Bits 限定"],
  [/struct FloatOperationTyper \{/, "FloatOperationTyper"],
  [/class Typer \{/, "Typer"],
  [/static Type TypeForRepresentation\(RegisterRepresentation rep\)/, "TypeForRepresentation"],
  [/class BranchRefinements/, "BranchRefinements"],
  [/T array_min\(const std::array<T, N>& a\)/, "array_min"],
  [/T array_max\(const std::array<T, N>& a\)/, "array_max"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. WordOperationTyper 方法面
const wordStart = h.indexOf("struct WordOperationTyper");
const floatStart = h.indexOf("struct FloatOperationTyper");
const typerStart = h.indexOf("class Typer {");
const wordBody = h.slice(wordStart, floatStart);
const floatBody = h.slice(floatStart, typerStart);
const wordMethods = [...new Set([...wordBody.matchAll(/^\s{2}static (?:type_t|word_t|std::pair<(?:type_t|word_t|Type), (?:type_t|word_t|Type)>|bool) (\w+)\(/gm)].map((m) => m[1]))];
const floatMethods = [...new Set([...floatBody.matchAll(/^\s{2}static (?:type_t|float_t|std::optional<type_t>|bool|std::pair<(?:type_t|Type), (?:type_t|Type)>) (\w+)\(/gm)].map((m) => m[1]))];
if (wordMethods.length < 12) throw new Error(`Word 方法 ${wordMethods.length} < 12`);

// 3. Restriction 面
const restrictions = [...new Set([...h.matchAll(/RestrictionFor(\w+)\(/g)].map((m) => m[1]))];
if (restrictions.length !== 8) throw new Error(`Restriction ${restrictions.length} != 8`);

// 4. cc: RefineTypes 的 ComparisonOp kind 面
const cmpKinds = [...cc.matchAll(/ComparisonOp::Kind::k(\w+)/g)].map((m) => m[1]);
const uniqueCmp = [...new Set(cmpKinds)];
if (uniqueCmp.length !== 5) throw new Error(`比较 kind ${uniqueCmp.length} != 5`);

let rust = banner(
  "src/compiler/turboshaft/typer.h.js",
  "src/compiler/turboshaft/typer.{h,cc}",
  "类型器: 数值类型代数与分支精化",
);

rust += `
/// 原版 WordOperationTyper<Bits> (Bits ∈ {32,64}) — 整数类型代数。
/// 集合表示上限 kMaxSetSize=8 (types.h 侧常量), 超出转 Range
/// (可回绕: 元素跨度 > max/2 时构造回绕区间)。
/// 方法面 (${wordMethods.length} 个):
${wordMethods.map((m) => `/// - ${m}`).join("\n")}
/// - FromElements: 排序去重, <= kMaxSetSize 走 Set 否则 MakeRange
/// - MakeRange(区间贪心): 找最大空隙构造回绕/非回绕区间
/// - distance/is_wrapping: 区间距离与回绕判定
/// - Add/Subtract: 区间算术
/// - RestrictionForUnsignedLessThan{,_OrEqual}{_True,_False}: 分支收窄
/// - WidenMaximal (一步到极值) / WidenExponential (指数放宽)
/// v8: src/compiler/turboshaft/typer.h WordOperationTyper
pub struct WordOperationTyper<const BITS: u32>;

/// 原版 FloatOperationTyper — 浮点类型代数 (NaN/-0 特殊值位集)。
/// 方法面 (${floatMethods.length} 个):
${floatMethods.map((m) => `/// - ${m}`).join("\n")}
/// v8: src/compiler/turboshaft/typer.h FloatOperationTyper
pub struct FloatOperationTyper;

/// 原版 Typer — 静态入口
/// v8: typer.h Typer::TypeForRepresentation
pub fn type_for_representation(rep: RegisterRepresentation) -> Type {
  // Word32/Word64/Float32/Float64 -> Any; Tagged/Compressed/Simd128/256 -> Any
  todo!()
}

/// 原版 Typer::BranchRefinements (cc) — 按 ComparisonOp 精化分支类型。
/// kind 面 (${uniqueCmp.length} 个): ${uniqueCmp.join(" / ")}
/// (kEqual 原版未实现直接 return; Word32 仅实现无符号比较;
///  then/else 各调 RestrictionFor*)
/// v8: typer.cc Typer::BranchRefinements::RefineTypes
pub struct BranchRefinements;

/// 原版 array_min/array_max — 忽略 NaN, -0 规整为 +0
/// v8: typer.h array_min / array_max
pub fn array_min_float(a: &[f64]) -> f64 { todo!() }
pub fn array_max_float(a: &[f64]) -> f64 { todo!() }
`;

const p = writeSrc("v8-turboshaft", "typer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`Word方法=${wordMethods.length} Float方法=${floatMethods.length} Restriction=${restrictions.length} 比较kind=${uniqueCmp.length}`);
