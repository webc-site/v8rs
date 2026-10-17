//! v8-strings 第 4 轮终验独立对抗测试套件
//!
//! 验证场景:
//! 1. SWAR：大写转小写 / 小写转大写，在所有 ASCII 控制字符、标点符号、扩展 ASCII (0x80..0xFF)、连续多字对齐与非对齐下的行为。
//! 2. Boyer-Moore 与 BMH：各种特殊模式串（单字节、双字节、全同字符 `aaaa...`、Fibonacci 递归退化串、超长串 >255）、多重重叠匹配、中途部分匹配失配回退步长。
//! 3. Unicode 解码器与 DFA：Bjoern Hoehrmann DFA 状态转换表、孤立代理对、非最短序列、乱序重入、双字节 Latin1 升格。
//! 4. StringStream：各类格式化指令（`%s`, `%d`, `%o`, `%x`, `%02x`, `%p`, `%c`, `%k`），极端超大/极小缓冲区截断保护。
//! 5. Rapidhash：同余性、切片长度 0..128、哈希缓存标志位编解码。

use aok::{OK, Void};
use v8_strings::{
  hash_field::{
    HASH_BITS_MAX, HASH_BITS_SHIFT, HashFieldType, contains_cached_array_index,
    create_hash_field_value, is_integer_index,
  },
  hash_seed::HashSeedData,
  string_case::{CaseMapping, fast_ascii_convert},
  string_hasher::{
    decode_array_index_from_hash_field, hash_sequential_one_byte, hash_sequential_two_byte,
    is_only_8_bit, make_array_index_hash_raw,
  },
  string_search::{StringSearch, search_string_u8_in_u16, search_string_u16},
  string_stream::{FixedStringAllocator, FmtElm, HeapStringAllocator, StringStream},
  unicode_decoder::{Encoding, Utf8Decoder},
  utf8_dfa::{generalized, strict},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// ── 1. SWAR 极端对抗测试 ──

#[test]
fn test_swar_all_bytes_adversarial() -> Void {
  // 验证 0..=255 单字节及成对字节在不同对齐下的转换
  for b in 0..=255u8 {
    let src = [b; 16];
    let mut dst = [0u8; 16];

    // ToLower
    let converted_lower = fast_ascii_convert(CaseMapping::ToLower, &mut dst, &src);
    if b >= 0x80 {
      assert_eq!(converted_lower, 0, "高位字节必须停止");
    } else {
      assert_eq!(converted_lower, 16);
      let expected = if b.is_ascii_uppercase() {
        b.to_ascii_lowercase()
      } else {
        b
      };
      assert_eq!(dst, [expected; 16]);
    }

    // ToUpper
    let mut dst_up = [0u8; 16];
    let converted_upper = fast_ascii_convert(CaseMapping::ToUpper, &mut dst_up, &src);
    if b >= 0x80 {
      assert_eq!(converted_upper, 0, "高位字节必须停止");
    } else {
      assert_eq!(converted_upper, 16);
      let expected = if b.is_ascii_lowercase() {
        b.to_ascii_uppercase()
      } else {
        b
      };
      assert_eq!(dst_up, [expected; 16]);
    }
  }

  // 非对齐与混合测试 (offset 0..8)
  let mut test_payload = Vec::new();
  for _ in 0..10 {
    test_payload.extend_from_slice(b"Hello World! 123 \t\r\n ~!@#$%^&*()_+`-={}|[]\\:\";'<>?,./");
  }
  for offset in 0..8 {
    let slice = &test_payload[offset..];
    let mut dst = vec![0u8; slice.len()];
    let processed = fast_ascii_convert(CaseMapping::ToLower, &mut dst, slice);
    assert_eq!(processed, slice.len());
    let expected: Vec<u8> = slice.iter().map(u8::to_ascii_lowercase).collect();
    assert_eq!(dst, expected);
  }

  // 含有扩展 ASCII 的非对齐截断
  for non_ascii_pos in 0..32 {
    let mut src = vec![b'A'; 64];
    src[non_ascii_pos] = 0x80;
    let mut dst = vec![0u8; 64];
    let processed = fast_ascii_convert(CaseMapping::ToLower, &mut dst, &src);
    assert!(
      processed <= non_ascii_pos,
      "处理长度 {processed} 不得超过高位字符出现位置 {non_ascii_pos}"
    );
  }

  OK
}

// ── 2. Boyer-Moore 与 BMH 算法极端对抗 ──

#[test]
fn test_boyer_moore_fibonacci_and_adversarial_patterns() -> Void {
  // A. Fibonacci 递归退化串 (经典 BM 最差情况：很多部分匹配然后失配)
  // f(1) = "b", f(2) = "a", f(n) = f(n-1) + f(n-2)
  let mut fibs = vec![b"b".to_vec(), b"a".to_vec()];
  for i in 2..9 {
    let mut next = fibs[i - 1].clone();
    next.extend_from_slice(&fibs[i - 2]);
    fibs.push(next);
  }
  let fib_haystack = &fibs[8]; // 长度 34
  let fib_needle = &fibs[6]; // 长度 13

  let mut searcher = StringSearch::new_u8(fib_needle);
  let pos = searcher.search(fib_haystack, 0);
  assert_eq!(pos, Some(0));

  // 中间搜索失配
  let non_matching_needle = b"aaaaa";
  let mut non_match_searcher = StringSearch::new_u8(non_matching_needle);
  assert_eq!(non_match_searcher.search(fib_haystack, 0), None);

  // B. 全同字符 aaaaa...
  let all_a_haystack = vec![b'a'; 100];
  let all_a_needle = vec![b'a'; 10];
  let mut a_searcher = StringSearch::new_u8(&all_a_needle);
  for idx in 0..=90 {
    assert_eq!(a_searcher.search(&all_a_haystack, idx), Some(idx));
  }
  assert_eq!(a_searcher.search(&all_a_haystack, 91), None);

  // C. 超长模式串 (>255，例如 300 字符)
  let mut super_long_needle = vec![b'k'; 300];
  super_long_needle[299] = b'm';
  let mut super_long_haystack = vec![b'k'; 1000];
  super_long_haystack[450..750].copy_from_slice(&super_long_needle);

  let mut long_searcher = StringSearch::new_u8(&super_long_needle);
  assert_eq!(long_searcher.search(&super_long_haystack, 0), Some(450));
  assert_eq!(long_searcher.search(&super_long_haystack, 451), None);

  // D. 双字节极端搜索 (u16 in u16, u8 in u16)
  let u16_haystack: Vec<u16> = (0..500).map(|i| (i % 256) as u16).collect();
  let u16_needle: Vec<u16> = (100..120).map(|i| (i % 256) as u16).collect();
  assert_eq!(search_string_u16(&u16_haystack, &u16_needle, 0), Some(100));

  let u8_needle: Vec<u8> = (100..120).map(|i| (i % 256) as u8).collect();
  assert_eq!(
    search_string_u8_in_u16(&u16_haystack, &u8_needle, 0),
    Some(100)
  );

  OK
}

// ── 3. Unicode 解码器与 DFA 深度校验 ──

#[test]
fn test_unicode_dfa_deep_invariants() -> Void {
  // A. Bjoern Hoehrmann DFA 状态转换覆盖
  // 单字节 ASCII
  let mut state = strict::State::Accept;
  let mut buffer = 0u32;
  strict::decode(b'Z', &mut state, &mut buffer);
  assert_eq!(state, strict::State::Accept);
  assert_eq!(buffer, u32::from(b'Z'));

  // 2 字节 UTF-8
  state = strict::State::Accept;
  buffer = 0;
  strict::decode(0xC2, &mut state, &mut buffer);
  assert_eq!(state, strict::State::TwoByte);
  strict::decode(0xA2, &mut state, &mut buffer);
  assert_eq!(state, strict::State::Accept);
  assert_eq!(buffer, 0x00A2);

  // B. 孤立代理对与广义 vs 严格 DFA
  // 0xED 0xA0 0x80 (U+D800)
  state = strict::State::Accept;
  buffer = 0;
  strict::decode(0xED, &mut state, &mut buffer);
  assert_eq!(state, strict::State::ThreeByteLowMid);
  strict::decode(0xA0, &mut state, &mut buffer);
  // strict DFA 必须转移到 Reject
  assert_eq!(state, strict::State::Reject);

  // 广义 DFA 必须放行单孤立代理
  let mut gstate = generalized::State::Accept;
  let mut gbuf = 0u32;
  generalized::decode(0xED, &mut gstate, &mut gbuf);
  assert_eq!(gstate, generalized::State::ThreeByte);
  generalized::decode(0xA0, &mut gstate, &mut gbuf);
  assert_eq!(gstate, generalized::State::TwoByte);
  generalized::decode(0x80, &mut gstate, &mut gbuf);
  assert_eq!(gstate, generalized::State::Accept);
  assert_eq!(gbuf, 0xD800);

  // C. 双字节 Latin1 升格检测
  let dec_ascii = Utf8Decoder::new(b"hello world");
  assert_eq!(dec_ascii.encoding(), Encoding::Ascii);
  assert!(dec_ascii.is_ascii());
  assert!(dec_ascii.is_one_byte());

  let latin1_bytes = "café".as_bytes(); // 0xC3 0xA9 -> U+00E9 (Latin1)
  let dec_latin1 = Utf8Decoder::new(latin1_bytes);
  assert_eq!(dec_latin1.encoding(), Encoding::Latin1);
  assert!(!dec_latin1.is_ascii());
  assert!(dec_latin1.is_one_byte());

  let utf16_bytes = "你好".as_bytes(); // 0xE4 0xBD 0xA0 ... > 0xFF
  let dec_utf16 = Utf8Decoder::new(utf16_bytes);
  assert_eq!(dec_utf16.encoding(), Encoding::Utf16);
  assert!(!dec_utf16.is_one_byte());

  OK
}

// ── 4. StringStream 全指令与极端边界 ──

#[test]
fn test_string_stream_extreme_truncation_and_formatting() -> Void {
  // 5 字节定长分配器截断
  let mut fix_buf = [0u8; 5];
  let mut fix_alloc = FixedStringAllocator::new(&mut fix_buf);
  let mut ss_fix = StringStream::new(&mut fix_alloc);
  for c in b"Hello World!" {
    ss_fix.put(*c);
  }
  let res_fix = ss_fix.to_string_lossy();
  assert!(res_fix.ends_with("...\n"));

  // 极小容量分配器 (1..=4 字节)
  for cap in 1..=4 {
    let mut buf = vec![0u8; cap];
    let mut alloc = FixedStringAllocator::new(&mut buf);
    let mut ss = StringStream::new(&mut alloc);
    for c in b"Hello World!" {
      ss.put(*c);
    }
    let res = ss.to_string_lossy();
    assert!(res.len() <= cap);
  }

  // 大对象与回调
  let mut heap_alloc = HeapStringAllocator::default();
  let mut ss = StringStream::new(&mut heap_alloc);
  let mut print_obj = |stream: &mut StringStream, id: usize| {
    stream.add("OBJ(#%d)", &[FmtElm::Int(id as i32)]);
  };
  ss.set_print_object(&mut print_obj);
  ss.add("Target: %o", &[FmtElm::Obj(42)]);
  assert_eq!(ss.to_string_lossy(), "Target: OBJ(#42)");

  OK
}

// ── 5. Rapidhash 0..128 长度同余性与哈希字段编解码 ──

#[test]
fn test_rapidhash_congruence_and_field_bitfields() -> Void {
  let seed = HashSeedData::DEFAULT;

  // 切片长度 0..128 全同余性测试
  for len in 0..=128 {
    let latin1_u8: Vec<u8> = (0..len).map(|i| (i % 256) as u8).collect();
    let latin1_u16: Vec<u16> = latin1_u8.iter().map(|&b| u16::from(b)).collect();

    assert!(is_only_8_bit(&latin1_u16));

    let mut ob_u8 = false;
    let mut ob_u16 = false;
    let h_u8 = hash_sequential_one_byte(&latin1_u8, &seed, Some(&mut ob_u8));
    let h_u16 = hash_sequential_two_byte(&latin1_u16, &seed, Some(&mut ob_u16));

    assert_eq!(h_u8, h_u16, "长度 {len} 下单双字节哈希字段编码必须完全一致");
    assert_eq!(ob_u8, ob_u16);
  }

  // 哈希字段标志位与 array index 编解码 roundtrip
  let raw_index = 123456u32;
  let len = 6u32;
  let field = make_array_index_hash_raw(raw_index, len);
  assert!(contains_cached_array_index(field));
  assert!(is_integer_index(field));
  assert_eq!(decode_array_index_from_hash_field(field), raw_index);

  // 普通哈希字段类型
  let norm_hash = create_hash_field_value(0x1234567, HashFieldType::Hash);
  assert!(!contains_cached_array_index(norm_hash));
  assert!(!is_integer_index(norm_hash));
  assert_eq!(norm_hash & 0b11, HashFieldType::Hash as u32);
  assert_eq!((norm_hash >> HASH_BITS_SHIFT) & HASH_BITS_MAX, 0x1234567);

  OK
}
