#!/usr/bin/env bun
// tool/src/objects/js-segment-iterator.h.js — JSSegmentIterator 族骨架 (3 类: 迭代器 + 数据对象 + WordLike)
// 组→文件映射: js-segment-iterator.h + js-segment-iterator-inl.h + js-segment-iterator.cc + js-segment-iterator.tq
// 产物: v8-objects-js-core/src/js_segment_iterator.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-segment-iterator.h");
const inl = readV8("src/objects/js-segment-iterator-inl.h");
const cc = readV8("src/objects/js-segment-iterator.cc");
const tq = readV8("src/objects/js-segment-iterator.tq");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`锚点失配: ${m}`); };
const eq = (a, e, m) => { checks++; if (a !== e) throw new Error(`锚点失配: ${m} (期望 ${e}, 实得 ${a})`); };

// ── 1. JSSegmentIterator: 类签名 + 字段槽 ──────────────────────────────
must(h.includes("V8_OBJECT class JSSegmentIterator : public JSObject {"), "迭代器类签名");
const itFields = [...h.slice(h.indexOf("V8_OBJECT class JSSegmentIterator"), h.indexOf("V8_OBJECT_END;"))
  .matchAll(/TaggedMember<(.+?)> (\w+)/g)].map((m) => m[2]);
eq(itFields.length, 3, "迭代器字段数");
must(itFields.join(",") === "icu_iterator_with_text_,raw_string_,flags_", `迭代器字段序 ${itFields}`);
must(h.includes("inline constexpr int JSSegmentIterator::kHeaderSize = sizeof(JSSegmentIterator);"), "迭代器 kHeaderSize");
must(h.includes("using GranularityBits =\n      base::BitField<JSSegmenter::Granularity, 0, 2, uint32_t>;"), "复用 GranularityBits");

// ── 2. JSSegmentDataObject + WithIsWordLike ─────────────────────────────
must(h.includes("V8_OBJECT class JSSegmentDataObject : public JSObject {"), "数据对象类签名");
must(h.includes("V8_IT_REUSE_PARENT;"), "V8_IT_REUSE_PARENT");
eq((h.match(/V8_IT_NO_AUTO_CHECKER;/g) || []).length, 2, "V8_IT_NO_AUTO_CHECKER 计数 (两类数据对象)");
const sdFields = [...h.slice(h.indexOf("V8_OBJECT class JSSegmentDataObject")).matchAll(/TaggedMember<(.+?)> (\w+);/g)].map((m) => [m[2], m[1]]);
eq(sdFields.length, 4, "两数据对象字段总数 (segment/index/input/is_word_like)");
must(sdFields[0][0] === "segment_" && sdFields[0][1] === "String", `字段0 ${sdFields[0]}`);
must(sdFields[1][0] === "index_" && sdFields[1][1] === "Number", `字段1 ${sdFields[1]}`);
must(sdFields[2][0] === "input_" && sdFields[2][1] === "String", `字段2 ${sdFields[2]}`);
must(sdFields[3][0] === "is_word_like_" && sdFields[3][1] === "Boolean", `字段3 ${sdFields[3]}`);
must(h.includes("V8_OBJECT class JSSegmentDataObjectWithIsWordLike : public JSSegmentDataObject {"), "WordLike 继承");
must(h.includes("inline constexpr int JSSegmentDataObject::kHeaderSize ="), "数据对象 kHeaderSize");
must(h.includes("inline constexpr int JSSegmentDataObjectWithIsWordLike::kHeaderSize ="), "WordLike kHeaderSize");

// ── 3. tq 交叉 (@hasSameInstanceTypeAsParent) ──────────────────────────
must(tq.includes("bitfield struct JSSegmentIteratorFlags extends uint31 {"), "tq 迭代器 bitfield");
must(tq.includes("granularity: JSSegmenterGranularity: 2 bit;"), "tq 2bit");
eq((tq.match(/@hasSameInstanceTypeAsParent/g) || []).length, 2, "tq @hasSameInstanceTypeAsParent 计数");
must(tq.includes("extern class JSSegmentDataObject extends JSObject {"), "tq 数据对象");
const tqIt = [...tq.slice(tq.indexOf("extern class JSSegmentIterator"), tq.indexOf("\n}", tq.indexOf("extern class JSSegmentIterator"))).matchAll(/^  (\w+):/gm)].map((m) => m[1]);
eq(tqIt.length, 3, "tq 迭代器字段数");

