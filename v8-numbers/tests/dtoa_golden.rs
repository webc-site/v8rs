//! unittests/base/dtoa-unittest.cc 的 DtoaGay* 三组黄金数据驱动测试
//! 数据: tests/data/gay-{shortest,fixed,precision}.tsv
//!   (tool/src/numbers/gay-data.js 从 test/unittests/gay-*.cc 提取, 各 10 万条)
//!
//! 断言规约 = dtoa-unittest.cc 260-322 逐条:
//!   shortest: sign==0 ∧ point==golden ∧ buffer==representation
//!   fixed:    sign==0 ∧ point==golden ∧ number_digits≥length-point ∧ trim(buffer)==representation
//!   precision: sign==0 ∧ point==golden ∧ number_digits≥length ∧ trim(buffer)==representation
//!
//! 通道覆盖 (todo 10.3: ECMAScript 格式层以 zmij 为最短内核):
//! - dtoa_gay_shortest: zmij (shortest_decimal) + v8-base
//!   DoubleToAscii(SHORTEST) 双通道各全量 10 万
//!   (上游 cached_powers.rs ceil 顺序缺陷已修, 通道恢复)
//! - dtoa_gay_fixed: v8-base DoubleToAscii(FIXED) 通道全量 10 万
//! - dtoa_gay_precision: v8-base DoubleToAscii(PRECISION) 通道全量 10 万
//!   (曾因上游缺陷骨架化, 修复后恢复全量比对)

use std::fs::read_to_string;

use aok::{OK, Void};
use v8_base::numbers::dtoa::{DtoaMode, double_to_ascii};
use v8_numbers::conversions::to_string::shortest_decimal;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 一行: (v, extra, digits, point); extra = fixed 的小数位 / precision 的有效位
fn load(name: &str) -> Vec<(f64, i32, String, i32)> {
  let p = format!("{}/tests/data/{name}", env!("CARGO_MANIFEST_DIR"));
  let raw = read_to_string(&p).unwrap_or_default();
  let mut rows = Vec::new();
  for line in raw.lines().skip(1) {
    if line.is_empty() {
      continue;
    }
    let mut it = line.split('\t');
    let bits = u64::from_str_radix(it.next().unwrap_or_default(), 16).unwrap_or_default();
    let extra: i32 = it.next().unwrap_or_default().parse().unwrap_or_default();
    let digits = it.next().unwrap_or_default().to_string();
    let point: i32 = it.next().unwrap_or_default().parse().unwrap_or_default();
    rows.push((f64::from_bits(bits), extra, digits, point));
  }
  rows
}

/// TrimRepresentation — 去缓冲区尾零, 返回截断后长度 (dtoa-unittest.cc 30-37)
fn trim(buf: &mut [u8], length: usize) -> usize {
  let mut end = length;
  while end > 0 && buf[end - 1] == b'0' {
    end -= 1;
  }
  end
}

/// 缓冲区 NUL 终止符位置 (原版 strcmp 语义的有效长度)
fn strlen(buf: &[u8]) -> usize {
  buf.iter().position(|&c| c == 0).unwrap_or(buf.len())
}

