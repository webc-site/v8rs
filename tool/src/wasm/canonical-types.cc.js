#!/usr/bin/env bun
// tool/src/wasm/canonical-types.cc.js
// 机械提取 v8/src/wasm/canonical-types.cc 的差集内容:
// AddPredefinedTypes 的 kPredefinedArrayTypes (6) / kPredefinedSigs (25) / kReps_* 形参序列,
// 以及 TypeCanonicalizer 方法定义台账。索引常量在 canonical_types.rs (canonical-types.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/canonical-types.cc");

// ---------- 锚断言 ----------
const anchors = [
  ["include 自头", src.includes('#include "src/wasm/canonical-types.h"')],
  ["AddPredefinedTypes", src.includes("void TypeCanonicalizer::AddPredefinedTypes() {")],
  ["AddRecursiveGroup", src.includes("void TypeCanonicalizer::AddRecursiveGroup(WasmModule* module, uint32_t size) {")],
  ["数组表", src.includes("kPredefinedArrayTypes[] = {")],
  ["签名表", src.includes("kPredefinedSigs[] = {")],
];
for (const [n, ok] of anchors) {
  if (!ok) throw new Error(`锚点缺失: ${n}`);
}

// ---------- 提取预定义数组类型: {索引, 元素类型, shared} ----------
const arrayRegion = src.slice(
  src.indexOf("kPredefinedArrayTypes[] = {"),
  src.indexOf("canonical_types_.reserve(kNumberOfPredefinedTypes"),
);
const arrayTypes = [...arrayRegion.matchAll(
  /\{(kPredefinedArray\w+), \{(kWasm\w+)\}, SharedFlag\{(true|false)\}\}/g,
)].map((m) => ({ index: m[1], elem: m[2], shared: m[3] === "true" }));
if (arrayTypes.length !== 6) throw new Error(`预定义数组类型 ${arrayTypes.length} 条, 期望 6`);

// ---------- 提取 kReps_* 形参序列 ----------
const repsRegion = src.slice(src.indexOf("kReps_e_i[] = {"), src.indexOf("kPredefinedSigs[] = {"));
const repsMaps = {};
for (const m of repsRegion.matchAll(/k(Reps_\w+)\[\] = \{([^}]*)\}/g)) {
  repsMaps[m[1]] = [...m[2].matchAll(/k\w+/g)].map((s) => s[0]);
}
const repsCount = Object.keys(repsMaps).length;
if (repsCount < 20) throw new Error(`kReps_* 数组 ${repsCount} 个, 期望 >= 20`);

// ---------- 提取预定义签名: {索引, ret, param, reps} ----------
const sigRegion = src.slice(src.indexOf("kPredefinedSigs[] = {"), src.indexOf("for (auto [index, return_count, parameter_count, reps] : kPredefinedSigs)"));
const sigs = [...sigRegion.matchAll(
  /\{(kPredefinedSigIndex_\w+), (\d+), (\d+), k(Reps_\w+)\}/g,
)].map((m) => ({ index: m[1], ret: Number(m[2]), param: Number(m[3]), reps: m[4] }));
if (sigs.length !== 25) throw new Error(`预定义签名 ${sigs.length} 条, 期望 25`);
// 对数: 每个 sig 的 reps 长度 >= ret + param (C++ 侧允许前缀复用更长数组, 如 e_r 复用 e_rr)
for (const s of sigs) {
  const len = repsMaps[s.reps]?.length ?? -1;
  if (len < s.ret + s.param) {
    throw new Error(`${s.index}: reps ${s.reps} 长度 ${len} < ret+param ${s.ret + s.param}`);
  }
}
// 锚: 尾条 configureAll 0 返回 4 参数
const last = sigs[sigs.length - 1];
if (last.index !== "kPredefinedSigIndex_configureAll" || last.ret !== 0 || last.param !== 4) {
  throw new Error("configureAll 尾条锚点漂移");
}

