#!/usr/bin/env bun
// tool/src/objects/js-relative-time-format.h.js — JSRelativeTimeFormat 族骨架 (4 槽 + Numeric 1bit)
// 组→文件映射: js-relative-time-format.h + js-relative-time-format-inl.h + js-relative-time-format.cc + js-relative-time-format.tq
// 产物: v8-objects-js-core/src/js_relative_time_format.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-relative-time-format.h");
const inl = readV8("src/objects/js-relative-time-format-inl.h");
const cc = readV8("src/objects/js-relative-time-format.cc");
const tq = readV8("src/objects/js-relative-time-format.tq");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`锚点失配: ${m}`); };
const eq = (a, e, m) => { checks++; if (a !== e) throw new Error(`锚点失配: ${m} (期望 ${e}, 实得 ${a})`); };

// ── 1. 类签名 + 字段槽 ──────────────────────────────────────────────────
must(h.includes("V8_OBJECT class JSRelativeTimeFormat : public JSObject {"), "类签名");
const fields = [...h.slice(h.indexOf("V8_OBJECT class JSRelativeTimeFormat"), h.indexOf("} V8_OBJECT_END;"))
  .matchAll(/TaggedMember<([^>]+)> (\w+)/g)].map((m) => m[2]);
eq(fields.length, 4, "TaggedMember 字段数");
must(fields.join(",") === "locale_,numberingSystem_,icu_formatter_,flags_", `字段序 ${fields}`);
const K_LOCALE = 24, K_NUMBERING = 32, K_ICU = 40, K_FLAGS = 48, K_SIZE = 56;
must(h.includes("inline constexpr int JSRelativeTimeFormat::kHeaderSize ="), "kHeaderSize 外置定义");

// ── 2. Numeric 枚举 + NumericBit ────────────────────────────────────────
must(h.includes("enum class Numeric {\n    ALWAYS,  // numerical descriptions are always used (\"1 day ago\")\n    AUTO     // numerical descriptions are used only when no more specific"),
  "Numeric 枚举 (ALWAYS=0, AUTO=1)");
must(h.includes("using NumericBit =\n      base::BitField<JSRelativeTimeFormat::Numeric, 0, 1, uint32_t>;"), "NumericBit bit0 宽1");
must(h.includes("static_assert(NumericBit::is_valid(Numeric::AUTO));"), "AUTO 有效断言");
must(h.includes("Handle<String> NumericAsString(Isolate* isolate) const;"), "NumericAsString 声明");

// ── 3. inl + tq 交叉 ────────────────────────────────────────────────────
must(inl.includes("DCHECK(NumericBit::is_valid(numeric));"), "inl NumericBit::is_valid");
must(inl.includes("hints = NumericBit::update(hints, numeric);"), "inl NumericBit::update");
must(inl.includes("return NumericBit::decode(flags());"), "inl NumericBit::decode");
must(tq.includes("type JSRelativeTimeFormatNumeric extends int32"), "tq Numeric 映射");
must(tq.includes("bitfield struct JSRelativeTimeFormatFlags extends uint31 {"), "tq bitfield");
must(tq.includes("numeric: JSRelativeTimeFormatNumeric: 1 bit;"), "tq 1bit");
const tqFields = [...tq.slice(tq.indexOf("extern class JSRelativeTimeFormat")).matchAll(/^  (\w+): [^;]+;/gm)].map((m) => m[1]);
eq(tqFields.length, 4, "tq 字段数");

// ── 4. cc 关键锚点 ──────────────────────────────────────────────────────
must(cc.includes("{\"always\", \"auto\"}"), "cc numeric 选项表 {always, auto}");
must(cc.includes("JSRelativeTimeFormat::NumericAsString(Isolate* isolate) const {"), "cc NumericAsString 定义");
for (const sig of ["JSRelativeTimeFormat::New(", "JSRelativeTimeFormat::ResolvedOptions(",
  "JSRelativeTimeFormat::Format(", "JSRelativeTimeFormat::FormatToParts(", "JSRelativeTimeFormat::GetAvailableLocales("])
  must(cc.includes(sig), `cc 方法 ${sig}`);

// ── 5. 骨架产物 ──────────────────────────────────────────────────────────
const body = `${banner("src/objects/js-relative-time-format.h.js", "src/objects/js-relative-time-format.h + -inl.h + .cc + .tq", "JSRelativeTimeFormat 槽位/Numeric 位域差集骨架")}
//! JSRelativeTimeFormat: Intl.RelativeTimeFormat; icu_formatter_ 为 Foreign 槽
//! 承载 Managed<icu::RelativeDateTimeFormatter>。

/// v8: src/objects/js-relative-time-format.h JSRelativeTimeFormat::locale_
pub const K_LOCALE_OFFSET: usize = ${K_LOCALE};
/// v8: src/objects/js-relative-time-format.h JSRelativeTimeFormat::numberingSystem_
pub const K_NUMBERING_SYSTEM_OFFSET: usize = ${K_NUMBERING};
/// v8: src/objects/js-relative-time-format.h JSRelativeTimeFormat::icu_formatter_
pub const K_ICU_FORMATTER_OFFSET: usize = ${K_ICU};
/// v8: src/objects/js-relative-time-format.h JSRelativeTimeFormat::flags_
pub const K_FLAGS_OFFSET: usize = ${K_FLAGS};
/// v8: src/objects/js-relative-time-format.h JSRelativeTimeFormat::kHeaderSize
pub const K_HEADER_SIZE: usize = ${K_SIZE};

/// v8: src/objects/js-relative-time-format.h JSRelativeTimeFormat::Numeric
#[derive(Clone, Copy, PartialEq)]
pub enum Numeric { Always = 0, Auto = 1 } // NumericBit: bit 0, 宽 1
pub const K_NUMERIC_SHIFT: u32 = 0;
pub const K_NUMERIC_BITS: u32 = 1;

// 方法映射 (语义推迟, 依赖 ICU/Isolate): New / ResolvedOptions / Format / FormatToParts /
//   GetAvailableLocales / NumericAsString ("always"|"auto")
`;

const p = writeSrc("v8-objects-js-core", "js_relative_time_format.rs.skel", body);
console.log(
  `js-relative-time-format.h.js: ${p} — 槽 4/4 (${K_LOCALE}/${K_NUMBERING}/${K_ICU}/${K_FLAGS}, size ${K_SIZE})、Numeric 2 值 1bit、tq 字段 ${tqFields.length}/4、cc 方法锚 5/5、断言 ${checks} 项全部通过`,
);
