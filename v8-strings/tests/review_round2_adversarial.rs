// 第 2 轮对抗式审查差分 (round2): 参考实现按原版 C++ 源码独立重写 (规则式 / 朴素式,
// 不复用 DFA 与查表), 与 crate 实现做固定 seed 随机差分 + 全域扫描。
// 角度: (a) utf-8 解码主循环 vs DFA / (b) uri Escape/Unescape 全码点 / (c) string-search
// 策略选择与升降级 / (d) string_case 全字节 / (e) hash_field 位布局。

use aok::OK;
use fastrand::Rng;
use v8_strings::{
  hash_field::{
    ARRAY_INDEX_LENGTH_SHIFT, ARRAY_INDEX_VALUE_MASK, HASH_BITS_MAX, HashFieldType,
    MAX_ARRAY_INDEX, MAX_CACHED_ARRAY_INDEX_LENGTH, contains_cached_array_index,
    create_hash_field_value, is_integer_index,
  },
  string_case::{CaseMapping, fast_ascii_case_prefix_length, fast_ascii_convert},
  string_hasher::{decode_array_index_from_hash_field, make_array_index_hash_raw},
  string_search::{
    Strategy, StringSearch, search_string_u8, search_string_u8_in_u16, search_string_u16,
    search_string_u16_in_u8,
  },
  unicode::{
    latin1,
    utf8::{self, BAD_CHAR},
    utf16,
    wtf8::validate_encoding,
  },
  unicode_decoder::Utf8Decoder,
  uri,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// ---------------------------------------------------------------------------
// (a) 严格 UTF-8 规则式参考解码 (unicode-inl.h ValueOf/CalculateValue 语义独立重写)
// ---------------------------------------------------------------------------

/// 规则式参考解码: 返回 (码点 | 0xFFFD, 消费字节数)。
/// 错误消费语义与 ValueOfIncremental 对齐: 坏 lead 消费 1 字节; 序列中途坏字节不消费
/// (游标停在该字节, Reject 回退); 截断游标到尾。
fn ref_value_of(bytes: &[u8]) -> (u32, usize) {
  let b0 = u32::from(bytes[0]);
  if b0 <= utf8::MAX_ONE_BYTE_CHAR {
    return (b0, 1);
  }
  // (总长, 第二字节合法区间) — 覆盖超长/代理/越界拒绝
  let (total, lo2, hi2): (usize, u32, u32) = match b0 {
    0xC2..=0xDF => (2, 0x80, 0xBF),
    0xE0 => (3, 0xA0, 0xBF),
    0xE1..=0xEC | 0xEE..=0xEF => (3, 0x80, 0xBF),
    0xED => (3, 0x80, 0x9F), // 代理区拒绝
    0xF0 => (4, 0x90, 0xBF), // 超长拒绝
    0xF1..=0xF3 => (4, 0x80, 0xBF),
    0xF4 => (4, 0x80, 0x8F),   // > U+10FFFF 拒绝
    _ => return (BAD_CHAR, 1), // 0x80..=0xBF / 0xC0 / 0xC1 / 0xF5..=0xFF: 坏 lead
  };
  if bytes.len() < 2 {
    return (BAD_CHAR, bytes.len()); // 截断
  }
  let b1 = u32::from(bytes[1]);
  if b1 < lo2 || b1 > hi2 {
    return (BAD_CHAR, 1); // 坏第二字节: 游标停在该字节
  }
  if total == 2 {
    return (((b0 & 0x1F) << 6) | (b1 & 0x3F), 2);
  }
  if bytes.len() < 3 {
    return (BAD_CHAR, bytes.len());
  }
  let b2 = u32::from(bytes[2]);
  if !(0x80..=0xBF).contains(&b2) {
    return (BAD_CHAR, 2);
  }
  if total == 3 {
    return (((b0 & 0x0F) << 12) | ((b1 & 0x3F) << 6) | (b2 & 0x3F), 3);
  }
  if bytes.len() < 4 {
    return (BAD_CHAR, bytes.len());
  }
  let b3 = u32::from(bytes[3]);
  if !(0x80..=0xBF).contains(&b3) {
    return (BAD_CHAR, 3);
  }
  (
    ((b0 & 0x07) << 18) | ((b1 & 0x3F) << 12) | ((b2 & 0x3F) << 6) | (b3 & 0x3F),
    4,
  )
}

/// 参考流式解码: 反复取首个码点。
fn ref_stream_decode(bytes: &[u8]) -> Vec<u32> {
  let mut out = Vec::new();
  let mut pos = 0usize;
  while pos < bytes.len() {
    let (v, n) = ref_value_of(&bytes[pos..]);
    out.push(v);
    pos += n;
  }
  out
}

/// crate 侧流式解码 (DecodeNormally 语义)。
fn normal_stream_decode(bytes: &[u8]) -> Vec<u32> {
  let mut out = Vec::new();
  let mut cursor = 0usize;
  while cursor < bytes.len() {
    let rest = &bytes[cursor..];
    out.push(utf8::value_of(rest, &mut cursor));
  }
  out
}

/// crate 侧增量解码 (DecodeIncrementally 语义)。
fn incremental_stream_decode(bytes: &[u8]) -> Vec<u32> {
  use v8_strings::utf8_dfa::strict::State;
  let mut out = Vec::new();
  let mut buffer = 0u32;
  let mut state = State::Accept;
  let mut cursor = 0usize;
  while cursor < bytes.len() {
    let r = utf8::value_of_incremental(bytes, &mut cursor, &mut state, &mut buffer);
    if r != utf8::INCOMPLETE {
      out.push(r);
    }
  }
  let r = utf8::value_of_incremental_finish(&mut state);
  if r != utf8::BUFFER_EMPTY {
    out.push(r);
  }
  out
}

/// crate 侧 utf16 预扫描解码 (DecodeUtf16 语义)。
fn utf16_stream_decode(bytes: &[u8]) -> Vec<u32> {
  let decoder = Utf8Decoder::new(bytes);
  let mut buf = vec![0u16; decoder.utf16_length() as usize];
  decoder.decode_to_two_byte(&mut buf, bytes);
  let mut out = Vec::new();
  let mut i = 0;
  while i < buf.len() {
    let b = buf[i];
    if utf16::is_lead_surrogate(i32::from(b)) && i + 1 < buf.len() {
      i += 1;
      out.push(utf16::combine_surrogate_pair(
        u32::from(b),
        u32::from(buf[i]),
      ));
    } else {
      out.push(u32::from(b));
    }
    i += 1;
  }
  out
}

/// 差分一批字节序列: 四路解码 (value_of 流 / 增量 / utf16 预扫描) vs 参考流。
fn diff_utf8_stream(cases: &[Vec<u8>]) {
  for bytes in cases {
    let want = ref_stream_decode(bytes);
    assert_eq!(
      normal_stream_decode(bytes),
      want,
      "value_of 流差分 bytes={bytes:02x?}"
    );
    assert_eq!(
      incremental_stream_decode(bytes),
      want,
      "增量差分 bytes={bytes:02x?}"
    );
    // utf16 预扫描: 码点语义一致 (孤立 BAD_CHAR 亦逐一对齐)
    assert_eq!(
      utf16_stream_decode(bytes),
      want,
      "utf16 差分 bytes={bytes:02x?}"
    );
  }
}

/// 参考严格 UTF-8 校验: 与 Utf8::ValidateEncoding 同义 (合法 U+FFFD 编码放行)。
fn ref_validate(bytes: &[u8]) -> bool {
  let mut pos = 0usize;
  while pos < bytes.len() {
    let rest = &bytes[pos..];
    let (v, n) = ref_value_of(rest);
    // BAD_CHAR 亦可能是合法的 U+FFFD 编码 (EF BF BD)
    if v == BAD_CHAR && !(rest.len() == 3 && rest == [0xEF, 0xBF, 0xBD]) {
      return false;
    }
    pos += n;
  }
  true
}

// TEST: 规则式参考 vs DFA — 全二字节域 + 三/四字节结构化扫描 + 固定 seed 随机。
#[test]
fn utf8_value_of_vs_rule_reference_differential() -> aok::Void {
  let mut cases: Vec<Vec<u8>> = Vec::new();
  // 全 65536 二字节组合
  for b0 in 0x80..=0xFF {
    for b1 in 0..=0xFF {
      cases.push(vec![b0, b1]);
    }
  }
  // 三字节: 每个 lead × 全 b1 × 代表性 b2
  for b0 in 0xE0..=0xF4 {
    for b1 in 0..=0xFF {
      for &b2 in &[0x00, 0x20, 0x7F, 0x80, 0x9F, 0xA0, 0xBF, 0xC0, 0xED, 0xFF] {
        cases.push(vec![b0, b1, b2]);
      }
    }
  }
  // 四字节: 结构化组合
  for &b0 in &[0xF0, 0xF1, 0xF3, 0xF4] {
    for &b1 in &[0x7F, 0x80, 0x8F, 0x90, 0xBF, 0xC0] {
      for &b2 in &[0x7F, 0x80, 0xBF, 0xC0] {
        for &b3 in &[0x7F, 0x80, 0xBF, 0xC0] {
          cases.push(vec![b0, b1, b2, b3]);
        }
      }
    }
  }
  diff_utf8_stream(&cases);

  // 固定 seed 随机: 3 个字节池 (全字节 / lead 偏置 / continuation 偏置) × 长度 1..=24
  let all: Vec<u8> = (0..=0xFF).collect();
  let leads: Vec<u8> = (0xC0..=0xF4).chain(0x80..=0x90).collect();
  let conts: Vec<u8> = (0x80..=0xBF)
    .chain([0x00, 0x41, 0xC2, 0xE0, 0xF0])
    .collect();
  for (pi, pool) in [&all, &leads, &conts].into_iter().enumerate() {
    let mut rng = Rng::with_seed(0xA11CE + pi as u64);
    let mut rand_cases: Vec<Vec<u8>> = Vec::with_capacity(4000);
    for _ in 0..4000 {
      let len = rng.usize(1..=24);
      rand_cases.push((0..len).map(|_| pool[rng.usize(0..pool.len())]).collect());
    }
    diff_utf8_stream(&rand_cases);
  }

  // validate_encoding vs 参考: 随机 + 全二字节域
  let mut rng = Rng::with_seed(0xDFA);
  for _ in 0..20000 {
    let len = rng.usize(0..=16);
    let bytes: Vec<u8> = (0..len).map(|_| rng.u8(..)).collect();
    assert_eq!(
      utf8::validate_encoding(&bytes),
      ref_validate(&bytes),
      "validate {bytes:02x?}"
    );
  }
  for b0 in 0..=0xFFu8 {
    for b1 in 0..=0xFFu8 {
      let bytes = [b0, b1];
      assert_eq!(
        utf8::validate_encoding(&bytes),
        ref_validate(&bytes),
        "validate {bytes:02x?}"
      );
    }
  }
  OK
}

// TEST: generalized DFA (wtf8::validate_encoding) vs 规则式参考 (允许孤立代理, 拒代理对)。
#[test]
fn wtf8_validate_vs_rule_reference_differential() -> aok::Void {
  // 参考: 广义 UTF-8 校验 (E1-EF 第二字节 80..BF; 代理码点合法) + 拒 lead-后跟-trail。
  let ref_wtf8_valid = |bytes: &[u8]| -> bool {
    let mut pos = 0usize;
    let mut prev: Option<u32> = None;
    while pos < bytes.len() {
      let b0 = u32::from(bytes[pos]);
      let (total, lo2, hi2): (usize, u32, u32) = match b0 {
        0x00..=0x7F => {
          prev = Some(b0);
          pos += 1;
          continue;
        }
        0xC2..=0xDF => (2, 0x80, 0xBF),
        0xE0 => (3, 0xA0, 0xBF),
        0xE1..=0xEF => (3, 0x80, 0xBF), // 广义: 允许 ED A0..BF (代理码点)
        0xF0 => (4, 0x90, 0xBF),
        0xF1..=0xF3 => (4, 0x80, 0xBF),
        0xF4 => (4, 0x80, 0x8F),
        _ => return false, // 80-BF / C0 / C1 / F5-FF
      };
      if pos + total > bytes.len() {
        return false;
      }
      let bs: Vec<u32> = bytes[pos + 1..pos + total]
        .iter()
        .map(|&b| u32::from(b))
        .collect();
      if bs[0] < lo2 || bs[0] > hi2 {
        return false;
      }
      for &b in &bs[1..] {
        if !(0x80..=0xBF).contains(&b) {
          return false;
        }
      }
      let cp = match total {
        2 => ((b0 & 0x1F) << 6) | (bs[0] & 0x3F),
        3 => ((b0 & 0x0F) << 12) | ((bs[0] & 0x3F) << 6) | (bs[1] & 0x3F),
        _ => ((b0 & 0x07) << 18) | ((bs[0] & 0x3F) << 12) | ((bs[1] & 0x3F) << 6) | (bs[2] & 0x3F),
      };
      // WTF-8 附加约束: lead 代理后不得跟 trail 代理
      if let Some(p) = prev
        && utf16::is_lead_surrogate(p as i32)
        && utf16::is_trail_surrogate(cp as i32)
      {
        return false;
      }
      prev = Some(cp);
      pos += total;
    }
    true
  };
  let mut cases: Vec<Vec<u8>> = Vec::new();
  for b0 in 0xC0..=0xF4u8 {
    for b1 in 0..=0xFFu8 {
      cases.push(vec![b0, b1]);
      for &b2 in &[0x80, 0xA0, 0xBF, 0x41] {
        cases.push(vec![b0, b1, b2]);
      }
    }
  }
  // 代理对编码 (U+D800 U+DC00) 与孤立代理编码混排
  cases.push(vec![0xED, 0xA0, 0x80, 0xED, 0xB0, 0x80]); // 成对 → 拒
  cases.push(vec![0xED, 0xA0, 0x80, 0x41]); // 孤立 lead → 受理
  cases.push(vec![0x41, 0xED, 0xB0, 0x80]); // 孤立 trail → 受理
  let mut rng = Rng::with_seed(0x57A9 + 0x3E8);
  let pool: Vec<u8> = (0xC0..=0xF4)
    .chain(0x80..=0xBF)
    .chain([0x00, 0x41])
    .collect();
  for _ in 0..8000 {
    let len = rng.usize(0..=12);
    cases.push((0..len).map(|_| pool[rng.usize(0..pool.len())]).collect());
  }
  for bytes in &cases {
    assert_eq!(
      validate_encoding(bytes),
      ref_wtf8_valid(bytes),
      "wtf8 {bytes:02x?}"
    );
  }
  OK
}

// ---------------------------------------------------------------------------
// (b) uri 全域差分
// ---------------------------------------------------------------------------

/// 参考: 字母数字 (IsAlphaNumeric 标量语义)。
fn ref_is_alpha_numeric(c: u32) -> bool {
  (c | 0x20).wrapping_sub(0x61) <= 0x19 || (0x30..=0x39).contains(&c)
}

/// 参考: IsNotEscaped (uri.cc:507-527)。
fn ref_is_not_escaped(c: u32) -> bool {
  ref_is_alpha_numeric(c) || matches!(c, 0x40 | 0x2A | 0x5F | 0x2B | 0x2D | 0x2E | 0x2F)
}

/// 参考: IsUnescapePredicateInUriComponent (uri.cc:313-331)。
fn ref_is_unescape_component(c: u32) -> bool {
  ref_is_alpha_numeric(c)
    || matches!(
      c,
      0x21 | 0x27 | 0x28 | 0x29 | 0x2A | 0x2D | 0x2E | 0x5F | 0x7E
    )
}

/// 参考: IsUriSeparator (uri.cc:333-350)。
fn ref_is_uri_separator(c: u32) -> bool {
  matches!(
    c,
    0x23 | 0x3A | 0x3B | 0x2F | 0x3F | 0x24 | 0x26 | 0x2B | 0x2C | 0x40 | 0x3D
  )
}

/// 参考: IsReservedPredicate (uri.cc:23-38)。
fn ref_is_reserved(c: u32) -> bool {
  matches!(
    c,
    0x23 | 0x24 | 0x26 | 0x2B | 0x2C | 0x2F | 0x3A | 0x3B | 0x3D | 0x3F | 0x40
  )
}

/// 参考: TwoDigitHex (uri.cc:70-80; base::HexValue 接受 0-9/A-F/a-f)。
fn ref_two_digit_hex(c1: u16, c2: u16) -> Option<u8> {
  let hex = |c: u16| -> Option<u8> {
    let c = u32::from(c);
    if (0x30..=0x39).contains(&c) {
      Some((c - 0x30) as u8)
    } else if (0x41..=0x46).contains(&c) || (0x61..=0x66).contains(&c) {
      Some(((c & 0x0F) + 9) as u8)
    } else {
      None
    }
  };
  Some(hex(c1)? << 4 | hex(c2)?)
}

/// 参考: 手写 UTF-8 编码 (Unicode 标准规则, 非 crate 实现)。
fn ref_utf8_encode(cp: u32) -> Vec<u8> {
  if cp < 0x80 {
    vec![cp as u8]
  } else if cp < 0x800 {
    vec![0xC0 | (cp >> 6) as u8, 0x80 | (cp & 0x3F) as u8]
  } else if cp < 0x10000 {
    vec![
      0xE0 | (cp >> 12) as u8,
      0x80 | ((cp >> 6) & 0x3F) as u8,
      0x80 | (cp & 0x3F) as u8,
    ]
  } else {
    vec![
      0xF0 | (cp >> 18) as u8,
      0x80 | ((cp >> 12) & 0x3F) as u8,
      0x80 | ((cp >> 6) & 0x3F) as u8,
      0x80 | (cp & 0x3F) as u8,
    ]
  }
}

/// 参考: EscapePrivate 主循环 (uri.cc:543-598)。
fn ref_escape(src: &[u16]) -> Vec<u8> {
  let hex = |v: u32| -> u8 {
    if v < 10 {
      b'0' + v as u8
    } else {
      b'A' + (v - 10) as u8
    }
  };
  let mut out = Vec::new();
  for &c in src {
    let c = u32::from(c);
    if c >= 256 {
      out.extend_from_slice(&[
        b'%',
        b'u',
        hex(c >> 12),
        hex((c >> 8) & 0xF),
        hex((c >> 4) & 0xF),
        hex(c & 0xF),
      ]);
    } else if ref_is_not_escaped(c) {
      out.push(c as u8);
    } else {
      out.extend_from_slice(&[b'%', hex(c >> 4), hex(c & 0xF)]);
    }
  }
  out
}

/// 参考: UnescapeChar + UnescapeSlow 主体 (uri.cc:428-452, 454-505)。
fn ref_unescape(src: &[u16]) -> Vec<u16> {
  let length = src.len();
  let mut out = Vec::new();
  let mut i = 0usize;
  while i < length {
    let character = src[i];
    let (val, step): (u16, usize) = if character == u16::from(b'%')
      && i + 6 <= length
      && src[i + 1] == u16::from(b'u')
      && let (Some(hi), Some(lo)) = (
        ref_two_digit_hex(src[i + 2], src[i + 3]),
        ref_two_digit_hex(src[i + 4], src[i + 5]),
      ) {
      ((u16::from(hi)) << 8 | u16::from(lo), 6)
    } else if character == u16::from(b'%')
      && i + 3 <= length
      && let Some(lo) = ref_two_digit_hex(src[i + 1], src[i + 2])
    {
      (u16::from(lo), 3)
    } else {
      (character, 1)
    };
    out.push(val);
    i += step;
  }
  out
}

/// 参考: EncodeHelperTwoByte (uri.cc:361-404)。
fn ref_encode(uri_in: &[u16], is_uri: bool) -> Result<Vec<u8>, ()> {
  let mut out = Vec::new();
  let push_percent = |out: &mut Vec<u8>, cp: u32| {
    for b in ref_utf8_encode(cp) {
      out.push(b'%');
      out.push(if b >> 4 < 10 {
        b'0' + (b >> 4)
      } else {
        b'A' + (b >> 4) - 10
      });
      out.push(if b & 0xF < 10 {
        b'0' + (b & 0xF)
      } else {
        b'A' + (b & 0xF) - 10
      });
    }
  };
  let mut k = 0usize;
  while k < uri_in.len() {
    let cc1 = uri_in[k];
    if utf16::is_lead_surrogate(i32::from(cc1)) {
      k += 1;
      if k >= uri_in.len() {
        return Err(());
      }
      let cc2 = uri_in[k];
      if !utf16::is_trail_surrogate(i32::from(cc2)) {
        return Err(());
      }
      push_percent(
        &mut out,
        utf16::combine_surrogate_pair(u32::from(cc1), u32::from(cc2)),
      );
    } else if utf16::is_trail_surrogate(i32::from(cc1)) {
      return Err(());
    } else if ref_is_unescape_component(u32::from(cc1))
      || (is_uri && ref_is_uri_separator(u32::from(cc1)))
    {
      out.push(cc1 as u8);
    } else {
      push_percent(&mut out, u32::from(cc1));
    }
    k += 1;
  }
  Ok(out)
}

/// 参考: IntoTwoByte (uri.cc:88-130; continuation 计数 (decoded << ++n) & 0x80)。
fn ref_into_two_byte(uri_in: &[u16], start: usize, is_uri: bool, buffer: &mut Vec<u16>) -> bool {
  let uri_length = uri_in.len();
  let mut k = start;
  while k < uri_length {
    let code = uri_in[k];
    if code == u16::from(b'%') {
      let Some(two_digits) = (k + 2 < uri_length)
        .then(|| ref_two_digit_hex(uri_in[k + 1], uri_in[k + 2]))
        .flatten()
      else {
        return false;
      };
      k += 2;
      let decoded = u16::from(two_digits);
      if decoded > utf8::MAX_ONE_BYTE_CHAR as u16 {
        let mut octets = [0u8; 4];
        octets[0] = two_digits;
        let mut n = 0usize;
        loop {
          n += 1;
          if (u32::from(decoded) << n) & 0x80 == 0 {
            break;
          }
          if n > 3 || k + 3 >= uri_length {
            return false;
          }
          k += 1;
          if uri_in[k] != u16::from(b'%') {
            return false;
          }
          let Some(cont) = ref_two_digit_hex(uri_in[k + 1], uri_in[k + 2]) else {
            return false;
          };
          k += 2;
          octets[n] = cont;
        }
        // DecodeOctets: 规则式参考解码
        let (value, _) = ref_value_of(&octets[..n]);
        if value == BAD_CHAR && octets[..n] != [0xEF, 0xBF, 0xBD] {
          return false;
        }
        if value <= utf16::MAX_NON_SURROGATE_CHAR_CODE {
          buffer.push(value as u16);
        } else {
          buffer.push(utf16::lead_surrogate(value));
          buffer.push(utf16::trail_surrogate(value));
        }
      } else if is_uri && ref_is_reserved(u32::from(decoded)) {
        // AddToBuffer: 回写 "%XX" (index = k-2 为 '%' 位置)
        buffer.push(u16::from(b'%'));
        buffer.push(uri_in[k - 1]);
        buffer.push(uri_in[k]);
      } else {
        buffer.push(decoded);
      }
    } else {
      buffer.push(code);
    }
    k += 1;
  }
  true
}

/// 参考: IntoOneAndTwoByte 合流输出 (uri.cc:132-170)。
fn ref_decode(uri_in: &[u16], is_uri: bool) -> Result<Vec<u16>, ()> {
  let mut out: Vec<u16> = Vec::new();
  let uri_length = uri_in.len();
  let mut k = 0usize;
  while k < uri_length {
    let code = uri_in[k];
    if code == u16::from(b'%') {
      let Some(two_digits) = (k + 2 < uri_length)
        .then(|| ref_two_digit_hex(uri_in[k + 1], uri_in[k + 2]))
        .flatten()
      else {
        return Err(());
      };
      let decoded = u16::from(two_digits);
      if decoded > utf8::MAX_ONE_BYTE_CHAR as u16 {
        return if ref_into_two_byte(uri_in, k, is_uri, &mut out) {
          Ok(out)
        } else {
          Err(())
        };
      }
      if is_uri && ref_is_reserved(u32::from(decoded)) {
        out.push(u16::from(b'%'));
        out.push(uri_in[k + 1]);
        out.push(uri_in[k + 2]);
      } else {
        out.push(decoded);
      }
      k += 2;
    } else {
      if code > utf8::MAX_ONE_BYTE_CHAR as u16 {
        return if ref_into_two_byte(uri_in, k, is_uri, &mut out) {
          Ok(out)
        } else {
          Err(())
        };
      }
      out.push(code);
    }
    k += 1;
  }
  Ok(out)
}

// TEST: escape 全 u16 域 (65536) + 全 0-255 字节双宽度 + 随机序列 + max_length 边界。
#[test]
fn uri_escape_full_domain_differential() -> aok::Void {
  // 单字符全 u16 域 (latin1 走 u8 输入, 其余 u16)
  for c in 0..=0xFFFFu16 {
    let want = ref_escape(&[c]);
    let got = uri::escape(&[c], usize::MAX).ok();
    assert_eq!(got.as_deref(), Some(want.as_slice()), "escape u16 {c:#06x}");
    if c <= latin1::MAX_CHAR {
      let got8 = uri::escape(&[c as u8], usize::MAX).ok();
      assert_eq!(got8.as_deref(), Some(want.as_slice()), "escape u8 {c:#06x}");
    }
  }
  // 随机序列双宽度
  let mut rng = Rng::with_seed(0xE5CA9E);
  for _ in 0..500 {
    let len = rng.usize(0..=32);
    let seq: Vec<u16> = (0..len).map(|_| rng.u16(..)).collect();
    let want = ref_escape(&seq);
    let got = uri::escape(&seq, usize::MAX).ok();
    assert_eq!(
      got.as_deref(),
      Some(want.as_slice()),
      "escape rand {seq:02x?}"
    );
    let seq8: Vec<u8> = seq.iter().filter_map(|&c| u8::try_from(c).ok()).collect();
    let wide: Vec<u16> = seq8.iter().map(|&c| u16::from(c)).collect();
    let want8 = ref_escape(&wide);
    let got8 = uri::escape(&seq8, usize::MAX).ok();
    assert_eq!(
      got8.as_deref(),
      Some(want8.as_slice()),
      "escape rand8 {seq8:02x?}"
    );
  }
  // max_length: 恰等 → Ok; 少 1 → TooLong
  for c in [0x41u16, 0xE9, 0x2028, 0xFFFD] {
    let full = ref_escape(&[c]).len();
    assert!(uri::escape(&[c], full).is_ok(), "escape 上界 {c:#06x}");
    assert!(
      matches!(uri::escape(&[c], full - 1), Err(uri::UriError::TooLong)),
      "escape 超限 {c:#06x}"
    );
  }
  OK
}

// TEST: unescape 全域 — 全 %XX (256) / 全 %uXXXX (65536) / 截断边界 / 随机混排双宽度。
#[test]
fn uri_unescape_full_domain_differential() -> aok::Void {
  for hi in 0..=0xFFu8 {
    // %XX
    let pat: Vec<u16> = [b'%', hi / 16, hi % 16]
      .iter()
      .map(|&b| u16::from(b))
      .collect();
    assert_eq!(
      uri::unescape(&pat),
      ref_unescape(&pat),
      "unescape %{hi:02X}"
    );
    // %uXXXX
    for lo in 0..=0xFFu8 {
      let pat4: Vec<u16> = [b'%', b'u', hi, lo, 0x30, 0x30]
        .iter()
        .map(|&b| u16::from(b))
        .collect();
      assert_eq!(
        uri::unescape(&pat4),
        ref_unescape(&pat4),
        "unescape %u{hi:02X}{lo:02X}"
      );
    }
  }
  // 截断边界 (UnescapeChar 的 i+6/i+3 上界)
  for tail in [
    "u".as_bytes(),
    "u1".as_bytes(),
    "u12".as_bytes(),
    "u123".as_bytes(),
    "u1234".as_bytes(),
    "u12345".as_bytes(),
  ] {
    let mut pat: Vec<u16> = vec![u16::from(b'%')];
    pat.extend(tail.iter().map(|&b| u16::from(b)));
    assert_eq!(
      uri::unescape(&pat),
      ref_unescape(&pat),
      "unescape 截断 {tail:02x?}"
    );
  }
  // 大小写 hex 混排 (TwoDigitHex 接受 A-F/a-f)
  for pat in [
    &b"%Ab"[..],
    &b"%aB"[..],
    &b"%AB"[..],
    &b"%4F"[..],
    &b"%4f"[..],
  ] {
    let pat16: Vec<u16> = pat.iter().map(|&b| u16::from(b)).collect();
    assert_eq!(
      uri::unescape(&pat16),
      ref_unescape(&pat16),
      "unescape {pat:02x?}"
    );
  }
  // 随机混排双宽度
  let mut rng = Rng::with_seed(0xC0DE + 0x5E7);
  for _ in 0..2000 {
    let len = rng.usize(0..=24);
    let seq: Vec<u16> = (0..len)
      .map(|_| match rng.usize(0..3) {
        0 => u16::from(b'%'),
        1 => rng.u16(..),
        _ => u16::from(rng.u8(..)),
      })
      .collect();
    assert_eq!(
      uri::unescape(&seq),
      ref_unescape(&seq),
      "unescape rand {seq:02x?}"
    );
    let seq8: Vec<u8> = seq.iter().map(|&c| c as u8).collect();
    let wide8: Vec<u16> = seq8.iter().copied().map(u16::from).collect();
    assert_eq!(
      uri::unescape(&seq8),
      ref_unescape(&wide8),
      "unescape rand8 {seq8:02x?}"
    );
  }
  OK
}

// TEST: encode 全 u16 单字符域 (65536 × is_uri 双态) + 随机多字符 + 代理畸形 + 上限。
#[test]
fn uri_encode_full_domain_differential() -> aok::Void {
  for c in 0..=0xFFFFu16 {
    for is_uri in [false, true] {
      let want = ref_encode(&[c], is_uri).ok();
      let got = uri::encode(&[c], is_uri, usize::MAX).ok();
      assert_eq!(
        got.as_deref(),
        want.as_deref(),
        "encode {c:#06x} is_uri={is_uri}"
      );
    }
  }
  // 随机多字符
  let mut rng = Rng::with_seed(0xE4C0DE);
  for _ in 0..2000 {
    let len = rng.usize(0..=24);
    let seq: Vec<u16> = (0..len).map(|_| rng.u16(..)).collect();
    for is_uri in [false, true] {
      let want = ref_encode(&seq, is_uri).ok();
      let got = uri::encode(&seq, is_uri, usize::MAX).ok();
      assert_eq!(got.as_deref(), want.as_deref(), "encode rand {seq:02x?}");
    }
  }
  // 畸形代理: 尾随 lead / lead+lead / 孤立 trail
  for seq in [
    vec![0xD83Du16],
    vec![0xD83D, 0xD83D],
    vec![0xDC00u16],
    vec![0x41, 0xDC00, 0x42],
  ] {
    assert!(ref_encode(&seq, false).is_err());
    assert!(
      matches!(
        uri::encode(&seq, false, usize::MAX),
        Err(uri::UriError::Malformed)
      ),
      "encode 畸形代理 {seq:02x?}"
    );
  }
  // 合法代理对: D83D DE00 = U+1F600 → 4 字节 UTF-8 → 12 字节 %XX
  let pair = vec![0xD83Du16, 0xDE00];
  let got = uri::encode(&pair, false, usize::MAX).unwrap_or_default();
  assert_eq!(got, b"%F0%9F%98%80", "代理对编码");
  // max_length 上界
  assert!(matches!(
    uri::encode(&[0x7Eu16], false, 0),
    Err(uri::UriError::TooLong)
  ));
  assert!(uri::encode(&[0x7Eu16], false, 1).is_ok());
  OK
}

// TEST: decode 全 %XX 域 (65536 × is_uri 双态) + 双字节路径结构化扫描 + 随机 + 往返。
#[test]
fn uri_decode_full_domain_differential() -> aok::Void {
  for hi in 0..=0xFFu8 {
    for lo in 0..=0xFFu8 {
      let pat: Vec<u16> = [b'%', hi, lo].iter().map(|&b| u16::from(b)).collect();
      for is_uri in [false, true] {
        let want = ref_decode(&pat, is_uri).ok();
        let got = uri::decode(&pat, is_uri).map(|o| o.into_u16_vec()).ok();
        assert_eq!(
          got.as_deref(),
          want.as_deref(),
          "decode %{hi:02X}{lo:02X} is_uri={is_uri}"
        );
      }
    }
  }
  // 双字节路径: lead × b1 结构化 (补全 3 字节八位组)
  let pct = |b: u8| -> Vec<u16> {
    [b'%', b >> 4, b & 0xF]
      .iter()
      .map(|&x| u16::from(x))
      .collect()
  };
  for b0 in 0xC2..=0xF4u8 {
    for &b1 in &[
      0x00u8, 0x41, 0x7F, 0x80, 0x8F, 0x90, 0x9F, 0xA0, 0xBF, 0xC0, 0xFF,
    ] {
      let mut seq = pct(b0);
      seq.extend(pct(b1));
      seq.extend(pct(0x80));
      let want = ref_decode(&seq, false).ok();
      let got = uri::decode(&seq, false).map(|o| o.into_u16_vec()).ok();
      assert_eq!(
        got.as_deref(),
        want.as_deref(),
        "decode 3字节 {b0:02X} {b1:02X}"
      );
    }
  }
  // 保留字符回写 (decodeURI): "%23" is_uri → '%','2','3'
  let seq: Vec<u16> = b"%23".iter().map(|&b| u16::from(b)).collect();
  let got = uri::decode(&seq, true).unwrap_or_default().into_u16_vec();
  assert_eq!(got, vec![0x25, 0x32, 0x33], "decodeURI 保留 #");
  // 随机混排 (含畸形)
  let mut rng = Rng::with_seed(0xEC0DE);
  let hexes: Vec<u16> = b"0123456789abcdefABCDEF"
    .iter()
    .map(|&b| u16::from(b))
    .collect();
  for _ in 0..3000 {
    let len = rng.usize(0..=16);
    let seq: Vec<u16> = (0..len)
      .map(|_| match rng.usize(0..4) {
        0 => u16::from(b'%'),
        1 => hexes[rng.usize(0..hexes.len())],
        2 => rng.u16(..),
        _ => u16::from(rng.u8(..)),
      })
      .collect();
    for is_uri in [false, true] {
      let want = ref_decode(&seq, is_uri).ok();
      let got = uri::decode(&seq, is_uri).map(|o| o.into_u16_vec()).ok();
      assert_eq!(got.as_deref(), want.as_deref(), "decode rand {seq:02x?}");
    }
  }
  // 往返: 非代理码元 x → encode → decode == x
  let mut rng2 = Rng::with_seed(0x1234);
  for _ in 0..4000 {
    let c = loop {
      let c = rng2.u16(..);
      if !utf16::is_lead_surrogate(i32::from(c)) && !utf16::is_trail_surrogate(i32::from(c)) {
        break c;
      }
    };
    for is_uri in [false, true] {
      let enc = uri::encode(&[c], is_uri, usize::MAX).unwrap_or_default();
      let enc16: Vec<u16> = enc.iter().map(|&b| u16::from(b)).collect();
      let dec = uri::decode(&enc16, is_uri)
        .unwrap_or_default()
        .into_u16_vec();
      assert_eq!(dec, vec![c], "往返 {c:#06x} enc={enc:02x?}");
    }
  }
  OK
}

// ---------------------------------------------------------------------------
// (c) string-search 策略与随机差分
// ---------------------------------------------------------------------------

/// 朴素参考搜索。
fn naive_search<S: Copy + PartialEq>(subject: &[S], pattern: &[S], start: usize) -> Option<usize> {
  if pattern.is_empty() || pattern.len() > subject.len() || start > subject.len() - pattern.len() {
    return None;
  }
  (start..=subject.len() - pattern.len()).find(|&i| subject[i..i + pattern.len()] == *pattern)
}

/// 混合宽度朴素参考 (u16 subject × u8 pattern)。
fn naive_search_mixed(subject: &[u16], pattern: &[u8], start: usize) -> Option<usize> {
  if pattern.is_empty() || pattern.len() > subject.len() || start > subject.len() - pattern.len() {
    return None;
  }
  (start..=subject.len() - pattern.len()).find(|&i| {
    subject[i..i + pattern.len()]
      .iter()
      .zip(pattern)
      .all(|(&s, &p)| s == u16::from(p))
  })
}

// TEST: 四宽度组合 × needle 长度 1/2/3/4/6/7/8/16/63/64/250/251/300 × 随机 subject/start
// vs 朴素参考; 含 1/2/3 全起点对齐扫描。
#[test]
fn string_search_random_differential_and_alignments() -> aok::Void {
  let mut rng = Rng::with_seed(0x5EA1C7);
  let lens = [1usize, 2, 3, 4, 6, 7, 8, 16, 63, 64, 250, 251, 300];
  // u8/u16 生成器: 宽池偏置产生 >0xFF 字符 (触发 Fail 策略与等价类桶)
  let gen8 = |rng: &mut fastrand::Rng, len: usize| -> Vec<u8> {
    (0..len)
      .map(|_| if rng.bool() { rng.u8(..) } else { rng.u8(0..4) })
      .collect()
  };
  let gen16 = |rng: &mut fastrand::Rng, len: usize, wide: bool| -> Vec<u16> {
    (0..len)
      .map(|_| {
        if wide && rng.bool() {
          rng.u16(0x100..)
        } else {
          rng.u16(..0x100)
        }
      })
      .collect()
  };
  for &plen in &lens {
    for case_i in 0..60 {
      let wide_pattern = case_i % 3 == 2;
      let subject_len = rng.usize(0..=40) + if plen > 16 { plen + 20 } else { 0 };
      let subject8 = gen8(&mut rng, subject_len);
      let subject16_wide = gen16(&mut rng, subject8.len().max(plen * 2), true);
      let subject16_narrow: Vec<u16> = subject8.iter().map(|&b| u16::from(b)).collect();
      let pattern8 = gen8(&mut rng, plen);
      let pattern16 = gen16(&mut rng, plen, wide_pattern);
      let start = rng.usize(0..=4);
      assert_eq!(
        search_string_u8(&subject8, &pattern8, start),
        naive_search(&subject8, &pattern8, start),
        "u8×u8 plen={plen} i={case_i} subj={subject8:02x?} pat={pattern8:02x?} start={start}"
      );
      assert_eq!(
        search_string_u8_in_u16(&subject16_narrow, &pattern8, start),
        naive_search_mixed(&subject16_narrow, &pattern8, start),
        "u16窄×u8 plen={plen} i={case_i}"
      );
      assert_eq!(
        search_string_u16(&subject16_wide, &pattern16, start),
        naive_search(&subject16_wide, &pattern16, start),
        "u16×u16 plen={plen} i={case_i}"
      );
      // u8 subject × u16 pattern: pattern 含 >0xFF 必败, 否则等价 u8 pattern
      let expect_mixed = if pattern16.iter().any(|&c| c > 0xFF) {
        None
      } else {
        let p8: Vec<u8> = pattern16.iter().map(|&c| c as u8).collect();
        naive_search(&subject8, &p8, start)
      };
      assert_eq!(
        search_string_u16_in_u8(&subject8, &pattern16, start),
        expect_mixed,
        "u8×u16 plen={plen} i={case_i}"
      );
    }
  }
  // needle 长度 1/2/3: 全起点对齐扫描 (每个起点都是一次 Search)
  for plen in [1usize, 2, 3] {
    for _trial in 0..40 {
      let subject8 = gen8(&mut rng, 40);
      let pattern8 = gen8(&mut rng, plen);
      let subject16: Vec<u16> = subject8.iter().map(|&b| u16::from(b)).collect();
      let subject16_wide = gen16(&mut rng, 40, true);
      let pattern16 = gen16(&mut rng, plen, true);
      for start in 0..=(40 - plen) {
        assert_eq!(
          search_string_u8(&subject8, &pattern8, start),
          naive_search(&subject8, &pattern8, start),
          "对齐 u8 plen={plen} start={start}"
        );
        assert_eq!(
          search_string_u16(&subject16_wide, &pattern16, start),
          naive_search(&subject16_wide, &pattern16, start),
          "对齐 u16 plen={plen} start={start}"
        );
        assert_eq!(
          search_string_u8_in_u16(&subject16, &pattern8, start),
          naive_search_mixed(&subject16, &pattern8, start),
          "对齐 u16窄×u8 plen={plen} start={start}"
        );
      }
    }
  }
  // 边界: 空 pattern / pattern 长于 subject / start 越界
  assert_eq!(search_string_u8(b"abc", b"", 0), None);
  assert_eq!(search_string_u8(b"ab", b"abc", 0), None);
  assert_eq!(search_string_u8(b"abc", b"b", 3), None);
  assert_eq!(search_string_u8(b"", b"a", 0), None);
  OK
}

// TEST: 策略升降级 — Initial 升 BMH、BMH 升 BM; 构造期策略阈值 (1=SingleChar,
// 2..6=Linear, >=7=Initial); 升级后同对象跨 subject 复用仍正确。
#[test]
fn string_search_strategy_lifecycle_differential() -> aok::Void {
  // 构造期策略选择 (string-search.h 构造函数阈值)
  let strat = |pat: &[u8]| {
    let mut s = StringSearch::new_u8(pat);
    let _ = s.search(b"zzzzzzzzzz", 0);
    s.strategy()
  };
  assert_eq!(strat(b"x"), Strategy::SingleChar);
  for len in 2..=6 {
    assert_eq!(strat(&vec![b'a'; len]), Strategy::Linear, "len={len}");
  }
  assert_eq!(strat(&[b'a'; 7]), Strategy::Initial);
  assert_eq!(strat(&[b'a'; 251]), Strategy::Initial);

  // Initial → BMH 升级: 长 subject 近命中连续出现, badness 耗尽
  let pattern: Vec<u8> = b"abcabcx".to_vec();
  let mut subject: Vec<u8> = Vec::new();
  for _ in 0..60 {
    subject.extend_from_slice(b"abcabcw");
  }
  subject.extend_from_slice(&pattern);
  let want = naive_search(&subject, &pattern, 0);
  let mut search = StringSearch::new_u8(&pattern);
  assert_eq!(search.search(&subject, 0), want);
  assert_eq!(
    search.strategy(),
    Strategy::BoyerMooreHorspool,
    "应升级 BMH"
  );

  // BMH → BM 升级: last_char 早现 + 首字符错配 (j=0 → badness 每窗 +1)
  // pattern "B" + "A"*14 + "B": occ('B')=0 → last_char_shift=15; 窗口末字符 'B' 命中
  // 而首字符 'A' 错配 → badness 单调涨穿 0 (string-search.h:495-501)
  let pattern2: Vec<u8> = b"BAAAAAAAAAAAAAAB".to_vec();
  let mut subject2: Vec<u8> = b"AAAAAAAAAAAAAAAAA".to_vec(); // 17 A: 无 B 起窗
  for _ in 0..30 {
    subject2.extend_from_slice(b"BAAAAAAAAAAAAAAA"); // B + 15 A, 间距 16
  }
  let want2 = naive_search(&subject2, &pattern2, 0);
  let mut search2 = StringSearch::new_u8(&pattern2);
  assert_eq!(search2.search(&subject2, 0), want2);
  assert_eq!(search2.strategy(), Strategy::BoyerMoore, "应升级 BM");

  // 升级后同对象跨 subject 复用 (表已在对象内, 二次搜索仍须正确)
  assert_eq!(search2.search(&subject2, 0), want2);
  let mut subject3 = b"xxAAAAAABxx".to_vec();
  subject3.extend_from_slice(&pattern2);
  assert_eq!(
    search2.search(&subject3, 0),
    naive_search(&subject3, &pattern2, 0),
    "BM 策略二次搜索"
  );

  // Fail 策略: u16 pattern 含 >0xFF 而 subject 为 u8
  let mut search3 = StringSearch::new_u16(&[0x1234u16]);
  assert_eq!(search3.search_u8(b"abc", 0), None);
  assert_eq!(search3.strategy(), Strategy::Fail);
  OK
}

// ---------------------------------------------------------------------------
// (d) string_case 全字节差分
// ---------------------------------------------------------------------------

/// 参考: 标量转换循环 (string-case.cc FastAsciiConvert 标量尾语义)。
fn ref_convert(case: CaseMapping, src: &[u8]) -> (usize, Vec<u8>) {
  let (lo, hi) = (case.lo(), case.hi());
  let mut dst = Vec::with_capacity(src.len());
  for &c in src {
    if c & 0x80 != 0 {
      return (dst.len(), dst);
    }
    if lo < c && c < hi {
      dst.push(c ^ (1 << 5));
    } else {
      dst.push(c);
    }
  }
  (src.len(), dst)
}

/// 参考: 前缀长度 (非 ASCII 或需转换即停)。
fn ref_prefix(case: CaseMapping, src: &[u8]) -> usize {
  let (lo, hi) = (case.lo(), case.hi());
  src
    .iter()
    .position(|&c| c & 0x80 != 0 || (lo < c && c < hi))
    .unwrap_or(src.len())
}

// TEST: 全 256 单字节 / 全 65536 双字节前缀 / 随机序列 / 土耳其 İ 与 ß 边界。
#[test]
fn string_case_full_byte_domain_differential() -> aok::Void {
  for &case in &[CaseMapping::ToLower, CaseMapping::ToUpper] {
    // 全 256 单字节: 转换结果与前缀
    for b in 0..=0xFFu8 {
      let (n, dst) = ref_convert(case, &[b]);
      let mut buf = [0u8; 1];
      let got = fast_ascii_convert(case, &mut buf, &[b]);
      assert_eq!(got, n, "convert 单字节 {b:#04x} case={case:?}");
      assert_eq!(&buf[..got], &dst[..got], "convert 内容 {b:#04x}");
      assert_eq!(
        fast_ascii_case_prefix_length(case, &[b]),
        ref_prefix(case, &[b])
      );
    }
    // 全 65536 双字节前缀
    for a in 0..=0xFFu8 {
      for b in 0..=0xFFu8 {
        let two = [a, b];
        assert_eq!(
          fast_ascii_case_prefix_length(case, &two),
          ref_prefix(case, &two),
          "prefix {a:#04x}{b:#04x}"
        );
      }
    }
    // 随机序列: convert 的词级命中返回保守前缀 (原版 FastAsciiConvert 词循环 return
    // 词边界, string-case.cc:105-107 同构); 断言 三性质: 不越精确停点 / 已处理前缀
    // 逐字节正确 / 全程可转换时整串完成
    let mut rng = Rng::with_seed(0xCA5E + u64::from(u8::from(case.is_to_lower())));
    let (lo, hi) = (case.lo(), case.hi());
    for _ in 0..300 {
      let len = rng.usize(0..=64);
      let src: Vec<u8> = (0..len).map(|_| rng.u8(..)).collect();
      let (exact_n, _) = ref_convert(case, &src);
      let mut dst = vec![0u8; src.len().max(1)];
      let got = fast_ascii_convert(case, &mut dst, &src);
      assert!(
        got <= exact_n,
        "保守前缀越界 {src:02x?} got={got} exact={exact_n}"
      );
      let scalar: Vec<u8> = src[..got]
        .iter()
        .map(|&c| if lo < c && c < hi { c ^ (1 << 5) } else { c })
        .collect();
      assert_eq!(&dst[..got], &scalar[..], "前缀内容 rand {src:02x?}");
      if exact_n == src.len() {
        assert_eq!(got, src.len(), "全程可转换须整串 {src:02x?}");
      }
      assert_eq!(
        fast_ascii_case_prefix_length(case, &src),
        ref_prefix(case, &src)
      );
    }
  }
  // 土耳其 İ (U+0130 = C4 B0) 与 ß (U+00DF = C3 9F): 非 ASCII 边界即停
  for seq in [
    &b"ai\xC4\xB0b"[..],
    &b"AB\xC3\x9Fcd"[..],
    &b"\xC4\xB0"[..],
    &b"a\xC3\x9F"[..],
  ] {
    for &case in &[CaseMapping::ToLower, CaseMapping::ToUpper] {
      let (want_n, want_dst) = ref_convert(case, seq);
      let mut dst = vec![0u8; seq.len()];
      let got = fast_ascii_convert(case, &mut dst, seq);
      assert_eq!(got, want_n, "İ/ß 边界 {seq:02x?}");
      assert_eq!(&dst[..got], &want_dst[..got]);
    }
  }
  // dst 短于 src: 截断于 dst 容量 (安全上限语义)
  let mut small = [0u8; 2];
  let got = fast_ascii_convert(CaseMapping::ToUpper, &mut small, b"abcdef");
  assert_eq!(got, 2);
  assert_eq!(&small, b"AB");
  OK
}

// ---------------------------------------------------------------------------
// (e) hash_field 位布局 (name.h static_assert 组的运行时镜像)
// ---------------------------------------------------------------------------

// TEST: 位域常量与掩码逐项对拍 name.h, MakeArrayIndexHash 编解码往返, 缓存判定语义。
#[test]
fn hash_field_layout_snapshot_invariants() -> aok::Void {
  // name.h:169-174 枚举值
  assert_eq!(HashFieldType::Hash as u32, 0b10);
  assert_eq!(HashFieldType::IntegerIndex as u32, 0b00);
  assert_eq!(HashFieldType::ForwardingIndex as u32, 0b01);
  assert_eq!(HashFieldType::Empty as u32, 0b11);
  // 位域几何 (name.h:175-178, 217-239)
  assert_eq!(ARRAY_INDEX_VALUE_MASK, 0x00FF_FFFF);
  assert_eq!(ARRAY_INDEX_LENGTH_SHIFT, 26);
  assert_eq!(HASH_BITS_MAX, 0x3FFF_FFFF);
  assert_eq!(MAX_ARRAY_INDEX, u32::MAX - 1);
  assert_eq!(MAX_CACHED_ARRAY_INDEX_LENGTH, 7);
  // CreateHashFieldValue: hash 截 30 bit | 类型 (name-inl.h:150-156)
  let h = create_hash_field_value(0xFFFF_FFFF, HashFieldType::Hash);
  assert_eq!(h, (0x3FFF_FFFF << 2) | 0b10);
  // MakeArrayIndexHash 编解码往返 (采样: 0 / 边界 / 10^n / 24 位上溢截断同原版)
  let samples = [
    0u32,
    1,
    9,
    10,
    0x00FF_FFFF,
    999_999,
    4_294_967_294,
    123_456_789,
  ];
  for v in samples {
    for len in 1u32..=10 {
      let hash = make_array_index_hash_raw(v, len);
      // 24 位域: >24 bit 的值按原版移位截断语义
      assert_eq!(
        decode_array_index_from_hash_field(hash),
        v & ARRAY_INDEX_VALUE_MASK,
        "往返 v={v} len={len}"
      );
      assert_eq!(hash & 0b11, 0b00, "array index 类型位必为 IntegerIndex");
      // 长度 ≤ 7 → 含缓存 index; > 7 → 不含 (name.h:241-253)。
      // 上游不变量前提: 可缓存长度 (≤7 位十进制) ⇒ 值 < 10^7 < 2^24, 不污染长度域;
      // 超过 24 bit 的值 (仅长度 >7 时可达) 移位后侵入长度域, 与原版移位截断一致。
      if v < (1 << 24) {
        assert_eq!(
          contains_cached_array_index(hash),
          len <= MAX_CACHED_ARRAY_INDEX_LENGTH,
          "缓存判定 v={v} len={len}"
        );
      }
      assert!(is_integer_index(hash), "IntegerIndex 类型位判定");
    }
  }
  // 0 值 index ("0" 串) 仍非零哈希字段
  assert_ne!(make_array_index_hash_raw(0, 1), 0);
  // 普通哈希类型位
  assert_eq!(create_hash_field_value(0, HashFieldType::Hash) & 0b11, 0b10);
  OK
}
