#!/usr/bin/env bun
// tool/src/objects/regexp-match-info.h.js — 覆盖 v8/src/objects/regexp-match-info.{-inl.h,cc,tq}
// 组→文件映射 (本脚本管辖): 席位表 3 文件 (.h 布局归既有 regexp-match-info.h.js 若有; 本表无 .h)
// → v8-objects-js-async/src/regexp_match_info.rs
// 机械提取: tq 布局 (3 字段 + 变长槽) + capture 宏 (i*2 / i*2+1) + cc New/ReserveCaptures 骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const tq = readV8("src/objects/regexp-match-info.tq");
const inl = readV8("src/objects/regexp-match-info-inl.h");
const cc = readV8("src/objects/regexp-match-info.cc");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[regexp-match-info.h.js] ${m}`); };

// ── 1. tq 布局: FixedArrayBase 派生, 3 具名字段 + 变长 Smi 槽 ───────────
const cls = tq.match(/extern class RegExpMatchInfo extends FixedArrayBase \{([\s\S]*?)\n\}/);
must(cls, "tq 缺 RegExpMatchInfo");
const named = [...cls[1].matchAll(/^  (\w+): ([^;]+);$/gm)].map((m) => [m[1], m[2].trim()]);
must(named.length === 3, `tq 具名字段数 ${named.length} != 3`);
must(named.map((f) => f[0]).join(",") === "number_of_capture_registers,last_subject,last_input", "tq 具名字段序漂移");
must(named[0][1] === "Smi" && named[1][1] === "String" && named[2][1] === "Object", "tq 字段类型漂移");
must(cls[1].includes("objects[length]: Smi;"), "tq 缺变长 Smi 槽 objects[length]");

// ── 2. tq capture 宏 (start = i*2, end = i*2+1) ──────────────────────────
must(cls[1].includes("return this.objects[i * 2];"), "tq GetStartOfCapture 公式漂移 (i*2)");
must(cls[1].includes("return this.objects[i * 2 + 1];"), "tq GetEndOfCapture 公式漂移 (i*2+1)");

// ── 3. -inl.h 访问器面 (3 对 get/set + capture 读写) ─────────────────────
for (const s of [
  "int RegExpMatchInfo::number_of_capture_registers() const",
  "Tagged<String> RegExpMatchInfo::last_subject() const",
  "Tagged<Object> RegExpMatchInfo::last_input() const",
  "int RegExpMatchInfo::capture(int index) const { return get(index).value(); }",
  "void RegExpMatchInfo::set_capture(int index, int value) {",
  "set(index, Smi::FromInt(value));",
]) must(inl.includes(s), `inl 缺 ${s}`);

// ── 4. .cc: New/ReserveCaptures + 容量公式 ───────────────────────────────
must(cc.includes("DirectHandle<RegExpMatchInfo> RegExpMatchInfo::New(Isolate* isolate,"), "cc 缺 New");
must(cc.includes("DirectHandle<RegExpMatchInfo> RegExpMatchInfo::ReserveCaptures("), "cc 缺 ReserveCaptures");
must(cc.includes("int capacity = JSRegExp::RegistersForCaptureCount(capture_count);"), "cc 缺容量公式 (RegistersForCaptureCount)");
must(cc.includes("RegExpMatchInfo::CopyElements(isolate, *new_info, 0, *match_info, 0, cap);"), "cc 缺扩容拷贝锚");
must(cc.includes("result->set_number_of_capture_registers(capacity);"), "cc 缺 capacity 落位");
// JSRegExp::RegistersForCaptureCount 公式交叉 (capture_count + 1) * 2
const jsre = readV8("src/objects/js-regexp.h");
const rfc = jsre.match(/RegistersForCaptureCount\(int count\)\s*\{\s*\n\s*return \(count \+ 1\) \* 2;\s*\n\s*\}/);
must(rfc, "js-regexp.h 缺 RegistersForCaptureCount = (count + 1) * 2 公式");

let rust = banner("src/objects/regexp-match-info.h.js", "src/objects/regexp-match-info.{-inl.h,cc,tq}",
  "正则匹配结果槽: 具名字段 + 变长 capture 寄存器对");
rust += `//! RegExpMatchInfo: 正则上次匹配的捕获组边界寄存器 (FixedArrayBase 派生)。
//!
//! 布局: 3 具名头字段 + 变长 Smi 槽 (捕获组 i 的 start/end 位于 objects[i*2] / objects[i*2+1]);
//! 容量 = (capture_count + 1) * 2 (组 0 = 整体匹配)。

// ── 布局 (tq, 源码序) ──
pub const REGEXP_MATCH_INFO_FIELDS: [(&str, &str); ${named.length}] = [
`;
for (const f of named) rust += `  ("${f[0]}", "${f[1]}"),\n`;
rust += `];
/// 变长槽 (tq objects[length]: Smi; 捕获寄存器对)。
pub const REGEXP_MATCH_INFO_VAR_SLOT: &str = "objects[length]: Smi";

// ── capture 槽位公式 (tq 宏 1:1) ──
/// 捕获组 i 的 start 寄存器下标 (原版 GetStartOfCapture: i * 2)。
#[inline]
#[must_use]
pub const fn start_of_capture(i: usize) -> usize {
  i * 2
}

/// 捕获组 i 的 end 寄存器下标 (原版 GetEndOfCapture: i * 2 + 1)。
#[inline]
#[must_use]
pub const fn end_of_capture(i: usize) -> usize {
  i * 2 + 1
}

/// 所需容量 (原版 JSRegExp::RegistersForCaptureCount: (capture_count + 1) * 2)。
#[inline]
#[must_use]
pub const fn registers_for_capture_count(capture_count: usize) -> usize {
  (capture_count + 1) * 2
}

// ── .cc 签名骨架 (语义就地修订) ──
/// 原版 RegExpMatchInfo::New (cc: 分配 + MemsetTagged 清零 + 空 subject/input)。
pub fn new() {
  todo!("就地修订: regexp-match-info.cc New")
}

/// 原版 RegExpMatchInfo::ReserveCaptures (cc: 容量不足则新建并 CopyElements 平移)。
pub fn reserve_captures() {
  todo!("就地修订: regexp-match-info.cc ReserveCaptures")
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(REGEXP_MATCH_INFO_FIELDS.len() == 3);
  assert!(REGEXP_MATCH_INFO_FIELDS[0].0 == "number_of_capture_registers");
  assert!(start_of_capture(0) == 0);
  assert!(end_of_capture(0) == 1);
  assert!(start_of_capture(3) == 6);
  assert!(end_of_capture(3) == 7);
  assert!(registers_for_capture_count(2) == 6);
  assert!(registers_for_capture_count(0) == 2);
};
`;

const p = writeSrc("v8-objects-js-async", "regexp_match_info.rs", rust);
console.log(`regexp-match-info.h.js: ${p} — 布局 ${named.length} 字段 + 变长槽 + capture 公式 (i*2/i*2+1) + 容量公式, 断言 ${checks} 项全过`);
