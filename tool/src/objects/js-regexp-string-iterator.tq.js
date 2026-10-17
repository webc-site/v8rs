#!/usr/bin/env bun
// tool/src/objects/js-regexp-string-iterator.tq.js — JSRegExpStringIterator tq 差集 (3 bit flags + 3 槽)
// 组→文件映射 (本脚本管辖): js-regexp-string-iterator.tq (席位表差集; .h/-inl.h 归既有 js-regexp-string-iterator.h.js, 产 v8-objects-strings/src/js_regexp_string_iterator.rs)
// 产物: v8-objects-js-core/src/js_regexp_string_iterator.rs.skel (差集锚; 落地版在 strings crate 就地修订)
import { readV8, writeSrc, banner } from "../../lib.js";

const tq = readV8("src/objects/js-regexp-string-iterator.tq");
const h = readV8("src/objects/js-regexp-string-iterator.h");
const inl = readV8("src/objects/js-regexp-string-iterator-inl.h");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`锚点失配: ${m}`); };
const eq = (a, e, m) => { checks++; if (a !== e) throw new Error(`锚点失配: ${m} (期望 ${e}, 实得 ${a})`); };

// ── 1. tq bitfield 解析 ─────────────────────────────────────────────────
must(tq.includes("@cppScope('JSRegExpStringIterator')"), "tq @cppScope");
must(tq.includes("bitfield struct JSRegExpStringIteratorFlags extends uint31 {"), "tq bitfield 声明");
const bf = [...tq.slice(tq.indexOf("bitfield struct")).matchAll(/^  (\w+): bool: 1 bit;$/gm)].map((m) => m[1]);
eq(bf.length, 3, "tq 位域字段数");
must(bf.join(",") === "done,global,unicode", `tq 位域名序 ${bf}`);
const shifts = { done: 0, global: 1, unicode: 2 };

// ── 2. tq extern class 字段 ─────────────────────────────────────────────
must(tq.includes("extern class JSRegExpStringIterator extends JSObject {"), "tq 继承 JSObject");
const cls = tq.slice(tq.indexOf("extern class JSRegExpStringIterator"));
const tqFields = [...cls.matchAll(/^  (\w+): ([^;]+);$/gm)].map((m) => [m[1], m[2].trim()]);
eq(tqFields.length, 3, "tq 字段数");
must(tqFields[0][0] === "iterating_reg_exp" && tqFields[0][1] === "JSReceiver", `tq 字段0 ${tqFields[0]}`);
must(tqFields[1][0] === "iterated_string" && tqFields[1][1] === "String", `tq 字段1 ${tqFields[1]}`);
must(tqFields[2][0] === "flags" && tqFields[2][1] === "SmiTagged<JSRegExpStringIteratorFlags>", `tq 字段2 ${tqFields[2]}`);
must(tq.includes("// The [[IteratingRegExp]] internal property."), "[[IteratingRegExp]] 注释锚");
must(tq.includes("// The [[IteratedString]] internal property."), "[[IteratedString]] 注释锚");

// ── 3. 与 .h/-inl.h 交叉核对 (h 侧细节归 js-regexp-string-iterator.h.js) ──
must(h.includes("V8_OBJECT class JSRegExpStringIterator : public JSObject {"), "h 类签名");
for (const f of ["iterating_reg_exp_", "iterated_string_", "flags_"]) must(h.includes(f), `h 字段 ${f}`);
must(inl.includes("int JSRegExpStringIterator::flags() const { return flags_.load().value(); }"), "inl flags 访问器");
must(inl.includes("flags_.store(this, Smi::From31BitPattern(value));"), "inl set_flags 31-bit pattern");
const boolAcc = [...inl.matchAll(/BOOL_ACCESSORS\(JSRegExpStringIterator, flags, (\w+), (\w+)Bit::kShift\)/g)].map((m) => `${m[1]}@${m[2]}Bit`);
eq(boolAcc.length, 3, "inl BOOL_ACCESSORS 计数");
must(boolAcc.join(",") === "done@DoneBit,global@GlobalBit,unicode@UnicodeBit", `inl 位域访问器名序 ${boolAcc}`);

// ── 4. 槽位推导 (非压缩 8B; JSObject 头 3 槽) ──────────────────────────
const K_ITERATING = 24, K_ITERATED = 32, K_FLAGS = 40, K_SIZE = 48;

// ── 5. 骨架产物 ──────────────────────────────────────────────────────────
const body = `${banner("src/objects/js-regexp-string-iterator.tq.js", "src/objects/js-regexp-string-iterator.tq + js-regexp-string-iterator.h + -inl.h", "JSRegExpStringIterator tq 位域/槽位差集骨架")}
//! tq 差集: 3 个 1-bit 标志 + 3 数据槽; .h/-inl.h 布局常量归
//! tool/src/objects/js-regexp-string-iterator.h.js (产 v8-objects-strings)。

// JSRegExpStringIteratorFlags (uint31; tq js-regexp-string-iterator.tq:6-10)
pub const K_DONE_SHIFT: u32 = ${shifts.done};
pub const K_GLOBAL_SHIFT: u32 = ${shifts.global};
pub const K_UNICODE_SHIFT: u32 = ${shifts.unicode};
pub const K_FLAG_BITS: u32 = 1; // 每域 1 bit, 共 3 bit

/// v8: src/objects/js-regexp-string-iterator.tq JSRegExpStringIterator.iterating_reg_exp ([[IteratingRegExp]])
pub const K_ITERATING_REG_EXP_OFFSET: usize = ${K_ITERATING};
/// v8: src/objects/js-regexp-string-iterator.tq JSRegExpStringIterator.iterated_string ([[IteratedString]])
pub const K_ITERATED_STRING_OFFSET: usize = ${K_ITERATED};
/// v8: src/objects/js-regexp-string-iterator.tq JSRegExpStringIterator.flags
pub const K_FLAGS_OFFSET: usize = ${K_FLAGS};
pub const K_HEADER_SIZE: usize = ${K_SIZE};
`;

const p = writeSrc("v8-objects-js-core", "js_regexp_string_iterator.rs.skel", body);
console.log(
  `js-regexp-string-iterator.tq.js: ${p} — 位域 ${bf.length}/3 (shift ${Object.values(shifts).join("/")})、tq 字段 ${tqFields.length}/3、槽位 ${K_ITERATING}/${K_ITERATED}/${K_FLAGS}/${K_SIZE}、断言 ${checks} 项全部通过`,
);
