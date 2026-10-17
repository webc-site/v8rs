//! fast_dtoa 全指数域扫描 + strtod 回读不变量
//!
//! Grisu3 契约 (fast-dtoa.h): SHORTEST 成功时 v == (double)(buffer × 10^(point-length))
//! 逐位成立, 且正确舍入的 strtod 回读必还原输入。
//!
//! 覆盖三类回归:
//! 1. cached power 选择域: 全部可达 w.e() (规格化 e∈[-1085,960] + denormal 至 -1137),
//!    对应 cached-powers.cc ceil 先乘后取的索引公式;
//! 2. DigitGen 的 DIV 魔数表索引域 (divisor_exponent ≤ 8; 原版 div[] 仅 9 项,
//!    integrals ≥ 1e9 会索引 div[9] 越界 — 本扫描实证其不可达);
//! 3. RoundWeed / RoundWeedCounted 全部 debug 前置条件与输出正确性。

use aok::{OK, Void};
use fastrand::Rng;
use v8_base::numbers::{
  dtoa::{DtoaMode, double_to_ascii},
  fast_dtoa::{FAST_DTOA_MAXIMAL_LENGTH, FastDtoaMode, fast_dtoa},
  strtod::strtod,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 单值探针: SHORTEST 成功则 strtod 回读位级还原; PRECISION 多档位数仅验证不断言
/// (v==0 由 DoubleToAscii 入口单独处理, fast_dtoa 前置要求严格正)
fn probe(v: f64) {
  if !(v.is_finite() && v > 0.0) {
    return;
  }
  let mut buf = [0u8; 32];
  if let Some(r) = fast_dtoa(v, FastDtoaMode::Shortest, 0, &mut buf) {
    let digits = &buf[..r.length];
    let back = strtod(digits, r.decimal_point - r.length as i32);
    assert_eq!(
      back.to_bits(),
      v.to_bits(),
      "shortest 回读失败: v={v:e} digits={} point={}",
      String::from_utf8_lossy(digits),
      r.decimal_point
    );
  }
  for p in [1, 2, 9, 17, 20] {
    let mut pbuf = [0u8; 24];
    let _ = fast_dtoa(v, FastDtoaMode::Precision, p, &mut pbuf);
  }
}

/// 规格化全指数域: E ∈ [-1022, 1023] × 角点/均匀/随机尾数
#[test]
fn sweep_normalized_domain() -> Void {
  let mut rng = Rng::new();
  rng.seed(0xD1B7_1758_E219_652C);
  for e in -1022..=1023i32 {
    let biased = ((e + 1023) as u64) << 52;
    // 角点: 全 0 / 1 / 2 / 半 / 满尾数
    for m in [0u64, 1, 2, 1 << 51, (1 << 52) - 2, (1 << 52) - 1] {
      probe(f64::from_bits(biased | m));
    }
    // 均匀 64 档 + 随机 16 档尾数
    for i in 0..64u64 {
      probe(f64::from_bits(biased | (i * ((1 << 52) - 1) / 63)));
    }
    for _ in 0..16 {
      probe(f64::from_bits(biased | (rng.u64(..) & 0xF_FFFF_FFFF_FFFF)));
    }
  }
  OK
}

/// denormal 域: 小尾数角点 + 满尾数 + 均匀档
#[test]
fn sweep_denormal_domain() -> Void {
  for m in [1u64, 2, 3, 1 << 25, 1 << 51, 0xF_FFFF_FFFF_FFFF] {
    probe(f64::from_bits(m));
  }
  for i in 0..256u64 {
    probe(f64::from_bits(i * 0xF_FFFF_FFFF_FFFF / 255));
  }
  OK
}

/// 固定种子随机全域: 规避特殊值 (exp=0x7FF) 与非正数后探针
#[test]
fn sweep_seeded_random() -> Void {
  let mut rng = Rng::new();
  rng.seed(0x3F2F_EE7B_6DB9_0616);
  for _ in 0..100_000 {
    let raw = rng.u64(..) & 0x7FFF_FFFF_FFFF_FFFF;
    let v = f64::from_bits(raw);
    if v > 0.0 && v.is_finite() {
      probe(v);
    }
  }
  OK
}

/// DoubleToAscii 全模式烟测: 覆盖 fast→bignum 回退调度与缓冲区 NUL 终止
/// (bignum FIXED 兜底对大数输出全部整数位, 缓冲须按 dtoa.h 契约给足)
#[test]
fn double_to_ascii_all_modes() -> Void {
  let mut buf = [0u8; 512];
  let mut rng = Rng::new();
  rng.seed(0xABAE_E17F_A23E_BF76);
  for i in 0..2_000u64 {
    let raw = rng.u64(..) & 0x7FFF_FFFF_FFFF_FFFF;
    let v = f64::from_bits(raw);
    if !(v > 0.0 && v.is_finite()) {
      continue;
    }
    for mode in [DtoaMode::Shortest, DtoaMode::Fixed, DtoaMode::Precision] {
      let requested = match mode {
        DtoaMode::Shortest => 0,
        DtoaMode::Fixed => (i % 21) as i32,
        DtoaMode::Precision => 1 + (i % 20) as i32,
      };
      let r = double_to_ascii(v, mode, requested, &mut buf);
      assert_eq!(buf[r.length], 0, "NUL 终止: mode={mode:?} v={v:e}");
      // dtoa-unittest.cc 规约: FIXED 要求 length-point ≤ requested;
      // PRECISION 要求 length ≤ requested; SHORTEST 至多 kBase10MaximalLength 位
      match mode {
        DtoaMode::Shortest => assert!(r.length <= 17),
        DtoaMode::Fixed => {
          assert!(r.length as i64 - i64::from(r.point) <= i64::from(requested));
        }
        DtoaMode::Precision => assert!(r.length <= requested as usize),
      }
    }
  }
  OK
}

/// Grisu3 成功率规约 — fast-dtoa-unittest.cc FastDtoaGayShortest (245-246) /
/// FastDtoaGayPrecision (282-285) 的 CHECK_GT 成功率阈值:
///   SHORTEST 成功率 > 0.99 且存在 length == kFastDtoaMaximalLength(17) 的输出;
///   PRECISION 总成功率 > 0.85; 请求位数 ≤ 15 时成功率 > 0.9999。
/// gay 语料 10 万条在 v8-numbers (见 dtoa_golden.rs); 此处以同量级固定种子
/// 随机全域样本作代理 (阈值按原版语料的随机 double 性质标定, 故不掺入
/// 2 的幂等角点 — Grisu 在精确幂次的高位数档天然回退, 属算法固有行为),
/// 角点正确性已由 sweep_normalized_domain / sweep_denormal_domain 覆盖。
#[test]
fn fast_dtoa_success_rate() -> Void {
  let mut buf = [0u8; 32];

  // 固定种子随机全域样本 10 万 (对齐 gay 语料量级与随机 double 分布)
  let mut samples: Vec<f64> = Vec::with_capacity(100_000);
  let mut rng = Rng::new();
  rng.seed(0x5EED_0F5A_1111_D0A5);
  for _ in 0..100_000 {
    let raw = rng.u64(..) & 0x7FFF_FFFF_FFFF_FFFF;
    let v = f64::from_bits(raw);
    if v > 0.0 && v.is_finite() {
      samples.push(v);
    }
  }

  // SHORTEST: 成功率 + 满长出现标记 (原版 needed_max_length)
  let (mut succ_shortest, mut needed_max_length) = (0usize, false);
  // PRECISION: 总计 与 请求位数 ≤ 15 分桶 (原版 succeeded/total 与 _15)
  let (mut succ_prec, mut total_prec, mut succ15, mut total15) = (0usize, 0usize, 0usize, 0usize);

  for &v in &samples {
    if let Some(r) = fast_dtoa(v, FastDtoaMode::Shortest, 0, &mut buf) {
      assert!(r.length <= FAST_DTOA_MAXIMAL_LENGTH);
      if r.length == FAST_DTOA_MAXIMAL_LENGTH {
        needed_max_length = true;
      }
      succ_shortest += 1;
    }
    // 请求位数在 [1, 20] 均匀轮转, 覆盖 >15 高失败率区与 ≤15 区
    // (计数器用 total_prec: 恒递增; 若用成功数会在 digits=20 档死锁)
    let digits = 1 + (total_prec % 20) as i32;
    total_prec += 1;
    if digits <= 15 {
      total15 += 1;
    }
    if let Some(r) = fast_dtoa(v, FastDtoaMode::Precision, digits, &mut buf) {
      assert!(r.length <= digits as usize, "GE({digits}, length)");
      succ_prec += 1;
      if digits <= 15 {
        succ15 += 1;
      }
    }
  }

  let rate_shortest = succ_shortest as f64 / samples.len() as f64;
  assert!(
    rate_shortest > 0.99,
    "SHORTEST 成功率 {rate_shortest:.4} 须 > 0.99"
  );
  assert!(
    needed_max_length,
    "须存在 17 位 (kFastDtoaMaximalLength) 输出"
  );

  let rate_prec = succ_prec as f64 / total_prec as f64;
  assert!(
    rate_prec > 0.85,
    "PRECISION 成功率 {rate_prec:.4} 须 > 0.85"
  );
  let rate15 = succ15 as f64 / total15 as f64;
  assert!(
    rate15 > 0.9999,
    "PRECISION(≤15位) 成功率 {rate15:.5} 须 > 0.9999"
  );
  OK
}
