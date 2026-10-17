//! v8-base/src/ieee754.rs 集成测试 — 对应 ieee754-unittest.cc
//!
//! 本文件只转写 ieee754.rs 中 **自含算法** 的用例 (acosh/asinh/atanh/cosh/sinh),
//! 逐字节比对原版 EXPECT_EQ 金标准. cos/sin/exp/log/tan 等委托平台 libm, 其金标准
//! 与 llvm-libc shared 内核绑定, 属推迟项, 不在此断言.
//! Rust 无法区分 signaling NaN, 原 `IsNaN()` 断言以 `is_nan()` 承接.

use aok::{OK, Void};
use v8_base::ieee754;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

const INFINITY: f64 = f64::INFINITY;
const NEG_INFINITY: f64 = f64::NEG_INFINITY;
const QNAN: f64 = f64::NAN;
// Rust 中 sNaN 与 qNaN 同为 NAN; 保留常量以对应原版每条 sNaN 断言.
const SNAN: f64 = f64::NAN;

#[test]
fn ieee754_acosh() -> Void {
  assert_eq!(INFINITY, ieee754::acosh(INFINITY));
  assert!(ieee754::acosh(NEG_INFINITY).is_nan());
  assert!(ieee754::acosh(QNAN).is_nan());
  assert!(ieee754::acosh(SNAN).is_nan());
  assert!(ieee754::acosh(0.9).is_nan());

  assert_eq!(0.0, ieee754::acosh(1.0));
  // acosh(1.5) = log((sqrt(5)+3)/2)
  assert_eq!(0.962_423_650_119_206_9e0, ieee754::acosh(1.5));
  // acosh(4) = log(sqrt(15)+4)
  assert_eq!(2.063_437_068_895_560_8e0, ieee754::acosh(4.0));
  // acosh(2^50)
  assert_eq!(
    35.350_506_208_557_21e0,
    ieee754::acosh(1_125_899_906_842_624.0)
  );
  // acosh(most-positive-float), 无上溢
  assert_eq!(
    710.475_860_073_943_9e0,
    ieee754::acosh(1.797_693_134_862_315_7e308)
  );
  OK
}

#[test]
fn ieee754_asinh() -> Void {
  assert_eq!(INFINITY, ieee754::asinh(INFINITY));
  assert_eq!(NEG_INFINITY, ieee754::asinh(NEG_INFINITY));

  assert!(ieee754::asinh(0.0).to_bits() == 0.0f64.to_bits());
  // asinh(-0.0) 保持 -0.0 位模式
  assert!(ieee754::asinh(-0.0).to_bits() == (-0.0f64).to_bits());
  // asinh(2^-29) = 2^-29
  assert_eq!(
    1.862_645_149_230_957e-9,
    ieee754::asinh(1.862_645_149_230_957e-9)
  );
  assert_eq!(
    -1.862_645_149_230_957e-9,
    ieee754::asinh(-1.862_645_149_230_957e-9)
  );
  // asinh(2^-28)
  assert_eq!(
    3.725_290_298_461_914e-9,
    ieee754::asinh(3.725_290_298_461_914e-9)
  );
  assert_eq!(
    -3.725_290_298_461_914e-9,
    ieee754::asinh(-3.725_290_298_461_914e-9)
  );
  // asinh(±1)
  assert_eq!(0.881_373_587_019_543e0, ieee754::asinh(1.0));
  assert_eq!(-0.881_373_587_019_543e0, ieee754::asinh(-1.0));
  // asinh(±5)
  assert_eq!(2.312_438_341_272_752_5e0, ieee754::asinh(5.0));
  assert_eq!(-2.312_438_341_272_752_5e0, ieee754::asinh(-5.0));
  // asinh(2^28)
  assert_eq!(20.101_268_236_238_415e0, ieee754::asinh(268_435_456.0));
  assert_eq!(-20.101_268_236_238_415e0, ieee754::asinh(-268_435_456.0));
  // asinh(±most-positive-float), 无上溢
  assert_eq!(
    710.475_860_073_943_9e0,
    ieee754::asinh(1.797_693_134_862_315_7e308)
  );
  assert_eq!(
    -710.475_860_073_943_9e0,
    ieee754::asinh(-1.797_693_134_862_315_7e308)
  );
  OK
}

#[test]
fn ieee754_atanh() -> Void {
  assert!(ieee754::atanh(QNAN).is_nan());
  assert!(ieee754::atanh(SNAN).is_nan());
  assert!(ieee754::atanh(INFINITY).is_nan());
  assert_eq!(INFINITY, ieee754::atanh(1.0));
  assert_eq!(NEG_INFINITY, ieee754::atanh(-1.0));
  assert!((ieee754::atanh(0.5) - 0.549_306_144_334_054_9).abs() <= 1e-15);
  OK
}

#[test]
fn ieee754_cosh() -> Void {
  assert!(ieee754::cosh(QNAN).is_nan());
  assert!(ieee754::cosh(SNAN).is_nan());
  assert_eq!(INFINITY, ieee754::cosh(INFINITY));
  assert_eq!(INFINITY, ieee754::cosh(NEG_INFINITY));
  assert_eq!(1.0, ieee754::cosh(0.0));
  assert_eq!(1.0, ieee754::cosh(-0.0));
  OK
}

#[test]
fn ieee754_sinh() -> Void {
  assert!(ieee754::sinh(QNAN).is_nan());
  assert!(ieee754::sinh(SNAN).is_nan());
  assert_eq!(INFINITY, ieee754::sinh(INFINITY));
  assert_eq!(NEG_INFINITY, ieee754::sinh(NEG_INFINITY));
  assert_eq!(0.0, ieee754::sinh(0.0));
  // sinh(-0.0) 保持 -0.0
  assert!(ieee754::sinh(-0.0).to_bits() == (-0.0f64).to_bits());
  OK
}
