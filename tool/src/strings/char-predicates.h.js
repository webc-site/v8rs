#!/usr/bin/env bun
// tool/src/strings/char-predicates.h.js — v8/src/strings/char-predicates.{h,cc,-inl.h} → v8-strings/src/char_predicates.rs
// 规则部分: OneByteCharFlags 位标志 + kOneByteCharFlags[256] 表 (js 重算 BuildOneByteCharFlags);
// 语义部分 (icu_properties 承接慢路径 + 快速路径分派) 由脚本直接产出
import { banner, readV8, writeSrc } from "../../lib.js";

const cpH = readV8("src/strings/char-predicates.h");
const cpInl = readV8("src/strings/char-predicates-inl.h");
if (!cpH.includes("IsIdentifierStart") || !cpInl.includes("BuildOneByteCharFlags")) {
  throw new Error("char-predicates.h / char-predicates-inl.h 结构漂移");
}

// ---- 原版 BuildOneByteCharFlags 语义在 js 里重算 (char-predicates-inl.h:118-133) ----
const K_IS_ID_START = 1 << 0;
const K_IS_ID_PART = 1 << 1;
const K_IS_WHITE_SPACE = 1 << 2;
const K_IS_WS_OR_LT = 1 << 3;
const K_MAYBE_LINE_END = 1 << 4;

function isOneByteIDStart(c) {
  return c === 0x24 || (c >= 0x41 && c <= 0x5a) || c === 0x5f || (c >= 0x61 && c <= 0x7a) ||
    c === 0xaa || c === 0xb5 || c === 0xba || (c >= 0xc0 && c <= 0xd6) ||
    (c >= 0xd8 && c <= 0xf6) || (c >= 0xf8 && c <= 0xff);
}
function isOneByteIDContinue(c) {
  return c === 0x24 || (c >= 0x30 && c <= 0x39) || c === 0x5f || (c >= 0x41 && c <= 0x5a) ||
    (c >= 0x61 && c <= 0x7a) || c === 0xaa || c === 0xb5 || c === 0xb7 || c === 0xba ||
    (c >= 0xc0 && c <= 0xd6) || (c >= 0xd8 && c <= 0xf6) || (c >= 0xf8 && c <= 0xff);
}
function isOneByteWhitespace(c) {
  return c === 0x09 || c === 0x0b || c === 0x0c || c === 0x20 || c === 0xa0;
}
function buildOneByteCharFlags(c) {
  let r = 0;
  if (isOneByteIDStart(c) || c === 0x5c /* \\ */) r |= K_IS_ID_START;
  if (isOneByteIDContinue(c) || c === 0x5c) r |= K_IS_ID_PART;
  if (isOneByteWhitespace(c)) r |= K_IS_WHITE_SPACE | K_IS_WS_OR_LT;
  if (c === 0x0d || c === 0x0a) r |= K_IS_WS_OR_LT | K_MAYBE_LINE_END;
  // 原版: 标记 0x2028/0x2029 — 单字节下 static_cast<uint8_t> 截断后两者都 == 0x28
  if ((c & 0xff) === (0x2028 & 0xff) || (c & 0xff) === (0x2029 & 0xff)) r |= K_MAYBE_LINE_END;
  return r;
}
const table = Array.from({ length: 256 }, (_, i) => buildOneByteCharFlags(i));

const rows = [];
for (let i = 0; i < 256; i += 16) {
  rows.push("    " + table.slice(i, i + 16).map((v) => String(v)).join(", ") + ",");
}

