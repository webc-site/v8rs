// round2 对抗审查差分 (独立 Reviewer 会话生成)
//! 覆盖: (a) from_string 500-2000 digit 十进制 × 全 radix 往返 + Python int golden 对拍
//!       (b) asIntN/asUintN 跨 digit 边界 (63/64/65/127/128/129/.../257) 符号扩展矩阵
//!       (c) 除法三算法阈值 ±1 (schoolbook↔Burnikel @57, Burnikel↔Barrett @13000)
//!       (d) 乘法阈值 ±1 (karatsuba↔toom @210, toom↔fft @720) + to_string fast 阈值 @23
//! golden 文件 tests/golden_round2.txt 由 python3 (seed 314159265359) 生成:
//!   asIntN 公式: x mod 2^n, 若 >= 2^(n-1) 再减 2^n; asUintN 公式: x mod 2^n
//! 除法一致性判据: q*b + r == a ∧ 0 <= r < b (商余唯一性, 数学完备, 不依赖被测函数互证)
//!
use v8_bigint::bigint::{
  as_int_n, as_uint_n_neg, as_uint_n_pos, as_uint_n_pos_result_length, div_ceil, multiply_small,
};
mod common;

use aok::{OK, Void};
use common::{Rng, format_hex, generate_random, generate_random_string, processor};
use log::info;
use v8_bigint::{
  bigint::{
    DefaultPlatform, Digit, K_BARRETT_THRESHOLD, K_BURNIKEL_THRESHOLD, K_FFT_THRESHOLD,
    K_FROM_STRING_LARGE_THRESHOLD, K_KARATSUBA_THRESHOLD, K_TO_STRING_FAST_THRESHOLD,
    K_TOOM_THRESHOLD, Status, add, compare, divide_result_length, multiply_result_length, norm,
  },
  bigint_internal::Processor,
  from_string::{FromStringAccumulator, ParseResult},
  mul_helpers::multiply_schoolbook,
  to_string::to_string_result_length,
};

/// golden 数据段 (python3 生成脚本入提交记录)
const GOLDEN: &str = include_str!("golden_round2.txt");

// ---------------------------------------------------------------------------
// 辅助: hex 串 ↔ digits (低位在前)
// ---------------------------------------------------------------------------

fn hex_to_digits(s: &str) -> Vec<Digit> {
  let mut out = vec![0u64; s.len().div_ceil(16)];
  for (i, c) in s.as_bytes().iter().rev().enumerate() {
    let v = match c {
      b'0'..=b'9' => (c - b'0') as u64,
      b'a'..=b'f' => (c - b'a' + 10) as u64,
      _ => panic!("bad hex char"),
    };
    out[i / 16] |= v << (4 * (i % 16));
  }
  out
}

fn digits_to_hex(d: &[Digit]) -> String {
  let d = norm(d);
  if d.is_empty() {
    return "0".to_string();
  }
  let mut s = format!("{:x}", d[d.len() - 1]);
  for &x in d[..d.len() - 1].iter().rev() {
    s += &format!("{:016x}", x);
  }
  s
}

/// golden 提取: <tag> 后的整行
fn golden_line(tag: &str) -> &'static str {
  let prefix = format!("{tag} ");
  for line in GOLDEN.lines() {
    if let Some(rest) = line.strip_prefix(&prefix) {
      return rest.trim();
    }
  }
  panic!("golden tag missing: {tag}");
}

/// 十进制串 → digits (走被测 parse; 值由往返 + golden 双向锁定)
fn dec_to_digits(proc: &mut Processor<DefaultPlatform>, s: &[u8]) -> Vec<Digit> {
  let mut acc = FromStringAccumulator::new(u32::MAX);
  let consumed = acc.parse(s, 10);
  assert_eq!(consumed, s.len());
  assert_eq!(acc.result(), ParseResult::Ok);
  let mut z = vec![0u64; acc.result_length() as usize];
  let st = proc.parse_string(&mut z, &mut acc);
  assert_eq!(st, Status::Ok);
  z
}

fn to_radix_string(proc: &mut Processor<DefaultPlatform>, d: &[Digit], radix: u32) -> Vec<u8> {
  let n = to_string_result_length(d, radix, false) as usize;
  let mut out = vec![0u8; n];
  let (_, w) = proc.write_string(&mut out, d, radix, false);
  out[..w as usize].to_vec()
}

// ---------------------------------------------------------------------------
// (a) from_string: golden 大数十进制串 parse→to_string 对拍
// ---------------------------------------------------------------------------

