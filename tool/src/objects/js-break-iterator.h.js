#!/usr/bin/env bun
// tool/src/objects/js-break-iterator.h.js — JSV8BreakIterator 布局与签名骨架提取
// 组→文件映射 (本脚本管辖): v8/src/objects/js-break-iterator.{h,-inl.h,cc,tq}
//   → v8-objects-js-core/src/js_break_iterator.rs.skel
// 配置登记: 64 位非压缩 (kTaggedSize=8, JSObject 头 = 3 tagged = 24)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-break-iterator.h");
const inl = readV8("src/objects/js-break-iterator-inl.h");
const cc = readV8("src/objects/js-break-iterator.cc");
const tq = readV8("src/objects/js-break-iterator.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};
const has = (s, sub, f) => {
  checks++;
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 64)}"`);
};

// ── 1. tq 字段真解析 (7 字段) ─────────────────────────────────────────────
const seg = tq.slice(tq.indexOf("extern class JSV8BreakIterator"));
const tqF = [...seg.slice(0, seg.indexOf("\n}")).matchAll(/^  (\w+):\s*([^;]+);/gm)]
  .map((m) => [m[1], m[2].replace(/\s+/g, " ").trim()]);
must(tqF.length === 7, `tq 字段应 7, 实得 ${tqF.length}`);
must(JSON.stringify(tqF.map((f) => f[0])) === JSON.stringify([
  "locale", "icu_iterator_with_text", "bound_adopt_text", "bound_first",
  "bound_next", "bound_current", "bound_break_type",
]), `tq 字段序失配: ${JSON.stringify(tqF.map((f) => f[0]))}`);
must(tqF[0][1] === "String" && tqF[1][1] === "CppGCManagedBase", "tq 前两字段类型失配");
for (const f of tqF.slice(2)) must(f[1] === "Undefined|JSFunction", `tq ${f[0]} 类型失配: ${f[1]}`);

// h 侧成员逐一断言
for (const [n, _] of tqF) has(h, `TaggedMember<`, `${n}_`);
has(h, "V8_OBJECT class JSV8BreakIterator : public JSObject {", "h 类头");

// ── 2. 静态方法全集 (8) ───────────────────────────────────────────────────
for (const m of ["New(", "ResolvedOptions(", "GetAvailableLocales();", "AdoptText(",
  "Current(", "First(", "Next(", "BreakType("])
  has(h, m, `JSV8BreakIterator::${m}`);

// ── 3. inl/cc 锚点 ────────────────────────────────────────────────────────
has(inl, "JSV8BreakIterator::locale() const", "inl locale");
must((inl.match(/^DEFINE_BOUND_ACCESSORS\((\w+)\)/gm) || []).length === 5, "DEFINE_BOUND_ACCESSORS 使用应 5 处");
has(inl, "DEFINE_BOUND_ACCESSORS(bound_break_type)", "inl bound_break_type");
has(cc, "icu::BreakIterator", "cc icu::BreakIterator");

// ── 4. 64 位布局 ──────────────────────────────────────────────────────────
const T = 8;
const K_HEADER = 3 * T + tqF.length * T; // 24 + 56
must(K_HEADER === 80, `布局推导失配 ${K_HEADER}`);

const body = `${banner("src/objects/js-break-iterator.h.js", "src/objects/js-break-iterator.{h,-inl.h,cc,tq}", "JSV8BreakIterator 布局与 Intl.v8BreakIterator 签名骨架")}
//! 差集骨架: JSV8BreakIterator (icu::BreakIterator + bound 方法缓存)。
//! 配置: 64 位非压缩 (kTaggedSize=8)。ICU 分词语义面留 Developer。

/// v8: src/objects/js-break-iterator.h JSV8BreakIterator 字段布局:
${tqF.map(([n, ty], i) => `///   ${n}_ offset=${3 * T + i * T} (${ty})`).join("\n")}
pub const K_JS_V8_BREAK_ITERATOR_HEADER_SIZE: usize = ${K_HEADER};

/// v8: src/objects/js-break-iterator-inl.h JSV8BreakIterator accessor 族 (locale/
///   icu_iterator_with_text/bound_adopt_text/bound_first/bound_next/bound_current/bound_break_type)
pub struct JSV8BreakIterator { /* 7 个 TaggedMember 字段, 见上布局 */ }

/// v8: src/objects/js-break-iterator.h JSV8BreakIterator::New / ResolvedOptions /
///   AdoptText / Current / First / Next / BreakType / GetAvailableLocales (8 个静态入口)
`;

const p = writeSrc("v8-objects-js-core", "js_break_iterator.rs.skel", body);
console.log(`js-break-iterator.h.js: ${p} — 字段 ${tqF.length}/7、方法 8、布局 ${K_HEADER}、断言 ${checks} 项通过`);