const body = `${banner("src/strings/char-predicates.h.js", "src/strings/char-predicates.h + char-predicates.cc + char-predicates-inl.h", "V8_INTL_SUPPORT 路径 (上游默认): 慢路径由 icu_properties 承接, 快速路径表 1:1")}
//! ECMA-262 字符谓词 (词法分析用)。
//! 快速路径: \`c <= 0xFF\` 查 \`K_ONE_BYTE_CHAR_FLAGS\`; 慢路径 \`icu_properties\` (IdStart/IdContinue/White_Space 集)。

use crate::unicode::{is_line_terminator, white_space_slow};

/// 单字节字符标志位 (OneByteCharFlags)。
pub const K_IS_IDENTIFIER_START: u8 = 1 << 0;
pub const K_IS_IDENTIFIER_PART: u8 = 1 << 1;
pub const K_IS_WHITE_SPACE: u8 = 1 << 2;
pub const K_IS_WHITE_SPACE_OR_LINE_TERMINATOR: u8 = 1 << 3;
pub const K_MAYBE_LINE_END: u8 = 1 << 4;

/// constexpr 缓存表 (kOneByteCharFlags), 256 项 — 与原版逐项一致。
pub static K_ONE_BYTE_CHAR_FLAGS: [u8; 256] = [
${rows.join("\n")}
];

/// c 为 'A'-'Z' 或 'a'-'z' 时返回其小写形态, 否则返回字母区外的值; 忽略 LOCALE。
#[inline]
#[must_use]
pub fn ascii_alpha_to_lower(c: u32) -> u32 {
  c | 0x20
}

#[inline]
#[must_use]
pub fn is_carriage_return(c: u32) -> bool {
  c == 0x000D
}

#[inline]
#[must_use]
pub fn is_line_feed(c: u32) -> bool {
  c == 0x000A
}

/// ASCII 标识符字符: 字母数字 / '$' / '_'。
#[inline]
#[must_use]
pub fn is_ascii_identifier(c: u32) -> bool {
  is_alpha_numeric(c) || c == u32::from(b'$') || c == u32::from(b'_')
}

#[inline]
#[must_use]
pub fn is_alpha_numeric(c: u32) -> bool {
  (u32::from(b'a')..=u32::from(b'z')).contains(&ascii_alpha_to_lower(c)) || is_decimal_digit(c)
}

/// ECMA-262 3rd 7.8.3。
#[inline]
#[must_use]
pub fn is_decimal_digit(c: u32) -> bool {
  (u32::from(b'0')..=u32::from(b'9')).contains(&c)
}

/// 字符是否为十六进制数字 ('0'-'9' / 'a'-'f' / 'A'-'F')。
#[inline]
#[must_use]
pub fn is_hex_digit(c: u32) -> bool {
  is_decimal_digit(c) || (u32::from(b'a')..=u32::from(b'f')).contains(&ascii_alpha_to_lower(c))
}

/// 字符是否为八进制数字 ('0'-'7')。
#[inline]
#[must_use]
pub fn is_octal_digit(c: u32) -> bool {
  (u32::from(b'0')..=u32::from(b'7')).contains(&c)
}

/// 非八进制十进制数字 ('8' / '9')。
#[inline]
#[must_use]
pub fn is_non_octal_decimal_digit(c: u32) -> bool {
  (u32::from(b'8')..=u32::from(b'9')).contains(&c)
}

/// 字符是否为二进制数字 ('0' / '1')。
#[inline]
#[must_use]
pub fn is_binary_digit(c: u32) -> bool {
  c == u32::from(b'0') || c == u32::from(b'1')
}

#[inline]
#[must_use]
pub fn is_ascii(c: u32) -> bool {
  c & !0x7F == 0
}

#[inline]
#[must_use]
pub fn is_ascii_lower(c: u8) -> bool {
  c.is_ascii_lowercase()
}

#[inline]
#[must_use]
pub fn is_ascii_upper(c: u8) -> bool {
  c.is_ascii_uppercase()
}

/// 十六进制字符转数值 (0-15); 非十六进制字符返回 None。
#[inline]
#[must_use]
pub fn hex_value(c: u32) -> Option<u32> {
  if (u32::from(b'0')..=u32::from(b'9')).contains(&c) {
    Some(c - u32::from(b'0'))
  } else if (u32::from(b'a')..=u32::from(b'f')).contains(&ascii_alpha_to_lower(c)) {
    Some(ascii_alpha_to_lower(c) - u32::from(b'a') + 10)
  } else {
    None
  }
}

/// 二进制字符转数值 (0-1); 非 '0'/'1' 返回 None。
#[inline]
#[must_use]
pub fn binary_value(c: u32) -> Option<u32> {
  if (u32::from(b'0')..=u32::from(b'1')).contains(&c) {
    Some(c - u32::from(b'0'))
  } else {
    None
  }
}

/// 八进制字符转数值 (0-7); 非八进制字符返回 None。
#[inline]
#[must_use]
pub fn octal_value(c: u32) -> Option<u32> {
  if (u32::from(b'0')..=u32::from(b'7')).contains(&c) {
    Some(c - u32::from(b'0'))
  } else {
    None
  }
}

/// 十进制字符转数值 (0-9); 非十进制字符返回 None。
#[inline]
#[must_use]
pub fn decimal_value(c: u32) -> Option<u32> {
  if (u32::from(b'0')..=u32::from(b'9')).contains(&c) {
    Some(c - u32::from(b'0'))
  } else {
    None
  }
}

/// u32 宽度的小写判断。
#[inline]
#[must_use]
pub fn is_ascii_lower_u32(c: u32) -> bool {
  (u32::from(b'a')..=u32::from(b'z')).contains(&c)
}

/// u32 宽度的大写判断。
#[inline]
#[must_use]
pub fn is_ascii_upper_u32(c: u32) -> bool {
  (u32::from(b'A')..=u32::from(b'Z')).contains(&c)
}

#[inline]
#[must_use]
pub fn to_ascii_upper(c: u32) -> u32 {
  c & !(u32::from(is_ascii_lower_u32(c)) << 5)
}

#[inline]
#[must_use]
pub fn to_ascii_lower(c: u32) -> u32 {
  c | (u32::from(is_ascii_upper_u32(c)) << 5)
}

/// 正则 \\w 字符: 字母数字或 '_'。
#[inline]
#[must_use]
pub fn is_regexp_word(c: u32) -> bool {
  is_alpha_numeric(c) || c == u32::from(b'_')
}

/// https://tc39.es/ecma262/#sec-names-and-keywords
/// UnicodeIDStart + '$' + '_' + '\\\\'
pub fn is_identifier_start_slow(c: u32) -> bool {
  // 原版 (char-predicates.cc:18): u_hasBinaryProperty(c, UCHAR_ID_START) ||
  //   (c < 0x60 && (c == '$' || c == '\\\\' || c == '_'))
  // icu_properties 的 IdStart 集 = ICU UCHAR_ID_START (含 Other_ID_Start, 排除 Pattern_Syntax)
  let id_start = icu_properties::CodePointSetData::new::<icu_properties::props::IdStart>();
  char::from_u32(c).is_some_and(|ch| id_start.contains(ch)) ||
    (c < 0x60 && (c == u32::from(b'$') || c == u32::from(b'\\\\') || c == u32::from(b'_')))
}

/// https://tc39.es/ecma262/#sec-names-and-keywords
/// UnicodeIDContinue + '$' + '_' + '\\\\' + ZWNJ(U+200C) + ZWJ(U+200D)
pub fn is_identifier_part_slow(c: u32) -> bool {
  // 原版 (char-predicates.cc:27): u_hasBinaryProperty(c, UCHAR_ID_CONTINUE) || ... || ZWNJ || ZWJ
  let id_continue = icu_properties::CodePointSetData::new::<icu_properties::props::IdContinue>();
  char::from_u32(c).is_some_and(|ch| id_continue.contains(ch)) ||
    (c < 0x60 && (c == u32::from(b'$') || c == u32::from(b'\\\\') || c == u32::from(b'_'))) ||
    c == 0x200C || c == 0x200D
}

/// https://tc39.es/ecma262/#sec-white-space — gC=Zs, U+0009, U+000B, U+000C, U+FEFF
pub fn is_white_space_slow(c: u32) -> bool {
  // 原版 (char-predicates.cc:37): u_charType(c) == U_SPACE_SEPARATOR || ...
  white_space_slow(c)
}

/// ID_Start 快速路径: 单字节查表, 否则慢路径 (unicode-ident)。
#[inline]
#[must_use]
pub fn is_identifier_start(c: u32) -> bool {
  if let Ok(b) = u8::try_from(c) {
    return K_ONE_BYTE_CHAR_FLAGS[usize::from(b)] & K_IS_IDENTIFIER_START != 0;
  }
  is_identifier_start_slow(c)
}

/// ID_Continue 快速路径: 单字节查表, 否则慢路径 (unicode-ident)。
#[inline]
#[must_use]
pub fn is_identifier_part(c: u32) -> bool {
  if let Ok(b) = u8::try_from(c) {
    return K_ONE_BYTE_CHAR_FLAGS[usize::from(b)] & K_IS_IDENTIFIER_PART != 0;
  }
  is_identifier_part_slow(c)
}

/// WhiteSpace 快速路径: 单字节查表, 否则慢路径 (icu Zs)。
#[inline]
#[must_use]
pub fn is_white_space(c: u32) -> bool {
  if let Ok(b) = u8::try_from(c) {
    return K_ONE_BYTE_CHAR_FLAGS[usize::from(b)] & K_IS_WHITE_SPACE != 0;
  }
  is_white_space_slow(c)
}

/// WhiteSpace 或 LineTerminator (ES6 11.2/11.3: 类别 Z + U+0009-U+000D + U+FEFF)。
#[inline]
#[must_use]
pub fn is_white_space_or_line_terminator(c: u32) -> bool {
  if let Ok(b) = u8::try_from(c) {
    return K_ONE_BYTE_CHAR_FLAGS[usize::from(b)] & K_IS_WHITE_SPACE_OR_LINE_TERMINATOR != 0;
  }
  is_white_space_slow(c) || is_line_terminator(c)
}

/// 行终止符序列 (单字节字符串, 两字符窗口): 仅 \\n / \\r, \\r 后跟 \\n 记为一整个序列。
#[inline]
#[must_use]
pub fn is_line_terminator_sequence_one_byte(c: u8, next: u8) -> bool {
  if c == b'\\n' {
    return true;
  }
  if c == b'\\r' {
    return next != b'\\n';
  }
  false
}

/// 行终止符序列 (双字节字符串, 两字符窗口): \\n / \\r / U+2028 / U+2029。
#[inline]
#[must_use]
pub fn is_line_terminator_sequence_two_byte(c: u16, next: u16) -> bool {
  if K_ONE_BYTE_CHAR_FLAGS[usize::from(c & 0xFF)] & K_MAYBE_LINE_END != 0 {
    if c == u16::from(b'\\n') {
      return true;
    }
    if c == u16::from(b'\\r') {
      return next != u16::from(b'\\n');
    }
    return matches!(c, 0x2028..=0x2029);
  }
  false
}
`;

writeSrc("v8-strings", "char_predicates.rs", body);
console.log("written: char_predicates.rs");