// DtoaTest.DtoaGayShortest — 10 万条 × 双通道: zmij 内核 + v8-base DoubleToAscii(SHORTEST)
#[test]
fn dtoa_gay_shortest() -> Void {
  let rows = load("gay-shortest.tsv");
  assert_eq!(100_000, rows.len(), "黄金数据条数");
  let mut zmij_digits = [0u8; 24];
  let mut mismatch = 0usize;
  for (i, (v, _, digits, point)) in rows.iter().enumerate() {
    // 最短表示: v = digits × 10^dec_exp; decimal_point = 有效位数 + dec_exp
    let (len, dec_exp, _) = shortest_decimal(*v, &mut zmij_digits);
    let zmij_point = len as i32 + dec_exp;
    if zmij_point != *point || &zmij_digits[..len] != digits.as_bytes() {
      mismatch += 1;
      if mismatch <= 5 {
        log::warn!(
          "zmij 差异 @{i}: v={v} 期望 {}e{point} 实得 {}e{zmij_point}",
          digits,
          String::from_utf8_lossy(&zmij_digits[..len])
        );
      }
    }
  }
  assert_eq!(0, mismatch, "zmij 内核与黄金数据差异条数");

  // v8-base DoubleToAscii(SHORTEST) 通道全量:
  //   sign==0 ∧ point==golden ∧ buffer[..length]==representation
  let mut v8_mismatch = 0usize;
  let mut buf = [0u8; 128];
  for (i, (v, _, digits, point)) in rows.iter().enumerate() {
    let r = double_to_ascii(*v, DtoaMode::Shortest, 0, &mut buf);
    if r.sign != 0 || r.point != *point || &buf[..r.length] != digits.as_bytes() {
      v8_mismatch += 1;
      if v8_mismatch <= 5 {
        log::warn!(
          "v8-base shortest @{i}: v={v} 期望 {}e{point} 实得 {}e{}",
          digits,
          String::from_utf8_lossy(&buf[..r.length]),
          r.point
        );
      }
    }
  }
  assert_eq!(0, v8_mismatch, "v8-base SHORTEST 通道与黄金数据差异条数");
  OK
}

// DtoaTest.DtoaGayFixed — 10 万条 v8-base DoubleToAscii(FIXED) 全量比对
#[test]
fn dtoa_gay_fixed() -> Void {
  let rows = load("gay-fixed.tsv");
  assert_eq!(100_000, rows.len());
  let mut buf = [0u8; 1024];
  let mut mismatch = 0usize;
  for (i, (v, frac, digits, point)) in rows.iter().enumerate() {
    let r = double_to_ascii(*v, DtoaMode::Fixed, *frac, &mut buf);
    let n = strlen(&buf);
    let t = trim(&mut buf, n);
    if r.sign != 0
      || r.point != *point
      // 小数位足够容纳 (断言规约: number_digits >= length - point)
      || (*frac as usize) < r.length.saturating_sub(r.point as usize)
      || &buf[..t] != digits.as_bytes()
    {
      mismatch += 1;
      if mismatch <= 5 {
        log::warn!(
          "fixed @{i}: v={v:e} frac={frac} sign={} point={} got={} want point={point} digits={digits}",
          r.sign,
          r.point,
          String::from_utf8_lossy(&buf[..t])
        );
      }
    }
  }
  assert_eq!(0, mismatch, "FIXED 通道与黄金数据差异条数");
  OK
}

// DtoaTest.DtoaGayPrecision — 10 万条 v8-base DoubleToAscii(PRECISION) 全量比对
#[test]
fn dtoa_gay_precision() -> Void {
  let rows = load("gay-precision.tsv");
  assert_eq!(100_000, rows.len());
  assert!(
    rows
      .iter()
      .all(|(_, p, ..)| (1..=MAX_PRECISION).contains(p)),
    "precision 域"
  );
  // 全量比对 (dtoa-unittest.cc 260-322 规约):
  //   sign==0 ∧ point==golden ∧ number_digits>=length ∧ trim_rep(buffer)==golden
  let mut mismatch = 0usize;
  let mut buf = [0u8; MAX_PRECISION as usize + 1];
  for (i, (v, prec, digits, point)) in rows.iter().enumerate() {
    let r = double_to_ascii(*v, DtoaMode::Precision, *prec, &mut buf);
    let t = trim(&mut buf, r.length);
    if r.sign != 0
      || r.point != *point
      // 请求位数须容纳实际产出 (断言规约: number_digits >= length)
      || (*prec as usize) < r.length
      || &buf[..t] != digits.as_bytes()
    {
      mismatch += 1;
      if mismatch <= 5 {
        log::warn!(
          "precision @{i}: v={v:e} p={prec} sign={} point={} len={} got={} want point={point} digits={digits}",
          r.sign,
          r.point,
          r.length,
          String::from_utf8_lossy(&buf[..t])
        );
      }
    }
  }
  assert_eq!(0, mismatch, "PRECISION 通道与黄金数据差异条数");
  OK
}

/// 黄金 precision 域上界 (kMaxFractionDigits)
const MAX_PRECISION: i32 = 100;