#[test]
fn round2_from_string_golden_roundtrip() -> Void {
  let mut proc = processor();
  for (dec_tag, radix, radix_tag) in [
    ("G1_DEC", 36u32, "G1_R36"),
    ("G2_DEC", 16, "G2_R16"),
    ("G3_DEC", 2, "G3_R2"),
  ] {
    let dec = golden_line(dec_tag);
    let expected = golden_line(radix_tag);
    let d = dec_to_digits(&mut proc, dec.as_bytes());
    // 往返: parse(10) → to_string(10) 还原原串
    assert_eq!(
      to_radix_string(&mut proc, &d, 10),
      dec.as_bytes(),
      "radix10 roundtrip {dec_tag}"
    );
    // golden: to_string(radix) 与 Python int 对拍
    let got = to_radix_string(&mut proc, &d, radix);
    assert_eq!(
      got,
      expected.as_bytes(),
      "radix{radix} vs python golden ({dec_tag})"
    );
    // 反向: parse(golden radix 串) → to_string(10) 还原原串
    let mut acc = FromStringAccumulator::new(u32::MAX);
    let consumed = acc.parse(expected.as_bytes(), radix as Digit);
    assert_eq!(consumed, expected.len());
    assert_eq!(acc.result(), ParseResult::Ok);
    let mut z2 = vec![0u64; acc.result_length() as usize];
    let st = proc.parse_string(&mut z2, &mut acc);
    assert_eq!(st, Status::Ok);
    assert_eq!(
      to_radix_string(&mut proc, &z2, 10),
      dec.as_bytes(),
      "reverse roundtrip {dec_tag}"
    );
  }
  info!("> golden_roundtrip: 3 values x 2 directions OK");
  OK
}

#[test]
fn round2_from_string_random_all_radix_roundtrip() -> Void {
  let mut rng = Rng::new(314159265359);
  let mut proc = processor();
  // (长度, radix 集合): 大串抽常用进制控制耗时
  let plan: [(usize, Vec<u32>); 5] = [
    (500, (2..=36).collect()),
    (777, (2..=36).collect()),
    (1000, vec![2, 8, 10, 16, 32, 36]),
    (1500, (2..=36).collect()),
    (2000, vec![10, 16, 36]),
  ];
  let mut count = 0u64;
  for (len, radixes) in plan {
    let s = generate_random_string(&mut rng, len, 10);
    let d = dec_to_digits(&mut proc, &s);
    // radix 10 往返 + classic 双路径对拍
    assert_eq!(to_radix_string(&mut proc, &d, 10), s, "roundtrip len={len}");
    let n10 = to_string_result_length(&d, 10, false) as usize;
    let mut out_cl = vec![0u8; n10];
    let w_cl = proc.write_impl(&mut out_cl, &d, 10, false, false);
    assert_eq!(&out_cl[..w_cl as usize], s.as_slice(), "classic len={len}");
    // 全 radix: to_string(r) → parse(r) → to_string(10) 还原
    for radix in radixes {
      let repr = to_radix_string(&mut proc, &d, radix);
      let mut acc = FromStringAccumulator::new(u32::MAX);
      let consumed = acc.parse(&repr, radix as Digit);
      assert_eq!(consumed, repr.len(), "parse radix={radix} len={len}");
      assert_eq!(acc.result(), ParseResult::Ok);
      let mut z = vec![0u64; acc.result_length() as usize];
      let st = proc.parse_string(&mut z, &mut acc);
      assert_eq!(st, Status::Ok);
      assert_eq!(
        to_radix_string(&mut proc, &z, 10),
        s,
        "roundtrip radix={radix} len={len}"
      );
      count += 1;
    }
  }
  info!("> random_all_radix_roundtrip: {count} radix roundtrips");
  OK
}

/// classic ↔ large 解析边界 (ResultLength 25 ± 1: golden G4=464 / G5=481 digit)
#[test]
fn round2_from_string_classic_large_boundary() -> Void {
  let mut proc = processor();
  for (tag, want_large) in [("G4_DEC", false), ("G5_DEC", true)] {
    let s = golden_line(tag);
    let mut acc = FromStringAccumulator::new(u32::MAX);
    let consumed = acc.parse(s.as_bytes(), 10);
    assert_eq!(consumed, s.len());
    let n_parts = acc.result_length();
    assert_eq!(
      n_parts >= K_FROM_STRING_LARGE_THRESHOLD,
      want_large,
      "{tag}: {n_parts} parts on wrong side of threshold"
    );
    let mut z = vec![0u64; n_parts as usize];
    let st = proc.parse_string(&mut z, &mut acc);
    assert_eq!(st, Status::Ok);
    assert_eq!(
      to_radix_string(&mut proc, &z, 10),
      s.as_bytes(),
      "{tag} roundtrip"
    );
  }
  OK
}

// ---------------------------------------------------------------------------
// (b) asIntN / asUintN 边界矩阵 (golden 驱动)
// ---------------------------------------------------------------------------

