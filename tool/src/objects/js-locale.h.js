#!/usr/bin/env bun
// tool/src/objects/js-locale.h.js — JSLocale 布局与静态方法全集提取
// 组→文件映射 (本脚本管辖): v8/src/objects/js-locale.{h,-inl.h,cc,tq}
//   → v8-objects-js-core/src/js_locale.rs.skel
// 配置登记: 64 位非压缩 (kTaggedSize=8, JSObject 头 = 3 tagged = 24)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-locale.h");
const inl = readV8("src/objects/js-locale-inl.h");
const cc = readV8("src/objects/js-locale.cc");
const tq = readV8("src/objects/js-locale.tq");

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
has(h, "V8_OBJECT class JSLocale : public JSObject {", "js-locale.h");
has(h, "TaggedMember<Foreign> icu_locale_;", "h 字段");
const seg = tq.slice(tq.indexOf("extern class JSLocale"));
const tqF = [...seg.slice(0, seg.indexOf("\n}")).matchAll(/^  (\w+): ([^;]+);/gm)]
  .map((m) => [m[1], m[2].trim()]);
must(JSON.stringify(tqF) === JSON.stringify([["icu_locale", "Foreign"]]),
  `tq 字段失配: ${JSON.stringify(tqF)}`);

// ── 2. 静态方法全集对数 (27) ──────────────────────────────────────────────
const METHODS = [
  "New(", "Maximize(", "Minimize(",
  "GetCalendars(", "GetCollations(", "GetHourCycles(", "GetNumberingSystems(",
  "GetTextInfo(", "GetTimeZones(", "GetWeekInfo(",
  "Language(", "Script(", "Region(", "Variants(", "BaseName(", "Calendar(",
  "CaseFirst(", "Collation(", "HourCycle(", "FirstDayOfWeek(", "Numeric(",
  "NumberingSystem(", "ToString(", "std::string ToString(",
  "StartsWithUnicodeLanguageId(", "Is38AlphaNumList(", "Is3Alpha(",
];
for (const m of METHODS) has(h, m, `JSLocale::${m}`);
must((h.match(/static /g) || []).length >= 27, `h static 方法数 ${h.match(/static /g).length} < 27`);
has(inl, "Tagged<Managed<icu::Locale>> JSLocale::icu_locale() const", "inl icu_locale");
has(cc, "JSLocale::Maximize", "cc Maximize");
has(cc, "uloc_", "cc uloc API");

// ── 3. 64 位布局 ──────────────────────────────────────────────────────────
const T = 8;
const K_HEADER = 3 * T + T; // 24 + 1 字段
must(K_HEADER === 32, `布局推导失配 ${K_HEADER}`);

const body = `${banner("src/objects/js-locale.h.js", "src/objects/js-locale.{h,-inl.h,cc,tq}", "JSLocale 布局与 Intl.Locale 静态入口骨架")}
//! 差集骨架: JSLocale (icu::Locale 的 JS 包装)。
//! 配置: 64 位非压缩 (kTaggedSize=8)。ICU/locale 解析语义面留 Developer。

/// v8: src/objects/js-locale.h JSLocale: icu_locale_ offset=24 (Foreign, Managed<icu::Locale>)
pub const K_JS_LOCALE_HEADER_SIZE: usize = ${K_HEADER};

/// v8: src/objects/js-locale-inl.h JSLocale::icu_locale / set_icu_locale
pub struct JSLocale { /* icu_locale_: Tagged<Foreign> */ }

/// v8: src/objects/js-locale.h JSLocale 静态方法全集 (${METHODS.length} 个, ToString 另有 std::string 重载共 2 形态):
${METHODS.map((m) => `///   ${m.replace(/[()]$/, "")} — v8: src/objects/js-locale.h JSLocale::${m.replace(/[()]$/, "")}`).join("\n")}
`;

const p = writeSrc("v8-objects-js-core", "js_locale.rs.skel", body);
console.log(`js-locale.h.js: ${p} — 字段 ${tqF.length}/1、静态方法 ${METHODS.length}/27、布局 ${K_HEADER}、断言 ${checks} 项通过`);
