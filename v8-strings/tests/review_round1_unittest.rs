//! v8-strings 运行时第 1 轮对抗式审查边界测试
//!
//! 覆盖场景:
//! 1. Boyer-Moore & BMH 算法极端对抗：好后缀与坏字符表位移、周期串多重回退、超长模式 (>250) 截断、跨宽度 Fail 分派
//! 2. StringHasher & HashField 编码与 Rapidhash：数组索引极值边界、安全整数边界、前导零、cached index 防撞、1字节与2字节相同内容哈希等价
//! 3. SWAR 大小写快速转换边界：不足 1 word、跨 word、邻接非字母 ASCII 符号、非 ASCII 字节截断、切片长度不对等防御
//! 4. DFA UTF-8 校验与解码：严格 DFA vs 泛化 DFA、孤立代理 vs 代理对、超长编码、增量解码非 ASCII 中断与回退重试
//! 5. 诊断流 StringStream：定长极小缓冲区截断、扩容倍增、全指令格式化 (%s/%w/%k/%d/%x/%X/%u/%c/%f/%e/%p)、空缓冲防御
//! 6. URI 编解码对抗：保留字符集细分、非法十六进制、截断多字节序列、U+FFFD 放行、escape/unescape 宽字符

use std::f64::consts::PI;

use aok::{OK, Void};
use v8_strings::{
  hash_field::{
    HashFieldType, MAX_ARRAY_INDEX, MAX_HASH_CALC_LENGTH, contains_cached_array_index,
    is_integer_index,
  },
  hash_seed::HashSeedData,
  string_case::{CaseMapping, fast_ascii_case_prefix_length, fast_ascii_convert},
  string_hasher::{
    RunningStringHasher, ZERO_HASH, decode_array_index_from_hash_field, get_trivial_hash,
    hash_sequential_one_byte, hash_sequential_two_byte, is_only_8_bit,
  },
  string_search::{
    Strategy, StringSearch, search_string_u8, search_string_u8_in_u16, search_string_u16,
    search_string_u16_in_u8,
  },
  string_stream::{FixedStringAllocator, FmtElm, HeapStringAllocator, StringStream},
  unicode::{
    utf8::{BAD_CHAR, INCOMPLETE, validate_encoding, value_of_incremental},
    utf16::{has_unpaired_surrogate, replace_unpaired_surrogates},
    wtf8,
  },
  unicode_decoder::{Encoding, StrictUtf8Decoder, Utf8Decoder, Wtf8Decoder},
  uri::{UriError, decode, encode as uri_encode, escape, unescape},
  utf8_dfa::strict,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// ── 1. Boyer-Moore & BMH 算法极端对抗 ──

#[test]
fn test_boyer_moore_adversarial_shifts_and_fallbacks() -> Void {
  // 场景 A: 周期串极度对抗 "aaaa...aab" 在 "aaaa...aaaa"
  // 频繁首尾部分命中，迫使 badness 迅速越界并从 Initial -> BMH -> BoyerMoore 升阶
  let needle = b"aaaaaab";
  let mut haystack = vec![b'a'; 1000];
  haystack.extend_from_slice(b"aaaaaab");
  haystack.extend_from_slice(b"aaaa");

  let mut searcher = StringSearch::new_u8(needle);
  assert_eq!(searcher.strategy(), Strategy::Initial);
  let pos = searcher.search(&haystack, 0);
  assert_eq!(pos, Some(1000));
  // 经过大量坏匹配后，必须已升级到 BM 或 BMH
  assert!(matches!(
    searcher.strategy(),
    Strategy::BoyerMoore | Strategy::BoyerMooreHorspool
  ));

  // 场景 B: 超过 BM_MAX_SHIFT (250) 的超长模式
  // 验证好后缀与坏字符表建立在截断末段时的位移与回退逻辑
  let mut long_pattern = vec![b'x'; 300];
  long_pattern[295..300].copy_from_slice(b"MATCH");
  let mut long_haystack = vec![b'x'; 600];
  long_haystack[250..550].copy_from_slice(&long_pattern);

  let mut long_searcher = StringSearch::new_u8(&long_pattern);
  assert_eq!(long_searcher.search(&long_haystack, 0), Some(250));
  assert_eq!(long_searcher.search(&long_haystack, 251), None);

  // 场景 C: 边界索引对抗
  assert_eq!(search_string_u8(b"hello", b"hello", 0), Some(0));
  assert_eq!(search_string_u8(b"hello", b"hello", 1), None);
  assert_eq!(search_string_u8(b"hello", b"hello", 100), None);
  assert_eq!(search_string_u8(b"short", b"longer_pattern", 0), None);
  assert_eq!(search_string_u8(b"hello", b"", 0), None);
  assert_eq!(search_string_u8(b"", b"", 0), None);
  assert_eq!(search_string_u8(b"", b"a", 0), None);

  OK
}

#[test]
fn test_string_search_widths_and_fail_strategy() -> Void {
  // 双字节 pattern 含非 Latin-1 (>0xFF) 字符时在单字节 subject 中必败
  let u16_pattern = [0x0061u16, 0x1234u16, 0x0062u16];
  let u8_subject = b"abc";
  let mut searcher = StringSearch::new_u16(&u16_pattern);
  assert_eq!(searcher.search_u8(u8_subject, 0), None);
  // 应标记为 Fail
  assert_eq!(searcher.strategy(), Strategy::Fail);

  // u16 in u8 一次性入口同样必须被拒绝
  assert_eq!(search_string_u16_in_u8(u8_subject, &u16_pattern, 0), None);

  // u8 in u16: 单字节模式在双字节目标中命中
  let u8_pat = b"search";
  let u16_sub: Vec<u16> = "prefix search suffix".encode_utf16().collect();
  assert_eq!(search_string_u8_in_u16(&u16_sub, u8_pat, 0), Some(7));

  // u16 in u16: 包含模 256 等价类冲突的字符 (如 0x0041 与 0x0141 模 256 同余)
  let pat_eq: Vec<u16> = [0x0141u16, 0x0142u16].to_vec();
  let sub_eq: Vec<u16> = [0x0041u16, 0x0042u16, 0x0141u16, 0x0142u16].to_vec();
  assert_eq!(search_string_u16(&sub_eq, &pat_eq, 0), Some(2));

  OK
}

// ── 2. StringHasher & HashField 编码与 Rapidhash 等效性 ──

#[test]
fn test_string_hasher_array_and_integer_indices_boundaries() -> Void {
  let seed = HashSeedData::DEFAULT;

  // 1. "0": 唯一允许的前导零索引
  let mut is_one_byte = false;
  let hash_0 = hash_sequential_one_byte(b"0", &seed, Some(&mut is_one_byte));
  assert!(is_one_byte);
  assert!(contains_cached_array_index(hash_0));
  assert!(is_integer_index(hash_0));
  assert_eq!(decode_array_index_from_hash_field(hash_0), 0);

  // 2. "00", "01": 非法前导零必须作为普通字符串哈希处理
  let hash_00 = hash_sequential_one_byte(b"00", &seed, None);
  assert!(!is_integer_index(hash_00));
  let hash_01 = hash_sequential_one_byte(b"01", &seed, None);
  assert!(!is_integer_index(hash_01));

  // 3. MAX_ARRAY_INDEX (4294967294 = 2^32 - 2)
  let s_max = MAX_ARRAY_INDEX.to_string();
  let hash_max = hash_sequential_one_byte(s_max.as_bytes(), &seed, None);
  assert!(is_integer_index(hash_max));
  assert_eq!(
    decode_array_index_from_hash_field(hash_max),
    MAX_ARRAY_INDEX & 0xFFFFFF
  );

  // 4. MAX_ARRAY_INDEX + 1 (4294967295): 溢出 array index 但落在安全整数内
  let s_overflow = (u64::from(MAX_ARRAY_INDEX) + 1).to_string();
  let hash_overflow = hash_sequential_one_byte(s_overflow.as_bytes(), &seed, None);
  // 必须是 IntegerIndex 类型, 且不是 cached array index
  assert!(is_integer_index(hash_overflow));
  assert!(!contains_cached_array_index(hash_overflow));

  // 5. MAX_SAFE_INTEGER (9007199254740991 = 2^53 - 1, 16 位)
  let s_safe = "9007199254740991";
  let hash_safe = hash_sequential_one_byte(s_safe.as_bytes(), &seed, None);
  assert!(is_integer_index(hash_safe));

  // 6. MAX_SAFE_INTEGER + 1 (9007199254740992): 溢出安全整数，降级为普通字符串哈希
  let s_unsafe = "9007199254740992";
  let hash_unsafe = hash_sequential_one_byte(s_unsafe.as_bytes(), &seed, None);
  assert!(!is_integer_index(hash_unsafe));

  // 7. 空字符串哈希必须具有 Hash 类型且非零
  let hash_empty = hash_sequential_one_byte(b"", &seed, None);
  assert_eq!(hash_empty & 0b11, HashFieldType::Hash as u32);
  assert_ne!(hash_empty >> 2, 0);

  // 8. 超大字符串 (> 16383) 采用 TrivialHash
  let len_large = MAX_HASH_CALC_LENGTH + 100;
  let triv = get_trivial_hash(len_large);
  assert_eq!(triv & 0b11, HashFieldType::Hash as u32);
  assert_eq!((triv >> 2) & 0x3FFFFFFF, len_large);

  OK
}

#[test]
fn test_rapidhash_cross_representation_invariants() -> Void {
  let seed = HashSeedData::DEFAULT;

  // 核心不变量: 全在 Latin-1 (<=0xFF) 范围内的双字节串，其哈希值必须与对应的单字节串严格相等！
  let test_bytes: &[&[u8]] = &[
    b"a",
    b"abc",
    b"Hello, Rapidhash World!",
    b"12345678901234567890123456789012345678901234567890",
    b"\0\x01\x7f\x80\xff",
  ];

  for &bytes in test_bytes {
    let two_byte: Vec<u16> = bytes.iter().map(|&b| u16::from(b)).collect();
    assert!(is_only_8_bit(&two_byte));

    let mut is_one_byte_flag = false;
    let h1 = hash_sequential_one_byte(bytes, &seed, None);
    let h2 = hash_sequential_two_byte(&two_byte, &seed, Some(&mut is_one_byte_flag));

    assert!(is_one_byte_flag, "Latin-1 双字节串必须被探测为 is_one_byte");
    assert_eq!(
      h1, h2,
      "单双字节 Latin-1 内容哈希必须严格相等: bytes={bytes:?}"
    );
  }

  // 非 Latin-1 双字节串必须被判定为 false
  let non_latin1: Vec<u16> = "你好世界".encode_utf16().collect();
  assert!(!is_only_8_bit(&non_latin1));
  let mut is_ob = true;
  let _ = hash_sequential_two_byte(&non_latin1, &seed, Some(&mut is_ob));
  assert!(!is_ob);

  // RunningStringHasher 增量确定性
  let mut rsh = RunningStringHasher::new(0x12345678);
  for &c in b"streamed_hash" {
    rsh.add_character(u16::from(c));
  }
  let finalized = rsh.finalize();
  assert_ne!(finalized, 0);
  assert_ne!(finalized, ZERO_HASH);

  OK
}

// ── 3. SWAR 大小写快速转换边界 ──

#[test]
fn test_fast_ascii_case_convert_swar_boundaries() -> Void {
  // 1. 各长度对齐边界 (0, 1, 7, 8, 9, 15, 16, 24, 33)
  let lengths = [0, 1, 3, 7, 8, 9, 15, 16, 23, 24, 31, 32, 65];
  for &len in &lengths {
    let src = vec![b'A'; len];
    let mut dst = vec![0u8; len];
    let converted = fast_ascii_convert(CaseMapping::ToLower, &mut dst, &src);
    assert_eq!(converted, len);
    assert_eq!(dst, vec![b'a'; len]);
  }

  // 2. 邻接非字母 ASCII 符号: '@' (0x40), '[' (0x5B), '`' (0x60), '{' (0x7B) 绝不能被误翻转
  let symbols = b"@[\x60{123+-*/=";
  let mut dst_sym = vec![0u8; symbols.len()];
  let conv_sym = fast_ascii_convert(CaseMapping::ToLower, &mut dst_sym, symbols);
  assert_eq!(conv_sym, symbols.len());
  assert_eq!(&dst_sym, symbols, "非字母 ASCII 绝不能发生位异或");

  // 3. 非 ASCII 字节截断测试:
  // SWAR 按 word 粒度预检: 若第 1 个 word 内含有非 ASCII 字节，则整 word 提前退出 (返回 0)
  let mut non_ascii_src = b"ABCDEFGH12345678".to_vec();
  non_ascii_src[0] = 0x80;
  let mut dst = vec![0u8; 16];
  assert_eq!(
    fast_ascii_convert(CaseMapping::ToLower, &mut dst, &non_ascii_src),
    0
  );

  non_ascii_src = b"ABCDEFGH12345678".to_vec();
  non_ascii_src[7] = 0x80; // 仍位于第 1 个 word 内，word 预检拦截返回 0
  assert_eq!(
    fast_ascii_convert(CaseMapping::ToLower, &mut dst, &non_ascii_src),
    0
  );

  non_ascii_src = b"ABCDEFGH12345678".to_vec();
  non_ascii_src[8] = 0x80; // 位于第 2 个 word 起始: 第 1 个 word 已完成转换，在第 2 个 word 处拦截返回 8
  assert_eq!(
    fast_ascii_convert(CaseMapping::ToLower, &mut dst, &non_ascii_src),
    8
  );
  assert_eq!(&dst[..8], b"abcdefgh");

  // 标量尾路径 (不足 1 个 word 长度, 7 字节): 逐字节扫描并在首个非 ASCII 处停步
  let scalar_src = b"ABCDEF\x80";
  let mut scalar_dst = vec![0u8; 7];
  assert_eq!(
    fast_ascii_convert(CaseMapping::ToLower, &mut scalar_dst, scalar_src),
    6
  );
  assert_eq!(&scalar_dst[..6], b"abcdef");

  // 4. 前缀匹配长度 fast_ascii_case_prefix_length
  assert_eq!(
    fast_ascii_case_prefix_length(CaseMapping::ToLower, b"abcdefg"),
    7
  );
  assert_eq!(
    fast_ascii_case_prefix_length(CaseMapping::ToLower, b"abcDefg"),
    3
  );
  assert_eq!(
    fast_ascii_case_prefix_length(CaseMapping::ToUpper, b"ABC\x80DEF"),
    3
  );

  // 5. dst 长度不足时的防御保护 (不 panic, 安全裁剪到 dst.len())
  let long_src = b"HELLO WORLD THIS IS A TEST";
  let mut short_dst = vec![0u8; 5];
  let processed = fast_ascii_convert(CaseMapping::ToLower, &mut short_dst, long_src);
  assert_eq!(processed, 5);
  assert_eq!(&short_dst, b"hello");

  OK
}

// ── 4. DFA UTF-8 校验与解码状态机 ──

#[test]
fn test_utf8_dfa_strict_vs_generalized_states() -> Void {
  // 1. 孤立代理码点 (U+D800: 0xED 0xA0 0x80, U+DFFF: 0xED 0xBF 0xBF)
  let lone_surrogate_d800 = [0xED, 0xA0, 0x80];
  let lone_surrogate_dfff = [0xED, 0xBF, 0xBF];

  // 严格 UTF-8 必须校验失败
  assert!(!validate_encoding(&lone_surrogate_d800));
  assert!(!validate_encoding(&lone_surrogate_dfff));

  // WTF-8 允许孤立代理
  assert!(wtf8::validate_encoding(&lone_surrogate_d800));
  assert!(wtf8::validate_encoding(&lone_surrogate_dfff));

  // WTF-8 拒绝连续代理对 (成对代理必须用 4 字节表示)
  let mut surrogate_pair = Vec::new();
  surrogate_pair.extend_from_slice(&lone_surrogate_d800);
  surrogate_pair.extend_from_slice(&lone_surrogate_dfff);
  assert!(!wtf8::validate_encoding(&surrogate_pair));

  // 2. 超长编码 (Overlong sequences) 严格和泛化 DFA 都必须拒绝
  let overlong_2 = [0xC0, 0xAF]; // 应当用 1 字节表示的 '/'
  let overlong_3 = [0xE0, 0x80, 0xAF];
  let overlong_4 = [0xF0, 0x80, 0x80, 0xAF];
  assert!(!validate_encoding(&overlong_2));
  assert!(!validate_encoding(&overlong_3));
  assert!(!validate_encoding(&overlong_4));
  assert!(!wtf8::validate_encoding(&overlong_2));
  assert!(!wtf8::validate_encoding(&overlong_3));
  assert!(!wtf8::validate_encoding(&overlong_4));

  // 3. 超出 0x10FFFF 范围
  let out_of_range = [0xF4, 0x90, 0x80, 0x80]; // U+110000
  assert!(!validate_encoding(&out_of_range));
  assert!(!wtf8::validate_encoding(&out_of_range));

  // 4. 增量解码遇到非法字节序列的回退重试机制
  let malformed_seq = [0xE0, 0x41]; // 0xE0 后缺少合法 continuation，紧跟 'A'
  let mut state = strict::State::Accept;
  let mut buffer = 0u32;
  let mut cursor = 0usize;

  // 喂 0xE0 -> 处于中间态
  let res1 = value_of_incremental(&malformed_seq, &mut cursor, &mut state, &mut buffer);
  assert_eq!(res1, INCOMPLETE);
  assert_eq!(cursor, 1);

  // 喂 0x41 -> 识别为 Reject，重置 state 为 Accept 并回退 1 步 (cursor 仍为 1)，返回 BAD_CHAR
  let res2 = value_of_incremental(&malformed_seq, &mut cursor, &mut state, &mut buffer);
  assert_eq!(res2, BAD_CHAR);
  assert_eq!(cursor, 1, "回退让 0x41 重新按 ASCII 解码");

  // 再次消费 0x41 -> 正常输出 'A'
  let res3 = value_of_incremental(&malformed_seq, &mut cursor, &mut state, &mut buffer);
  assert_eq!(res3, 0x41);
  assert_eq!(cursor, 2);

  OK
}

#[test]
fn test_unicode_decoder_three_way_adversarial() -> Void {
  // 1. Utf8Decoder (有损替换): 坏序列以 U+FFFD 兜底且不报错
  let broken = [0x61, 0xFF, 0x62];
  let dec = Utf8Decoder::new(&broken);
  assert!(!dec.is_invalid());
  assert_eq!(dec.utf16_length(), 3);
  let mut out16 = vec![0u16; dec.utf16_length() as usize];
  dec.decode_to_two_byte(&mut out16, &broken);
  assert_eq!(out16, vec![0x61, 0xFFFD, 0x62]);

  // 2. StrictUtf8Decoder (严格报错): 坏序列立即置为 Invalid
  let strict_dec = StrictUtf8Decoder::new(&broken);
  assert!(strict_dec.is_invalid());
  assert_eq!(strict_dec.encoding(), Encoding::Invalid);

  // 3. Wtf8Decoder: 允许孤立代理
  let lone_d800 = [0xED, 0xA0, 0x80];
  let wtf_dec = Wtf8Decoder::new(&lone_d800);
  assert!(!wtf_dec.is_invalid());
  let mut wtf_out = vec![0u16; wtf_dec.utf16_length() as usize];
  wtf_dec.decode_to_two_byte(&mut wtf_out, &lone_d800);
  assert_eq!(wtf_out, vec![0xD800]);

  // 4. Utf16 代理对检测与替换
  let unp = [0xD800u16, 0x0041u16];
  assert!(has_unpaired_surrogate(&unp));
  let mut replaced = vec![0u16; 2];
  replace_unpaired_surrogates(&unp, &mut replaced);
  assert_eq!(replaced, vec![0xFFFDu16, 0x0041u16]);

  OK
}

// ── 5. 诊断流 StringStream 对抗测试 ──

#[test]
fn test_string_stream_allocators_and_formatting_stress() -> Void {
  // 1. 定长超小缓冲 (小于 4 字节无法容纳 "...\n")
  let mut tiny_buf = [0u8; 3];
  let mut fixed_alloc = FixedStringAllocator::new(&mut tiny_buf);
  let mut stream = StringStream::new(&mut fixed_alloc);
  for c in b"overflowing text" {
    stream.put(*c);
  }
  // 截断时安全填充 '.' 且不越界
  assert_eq!(stream.length(), 2);
  assert_eq!(stream.to_string_lossy(), "..");

  // 2. 堆扩容与全指令格式化
  let mut heap_alloc = HeapStringAllocator::default();
  let mut stream2 = StringStream::new(&mut heap_alloc);

  stream2.add(
    "str=%s int=%d hex=%x HEX=%X u=%u c=%c float=%.2f sci=%e char=%k ptr=%p",
    &[
      FmtElm::CStr("test"),
      FmtElm::Int(-42),
      FmtElm::Int(255),
      FmtElm::Int(255),
      FmtElm::Int(100),
      FmtElm::Int(65),
      FmtElm::Double(PI),
      FmtElm::Double(1000.0),
      FmtElm::Char(0x07), // 控制字符按 \x07
      FmtElm::Pointer(0xdeadbeef),
    ],
  );

  let formatted = stream2.to_string_lossy();
  assert!(formatted.contains("str=test"));
  assert!(formatted.contains("int=-42"));
  assert!(formatted.contains("hex=ff"));
  assert!(formatted.contains("HEX=FF"));
  assert!(formatted.contains("u=100"));
  assert!(formatted.contains("c=A"));
  assert!(formatted.contains("float=3.14"));
  // %e 为 vsnprintf 语义: 默认精度 6 + 指数符号两位 (原版 base::SNPrintF)
  assert!(formatted.contains("sci=1.000000e+03"));
  assert!(formatted.contains("char=\\x07"));
  assert!(formatted.contains("ptr=0xdeadbeef"));

  // 3. 重置复用
  stream2.reset();
  assert_eq!(stream2.length(), 0);
  assert_eq!(stream2.to_string_lossy(), "");
  stream2.add_str("reused");
  assert_eq!(stream2.to_string_lossy(), "reused");

  OK
}

// ── 6. URI 编解码对抗测试 ──

#[test]
fn test_uri_encode_decode_adversarial_rfc_invariants() -> Void {
  // 1. encodeURI 与 encodeURIComponent 保留字差异
  // # : ; / ? $ & + , @ = 在 encodeURI 保留，在 encodeURIComponent 转义
  let reserved_sub: Vec<u16> = "#:;/?$&+,@=".encode_utf16().collect();
  let enc_uri = uri_encode(&reserved_sub, true, usize::MAX)?;
  assert_eq!(String::from_utf8(enc_uri)?, "#:;/?$&+,@=");

  let enc_comp = uri_encode(&reserved_sub, false, usize::MAX)?;
  assert_eq!(
    String::from_utf8(enc_comp)?,
    "%23%3A%3B%2F%3F%24%26%2B%2C%40%3D"
  );

  // 2. U+FFFD 的合法编码 %EF%BF%BD 在 decode 中必须放行
  let fffd_encoded: Vec<u16> = "%EF%BF%BD".encode_utf16().collect();
  let decoded_fffd = decode(&fffd_encoded, false)?;
  assert_eq!(decoded_fffd.into_u16_vec(), vec![0xFFFDu16]);

  // 3. 非法十六进制与截断 continuation 报错 Malformed
  let bad_cases: &[&str] = &[
    "%",
    "%1",
    "%ZZ",
    "%E0%80",    // 3 字节 UTF-8 缺少第 3 字节
    "%F0%90%80", // 4 字节 UTF-8 缺少第 4 字节
    "%C0%AF",    // 超长
  ];
  for &bad in bad_cases {
    let bad_u16: Vec<u16> = bad.encode_utf16().collect();
    assert_eq!(
      decode(&bad_u16, false),
      Err(UriError::Malformed),
      "bad={bad}"
    );
  }

  // 4. escape 与 unescape 对宽字符与特定 ASCII 符号
  let special = "Hello World! @*_+-./";
  let special_u16: Vec<u16> = special.encode_utf16().collect();
  let escaped = escape(&special_u16, usize::MAX)?;
  // 空格转 %20, 其余非转义保留
  assert_eq!(
    String::from_utf8(escaped.clone())?,
    "Hello%20World%21%20@*_+-./"
  );
  let unescaped = unescape(&special_u16);
  assert_eq!(unescaped, special_u16);

  // %u 宽字符 unescape
  let unicode_escape: Vec<u16> = "%u4F60%u597D".encode_utf16().collect();
  let unesc_res = unescape(&unicode_escape);
  assert_eq!(unesc_res, "你好".encode_utf16().collect::<Vec<_>>());

  OK
}
