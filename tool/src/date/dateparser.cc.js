#!/usr/bin/env bun
// tool/src/date/dateparser.cc.js — dateparser.cc 关键字表对数断言 (只读校验, 不写 src/)
// 断言 v8-date/src/dateparser/tables.rs 的 KEYWORD_TABLE 与 v8 原表逐条一致
import { readV8, root } from "../../lib.js";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const cc = readV8("src/date/dateparser.cc");
const tablesRs = readFileSync(join(root, "v8-date/src/dateparser/tables.rs"), "utf8");

// ---- 1. 提取 v8 KeywordTable::array ----
const arrayMatch = cc.match(/KeywordTable::array\[\]\[[^\]]+\]\s*=\s*\{([\s\S]*?)\};/);
if (!arrayMatch) throw new Error("dateparser.cc: KeywordTable::array 未找到");

const entryRe = /\{\s*'([^']*)'\s*,\s*'([^']*)'\s*,\s*'([^']*)'\s*,\s*DateParser::([A-Z_]+)\s*,\s*(-?\d+)\s*\}/g;
const v8Entries = [];
let m;
while ((m = entryRe.exec(arrayMatch[1]))) {
  v8Entries.push({ pre: [m[1], m[2], m[3]], type: m[4], value: parseInt(m[5], 10) });
}

// ---- 2. 提取 Rust KEYWORD_TABLE ----
const rsBlock = tablesRs.match(/pub const KEYWORD_TABLE: \[KeywordEntry; \d+\] = \[([\s\S]*?)\];/);
if (!rsBlock) throw new Error("tables.rs: KEYWORD_TABLE 未找到");
const rsEntryRe = /prefix: \*b"(.*)",\s*kw_type: KeywordType::(\w+),\s*value: (-?\d+)/g;
const rsEntries = [];
while ((m = rsEntryRe.exec(rsBlock[1]))) {
  // 按字节序列切分: "\0" 哨兵或单字符
  const pre = m[1].match(/\\0|./g) ?? [];
  rsEntries.push({ pre, type: m[2], value: parseInt(m[3], 10) });
}
const toPascal = n => n.toLowerCase().split("_").map(s => s.charAt(0).toUpperCase() + s.slice(1)).join("");
const normType = t => toPascal(t);

// ---- 3. 对数断言 ----
const fail = [];
if (v8Entries.length !== rsEntries.length) {
  fail.push(`条目数不一致: v8=${v8Entries.length} rust=${rsEntries.length}`);
}
const n = Math.min(v8Entries.length, rsEntries.length);
for (let i = 0; i < n; i++) {
  const a = v8Entries[i], b = rsEntries[i];
  const preA = a.pre.map(c => (c === "\\0" ? "\\0" : c)).join("");
  const preB = b.pre.join("");
  if (preA !== preB) fail.push(`#${i} prefix: v8="${preA}" rust="${preB}"`);
  if (normType(a.type) !== b.type) fail.push(`#${i} type: v8=${normType(a.type)} rust=${b.type}`);
  if (a.value !== b.value) fail.push(`#${i} value: v8=${a.value} rust=${b.value}`);
}

// ---- 4. 结构对数断言 (分组计数) ----
const expectCounts = { MonthName: 12, AmPm: 2, TimeZoneName: 12, TimeSeparator: 1, Invalid: 1 };
for (const [t, c] of Object.entries(expectCounts)) {
  const v8c = v8Entries.filter(e => normType(e.type) === t).length;
  const rsc = rsEntries.filter(e => e.type === t).length;
  if (v8c !== c || rsc !== c) fail.push(`${t} 计数: 期望=${c} v8=${v8c} rust=${rsc}`);
}

// ---- 5. ReadMilliseconds 缩放因子对数断言 (POW10 7 项, idx 上限 6) ----
const powMatch = tablesRs.match(/const POW10: \[i32; 7\] = \[([\d_, ]+)\];/);
if (!powMatch) {
  // POW10 在 parser.rs
  const parserRs = readFileSync(join(root, "v8-date/src/dateparser/parser.rs"), "utf8");
  const p = parserRs.match(/const POW10: \[i32; 7\] = \[([\d_, ]+)\];/);
  if (!p) throw new Error("POW10 未找到 (tables.rs / parser.rs)");
  var pow10 = p[1].split(",").map(s => parseInt(s.replace(/_/g, ""), 10));
} else {
  var pow10 = powMatch[1].split(",").map(s => parseInt(s.replace(/_/g, ""), 10));
}
for (let i = 0; i < 7; i++) {
  if (pow10[i] !== 10 ** i) fail.push(`POW10[${i}]=${pow10[i]} 期望 ${10 ** i}`);
}

if (fail.length) {
  console.error("dateparser.cc.js 对数断言失败:\n" + fail.map(f => "  - " + f).join("\n"));
  process.exit(1);
}
console.log(`dateparser.cc.js: KEYWORD_TABLE ${v8Entries.length} 条 + POW10 7 项对数断言通过`);
