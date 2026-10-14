#!/usr/bin/env bun
// tool/src/objects/oddball.h.js — 覆盖 v8/src/objects/oddball.{h,tq}
// → v8-objects-rest/src/oddball.rs
// 机械提取: kind 常量表 + .tq extern class 字段布局表 + 类层级 (V8_IT 注解)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/oddball.h");
const tqSrc = readV8("src/objects/oddball.tq");

// 1. kind 常量 (oddball.h static constexpr uint8_t)
function extractConst(name) {
  const m = hSrc.match(new RegExp(`static constexpr uint8_t ${name} = ([^;]+);`));
  if (!m) throw new Error(`未找到 Oddball::${name}`);
  let v = m[1].trim();
  if (v.startsWith("static_cast<uint8_t>(~1)")) return 0xfe; // ~1 截断到 u8
  const n = parseInt(v, 10);
  if (Number.isNaN(n)) throw new Error(`Oddball::${name} 值不可解析: ${v}`);
  return n;
}

const kFalse = extractConst("kFalse");
const kTrue = extractConst("kTrue");
const kNotBooleanMask = extractConst("kNotBooleanMask");
const kNull = extractConst("kNull");
const kUndefined = extractConst("kUndefined");

// 对数: 原版 5 常量, 语义锚 (oddball.h static_assert 同款)
if (kFalse !== 0 || kTrue !== 1) throw new Error("布尔 kind 值失配");
if (kNull !== 3) throw new Error("kNull != 3 (Internals::kNullOddballKind 锚)");
if (kUndefined !== 4) throw new Error("kUndefined != 4 (Internals::kUndefinedOddballKind 锚)");
if (kNotBooleanMask !== 0xfe) throw new Error("kNotBooleanMask 失配");

// 2. .tq extern class 布局: Oddball 5 字段; 子类 5 个全部空体 (继承布局)
function parseTqClass(name) {
  const re = new RegExp(`extern class ${name}(\\s+extends\\s+(\\w+))?\\s*\\{([\\s\\S]*?)\\n?\\}`);
  const m = tqSrc.match(re);
  if (!m) throw new Error(`未找到 .tq class ${name}`);
  const fields = [];
  for (const line of m[3].split("\n")) {
    const fm = line.match(/^\s*(\w+):\s*([^;]+);/);
    if (fm) fields.push({ name: fm[1], type: fm[2].trim() });
  }
  return { base: m[2] ?? null, fields };
}

const oddballTq = parseTqClass("Oddball");
if (oddballTq.base !== "PrimitiveHeapObject") throw new Error(`Oddball 基类失配: ${oddballTq.base}`);
if (oddballTq.fields.length !== 5) throw new Error(`Oddball 字段数 ${oddballTq.fields.length} != 5`);

// 子类清单 (oddball.tq 顺序)
const SUBCLASSES = ["Null", "Undefined", "Boolean", "True", "False"];
for (const c of SUBCLASSES) {
  const cc = parseTqClass(c);
  if (cc.fields.length !== 0) throw new Error(`${c} 应为空体继承类, 实得 ${cc.fields.length} 字段`);
}
// h 侧子类 V8_IT_REUSE_PARENT 计数对数
const reuseCount = (hSrc.match(/V8_IT_REUSE_PARENT/g) ?? []).length;
if (reuseCount !== 5) throw new Error(`V8_IT_REUSE_PARENT 计数 ${reuseCount} != 5`);

// 3. 发射
let rust = banner("src/objects/oddball.h.js", "src/objects/oddball.{h,tq}", "Oddball kind 常量 + Torque 布局字段表");
rust += `//! Oddball: true/false/null/undefined/boolean 等单例基类。

/// kind 值: false (原版 kFalse)。
pub const K_FALSE: u8 = ${kFalse};
/// kind 值: true (原版 kTrue)。
pub const K_TRUE: u8 = ${kTrue};
/// 非布尔掩码 (原版 kNotBooleanMask = ~1 截断 u8)。
pub const K_NOT_BOOLEAN_MASK: u8 = 0x${kNotBooleanMask.toString(16)};
/// kind 值: null (原版 kNull; == Internals::kNullOddballKind)。
pub const K_NULL: u8 = ${kNull};
/// kind 值: undefined (原版 kUndefined; == Internals::kUndefinedOddballKind)。
pub const K_UNDEFINED: u8 = ${kUndefined};

/// 是否布尔 kind (低 1 位可分辨)。
#[inline]
#[must_use]
pub const fn is_boolean_kind(kind: u8) -> bool {
  (kind & K_NOT_BOOLEAN_MASK) != K_NOT_BOOLEAN_MASK
}

/// Oddball Torque 布局字段表 (oddball.tq \`extern class Oddball extends PrimitiveHeapObject\`, 顺序 1:1)。
pub const ODDBALL_FIELDS: [(&str, &str); ${oddballTq.fields.length}] = [
`;
for (const f of oddballTq.fields) rust += `  ("${f.name}", "${f.type}"),\n`;
rust += `];

/// 子类层级 (oddball.tq; 全部空体继承, 实例类型复用父段)。
/// Null/Undefined/Boolean/True/False → 各自 h 侧 V8_IT_REUSE_PARENT。
pub const SUBCLASS_HIERARCHY: [(&str, &str); ${SUBCLASSES.length}] = [
  ("Null", "Oddball"),
  ("Undefined", "Oddball"),
  ("Boolean", "Oddball"),
  ("True", "Boolean"),
  ("False", "Boolean"),
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_FALSE == 0 && K_TRUE == 1);
  assert!(K_NULL == 3 && K_UNDEFINED == 4);
  assert!(K_NOT_BOOLEAN_MASK == 0xfe);
  assert!(ODDBALL_FIELDS.len() == 5);
  assert!(ODDBALL_FIELDS[0] == ("to_number_raw", "float64"));
  assert!(ODDBALL_FIELDS[4] == ("kind", "Smi"));
};
`;

const p = writeSrc("v8-objects-rest", "oddball.rs", rust);
console.log(`oddball.rs: 5 kind 常量 + 5 布局字段 + 5 子类层级, 对数通过`);
