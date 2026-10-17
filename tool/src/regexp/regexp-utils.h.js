#!/usr/bin/env bun
// tool/src/regexp/regexp-utils.h.js — v8/src/regexp/regexp-utils.h → v8-regexp-exec/src/regexp_utils.rs
// 规则: Utils(AllStatic) 静态方法签名全集 (8) + V8_WARN_UNUSED_RESULT/前置声明/命名空间结构对数;
//   纯逻辑面 (AdvanceStringIndex/IsMatchedCapture) 交叉核对 regexp-utils.cc 与
//   regexp-match-info.h 的实现锚 (代理区区间 / 索引公式 / -1 哨兵), Isolate 硬耦合面推迟登记
// 产物: v8-regexp-exec/src/regexp_utils.rs.skel (签名表 + 落地/推迟处置) + 落地对数核对
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const h = readV8("src/regexp/regexp-utils.h");
const cc = readV8("src/regexp/regexp-utils.cc");
const mi = readV8("src/objects/regexp-match-info.h");

let assertCount = 0;
const fail = (label, got, want) => {
  throw new Error(`对数失配: ${label} got=${JSON.stringify(got)} want=${JSON.stringify(want)}`);
};
const count = (src, re) => (src.match(re) || []).length;

// 1) 结构锚: AllStatic 类 + 命名空间 + 前置声明 ×4
if (count(h, /class Utils : public AllStatic/g) !== 1) fail("Utils AllStatic 类", count(h, /class Utils : public AllStatic/g), 1);
assertCount++;
for (const fwd of ["JSReceiver", "Object", "RegExpMatchInfo", "String"]) {
  if (!new RegExp(`class ${fwd};`).test(h)) fail(`前置声明 ${fwd}`, true, false);
  assertCount++;
}

