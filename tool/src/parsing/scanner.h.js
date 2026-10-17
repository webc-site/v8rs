#!/usr/bin/env bun
// tool/src/parsing/scanner.h.js
// 机械提取 v8/src/parsing/scanner.h 与 scanner-inl.h 中的常量、标记位与预分配尺寸

import { readV8, writeSrc, banner } from "../../lib.js";

const scannerH = readV8("src/parsing/scanner.h");
const scannerInlH = readV8("src/parsing/scanner-inl.h");

// 1. 提取 scanner.h 常量
const lookaheadMatch = scannerH.match(/static const int kCharacterLookaheadBufferSize = (\d+);/);
const lookaheadBufSize = lookaheadMatch ? parseInt(lookaheadMatch[1], 10) : 1;

const maxAsciiMatch = scannerH.match(/static const int kMaxAscii = (\d+);/);
const maxAscii = maxAsciiMatch ? parseInt(maxAsciiMatch[1], 10) : 127;

// 2. 提取 NumberKind
const numberKindMatch = scannerH.match(/enum NumberKind \{([\s\S]*?)\};/);
if (!numberKindMatch) throw new Error("未找到 NumberKind 定义");
const numberKinds = numberKindMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter((s) => s.length > 0 && !s.startsWith("//"));

// 3. 提取 KEYWORDS 宏中的所有关键字
const kwRe = /KEYWORD\(\s*"([^"]+)"\s*,\s*Token::k(\w+)\s*\)/g;
const keywords = [];
let kwm;
while ((kwm = kwRe.exec(scannerInlH)) !== null) {
  keywords.push({ word: kwm[1], token: kwm[2] });
}
if (keywords.length === 0) throw new Error("未提取到 keywords");

const keywordChars = new Set();
const keywordStartChars = new Set();
for (const kw of keywords) {
  keywordStartChars.add(kw.word[0]);
  for (const c of kw.word) {
    keywordChars.add(c);
  }
}

function isAsciiIdentifier(ch) {
  return (
    ch === "$" ||
    ch === "_" ||
    (ch >= "a" && ch <= "z") ||
    (ch >= "A" && ch <= "Z") ||
    (ch >= "0" && ch <= "9")
  );
}

function getScanFlags(c) {
  const ch = String.fromCharCode(c);
  const isAsciiId = isAsciiIdentifier(ch);
  const canBeKwChar = keywordChars.has(ch);
  const isKwStart = keywordStartChars.has(ch);

  let flags = 0;
  if (isAsciiId && !canBeKwChar) flags |= 1 << 1; // kCannotBeKeyword
  if (!isKwStart) flags |= 1 << 2; // kCannotBeKeywordStart
  if (!isAsciiId) flags |= 1 << 0; // kTerminatesLiteral
  if (ch === "'" || ch === '"' || ch === "\n" || ch === "\r" || ch === "\\") {
    flags |= 1 << 3; // kStringTerminator
  }
  if (ch === "\\") flags |= 1 << 4; // kIdentifierNeedsSlowPath
  if (ch === "\n" || ch === "\r" || ch === "*") {
    flags |= 1 << 5; // kMultilineCommentCharacterNeedsSlowPath
  }
  return flags;
}

function getOneCharToken(c) {
  const ch = String.fromCharCode(c);
  switch (ch) {
    case "(": return "LeftParen";
    case ")": return "RightParen";
    case "{": return "LeftBrace";
    case "}": return "RightBrace";
    case "[": return "LeftBracket";
    case "]": return "RightBracket";
    case "?": return "Conditional";
    case ":": return "Colon";
    case ";": return "Semicolon";
    case ",": return "Comma";
    case ".": return "Period";
    case "|": return "BitOr";
    case "&": return "BitAnd";
    case "^": return "BitXor";
    case "~": return "BitNot";
    case "!": return "Not";
    case "<": return "LessThan";
    case ">": return "GreaterThan";
    case "%": return "Mod";
    case "=": return "Assign";
    case "+": return "Add";
    case "-": return "Sub";
    case "*": return "Mul";
    case "/": return "Div";
    case "#": return "PrivateName";
    case '"': return "String";
    case "'": return "String";
    case "`": return "TemplateSpan";
    case "\\": return "Identifier";
    case " ":
    case "\t":
    case "\v":
    case "\f":
    case "\r":
    case "\n": return "Whitespace";
    default:
      if (ch >= "0" && ch <= "9") return "Number";
      if (isAsciiIdentifier(ch)) return "Identifier";
      return "Illegal";
  }
}

// 4. 计算 128 项表
const scanFlagsTable = Array.from({ length: 128 }, (_, i) => getScanFlags(i));
const oneCharTokensTable = Array.from({ length: 128 }, (_, i) => getOneCharToken(i));

