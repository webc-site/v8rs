#!/usr/bin/env bun
// tool/src/json/json-parser.h.js
// 机械提取 v8/src/json/json-parser.{h,cc} 中的 Token 体系与常量表

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/json/json-parser.h");
const ccSrc = readV8("src/json/json-parser.cc");

// 1. 提取 JsonToken 枚举
const tokenEnumMatch = hSrc.match(/enum class JsonToken : uint8_t\s*\{([^}]+)\};/);
if (!tokenEnumMatch) throw new Error("未找到 JsonToken 枚举");

const rawTokens = tokenEnumMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);

// 映射为 PascalCase
function toPascalCase(name) {
  return name
    .toLowerCase()
    .split("_")
    .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
    .join("");
}

const tokens = rawTokens.map((t, idx) => ({
  name: t,
  rustName: toPascalCase(t),
  repr: idx,
}));

console.log(`成功提取 ${tokens.length} 个 JsonToken 定义`);

// 2. 提取 JsonTokenToCharacter 映射
const tokenToCharMatch = hSrc.match(
  /constexpr uint8_t JsonTokenToCharacter\(JsonToken token\)\s*\{([\s\S]*?CONSTEXPR_UNREACHABLE\(\);\s*\})/,
);
if (!tokenToCharMatch) throw new Error("未找到 JsonTokenToCharacter");

const tokenCharMap = new Map();
const charCaseRegex = /case JsonToken::(\w+):\s*return\s*'([^']+)';/g;
let cm;
while ((cm = charCaseRegex.exec(tokenToCharMatch[1])) !== null) {
  tokenCharMap.set(cm[1], cm[2]);
}

// 3. 提取常量
const maxContextMatch = hSrc.match(/static const int kMaxContextCharacters = (\d+);/);
const maxContext = maxContextMatch ? parseInt(maxContextMatch[1], 10) : 10;

const minContextMatch = hSrc.match(/static const int kMinOriginalSourceLengthForContext =\s*\(([^)]+)\);/);
const minContext = (maxContext * 2) + 1;

const initSpecialStrMatch = hSrc.match(/static const int kInitialSpecialStringLength = (\d+);/);
const initSpecialStr = initSpecialStrMatch ? parseInt(initSpecialStrMatch[1], 10) : 32;

// 4. 提取 EscapeKind
const escapeKindMatch = ccSrc.match(/enum class EscapeKind : uint8_t\s*\{([^}]+)\};/);
if (!escapeKindMatch) throw new Error("未找到 EscapeKind 枚举");

const rawEscapeKinds = escapeKindMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);

const escapeKinds = rawEscapeKinds.map((k, idx) => {
  // kIllegal -> Illegal, kSelf -> SelfEscape, kBackspace -> Backspace, etc.
  let rustName = k.replace(/^k/, "");
  if (rustName === "Self") rustName = "SelfEscape";
  return {
    raw: k,
    rustName,
    repr: idx,
  };
});

// 5. 模拟 GetJsonScanFlags(c) 生成 256 表
// EscapeKindField: 0..3 (bits 0,1,2)
// MayTerminateStringField: next 1 bit (bit 3)
// NumberPartField: next 1 bit (bit 4)
const scanFlags = new Uint8Array(256);
for (let c = 0; c < 256; c++) {
  const ch = String.fromCharCode(c);
  let escapeKind = 0; // kIllegal
  if (ch === 'b') escapeKind = 2; // kBackspace
  else if (ch === 't') escapeKind = 3; // kTab
  else if (ch === 'n') escapeKind = 4; // kNewLine
  else if (ch === 'f') escapeKind = 5; // kFormFeed
  else if (ch === 'r') escapeKind = 6; // kCarriageReturn
  else if (ch === 'u') escapeKind = 7; // kUnicode
  else if (ch === '"' || ch === '\\' || ch === '/') escapeKind = 1; // kSelf

  const mayTerminate = (c < 0x20 || ch === '"' || ch === '\\') ? 1 : 0;
  const isDigit = c >= 0x30 && c <= 0x39;
  const isNumberPart = (ch === '.' || ch === 'e' || ch === 'E' || isDigit || ch === '-' || ch === '+') ? 1 : 0;

  scanFlags[c] = (escapeKind & 0x07) | ((mayTerminate & 0x01) << 3) | ((isNumberPart & 0x01) << 4);
}

