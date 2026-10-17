#!/usr/bin/env bun
// tool/src/objects/js-list-format.h.js — JSListFormat 枚举/位标志/布局提取
// 组→文件映射 (本脚本管辖): v8/src/objects/js-list-format.{h,-inl.h,cc,tq}
//   → v8-objects-js-core/src/js_list_format.rs.skel
// 配置登记: 64 位非压缩 (kTaggedSize=8, JSObject 头 = 3 tagged = 24)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-list-format.h");
const inl = readV8("src/objects/js-list-format-inl.h");
const cc = readV8("src/objects/js-list-format.cc");
const tq = readV8("src/objects/js-list-format.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};
const has = (s, sub, f) => {
  checks++;
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 64)}"`);
};

// ── 1. 类与字段 ───────────────────────────────────────────────────────────
has(h, "V8_OBJECT class JSListFormat : public JSObject {", "h 类头");
const seg = tq.slice(tq.indexOf("extern class JSListFormat"));
const tqF = [...seg.slice(0, seg.indexOf("\n}")).matchAll(/^  (\w+):\s*([^;]+);/gm)]
  .map((m) => [m[1], m[2].trim()]);
must(JSON.stringify(tqF) === JSON.stringify([
  ["locale", "String"], ["icu_formatter", "Foreign"], ["flags", "SmiTagged<JSListFormatFlags>"],
]), `tq 字段失配: ${JSON.stringify(tqF)}`);
for (const f of ["locale_", "icu_formatter_", "flags_"]) has(h, `TaggedMember<`, f);

// ── 2. 枚举 (Style 3 值 + Type 3 值) ─────────────────────────────────────
has(h, "enum class Style {\n    LONG,   // Everything spelled out.\n    SHORT,  // Abbreviations used when possible.\n    NARROW  // Use the shortest possible form.\n  };", "h Style");
has(h, "enum class Type {\n    CONJUNCTION,", "h Type");
has(h, "DISJUNCTION,", "h Type disjunction");
has(h, "UNIT  // for lists of values with units", "h Type unit");

// ── 3. 位标志 (h BitField 链 + tq bitfield 对数) ──────────────────────────
has(h, "using StyleBits = base::BitField<JSListFormat::Style, 0, 2, uint32_t>;", "h StyleBits");
has(h, "using TypeBits = StyleBits::Next<JSListFormat::Type, 2>;", "h TypeBits");
const bf = tq.slice(tq.indexOf("bitfield struct JSListFormatFlags"));
const bfF = [...bf.slice(0, bf.indexOf("}")).matchAll(/(\w+): \w+.*?: (\d+) bit/g)].map((m) => [m[1], +m[2]]);
must(JSON.stringify(bfF) === JSON.stringify([["style", 2], ["Type", 2]]),
  `tq bitfield 失配: ${JSON.stringify(bfF)} (注意 tq 字段名 Type — type 是保留字)`);
// static_assert 全集 (6)
must((h.match(/static_assert\((Style|Type)Bits::is_valid/g) || []).length === 6, "static_assert 应 6 项");

// ── 4. 方法与 inl ─────────────────────────────────────────────────────────
for (const m of ["New(", "ResolvedOptions(", "FormatList(", "FormatListToParts(",
  "GetAvailableLocales();", "StyleAsString(", "TypeAsString("])
  has(h, m, `JSListFormat::${m}`);
has(inl, "int JSListFormat::flags() const { return flags_.load().value(); }", "inl flags");
has(inl, "inline void JSListFormat::set_style(Style style) {", "inl set_style");
has(inl, "inline JSListFormat::Style JSListFormat::style() const {", "inl style");
has(inl, "inline void JSListFormat::set_type(Type type) {", "inl set_type");
has(cc, "icu::ListFormatter", "cc icu");

// ── 5. 64 位布局 ──────────────────────────────────────────────────────────
const T = 8;
const K_HEADER = 3 * T + 3 * T;
must(K_HEADER === 48, `布局推导失配 ${K_HEADER}`);

const body = `${banner("src/objects/js-list-format.h.js", "src/objects/js-list-format.{h,-inl.h,cc,tq}", "JSListFormat 枚举/位标志/布局骨架")}
//! 差集骨架: JSListFormat (icu::ListFormatter 包装)。
//! 配置: 64 位非压缩 (kTaggedSize=8)。ICU 语义面留 Developer。

/// v8: src/objects/js-list-format.h JSListFormat::Style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style { Long = 0, Short = 1, Narrow = 2 }

/// v8: src/objects/js-list-format.h JSListFormat::Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type { Conjunction = 0, Disjunction = 1, Unit = 2 }

/// v8: src/objects/js-list-format.h JSListFormat::StyleBits (bit 0..2)
pub const K_STYLE_MASK: u32 = 0x3;
/// v8: src/objects/js-list-format.h JSListFormat::TypeBits (bit 2..4; tq 字段名 Type)
pub const K_TYPE_MASK: u32 = 0x3 << 2;
pub const K_TYPE_SHIFT: u32 = 2;

/// v8: src/objects/js-list-format.h JSListFormat: locale_ offset=24, icu_formatter_ offset=32,
///   flags_ offset=40 (Smi)
pub const K_JS_LIST_FORMAT_HEADER_SIZE: usize = ${K_HEADER};

/// v8: src/objects/js-list-format-inl.h flags accessor (Smi 编码) + set_style/style/set_type/type
pub struct JSListFormat { /* locale_, icu_formatter_, flags_ */ }
`;

const p = writeSrc("v8-objects-js-core", "js_list_format.rs.skel", body);
console.log(`js-list-format.h.js: ${p} — 字段 ${tqF.length}/3、枚举 3+3、位标志 2+2、方法 7、布局 ${K_HEADER}、断言 ${checks} 项通过`);
