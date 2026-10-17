//! v8-base/src/numbers/diy_fp.rs 集成测试 — 1:1 对应 diy-fp-unittest.cc
//!
//! 原版用例: `DiyFpTest.Subtract` / `DiyFpTest.Multiply` (含 128 位乘积舍入边界).

use aok::{OK, Void};
use v8_base::numbers::diy_fp::DiyFp;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn diy_fp_subtract() -> Void {
  let diy_fp1 = DiyFp::new(3, 0);
  let diy_fp2 = DiyFp::new(1, 0);
  let diff = DiyFp::minus(diy_fp1, diy_fp2);
  assert_eq!(2u64, diff.f());
  assert_eq!(0i32, diff.e());

  let mut a = DiyFp::new(3, 0);
  a.subtract(diy_fp2);
  assert_eq!(2u64, a.f());
  assert_eq!(0i32, a.e());
  OK
}

#[test]
fn diy_fp_multiply() -> Void {
  let mut diy_fp1 = DiyFp::new(3, 0);
  let diy_fp2 = DiyFp::new(2, 0);
  let product = DiyFp::times(diy_fp1, diy_fp2);
  assert_eq!(0u64, product.f());
  assert_eq!(64i32, product.e());

  diy_fp1.multiply(diy_fp2);
  assert_eq!(0u64, diy_fp1.f());
  assert_eq!(64i32, diy_fp1.e());

  diy_fp1 = DiyFp::new(0x8000_0000_0000_0000, 11);
  let mut fp2b = DiyFp::new(2, 13);
  let product = DiyFp::times(diy_fp1, fp2b);
  assert_eq!(1u64, product.f());
  assert_eq!(11 + 13 + 64, product.e());

  // 舍入测试.
  diy_fp1 = DiyFp::new(0x8000_0000_0000_0001, 11);
  fp2b = DiyFp::new(1, 13);
  let product = DiyFp::times(diy_fp1, fp2b);
  assert_eq!(1u64, product.f());
  assert_eq!(11 + 13 + 64, product.e());

  diy_fp1 = DiyFp::new(0x7FFF_FFFF_FFFF_FFFF, 11);
  fp2b = DiyFp::new(1, 13);
  let product = DiyFp::times(diy_fp1, fp2b);
  assert_eq!(0u64, product.f());
  assert_eq!(11 + 13 + 64, product.e());

  // 恰 halfway 情形允许任一方向舍入, 原版不校验.

  // 大数: 128 位乘积 0xFFFFFFFFFFFFFFFE0000000000000001.
  diy_fp1 = DiyFp::new(0xFFFF_FFFF_FFFF_FFFF, 11);
  fp2b = DiyFp::new(0xFFFF_FFFF_FFFF_FFFF, 13);
  let product = DiyFp::times(diy_fp1, fp2b);
  assert_eq!(0xFFFF_FFFF_FFFF_FFFE, product.f());
  assert_eq!(11 + 13 + 64, product.e());
  OK
}
