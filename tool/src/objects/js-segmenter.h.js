#!/usr/bin/env bun
// tool/src/objects/js-segmenter.h.js — JSSegmenter 族骨架 (3 槽 + Granularity 2bit)
// 组→文件映射: js-segmenter.h + js-segmenter-inl.h + js-segmenter.cc + js-segmenter.tq
// 产物: v8-objects-js-core/src/js_segmenter.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-segmenter.h");
const inl = readV8("src/objects/js-segmenter-inl.h");
const cc = readV8("src/objects/js-segmenter.cc");
const tq = readV8("src/objects/js-segmenter.tq");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`锚点失配: ${m}`); };
const eq = (a, e, m) => { checks++; if (a !== e) throw new Error(`锚点失配: ${m} (期望 ${e}, 实得 ${a})`); };

// ── 1. 类签名 + 字段槽 ──────────────────────────────────────────────────
must(h.includes("V8_OBJECT class JSSegmenter : public JSObject {"), "类签名");
const fields = [...h.slice(h.indexOf("V8_OBJECT class JSSegmenter"), h.indexOf("} V8_OBJECT_END;"))
  .matchAll(/TaggedMember<([^>]+)> (\w+)/g)].map((m) => m[2]);
eq(fields.length, 3, "TaggedMember 字段数");
must(fields.join(",") === "locale_,icu_break_iterator_,flags_", `字段序 ${fields}`);
const K_LOCALE = 24, K_ICU = 32, K_FLAGS = 40, K_SIZE = 48;
must(h.includes("inline constexpr int JSSegmenter::kHeaderSize = sizeof(JSSegmenter);"), "kHeaderSize 外置定义");

// ── 2. Granularity 枚举 + GranularityBits ──────────────────────────────
must(h.includes("enum class Granularity {\n    GRAPHEME,  // for character-breaks\n    WORD,      // for word-breaks\n    SENTENCE   // for sentence-breaks\n  };"), "Granularity 枚举 (3 值)");
must(h.includes("using GranularityBits =\n      base::BitField<JSSegmenter::Granularity, 0, 2, uint32_t>;"), "GranularityBits bit0 宽2");
for (const g of ["GRAPHEME", "WORD", "SENTENCE"]) must(h.includes(`static_assert(GranularityBits::is_valid(Granularity::${g}));`), `${g} 有效断言`);
must(h.includes("Handle<String> GranularityAsString(Isolate* isolate) const;"), "GranularityAsString 声明");
must(h.includes("Handle<String> static GetGranularityString(Isolate* isolate,"), "static GetGranularityString 声明");

// ── 3. inl + tq 交叉 ────────────────────────────────────────────────────
must(inl.includes("DCHECK(GranularityBits::is_valid(granularity));"), "inl GranularityBits::is_valid");
must(inl.includes("hints = GranularityBits::update(hints, granularity);"), "inl GranularityBits::update");
must(tq.includes("type JSSegmenterGranularity extends int32"), "tq Granularity 映射");
must(tq.includes("bitfield struct JSSegmenterFlags extends uint31 {"), "tq bitfield");
must(tq.includes("granularity: JSSegmenterGranularity: 2 bit;"), "tq 2bit");
const tqFields = [...tq.slice(tq.indexOf("extern class JSSegmenter")).matchAll(/^  (\w+): [^;]+;/gm)].map((m) => m[1]);
eq(tqFields.length, 3, "tq 字段数");

// ── 4. cc 关键锚点 ──────────────────────────────────────────────────────
must(cc.includes("{\"grapheme\", \"word\", \"sentence\"}"), "cc granularity 选项表");
must(cc.includes("JSSegmenter::GranularityAsString(Isolate* isolate) const {"), "cc GranularityAsString 定义");
must(cc.includes("Handle<String> JSSegmenter::GetGranularityString(Isolate* isolate,"), "cc GetGranularityString 定义");
for (const sig of ["JSSegmenter::New(", "JSSegmenter::ResolvedOptions(", "JSSegmenter::GetAvailableLocales("])
  must(cc.includes(sig), `cc 方法 ${sig}`);

// ── 5. 骨架产物 ──────────────────────────────────────────────────────────
const body = `${banner("src/objects/js-segmenter.h.js", "src/objects/js-segmenter.h + -inl.h + .cc + .tq", "JSSegmenter 槽位/Granularity 位域差集骨架")}
//! JSSegmenter: Intl.Segmenter; icu_break_iterator_ 为 Foreign 槽承载
//! Managed<icu::BreakIterator>。Granularity 三值共享给 JSSegments/JSSegmentIterator。

/// v8: src/objects/js-segmenter.h JSSegmenter::locale_
pub const K_LOCALE_OFFSET: usize = ${K_LOCALE};
/// v8: src/objects/js-segmenter.h JSSegmenter::icu_break_iterator_
pub const K_ICU_BREAK_ITERATOR_OFFSET: usize = ${K_ICU};
/// v8: src/objects/js-segmenter.h JSSegmenter::flags_
pub const K_FLAGS_OFFSET: usize = ${K_FLAGS};
/// v8: src/objects/js-segmenter.h JSSegmenter::kHeaderSize
pub const K_HEADER_SIZE: usize = ${K_SIZE};

/// v8: src/objects/js-segmenter.h JSSegmenter::Granularity
#[derive(Clone, Copy, PartialEq)]
pub enum Granularity { Grapheme = 0, Word = 1, Sentence = 2 } // GranularityBits: bit 0, 宽 2
pub const K_GRANULARITY_SHIFT: u32 = 0;
pub const K_GRANULARITY_BITS: u32 = 2;

// 方法映射 (语义推迟, 依赖 ICU/Isolate): New / ResolvedOptions / GetAvailableLocales /
//   GranularityAsString / GetGranularityString ("grapheme"|"word"|"sentence")
`;

const p = writeSrc("v8-objects-js-core", "js_segmenter.rs.skel", body);
console.log(
  `js-segmenter.h.js: ${p} — 槽 3/3 (${K_LOCALE}/${K_ICU}/${K_FLAGS}, size ${K_SIZE})、Granularity 3 值 2bit、tq 字段 ${tqFields.length}/3、cc 方法锚 5/5、断言 ${checks} 项全部通过`,
);
