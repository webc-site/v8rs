#!/usr/bin/env bun
// tool/src/strings/unicode.h.js — 覆盖 v8/src/strings/unicode.{h,cc,-inl.h}
//        + third_party/utf8-decoder/{utf8-decoder.h,generalized-utf8-decoder.h}
//        → v8-strings/src/unicode.rs + utf8_dfa.rs
// 规则部分: DFA transitions/states 两表从 third_party 源码逐数字提取;
//           unicode.h 常量组 / Utf8Variant / Encoding 枚举 1:1; 语义函数体随脚本产出
import { banner, readV8, writeSrc } from "../../lib.js";

// ---- 从 third_party DFA 源提取 transitions / states 表 ----
function extractTables(rel) {
  const src = readV8(rel);
  const grab = (name) => {
    const i = src.indexOf(`${name}[] = {`);
    if (i < 0) throw new Error(`${name} not found in ${rel}`);
    const j = src.indexOf("};", i);
    // 先剥离注释 (行内十六进制样注释如 "00-0F" 会混入十进制数字)
    const clean = src.slice(i, j).replace(/\/\/[^\n]*/g, "");
    return clean.split(/[^0-9]+/).filter(Boolean).map(Number);
  };
  return { transitions: grab("transitions"), states: grab("states") };
}

function fmtTable(nums, per) {
  const rows = [];
  for (let i = 0; i < nums.length; i += per) {
    rows.push("    " + nums.slice(i, i + per).map((v) => String(v)).join(", ") + ",");
  }
  return rows.join("\n");
}

const strict = extractTables("third_party/utf8-decoder/utf8-decoder.h");
const generalized = extractTables("third_party/utf8-decoder/generalized-utf8-decoder.h");
if (strict.transitions.length !== 256 || strict.states.length !== 108) {
  throw new Error(`utf8-decoder.h 表长异常: ${strict.transitions.length}/${strict.states.length}`);
}
if (generalized.transitions.length !== 256 || generalized.states.length !== 88) {
  throw new Error(`generalized 表长异常: ${generalized.transitions.length}/${generalized.states.length}`);
}

