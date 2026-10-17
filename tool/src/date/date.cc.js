#!/usr/bin/env bun
// tool/src/date/date.cc.js — date.{h,cc} 常量表对数断言 (只读校验, 不写 src/)
// 断言 v8-date/src/date_cache/constants.rs 的表与 v8 原表逐项一致
import { readV8, root } from "../../lib.js";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const cc = readV8("src/date/date.cc");
const h = readV8("src/date/date.h");
const consts = readFileSync(join(root, "v8-date/src/date_cache/constants.rs"), "utf8");

const fail = [];

function parseRustArray(name) {
  const re = new RegExp(`pub const ${name}: \\[[^\\]]+\\] = \\[([^\\]]*)\\]`);
  const m = consts.match(re);
  if (!m) throw new Error(`constants.rs: ${name} 未找到`);
  return m[1].split(",").map(s => s.trim().replace(/"/g, "").replace(/_/g, "")).filter(Boolean);
}

// ---- 1. 星期表 (date.cc kShortWeekDays, 7 项) ----
const wk = parseRustArray("SHORT_WEEK_DAYS");
const wkV8 = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
if (wk.length !== 7) fail.push(`SHORT_WEEK_DAYS 长度 ${wk.length} != 7`);
wkV8.forEach((d, i) => { if (wk[i] !== d) fail.push(`SHORT_WEEK_DAYS[${i}] ${wk[i]} != ${d}`); });

// ---- 2. 月份表 (date.cc kShortMonths, 12 项) ----
const mo = parseRustArray("SHORT_MONTHS");
const moV8 = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
if (mo.length !== 12) fail.push(`SHORT_MONTHS 长度 ${mo.length} != 12`);
moV8.forEach((d, i) => { if (mo[i] !== d) fail.push(`SHORT_MONTHS[${i}] ${mo[i]} != ${d}`); });

// ---- 3. 每月天数表 (date.cc kDaysInMonths, 12 项, 总和 365) ----
const dm = parseRustArray("DAYS_IN_MONTHS").map(Number);
const dmV8Match = cc.match(/kDaysInMonths\[\]\s*=\s*\{([^}]*)\}/);
if (!dmV8Match) throw new Error("date.cc: kDaysInMonths 未找到");
const dmV8 = dmV8Match[1].split(",").map(s => parseInt(s.trim(), 10));
if (dm.length !== 12 || dmV8.length !== 12) fail.push("DAYS_IN_MONTHS 长度 != 12");
dmV8.forEach((d, i) => { if (dm[i] !== d) fail.push(`DAYS_IN_MONTHS[${i}] ${dm[i]} != ${d}`); });
const dmSum = dm.reduce((a, b) => a + b, 0);
if (dmSum !== 365) fail.push(`DAYS_IN_MONTHS 总和 ${dmSum} != 365`);

// ---- 4. 年内累计天数表 (date.cc DaysFromYearMonth 内 day_from_month / _leap) ----
const dfm = parseRustArray("DAY_FROM_MONTH").map(Number);
const dfmL = parseRustArray("DAY_FROM_MONTH_LEAP").map(Number);
const dfmV8 = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
const dfmLV8 = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];
dfmV8.forEach((d, i) => { if (dfm[i] !== d) fail.push(`DAY_FROM_MONTH[${i}] ${dfm[i]} != ${d}`); });
dfmLV8.forEach((d, i) => { if (dfmL[i] !== d) fail.push(`DAY_FROM_MONTH_LEAP[${i}] ${dfmL[i]} != ${d}`); });
if (dfm[11] !== 334 || dfmL[11] !== 335) fail.push("DAY_FROM_MONTH 末项 != 334/335");

// ---- 5. 关键常量对数断言 (date.h 声明 vs constants.rs) ----
const pairs = [
  ["MS_PER_MIN", "i32", "60 * 1000"],
  ["SEC_PER_DAY", "i32", "24 * 60 * 60"],
  ["MAX_TIME_IN_MS", "i64", "864_000_000_i64 * 10_000_000_i64"],
  ["CACHE_SIZE", "usize", "32"],
  ["DAYS_IN400_YEARS", "i32", "4 * DAYS_IN100_YEARS + 1"],
  ["DAYS_OFFSET", "i32", "1000 * DAYS_IN400_YEARS + 5 * DAYS_IN400_YEARS - DAYS1970TO2000"],
];
for (const [name, ty, expr] of pairs) {
  const re = new RegExp(`pub const ${name}: ${ty} = ${expr.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")};`);
  if (!re.test(consts.replace(/\s+/g, " ").replace(/\n/g, " "))) {
    // 宽松二次匹配: 多行表达式折叠后比较
    const flat = consts.split("\n").join(" ").replace(/\s+/g, " ");
    if (!flat.includes(`pub const ${name}: ${ty} = ${expr}`)) {
      fail.push(`常量 ${name}: ${expr} 未按原式定义`);
    }
  }
}

if (fail.length) {
  console.error("date.cc.js 对数断言失败:\n" + fail.map(f => "  - " + f).join("\n"));
  process.exit(1);
}
console.log("date.cc.js: 星期 7 / 月份 12 / 月天数 12(Σ365) / 累计天数 12×2 + 关键常量 6 对数断言通过");
