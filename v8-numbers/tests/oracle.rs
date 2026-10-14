//! ECMAScript 差分语料驱动测试 (bun/JSC 为参考实现, tool/src/numbers/oracle-data.js 录制)
//!
//! - oracle-parse.tsv: Number() / parseFloat() / parseInt(r) × 12k 条
//! - oracle-fmt.tsv:   toString / toFixed / toPrecision / toExponential / toString(radix) × 2 万条
//! - oracle-rng.tsv:   MurmurHash3 / xorshift128+ / ToDouble 序列
//! - oracle-pow.tsv:   Math.pow 边界 (±1 ** ±∞ = NaN 等)

use std::fs::read_to_string;

use aok::{OK, Void};
use v8_numbers::{
  conversions::{
    ConversionFlag, double_to_exponential_string_view, double_to_fixed_string_view,
    double_to_precision_string_view, double_to_radix_string_view, double_to_string_view,
    string_to_double, string_to_int,
  },
  ieee754::pow,
  math_random::{default_state_from_seed, murmur_hash3, random_to_double, xor_shift128},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

fn data(name: &str) -> String {
  let p = format!("{}/tests/data/{name}", env!("CARGO_MANIFEST_DIR"));
  read_to_string(&p).unwrap_or_default()
}

fn bits(s: &str) -> f64 {
  f64::from_bits(u64::from_str_radix(s, 16).unwrap_or_default())
}

/// 解码录制时转义的空白
fn unescape(s: &str) -> String {
  let mut out = String::new();
  let mut it = s.chars();
  while let Some(c) = it.next() {
    if c == '\\' {
      match it.next() {
        Some('t') => out.push('\t'),
        Some('n') => out.push('\n'),
        Some('r') => out.push('\r'),
        Some('\\') => out.push('\\'),
        Some(other) => {
          out.push('\\');
          out.push(other);
        }
        None => out.push('\\'),
      }
    } else {
      out.push(c);
    }
  }
  out
}

#[test]
fn oracle_parse() -> Void {
  let raw = data("oracle-parse.tsv");
  let mut n = 0usize;
  for line in raw.lines().skip(1) {
    if line.is_empty() {
      continue;
    }
    let mut it = line.split('\t');
    let input = unescape(it.next().unwrap_or_default());
    let op = it.next().unwrap_or_default();
    let radix: i32 = it.next().unwrap_or_default().parse().unwrap_or_default();
    let expected = bits(it.next().unwrap_or_default());
    let bytes = input.as_bytes();
    let got = match op {
      // Number(): ToNumber 应用于 String — 允许 0x/0o/0b 前缀与 Infinity 字面量
      "number" => string_to_double(bytes, ConversionFlag::AllowNonDecimalPrefix, 0.0),
      // parseFloat(): 允许尾随垃圾, 空串 → NaN
      "parsefloat" => string_to_double(bytes, ConversionFlag::AllowTrailingJunk, f64::NAN),
      // parseInt(str, radix): radix 0 → 探测; junk/空 → NaN
      "parseint" => string_to_int(bytes, radix),
      _ => f64::NAN,
    };
    // NaN 位型可能不同 (quiet NaN 规范无位级要求), 以 is_nan 分流
    if expected.is_nan() {
      assert!(got.is_nan(), "{op}({input:?}, {radix}) 期望 NaN 实得 {got}");
    } else if op == "parseint" {
      // JSC 的 parseInt 大数域 (含 radix 16) 为 double 累乘近似, 有 1 ulp 级误差
      // (python 已证 V8 算法为精确舍入); 非特殊 radix 超域按规范允许的实现近似放宽
      let tol = if radix != 0
        && ![2, 4, 8, 10, 16, 32].contains(&radix)
        && expected.abs() > 72_057_594_037_927_936.0
      {
        expected.abs() * 1e-9
      } else {
        expected.abs().max(1.0) * f64::EPSILON
      };
      assert!(
        (got - expected).abs() <= tol,
        "{op}({input:?}, {radix}) 期望 {expected} 实得 {got}"
      );
    } else {
      assert_eq!(
        expected.to_bits(),
        got.to_bits(),
        "{op}({input:?}, {radix})"
      );
    }
    n += 1;
  }
  assert!(n > 12_000, "语料条数 {n}");
  OK
}

#[test]
fn oracle_fmt() -> Void {
  let raw = data("oracle-fmt.tsv");
  let mut n = 0usize;
  let mut buf = [0u8; 2200];
  for line in raw.lines().skip(1) {
    if line.is_empty() {
      continue;
    }
    let mut it = line.split('\t');
    let v = bits(it.next().unwrap_or_default());
    let op = it.next().unwrap_or_default();
    let arg: i32 = it.next().unwrap_or_default().parse().unwrap_or_default();
    let expected = it.next().unwrap_or_default();
    let got = match op {
      "tostring" => double_to_string_view(v, &mut buf).to_string(),
      "tofixed" => double_to_fixed_string_view(v, arg, &mut buf).to_string(),
      "toprecision" => double_to_precision_string_view(v, arg, &mut buf).to_string(),
      "toexponential" => double_to_exponential_string_view(v, arg, &mut buf).to_string(),
      "toradix" => double_to_radix_string_view(v, arg, &mut buf).to_string(),
      _ => String::new(),
    };
    if op == "toradix" {
      // toString(radix≠10) 属 ECMAScript implementation-defined: JSC 与 V8
      // 的整数/小数近似策略均不同, 不做 JSC 差分;
      // V8 行为由 mjsunit number-tostring.js 权威向量护航 (conversions.rs)
      continue;
    }
    assert_eq!(expected, got, "{op}({v}, {arg})");
    n += 1;
  }
  // tostring/tofixed/toprecision/toexponential 全量差分 (上游 dtoa 修复后恢复);
  // toradix 为 implementation-defined 不差分 (V8 向量另测, 见 conversions.rs)
  assert!(n > 16_000, "语料条数 {n}");
  OK
}

#[test]
fn oracle_rng() -> Void {
  let raw = data("oracle-rng.tsv");
  let mut n = 0usize;
  for line in raw.lines().skip(1) {
    if line.is_empty() {
      continue;
    }
    let mut it = line.split('\t');
    let op = it.next().unwrap_or_default();
    let arg = it.next().unwrap_or_default();
    let expected = it.next().unwrap_or_default();
    match op {
      "murmur3" => {
        let seed = u64::from_str_radix(arg, 16).unwrap_or_default();
        assert_eq!(expected, format!("{:x}", murmur_hash3(seed)));
      }
      "state" => {
        // 种子派生: s0 = MurmurHash3(seed), s1 = MurmurHash3(~seed)
        let seed = u64::from_str_radix(arg, 16).unwrap_or_default();
        let state = default_state_from_seed(seed);
        assert_eq!(
          expected,
          format!("{:x},{:x}", state.s0, state.s1),
          "state(seed={arg})"
        );
      }
      "xorshift128" => {
        // 序列逐项重放比对: "seed:idx"
        let (seed_hex, idx) = arg.split_once(':').unwrap_or(("", "0"));
        let seed = u64::from_str_radix(seed_hex, 16).unwrap_or_default();
        let idx: usize = idx.parse().unwrap_or_default();
        let state = default_state_from_seed(seed);
        let (mut s0, mut s1) = (state.s0, state.s1);
        let mut out = 0u64;
        for _ in 0..=idx {
          out = xor_shift128(&mut s0, &mut s1);
        }
        assert_eq!(expected, format!("{out:x}"), "xorshift128 seed={arg}");
      }
      "to_double" => {
        let v = u64::from_str_radix(arg, 16).unwrap_or_default();
        assert_eq!(
          expected.parse::<f64>().unwrap_or_default(),
          random_to_double(v)
        );
      }
      _ => {}
    }
    n += 1;
  }
  assert!(n > 130, "语料条数 {n}");
  OK
}

#[test]
fn oracle_pow() -> Void {
  let raw = data("oracle-pow.tsv");
  let mut n = 0usize;
  for line in raw.lines().skip(1) {
    if line.is_empty() {
      continue;
    }
    let mut it = line.split('\t');
    let x = bits(it.next().unwrap_or_default());
    let y = bits(it.next().unwrap_or_default());
    let expected = bits(it.next().unwrap_or_default());
    let got = pow(x, y);
    if expected.is_nan() {
      assert!(got.is_nan(), "pow({x}, {y}) 期望 NaN 实得 {got}");
    } else if (got - expected).abs() > f64::EPSILON * expected.abs() * 8.0 {
      // Math.pow 属 ECMAScript implementation-approximated, 允许实现级近似;
      // 超 8 ulp 才算缺陷 (legacy::pow 目前委托平台 libm, 见 README 遗留)
      panic!("pow({x}, {y}) 期望 {expected} 实得 {got}");
    }
    n += 1;
  }
  assert!(n >= 30, "语料条数 {n}");
  OK
}
