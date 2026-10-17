//! v8-base/src/vlq_base64.rs 集成测试 — 1:1 对应 vlq-base64-unittest.cc
//!
//! 原版 4 用例:
//! - `VLQBASE64.charToDigit` — 256 全字符查表
//! - `VLQBASE64.DecodeOneSegment` — 30+ 输入 (含边界: 空串/非法符号/溢出/完整/未完成)
//! - `VLQBASE64.DecodeTwoSegment` — 25 组
//! - `VLQBASE64.DecodeFourSegment` — 7 组

use std::str::from_utf8;

use aok::{OK, Void};
use v8_base::vlq_base64::{char_to_digit_decode_for_testing, vlq_base64_decode};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版期望值哨兵: 非法输入返回 INT32_MIN
const BAD: i32 = i32::MIN;

/// 原版 `TestVLQBase64Decode(str, {{pos, result}, ...})` 的 Rust 等价.
fn test_vlq_base64_decode(str: &[u8], expected: &[(usize, i32)]) {
  let mut pos: usize = 0;
  for &(want_pos, want_result) in expected {
    let result = vlq_base64_decode(str, &mut pos);
    assert_eq!(
      want_result,
      result,
      "input={:?} pos_after={}",
      from_utf8(str).unwrap_or("<bin>"),
      pos
    );
    assert_eq!(want_pos, pos);
  }
}

#[test]
fn vlqbase64_char_to_digit() -> Void {
  let syms: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
  for i in 0..256u32 {
    let ch = i as u8;
    let pos = syms.iter().position(|&c| c == ch);
    let expected: i8 = match (i, pos) {
      (0, _) => -1,
      (_, Some(p)) => p as i8,
      (_, None) => -1,
    };
    assert_eq!(
      expected,
      char_to_digit_decode_for_testing(ch),
      "codepoint {i}"
    );
  }
  OK
}

#[test]
fn vlqbase64_decode_one_segment() -> Void {
  test_vlq_base64_decode(b"", &[(0, BAD)]);
  // Unsupported symbol.
  test_vlq_base64_decode(b"*", &[(0, BAD)]);
  test_vlq_base64_decode(b"&", &[(0, BAD)]);
  test_vlq_base64_decode(b"kt:", &[(2, BAD)]);
  test_vlq_base64_decode(b"k^C", &[(1, BAD)]);
  // Imcomplete string.
  test_vlq_base64_decode(b"kth4yp", &[(6, BAD)]);
  // Interpretable strings.
  test_vlq_base64_decode(b"A", &[(1, 0)]);
  test_vlq_base64_decode(b"C", &[(1, 1)]);
  test_vlq_base64_decode(b"Y", &[(1, 12)]);
  test_vlq_base64_decode(b"2H", &[(2, 123)]);
  test_vlq_base64_decode(b"ktC", &[(3, 1234)]);
  test_vlq_base64_decode(b"yjY", &[(3, 12345)]);
  test_vlq_base64_decode(b"gkxH", &[(4, 123456)]);
  test_vlq_base64_decode(b"uorrC", &[(5, 1234567)]);
  test_vlq_base64_decode(b"80wxX", &[(5, 12345678)]);
  test_vlq_base64_decode(b"qxmvrH", &[(6, 123456789)]);
  test_vlq_base64_decode(b"kth4ypC", &[(7, 1234567890)]);
  test_vlq_base64_decode(b"+/////D", &[(7, i32::MAX)]);
  test_vlq_base64_decode(b"D", &[(1, -1)]);
  test_vlq_base64_decode(b"Z", &[(1, -12)]);
  test_vlq_base64_decode(b"3H", &[(2, -123)]);
  test_vlq_base64_decode(b"ltC", &[(3, -1234)]);
  test_vlq_base64_decode(b"zjY", &[(3, -12345)]);
  test_vlq_base64_decode(b"hkxH", &[(4, -123456)]);
  test_vlq_base64_decode(b"vorrC", &[(5, -1234567)]);
  test_vlq_base64_decode(b"90wxX", &[(5, -12345678)]);
  test_vlq_base64_decode(b"rxmvrH", &[(6, -123456789)]);
  test_vlq_base64_decode(b"lth4ypC", &[(7, -1234567890)]);
  test_vlq_base64_decode(b"//////D", &[(7, -i32::MAX)]);
  // Overflowed values.
  test_vlq_base64_decode(b"qjuw7/2A", &[(6, BAD)]);
  test_vlq_base64_decode(b"ohtkz+lH", &[(6, BAD)]);
  test_vlq_base64_decode(b"ggggggE", &[(6, BAD)]);
  test_vlq_base64_decode(b"rjuw7/2A", &[(6, BAD)]);
  test_vlq_base64_decode(b"phtkz+lH", &[(6, BAD)]);
  test_vlq_base64_decode(b"hgggggE", &[(6, BAD)]);
  OK
}

