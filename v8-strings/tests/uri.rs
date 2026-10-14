// 迁移自 v8/test/mjsunit/uri.js 与 v8/test/mjsunit/escape.js
// 规约: 1:1 覆盖原版用例与断言

use aok::OK;
use v8_strings::uri::{UriError, decode, encode, escape, unescape};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

fn to_u16(s: &str) -> Vec<u16> {
  s.encode_utf16().collect()
}

fn decode_to_string(uri_u16: &[u16], is_uri: bool) -> Result<String, UriError> {
  let out = decode(uri_u16, is_uri)?;
  if !out.two_byte.is_empty() {
    let mut all = out.one_byte.into_iter().map(u16::from).collect::<Vec<_>>();
    all.extend(out.two_byte);
    String::from_utf16(&all).map_err(|_| UriError::Malformed)
  } else {
    String::from_utf8(out.one_byte).map_err(|_| UriError::Malformed)
  }
}

// v8/test/mjsunit/uri.js:30
#[test]
fn uri_ascii_invariants() -> aok::Void {
  let ascii_set = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-_.!~*'();/?:@&=+$,#";
  let input = to_u16(ascii_set);
  let encoded = encode(&input, true, usize::MAX)?;
  assert_eq!(String::from_utf8(encoded)?, ascii_set);
  OK
}

// v8/test/mjsunit/uri.js:33-65
#[test]
fn uri_special_characters_encode() -> aok::Void {
  let cases: &[(&[u16], &str)] = &[
    (&[0x007D], "%7D"),
    (&[0x0000], "%00"),
    (&[0x0080], "%C2%80"),
    (&[0x0555], "%D5%95"),
    (&[0x07FF], "%DF%BF"),
    (&[0x0800], "%E0%A0%80"),
    (&[0xAEEE], "%EA%BB%AE"),
    (&[0xD800, 0xDC00], "%F0%90%80%80"),
    (&[0xDBFF, 0xDFFF], "%F4%8F%BF%BF"),
    (&[0xE000], "%EE%80%80"),
  ];

  for &(input, expected) in cases {
    let enc = encode(input, true, usize::MAX)?;
    assert_eq!(String::from_utf8(enc)?, expected);
  }
  OK
}

// v8/test/mjsunit/uri.js:67-85
#[test]
fn uri_roundtrip() -> aok::Void {
  let strings = [
    "",
    "abcd",
    "\u{1234}\u{0123}\u{abcd}",
    "ab<\u{1234}\u{0123}",
    "ab\u{1234}<\u{0123}",
  ];

  for s in strings {
    let u16_in = to_u16(s);
    let enc = encode(&u16_in, true, usize::MAX)?;
    let enc_u16: Vec<u16> = enc.iter().map(|&b| u16::from(b)).collect();
    let dec = decode_to_string(&enc_u16, true)?;
    assert_eq!(dec, s);
  }
  OK
}

// v8/test/mjsunit/uri.js 错误边界
#[test]
fn uri_malformed() -> aok::Void {
  let malformed_inputs = ["%", "%1", "%1g", "%GG", "%E0%A0"];
  for m in malformed_inputs {
    let u16_in = to_u16(m);
    assert_eq!(decode(&u16_in, true), Err(UriError::Malformed));
  }

  // 孤立代理测试
  let lone_surrogate = [0xD800u16];
  assert_eq!(
    encode(&lone_surrogate, true, usize::MAX),
    Err(UriError::Malformed)
  );
  let lone_trail = [0xDC00u16];
  assert_eq!(
    encode(&lone_trail, true, usize::MAX),
    Err(UriError::Malformed)
  );

  OK
}

// v8/test/mjsunit/escape.js:36-52
#[test]
fn escape_unescape_basic() -> aok::Void {
  let unescaped_str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789@*_+-./";
  let input = to_u16(unescaped_str);
  let escaped = escape(&input, usize::MAX)?;
  assert_eq!(String::from_utf8(escaped.clone())?, unescaped_str);

  assert_eq!(
    String::from_utf8(escape(&to_u16(" / "), usize::MAX)?)?,
    "%20/%20"
  );
  assert_eq!(
    String::from_utf8(escape(&[0, b'0' as u16], usize::MAX)?)?,
    "%000"
  );

  let mixed = [0x41, 0x20, 0x42, 0x1234, 0, 0x20, 0x43];
  assert_eq!(
    String::from_utf8(escape(&mixed, usize::MAX)?)?,
    "A%20B%u1234%00%20C"
  );
  assert_eq!(
    String::from_utf8(escape(&[0x123u16], usize::MAX)?)?,
    "%u0123"
  );
  assert_eq!(
    String::from_utf8(escape(&[0xABCDu16], usize::MAX)?)?,
    "%uABCD"
  );
  assert_eq!(String::from_utf8(escape(&[0xABu16], usize::MAX)?)?, "%AB");
  assert_eq!(
    String::from_utf8(escape(&[b'\n' as u16], usize::MAX)?)?,
    "%0A"
  );

  let unesc = unescape(&to_u16("A%20B%u1234%00%20C"));
  assert_eq!(unesc, mixed);
  OK
}

// v8/test/mjsunit/escape.js:53-67
#[test]
fn escape_unescape_first_1000_chars() -> aok::Void {
  for i in 0..1000u16 {
    let orig = [i];
    let enc = escape(&orig, usize::MAX)?;
    let enc_u16: Vec<u16> = enc.iter().map(|&b| u16::from(b)).collect();
    let dec = unescape(&enc_u16);
    assert_eq!(dec, orig, "char {i:#06x} escape/unescape roundtrip 失败");
  }
  OK
}
