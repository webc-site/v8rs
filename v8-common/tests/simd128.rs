//! Simd128 bit_cast 往返与 splat 校验

use aok::{OK, Void};
use v8_common::{Float32x4, Float64x2, Int8x16, Int16x8, Int32x4, Int64x2, Simd128};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 各类型 splat → to 位级往返 (对应原版 Splat / to_##name)
#[test]
fn simd128_splat_roundtrip() -> Void {
  // f64x2: 值 + NaN 位型
  let f = Simd128::splat_f64x2(1.5);
  assert_eq!(f.to_f64x2(), [1.5, 1.5]);
  let nan = f64::from_bits(0xFFF7_FFFF_FFF7_FFFF);
  let roundtrip = Simd128::splat_f64x2(nan).to_f64x2();
  assert_eq!(roundtrip[0].to_bits(), nan.to_bits());
  assert_eq!(roundtrip[1].to_bits(), nan.to_bits());

  // f32x4
  assert_eq!(Simd128::splat_f32x4(-2.25).to_f32x4(), [-2.25f32; 4]);

  // 整数族
  assert_eq!(Simd128::splat_i64x2(-7).to_i64x2(), [-7i64; 2]);
  assert_eq!(
    Simd128::splat_i32x4(0x1234_5678).to_i32x4(),
    [0x1234_5678i32; 4]
  );
  assert_eq!(Simd128::splat_i16x8(0x1234).to_i16x8(), [0x1234i16; 8]);
  assert_eq!(Simd128::splat_i8x16(-1).to_i8x16(), [-1i8; 16]);
  OK
}

/// from → bytes → from: 原始字节一致性 (bit_cast = memcpy)
#[test]
fn simd128_bytes_roundtrip() -> Void {
  let v: Float64x2 = [1.234_567_890_123_456_8, -0.5];
  let simd = Simd128::from_f64x2(v);
  let bytes = *simd.bytes();
  let back = Simd128::from_bytes(bytes);
  assert_eq!(back.to_f64x2(), v);
  // 混合解释: 同一 16 字节按 i32x4 读出 (端序敏感, 宿主 LE)
  let i32x4: Int32x4 = simd.to_i32x4();
  let f64_bits = [v[0].to_bits(), v[1].to_bits()];
  assert_eq!(
    i32x4,
    [
      (f64_bits[0] & 0xFFFF_FFFF) as u32 as i32,
      (f64_bits[0] >> 32) as u32 as i32,
      (f64_bits[1] & 0xFFFF_FFFF) as u32 as i32,
      (f64_bits[1] >> 32) as u32 as i32,
    ]
  );
  OK
}

/// 全零默认值与 PartialEq (simd128.h:31, 47)
#[test]
fn simd128_zero_equality() -> Void {
  assert_eq!(Simd128::ZERO, Simd128::default());
  assert_eq!(Simd128::ZERO.bytes(), &[0u8; 16]);
  assert_eq!(Simd128::ZERO, Simd128::from_i64x2([0, 0]));
  assert_ne!(Simd128::ZERO, Simd128::splat_i8x16(1));
  OK
}

/// 类型别名 lane 数对数 (FOREACH_SIMD_TYPE)
#[test]
fn simd128_lane_counts() -> Void {
  let _: Float64x2 = [0.0; 2];
  let _: Float32x4 = [0.0; 4];
  let _: Int64x2 = [0; 2];
  let _: Int32x4 = [0; 4];
  let _: Int16x8 = [0; 8];
  let _: Int8x16 = [0; 16];
  // 16 字节容量
  assert_eq!(size_of::<Simd128>(), 16);
  OK
}