#[test]
fn vlqbase64_decode_two_segment() -> Void {
  test_vlq_base64_decode(b"AA", &[(1, 0), (2, 0)]);
  test_vlq_base64_decode(b"KA", &[(1, 5), (2, 0)]);
  test_vlq_base64_decode(b"AQ", &[(1, 0), (2, 8)]);
  test_vlq_base64_decode(b"MG", &[(1, 6), (2, 3)]);
  test_vlq_base64_decode(b"a4E", &[(1, 13), (3, 76)]);
  test_vlq_base64_decode(b"4GyO", &[(2, 108), (4, 233)]);
  test_vlq_base64_decode(b"ggEqnD", &[(3, 2048), (6, 1653)]);
  test_vlq_base64_decode(b"g2/D0ilF", &[(4, 65376), (8, 84522)]);
  test_vlq_base64_decode(b"ss6gBy0m3B", &[(5, 537798), (10, 904521)]);
  test_vlq_base64_decode(b"LA", &[(1, -5), (2, 0)]);
  test_vlq_base64_decode(b"AR", &[(1, 0), (2, -8)]);
  test_vlq_base64_decode(b"NH", &[(1, -6), (2, -3)]);
  test_vlq_base64_decode(b"b5E", &[(1, -13), (3, -76)]);
  test_vlq_base64_decode(b"5GzO", &[(2, -108), (4, -233)]);
  test_vlq_base64_decode(b"hgErnD", &[(3, -2048), (6, -1653)]);
  test_vlq_base64_decode(b"h2/D1ilF", &[(4, -65376), (8, -84522)]);
  test_vlq_base64_decode(b"ts6gBz0m3B", &[(5, -537798), (10, -904521)]);
  test_vlq_base64_decode(b"4GzO", &[(2, 108), (4, -233)]);
  test_vlq_base64_decode(b"ggErnD", &[(3, 2048), (6, -1653)]);
  test_vlq_base64_decode(b"g2/D1ilF", &[(4, 65376), (8, -84522)]);
  test_vlq_base64_decode(b"ss6gBz0m3B", &[(5, 537798), (10, -904521)]);
  test_vlq_base64_decode(b"5GyO", &[(2, -108), (4, 233)]);
  test_vlq_base64_decode(b"hgEqnD", &[(3, -2048), (6, 1653)]);
  test_vlq_base64_decode(b"h2/D0ilF", &[(4, -65376), (8, 84522)]);
  test_vlq_base64_decode(b"ts6gBy0m3B", &[(5, -537798), (10, 904521)]);
  OK
}

