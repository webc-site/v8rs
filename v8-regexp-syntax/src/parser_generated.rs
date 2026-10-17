// 由 tool/src/regexp/regexp-parser.cc.js 从 v8/src/regexp/regexp-parser.cc 生成; 修订就地改, 复跑覆盖
// 语法字符集提取; 解析状态机在 parser.rs

/// Ecma-262 SyntaxCharacter (14 个, 升序; IsSyntaxCharacterOrSlash 再并 '/',
/// regexp-parser.cc:900-920 switch 15 case)
pub static SYNTAX_CHARS: [char; 14] = [
  '$', '(', ')', '*', '+', '.', '?', '[', ']', '^', '{', '|', '}', '\\',
];

/// IsClassSetSyntaxCharacter (regexp-parser.cc:926-945, 10 个, 升序)
pub static CLASS_SET_SYNTAX_CHARS: [char; 10] = ['(', ')', '-', '/', '[', ']', '{', '|', '}', '\\'];

/// IsClassSetReservedPunctuator (regexp-parser.cc:947-967, 14 个, 升序)
pub static CLASS_SET_RESERVED_PUNCTUATORS: [char; 14] = [
  '!', '#', '%', '&', ',', '-', ':', ';', '<', '=', '>', '@', '`', '~',
];

/// IsClassSetReservedDoublePunctuator (regexp-parser.cc:970-998, 19 个, 升序)
pub static CLASS_SET_DOUBLE_PUNCTUATORS: [char; 19] = [
  '!', '#', '$', '%', '&', '*', '+', ',', '.', ':', ';', '<', '=', '>', '?', '@', '^', '`', '~',
];

/// 字符数组 → 128 位位图 (const 求值, c < 128 全体成立)
const fn chars_mask(chars: &[char]) -> u128 {
  let mut m = 0u128;
  let mut i = 0;
  while i < chars.len() {
    m |= 1u128 << (chars[i] as u32);
    i += 1;
  }
  m
}

/// IsSyntaxCharacterOrSlash 位图 (SyntaxCharacter ∪ {'/'}, 15 字符)
pub const SYNTAX_CHARS_OR_SLASH_MASK: u128 = chars_mask(&SYNTAX_CHARS) | (1u128 << b'/' as u32);

/// IsClassSetSyntaxCharacter 位图 (10 字符)
pub const CLASS_SET_SYNTAX_CHARS_MASK: u128 = chars_mask(&CLASS_SET_SYNTAX_CHARS);

/// IsClassSetReservedPunctuator 位图 (14 字符)
pub const CLASS_SET_RESERVED_PUNCTUATORS_MASK: u128 = chars_mask(&CLASS_SET_RESERVED_PUNCTUATORS);

/// IsClassSetReservedDoublePunctuator 位图 (19 字符)
pub const CLASS_SET_DOUBLE_PUNCTUATORS_MASK: u128 = chars_mask(&CLASS_SET_DOUBLE_PUNCTUATORS);

// ── 对数断言: 集合实数 == 原版 case 数 ──
const _: () = assert!(SYNTAX_CHARS.len() == 14);
const _: () = assert!(CLASS_SET_SYNTAX_CHARS.len() == 10);
const _: () = assert!(CLASS_SET_RESERVED_PUNCTUATORS.len() == 14);
const _: () = assert!(CLASS_SET_DOUBLE_PUNCTUATORS.len() == 19);
const _: () = assert!(SYNTAX_CHARS_OR_SLASH_MASK.count_ones() == 15);
const _: () = assert!(CLASS_SET_SYNTAX_CHARS_MASK.count_ones() == 10);
const _: () = assert!(CLASS_SET_RESERVED_PUNCTUATORS_MASK.count_ones() == 14);
const _: () = assert!(CLASS_SET_DOUBLE_PUNCTUATORS_MASK.count_ones() == 19);
