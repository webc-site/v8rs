#!/usr/bin/env bun
// tool/src/parsing/keywords.js
// 机械提取 v8/src/parsing/keywords.txt 中的关键字映射并生成 phf 静态完美散列查找表

import { readV8, writeSrc, banner } from "../../lib.js";

const txt = readV8("src/parsing/keywords.txt");
const lines = txt.split("\n");

const entries = [];
let inEntries = false;

for (const line of lines) {
  const trimmed = line.trim();
  if (trimmed === "%%") {
    inEntries = true;
    continue;
  }
  if (!inEntries || !trimmed || trimmed.startsWith("//")) continue;

  const parts = trimmed.split(",").map(s => s.trim());
  if (parts.length === 2) {
    const kw = parts[0];
    const tok = parts[1].replace(/^Token::k/, "");
    entries.push({ keyword: kw, token: tok });
  }
}

console.log(`成功提取 ${entries.length} 个关键字映射`);

let rust = banner("src/parsing/keywords.js", "src/parsing/keywords.txt", "编译期完美散列关键字映射表");

rust += `
use phf::phf_map;
use crate::token_generated::Token;

/// 关键字完美散列查找表 (原版 PerfectKeywordHash, 共 ${entries.length} 个)
pub static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
`;

for (const e of entries) {
  rust += `  "${e.keyword}" => Token::${e.token},\n`;
}

rust += `};

/// 根据文本查找 Token, 若不是关键字则返回 None
#[inline]
pub fn lookup_keyword(name: &str) -> Option<Token> {
  KEYWORDS.get(name).copied()
}
`;

writeSrc("v8-parsing", "keywords_generated.rs", rust);
console.log("写入完成: v8-parsing/src/keywords_generated.rs");
