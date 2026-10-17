// 由 tool/src/strings/uri.h.js 从 v8/src/strings/uri.h + uri.cc 生成; 修订就地改, 复跑覆盖
// uri 字符集合 1:1; 纯算法内核 (Encode/Decode/Escape/Unescape) 语义体已就地补写

//! encodeURI / decodeURI / encodeURIComponent / decodeURIComponent / escape / unescape
//! 的纯算法内核。堆字符串外壳 (Flatten/NewStringFromOneByte/异常) 归 objects 层。

use crate::{
  char_predicates::is_alpha_numeric,
  unicode::{utf8, utf16},
};

/// decodeURI 保留字符 (IsReservedPredicate): # $ & + , / : ; = ? @
#[inline]
#[must_use]
pub fn is_reserved_predicate(c: u16) -> bool {
  matches!(
    c,
    0x23 | 0x24 | 0x26 | 0x2B | 0x2C | 0x2F | 0x3A | 0x3B | 0x3D | 0x3F | 0x40
  )
}

/// 是否 U+FFFD 的 UTF-8 编码 (EF BF BD) — IsReplacementCharacter。
#[inline]
#[must_use]
pub fn is_replacement_character(octets: &[u8]) -> bool {
  octets == [0xEF, 0xBF, 0xBD]
}

/// escape 不转义字符 (IsNotEscaped): 字母数字与 @*_+-./
#[inline]
#[must_use]
pub fn is_not_escaped(c: u16) -> bool {
  if is_alpha_numeric(u32::from(c)) {
    return true;
  }
  matches!(c, 0x40 | 0x2A | 0x5F | 0x2B | 0x2D | 0x2E | 0x2F)
}

/// encodeURIComponent 不转义字符 (IsUnescapePredicateInUriComponent): 字母数字与 !'()*-._~
#[inline]
#[must_use]
pub fn is_unescape_predicate_in_uri_component(c: u16) -> bool {
  if is_alpha_numeric(u32::from(c)) {
    return true;
  }
  matches!(
    c,
    0x21 | 0x27 | 0x28 | 0x29 | 0x2A | 0x2D | 0x2E | 0x5F | 0x7E
  )
}

/// encodeURI 额外保留的分隔符 (IsUriSeparator): # : ; / ? $ & + , @ =
#[inline]
#[must_use]
pub fn is_uri_separator(c: u16) -> bool {
  matches!(
    c,
    0x23 | 0x3A | 0x3B | 0x2F | 0x3F | 0x24 | 0x26 | 0x2B | 0x2C | 0x40 | 0x3D
  )
}

/// 两位十六进制 → 值; 非法返回 None (TwoDigitHex 的 -1)。
/// 原版仅接受小写区间判定: c > 'f' 短路 + HexValue ('0'-'9' / 'a'-'f')。
#[inline]
#[must_use]
pub fn two_digit_hex(c1: u16, c2: u16) -> Option<u8> {
  #[inline]
  fn hex(c: u16) -> Option<u8> {
    match u32::from(c) {
      d @ 0x30..=0x39 => Some((d - 0x30) as u8),
      a @ 0x41..=0x46 => Some((a - 0x41 + 10) as u8),
      a @ 0x61..=0x66 => Some((a - 0x61 + 10) as u8),
      _ => None,
    }
  }
  Some(hex(c1)? << 4 | hex(c2)?)
}

/// 值 → 大写十六进制字符 (base::HexCharOfValue)。
#[inline]
#[must_use]
pub fn hex_char_of_value(value: u8) -> u8 {
  match value {
    0..=9 => b'0' + value,
    _ => b'A' + (value - 10),
  }
}

/// 解码输出: 单字节前缀 + 双字节尾段拼接为完整结果 (one_byte_buffer / two_byte_buffer)。
#[derive(Debug, Default, PartialEq, Eq)]
pub struct DecodeOutput {
  /// 单字节前缀 (可为空)。
  pub one_byte: Vec<u8>,
  /// 双字节尾段 (可为空)。
  pub two_byte: Vec<u16>,
}

impl DecodeOutput {
  /// 合并单字节前缀与双字节尾段为 UTF-16 向量。
  #[must_use]
  pub fn to_u16_vec(&self) -> Vec<u16> {
    let mut all = Vec::with_capacity(self.one_byte.len() + self.two_byte.len());
    all.extend(self.one_byte.iter().map(|&b| u16::from(b)));
    all.extend_from_slice(&self.two_byte);
    all
  }

  /// 消费自身合并为 UTF-16 向量。
  #[must_use]
  pub fn into_u16_vec(self) -> Vec<u16> {
    if self.two_byte.is_empty() {
      self.one_byte.into_iter().map(u16::from).collect()
    } else {
      let mut all = self.one_byte.into_iter().map(u16::from).collect::<Vec<_>>();
      all.extend(self.two_byte);
      all
    }
  }
}

