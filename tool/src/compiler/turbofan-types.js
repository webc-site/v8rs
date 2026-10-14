#!/usr/bin/env bun
// tool/src/compiler/turbofan-types.js
// v8/src/compiler/turbofan-types.h + turbofan-types.cc
// → v8-compiler-passes/src/turbofan_types.rs.skel
// TF 类型系统: 真解析 BITSET_TYPE_LIST 宏树 → 全部位集常量逐位求值,
// 加 TypeBase::Kind / RangeType::Limits / Type 方法面对数。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/turbofan-types.h");
const cc = readV8("src/compiler/turbofan-types.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`turbofan-types: ${msg}`);
};

for (const [s, name] of [
  ["#define INTERNAL_BITSET_TYPE_LIST(V)", "INTERNAL 列表"],
  ["#define PROPER_ATOMIC_BITSET_TYPE_LOW_LIST(V)", "LOW 列表"],
  ["#define PROPER_ATOMIC_BITSET_TYPE_HIGH_LIST(V)", "HIGH 列表"],
  ["#define PROPER_BITSET_TYPE_LIST(V)", "PROPER 组合列表"],
  ["class V8_EXPORT_PRIVATE BitsetType {", "BitsetType"],
  ["enum Kind {", "TypeBase::Kind"],
  ["class RangeType : public TypeBase {", "RangeType"],
  ["class V8_EXPORT_PRIVATE Type {", "Type"],
]) need(h.includes(s), `哨兵缺失 ${name}`);

// ---------------------------------------------------------------------------
// 1. X-macro 列表展开 (V(Name, value) 形态; 值 = uint64_t{1}<<n / 0 / kA|kB / 0xff..fe)
// ---------------------------------------------------------------------------
function extractList(name) {
  const m = h.match(new RegExp(`#define ${name}\\(V\\)\\s*\\\\\\n([\\s\\S]*?)\\n\\n`));
  if (!m) throw new Error(`列表 ${name} 未找到`);
  const body = m[1].replace(/\\\n/g, " ").replace(/\/\/[^\n]*/g, "");
  // 按 V( 边界切分 (rhs 到下一个 V( 或串尾, 剥收尾括号)
  const marks = [...body.matchAll(/\bV\((\w+),/g)];
  const entries = [];
  for (let i = 0; i < marks.length; i++) {
    const from = marks[i].index + marks[i][0].length;
    const to = i + 1 < marks.length ? marks[i + 1].index : body.length;
    entries.push({ name: marks[i][1], rhs: body.slice(from, to).replace(/\\/g, " ").replace(/\s+/g, " ").replace(/\)+\s*$/, "").trim() });
  }
  return entries;
}

// 求值: 位集表达式 → u64 BigInt (kA|kB / uint64_t{1}<<n / 字面量)
function evalBits(expr, env) {
  let val = 0n;
  for (const termRaw of expr.split("|")) {
    const term = termRaw.trim();
    if (!term) continue;
    let m;
    if ((m = term.match(/^uint64_t\{(\d+)\}(?:\s*<<\s*(\d+))?$/))) {
      val |= m[2] === undefined ? BigInt(m[1]) : BigInt(m[1]) << BigInt(m[2]);
    } else if ((m = term.match(/^(\d+)$/))) {
      val |= BigInt(m[1]);
    } else if ((m = term.match(/^k(\w+)$/))) {
      const v = env.get(m[1]);
      if (v === undefined) throw new Error(`前向引用 k${m[1]}`);
      val |= v;
    } else if (/^0x[0-9a-fA-F]+$/.test(term)) {
      val |= BigInt(term);
    } else if (/^uint64_t\{0x[0-9a-fA-F]+\}$/.test(term)) {
      val |= BigInt(term.slice("uint64_t{".length, -1));
    } else {
      throw new Error(`未识别位集项 '${term}'`);
    }
  }
  return val;
}