// ── 4. inl + cc 锚点 ────────────────────────────────────────────────────
must(inl.includes("JSSegmentIterator::flags()") || inl.includes("BIT_FIELD_ACCESSORS(JSSegmentIterator"), "inl flags 访问器");
for (const sig of ["JSSegmentIterator::Create(", "JSSegmentIterator::Next(", "JSSegmentIterator::GranularityAsString("])
  must(cc.includes(sig), `cc 方法 ${sig}`);

// ── 5. 槽位推导 (非压缩 8B) ────────────────────────────────────────────
const K_IT_ICU = 24, K_IT_RAW = 32, K_IT_FLAGS = 40, K_IT_SIZE = 48;
const K_SD_SEG = 24, K_SD_IDX = 32, K_SD_INPUT = 40, K_SD_SIZE = 48, K_SDW_SIZE = 56;

// ── 6. 骨架产物 ──────────────────────────────────────────────────────────
const body = `${banner("src/objects/js-segment-iterator.h.js", "src/objects/js-segment-iterator.h + -inl.h + .cc + .tq", "JSSegmentIterator + 两数据对象差集骨架")}
//! JSSegmentIterator: %SegmentIteratorPrototype%; 数据对象与父类共享实例类型
//! (@hasSameInstanceTypeAsParent), 字段仍按固定槽排布。

/// v8: src/objects/js-segment-iterator.h JSSegmentIterator::icu_iterator_with_text_
pub const K_IT_ICU_ITERATOR_WITH_TEXT_OFFSET: usize = ${K_IT_ICU};
/// v8: src/objects/js-segment-iterator.h JSSegmentIterator::raw_string_
pub const K_IT_RAW_STRING_OFFSET: usize = ${K_IT_RAW};
/// v8: src/objects/js-segment-iterator.h JSSegmentIterator::flags_
pub const K_IT_FLAGS_OFFSET: usize = ${K_IT_FLAGS};
/// v8: src/objects/js-segment-iterator.h JSSegmentIterator::kHeaderSize
pub const K_IT_HEADER_SIZE: usize = ${K_IT_SIZE};

// JSSegmentDataObject (V8_IT_REUSE_PARENT + NO_AUTO_CHECKER):
/// v8: src/objects/js-segment-iterator.h JSSegmentDataObject::segment_
pub const K_SEGMENT_OFFSET: usize = ${K_SD_SEG};
/// v8: src/objects/js-segment-iterator.h JSSegmentDataObject::index_
pub const K_INDEX_OFFSET: usize = ${K_SD_IDX};
/// v8: src/objects/js-segment-iterator.h JSSegmentDataObject::input_
pub const K_INPUT_OFFSET: usize = ${K_SD_INPUT};
pub const K_SEGMENT_DATA_OBJECT_SIZE: usize = ${K_SD_SIZE};
/// v8: src/objects/js-segment-iterator.h JSSegmentDataObjectWithIsWordLike::is_word_like_
pub const K_IS_WORD_LIKE_OFFSET: usize = ${K_SD_SIZE}; // 追加 1 槽
pub const K_SEGMENT_DATA_OBJECT_WITH_IS_WORD_LIKE_SIZE: usize = ${K_SDW_SIZE};

// 方法映射 (语义推迟): Create / Next / GranularityAsString; granularity 复用 JSSegmenter 位域
`;

const p = writeSrc("v8-objects-js-core", "js_segment_iterator.rs.skel", body);
console.log(
  `js-segment-iterator.h.js: ${p} — 迭代器槽 3/3 (size ${K_IT_SIZE})、数据对象槽 4 (size ${K_SD_SIZE}/${K_SDW_SIZE})、tq 交叉 ${tqIt.length}/3 + 2 same-instance-type、cc 方法锚 3/3、断言 ${checks} 项全部通过`,
);