// 6. 模拟 GetOneCharJsonToken(c) 生成 256 表
const oneCharTokens = [];
for (let c = 0; c < 256; c++) {
  const ch = String.fromCharCode(c);
  let tokenName = "ILLEGAL";
  if (ch === '"') tokenName = "STRING";
  else if ((c >= 0x30 && c <= 0x39) || ch === '-') tokenName = "NUMBER";
  else if (ch === '[') tokenName = "LBRACK";
  else if (ch === '{') tokenName = "LBRACE";
  else if (ch === ']') tokenName = "RBRACK";
  else if (ch === '}') tokenName = "RBRACE";
  else if (ch === 't') tokenName = "TRUE_LITERAL";
  else if (ch === 'f') tokenName = "FALSE_LITERAL";
  else if (ch === 'n') tokenName = "NULL_LITERAL";
  else if (ch === ' ' || ch === '\t' || ch === '\r' || ch === '\n') tokenName = "WHITESPACE";
  else if (ch === ':') tokenName = "COLON";
  else if (ch === ',') tokenName = "COMMA";

  oneCharTokens.push(toPascalCase(tokenName));
}

// 7. 生成 Rust 模块
let rust = banner("src/json/json-parser.h.js", "src/json/json-parser.{h,cc}", "JSON Token 体系与词法常量");

rust += `
/// JSON 词法 Token 枚举 (对应原版 enum class JsonToken : uint8_t, 共 ${tokens.length} 种)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum JsonToken {
`;

for (const t of tokens) {
  const ch = tokenCharMap.get(t.name);
  const chDoc = ch ? ` (单字符 \`${ch}\`)` : "";
  rust += `  /// ${t.name}${chDoc}\n  ${t.rustName} = ${t.repr},\n`;
}

rust += `}

impl JsonToken {
  /// Token 总数
  pub const COUNT: usize = ${tokens.length};

  /// 对应单字符字面量 (原版 JsonTokenToCharacter)
  #[inline]
  #[must_use]
  pub const fn to_char(self) -> Option<char> {
    match self {
`;

for (const t of tokens) {
  const ch = tokenCharMap.get(t.name);
  if (ch) {
    const escapedCh = ch === '\\' ? "'\\\\'" : ch === "'" ? "'\\''" : `'${ch}'`;
    rust += `      Self::${t.rustName} => Some(${escapedCh}),\n`;
  }
}

rust += `      _ => None,
    }
  }

  /// 从数值反查 Token
  #[inline]
  #[must_use]
  pub const fn from_repr(val: u8) -> Option<Self> {
    match val {
`;

for (const t of tokens) {
  rust += `      ${t.repr} => Some(Self::${t.rustName}),\n`;
}

rust += `      _ => None,
    }
  }
}

/// JSON 转义字符类别 (对应原版 enum class EscapeKind : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum EscapeKind {
`;

for (const k of escapeKinds) {
  rust += `  /// ${k.raw}\n  ${k.rustName} = ${k.repr},\n`;
}

rust += `}

impl EscapeKind {
  /// 从数值反查 EscapeKind
  #[inline]
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    match val & 0x07 {
`;

for (const k of escapeKinds) {
  rust += `      ${k.repr} => Self::${k.rustName},\n`;
}

rust += `      _ => Self::Illegal,
    }
  }
}

/// 字符串终止哨兵 (原版 kEndOfString)
pub const END_OF_STRING: u32 = u32::MAX;

/// 非法 Unicode 字符哨兵 (原版 kInvalidUnicodeCharacter)
pub const INVALID_UNICODE_CHARACTER: u32 = u32::MAX;

/// 错误上下文显示最大字符数 (原版 kMaxContextCharacters)
pub const MAX_CONTEXT_CHARACTERS: usize = ${maxContext};

/// 带有上下文的最小原始源长度 (原版 kMinOriginalSourceLengthForContext)
pub const MIN_ORIGINAL_SOURCE_LENGTH_FOR_CONTEXT: usize = ${minContext};

/// 初始特殊字符串长度 (原版 kInitialSpecialStringLength)
pub const INITIAL_SPECIAL_STRING_LENGTH: usize = ${initSpecialStr};

/// 单字符 Token 查询表 (原版 one_char_json_tokens[256])
#[rustfmt::skip]
pub static ONE_CHAR_JSON_TOKENS: [JsonToken; 256] = [
`;

