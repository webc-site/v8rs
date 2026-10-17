#!/usr/bin/env bun
// tool/src/date/date.h.js — 从 v8/src/date/date.{h,cc} 提取常量与表
import { readV8, writeSrc, banner } from "../../lib.js";

const CRATE = "v8-date";

const h = readV8("src/date/date.h");
const cc = readV8("src/date/date.cc");

function toScreamingSnake(name) {
  return name.replace(/^k([A-Z])/, "$1")
             .replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2")
             .replace(/([a-z\d])([A-Z])/g, "$1_$2")
             .replace(/DAYS_IN4_/, "DAYS_IN_4_")
             .replace(/DAYS_IN100_/, "DAYS_IN_100_")
             .replace(/DAYS_IN400_/, "DAYS_IN_400_")
             .replace(/DAYS1970TO2000/, "DAYS_1970_TO_2000")
             .toUpperCase();
}

// 提取常量
const consts = [];
const constRe = /(?:static\s+)?const (int|int64_t|double) (\w+) =([\s\S]*?);/g;
let m;

while ((m = constRe.exec(h))) {
  const ty = m[1] === "int" ? "i32" : m[1] === "int64_t" ? "i64" : "f64";
  consts.push({ type: ty, name: m[2], expr: m[3].trim().replace(/\s+/g, " ") });
}

while ((m = constRe.exec(cc))) {
  const ty = m[1] === "int" ? "i32" : m[1] === "int64_t" ? "i64" : "f64";
  consts.push({ type: ty, name: m[2], expr: m[3].trim().replace(/\s+/g, " ") });
}

// 表达式修饰
function toRustExpr(expr, ty) {
  let res = expr
    .replace(/kMaxInt/g, "i32::MAX")
    .replace(/static_cast<int64_t>\((.*?)\)/g, "($1 as i64)")
    .replace(/static_cast<int>\((.*?)\)/g, "($1 as i32)")
    .replace(/(\b\d+)U\b/g, "$1")
    .replace(/(\b\d+)LL\b/g, "$1")
    .replace(/year_delta/g, "YEAR_DELTA");

  if (ty === "f64") {
    res = res.replace(/-10000000\.0/g, "-10_000_000.0")
             .replace(/-1000000\.0/g, "-1_000_000.0")
             .replace(/86400000\.0/g, "86_400_000.0")
             .replace(/60000\.0/g, "60_000.0")
             .replace(/3600000\.0/g, "3_600_000.0")
             .replace(/1000\.0/g, "1_000.0");
  } else {
    res = res.replace(/\b864000000\b/g, "864_000_000_i64")
             .replace(/\b10000000\b/g, "10_000_000_i64");
  }

  // 符号名替换为 SCREAMING_SNAKE
  res = res.replace(/\bk([A-Za-z0-9]+)\b/g, (_, name) => {
    return toScreamingSnake("k" + name);
  });

  if (ty === "i64") {
    res = res.replace(/\bSEC_PER_DAY \* 1000\b/, "SEC_PER_DAY as i64 * 1000");
    // 冗余的 (x_i64 as i64) 收敛为 x_i64
    res = res.replace(/\((\w+_i64) as i64\)/g, "$1");
  }

  return res;
}

// 超长表达式在顶层 (括号深度 0) 的 " + " / " - " 处断为多行 (与 rustfmt 风格一致)
function wrapLongExpr(decl) {
  if (decl.length <= 100) return decl;
  const eq = decl.indexOf(" = ");
  const head = decl.slice(0, eq + 3);
  const expr = decl.slice(head.length, -1); // 去掉结尾分号
  // 深度为 0 处收集顶层 + / - 分割点
  const cuts = [];
  let depth = 0;
  for (let i = 0; i < expr.length; i++) {
    const c = expr[i];
    if (c === "(") depth++;
    else if (c === ")") depth--;
    else if (depth === 0 && (c === "+" || c === "-") && expr[i + 1] === " " && expr[i - 1] === " ") {
      cuts.push(i);
    }
  }
  if (cuts.length < 2) return decl;
  // 首行含到第二个分割点前, 其余每个分割点起一段
  const first = head + expr.slice(0, cuts[1]).trimEnd();
  const segs = [];
  let prev = cuts[1];
  for (const c of cuts.slice(2)) {
    segs.push(expr.slice(prev, c).trim());
    prev = c;
  }
  segs.push(expr.slice(prev).trim());
  return first + "\n" + segs.map(s => "  " + s).join("\n") + ";";
}

let rustConstLines = [];
const seenNames = new Set();
for (const c of consts) {
  let rustName = toScreamingSnake(c.name);
  if (rustName === "MS_PER_DAY" && c.type === "f64") {
    rustName = "MS_PER_DAY_F64";
  } else if (rustName === "MS_PER_HOUR" && c.type === "f64") {
    rustName = "MS_PER_HOUR_F64";
  }
  if (seenNames.has(rustName)) continue;
  seenNames.add(rustName);
  // CACHE_SIZE 作数组长度用, 生成 usize
  const ty = rustName === "CACHE_SIZE" ? "usize" : c.type;
  rustConstLines.push(wrapLongExpr(`pub const ${rustName}: ${ty} = ${toRustExpr(c.expr, c.type)};`));
}

// 提取 kDaysInMonths
const daysInMonthsMatch = cc.match(/static const char kDaysInMonths\[\] = \{([^\}]+)\};/);
let daysInMonths = "";
if (daysInMonthsMatch) {
  const items = daysInMonthsMatch[1].split(",").map(s => s.trim()).filter(Boolean);
  daysInMonths = `pub const DAYS_IN_MONTHS: [i8; 12] = [${items.join(", ")}];`;
}

// 提取 kDayFromMonth and kDayFromMonthLeap
const dayFromMonthMatch = cc.match(/static const int day_from_month\[\] = \{([^\}]+)\};/);
const dayFromMonthLeapMatch = cc.match(/static const int day_from_month_leap\[\] = \{([^\}]+)\};/);

let dayFromMonthStr = "";
if (dayFromMonthMatch) {
  const items = dayFromMonthMatch[1].split(",").map(s => s.trim()).filter(Boolean);
  dayFromMonthStr = `pub const DAY_FROM_MONTH: [i32; 12] = [${items.join(", ")}];`;
}

let dayFromMonthLeapStr = "";
if (dayFromMonthLeapMatch) {
  const items = dayFromMonthLeapMatch[1].split(",").map(s => s.trim()).filter(Boolean);
  dayFromMonthLeapStr = `pub const DAY_FROM_MONTH_LEAP: [i32; 12] = [${items.join(", ")}];`;
}

const outBody = `${banner("src/date/date.h.js", "src/date/date.h")}
${rustConstLines.join("\n")}

${daysInMonths}

${dayFromMonthStr}

${dayFromMonthLeapStr}

pub const SHORT_WEEK_DAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
pub const SHORT_MONTHS: [&str; 12] = [
  "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToDateStringMode {
  LocalDate,
  LocalTime,
  LocalDateAndTime,
  UTCDateAndTime,
  ISODateAndTime,
}
`;

writeSrc(CRATE, "date_cache/constants.rs", outBody);
console.log("date.h.js: 成功生成 v8-date/src/date_cache/constants.rs");
