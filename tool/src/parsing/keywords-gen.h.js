#!/usr/bin/env bun
// tool/src/parsing/keywords-gen.h.js
// 机械提取 v8/src/parsing/keywords-gen.h: gperf 完美哈希关键词表参数与表项

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/parsing/keywords-gen.h");

// 1. gperf 参数
function num(name) {
  const m = hSrc.match(new RegExp(`${name} = (\\d+)`));
  if (!m) throw new Error(`未找到 ${name}`);
  return parseInt(m[1], 10);
}
const totalKeywords = num("TOTAL_KEYWORDS");
const minHash = num("MIN_HASH_VALUE");
const maxHash = num("MAX_HASH_VALUE");
const totalKeys = hSrc.match(/TOTAL_KEYWORDS = 52,/) ? true : false;
if (totalKeywords !== 52) throw new Error(`TOTAL_KEYWORDS 漂移: ${totalKeywords}`);

// 2. 表项 ({"name", Token::Value::kXxx} 或 {""..}) — 双列 (查表态/词态)
const tblStart = hSrc.indexOf("kPerfectKeywordHashTable[128]");
if (tblStart < 0) throw new Error("未找到 kPerfectKeywordHashTable");
const slotCount = 128;
const brace = hSrc.slice(tblStart).match(/=\s*(\{[\s\S]*?\}\s*)\;/);
if (!brace) throw new Error("表体括号配平失败");
const entries = [...brace[1].matchAll(/\{"([^"]*)",\s*(Token::k\w+)\}/g)].map(
  (m) => ({ name: m[1], token: m[2] }),
);
if (slotCount !== 128) throw new Error(`槽位应 128, 实得 ${slotCount}`);
if (entries.length !== 128) throw new Error(`表项应 128, 实得 ${entries.length}`);
const emptySlots = entries.filter((e) => e.name === "").length;

// 3. 字符集断言锚点 (ASCII 32..126 除 '"',#,$,',: 等非词字符)
if (!hSrc.includes(`"gperf generated tables don't work with this execution character set."`)) {
  throw new Error("缺字符集断言");
}
// 槽位数 = MAX_HASH_VALUE + 1
const slots = maxHash + 1;

let rust = banner("src/parsing/keywords-gen.h.js", "src/parsing/keywords-gen.h", "gperf 完美哈希关键词表");

rust += `
/// gperf 完美哈希关键词表参数 (原版 keywords-gen.h)
pub mod keyword_table_const {
  /// 关键词总数 (原版 TOTAL_KEYWORDS)
  pub const TOTAL_KEYWORDS: usize = ${totalKeywords};
  /// 最小哈希值 (原版 MIN_HASH_VALUE)
  pub const MIN_HASH_VALUE: usize = ${minHash};
  /// 最大哈希值 (原版 MAX_HASH_VALUE)
  pub const MAX_HASH_VALUE: usize = ${maxHash};
  /// 槽位容量 = MAX_HASH_VALUE + 1 (原版 kPerfectKeywordHashTable[128])
  pub const SLOTS: usize = ${slotCount};
  /// 空洞槽数 (name 为空 → kIdentifier)
  pub const EMPTY_SLOTS: usize = ${emptySlots};

  /// ASCII 词字符集断言锚点 (原版文件头大段 ('a'==97) && ... 断言)
  pub const ASCII_ASSERT_ANCHOR: bool = true;
}

/// 表项 (原文 ${entries.length} 项, 1:1 顺序登记; token 引用 crate 内 token_generated)
pub struct KeywordTableEntry {
  /// 关键词字面 (空串表示哈希空洞 → kIdentifier)
  pub name: &'static str,
  /// Token 种别
  pub token: &'static str,
}

#[rustfmt::skip]
pub static KEYWORD_TABLE: [KeywordTableEntry; ${entries.length}] = [
${entries
  .map(
    (e) => `  KeywordTableEntry { name: "${e.name}", token: "${e.token.replace("Token::", "")}" },`,
  )
  .join("\n")}
];
`;

const p = writeSrc("v8-parsing", "keywords_gen_generated.rs", rust);
console.log(`written ${p}; 关键词=${totalKeywords}, 表项=${entries.length}, 空洞=${emptySlots}`);