/// abs digits pad 到至少 len 位 (高位零) — as_int_n 契约: x.len() >= ceil(n/64)
fn pad_to(d: &[Digit], len: usize) -> Vec<Digit> {
  let mut v = d.to_vec();
  v.resize(len.max(v.len()), 0);
  v
}

#[test]
fn round2_as_int_n_boundary_matrix() -> Void {
  let mut count = 0u64;
  for line in GOLDEN.lines() {
    let sp = line.find(' ').unwrap();
    if !line.starts_with("N_") {
      continue;
    }
    let tag = &line[2..sp];
    let f: Vec<&str> = line[sp + 1..].split_whitespace().collect();
    let n: u32 = f[0].parse().unwrap();
    let exp_sign = f[1] == "1";
    let exp_abs = f[2];
    let i_f: Vec<&str> = golden_line(&format!("I_{tag}"))
      .split_whitespace()
      .collect();
    let (in_sign, in_abs) = (i_f[0] == "1", i_f[1]);
    let x_raw = hex_to_digits(in_abs);
    let needed = div_ceil(n, 64) as usize;
    let x = pad_to(&x_raw, needed);
    let mut z = vec![0u64; needed];
    let res_negative = as_int_n(&mut z, &x, in_sign, n);
    // V8 上层 (objects/bigint) 对绝对值为 0 的结果强制正号; AsIntN 层返回原始符号,
    // 故按上层可观察行为建模: 结果为 0 时符号必为正
    let final_negative = if norm(&z).is_empty() {
      false
    } else {
      res_negative
    };
    assert_eq!(final_negative, exp_sign, "asIntN({n}) sign, tag {tag}");
    assert_eq!(digits_to_hex(&z), exp_abs, "asIntN({n}) value, tag {tag}");
    count += 1;
  }
  info!("> as_int_n matrix: {count} cases");
  OK
}

#[test]
fn round2_as_uint_n_boundary_matrix() -> Void {
  let mut count = 0u64;
  for line in GOLDEN.lines() {
    let sp = line.find(' ').unwrap();
    if !line.starts_with("U_") {
      continue;
    }
    let tag = &line[2..sp];
    let f: Vec<&str> = line[sp + 1..].split_whitespace().collect();
    let n: u32 = f[0].parse().unwrap();
    let exp_hex = f[1];
    let i_f: Vec<&str> = golden_line(&format!("I_{tag}"))
      .split_whitespace()
      .collect();
    let (in_sign, in_abs) = (i_f[0] == "1", i_f[1]);
    let x_raw = hex_to_digits(in_abs);
    let needed = div_ceil(n, 64) as usize;
    let x = pad_to(&x_raw, needed);
    let mut z = vec![0u64; needed];
    if in_sign {
      as_uint_n_neg(&mut z, &x, n);
    } else if as_uint_n_pos_result_length(&x, n) > 0 {
      as_uint_n_pos(&mut z, &x, n);
    } else {
      // 原版语义: 截断是空操作, 结果即输入
      z.copy_from_slice(&x[..needed]);
    }
    assert_eq!(digits_to_hex(&z), exp_hex, "asUintN({n}) tag {tag}");
    count += 1;
  }
  info!("> as_uint_n matrix: {count} cases");
  OK
}

// ---------------------------------------------------------------------------
// (c) 除法三算法阈值 ±1
// ---------------------------------------------------------------------------

/// 校验 q*b + r == a 且 r < b (乘法用独立 schoolbook 直调, 不经算法调度互证)
fn assert_division_consistent(proc: &mut Processor<DefaultPlatform>, a: &[Digit], b: &[Digit]) {
  let q_len = divide_result_length(a, b) as usize;
  let mut q = vec![0u64; q_len];
  let st_q = proc.divide_large(&mut q, a, b);
  assert_eq!(st_q, Status::Ok);
  let mut r = vec![0u64; b.len()];
  let st_r = proc.modulo_large(&mut r, a, b);
  assert_eq!(st_r, Status::Ok);
  let qn = norm(&q);
  let rn = norm(&r);
  assert!(
    compare(rn, norm(b)) < 0,
    "remainder >= divisor
a: {}
b: {}",
    format_hex(a),
    format_hex(b)
  );
  let prod_len = multiply_result_length(&q, b) as usize;
  let mut prod = vec![0u64; prod_len];
  // multiply_schoolbook 契约: 前参 >= 后参
  if qn.len() >= b.len() {
    multiply_schoolbook(&mut prod, qn, b);
  } else {
    multiply_schoolbook(&mut prod, b, qn);
  }
  let mut sum = vec![0u64; prod_len + 1];
  add(&mut sum, &prod, rn);
  assert_eq!(
    norm(&sum),
    norm(a),
    "q*b + r != a
a: {}
b: {}",
    format_hex(a),
    format_hex(b)
  );
}

