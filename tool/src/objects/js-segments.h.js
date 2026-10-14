#!/usr/bin/env bun
// tool/src/objects/js-segments.h.js — JSSegments 族骨架 (3 槽 + granularity 2bit 复用)
// 组→文件映射: js-segments.h + js-segments-inl.h + js-segments.cc + js-segments.tq
// 产物: v8-objects-js-core/src/js_segments.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-segments.h");
const inl = readV8("src/objects/js-segments-inl.h");
const cc = readV8("src/objects/js-segments.cc");
const tq = readV8("src/objects/js-segments.tq");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`锚点失配: ${m}`); };
const eq = (a, e, m) => { checks++; if (a !== e) throw new Error(`锚点失配: ${m} (期望 ${e}, 实得 ${a})`); };

// ── 1. 类签名 + 字段槽 ──────────────────────────────────────────────────
must(h.includes("V8_OBJECT class JSSegments : public JSObject {"), "类签名");
const fields = [...h.slice(h.indexOf("V8_OBJECT class JSSegments"), h.indexOf("} V8_OBJECT_END;"))
  .matchAll(/TaggedMember<(.+?)> (\w+)/g)].map((m) => m[2]);
eq(fields.length, 3, "TaggedMember 字段数");
must(fields.join(",") === "icu_iterator_with_text_,raw_string_,flags_", `字段序 ${fields}`);
const K_ICU = 24, K_RAW = 32, K_FLAGS = 40, K_SIZE = 48;
must(h.includes("inline constexpr int JSSegments::kHeaderSize = sizeof(JSSegments);"), "kHeaderSize 外置定义");

// ── 2. GranularityBits 复用 + 类型依赖 ──────────────────────────────────
must(h.includes("using GranularityBits =\n      base::BitField<JSSegmenter::Granularity, 0, 2, uint32_t>;"), "复用 JSSegmenter::Granularity 的 Bits");
must(h.includes("#include \"src/objects/js-segmenter.h\""), "依赖 js-segmenter.h");
must(h.includes("Tagged<CppGCManaged<IcuBreakIteratorWithText>> icu_iterator_with_text()"), "CppGCManaged 访问器签名");
must(h.includes("inline void set_granularity(JSSegmenter::Granularity granularity);"), "granularity setter");

// ── 3. inl + tq 交叉 ────────────────────────────────────────────────────
must(inl.includes("inline void JSSegments::set_granularity(JSSegmenter::Granularity granularity) {"), "inl set_granularity");
must(inl.includes("inline JSSegmenter::Granularity JSSegments::granularity() const {"), "inl granularity");
must(tq.includes("bitfield struct JSSegmentsFlags extends uint31 {"), "tq bitfield");
must(tq.includes("granularity: JSSegmenterGranularity: 2 bit;"), "tq 2bit (复用 JSSegmenterGranularity)");
must(tq.includes("extern class JSSegments extends JSObject {"), "tq 类");
const tqFields = [...tq.slice(tq.indexOf("extern class JSSegments"), tq.indexOf("\n}", tq.indexOf("extern class JSSegments"))).matchAll(/^  (\w+):/gm)].map((m) => m[1]);
eq(tqFields.length, 3, "tq 字段数");

// ── 4. cc 关键锚点 ──────────────────────────────────────────────────────
for (const sig of ["JSSegments::Create(", "JSSegments::Containing(", "JSSegments::CreateSegmentDataObject(",
  "JSSegments::GranularityAsString("])
  must(cc.includes(sig), `cc 方法 ${sig}`);

// ── 5. 骨架产物 ──────────────────────────────────────────────────────────
const body = `${banner("src/objects/js-segments.h.js", "src/objects/js-segments.h + -inl.h + .cc + .tq", "JSSegments 槽位/granularity 位域差集骨架")}
//! JSSegments: Intl.Segmenter.prototype.segment 返回的迭代句柄;
//! icu_iterator_with_text_ 为 CppGCManaged<IcuBreakIteratorWithText> 槽。

/// v8: src/objects/js-segments.h JSSegments::icu_iterator_with_text_
pub const K_ICU_ITERATOR_WITH_TEXT_OFFSET: usize = ${K_ICU};
/// v8: src/objects/js-segments.h JSSegments::raw_string_
pub const K_RAW_STRING_OFFSET: usize = ${K_RAW};
/// v8: src/objects/js-segments.h JSSegments::flags_
pub const K_FLAGS_OFFSET: usize = ${K_FLAGS};
/// v8: src/objects/js-segments.h JSSegments::kHeaderSize
pub const K_HEADER_SIZE: usize = ${K_SIZE};

// GranularityBits 复用 JSSegmenter::Granularity (bit 0, 宽 2; 三值 Grapheme/Word/Sentence)
// 方法映射 (语义推迟, 依赖 ICU/Isolate): Create / Containing / CreateSegmentDataObject /
//   GranularityAsString (借 JSSegmenter::GetGranularityString)
`;

const p = writeSrc("v8-objects-js-core", "js_segments.rs.skel", body);
console.log(
  `js-segments.h.js: ${p} — 槽 3/3 (${K_ICU}/${K_RAW}/${K_FLAGS}, size ${K_SIZE})、granularity 2bit 复用、tq 字段 ${tqFields.length}/3、cc 方法锚 4/4、断言 ${checks} 项全部通过`,
);