/// 解码/编码失败原因。
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum UriError {
  #[error("URI malformed")]
  /// URIError: malformed URI。
  Malformed,
  #[error("URI result too long")]
  /// 长度超上限 (encode/escape 路径)。
  TooLong,
}

/// %XX 八位组序列 → 码点, 追加到 buffer (DecodeOctets)。
/// U+FFFD 的显式编码放行, 其余非法 UTF-8 判 malformed。
fn decode_octets(octets: &[u8], buffer: &mut Vec<u16>) -> bool {
  let mut cursor = 0usize;
  let value = utf8::value_of(octets, &mut cursor);
  if value == utf8::BAD_CHAR && !is_replacement_character(octets) {
    return false;
  }
  if value <= utf16::MAX_NON_SURROGATE_CHAR_CODE {
    buffer.push(value as u16);
  } else {
    buffer.push(utf16::lead_surrogate(value));
    buffer.push(utf16::trail_surrogate(value));
  }
  true
}

/// 追加解码值: decodeURI 保留字符按 "%XX" 原样回写 (AddToBuffer)。
/// uri 为源串, index 为 '%' 的位置。
fn add_to_buffer(decoded: u16, uri: &[u16], index: usize, is_uri: bool, buffer: &mut Vec<u16>) {
  if is_uri && is_reserved_predicate(decoded) {
    buffer.push(u16::from(b'%'));
    buffer.push(uri[index + 1]);
    buffer.push(uri[index + 2]);
  } else {
    buffer.push(decoded);
  }
}

/// 双字节收集路径 (IntoTwoByte): 自 index 起逐字符解码到 two_byte。
/// 首个 %XX 八位组大于 0x7F 或裸非单字节字符时切入。
fn into_two_byte(uri: &[u16], index: usize, is_uri: bool, two_byte: &mut Vec<u16>) -> bool {
  let uri_length = uri.len();
  let mut k = index;
  while k < uri_length {
    let code = uri[k];
    if code == u16::from(b'%') {
      let two_digits = match (k + 2 < uri_length).then(|| two_digit_hex(uri[k + 1], uri[k + 2])) {
        Some(Some(v)) => v,
        _ => return false,
      };
      k += 2;
      let decoded = u16::from(two_digits);
      if decoded > utf8::MAX_ONE_BYTE_CHAR as u16 {
        // 多字节序列: 原版以 (decoded << ++n) & 0x80 判定 continuation 八位组数
        let mut octets = [0u8; utf8::MAX_ENCODED_SIZE as usize];
        octets[0] = two_digits;
        let mut n_cont = 0usize;
        while (u32::from(octets[0]) << (n_cont + 1)) & 0x80 != 0 {
          n_cont += 1;
          if n_cont > 3 || k + 3 >= uri_length {
            return false;
          }
          k += 1;
          if uri[k] != u16::from(b'%') {
            return false;
          }
          let cont = match two_digit_hex(uri[k + 1], uri[k + 2]) {
            Some(v) => v,
            None => return false,
          };
          k += 2;
          if n_cont >= octets.len() {
            return false;
          }
          octets[n_cont] = cont;
        }
        // 序列总长 = 首八位组 + n_cont 个 continuation
        if !decode_octets(&octets[..=n_cont], two_byte) {
          return false;
        }
      } else {
        add_to_buffer(decoded, uri, k - 2, is_uri, two_byte);
      }
    } else {
      two_byte.push(code);
    }
    k += 1;
  }
  true
}

/// decodeURI / decodeURIComponent 内核 (Uri::Decode)。
/// 输入为扁平化 UTF-16 内容; 失败返回 Malformed。
pub fn decode(uri: &[u16], is_uri: bool) -> Result<DecodeOutput, UriError> {
  let mut out = DecodeOutput::default();
  let uri_length = uri.len();
  let mut k = 0usize;
  while k < uri_length {
    let code = uri[k];
    if code == u16::from(b'%') {
      let two_digits = match (k + 2 < uri_length).then(|| two_digit_hex(uri[k + 1], uri[k + 2])) {
        Some(Some(v)) => v,
        _ => return Err(UriError::Malformed),
      };
      let decoded = u16::from(two_digits);
      if decoded > utf8::MAX_ONE_BYTE_CHAR as u16 {
        // 切入双字节路径, 自当前 % 继续
        if !into_two_byte(uri, k, is_uri, &mut out.two_byte) {
          return Err(UriError::Malformed);
        }
        return Ok(out);
      }
      if is_uri && is_reserved_predicate(decoded) {
        out.one_byte.push(b'%');
        out.one_byte.push(uri[k + 1] as u8);
        out.one_byte.push(uri[k + 2] as u8);
      } else {
        out.one_byte.push(two_digits);
      }
      k += 2;
    } else if code > utf8::MAX_ONE_BYTE_CHAR as u16 {
      // 裸非单字节字符: 切入双字节路径
      if !into_two_byte(uri, k, is_uri, &mut out.two_byte) {
        return Err(UriError::Malformed);
      }
      return Ok(out);
    } else {
      out.one_byte.push(code as u8);
    }
    k += 1;
  }
  Ok(out)
}