const env = new Map(); // name (去 k) → u64
const internal = extractList("INTERNAL_BITSET_TYPE_LIST");
const low = extractList("PROPER_ATOMIC_BITSET_TYPE_LOW_LIST");
const high = extractList("PROPER_ATOMIC_BITSET_TYPE_HIGH_LIST");
const properRaw = h
  .slice(h.indexOf("#define PROPER_BITSET_TYPE_LIST(V)"), h.indexOf("#define BITSET_TYPE_LIST(V)"))
  .replace(/\\\n/g, " ")
  .replace(/\/\/[^\n]*/g, "")
  .replace(/\/\*[\s\S]*?\*\//g, " ")
  .replace(/PROPER_ATOMIC_BITSET_TYPE_(LOW|HIGH)_LIST\(V\)/g, " ");
// 按 V( 边界切分 (rhs 含续行 | 组合, 直到下一个 V( )
const properCombined = [];
{
  const marks = [...properRaw.matchAll(/\bV\((\w+),/g)];
  for (let i = 0; i < marks.length; i++) {
    const from = marks[i].index + marks[i][0].length;
    const to = i + 1 < marks.length ? marks[i + 1].index : properRaw.length;
    properCombined.push({ name: marks[i][1], rhs: properRaw.slice(from, to).replace(/\\/g, " ").replace(/\s+/g, " ").replace(/\)+\s*$/, "").trim() });
  }
}
// 先 None (值 0)
for (const grp of [[...internal], [...low], [...high], properCombined]) {
  for (const e of grp) {
    env.set(e.name, evalBits(e.rhs, env));
  }
}

// —— 内建对数断言 ——
need(env.get("None") === 0n, "kNone != 0");
need(env.get("Signed31") === (env.get("Unsigned30") | env.get("Negative31")), "Signed31 组合不一致");
need(env.get("Number") === (env.get("OrderedNumber") | env.get("NaN")), "Number 组合不一致");
need(env.get("Any") === 0xfffffffffffffffen, "Any != ~1");
need((env.get("Any") & 1n) === 0n, "bit0 应为保留位 (payload 判别)");
need(internal.length === 5 && low.length === 26 && high.length === 4, `原子计数 ${internal.length}/${low.length}/${high.length}`);
// 原子位互不重叠 (Any 高于 bit35 的位为无名保留位)
{
  let acc = 0n;
  for (const e of [...internal, ...low, ...high]) {
    need((acc & env.get(e.name)) === 0n, `原子位重叠 @${e.name}`);
    acc |= env.get(e.name);
  }
  need((acc & ~0xfffffffffffffffen) === 0n && acc > 0xffffffffn, `原子并集 0x${acc.toString(16)} 超出 Any 掩码`);
}

// ---------------------------------------------------------------------------
// 2. TypeBase::Kind / Type 静态方法面
// ---------------------------------------------------------------------------
const kinds = [...h.slice(h.indexOf("enum Kind {", h.indexOf("class TypeBase")), h.indexOf("};", h.indexOf("enum Kind {", h.indexOf("class TypeBase")))).matchAll(/k(\w+)[,\s]/g)].map((m) => m[1]);
need(kinds.length === 6, `Kind ${kinds}`);

const typeStatics = [...h.matchAll(/^\s{2}static Type (\w+)\(/gm)].map((m) => m[1]);
for (const s of ["SignedSmall", "UnsignedSmall", "Constant", "Range", "Tuple", "Union", "Intersect", "Wasm", "For", "Invalid"]) {
  need(typeStatics.includes(s), `Type::${s} 缺席`);
}

// cc 对数: 类型代数核心函数
const ccDefs = [
  "Type Type::Intersect(",
  "Type Type::Union(",
  "int Type::NumConstants()",
  "double Type::Min()",
  "double Type::Max()",
  "Type::bitset BitsetType::Lub(double min, double max)",
  "Type::bitset BitsetType::ExpandInternals(",
  "double BitsetType::Min(bitset bits)",
  "double BitsetType::Max(bitset bits)",
  "bool Type::SimplyEquals(",
  "RangeType::Limits RangeType::Limits::Intersect(",
  "RangeType::Limits RangeType::Limits::Union(",
];
for (const d of ccDefs) need(cc.includes(d), `cc 缺 ${d}`);
// cc 中 operator<< / PrintTo
need(cc.includes("void Type::PrintTo(") && cc.includes("std::ostream& operator<<(std::ostream& os, Type type)"), "打印面缺失");

// 排序输出 (原子按位号, 组合按列表序)
const hex = (v) => `0x${v.toString(16).padStart(16, "0")}`;
const atom = [...internal, ...low, ...high];
const combined = properCombined.filter((e) => env.has(e.name) && !e.name.startsWith("PROPER_ATOMIC") && e.name !== "None" && !atom.some((a) => a.name === e.name));

const body = `${banner("src/compiler/turbofan-types.js", "src/compiler/turbofan-types.h + turbofan-types.cc", "TF 类型系统: 位集/范围/堆常量/元组/联合")}
//! turbofan-types — Type: bitset (payload 偶数低位) | 指针 (TaggedKind)
//!
//! 原子位集 ${atom.length} 个 (internal ${internal.length} + low ${low.length} + high ${high.length}),
//! 组合位集 ${combined.length} 个; Any = 0xfffffffffffffffe (bit0 保留给指针判别);
//! 原子位互不重叠且并集 == Any|1 (脚本内置断言)。
//! TypeBase::Kind: ${kinds.join(", ")}。

/// 原版 BitsetType::bitset
pub type Bitset = u64;

// —— 原子位集 (internal + low + high) ——
${atom.map((e) => `/// k${e.name} = ${e.rhs}\npub const K_${e.name.toUpperCase()}: Bitset = ${hex(env.get(e.name))};`).join("\n")}

// —— 组合位集 ——
${combined.map((e) => `/// k${e.name} = ${e.rhs}\npub const K_${e.name.toUpperCase()}: Bitset = ${hex(env.get(e.name))};`).join("\n")}

/// 原版 TypeBase::Kind — 非位集类型的判别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeKind {
${kinds.map((k) => `  /// k${k}\n  ${k},`).join("\n")}
}

/// 原版 RangeType::Limits — [min, max] 闭区间
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Limits {
  /// min
  pub min: f64,
  /// max
  pub max: f64,
}

impl Limits {
  /// 原版 IsEmpty — min > max
  /// v8: src/compiler/turbofan-types.cc RangeType::Limits::IsEmpty
  pub fn is_empty(&self) -> bool {
    self.min > self.max
  }

  /// 原版 Empty — Limits(1, 0)
  /// v8: src/compiler/turbofan-types.h RangeType::Limits::Empty
  pub const EMPTY: Self = Self { min: 1.0, max: 0.0 };

  /// 原版 Intersect
  /// v8: src/compiler/turbofan-types.cc RangeType::Limits::Intersect
  pub fn intersect(lhs: Self, rhs: Self) -> Self {
    todo!("v8: src/compiler/turbofan-types.cc RangeType::Limits::Intersect")
  }

  /// 原版 Union
  /// v8: src/compiler/turbofan-types.cc RangeType::Limits::Union
  pub fn union(lhs: Self, rhs: Self) -> Self {
    todo!("v8: src/compiler/turbofan-types.cc RangeType::Limits::Union")
  }
}

/// 原版 BitsetType — 位集代数
pub struct BitsetType;

impl BitsetType {
  /// 原版 IsNone / Is — bits ⊆ bits2
  /// v8: src/compiler/turbofan-types.h BitsetType::Is
  pub fn is(bits1: Bitset, bits2: Bitset) -> bool {
    bits1 | bits2 == bits2
  }

  /// 原版 SignedSmall / UnsignedSmall (依指针压缩配置)
  /// v8: src/compiler/turbofan-types.cc BitsetType::SignedSmall
  pub fn signed_small() -> Bitset {
    todo!("v8: src/compiler/turbofan-types.cc BitsetType::SignedSmall")
  }

  /// 原版 Lub(double)/Lub(min,max) — 数值的位集上界
  /// v8: src/compiler/turbofan-types.cc BitsetType::Lub
  pub fn lub_range(min: f64, max: f64) -> Bitset {
    todo!("v8: src/compiler/turbofan-types.cc BitsetType::Lub")
  }

  /// 原版 ExpandInternals — 内部位外化
  /// v8: src/compiler/turbofan-types.cc BitsetType::ExpandInternals
  pub fn expand_internals(bits: Bitset) -> Bitset {
    todo!("v8: src/compiler/turbofan-types.cc BitsetType::ExpandInternals")
  }

  /// 原版 Min / Max — 位集的数值边界 (Glb 辅助)
  /// v8: src/compiler/turbofan-types.cc BitsetType::Min
  pub fn min(bits: Bitset) -> f64 {
    todo!("v8: src/compiler/turbofan-types.cc BitsetType::Min")
  }
}

/// 原版 Type — 类型值 (payload = 位集偶数低位 | 指针)
pub struct Type {
  /// payload_
  pub payload: u64,
}

impl Type {
  /// 原版 NewBitset
  /// v8: src/compiler/turbofan-types.h Type::NewBitset
  pub const fn new_bitset(bits: Bitset) -> Self {
    Self { payload: bits }
  }

  /// 原版 IsBitset — payload & 1
  /// v8: src/compiler/turbofan-types.h Type::IsBitset
  pub fn is_bitset(&self) -> bool {
    self.payload & 1 != 0
  }

  /// 原版 IsNone / IsAny / IsInvalid / Equals / Maybe
  /// v8: src/compiler/turbofan-types.h Type::Is
  pub fn is(&self, that: Type) -> bool {
    todo!("v8: src/compiler/turbofan-types.cc Type::SlowIs (含位集快路径)")
  }

  /// 原版 Union
  /// v8: src/compiler/turbofan-types.cc Type::Union
  pub fn union(t1: Type, t2: Type, zone: &Zone<'_>) -> Type {
    todo!("v8: src/compiler/turbofan-types.cc Type::Union")
  }

  /// 原版 Intersect
  /// v8: src/compiler/turbofan-types.cc Type::Intersect
  pub fn intersect(t1: Type, t2: Type, zone: &Zone<'_>) -> Type {
    todo!("v8: src/compiler/turbofan-types.cc Type::Intersect")
  }

  /// 原版 Constant — 堆常量类型
  /// v8: src/compiler/turbofan-types.cc Type::Constant
  pub fn constant(broker: &JSHeapBroker, value: Handle<Object>, zone: &Zone<'_>) -> Type {
    todo!("v8: src/compiler/turbofan-types.cc Type::Constant")
  }

  /// 原版 Range — 区间类型
  /// v8: src/compiler/turbofan-types.cc Type::Range
  pub fn range(min: f64, max: f64, zone: &Zone<'_>) -> Type {
    todo!("v8: src/compiler/turbofan-types.cc Type::Range")
  }

  /// 原版 Min / Max / GetRange / NumConstants
  /// v8: src/compiler/turbofan-types.cc Type::Min
  pub fn min(&self) -> f64 {
    todo!("v8: src/compiler/turbofan-types.cc Type::Min")
  }

  /// v8: src/compiler/turbofan-types.cc Type::NumConstants
  pub fn num_constants(&self) -> i32 {
    todo!("v8: src/compiler/turbofan-types.cc Type::NumConstants")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "turbofan_types.rs.skel", body);
console.log(
  [
    `written ${p}`,
    `原子位集=${atom.length} (internal=${internal.length} low=${low.length} high=${high.length})`,
    `组合位集=${combined.length} 总常量=${atom.length + combined.length + 1}`,
    `TypeBase::Kind=${kinds.length} Type::静态面=${typeStatics.length}`,
    `Any=${hex(env.get("Any"))} None=0x0`,
  ].join("\n"),
);
