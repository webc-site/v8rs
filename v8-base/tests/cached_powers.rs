//! v8-base/src/numbers/cached_powers.rs 回归测试 — GetCachedPowerForBinaryExponentRange
//! ceil 位置转写缺陷 (原版先乘 kD_1_LOG2_10 再 ceil, 转写误成先 ceil 后乘 + as i32 截断)
//!
//! 缺陷影响: min_exponent + kQ - 1 > 0 (约 v ≲ 4) 且乘积含小数部分时选错 cached power,
//! debug 下 cached_powers.rs 断言崩, release 下 fast_dtoa (Grisu3/Precision) 输出错乱。
//! 以下用例取自 gay-precision / gay-shortest 黄金数据中修复前的真实失败行
//! (10 万条差分: 修复前 shortest 5326 / precision 4252 条错乱, 修复后 0)。

use aok::{OK, Void};
use v8_base::numbers::{
  cached_powers::PowersOfTenCache,
  dtoa::{DtoaMode, double_to_ascii},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// TrimRepresentation — 去缓冲区尾零 (dtoa-unittest.cc 30-37)
fn trim_len(buf: &[u8], length: usize) -> usize {
  let mut end = length;
  while end > 0 && buf[end - 1] == b'0' {
    end -= 1;
  }
  end
}

/// (bits, requested_digits, 期望 digits, 期望 point)
const PRECISION_CASES: [(u64, i32, &str, i32); 10] = [
  (0x2ec65d5a0323a745, 2, "23", -82),
  (0x1aab70677eb017d7, 14, "33062991029888", -179),
  // 1.455e-4 量级: 0.0001455328016994476, 触发 cached_powers.rs:131 断言的代表输入
  (0x3f2313453f61fa21, 11, "1455328017", -3),
  (0x2b3d28837c4b96c1, 15, "208296995712757", -99),
  (0x10b52e6469ed2b94, 16, "3492643511338243", -227),
  (0x4f36589dcc5c8f7, 2, "82", -284),
  (0x2484d42f88c79412, 18, "917016647332706746", -132),
  (0x6cb7880721cab34, 3, "62", -275),
  (0x1c3a97181a0d22f2, 4, "1075", -171),
  (0x1c552260108519, 9, "394012682", -307),
];

/// (bits, 期望 digits, 期望 point)
const SHORTEST_CASES: [(u64, &str, i32); 4] = [
  (0x2b212f3bce5a6fef, "6138023139711468", -100),
  (0x130dce6d7cbdcf, "2649775247647607", -307),
  (0x3f2fee7b6db90616, "2436185521777357", -3),
  (0x3f2e408b6bacac9f, "23080541526474416", -3),
];

#[test]
fn cached_power_ceil_precision() -> Void {
  let mut buf = [0u8; 64];
  for (bits, requested, want_digits, want_point) in PRECISION_CASES {
    let r = double_to_ascii(
      f64::from_bits(bits),
      DtoaMode::Precision,
      requested,
      &mut buf,
    );
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    let t = trim_len(&buf, len);
    assert_eq!(
      (want_point, want_digits.as_bytes()),
      (r.point, &buf[..t]),
      "precision v={:x} requested={requested}",
      bits
    );
  }
  OK
}

#[test]
fn cached_power_ceil_shortest() -> Void {
  let mut buf = [0u8; 32];
  for (bits, want_digits, want_point) in SHORTEST_CASES {
    let r = double_to_ascii(f64::from_bits(bits), DtoaMode::Shortest, 0, &mut buf);
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    assert_eq!(
      (want_point, want_digits.as_bytes()),
      (r.point, &buf[..len]),
      "shortest v={:x}",
      bits
    );
  }
  OK
}

/// 索引公式扫描: 与原版 ceil((n) * kD_1_LOG2_10) 逐点对拍全可达指数域,
/// 保证选中的 cached power 的 binary_exponent 落在 [min, max] 内
#[test]
fn cached_power_binary_exponent_range() -> Void {
  // w.e() 可达域: 规格化数 exponent - 11 ∈ [-1085-11? .. 960];
  // denormal 最小 (5e-324) 时 w.e() = -1137, 最大 double 为 960
  for w_e in -1137..=960i32 {
    let min = -60 - (w_e + 64);
    let max = -32 - (w_e + 64);
    let (power, _) = PowersOfTenCache::get_cached_power_for_binary_exponent_range(min, max);
    assert!(
      min <= power.e() && power.e() <= max,
      "w_e={w_e}: cached binary exp {} ∉ [{min}, {max}]",
      power.e()
    );
  }
  OK
}
