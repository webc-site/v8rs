// ASCII 大小写快速转换测试 (v8/src/strings/string-case.h 语义)

use aok::OK;
use v8_strings::string_case::{CaseMapping, fast_ascii_case_prefix_length, fast_ascii_convert};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

#[test]
fn case_prefix_length() -> aok::Void {
  let lower = b"already all lower case ascii text here";
  assert_eq!(
    fast_ascii_case_prefix_length(CaseMapping::ToLower, lower),
    lower.len()
  );

  let upper = b"ALREADY ALL UPPER CASE ASCII TEXT HERE";
  assert_eq!(
    fast_ascii_case_prefix_length(CaseMapping::ToUpper, upper),
    upper.len()
  );

  let mixed = b"abcdefghIJK";
  assert_eq!(
    fast_ascii_case_prefix_length(CaseMapping::ToLower, mixed),
    8
  );
  assert_eq!(
    fast_ascii_case_prefix_length(CaseMapping::ToUpper, b"ABCDEFGHijk"),
    8
  );

  // 遇大写字符 'D' 停在索引 3
  assert_eq!(
    fast_ascii_case_prefix_length(CaseMapping::ToLower, b"abcDEFghi"),
    3
  );

  // 非 ASCII 字符停步
  let with_non_ascii = b"hello \x80world";
  assert_eq!(
    fast_ascii_case_prefix_length(CaseMapping::ToLower, with_non_ascii),
    6
  );
  OK
}

#[test]
fn case_convert_across_words() -> aok::Void {
  // 超过 8 字节 (usize word) 的 SWAR 转换
  let src = b"Hello, World! This is a test of fast_ascii_convert SWAR logic.";
  let mut dst = vec![0u8; src.len()];

  let converted_len = fast_ascii_convert(CaseMapping::ToLower, &mut dst, src);
  assert_eq!(converted_len, src.len());
  assert_eq!(
    &dst,
    b"hello, world! this is a test of fast_ascii_convert swar logic."
  );

  let mut dst_upper = vec![0u8; src.len()];
  let converted_upper_len = fast_ascii_convert(CaseMapping::ToUpper, &mut dst_upper, src);
  assert_eq!(converted_upper_len, src.len());
  assert_eq!(
    &dst_upper,
    b"HELLO, WORLD! THIS IS A TEST OF FAST_ASCII_CONVERT SWAR LOGIC."
  );
  OK
}

#[test]
fn case_convert_non_ascii_stops() -> aok::Void {
  let src = b"quick brown fox \xFE jumps over";
  let mut dst = vec![0u8; src.len()];
  let processed = fast_ascii_convert(CaseMapping::ToUpper, &mut dst, src);
  assert_eq!(processed, 16);
  assert_eq!(&dst[..16], b"QUICK BROWN FOX ");
  OK
}