// ---- utf8_dfa.rs ----
const dfa = `${banner("src/strings/unicode.h.js", "third_party/utf8-decoder/utf8-decoder.h + generalized-utf8-decoder.h", "DFA 两表逐数字提取 (Bjoern Hoehrmann DFA); 语义绑定, 禁库替换")}
//! UTF-8 / generalized-UTF-8 (WTF-8) DFA 解码器。
//! 状态转移表与原版逐字节一致; \`Decode\` 单步推进, buffer 累积码点。

/// 严格 UTF-8 DFA (拒绝代理区与超长编码)。
pub mod strict {
  /// DFA 状态 (枚举值 = states 表行基址)。
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  #[repr(u8)]
  pub enum State {
    Reject = 0,
    Accept = 12,
    TwoByte = 24,
    ThreeByte = 36,
    ThreeByteLowMid = 48,
    FourByte = 60,
    FourByteLow = 72,
    ThreeByteHigh = 84,
    FourByteMidHigh = 96,
  }

  /// 字节 → 转移类型。
  static TRANSITIONS: [u8; 256] = [
${fmtTable(strict.transitions, 16)}
  ];

  /// (状态, 转移类型) → 新状态。
  static STATES: [u8; 108] = [
${fmtTable(strict.states, 12)}
  ];

  /// 单步解码: 推进 state, 累积 buffer。
  #[inline]
  pub fn decode(byte: u8, state: &mut State, buffer: &mut u32) {
    let t = TRANSITIONS[usize::from(byte)];
    *state = State::from_table(STATES[usize::from(*state as u8) + usize::from(t)]);
    *buffer = (*buffer << 6) | u32::from(byte & (0x7F >> (t >> 1)));
  }

  impl State {
    /// states 表产出值 → 状态; 表值域恰为枚举基址, 兜底 Reject (不可达)。
    fn from_table(v: u8) -> State {
      match v {
        12 => State::Accept,
        24 => State::TwoByte,
        36 => State::ThreeByte,
        48 => State::ThreeByteLowMid,
        60 => State::FourByte,
        72 => State::FourByteLow,
        84 => State::ThreeByteHigh,
        96 => State::FourByteMidHigh,
        _ => State::Reject,
      }
    }
  }
}

/// 泛化 UTF-8 (WTF-8) DFA: 允许孤立代理, 拒绝代理对由外层捕获。
pub mod generalized {
  /// DFA 状态 (枚举值 = states 表行基址)。
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  #[repr(u8)]
  pub enum State {
    Reject = 0,
    Accept = 11,
    TwoByte = 22,
    ThreeByte = 33,
    FourByte = 44,
    FourByteLow = 55,
    ThreeByteHigh = 66,
    FourByteMidHigh = 77,
  }

  /// 字节 → 转移类型。
  static TRANSITIONS: [u8; 256] = [
${fmtTable(generalized.transitions, 16)}
  ];

  /// (状态, 转移类型) → 新状态。
  static STATES: [u8; 88] = [
${fmtTable(generalized.states, 11)}
  ];

  /// 单步解码: 推进 state, 累积 buffer。
  #[inline]
  pub fn decode(byte: u8, state: &mut State, buffer: &mut u32) {
    let t = TRANSITIONS[usize::from(byte)];
    *state = State::from_table(STATES[usize::from(*state as u8) + usize::from(t)]);
    *buffer = (*buffer << 6) | u32::from(byte & (0x7F >> (t >> 1)));
  }

  impl State {
    /// states 表产出值 → 状态; 表值域恰为枚举基址, 兜底 Reject (不可达)。
    fn from_table(v: u8) -> State {
      match v {
        11 => State::Accept,
        22 => State::TwoByte,
        33 => State::ThreeByte,
        44 => State::FourByte,
        55 => State::FourByteLow,
        66 => State::ThreeByteHigh,
        77 => State::FourByteMidHigh,
        _ => State::Reject,
      }
    }
  }
}
`;

