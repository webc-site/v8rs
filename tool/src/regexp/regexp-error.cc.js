#!/usr/bin/env bun
// tool/src/regexp/regexp-error.cc.js
// 机械提取 v8/src/regexp/regexp-error.cc 的 kErrorStrings 表
// (REGEXP_ERROR_MESSAGES X-macro TEMPLATE 展开; Error 枚举本体由
//  regexp-error.h 侧生成物覆盖, 此处只出字符串表)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/regexp/regexp-error.h"); // X-macro 定义所在
const ccSrc = readV8("src/regexp/regexp-error.cc");

// 宏体: REGEXP_ERROR_MESSAGES(T) T(kName, "string") ...
const start = hSrc.indexOf("#define REGEXP_ERROR_MESSAGES");
if (start < 0) throw new Error("未找到 REGEXP_ERROR_MESSAGES");
const lines = hSrc.slice(start).split("\n");
const body = [];
for (let i = 1; i < lines.length; i++) {
  const cont = /\\\s*$/.test(lines[i]);
  body.push(lines[i].replace(/\\\s*$/, ""));
  if (!cont) break;
}
const entries = [...body.join("\n").matchAll(/T\((\w+),\s*"([^"]*)"\)/g)].map((m) => ({ name: m[1], str: m[2] }));
if (entries.length < 10) throw new Error(`错误消息条数异常: ${entries.length}`);

// cc 侧 TEMPLATE 展开形状核对
if (!ccSrc.match(/#define TEMPLATE\(NAME, STRING\) STRING,/)) throw new Error("cc TEMPLATE 形状变更");
if (!/const char\* const kErrorStrings\[\]/.test(ccSrc)) throw new Error("未找到 kErrorStrings");

console.log(`regexp-error.cc: kErrorStrings=${entries.length}`);

let rust = banner(
  "src/regexp/regexp-error.cc.js",
  "src/regexp/regexp-error.cc + regexp-error.h",
  "正则错误消息表 (REGEXP_ERROR_MESSAGES 展开)",
);

rust += `
/// 错误消息字符串表 (对应原版 kErrorStrings[], 索引即 Error 枚举序)
pub const ERROR_STRINGS: &[&str] = &[
`;
for (const e of entries) {
  rust += `  "${e.str}", // ${e.name}\n`;
}
rust += `];

/// 错误名登记 (与 ERROR_STRINGS 同序)
pub const ERROR_NAMES: &[&str] = &[
`;
for (const e of entries) {
  rust += `  "${e.name}",\n`;
}
rust += `];

/// 原版 ErrorString(Error) — 越界由调用方保证 (DCHECK_LT(error, Error::NumErrors))
#[inline]
#[must_use]
pub fn error_string(error: usize) -> &'static str {
  ERROR_STRINGS[error]
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(ERROR_STRINGS.len() == ${entries.length});
  assert!(ERROR_NAMES.len() == ERROR_STRINGS.len());
};
`;

const p = writeSrc("v8-regexp-syntax", "regexp_error_strings_generated.rs", rust);
console.log(`written ${p}`);
