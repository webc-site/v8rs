// 字符串搜索策略集成测试 (v8/src/strings/string-search.h 语义)

use aok::OK;
use v8_strings::string_search::{
  Strategy, StringSearch, search_string_u8, search_string_u8_in_u16, search_string_u16,
  search_string_u16_in_u8,
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

#[test]
fn single_char_search() -> aok::Void {
  let subject = b"hello world";
  assert_eq!(search_string_u8(subject, b"h", 0), Some(0));
  assert_eq!(search_string_u8(subject, b"o", 0), Some(4));
  assert_eq!(search_string_u8(subject, b"o", 5), Some(7));
  assert_eq!(search_string_u8(subject, b"d", 0), Some(10));
  assert_eq!(search_string_u8(subject, b"z", 0), None);

  let mut searcher = StringSearch::new_u8(b"w");
  assert_eq!(searcher.strategy(), Strategy::SingleChar);
  assert_eq!(searcher.search(subject, 0), Some(6));
  OK
}

#[test]
fn linear_search() -> aok::Void {
  let subject = b"abcdefghijklmnop";
  assert_eq!(search_string_u8(subject, b"cde", 0), Some(2));
  assert_eq!(search_string_u8(subject, b"nop", 0), Some(13));
  assert_eq!(search_string_u8(subject, b"xyz", 0), None);

  let mut searcher = StringSearch::new_u8(b"def");
  assert_eq!(searcher.strategy(), Strategy::Linear);
  assert_eq!(searcher.search(subject, 0), Some(3));
  OK
}

#[test]
fn boyer_moore_search() -> aok::Void {
  // 长度 >= 7 进入 InitialSearch 并自适应升降级
  let pattern = b"longer_pattern";
  let mut searcher = StringSearch::new_u8(pattern);
  assert_eq!(searcher.strategy(), Strategy::Initial);

  let subject = b"this is a much longer text containing longer_pattern inside it!";
  assert_eq!(searcher.search(subject, 0), Some(38));

  // 重复搜索引发降级/升级
  let bad_subject = b"longer_pattx longer_patty longer_pattz longer_pattern";
  assert_eq!(searcher.search(bad_subject, 0), Some(39));
  OK
}

#[test]
fn u16_search_and_mixed() -> aok::Void {
  let s_u16: Vec<u16> = "JavaScript 引擎字符串搜索测试".encode_utf16().collect();
  let p_u16: Vec<u16> = "字符串".encode_utf16().collect();

  assert_eq!(search_string_u16(&s_u16, &p_u16, 0), Some(13));

  // 单字节 subject 与双字节 pattern (含超 0xFF 字符必败)
  let s_u8 = b"hello world";
  let p_out_of_range = [0x1234u16];
  assert_eq!(search_string_u16_in_u8(s_u8, &p_out_of_range, 0), None);

  let p_ascii_u16 = [b'w' as u16, b'o' as u16];
  assert_eq!(search_string_u16_in_u8(s_u8, &p_ascii_u16, 0), Some(6));

  // 双字节 subject 与单字节 pattern
  assert_eq!(search_string_u8_in_u16(&s_u16, b"Script", 0), Some(4));
  OK
}

#[test]
fn boundary_conditions() -> aok::Void {
  let s = b"short";
  assert_eq!(search_string_u8(s, b"", 0), None);
  assert_eq!(
    search_string_u8(s, b"very long pattern exceeds subject", 0),
    None
  );
  assert_eq!(search_string_u8(s, b"short", 0), Some(0));
  assert_eq!(search_string_u8(s, b"short", 1), None);

  // 单字符越界安全: index >= subject.len() 不 panic
  assert_eq!(search_string_u8(b"hello", b"l", 5), None);
  assert_eq!(search_string_u8(b"hello", b"l", 10), None);

  // Searcher 实例直接传入短 subject 长度下溢防护
  let mut searcher = StringSearch::new_u8(b"pattern_long");
  assert_eq!(searcher.search(b"short", 0), None);
  assert_eq!(searcher.search(b"", 0), None);
  assert_eq!(searcher.search(b"short", 10), None);
  OK
}

#[test]
fn boyer_moore_long_pattern_over_250_chars() -> aok::Void {
  // 长度 > BM_MAX_SHIFT (250) 验证 start > 0 边界
  let mut pattern = vec![b'a'; 260];
  pattern[259] = b'b';

  let mut searcher = StringSearch::new_u8(&pattern);
  let mut subject = vec![b'a'; 300];
  subject[299] = b'b'; // 目标在偏移 40 处

  // 必须正确匹配
  assert_eq!(searcher.search(&subject, 0), Some(40));

  // 末字符失配
  subject[299] = b'c';
  assert_eq!(searcher.search(&subject, 0), None);
  OK
}