// ---- unicode.rs ----
const unicode = `${banner("src/strings/unicode.h.js", "src/strings/unicode.h + unicode.cc + unicode-inl.h", "V8_INTL_SUPPORT 路径 (上游默认); Uppercase/Letter 由 icu_properties 承接, 其余自写")}
//! unibrow 命名空间的 Unicode 工具: Utf16 代理处理 / Utf8 编解码 / Wtf8 校验。
//! 单个字符大小写转换结果的最大长度 (kMaxMappingSize)。


/// unibrow::uchar — Unicode 码点。
pub type Uchar = u32;

/// 单字符大小写转换结果的最大长度。
pub const K_MAX_MAPPING_SIZE: i32 = 4;

/// UTF-16 代理对工具 (unibrow::Utf16)。
pub mod utf16 {
  pub const NO_PREVIOUS_CHARACTER: i32 = -1;
  pub const MAX_NON_SURROGATE_CHAR_CODE: u32 = 0xffff;
  /// 单个 UTF-16 码单元最多产出 3 字节 UTF-8 (代理尾码单元净增 1 字节)。
  pub const MAX_EXTRA_UTF8_BYTES_FOR_ONE_UTF16_CODE_UNIT: i32 = 3;
  /// 单个 (非法的) 孤立代理编码为 3 字节 UTF-8。
  pub const UTF8_BYTES_TO_CODE_A_SURROGATE: i32 = 3;

  #[inline]
  #[must_use]
  pub fn is_surrogate_pair(lead: i32, trail: i32) -> bool {
    is_lead_surrogate(lead) && is_trail_surrogate(trail)
  }

  #[inline]
  #[must_use]
  pub fn is_lead_surrogate(code: i32) -> bool {
    (code & 0x1ffc00) == 0xd800
  }

  #[inline]
  #[must_use]
  pub fn is_trail_surrogate(code: i32) -> bool {
    (code & 0x1ffc00) == 0xdc00
  }

  /// 合并代理对为标量值。
  #[inline]
  #[must_use]
  pub fn combine_surrogate_pair(lead: u32, trail: u32) -> u32 {
    0x10000 + ((lead & 0x3ff) << 10) + (trail & 0x3ff)
  }

  #[inline]
  #[must_use]
  pub fn lead_surrogate(char_code: u32) -> u16 {
    (0xd800 + (((char_code - 0x10000) >> 10) & 0x3ff)) as u16
  }

  #[inline]
  #[must_use]
  pub fn trail_surrogate(char_code: u32) -> u16 {
    (0xdc00 + (char_code & 0x3ff)) as u16
  }

  /// 是否存在未配对代理 (simdutf::validate_utf16 语义): lead 必须紧跟 trail, trail 必须有 lead。
  #[must_use]
  pub fn has_unpaired_surrogate(code_units: &[u16]) -> bool {
    let mut i = 0;
    while i < code_units.len() {
      let c = code_units[i];
      if (0xd800..0xdc00).contains(&c) {
        // lead: 下一个必须是 trail
        if i + 1 >= code_units.len() || !(0xdc00..0xe000).contains(&code_units[i + 1]) {
          return true;
        }
        i += 2;
      } else if (0xdc00..0xe000).contains(&c) {
        // trail 无前置 lead
        return true;
      } else {
        i += 1;
      }
    }
    false
  }

  /// 将未配对代理替换为 U+FFFD (simdutf::to_well_formed_utf16 语义)。
  pub fn replace_unpaired_surrogates(src: &[u16], dst: &mut [u16]) {
    let limit = src.len().min(dst.len());
    let mut i = 0;
    while i < limit {
      let c = src[i];
      if (0xd800..0xdc00).contains(&c) {
        if i + 1 < limit && (0xdc00..0xe000).contains(&src[i + 1]) {
          dst[i] = c;
          dst[i + 1] = src[i + 1];
          i += 2;
          continue;
        }
        dst[i] = crate::unicode::utf8::BAD_CHAR as u16;
      } else if (0xdc00..0xe000).contains(&c) {
        dst[i] = crate::unicode::utf8::BAD_CHAR as u16;
      } else {
        dst[i] = c;
      }
      i += 1;
    }
  }
}

/// Latin-1 范围常量 (unibrow::Latin1)。
pub mod latin1 {
  pub const MAX_CHAR: u16 = 0xff;
}

/// UTF-8 变体 (unibrow::Utf8Variant)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Utf8Variant {
  /// 严格 UTF-8: 非法序列或代理码点 → 错误。
  Utf8,
  /// UTF-8: 非法序列或代理码点 → 返回 None。
  Utf8NoTrap,
  /// WTF-8: 允许孤立 (但非成对) 代理。
  Wtf8,
  /// 有损 UTF-8: 非法字节替换为 U+FFFD; 编码时代理替换为 U+FFFD。
  LossyUtf8,
}

/// UTF-8 编解码 (unibrow::Utf8)。
pub mod utf8 {
  use super::utf16;
  use crate::utf8_dfa::strict::State;

  pub type IncrementalBuffer = u32;

  /// 替换字符, 标记非法 Unicode 序列 (如孤立代理)。
  pub const BAD_CHAR: u32 = 0xFFFD;
  pub const BUFFER_EMPTY: u32 = 0x0;
  /// 非法码点哨兵 (任何有效码点之外)。
  pub const INCOMPLETE: u32 = 0xFFFFFFFC;
  pub const MAX_ENCODED_SIZE: u32 = 4;
  pub const MAX_ONE_BYTE_CHAR: u32 = 0x7f;
  pub const MAX_TWO_BYTE_CHAR: u32 = 0x7ff;
  pub const MAX_THREE_BYTE_CHAR: u32 = 0xffff;
  pub const MAX_FOUR_BYTE_CHAR: u32 = 0x1fffff;
  /// 两个配对代理合并编码比孤立编码节省的字节数。
  pub const BYTES_SAVED_BY_COMBINING_SURROGATES: u32 = 2;
  pub const SIZE_OF_UNMATCHED_SURROGATE: u32 = 3;
  /// 单个 UTF-16 码单元 UTF-8 编码的最大长度。
  pub const MAX_16_BIT_CODE_UNIT_SIZE: u32 = 3;
  /// [0,0xff] 内码单元 UTF-8 编码的最大长度。
  pub const MAX_8_BIT_CODE_UNIT_SIZE: u32 = 2;

  const K_MASK: u32 = !(1 << 6);

  /// 单字节 (Latin-1) 字符的 UTF-8 编码长度。
  #[inline]
  #[must_use]
  pub fn length_one_byte(c: u8) -> u32 {
    if u32::from(c) <= MAX_ONE_BYTE_CHAR { 1 } else { 2 }
  }

  /// 码点 c 的 UTF-8 编码长度; previous 为前一码点 (代理合并时少 2 字节)。
  #[inline]
  #[must_use]
  pub fn length(c: u32, previous: i32) -> u32 {
    if c <= MAX_ONE_BYTE_CHAR {
      1
    } else if c <= MAX_TWO_BYTE_CHAR {
      2
    } else if c <= MAX_THREE_BYTE_CHAR {
      if utf16::is_surrogate_pair(previous, c as i32) {
        SIZE_OF_UNMATCHED_SURROGATE - BYTES_SAVED_BY_COMBINING_SURROGATES
      } else {
        3
      }
    } else {
      4
    }
  }

  /// 单字节字符编码 (Latin-1 → UTF-8), 返回写入字节数。
  #[inline]
  pub fn encode_one_byte(out: &mut [u8], c: u8) -> u32 {
    if u32::from(c) <= MAX_ONE_BYTE_CHAR {
      out[0] = c;
      1
    } else {
      out[0] = 0xC0 | (c >> 6);
      out[1] = 0x80 | (c & K_MASK as u8);
      2
    }
  }

  /// 编码 c (与前一码点 previous 合并代理对); replace_invalid 时孤立代理替换为 U+FFFD。
  /// 返回写入字节数; 代理合并语义下可能回写 previous 已占用的 3 字节。
  #[inline]
  pub fn encode(out: &mut [u8], mut c: u32, previous: i32, replace_invalid: bool) -> u32 {
    if c <= MAX_ONE_BYTE_CHAR {
      out[0] = c as u8;
      1
    } else if c <= MAX_TWO_BYTE_CHAR {
      out[0] = (0xC0 | (c >> 6)) as u8;
      out[1] = (0x80 | (c & K_MASK)) as u8;
      2
    } else if c <= MAX_THREE_BYTE_CHAR {
      if utf16::is_surrogate_pair(previous, c as i32) {
        // 合并代理: 调用方传入的 out 须自 write_index-3 起 (覆盖 previous 的 3 字节孤立编码)
        let combined = utf16::combine_surrogate_pair(previous as u32, c);
        let written = encode(out, combined, utf16::NO_PREVIOUS_CHARACTER, replace_invalid);
        return written - SIZE_OF_UNMATCHED_SURROGATE;
      } else if replace_invalid && (utf16::is_lead_surrogate(c as i32) || utf16::is_trail_surrogate(c as i32)) {
        c = BAD_CHAR;
      }
      out[0] = (0xE0 | (c >> 12)) as u8;
      out[1] = (0x80 | ((c >> 6) & K_MASK)) as u8;
      out[2] = (0x80 | (c & K_MASK)) as u8;
      3
    } else {
      out[0] = (0xF0 | (c >> 18)) as u8;
      out[1] = (0x80 | ((c >> 12) & K_MASK)) as u8;
      out[2] = (0x80 | ((c >> 6) & K_MASK)) as u8;
      out[3] = (0x80 | (c & K_MASK)) as u8;
      4
    }
  }

  /// 解码 bytes 的首个码点; cursor 前进相应字节数。
  /// 等价原版 ValueOf + CalculateValue。
  pub fn value_of(bytes: &[u8], cursor: &mut usize) -> u32 {
    if bytes.is_empty() {
      return BAD_CHAR;
    }
    let first = bytes[0];
    if u32::from(first) <= MAX_ONE_BYTE_CHAR {
      *cursor += 1;
      return u32::from(first);
    }
    // CalculateValue: DFA 循环 (Reject 时 state 被重置, 由终态判定兜底)
    let mut state = State::Accept;
    let mut buffer = 0u32;
    let mut pos = 0usize;
    let mut t;
    loop {
      t = value_of_incremental(bytes, &mut pos, &mut state, &mut buffer);
      if pos >= bytes.len() || t != INCOMPLETE {
        break;
      }
    }
    *cursor += pos;
    if state == State::Accept { t } else { BAD_CHAR }
  }

  /// 增量解码单字节 (原版 ValueOfIncremental): cursor 前进一步。
  pub fn value_of_incremental(bytes: &[u8], cursor: &mut usize, state: &mut State, buffer: &mut u32) -> u32 {
    let old_state = *state;
    let next = bytes[*cursor];
    *cursor += 1;
    if u32::from(next) <= MAX_ONE_BYTE_CHAR && old_state == State::Accept {
      return u32::from(next);
    }
    crate::utf8_dfa::strict::decode(next, state, buffer);
    match *state {
      State::Accept => {
        let t = *buffer;
        *buffer = 0;
        t
      }
      State::Reject => {
        *state = State::Accept;
        *buffer = 0;
        // 序列中途遇坏字节: 回退一步让该字节按新序列重处理
        if old_state != State::Accept {
          *cursor -= 1;
        }
        BAD_CHAR
      }
      _ => INCOMPLETE,
    }
  }

  /// 流结束时收尾: 残留未完序列 → U+FFFD。
  pub fn value_of_incremental_finish(state: &mut State) -> u32 {
    if *state == State::Accept {
      BUFFER_EMPTY
    } else {
      *state = State::Accept;
      BAD_CHAR
    }
  }

  /// 排除非字符 (non-characters) 后的有效码点判断。
  #[inline]
  #[must_use]
  pub fn is_valid_character(c: u32) -> bool {
    c < 0xD800 || (0xE000..0xFDD0).contains(&c) ||
      (c > 0xFDEF && c <= 0x10FFFF && c & 0xFFFE != 0xFFFE && c != BAD_CHAR)
  }

  /// 校验 UTF-8 (simdutf::validate_utf8 语义: 拒绝超长/代理/越界)。
  #[must_use]
  pub fn validate_encoding(bytes: &[u8]) -> bool {
    let mut state = State::Accept;
    let mut buffer = 0u32;
    for &b in bytes {
      crate::utf8_dfa::strict::decode(b, &mut state, &mut buffer);
      if state == State::Reject {
        return false;
      }
    }
    state == State::Accept
  }

  /// 写入 src 的前导 ASCII 前缀到 dest, 返回写入长度; 首个非 ASCII 处停。
  /// 原版 SIMD 批量 + 短串阈值短路 (性能差异, 数值语义一致: 调用方逐字节兜底)。
  #[must_use]
  pub fn write_leading_ascii_one_byte(src: &[u8], dest: &mut [u8]) -> usize {
    let n = src
      .iter()
      .position(|&b| !b.is_ascii())
      .unwrap_or(src.len())
      .min(dest.len());
    dest[..n].copy_from_slice(&src[..n]);
    n
  }

  /// 双字节源的 ASCII 前缀写回 (原版 TODO 未实现, 恒 0)。
  #[must_use]
  pub fn write_leading_ascii_two_byte(_src: &[u16], _dest: &mut [u8]) -> usize {
    0
  }

  /// 编码结果。
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct EncodingResult {
    pub bytes_written: usize,
    pub characters_processed: usize,
  }
}

/// WTF-8 (unibrow::Wtf8)。
pub mod wtf8 {
  use super::utf16;
  use crate::utf8_dfa::generalized::{self as gdfa, State};

  /// 校验广义 UTF-8 且无代理对 (wasm 约束)。
  #[must_use]
  pub fn validate_encoding(bytes: &[u8]) -> bool {
    let mut state = State::Accept;
    let mut current = 0u32;
    let mut previous = 0u32;
    for &b in bytes {
      gdfa::decode(b, &mut state, &mut current);
      if state == State::Reject {
        return false;
      }
      if state == State::Accept {
        if utf16::is_trail_surrogate(current as i32) && utf16::is_lead_surrogate(previous as i32) {
          return false;
        }
        previous = current;
        current = 0;
      }
    }
    state == State::Accept
  }

  /// 前置条件: 合法 WTF-8。记录代理码点 (0xED [A0,BF] [80,BF] 三字节序列) 的字节偏移。
  pub fn scan_for_surrogates(wtf8: &[u8], surrogate_offsets: &mut Vec<usize>) {
    const FIRST: u8 = 0xED;
    const SECOND_HIGH_BIT: u8 = 0x20;
    for (i, &b) in wtf8.iter().enumerate() {
      if b == FIRST && wtf8.get(i + 1).is_some_and(|&n| n & SECOND_HIGH_BIT != 0) {
        surrogate_offsets.push(i);
      }
    }
  }
}

/// unibrow::Uppercase — 原版 u_isupper (Lu ∪ Other_Uppercase)。
pub mod uppercase {
  /// icu binary property Uppercase = u_isupper 语义 (含罗马数字等 Other_Uppercase)。
  #[must_use]
  pub fn is(c: u32) -> bool {
    let up = icu_properties::CodePointSetData::new::<icu_properties::props::Uppercase>();
    char::from_u32(c).is_some_and(|ch| up.contains(ch))
  }
}

/// unibrow::Letter — 原版 u_isalpha (类别 L*)。
pub mod letter {
  use icu_properties::props::{GeneralCategory, GeneralCategoryGroup};
  use icu_properties::CodePointMapData;

  /// 类别 L* (Lu/Ll/Lt/Lm/Lo) = u_isalpha 语义。
  #[must_use]
  pub fn is(c: u32) -> bool {
    let gc = CodePointMapData::<GeneralCategory>::new();
    char::from_u32(c).is_some_and(|ch| GeneralCategoryGroup::Letter.contains(gc.get(ch)))
  }
}

/// 空白慢路径 (char-predicates.cc IsWhiteSpaceSlow 的 icu 承接): gC=Zs + U+0009/B/C + U+FEFF。
pub fn white_space_slow(c: u32) -> bool {
  use icu_properties::props::{GeneralCategory, GeneralCategoryGroup};
  use icu_properties::CodePointMapData;
  if char::from_u32(c).is_some_and(|ch| {
    let gc = CodePointMapData::<GeneralCategory>::new();
    GeneralCategoryGroup::SpaceSeparator.contains(gc.get(ch))
  }) {
    return true;
  }
  c < 0x0D && (c == 0x09 || c == 0x0B || c == 0x0C) || c == 0xFEFF
}

/// LineTerminator: LF / CR / LS(U+2028) / PS(U+2029)。
#[inline]
#[must_use]
pub fn is_line_terminator(c: u32) -> bool {
  c == 0x000A || c == 0x000D || c == 0x2028 || c == 0x2029
}

/// 字符串字面量内的行终止符 (仅 LF / CR)。
#[inline]
#[must_use]
pub fn is_string_literal_line_terminator(c: u32) -> bool {
  c == 0x000A || c == 0x000D
}

/// 大小写转换方向标记 (INTL 路径: 转换逻辑在 string_builder/objects 层)。
pub mod to_lowercase {
  pub const IS_TO_LOWER: bool = true;
}
pub mod to_uppercase {
  pub const IS_TO_LOWER: bool = false;
}
`;

writeSrc("v8-strings", "utf8_dfa.rs", dfa);
writeSrc("v8-strings", "unicode.rs", unicode);
console.log("written: utf8_dfa.rs, unicode.rs");
console.log(`strict dfa: transitions=${strict.transitions.length} states=${strict.states.length}`);
console.log(`generalized dfa: transitions=${generalized.transitions.length} states=${generalized.states.length}`);