for (let i = 0; i < 256; i += 8) {
  const slice = oneCharTokens.slice(i, i + 8).map((name) => `JsonToken::${name}`);
  rust += `  ${slice.join(", ")},\n`;
}

rust += `];

/// 字符扫描标志位表 (原版 character_json_scan_flags[256])
#[rustfmt::skip]
pub static CHARACTER_JSON_SCAN_FLAGS: [u8; 256] = [
`;

for (let i = 0; i < 256; i += 16) {
  const slice = Array.from(scanFlags.slice(i, i + 16)).map((v) => `0x${v.toString(16).padStart(2, "0")}`);
  rust += `  ${slice.join(", ")},\n`;
}

rust += `];

/// 判定是否可能终止 JSON 字符串 (原版 MayTerminateJsonString)
#[inline]
#[must_use]
pub const fn may_terminate_json_string(flags: u8) -> bool {
  (flags & (1 << 3)) != 0
}

/// 获取转义类型 (原版 GetEscapeKind)
#[inline]
#[must_use]
pub const fn get_escape_kind(flags: u8) -> EscapeKind {
  EscapeKind::from_repr(flags & 0x07)
}

/// 判定字符是否为数字组成部分 (原版 IsNumberPart)
#[inline]
#[must_use]
pub const fn is_number_part(flags: u8) -> bool {
  (flags & (1 << 4)) != 0
}

/// 查询单个 ASCII 字符对应的 JsonToken
#[inline]
#[must_use]
pub const fn get_one_char_json_token(c: u8) -> JsonToken {
  ONE_CHAR_JSON_TOKENS[c as usize]
}

/// 查询单个字符的扫描标志
#[inline]
#[must_use]
pub const fn get_character_json_scan_flags(c: u8) -> u8 {
  CHARACTER_JSON_SCAN_FLAGS[c as usize]
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(JsonToken::COUNT == 14);
  assert!(MAX_CONTEXT_CHARACTERS == 10);
  assert!(MIN_ORIGINAL_SOURCE_LENGTH_FOR_CONTEXT == 21);
  assert!(INITIAL_SPECIAL_STRING_LENGTH == 32);
  assert!(ONE_CHAR_JSON_TOKENS[b'"' as usize] as u8 == JsonToken::String as u8);
  assert!(ONE_CHAR_JSON_TOKENS[b'{' as usize] as u8 == JsonToken::Lbrace as u8);
  assert!(ONE_CHAR_JSON_TOKENS[b'}' as usize] as u8 == JsonToken::Rbrace as u8);
  assert!(ONE_CHAR_JSON_TOKENS[b'[' as usize] as u8 == JsonToken::Lbrack as u8);
  assert!(ONE_CHAR_JSON_TOKENS[b']' as usize] as u8 == JsonToken::Rbrack as u8);
  assert!(ONE_CHAR_JSON_TOKENS[b'0' as usize] as u8 == JsonToken::Number as u8);
  assert!(ONE_CHAR_JSON_TOKENS[b'-' as usize] as u8 == JsonToken::Number as u8);
  assert!(ONE_CHAR_JSON_TOKENS[b' ' as usize] as u8 == JsonToken::Whitespace as u8);
  assert!(ONE_CHAR_JSON_TOKENS[b':' as usize] as u8 == JsonToken::Colon as u8);
  assert!(ONE_CHAR_JSON_TOKENS[b',' as usize] as u8 == JsonToken::Comma as u8);
};
`;

const p = writeSrc("v8-json", "token.rs", rust);
console.log(`written ${p}`);
