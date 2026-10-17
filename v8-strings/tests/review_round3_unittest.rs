//! v8-strings 第 3 轮独立对抗式终验审查测试
//!
//! 覆盖场景:
//! 1. SWAR 转换：64 位与 32 位位掩码在各种特殊字符（0x80、0xFF、边界 ASCII、连续高位、非连续高位）下的行为，
//!    验证绝不下溢/上溢/污染相邻字节。
//! 2. Boyer-Moore 与 BMH 算法：单字符匹配、长模式串（>255 字节）、不匹配回退、重叠子串匹配（如 `aaaaa` 中搜 `aaa`）的步长计算正确性。
//! 3. UTF-8 解码状态机：Bjoern Hoehrmann DFA 状态转换完整性，非最短编码（Overlong UTF-8）、孤立高/低代理对（Surrogate halves 0xD800..0xDFFF）、超范围标量（>0x10FFFF）的拒绝与截断处理。
//! 4. StringStream：各类格式说明符（`%s`, `%d`, `%o`, `%x`, `%02x`, `%p`, `%c`, `%k`）的负数、边界长度、缓冲区满/溢出时的截断安全。
//! 5. Rapidhash：在任意切片长度、不同对齐方式下的哈希一致性，单字节与双字节（Latin1 字符升格为 Utf16）哈希等价性。