/// 码点 → %XX… 编码 (Encode 内部函数)。
fn encode_code_point(c: u32, out: &mut Vec<u8>) {
  let mut buf = [0u8; 4];
  let n = utf8::encode(&mut buf, c, utf16::NO_PREVIOUS_CHARACTER, false);
  for &b in &buf[..n as usize] {
    out.push(b'%');
    out.push(hex_char_of_value(b >> 4));
    out.push(hex_char_of_value(b & 0x0F));
  }
}

/// encodeURI / encodeURIComponent 内核 (Uri::Encode)。
/// 输入为扁平化 UTF-16 内容; 孤立代理返回 Malformed, 超长返回 TooLong。
pub fn encode(uri: &[u16], is_uri: bool, max_length: usize) -> Result<Vec<u8>, UriError> {
  let mut out: Vec<u8> = Vec::with_capacity(uri.len());
  let mut k = 0usize;
  while k < uri.len() {
    let cc1 = uri[k];
    if utf16::is_lead_surrogate(i32::from(cc1)) {
      k += 1;
      let Some(&cc2) = uri.get(k) else {
        // 尾随孤立 lead 代理
        return Err(UriError::Malformed);
      };
      if !utf16::is_trail_surrogate(i32::from(cc2)) {
        return Err(UriError::Malformed);
      }
      encode_code_point(
        utf16::combine_surrogate_pair(u32::from(cc1), u32::from(cc2)),
        &mut out,
      );
    } else if utf16::is_trail_surrogate(i32::from(cc1)) {
      // 无前置 lead 的 trail 代理
      return Err(UriError::Malformed);
    } else if is_unescape_predicate_in_uri_component(cc1) || (is_uri && is_uri_separator(cc1)) {
      out.push(cc1 as u8);
    } else {
      encode_code_point(u32::from(cc1), &mut out);
    }
    // 长度上限 (原版 ResizableBuffer max_capacity)
    if out.len() > max_length {
      return Err(UriError::TooLong);
    }
    k += 1;
  }
  Ok(out)
}

/// escape 内核 (ES B.2.1.1): 非转义字符原样, <256 转 %XX, 其余转 %uXXXX。
pub fn escape<C: Copy + Into<u32>>(src: &[C], max_length: usize) -> Result<Vec<u8>, UriError> {
  let mut out = Vec::with_capacity(src.len());
  for &c in src {
    let c = c.into();
    if c >= 256 {
      out.extend_from_slice(&[
        b'%',
        b'u',
        hex_char_of_value(((c >> 12) & 0xF) as u8),
        hex_char_of_value((c >> 8) as u8 & 0xF),
        hex_char_of_value((c >> 4) as u8 & 0xF),
        hex_char_of_value((c & 0xF) as u8),
      ]);
    } else if is_not_escaped(c as u16) {
      out.push(c as u8);
    } else {
      out.extend_from_slice(&[
        b'%',
        hex_char_of_value((c >> 4) as u8),
        hex_char_of_value((c & 0xF) as u8),
      ]);
    }
    if out.len() > max_length {
      return Err(UriError::TooLong);
    }
  }
  Ok(out)
}

/// unescape 内核字符级解码 (UnescapeChar): 返回 (值, 步进)。
fn unescape_char<C: Copy + Into<u32>>(v: &[C], i: usize, length: usize) -> (u16, usize) {
  let at = |k: usize| -> u16 { v[k].into() as u16 };
  let character = at(i);
  if character == u16::from(b'%')
    && i + 6 <= length
    && at(i + 1) == u16::from(b'u')
    && let (Some(hi), Some(lo)) = (
      two_digit_hex(at(i + 2), at(i + 3)),
      two_digit_hex(at(i + 4), at(i + 5)),
    )
  {
    return ((u16::from(hi) << 8) | u16::from(lo), 6);
  }
  if character == u16::from(b'%')
    && i + 3 <= length
    && let Some(lo) = two_digit_hex(at(i + 1), at(i + 2))
  {
    return (u16::from(lo), 3);
  }
  (character, 1)
}

/// unescape 内核 (ES B.2.1.2): 逐字符解码到 Vec<u16>。
#[must_use]
pub fn unescape<C: Copy + Into<u32>>(src: &[C]) -> Vec<u16> {
  let length = src.len();
  let mut out = Vec::with_capacity(length);
  let mut i = 0;
  while i < length {
    let (v, step) = unescape_char(src, i, length);
    out.push(v);
    i += step;
  }
  out
}
