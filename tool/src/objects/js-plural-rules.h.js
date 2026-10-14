#!/usr/bin/env bun
// tool/src/objects/js-plural-rules.h.js — JSPluralRules 族骨架 (4 槽 + Type 1bit 位域)
// 组→文件映射: js-plural-rules.h + js-plural-rules-inl.h + js-plural-rules.cc + js-plural-rules.tq
// 产物: v8-objects-js-core/src/js_plural_rules.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-plural-rules.h");
const inl = readV8("src/objects/js-plural-rules-inl.h");
const cc = readV8("src/objects/js-plural-rules.cc");
const tq = readV8("src/objects/js-plural-rules.tq");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`锚点失配: ${m}`); };
const eq = (a, e, m) => { checks++; if (a !== e) throw new Error(`锚点失配: ${m} (期望 ${e}, 实得 ${a})`); };

// ── 1. 类签名 + 字段槽 ──────────────────────────────────────────────────
must(h.includes("V8_OBJECT class JSPluralRules : public JSObject {"), "类签名");
const fields = [...h.slice(h.indexOf("V8_OBJECT class JSPluralRules"), h.indexOf("} V8_OBJECT_END;"))
  .matchAll(/TaggedMember<([^>]+)> (\w+)( V8_TQ_TYPE\([^)]*\))?;/g)].map((m) => m[2]);
eq(fields.length, 4, "TaggedMember 字段数");
must(fields.join(",") === "locale_,flags_,icu_plural_rules_,icu_number_formatter_", `字段序 ${fields}`);
const K_LOCALE = 24, K_FLAGS = 32, K_PLURAL = 40, K_FMT = 48, K_SIZE = 56;
must(h.includes("inline constexpr int JSPluralRules::kHeaderSize = sizeof(JSPluralRules);"), "kHeaderSize 外置定义");

// ── 2. Type 枚举 + TypeBit ──────────────────────────────────────────────
must(h.includes("enum class Type { CARDINAL, ORDINAL };"), "Type 枚举 (2 值)");
must(h.includes("using TypeBit = base::BitField<JSPluralRules::Type, 0, 1, uint32_t>;"), "TypeBit bit0 宽1");
must(h.includes("static_assert(TypeBit::is_valid(Type::CARDINAL));"), "CARDINAL 有效断言");
must(h.includes("static_assert(TypeBit::is_valid(Type::ORDINAL));"), "ORDINAL 有效断言");
must(h.includes("Handle<String> TypeAsString(Isolate* isolate) const;"), "TypeAsString 声明");

// ── 3. inl 位域访问器 + tq 交叉 ────────────────────────────────────────
must(inl.includes("DCHECK(TypeBit::is_valid(type));"), "inl TypeBit::is_valid");
must(inl.includes("hints = TypeBit::update(hints, type);"), "inl TypeBit::update");
must(inl.includes("return TypeBit::decode(flags());"), "inl TypeBit::decode");
must(tq.includes("type JSPluralRulesType extends int32 constexpr 'JSPluralRules::Type';"), "tq Type 映射");
must(tq.includes("bitfield struct JSPluralRulesFlags extends uint31 {"), "tq bitfield");
must(tq.includes("Type: JSPluralRulesType: 1 bit;  // \"type\" is a reserved word."), "tq 1bit (type 保留字注释)");
const tqFields = [...tq.slice(tq.indexOf("extern class JSPluralRules"), tq.indexOf("\n}", tq.indexOf("extern class JSPluralRules"))).matchAll(/^  (\w+):/gm)].map((m) => m[1]);
eq(tqFields.length, 4, "tq 字段数");

// ── 4. cc 关键锚点 (选项表 + 方法) ──────────────────────────────────────
must(cc.includes("{\"cardinal\", \"ordinal\"}"), "cc type 选项表 {cardinal, ordinal}");
must(cc.includes("JSPluralRules::TypeAsString(Isolate* isolate) const {"), "cc TypeAsString 定义");
for (const sig of ["JSPluralRules::New(", "JSPluralRules::ResolvedOptions(", "JSPluralRules::ResolvePlural(",
  "JSPluralRules::ResolvePluralRange(", "JSPluralRules::GetAvailableLocales("])
  must(cc.includes(sig), `cc 方法 ${sig}`);
must(cc.includes("JSNumberFormat::MinimumIntegerDigitsFromSkeleton") || cc.includes("MinimumIntegerDigitsFromSkeleton"),
  "复用 JSNumberFormat skeleton 解析");

// ── 5. 骨架产物 ──────────────────────────────────────────────────────────
const body = `${banner("src/objects/js-plural-rules.h.js", "src/objects/js-plural-rules.h + -inl.h + .cc + .tq", "JSPluralRules 槽位/Type 位域差集骨架")}
//! JSPluralRules: Intl.PluralRules; 两个 ICU 句柄均以 Foreign 槽承载 Managed。

/// v8: src/objects/js-plural-rules.h JSPluralRules::locale_
pub const K_LOCALE_OFFSET: usize = ${K_LOCALE};
/// v8: src/objects/js-plural-rules.h JSPluralRules::flags_ (SmiTagged<JSPluralRulesFlags>)
pub const K_FLAGS_OFFSET: usize = ${K_FLAGS};
/// v8: src/objects/js-plural-rules.h JSPluralRules::icu_plural_rules_
pub const K_ICU_PLURAL_RULES_OFFSET: usize = ${K_PLURAL};
/// v8: src/objects/js-plural-rules.h JSPluralRules::icu_number_formatter_
pub const K_ICU_NUMBER_FORMATTER_OFFSET: usize = ${K_FMT};
/// v8: src/objects/js-plural-rules.h JSPluralRules::kHeaderSize
pub const K_HEADER_SIZE: usize = ${K_SIZE};

/// v8: src/objects/js-plural-rules.h JSPluralRules::Type
#[derive(Clone, Copy, PartialEq)]
pub enum Type { Cardinal = 0, Ordinal = 1 } // TypeBit: bit 0, 宽 1
pub const K_TYPE_SHIFT: u32 = 0;
pub const K_TYPE_BITS: u32 = 1;

// 方法映射 (语义推迟, 依赖 ICU/Isolate): New / ResolvedOptions / ResolvePlural /
//   ResolvePluralRange / GetAvailableLocales / TypeAsString ("cardinal"|"ordinal")
`;

const p = writeSrc("v8-objects-js-core", "js_plural_rules.rs.skel", body);
console.log(
  `js-plural-rules.h.js: ${p} — 槽 4/4 (${K_LOCALE}/${K_FLAGS}/${K_PLURAL}/${K_FMT}, size ${K_SIZE})、Type 枚举 2 值 1bit、tq 字段 ${tqFields.length}/4、cc 方法锚 5/5、断言 ${checks} 项全部通过`,
);
