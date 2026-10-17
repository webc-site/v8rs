#!/usr/bin/env bun
// tool/src/json/json-stringifier.cc.js
// 从 v8/src/json/json-stringifier.cc 提取字符转义表与无须转义标记表
// 生成: v8-json/src/escape_table.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/json/json-stringifier.cc");

// 1. 提取 JsonEscapeTable
const escapeMatch = ccSrc.match(/constexpr const char\* const JsonEscapeTable =\s*([\s\S]*?);/);
if (!escapeMatch) throw new Error("未找到 JsonEscapeTable");

// 提取 JsonDoNotEscapeFlagTable
const flagMatch = ccSrc.match(/constexpr bool JsonDoNotEscapeFlagTable\[\] = \{([\s\S]*?)\};/);
if (!flagMatch) throw new Error("未找到 JsonDoNotEscapeFlagTable");

const rawFlags = flagMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean)
  .map((s) => s === "1");

console.log(`成功提取 ${rawFlags.length} 个无须转义标记 (JsonDoNotEscapeFlagTable)`);

// 构建前 256 字符的标准 JSON 转义映射
// 在 JSON 规范中：
// - U+0000 到 U+001F 必须被转义为 \u00XX 或缩写 (\b, \t, \n, \f, \r)
// - " 必须转义为 \"
// - \ 必须转义为 \\
const escapeStrings = [];
for (let i = 0; i < 256; i++) {
  if (i === 0x08) {
    escapeStrings.push('"\\\\b"');
  } else if (i === 0x09) {
    escapeStrings.push('"\\\\t"');
  } else if (i === 0x0a) {
    escapeStrings.push('"\\\\n"');
  } else if (i === 0x0c) {
    escapeStrings.push('"\\\\f"');
  } else if (i === 0x0d) {
    escapeStrings.push('"\\\\r"');
  } else if (i === 0x22) {
    escapeStrings.push('"\\\\\\""');
  } else if (i === 0x5c) {
    escapeStrings.push('"\\\\\\\\"');
  } else if (i < 0x20) {
    const hex = i.toString(16).padStart(4, "0");
    escapeStrings.push(`"\\\\u${hex}"`);
  } else {
    escapeStrings.push('""');
  }
}

const code = `${banner("tool/src/json/json-stringifier.cc.js 从 v8/src/json/json-stringifier.cc 生成; 修订就地改, 复跑覆盖")}
// JSON 字符转义表与无须转义快速过滤位图

/// Latin1 字符是否无须转义标记表 (对应原版 JsonDoNotEscapeFlagTable，长 256)
pub static JSON_DO_NOT_ESCAPE_FLAG_TABLE: [bool; 256] = [
${rawFlags.map((f, i) => `  /* 0x${i.toString(16).padStart(2, "0")} */ ${f},`).join("\n")}
];

/// Latin1 字符转义替换字符串表 (0..=255)
/// 非空表示必须转义，空字符串表示原样输出
pub static JSON_ESCAPE_TABLE: [&str; 256] = [
${escapeStrings.map((s, i) => `  /* 0x${i.toString(16).padStart(2, "0")} */ ${s},`).join("\n")}
];

/// 快速判断单字节 Latin1 字符是否不需要转义
#[inline]
#[must_use]
pub const fn is_do_not_escape(byte: u8) -> bool {
  JSON_DO_NOT_ESCAPE_FLAG_TABLE[byte as usize]
}

/// 获取单字节 Latin1 字符的转义字符串，若无需转义返回 None
#[inline]
#[must_use]
pub const fn get_escape_str(byte: u8) -> Option<&'static str> {
  let s = JSON_ESCAPE_TABLE[byte as usize];
  if s.is_empty() {
    None
  } else {
    Some(s)
  }
}
`;

writeSrc("v8-json", "escape_table.rs", code);
