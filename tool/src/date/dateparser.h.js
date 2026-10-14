#!/usr/bin/env bun
// tool/src/date/dateparser.h.js — 从 v8/src/date/dateparser.{h,cc} 提取关键字表与解析器常量
import { readV8, writeSrc, banner } from "../../lib.js";

const CRATE = "v8-date";

const h = readV8("src/date/dateparser.h");
const cc = readV8("src/date/dateparser.cc");

// 1. 提取 KeywordType
const kwTypeMatch = h.match(/enum KeywordType\s*\{([^}]+)\};/);
const kwTypes = kwTypeMatch ? kwTypeMatch[1].split(",").map(s => s.trim()).filter(Boolean) : [];

// 2. 提取 KeywordTable::array
const arrayMatch = cc.match(/KeywordTable::array\[\]\[[^\]]+\]\s*=\s*\{([\s\S]*?)\};/);
if (!arrayMatch) {
  throw new Error("KeywordTable::array 未找到");
}

const rawEntries = arrayMatch[1];
const entryRe = /\{\s*('[^']+'|'\\0')\s*,\s*('[^']+'|'\\0')\s*,\s*('[^']+'|'\\0')\s*,\s*DateParser::([A-Z_]+)\s*,\s*(-?\d+)\s*\}/g;
let m;
const entries = [];
while ((m = entryRe.exec(rawEntries))) {
  const raw0 = m[1];
  const raw1 = m[2];
  const raw2 = m[3];
  const kwType = m[4];
  const value = parseInt(m[5], 10);
  entries.push({ raw0, raw1, raw2, kwType, value });
}

function toPascal(name) {
  return name.toLowerCase().split("_").map(s => s.charAt(0).toUpperCase() + s.slice(1)).join("");
}

function charLiteralToByteStr(l) {
  if (l === "'\\0'") return "\\0";
  return l.slice(1, -1);
}

const rustKwEntries = entries.map(e => {
  const bs = charLiteralToByteStr(e.raw0) + charLiteralToByteStr(e.raw1) + charLiteralToByteStr(e.raw2);
  return `  KeywordEntry {
    prefix: *b"${bs}",
    kw_type: KeywordType::${toPascal(e.kwType)},
    value: ${e.value},
  },`;
});

const outBody = `${banner("src/date/dateparser.h.js", "src/date/dateparser.h")}
pub const YEAR: usize = 0;
pub const MONTH: usize = 1;
pub const DAY: usize = 2;
pub const HOUR: usize = 3;
pub const MINUTE: usize = 4;
pub const SECOND: usize = 5;
pub const MILLISECOND: usize = 6;
pub const UTC_OFFSET: usize = 7;
pub const OUTPUT_SIZE: usize = 8;

pub const K_NONE: i32 = i32::MAX;
pub const MAX_SIGNIFICANT_DIGITS: usize = 9;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeywordType {
  Invalid = 0,
  MonthName = 1,
  TimeZoneName = 2,
  TimeSeparator = 3,
  AmPm = 4,
}

#[derive(Debug, Clone, Copy)]
pub struct KeywordEntry {
  pub prefix: [u8; 3],
  pub kw_type: KeywordType,
  pub value: i32,
}

pub const KEYWORD_TABLE: [KeywordEntry; ${entries.length}] = [
${rustKwEntries.join("\n")}
];

pub const PREFIX_LENGTH: usize = 3;

/// KeywordTable::Lookup
pub fn lookup_keyword(pre: &[u32; 3], len: usize) -> Option<usize> {
  if pre[0] >= 128 || pre[1] >= 128 || pre[2] >= 128 {
    return None;
  }
  let p = [pre[0] as u8, pre[1] as u8, pre[2] as u8];
  for (i, entry) in KEYWORD_TABLE.iter().enumerate() {
    if entry.kw_type == KeywordType::Invalid {
      break;
    }
    if entry.prefix == p && (len <= PREFIX_LENGTH || entry.kw_type == KeywordType::MonthName) {
      return Some(i);
    }
  }
  None
}
`;

writeSrc(CRATE, "dateparser/tables.rs", outBody);
console.log("dateparser.h.js: 成功生成 v8-date/src/dateparser/tables.rs");