// ---------- 方法定义台账 ----------
// 正则: 返回类型 + [ *&] 分隔 + 类名 + ::方法( ; 类名捕获须完整 (分隔符不允许 \w)
const methods = [];
let prevStatic = false;
for (const [i, line] of src.split("\n").entries()) {
  if (/^\/\/ static$/.test(line)) { prevStatic = true; continue; }
  const m = line.match(/^([A-Za-z_][\w:<>,&*]*?)[ *&]+([A-Z]\w*(?:<[^>]*>)?)::~?(\w+)\(/);
  if (m) methods.push({ cls: m[2], name: m[3], line: i + 1, isStatic: prevStatic });
  prevStatic = false;
}
const tcMethods = methods.filter((x) => x.cls === "TypeCanonicalizer");
if (tcMethods.length < 20) throw new Error(`TypeCanonicalizer 方法 ${tcMethods.length} 条, 期望 >= 20`);

console.log(
  `canonical-types.cc: 数组=${arrayTypes.length} 签名=${sigs.length} reps=${repsCount} 方法=${methods.length} (TypeCanonicalizer=${tcMethods.length})`,
);

// ---------- 生成 Rust ----------
let rust = `${banner(
  "src/wasm/canonical-types.cc.js",
  "src/wasm/canonical-types.cc",
  "预定义类型表 (数组 6 + 签名 25 + reps 序列) 与 TypeCanonicalizer 方法台账",
)}
//! 预定义 canonical 类型注册表; 索引常量见 canonical_types.rs。

/// 预定义数组类型 (${arrayTypes.length} 条: 索引 / 元素类型 / shared)
/// v8: src/wasm/canonical-types.cc TypeCanonicalizer::AddPredefinedTypes
#[rustfmt::skip]
pub static PREDEFINED_ARRAY_TYPES: &[(&str, &str, bool)] = &[
`;
for (const a of arrayTypes) {
  rust += `  ("${a.index}", "${a.elem}", ${a.shared}),\n`;
}
rust += `];

/// 预定义签名 reps 形参序列 (kReps_* 共 ${repsCount} 组; 元素为 CanonicalValueType 记号)
/// v8: src/wasm/canonical-types.cc TypeCanonicalizer::AddPredefinedTypes
#[rustfmt::skip]
pub static PREDEFINED_SIG_REPS: &[(&str, &[&str])] = &[
`;
for (const [name, list] of Object.entries(repsMaps)) {
  rust += `  ("${name}", &[${list.map((x) => `"${x}"`).join(", ")}]),\n`;
}
rust += `];

/// 预定义签名 (${sigs.length} 条: 索引 / 返回数 / 参数数 / reps 组名)
/// v8: src/wasm/canonical-types.cc TypeCanonicalizer::AddPredefinedTypes
#[rustfmt::skip]
pub static PREDEFINED_SIGS: &[(&str, usize, usize, &str)] = &[
`;
for (const s of sigs) {
  rust += `  ("${s.index}", ${s.ret}, ${s.param}, "${s.reps}"),\n`;
}
rust += `];

/// TypeCanonicalizer 方法定义台账 (.cc 侧 ${tcMethods.length} 条)
/// v8: src/wasm/canonical-types.cc
#[rustfmt::skip]
pub static TYPE_CANONICALIZER_METHODS: &[(&str, u32)] = &[
`;
for (const mth of tcMethods) {
  rust += `  ("${mth.name}", ${mth.line}),\n`;
}
rust += `];

// —— 提取不变式断言 ——
const _: () = {
    assert!(PREDEFINED_ARRAY_TYPES.len() == 6);
    assert!(PREDEFINED_SIGS.len() == 25);
    assert!(PREDEFINED_SIG_REPS.len() == ${repsCount});
    assert!(PREDEFINED_ARRAY_TYPES[0].0 == "kPredefinedArrayI8Index");
    assert!(PREDEFINED_ARRAY_TYPES[5].0 == "kPredefinedArrayFuncRefIndex");
    assert!(PREDEFINED_SIGS[0].0 == "kPredefinedSigIndex_e_i");
    assert!(PREDEFINED_SIGS[24].0 == "kPredefinedSigIndex_configureAll");
    assert!(PREDEFINED_SIGS[24].1 == 0 && PREDEFINED_SIGS[24].2 == 4);
};
`;

const p = writeSrc("v8-wasm-core", "canonical_types_cc.rs.skel", rust);
console.log(`written ${p}`);
