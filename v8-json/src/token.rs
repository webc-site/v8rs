// 由 tool/src/json/json-parser.h.js 从 v8/src/json/json-parser.{h,cc} 生成; 修订就地改, 复跑覆盖
// JSON Token 体系与词法常量

/// JSON 词法 Token 枚举 (对应原版 enum class JsonToken : uint8_t, 共 14 种)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum JsonToken {
  /// NUMBER
  Number = 0,
  /// STRING (单字符 `"`)
  String = 1,
  /// LBRACE (单字符 `{`)
  Lbrace = 2,
  /// RBRACE (单字符 `}`)
  Rbrace = 3,
  /// LBRACK (单字符 `[`)
  Lbrack = 4,
  /// RBRACK (单字符 `]`)
  Rbrack = 5,
  /// TRUE_LITERAL (单字符 `t`)
  TrueLiteral = 6,
  /// FALSE_LITERAL (单字符 `f`)
  FalseLiteral = 7,
  /// NULL_LITERAL (单字符 `n`)
  NullLiteral = 8,
  /// WHITESPACE
  Whitespace = 9,
  /// COLON (单字符 `:`)
  Colon = 10,
  /// COMMA (单字符 `,`)
  Comma = 11,
  /// ILLEGAL
  Illegal = 12,
  /// EOS
  Eos = 13,
}

impl JsonToken {
  /// Token 总数
  pub const COUNT: usize = 14;

  /// 对应单字符字面量 (原版 JsonTokenToCharacter)
  #[inline]
  #[must_use]
  pub const fn to_char(self) -> Option<char> {
    match self {
      Self::String => Some('"'),
      Self::Lbrace => Some('{'),
      Self::Rbrace => Some('}'),
      Self::Lbrack => Some('['),
      Self::Rbrack => Some(']'),
      Self::TrueLiteral => Some('t'),
      Self::FalseLiteral => Some('f'),
      Self::NullLiteral => Some('n'),
      Self::Colon => Some(':'),
      Self::Comma => Some(','),
      _ => None,
    }
  }

  /// 从数值反查 Token
  #[inline]
  #[must_use]
  pub const fn from_repr(val: u8) -> Option<Self> {
    match val {
      0 => Some(Self::Number),
      1 => Some(Self::String),
      2 => Some(Self::Lbrace),
      3 => Some(Self::Rbrace),
      4 => Some(Self::Lbrack),
      5 => Some(Self::Rbrack),
      6 => Some(Self::TrueLiteral),
      7 => Some(Self::FalseLiteral),
      8 => Some(Self::NullLiteral),
      9 => Some(Self::Whitespace),
      10 => Some(Self::Colon),
      11 => Some(Self::Comma),
      12 => Some(Self::Illegal),
      13 => Some(Self::Eos),
      _ => None,
    }
  }
}

/// JSON 转义字符类别 (对应原版 enum class EscapeKind : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum EscapeKind {
  /// kIllegal
  Illegal = 0,
  /// kSelf
  SelfEscape = 1,
  /// kBackspace
  Backspace = 2,
  /// kTab
  Tab = 3,
  /// kNewLine
  NewLine = 4,
  /// kFormFeed
  FormFeed = 5,
  /// kCarriageReturn
  CarriageReturn = 6,
  /// kUnicode
  Unicode = 7,
}

impl EscapeKind {
  /// 从数值反查 EscapeKind
  #[inline]
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    match val & 0x07 {
      0 => Self::Illegal,
      1 => Self::SelfEscape,
      2 => Self::Backspace,
      3 => Self::Tab,
      4 => Self::NewLine,
      5 => Self::FormFeed,
      6 => Self::CarriageReturn,
      7 => Self::Unicode,
      _ => Self::Illegal,
    }
  }
}

/// 字符串终止哨兵 (原版 kEndOfString)
pub const END_OF_STRING: u32 = u32::MAX;

/// 非法 Unicode 字符哨兵 (原版 kInvalidUnicodeCharacter)
pub const INVALID_UNICODE_CHARACTER: u32 = u32::MAX;

/// 错误上下文显示最大字符数 (原版 kMaxContextCharacters)
pub const MAX_CONTEXT_CHARACTERS: usize = 10;

/// 带有上下文的最小原始源长度 (原版 kMinOriginalSourceLengthForContext)
pub const MIN_ORIGINAL_SOURCE_LENGTH_FOR_CONTEXT: usize = 21;

/// 初始特殊字符串长度 (原版 kInitialSpecialStringLength)
pub const INITIAL_SPECIAL_STRING_LENGTH: usize = 32;

/// 单字符 Token 查询表 (原版 one_char_json_tokens[256])
#[rustfmt::skip]
pub static ONE_CHAR_JSON_TOKENS: [JsonToken; 256] = [
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Whitespace, JsonToken::Whitespace, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Whitespace, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Whitespace, JsonToken::Illegal, JsonToken::String, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Comma, JsonToken::Number, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Number, JsonToken::Number, JsonToken::Number, JsonToken::Number, JsonToken::Number, JsonToken::Number, JsonToken::Number, JsonToken::Number,
  JsonToken::Number, JsonToken::Number, JsonToken::Colon, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Lbrack, JsonToken::Illegal, JsonToken::Rbrack, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::FalseLiteral, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::NullLiteral, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::TrueLiteral, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Lbrace, JsonToken::Illegal, JsonToken::Rbrace, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
  JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal, JsonToken::Illegal,
];

/// 字符扫描标志位表 (原版 character_json_scan_flags[256])
#[rustfmt::skip]
pub static CHARACTER_JSON_SCAN_FLAGS: [u8; 256] = [
  0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08,
  0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08,
  0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x10, 0x10, 0x01,
  0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x02, 0x00, 0x00, 0x10, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00,
  0x00, 0x00, 0x06, 0x00, 0x03, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

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