// 5. 生成 Rust 代码
const numberKindRust = numberKinds
  .map((k) => {
    // 转换为 PascalCase
    const pascal = k
      .toLowerCase()
      .split("_")
      .map((p) => p.charAt(0).toUpperCase() + p.slice(1))
      .join("");
    return `  ${pascal},`;
  })
  .join("\n");

const out = `${banner(
  "src/parsing/scanner.h.js",
  "src/parsing/scanner.h + scanner-inl.h",
  "词法扫描器扫描标志位、单字符Token表与数字类型",
)}
use crate::token_generated::Token;

/// 字符向前看缓冲大小 (${lookaheadBufSize})
pub const K_CHARACTER_LOOKAHEAD_BUFFER_SIZE: usize = ${lookaheadBufSize};

/// 最大 ASCII 字符值 (${maxAscii})
pub const K_MAX_ASCII: u32 = ${maxAscii};

/// 输入流结束标记
pub const K_END_OF_INPUT: u32 = u32::MAX;

/// 数值字面量分类 (原版 Scanner::NumberKind)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberKind {
${numberKindRust}
}

impl NumberKind {
  #[inline]
  #[must_use]
  pub const fn is_valid_bigint_kind(&self) -> bool {
    matches!(self, Self::Binary | Self::Octal | Self::Hex | Self::Decimal)
  }

  #[inline]
  #[must_use]
  pub const fn is_decimal_number_kind(&self) -> bool {
    matches!(self, Self::Decimal | Self::DecimalWithLeadingZero)
  }
}

/// 扫描标志位 (原版 Scanner::ScanFlags)
pub mod scan_flags {
  pub const K_TERMINATES_LITERAL: u8 = 1 << 0;
  pub const K_CANNOT_BE_KEYWORD: u8 = 1 << 1;
  pub const K_CANNOT_BE_KEYWORD_START: u8 = 1 << 2;
  pub const K_STRING_TERMINATOR: u8 = 1 << 3;
  pub const K_IDENTIFIER_NEEDS_SLOW_PATH: u8 = 1 << 4;
  pub const K_MULTILINE_COMMENT_CHARACTER_NEEDS_SLOW_PATH: u8 = 1 << 5;

  #[inline]
  #[must_use]
  pub const fn terminates_literal(flags: u8) -> bool {
    (flags & K_TERMINATES_LITERAL) != 0
  }

  #[inline]
  #[must_use]
  pub const fn can_be_keyword(flags: u8) -> bool {
    (flags & K_CANNOT_BE_KEYWORD) == 0
  }

  #[inline]
  #[must_use]
  pub const fn identifier_needs_slow_path(flags: u8) -> bool {
    (flags & K_IDENTIFIER_NEEDS_SLOW_PATH) != 0
  }

  #[inline]
  #[must_use]
  pub const fn multiline_comment_character_needs_slow_path(flags: u8) -> bool {
    (flags & K_MULTILINE_COMMENT_CHARACTER_NEEDS_SLOW_PATH) != 0
  }

  #[inline]
  #[must_use]
  pub const fn may_terminate_string(flags: u8) -> bool {
    (flags & K_STRING_TERMINATOR) != 0
  }
}

/// ASCII 0..127 扫描标志查找表 (character_scan_flags)
pub static CHARACTER_SCAN_FLAGS: [u8; 128] = [
${Array.from({ length: 8 }, (_, row) =>
  "  " + scanFlagsTable.slice(row * 16, (row + 1) * 16).join(", ") + ",",
).join("\n")}
];

/// ASCII 0..127 单字符 Token 映射表 (one_char_tokens)
pub static ONE_CHAR_TOKENS: [Token; 128] = [
${Array.from({ length: 16 }, (_, row) =>
  "  " +
  oneCharTokensTable
    .slice(row * 8, (row + 1) * 8)
    .map((t) => `Token::${t}`)
    .join(", ") +
  ",",
).join("\n")}
];

/// 源码位置区间 (原版 Scanner::Location)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Location {
  pub beg_pos: usize,
  pub end_pos: usize,
}

impl Location {
  #[inline]
  #[must_use]
  pub const fn new(beg_pos: usize, end_pos: usize) -> Self {
    Self { beg_pos, end_pos }
  }

  #[inline]
  #[must_use]
  pub const fn invalid() -> Self {
    Self {
      beg_pos: usize::MAX,
      end_pos: usize::MAX,
    }
  }

  #[inline]
  #[must_use]
  pub const fn is_valid(&self) -> bool {
    self.beg_pos != usize::MAX && self.beg_pos <= self.end_pos
  }

  #[inline]
  #[must_use]
  pub const fn length(&self) -> usize {
    self.end_pos.saturating_sub(self.beg_pos)
  }
}
`;

const target = writeSrc("v8-parsing", "scanner_generated.rs", out);
console.log(`成功生成 Scanner 常量表与标记位, 提取 ${keywords.length} 个关键字`);
console.log(`写入完成: ${target}`);
