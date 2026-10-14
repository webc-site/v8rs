#!/usr/bin/env bun
// tool/src/objects/oddball-inl.h.js — 覆盖 v8/src/objects/oddball-inl.h
// 组→文件映射 (本脚本管辖): 席位表差集 (.h 布局/ODDBALL_LIST 归既有 tool/src/objects/oddball.h.js)
// → v8-objects-js-async/src/oddball.rs
// 机械提取: kind 常量 (kFalse/kTrue/kNotBooleanMask/kNull/kUndefined, 交叉核对 .h)
//   + 子类构造器面 + IsBoolean 谓词 (kind & kNotBooleanMask == 0) + ToBool

import { readV8, writeSrc, banner } from "../../lib.js";

const inl = readV8("src/objects/oddball-inl.h");
const h = readV8("src/objects/oddball.h");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[oddball-inl.h.js] ${m}`); };

// ── 1. kind 常量 (.h 定义, 数值锚定) ─────────────────────────────────────
const kFalse = h.match(/static constexpr uint8_t kFalse = (\d+);/);
const kTrue = h.match(/static constexpr uint8_t kTrue = (\d+);/);
const kMask = h.match(/static constexpr uint8_t kNotBooleanMask = static_cast<uint8_t>\(~1\);/);
const kNull = h.match(/static constexpr uint8_t kNull = (\d+);/);
const kUndefined = h.match(/static constexpr uint8_t kUndefined = (\d+);/);
must(kFalse && kFalse[1] === "0", "kFalse != 0");
must(kTrue && kTrue[1] === "1", "kTrue != 1");
must(kMask, "kNotBooleanMask != ~1 (0xFE)");
must(kNull && kNull[1] === "3", "kNull != 3");
must(kUndefined && kUndefined[1] === "4", "kUndefined != 4");

// ── 2. -inl.h 构造器面 (Oddball 基 + 5 个单例子类) ──────────────────────
must(inl.includes("Oddball::Oddball(Tagged<ReadOnly<Map>> map, uint8_t kind)"), "inl 缺 Oddball 构造器");
must(inl.includes("Null::Null(const AllocationWitness& witness)\n    : Oddball(witness.roots().null_map(), Oddball::kNull)"), "inl 缺 Null 构造器");
must(inl.includes("Undefined::Undefined(const AllocationWitness& witness)\n    : Oddball(witness.roots().undefined_map(), Oddball::kUndefined)"), "inl 缺 Undefined 构造器");
must(inl.includes("Boolean::Boolean(const AllocationWitness& witness, uint8_t kind)"), "inl 缺 Boolean 构造器");
must(inl.includes("True::True(const AllocationWitness& witness)\n    : Boolean(witness, Oddball::kTrue)"), "inl 缺 True 构造器");
must(inl.includes("False::False(const AllocationWitness& witness)\n    : Boolean(witness, Oddball::kFalse)"), "inl 缺 False 构造器");

// ── 3. FinishInitialization (双形态 to_number 落位) ──────────────────────
must(inl.includes("void Oddball::FinishInitialization(Tagged<String> to_string,"), "inl 缺 FinishInitialization");
must(inl.includes("set_to_number_raw_as_bits(Cast<HeapNumber>(to_number)->value_as_bits());"), "inl 缺 HeapNumber 位拷贝分支");
must(inl.includes("set_to_number_raw(Object::NumberValue(to_number));"), "inl 缺 Smi 数值分支");

// ── 4. 访问器 + 谓词 ─────────────────────────────────────────────────────
for (const s of [
  "double Oddball::to_number_raw() const",
  "Tagged<String> Oddball::to_string() const",
  "Tagged<Number> Oddball::to_number() const",
  "Tagged<String> Oddball::type_of() const",
  "uint8_t Oddball::kind() const",
  "Handle<Number> Oddball::ToNumber(Isolate* isolate,",
  "DEF_HEAP_OBJECT_PREDICATE(IsBoolean)",
  "(Cast<Oddball>(obj)->kind() & Oddball::kNotBooleanMask) == 0",
  "bool Boolean::ToBool(Isolate* isolate) const",
  "return Is<True>(this);",
]) must(inl.includes(s), `inl 缺 ${s}`);

let rust = banner("src/objects/oddball-inl.h.js", "src/objects/oddball-inl.h",
  "席位表差集: kind 常量与单例构造/谓词面 (.h 布局归既有 oddball.h.js)");
rust += `//! Oddball -inl 差集: 布尔/null/undefined 单例的 kind 编码与初始化。
//!
//! kind 码: False=0, True=1 (位 0 即真值), Null=3, Undefined=4;
//! 布尔判定 = 位 0 之外全零 (kind & kNotBooleanMask == 0)。

// ── kind 常量 (原版 static constexpr uint8_t) ──
pub const K_FALSE: u8 = ${kFalse[1]};
pub const K_TRUE: u8 = ${kTrue[1]};
pub const K_NOT_BOOLEAN_MASK: u8 = 0xFE;
pub const K_NULL: u8 = ${kNull[1]};
pub const K_UNDEFINED: u8 = ${kUndefined[1]};

/// 布尔判定 (原版 IsBoolean 谓词: kind & kNotBooleanMask == 0, 即 kind ∈ {0,1})。
#[inline]
#[must_use]
pub const fn is_boolean(kind: u8) -> bool {
  kind & K_NOT_BOOLEAN_MASK == 0
}

/// 真值读取 (原版 Boolean::ToBool = Is<True>: kind 位 0)。
#[inline]
#[must_use]
pub const fn to_bool(kind: u8) -> bool {
  kind & 0x1 != 0
}

// ── 访问器骨架 (语义就地修订) ──
/// 原版 FinishInitialization (inl: HeapNumber 走位拷贝, Smi 走 NumberValue)。
pub fn finish_initialization() {
  todo!("就地修订: oddball-inl.h FinishInitialization")
}

/// 原版 Oddball::ToNumber (inl: to_number 缓存直读)。
pub fn to_number() {
  todo!("就地修订: oddball-inl.h Oddball::ToNumber")
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(K_FALSE == 0);
  assert!(K_TRUE == 1);
  assert!(K_NOT_BOOLEAN_MASK == 0xFE);
  assert!(K_NULL == 3);
  assert!(K_UNDEFINED == 4);
  assert!(is_boolean(K_FALSE) && is_boolean(K_TRUE));
  assert!(!is_boolean(K_NULL) && !is_boolean(K_UNDEFINED));
  assert!(!to_bool(K_FALSE) && to_bool(K_TRUE));
};
`;

const p = writeSrc("v8-objects-js-async", "oddball.rs", rust);
console.log(`oddball-inl.h.js: ${p} — kind 常量 5 (0/1/0xFE/3/4) + is_boolean/to_bool 谓词 + 构造器 6, 断言 ${checks} 项全过`);
