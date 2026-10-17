//! v8-base/src/float16.rs 集成测试
//!
//! 仓库内无 float16-unittest.cc (仅 mjsunit JS 测试), 故按 IEEE-754 binary16 标准
//! 参考值转写: 规格化/非规格化/±0/±inf/NaN 位模式与 RNE (round-to-nearest-even) 舍入.
//! 断言 `Float16::from_float32` 与 `Float16::to_float32` 的位级结果.

use aok::{OK, Void};
use v8_base::float16::Float16;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn float16_from_float32_normal() -> Void {
  assert_eq!(0x3C00, Float16::from_float32(1.0).get_bits());
  assert_eq!(0x3800, Float16::from_float32(0.5).get_bits());
  assert_eq!(0xC000, Float16::from_float32(-2.0).get_bits());
  assert_eq!(0xBC00, Float16::from_float32(-1.0).get_bits());
  // 最大规格化 half = 65504
  assert_eq!(0x7BFF, Float16::from_float32(65504.0).get_bits());
  OK
}

#[test]
fn float16_from_float32_zero_special() -> Void {
  assert_eq!(0x0000, Float16::from_float32(0.0).get_bits());
  assert_eq!(0x8000, Float16::from_float32(-0.0).get_bits());
  assert_eq!(0x7C00, Float16::from_float32(f32::INFINITY).get_bits());
  assert_eq!(0xFC00, Float16::from_float32(f32::NEG_INFINITY).get_bits());
  // 65536 超出最大规格化 → inf (溢出)
  assert_eq!(0x7C00, Float16::from_float32(65536.0).get_bits());
  // NaN 静默化, 保留高位 payload → 0x7E00
  let nan_bits = Float16::from_float32(f32::NAN).get_bits();
  assert_eq!(0x7C00, nan_bits & 0x7C00);
  assert_ne!(0x0000, nan_bits & 0x03FF);
  OK
}

#[test]
fn float16_from_float32_round_to_nearest_even() -> Void {
  // 1 + 2^-10 = 精确下一个 half → 0x3C01
  assert_eq!(
    0x3C01,
    Float16::from_float32(1.0 + 2.0f32.powi(-10)).get_bits()
  );
  // 1 + 2^-11: 恰处 1.0 与 0x3C01 中点, 舍向偶数 → 0x3C00
  assert_eq!(
    0x3C00,
    Float16::from_float32(1.0 + 2.0f32.powi(-11)).get_bits()
  );
  // 1 + 3*2^-11: 恰处 0x3C01 与 0x3C02 中点, 前值尾数 1 为奇 → 舍向偶 0x3C02
  assert_eq!(
    0x3C02,
    Float16::from_float32(1.0 + 3.0 * 2.0f32.powi(-11)).get_bits()
  );
  OK
}

#[test]
fn float16_from_float32_subnormal() -> Void {
  // 最小非规格化 2^-24 = 5.960464477539063e-8 → 0x0001
  assert_eq!(0x0001, Float16::from_float32(2.0f32.powi(-24)).get_bits());
  // 2^-25: 恰处 0 与 2^-24 中点, 舍向偶 0 → 0x0000
  assert_eq!(0x0000, Float16::from_float32(2.0f32.powi(-25)).get_bits());
  OK
}

#[test]
fn float16_to_float32_roundtrip() -> Void {
  assert_eq!(1.0f32, Float16::from_bits(0x3C00).to_float32());
  assert_eq!(-1.0f32, Float16::from_bits(0xBC00).to_float32());
  assert_eq!(0.5f32, Float16::from_bits(0x3800).to_float32());
  assert_eq!(-2.0f32, Float16::from_bits(0xC000).to_float32());
  assert_eq!(65504.0f32, Float16::from_bits(0x7BFF).to_float32());
  assert_eq!(f32::INFINITY, Float16::from_bits(0x7C00).to_float32());
  assert_eq!(f32::NEG_INFINITY, Float16::from_bits(0xFC00).to_float32());
  assert!((-0.0f32).to_bits() == Float16::from_bits(0x8000).to_float32().to_bits());
  // 最小非规格化还原为 2^-24
  assert_eq!(2.0f32.powi(-24), Float16::from_bits(0x0001).to_float32());
  // NaN 位 → NaN 值
  assert!(Float16::from_bits(0x7E00).to_float32().is_nan());
  OK
}

#[test]
fn float16_roundtrip_stable() -> Void {
  // 对可精确表示的 half 值, f32→half→f32 应逐位稳定
  for bits in [
    0x0000u16, 0x3800, 0x3C00, 0x3C01, 0x7BFF, 0xBC00, 0xC000, 0x8000,
  ] {
    let f = Float16::from_bits(bits).to_float32();
    assert_eq!(
      bits,
      Float16::from_float32(f).get_bits(),
      "bits {bits:#06x}"
    );
  }
  OK
}