#[test]
fn vlqbase64_decode_four_segment() -> Void {
  test_vlq_base64_decode(b"AAAA", &[(1, 0), (2, 0), (3, 0), (4, 0)]);
  test_vlq_base64_decode(b"QADA", &[(1, 8), (2, 0), (3, -1), (4, 0)]);
  test_vlq_base64_decode(b"ECQY", &[(1, 2), (2, 1), (3, 8), (4, 12)]);
  test_vlq_base64_decode(
    b"goGguCioPk9I",
    &[(3, 3200), (6, 1248), (9, 7809), (12, 4562)],
  );
  test_vlq_base64_decode(b"6/BACA", &[(3, 1021), (4, 0), (5, 1), (6, 0)]);
  test_vlq_base64_decode(b"urCAQA", &[(3, 1207), (4, 0), (5, 8), (6, 0)]);
  test_vlq_base64_decode(b"sDACA", &[(2, 54), (3, 0), (4, 1), (5, 0)]);
  OK
}

/// 对抗补充: 编码侧 (source-map 风格 5 位组 + 续接位) ↔ 解码往返。
/// 原版仅测解码; 往返按 64 字符表 + zigzag 语义对拍, 覆盖 5 位边界全域
/// 与末字节高位拒绝 (digit>>2 != 0 → BAD)、截断流 (BAD)。
#[test]
fn vlqbase64_roundtrip_and_rejects() -> Void {
  const CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

  // 编码: 与 vlq-base64 解码格式镜像 — 每字符 5 数据位 + 1 续接位 (bit 32),
  // 末字符无续接位; 末组保留 2 位符号余量 (digit>>2 须为 0)
  fn encode_u32(mut value: u32, out: &mut Vec<u8>) {
    loop {
      let digit = (value & 0x1F) as usize;
      value >>= 5;
      if value == 0 {
        // 末字符: 数据须满足高位组约束 (值 < 2^31 时 (digit>>2)==0 恒成立:
        // 末组只剩 2 有效位由 32 位总宽保证)
        out.push(CHARS[digit]);
        break;
      }
      out.push(CHARS[digit | 0x20]);
    }
  }

  // 5 位边界全域: 0..=63 全量 + 32 位全域的 2^k 边界 ±1 + zigzag 正负
  let mut corpus: Vec<u32> = (0..64u32).collect();
  for k in 0..=32u32 {
    for v in [
      1u64 << k,
      (1u64 << k).wrapping_sub(1),
      (1u64 << k).wrapping_add(1),
    ] {
      corpus.push(v as u32);
    }
  }
  // 合法解码值域 (res 为 zigzag 无符号面): 直接以 res 往返
  for res in corpus {
    let mut buf = Vec::new();
    encode_u32(res, &mut buf);
    let mut pos = 0;
    let expected = if res & 1 == 1 {
      -((res >> 1) as i32)
    } else {
      (res >> 1) as i32
    };
    assert_eq!(vlq_base64_decode(&buf, &mut pos), expected, "res={res}");
    assert_eq!(pos, buf.len(), "res={res}");
  }

  // 多值流: 编码串联 → 连续解码, pos 游标首尾相接
  let stream: [u32; 6] = [0, 1, 31, 32, 0x3FFF_FFFF, 0x7FFF_FFFF];
  let mut buf = Vec::new();
  for &v in &stream {
    encode_u32(v, &mut buf);
  }
  let mut pos = 0;
  for &v in &stream {
    let expected = if v & 1 == 1 {
      -((v >> 1) as i32)
    } else {
      (v >> 1) as i32
    };
    assert_eq!(
      vlq_base64_decode(&buf, &mut pos),
      expected,
      "stream v={v:#x}"
    );
  }
  assert_eq!(pos, buf.len());

  // 截断流: 有续接位但流耗尽 → BAD (游标停在流末)
  let truncated = vec![CHARS[0x20]]; // 续接位为 1, 无后续
  let mut pos = 0;
  assert_eq!(vlq_base64_decode(&truncated, &mut pos), BAD);
  assert_eq!(pos, 1);

  // 末字节高位数据 (digit>>2 != 0): '/'+续接位形态 → BAD
  // 构造: 6 组全续接 + 末字符 '/' (0x3F, 数据位超 2 位)
  let mut over = vec![CHARS[0x20]; 6];
  over.push(b'/');
  let mut pos = 0;
  assert_eq!(vlq_base64_decode(&over, &mut pos), BAD);
  OK
}
