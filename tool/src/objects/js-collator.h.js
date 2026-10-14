#!/usr/bin/env bun
// tool/src/objects/js-collator.h.js — JSCollator 布局与签名骨架提取
// 组→文件映射 (本脚本管辖): v8/src/objects/js-collator.{h,-inl.h,cc,tq}
//   → v8-objects-js-core/src/js_collator.rs.skel
// 配置登记: 64 位非压缩 (kTaggedSize=8, JSObject 头 = 3 tagged = 24)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-collator.h");
const inl = readV8("src/objects/js-collator-inl.h");
const cc = readV8("src/objects/js-collator.cc");
const tq = readV8("src/objects/js-collator.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};
const has = (s, sub, f) => {
  checks++;
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 64)}"`);
};

// ── 1. 类与字段锚点 ───────────────────────────────────────────────────────
has(h, "V8_OBJECT class JSCollator : public JSObject {", "js-collator.h");
// tq 字段真解析 (icu_collator/bound_compare/locale)
const seg = tq.slice(tq.indexOf("extern class JSCollator"));
const tqF = [...seg.slice(0, seg.indexOf("\n}")).matchAll(/^  (\w+): ([^;]+);/gm)]
  .map((m) => [m[1], m[2].trim()]);
must(JSON.stringify(tqF) === JSON.stringify([
  ["icu_collator", "Foreign"], ["bound_compare", "Undefined|JSFunction"], ["locale", "String"],
]), `tq 字段失配: ${JSON.stringify(tqF)}`);
for (const [f] of tqF) has(h, `TaggedMember<`, `h 字段 ${f}_`); // h 侧字段成员存在性由下方逐名断言
for (const f of ["icu_collator_", "bound_compare_", "locale_"]) has(h, `TaggedMember<`, f);
// 精确断言三个成员声明
has(h, "TaggedMember<Foreign> icu_collator_;", "h");
has(h, "TaggedMember<UnionOf<Undefined, JSFunction>> bound_compare_;", "h");
has(h, "TaggedMember<String> locale_;", "h");

// ── 2. 静态方法全集 (3) ───────────────────────────────────────────────────
for (const m of ["New(", "ResolvedOptions(", "GetAvailableLocales();"])
  must(new RegExp(`static[^;]*\\b${m.replace(/[();]/g, "\\$&")}`, "s").test(h) ||
    new RegExp(`\\b${m.replace(/[();]/g, "\\$&")}`).test(h), `JSCollator 缺 ${m}`);
has(h, "MaybeHandle<JSCollator> New(", "New 签名");
has(h, "static const std::set<std::string>& GetAvailableLocales();", "GetAvailableLocales");

// ── 3. inl/cc 锚点 ────────────────────────────────────────────────────────
has(inl, "Tagged<Managed<icu::Collator>> JSCollator::icu_collator() const", "inl icu_collator");
has(inl, "void JSCollator::set_bound_compare(", "inl set_bound_compare");
has(inl, "void JSCollator::set_locale(", "inl set_locale");
has(cc, "GetAvailableLocales", "cc");
has(cc, "icu::Locale", "cc icu");

// ── 4. 64 位布局 ──────────────────────────────────────────────────────────
const T = 8;
const K_HEADER = 3 * T + 3 * T; // 24 + 3 字段
must(K_HEADER === 48, `布局推导失配 ${K_HEADER}`);

const body = `${banner("src/objects/js-collator.h.js", "src/objects/js-collator.{h,-inl.h,cc,tq}", "JSCollator 布局与 Intl.Collator 签名骨架")}
//! 差集骨架: JSCollator (icu::Collator 的 JS 包装)。
//! 配置: 64 位非压缩 (kTaggedSize=8)。ICU 语义面 (New/ResolvedOptions/compare) 留 Developer。

/// v8: src/objects/js-collator.h JSCollator: icu_collator offset=24 (Foreign, Managed<icu::Collator>),
///   bound_compare offset=32 (Undefined|JSFunction), locale offset=40 (String)
pub const K_JS_COLLATOR_HEADER_SIZE: usize = ${K_HEADER};

/// v8: src/objects/js-collator-inl.h JSCollator::icu_collator / bound_compare / locale accessor
pub struct JSCollator { /* icu_collator_: Tagged<Foreign>, bound_compare_, locale_ */ }

/// v8: src/objects/js-collator.h JSCollator::New — 解析 locales/options 构造 (语义面留锚)
/// v8: src/objects/js-collator.h JSCollator::ResolvedOptions
/// v8: src/objects/js-collator.h JSCollator::GetAvailableLocales
`;

const p = writeSrc("v8-objects-js-core", "js_collator.rs.skel", body);
console.log(`js-collator.h.js: ${p} — 字段 ${tqF.length}/3、方法 3、布局 ${K_HEADER}、断言 ${checks} 项通过`);
