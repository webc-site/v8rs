#!/usr/bin/env bun
// tool/src/parsing/preparser.cc.js
// 机械提取 v8/src/parsing/ 预解析族: preparser.{h,cc} + preparse-data.{h,cc} +
// preparse-data-impl.h + preparser-logger.h

import { readV8, writeSrc, banner } from "../../lib.js";

const pph = readV8("src/parsing/preparser.h");
const ppc = readV8("src/parsing/preparser.cc");
const pdh = readV8("src/parsing/preparse-data.h");
const pdc = readV8("src/parsing/preparse-data.cc");
const pdi = readV8("src/parsing/preparse-data-impl.h");
const plg = readV8("src/parsing/preparser-logger.h");

function collectDefs(src, prefix) {
  const out = [];
  const ls = src.split("\n");
  for (let i = 0; i < ls.length; i++) {
    const l = ls[i];
    if (!new RegExp(`^[\\w:<>,*&\\s]*\\b${prefix}::~?\\w+\\(`).test(l) || /^\s*\/\//.test(l)) continue;
    let sig = l;
    let j = i;
    while (!/\{\}?\s*$|;\s*$/.test(sig) && j + 1 < ls.length) sig += " " + ls[++j].trim();
    sig = sig.replace(/\{\s*$/, "").replace(/\{\}\s*$/, "").trim();
    if (sig.endsWith(";")) continue;
    out.push(sig);
  }
  return out;
}

const ppDefs = collectDefs(ppc, "PreParser");
const pdDefs = collectDefs(pdc, "PreparseDataBuilder");
if (ppDefs.length < 3) throw new Error(`preparser.cc 定义头应 ≥3, 实得 ${ppDefs.length}`);
if (pdDefs.length < 2) throw new Error(`preparse-data.cc 定义头应 ≥2, 实得 ${pdDefs.length}`);

// preparse-data 结构锚点
for (const k of ["PreparseData", "ZonePreparseData", "PreparseDataBuilder"]) {
  if (!pdh.includes(k) && !pdi.includes(k)) throw new Error(`preparse-data 缺 ${k}`);
}
// 预解析器面
for (const k of ["class PreParser", "PreParserIdentifier", "PreParserExpression"]) {
  if (!pph.includes(k)) throw new Error(`preparser.h 缺 ${k}`);
}
// 日志器
if (!plg.includes("PreParserLogger") && !/class \w+Logger/.test(plg)) throw new Error("preparser-logger.h 面漂移");

let rust = banner(
  "src/parsing/preparser.cc.js",
  "src/parsing/preparser.{h,cc} + preparse-data.{h,cc,-impl.h} + preparser-logger.h",
  "预解析族骨架",
);

rust += `
/// 预解析器结构锚点 (原版 preparser.h: 惰性解析, 只记函数边界与跳过区)
pub const PREPARSER_ANCHORS: [&str; 3] = ["PreParser", "PreParserIdentifier", "PreParserExpression"];

/// 预解析数据结构锚点 (原版 preparse-data.h/-impl.h)
pub const PREPARSE_DATA_ANCHORS: [&str; 3] = ["PreParserData", "UnresolvedData", "PreparseDataBuilder"];

/// 预解析日志器 (原版 preparser-logger.h)
pub const PREPARSER_LOGGER_ANCHOR: bool = true;

// preparser.cc PreParser 定义头清单 (${ppDefs.length} 项):
${ppDefs.map((s) => "// " + s).join("\n")}

// preparse-data.cc 定义头清单 (${pdDefs.length} 项):
${pdDefs.map((s) => "// " + s).join("\n")}
`;

const p = writeSrc("v8-parsing", "preparser_generated.rs", rust);
console.log(`written ${p}; PreParser=${ppDefs.length}, 数据=${pdDefs.length}`);
