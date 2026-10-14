#!/usr/bin/env bun
// tool/src/parsing/scanner.cc.js
// 机械提取 v8/src/parsing/scanner.cc + scanner-character-streams.{h,cc}: 扫描器面

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/parsing/scanner.cc");
const csH = readV8("src/parsing/scanner-character-streams.h");
const csCc = readV8("src/parsing/scanner-character-streams.cc");

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

const scDefs = collectDefs(ccSrc, "Scanner");
if (scDefs.length < 20) throw new Error(`scanner.cc Scanner 定义头应 ≥20, 实得 ${scDefs.length}`);

// 字符流族
const csAnchors = ["ScannerStream"];
for (const k of csAnchors) {
  if (!csH.includes(k)) throw new Error(`scanner-character-streams.h 缺 ${k}`);
}
const csImpl = [...csCc.matchAll(/^(?:void|size_t|bool|uint16_t|uint32_t|ScannerStream)\s?[\w:<>,*& ]*?(\w+)\(/gm)].map((m) => m[1]);
if (csImpl.length < 5) throw new Error(`character-streams.cc 函数应 ≥5, 实得 ${csImpl.length}`);

// 关键锚点: SkipWhiteSpace / ScanSingleToken / unicode 转义
for (const k of ["ErrorState", "Scanner::", "Location"]) {
  if (!ccSrc.includes(k)) throw new Error(`scanner.cc 缺 ${k}`);
}

let rust = banner(
  "src/parsing/scanner.cc.js",
  "src/parsing/scanner.cc + scanner-character-streams.{h,cc}",
  "扫描器与字符流骨架",
);

rust += `
/// Scanner 定义头清单 (对数基准, ${scDefs.length} 项):
${scDefs.map((s) => "// " + s).join("\n")}

/// 字符流门面 (原版 scanner-character-streams.h ScannerStream:
/// 底座 Utf16CharacterStream, 分派缓冲/无缓冲实现)
pub const CHARACTER_STREAM_CLASSES: [&str; 1] = ["ScannerStream"];

pub mod scanner_cc_skel {
  /// 扫描热路径锚点
  pub const HOT_PATHS: [&str; 3] = ["ErrorState", "Location", "Scan"];

  /// 单 token 扫描骨架
  pub fn scan_single_token() {
    unimplemented!("骨架")
  }
}
`;

const p = writeSrc("v8-parsing", "scanner_cc_generated.rs", rust);
console.log(`written ${p}; Scanner=${scDefs.length}, 流实现=${csImpl.length}`);