use aok::{OK, Void};
use v8_strings::{
  hash_seed::HashSeedData,
  string_case::{CaseMapping, fast_ascii_case_prefix_length, fast_ascii_convert},
  string_hasher::{hash_sequential_one_byte, hash_sequential_two_byte, is_only_8_bit},
  string_search::{StringSearch, search_string_u8},
  string_stream::{FixedStringAllocator, FmtElm, HeapStringAllocator, StringStream},
  unicode::{utf8::validate_encoding, wtf8},
  unicode_decoder::{StrictUtf8Decoder, Utf8Decoder, Wtf8Decoder},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// ── 1. SWAR 转换极端对抗 ──

#[test]
fn test_swar_edge_characters_and_high_bits() -> Void {
  // A. 边界 ASCII 符号测试：确保 '@' (0x40), '[' (0x5B), '`' (0x60), '{' (0x7B), 0x7E '~', 0x7F DEL 绝不误翻转或污染
  let test_boundaries = b"@A[\x60a{\x7E\x7F";
  let mut dst = vec![0u8; test_boundaries.len()];
  let converted = fast_ascii_convert(CaseMapping::ToLower, &mut dst, test_boundaries);
  assert_eq!(converted, test_boundaries.len());
  // '@' -> '@', 'A' -> 'a', '[' -> '[', '`' -> '`', 'a' -> 'a', '{' -> '{', '~' -> '~', 0x7F -> 0x7F
  assert_eq!(dst[0], b'@');
  assert_eq!(dst[1], b'a');
  assert_eq!(dst[2], b'[');
  assert_eq!(dst[3], b'`');
  assert_eq!(dst[4], b'a');
  assert_eq!(dst[5], b'{');
  assert_eq!(dst[6], b'~');
  assert_eq!(dst[7], 0x7F);

  // B. 连续高位与非连续高位字符：0x80、0xFF、连续高位、非连续高位
  // 必须在首个高位字节所在的 word 处立即截断，绝不污染前后相邻字节
  let mut src_words = [b'A'; 32];
  src_words[0] = 0x80; // word 0 第 0 字节高位
  let mut dst_words = [0u8; 32];
  let res = fast_ascii_convert(CaseMapping::ToLower, &mut dst_words, &src_words);
  assert_eq!(res, 0, "首字节为 0x80 必须返回 0");

  src_words = [b'A'; 32];
  src_words[7] = 0xFF; // word 0 末字节高位
  let res2 = fast_ascii_convert(CaseMapping::ToLower, &mut dst_words, &src_words);
  assert_eq!(res2, 0, "word 0 内含 0xFF 必须安全返回 0 且无下溢/上溢");

  src_words = [b'A'; 32];
  src_words[8] = 0x80;
  src_words[9] = 0xFF; // word 1 连续高位
  let res3 = fast_ascii_convert(CaseMapping::ToLower, &mut dst_words, &src_words);
  assert_eq!(res3, 8, "word 1 遇高位停止, word 0 正常转换 8 字节");
  assert_eq!(&dst_words[..8], &[b'a'; 8]);

  // C. fast_ascii_case_prefix_length 对各种边界字符的精确定位
  assert_eq!(
    fast_ascii_case_prefix_length(CaseMapping::ToLower, b"abcdef"),
    6
  );
  assert_eq!(
    fast_ascii_case_prefix_length(CaseMapping::ToLower, b"abcDef"),
    3
  );
  assert_eq!(
    fast_ascii_case_prefix_length(CaseMapping::ToLower, b"abcdef\x80"),
    6
  );
  assert_eq!(
    fast_ascii_case_prefix_length(CaseMapping::ToUpper, b"ABCDEF"),
    6
  );
  assert_eq!(
    fast_ascii_case_prefix_length(CaseMapping::ToUpper, b"ABCdEF"),
    3
  );

  OK
}

// ── 2. Boyer-Moore 与 BMH 算法极端对抗 ──

#[test]
fn test_boyer_moore_overlapping_and_single_char() -> Void {
  // A. 重叠子串搜寻：在 "aaaaa" 中搜 "aaa"
  let haystack = b"aaaaa";
  let needle = b"aaa";
  assert_eq!(search_string_u8(haystack, needle, 0), Some(0));
  assert_eq!(search_string_u8(haystack, needle, 1), Some(1));
  assert_eq!(search_string_u8(haystack, needle, 2), Some(2));
  assert_eq!(search_string_u8(haystack, needle, 3), None);

  // B. 单字符在两端与中间匹配
  assert_eq!(search_string_u8(b"x", b"x", 0), Some(0));
  assert_eq!(search_string_u8(b"axb", b"x", 0), Some(1));
  assert_eq!(search_string_u8(b"abx", b"x", 2), Some(2));
  assert_eq!(search_string_u8(b"abx", b"x", 3), None);

  // C. 长模式串 (>255 字节，如 280 字节)，验证超过 BM_MAX_SHIFT (250) 时末段建表与好后缀回退
  let mut long_needle = vec![b'y'; 280];
  long_needle[279] = b'z';
  let mut long_haystack = vec![b'y'; 500];
  long_haystack[120..400].copy_from_slice(&long_needle);

  let mut searcher = StringSearch::new_u8(&long_needle);
  assert_eq!(searcher.search(&long_haystack, 0), Some(120));
  assert_eq!(searcher.search(&long_haystack, 121), None);

  // 重复失配促使升级并验证 BM 策略下的重叠子串
  let repeat_needle = b"abababa"; // 长度 7，触发 Initial -> BMH / BM
  let repeat_haystack = b"abababababababa";
  let mut repeat_searcher = StringSearch::new_u8(repeat_needle);
  assert_eq!(repeat_searcher.search(repeat_haystack, 0), Some(0));
  assert_eq!(repeat_searcher.search(repeat_haystack, 1), Some(2));
  assert_eq!(repeat_searcher.search(repeat_haystack, 3), Some(4));

  OK
}

// ── 3. UTF-8 DFA 状态机对抗 ──

#[test]
fn test_utf8_dfa_adversarial_spec() -> Void {
  // A. 非最短编码 (Overlong UTF-8)
  // 2 字节非最短：0xC0 0x80 ~ 0xC1 0xBF
  assert!(!validate_encoding(&[0xC0, 0x80]));
  assert!(!validate_encoding(&[0xC1, 0xBF]));
  assert!(!wtf8::validate_encoding(&[0xC0, 0x80]));
  assert!(!wtf8::validate_encoding(&[0xC1, 0xBF]));

  // 3 字节非最短：0xE0 0x80 0x80 ~ 0xE0 0x9F 0xBF
  assert!(!validate_encoding(&[0xE0, 0x80, 0x80]));
  assert!(!validate_encoding(&[0xE0, 0x9F, 0xBF]));
  assert!(!wtf8::validate_encoding(&[0xE0, 0x80, 0x80]));
  assert!(!wtf8::validate_encoding(&[0xE0, 0x9F, 0xBF]));

  // 4 字节非最短：0xF0 0x80 0x80 0x80 ~ 0xF0 0x8F 0xBF 0xBF
  assert!(!validate_encoding(&[0xF0, 0x80, 0x80, 0x80]));
  assert!(!validate_encoding(&[0xF0, 0x8F, 0xBF, 0xBF]));
  assert!(!wtf8::validate_encoding(&[0xF0, 0x80, 0x80, 0x80]));
  assert!(!wtf8::validate_encoding(&[0xF0, 0x8F, 0xBF, 0xBF]));

  // B. 孤立代理对 (0xD800..0xDFFF: 0xED 0xA0 0x80 ~ 0xED 0xBF 0xBF)
  let d800 = [0xED, 0xA0, 0x80];
  let dfff = [0xED, 0xBF, 0xBF];
  // 严格 UTF-8 必须拒绝
  assert!(!validate_encoding(&d800));
  assert!(!validate_encoding(&dfff));
  // 广义 WTF-8 必须允许单孤立代理
  assert!(wtf8::validate_encoding(&d800));
  assert!(wtf8::validate_encoding(&dfff));

  // C. 超范围标量 (> 0x10FFFF)
  assert!(!validate_encoding(&[0xF4, 0x90, 0x80, 0x80])); // 0x110000
  assert!(!validate_encoding(&[0xF5, 0x80, 0x80, 0x80]));
  assert!(!validate_encoding(&[0xFF]));
  assert!(!wtf8::validate_encoding(&[0xF4, 0x90, 0x80, 0x80]));
  assert!(!wtf8::validate_encoding(&[0xF5, 0x80, 0x80, 0x80]));

  // D. 截断处理与错误恢复 (三解码器对比)
  let truncated_seq = [0xE4, 0xBD]; // 3 字节 UTF-8 缺少尾字节
  let dec_lossy = Utf8Decoder::new(&truncated_seq);
  assert!(!dec_lossy.is_invalid());
  assert_eq!(dec_lossy.utf16_length(), 1); // 替换为 1 个 BAD_CHAR

  let dec_strict = StrictUtf8Decoder::new(&truncated_seq);
  assert!(dec_strict.is_invalid());

  let dec_wtf8 = Wtf8Decoder::new(&truncated_seq);
  assert!(dec_wtf8.is_invalid());

  OK
}

// ── 4. StringStream 格式化与截断安全 ──

#[test]
fn test_string_stream_all_format_specifiers_and_truncation() -> Void {
  // A. 全说明符测试：%s, %d, %o, %x, %02x, %p, %c, %k
  let mut alloc = HeapStringAllocator::default();
  let mut ss = StringStream::new(&mut alloc);

  ss.add(
    "s=%s d=%d o=%o x=%x hex2=%02x p=%p c=%c k_asc=%k k_esc=%k",
    &[
      FmtElm::CStr("str"),
      FmtElm::Int(-123),
      FmtElm::Int(0o755),
      FmtElm::Int(0xabc),
      FmtElm::Int(5),
      FmtElm::Pointer(0x1000),
      FmtElm::Int(66),    // 'B'
      FmtElm::Char(0x21), // '!'
      FmtElm::Char(0x05), // '\x05'
    ],
  );
  let res = ss.to_string_lossy();
  assert!(res.contains("s=str"));
  assert!(res.contains("d=-123"));
  assert!(res.contains("o=755"));
  assert!(res.contains("x=abc"));
  assert!(res.contains("hex2=05"));
  assert!(res.contains("p=0x1000"));
  assert!(res.contains("c=B"));
  assert!(res.contains("k_asc=!"));
  assert!(res.contains("k_esc=\\x05"));

  // B. 负数对于各类说明符的安全性
  ss.reset();
  ss.add(
    "neg_d=%d neg_x=%x neg_02x=%02x neg_o=%o",
    &[
      FmtElm::Int(-1),
      FmtElm::Int(-1),
      FmtElm::Int(-1),
      FmtElm::Int(-1),
    ],
  );
  let neg_res = ss.to_string_lossy();
  assert!(neg_res.contains("neg_d=-1"));
  assert!(neg_res.contains("neg_x=ffffffff"));
  assert!(neg_res.contains("neg_02x=ffffffff"));
  assert!(neg_res.contains("neg_o=37777777777"));

  // C. 缓冲区满/溢出时的截断安全：各类定长容量（0, 1, 2, 3, 4, 5, 8）
  for cap in 0..=8 {
    let mut buf = vec![0u8; cap];
    let mut fix_alloc = FixedStringAllocator::new(&mut buf);
    let mut fixed_ss = StringStream::new(&mut fix_alloc);
    for b in b"ABCDEFGHIJKLMNOPQRSTUVWXYZ" {
      fixed_ss.put(*b);
    }
    // 必须安全不 panic
    let s = fixed_ss.to_string_lossy();
    if cap >= 5 {
      assert!(s.ends_with("...\n"));
    }
  }

  OK
}

// ── 5. Rapidhash 切片长度与跨表示等价性 ──

#[test]
fn test_rapidhash_slice_lengths_and_representation_invariants() -> Void {
  let seed = HashSeedData::DEFAULT;

  // A. 遍历 0 到 100 任意切片长度，验证单字节 Latin1 与双字节 Utf16 严格等价
  for len in 0..=100 {
    let raw_bytes: Vec<u8> = (0..len).map(|i| (i % 256) as u8).collect();
    let utf16_chars: Vec<u16> = raw_bytes.iter().map(|&b| u16::from(b)).collect();

    assert!(is_only_8_bit(&utf16_chars));

    let mut ob1 = false;
    let mut ob2 = false;
    let h1 = hash_sequential_one_byte(&raw_bytes, &seed, Some(&mut ob1));
    let h2 = hash_sequential_two_byte(&utf16_chars, &seed, Some(&mut ob2));

    assert_eq!(
      h1, h2,
      "切片长度 {len} 下单字节与双字节 Latin1 内容哈希必须严格相等"
    );
    assert_eq!(ob1, ob2);
  }

  // B. 非对齐切片（子切片）的一致性
  let full_buffer: Vec<u8> = (0..200).map(|i| (i % 250) as u8).collect();
  let full_u16: Vec<u16> = full_buffer.iter().map(|&b| u16::from(b)).collect();

  for offset in [1, 3, 7, 15] {
    let sub_bytes = &full_buffer[offset..offset + 64];
    let sub_u16 = &full_u16[offset..offset + 64];

    let h1 = hash_sequential_one_byte(sub_bytes, &seed, None);
    let h2 = hash_sequential_two_byte(sub_u16, &seed, None);

    assert_eq!(
      h1, h2,
      "子切片偏移 {offset} 下单字节与双字节哈希必须严格一致"
    );
  }

  OK
}