// 2) 方法签名全集 (8 静态方法)
const METHODS = {
  GenericCaptureGetter: { re: /static Handle<String> GenericCaptureGetter\(/, warnsUnused: false },
  IsMatchedCapture: { re: /static bool IsMatchedCapture\(Tagged<RegExpMatchInfo> match_info, int capture\);/, warnsUnused: false },
  SetLastIndex: { re: /static V8_WARN_UNUSED_RESULT MaybeDirectHandle<Object> SetLastIndex\(/, warnsUnused: true },
  GetLastIndex: { re: /static V8_WARN_UNUSED_RESULT MaybeDirectHandle<Object> GetLastIndex\(/, warnsUnused: true },
  RegExpExec: { re: /static V8_WARN_UNUSED_RESULT MaybeDirectHandle<JSAny> RegExpExec\(/, warnsUnused: true },
  IsUnmodifiedRegExp: { re: /static bool IsUnmodifiedRegExp\(/, warnsUnused: false },
  AdvanceStringIndex: { re: /static uint64_t AdvanceStringIndex\(Tagged<String> string, uint64_t index,\s*bool unicode\);/, warnsUnused: false },
  SetAdvancedStringIndex: { re: /static V8_WARN_UNUSED_RESULT MaybeDirectHandle<Object> SetAdvancedStringIndex\(/, warnsUnused: true },
};
for (const [name, { re }] of Object.entries(METHODS)) {
  if (!re.test(h)) fail(`签名锚 ${name}`, true, false);
  assertCount++;
}
if (count(h, /^\s{2}static /gm) !== 8) fail("静态方法声明总数", count(h, /^\s{2}static /gm), 8);
assertCount++;
if (count(h, /V8_WARN_UNUSED_RESULT/g) !== 4) fail("V8_WARN_UNUSED_RESULT 总数", count(h, /V8_WARN_UNUSED_RESULT/g), 4);
assertCount++;

// 3) 纯逻辑面交叉锚 (regexp-utils.cc:187-203 / 42-55 + regexp-match-info.h:60-65)
for (const [src, re, want, label] of [
  [cc, /first >= 0xD800 && first <= 0xDBFF/g, 1, "代理对 lead 区间"],
  [cc, /second >= 0xDC00 && second <= 0xDFFF/g, 1, "代理对 trail 区间"],
  [cc, /return index \+ 2;/g, 1, "surrogate pair → +2"],
  [cc, /return index \+ 1;/g, 1, "BMP/非 unicode → +1"],
  [cc, /if \(capture == -1\) return false;/g, 1, "-1 哨兵"],
  [cc, /match_start != -1 && match_end != -1/g, 1, "双寄存器判命"],
  [mi, /return capture_index \* 2;/g, 1, "capture_start_index 公式"],
  [mi, /return capture_index \* 2 \+ 1;/g, 1, "capture_end_index 公式"],
]) {
  if (count(src, re) !== want) fail(label, count(src, re), want);
  assertCount++;
}

// ── 产物: 签名表骨架 ───────────────────────────────────────────────────────
const skel = `${banner(
  "src/regexp/regexp-utils.h.js",
  "src/regexp/regexp-utils.h",
  "Utils 静态方法签名表 (8) 与处置; 落地以 regexp_utils.rs 就地修订为准",
)}
//! regexp-utils.h → regexp_utils.rs 处置表 (8 方法):
//! - 落地 (纯逻辑, 参数注入):
//!   AdvanceStringIndex → advance_string_index(&[u16] 码元切片, u64, bool) → u64
//!   IsMatchedCapture → is_matched_capture(&[i32] 捕获寄存器组, i32 计数, i32) → bool
//!   (附带 capture_start/end_index 索引公式, regexp-match-info.h:60-65)
//! - 推迟 (Isolate/DirectHandle 硬耦合, 待 objects-js L3; 对齐 REG-1b 双引用惯例):
//!   GenericCaptureGetter / SetLastIndex / GetLastIndex / RegExpExec /
//!   IsUnmodifiedRegExp / SetAdvancedStringIndex
`;
writeSrc("v8-regexp-exec", "regexp_utils.rs.skel", skel);

// ── 落地对数核对 ───────────────────────────────────────────────────────────
const landed = readFileSync(join(root, "v8-regexp-exec", "src", "regexp_utils.rs"), "utf8");
const LAND_TABLE = [
  [/pub fn advance_string_index\(code_units: &\[u16\], index: u64, unicode: bool\) -> u64/g, 1, "advance_string_index 签名"],
  [/pub fn is_matched_capture\(captures: &\[i32\], register_count: i32, capture: i32\) -> bool/g, 1, "is_matched_capture 签名"],
  [/pub const fn capture_start_index\(capture: i32\) -> i32/g, 1, "capture_start_index"],
  [/pub const fn capture_end_index\(capture: i32\) -> i32/g, 1, "capture_end_index"],
  [/pub const fn capture_start_index\(capture: i32\) -> i32 \{\n  capture \* 2\n/g, 1, "start 公式落地 ×2"],
  [/pub const fn capture_end_index\(capture: i32\) -> i32 \{\n  capture \* 2 \+ 1\n/g, 1, "end 公式落地"],
  [/\(LEAD_LOW\.\.=LEAD_HIGH\)\.contains/g, 1, "lead 区间落地"],
  [/\(TRAIL_LOW\.\.=TRAIL_HIGH\)\.contains/g, 1, "trail 区间落地"],
  [/const LEAD_LOW: u16 = 0xD800;/g, 1, "LEAD_LOW=0xD800"],
  [/const LEAD_HIGH: u16 = 0xDBFF;/g, 1, "LEAD_HIGH=0xDBFF"],
  [/const TRAIL_LOW: u16 = 0xDC00;/g, 1, "TRAIL_LOW=0xDC00"],
  [/const TRAIL_HIGH: u16 = 0xDFFF;/g, 1, "TRAIL_HIGH=0xDFFF"],
  [/return index \+ 2;/g, 1, "+2 分支"],
  [/index \+ 1\b/g, 2, "返回值 +1 (尾返 + 判据 index+1<len)"],
  [/if capture == -1/g, 1, "-1 哨兵落地"],
  [/match_start != -1 && match_end != -1/g, 1, "双寄存器判命落地"],
  [/debug_assert!\(index <= K_MAX_SAFE_INTEGER_UINT64\);/g, 1, "DCHECK_LE(kMaxSafeInteger)"],
];
const missing = [];
for (const [re, want, label] of LAND_TABLE) {
  const got = count(landed, re);
  if (got !== want) missing.push(`${label}: got=${got} want=${want}`);
  assertCount++;
}
// 推迟登记注释面: 6 个硬耦合方法名逐项须出现在落地文件头注释
for (const name of ["GenericCaptureGetter", "SetLastIndex", "GetLastIndex", "RegExpExec", "IsUnmodifiedRegExp", "SetAdvancedStringIndex"]) {
  if (!landed.includes(name)) missing.push(`推迟登记锚 ${name}: 未找到`);
  assertCount++;
}
if (missing.length) throw new Error(`regexp_utils.rs 落地对数缺项 (票单):\n  ${missing.join("\n  ")}`);
// 覆盖不变式: 原版 8 = 落地 2 + 推迟 6
if (2 + 6 !== Object.keys(METHODS).length) fail("覆盖不变式 (8 方法)", 2 + 6, Object.keys(METHODS).length);
assertCount++;

console.log(
  `regexp-utils.h.js: 对数断言 ${assertCount} 条全过; 原版 Utils 8 静态方法签名全集 (V8_WARN_UNUSED_RESULT ×4) / ` +
    `纯逻辑面 cc 交叉锚 ×6 + match-info 索引公式 ×2 命中; 落地 17 锚 + 推迟登记 6 名逐项命中, 缺项 0; ` +
    `产物 v8-regexp-exec/src/regexp_utils.rs.skel`,
);
