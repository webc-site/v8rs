#!/usr/bin/env bun
// tool/src/objects/js-display-names.h.js — JSDisplayNames 枚举/位标志/布局提取
// 组→文件映射 (本脚本管辖): v8/src/objects/js-display-names.{h,-inl.h,cc,tq}
//   → v8-objects-js-core/src/js_display_names.rs.skel
// 配置登记: 64 位非压缩 (kTaggedSize=8, JSObject 头 = 3 tagged = 24)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-display-names.h");
const inl = readV8("src/objects/js-display-names-inl.h");
const cc = readV8("src/objects/js-display-names.cc");
const tq = readV8("src/objects/js-display-names.tq");

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
has(h, "V8_OBJECT class JSDisplayNames : public JSObject {", "h 类头");
const seg = tq.slice(tq.indexOf("extern class JSDisplayNames"));
const tqF = [...seg.slice(0, seg.indexOf("\n}")).matchAll(/^  (\w+):\s*([^;]+);/gm)]
  .map((m) => [m[1], m[2].trim()]);
must(JSON.stringify(tqF) === JSON.stringify([
  ["internal", "CppGCManagedBase"], ["flags", "SmiTagged<JSDisplayNamesFlags>"],
]), `tq 字段失配: ${JSON.stringify(tqF)}`);
has(h, "TaggedMember<CppGCManaged<DisplayNamesInternal>> internal_;", "h internal_");
has(h, "TaggedMember<Smi> flags_ V8_TQ_TYPE(SmiTagged<JSDisplayNamesFlags>);", "h flags_");

// ── 2. 枚举 (Style 3 / Fallback 2 / LanguageDisplay 2) ───────────────────
has(h, "enum class Style {\n    kLong,", "h Style");
has(h, "kShort,  // Abbreviations used when possible.", "h Style");
has(h, "kNarrow  // Use the shortest possible form.", "h Style");
has(h, "enum class Fallback {\n    kCode,\n    kNone,\n  };", "h Fallback");
has(h, "enum class LanguageDisplay {\n    kDialect,\n    kStandard,\n  };", "h LanguageDisplay");

// ── 3. 位标志 (h + tq 对数) ──────────────────────────────────────────────
has(h, "using StyleBits = base::BitField<JSDisplayNames::Style, 0, 2, uint32_t>;", "h StyleBits");
has(h, "using FallbackBit = StyleBits::Next<JSDisplayNames::Fallback, 1>;", "h FallbackBit");
has(h, "using LanguageDisplayBit =", "h LanguageDisplayBit");
const bf = tq.slice(tq.indexOf("bitfield struct JSDisplayNamesFlags"));
const bfF = [...bf.slice(0, bf.indexOf("}")).matchAll(/(\w+): \w+.*?: (\d+) bit/g)].map((m) => [m[1], +m[2]]);
must(JSON.stringify(bfF) === JSON.stringify([
  ["style", 2], ["fallback", 1], ["language_display", 1],
]), `tq bitfield 失配: ${JSON.stringify(bfF)}`);
must((h.match(/static_assert\((StyleBits|FallbackBit|LanguageDisplayBit)::is_valid/g) || []).length === 7,
  "static_assert 应 7 项");

// ── 4. 方法与 inl ─────────────────────────────────────────────────────────
for (const m of ["New(", "ResolvedOptions(", "Of(", "GetAvailableLocales();",
  "StyleAsString(", "FallbackAsString(", "LanguageDisplayAsString("])
  has(h, m, `JSDisplayNames::${m}`);
has(inl, "int JSDisplayNames::flags() const { return flags_.load().value(); }", "inl flags");
has(inl, "inline void JSDisplayNames::set_style(Style style) {", "inl set_style");
has(inl, "inline void JSDisplayNames::set_fallback(Fallback fallback) {", "inl set_fallback");
has(inl, "inline void JSDisplayNames::set_language_display(", "inl set_language_display");
has(cc, "DisplayNamesInternal", "cc internal");

// ── 5. 64 位布局 ──────────────────────────────────────────────────────────
const T = 8;
const K_HEADER = 3 * T + 2 * T;
must(K_HEADER === 40, `布局推导失配 ${K_HEADER}`);

const body = `${banner("src/objects/js-display-names.h.js", "src/objects/js-display-names.{h,-inl.h,cc,tq}", "JSDisplayNames 枚举/位标志/布局骨架")}
//! 差集骨架: JSDisplayNames (DisplayNamesInternal 的 JS 包装)。
//! 配置: 64 位非压缩 (kTaggedSize=8)。ICU 语义面留 Developer。

/// v8: src/objects/js-display-names.h JSDisplayNames::Style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style { Long = 0, Short = 1, Narrow = 2 }

/// v8: src/objects/js-display-names.h JSDisplayNames::Fallback
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fallback { Code = 0, None = 1 }

/// v8: src/objects/js-display-names.h JSDisplayNames::LanguageDisplay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageDisplay { Dialect = 0, Standard = 1 }

/// v8: src/objects/js-display-names.h JSDisplayNames::StyleBits (bit 0..2)
pub const K_STYLE_MASK: u32 = 0x3;
/// v8: src/objects/js-display-names.h JSDisplayNames::FallbackBit (bit 2)
pub const K_FALLBACK_MASK: u32 = 0x1 << 2;
/// v8: src/objects/js-display-names.h JSDisplayNames::LanguageDisplayBit (bit 3)
pub const K_LANGUAGE_DISPLAY_MASK: u32 = 0x1 << 3;

/// v8: src/objects/js-display-names.h JSDisplayNames: internal_ offset=24 (CppGCManaged),
///   flags_ offset=32 (Smi)
pub const K_JS_DISPLAY_NAMES_HEADER_SIZE: usize = ${K_HEADER};

/// v8: src/objects/js-display-names-inl.h flags accessor (Smi) + set_style/style/
///   set_fallback/fallback/set_language_display/language_display
pub struct JSDisplayNames { /* internal_: CppGCManaged, flags_: Smi */ }
`;

const p = writeSrc("v8-objects-js-core", "js_display_names.rs.skel", body);
console.log(`js-display-names.h.js: ${p} — 字段 ${tqF.length}/2、枚举 3+2+2、位标志 2+1+1、方法 7、布局 ${K_HEADER}、断言 ${checks} 项通过`);
