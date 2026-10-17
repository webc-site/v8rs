//! UTF-8 转换与 Latin-1/ASCII 判定规约 (cctest/test-strings.cc:881/1141/1612 +
//! string.h:622-672 + string-inl.h:1244-1305)。

pub mod common;

use aok::{OK, Void};
use common::{Arena, Maps};
use v8_objects_strings::string::{
  FlatContent, UTF8_FLAG_NULL_TERMINATE, UTF8_FLAG_REPLACE_INVALID, is_ascii, is_one_byte_uc16,
  is_well_formed_unicode_flat, non_one_byte_start, to_cstring, to_std_string, utf8_length_flat,
  write_utf8_flat,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

fn one_byte(chars: &'static [u8]) -> FlatContent<'static> {
  FlatContent::OneByte(chars)
}

fn two_byte(chars: &'static [u16]) -> FlatContent<'static> {
  FlatContent::TwoByte(chars)
}

/// Utf8Conversion (cctest:881): 长度预算与编码字节一致性。
#[test]
fn utf8_length_and_encode() -> Void {
  // ASCII 直通
  assert_eq!(utf8_length_flat(&one_byte(b"hello")), 5);
  // Latin-1 高位 (2 字节/字符)
  assert_eq!(
    utf8_length_flat(&one_byte(&[0x00, 0x7F, 0x80, 0xFF])),
    1 + 1 + 2 + 2
  );
  // UTF-16: BMP 2 字节, 代理对 4 字节
  let pairs: &'static [u16] = &[0x41, 0xE9, 0x4E2D, 0xD83D, 0xDE00]; // A é 中 😀
  assert_eq!(utf8_length_flat(&two_byte(pairs)), 1 + 2 + 3 + 4);
  // 孤立代理: lead 按独立 3 字节计
  let lone: &'static [u16] = &[0xD800];
  assert_eq!(utf8_length_flat(&two_byte(lone)), 3);

  // 编码: 写入字节数 == 预算, 字节逐一对上
  let content = one_byte(&[0x61, 0xE9, 0xFF]);
  let n = utf8_length_flat(&content);
  let mut buf = vec![0u8; n];
  let (written, processed) = write_utf8_flat(&content, &mut buf, 0);
  assert_eq!(written, n);
  assert_eq!(processed, 3);
  assert_eq!(&buf, &[0x61, 0xC3, 0xA9, 0xC3, 0xBF]);

  // 代理对编码 (surrogate pair → 4 字节)
  let mut buf = vec![0u8; 10];
  let (written, processed) = write_utf8_flat(&two_byte(pairs), &mut buf, 0);
  assert_eq!((written, processed), (10, 5));
  // 尾段 😀 = F0 9F 98 80
  assert_eq!(&buf[6..10], &[0xF0, 0x9F, 0x98, 0x80]);
  // 首字符 A + é(C3 A9) + 中(E4 B8 AD)
  assert_eq!(&buf[..3], &[0x41, 0xC3, 0xA9]);
  assert_eq!(&buf[3..6], &[0xE4, 0xB8, 0xAD]);
  // 首字符 A
  let mut small = vec![0u8; 1];
  let (w, p) = write_utf8_flat(&two_byte(&pairs[..1]), &mut small, 0);
  assert_eq!((w, p), (1, 1));
  assert_eq!(&small, b"A");

  // NUL 终止旗标: 尾字节 0 且计入 written
  let mut buf = vec![0xFFu8; 6];
  let (w, _) = write_utf8_flat(&one_byte(b"hello"), &mut buf, UTF8_FLAG_NULL_TERMINATE);
  assert_eq!(w, 6);
  assert_eq!(buf[5], 0);
  OK
}

/// ReplaceInvalidUtf8 (cctest:1141): 孤立代理 → U+FFFD。
#[test]
fn utf8_replace_invalid() -> Void {
  let lone: &'static [u16] = &[0xD800, 0x41]; // 孤立 lead + 'A'
  let content = two_byte(lone);
  let mut buf = vec![0u8; 16];
  let (w, p) = write_utf8_flat(&content, &mut buf, UTF8_FLAG_REPLACE_INVALID);
  assert_eq!(p, 2);
  assert_eq!(&buf[..w], &[0xEF, 0xBF, 0xBD, b'A']);

  // 不替换: 孤立 lead 原样 3 字节 UTF-8 (CESU-8 语义)
  let mut buf = vec![0u8; 16];
  let (w, p) = write_utf8_flat(&content, &mut buf, 0);
  assert_eq!(p, 2);
  assert_eq!(&buf[..w], &[0xED, 0xA0, 0x80, b'A']);
  OK
}

/// IsWellFormedUnicode (string-inl.h:1273)。
#[test]
fn well_formed_unicode() -> Void {
  assert!(is_well_formed_unicode_flat(&one_byte(b"anything")));
  let ok: &'static [u16] = &[0x41, 0xD83D, 0xDE00, 0x42];
  assert!(is_well_formed_unicode_flat(&two_byte(ok)));
  for bad in [
    &[0xD800u16][..],
    &[0xDC00u16][..],
    &[0xD83D, 0x41][..],
    &[0x41, 0xD83D][..],
  ] {
    // 静态提升 (bad 为 &'static 经 const 提升)
    let bad: &'static [u16] = bad;
    assert!(!is_well_formed_unicode_flat(&two_byte(bad)));
  }
  OK
}

/// ToCString/ToStdString (string.cc:732-778): NUL → 空格替换 + NUL 结尾。
#[test]
fn to_cstring_nul_replacement() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  let s = arena.seq_one_byte(maps.seq_one, b"a\x00b");
  // SAFETY: 竞技场对象布局契约成立
  unsafe {
    let bytes = to_cstring(s, 0, 3).ok().unwrap();
    assert_eq!(bytes, b"a b\0");
    let out = to_std_string(s).ok().unwrap();
    assert_eq!(out, "a b");
    // 双字节 + 代理对
    let pairs: Vec<u16> = "中😀".encode_utf16().collect();
    let t = arena.seq_two_byte(maps.seq_two, &pairs);
    assert_eq!(to_std_string(t).ok().unwrap(), "中😀");
    // 区间版
    let u = to_cstring(s, 1, 2).ok().unwrap();
    assert_eq!(u, b" b\0");
  }
  OK
}

/// NonOneByteStart/IsOneByte/IsAscii (string.h:622-672)。
#[test]
fn non_one_byte_start_swar() -> Void {
  // 纯 Latin-1 → length
  assert_eq!(non_one_byte_start(&[0x41, 0xFF, 0x00, 0x7F]), 4);
  // 首字符即非 Latin-1
  assert_eq!(non_one_byte_start(&[0x100, 0x41]), 0);
  // 中段命中 (对齐前 / 字内 / 尾部三态各测)
  for pos in [1usize, 3, 4, 5, 7, 8, 9, 15, 16, 31] {
    let mut v = vec![0x61u16; 40];
    v[pos] = 0x2028;
    assert_eq!(non_one_byte_start(&v), pos as u32, "pos={pos}");
    assert!(!is_one_byte_uc16(&v));
  }
  // 全 Latin-1 长串 (跨 word 边界)
  let v = vec![0xABu16; 100];
  assert_eq!(non_one_byte_start(&v), 100);
  assert!(is_one_byte_uc16(&v));
  // ASCII 判定
  assert!(is_ascii(b"plain ascii \x7F"));
  assert!(!is_ascii(&[0x00, 0x7F, 0x80]));
  assert!(is_ascii(b""));
  OK
}