#[test]
fn round2_division_burnikel_threshold_boundary() -> Void {
  // schoolbook ↔ Burnikel: K_BURNIKEL_THRESHOLD = 57 (除数宽度 ±1)
  let mut rng = Rng::new(314159265359);
  let mut proc = processor();
  let mut count = 0u64;
  for b_len in [
    (K_BURNIKEL_THRESHOLD - 1) as usize,
    K_BURNIKEL_THRESHOLD as usize,
    (K_BURNIKEL_THRESHOLD + 1) as usize,
  ] {
    for a_extra in [0usize, 1, b_len - 1, b_len] {
      let a_len = b_len + a_extra;
      let a = generate_random(&mut rng, a_len);
      let b = generate_random(&mut rng, b_len);
      assert_division_consistent(&mut proc, &a, &b);
      count += 1;
    }
  }
  info!("> burnikel threshold boundary: {count} divisions");
  OK
}

#[test]
fn round2_division_barrett_threshold_boundary() -> Void {
  // Burnikel ↔ Barrett: K_BARRETT_THRESHOLD = 13000 (除数宽度 ±1);
  // a.len() == b.len() 与 a.len() > 2*b.len() 的域切换一并覆盖
  let mut rng = Rng::new(314159265359);
  let mut proc = processor();
  let mut count = 0u64;
  for b_len in [
    (K_BARRETT_THRESHOLD - 1) as usize,
    K_BARRETT_THRESHOLD as usize,
    (K_BARRETT_THRESHOLD + 1) as usize,
  ] {
    for a_len in [b_len, 2 * b_len - 1, 2 * b_len + 1] {
      let a = generate_random(&mut rng, a_len);
      let b = generate_random(&mut rng, b_len);
      assert_division_consistent(&mut proc, &a, &b);
      count += 1;
    }
  }
  info!("> barrett threshold boundary: {count} divisions");
  OK
}

// ---------------------------------------------------------------------------
// (d) 乘法阈值 ±1 + to_string fast 阈值
// ---------------------------------------------------------------------------

#[test]
fn round2_multiply_threshold_boundary() -> Void {
  // karatsuba ↔ toom @210, toom ↔ fft @720: 大算法路径 vs schoolbook 对拍
  let mut rng = Rng::new(314159265359);
  let mut proc = processor();
  let mut count = 0u64;
  for y_len in [
    K_KARATSUBA_THRESHOLD - 1,
    K_KARATSUBA_THRESHOLD,
    K_TOOM_THRESHOLD - 1,
    K_TOOM_THRESHOLD,
    K_TOOM_THRESHOLD + 1,
    K_FFT_THRESHOLD - 1,
    K_FFT_THRESHOLD,
    K_FFT_THRESHOLD + 1,
  ] {
    // multiply_large 契约: 较短一方 >= K_KARATSUBA_THRESHOLD, 故取等长
    let a = generate_random(&mut rng, y_len as usize);
    let b = generate_random(&mut rng, y_len as usize);
    let rl = multiply_result_length(&a, &b) as usize;
    let mut fast = vec![0u64; rl];
    // 调度分界: 33→multiply_small 处理, 34→multiply_large(karatsuba), 210→toom, 720→fft
    if y_len < K_KARATSUBA_THRESHOLD {
      multiply_small(&mut fast, &a, &b);
    } else {
      proc.multiply_large(&mut fast, &a, &b);
    }
    let mut slow = vec![0u64; rl];
    multiply_schoolbook(&mut slow, &a, &b);
    assert_eq!(
      norm(&fast),
      norm(&slow),
      "multiply mismatch at y_len={y_len}"
    );
    count += 1;
  }
  info!("> multiply threshold boundary: {count} multiplications");
  OK
}

#[test]
fn round2_to_string_fast_threshold_boundary() -> Void {
  // fast ↔ classic 切换点: K_TO_STRING_FAST_THRESHOLD = 23 digits ± 1, 全 radix
  let mut rng = Rng::new(314159265359);
  let mut proc = processor();
  for size in [
    K_TO_STRING_FAST_THRESHOLD - 1,
    K_TO_STRING_FAST_THRESHOLD,
    K_TO_STRING_FAST_THRESHOLD + 1,
  ] {
    let digits = generate_random(&mut rng, size as usize);
    for radix in 2u32..=36 {
      let n = to_string_result_length(&digits, radix, false) as usize;
      let mut out_fast = vec![0u8; n];
      let w_fast = proc.write_impl(&mut out_fast, &digits, radix, false, true);
      let mut out_classic = vec![0u8; n];
      let w_cl = proc.write_impl(&mut out_classic, &digits, radix, false, false);
      assert_eq!(
        &out_fast[..w_fast as usize],
        &out_classic[..w_cl as usize],
        "fast vs classic radix={radix} size={size}"
      );
    }
  }
  OK
}
