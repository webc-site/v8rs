// 由 tool/src/common.js 从 v8/src/common/simd128.h 生成; 修订就地改, 复跑覆盖
//
// 修订 (Developer): FOREACH_SIMD_TYPE X-macro 展开为类型别名 + 宏生成的
// from / splat / to 三族方法 (simd128.h:26-42); bit_cast 语义 = 宿主端序
// memcpy (to_ne_bytes / from_ne_bytes)。

/// FOREACH_SIMD_TYPE 展开 (simd128.h:16-22)
pub type Float64x2 = [f64; 2];
pub type Float32x4 = [f32; 4];
pub type Int64x2 = [i64; 2];
pub type Int32x4 = [i32; 4];
pub type Int16x8 = [i16; 8];
pub type Int8x16 = [i8; 16];

/// 原版 class alignas(double) Simd128 — 16 字节按 f64 对齐的原始值容器
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(align(8))]
pub struct Simd128 {
  val: [u8; 16],
}

impl Default for Simd128 {
  /// 原版 constexpr Simd128() = default → 全零
  fn default() -> Self {
    Self::ZERO
  }
}

impl Simd128 {
  /// 原版 constexpr Simd128() = default → 全零
  pub const ZERO: Self = Self { val: [0; 16] };

  /// 原版 explicit Simd128(uint8_t*) memcpy (simd128.h:45) — 定长入参安全化
  pub const fn from_bytes(bytes: [u8; 16]) -> Self {
    Self { val: bytes }
  }

  /// 原版 bytes() (simd128.h:49)
  pub const fn bytes(&self) -> &[u8; 16] {
    &self.val
  }
}

/// 原版 DEFINE_SIMD_TYPE_SPECIFIC_METHODS 宏展开:
/// from_<name>(数组) 构造 / splat_<name>(标量) 铺满 / to_<name>() 取出
macro_rules! simd_type_methods {
  ($c:ty, $w:literal, $lanes:literal, $splat:ident, $from:ident, $to:ident, $l2b:ident, $b2l:ident) => {
    impl Simd128 {
      /// 原版 explicit constexpr Simd128(const sType val) — bit_cast
      pub const fn $from(val: [$c; $lanes]) -> Self {
        let mut bytes = [0u8; 16];
        let mut i = 0;
        while i < $lanes {
          let b = $l2b(val[i]);
          let mut j = 0;
          while j < $w {
            bytes[i * $w + j] = b[j];
            j += 1;
          }
          i += 1;
        }
        Self { val: bytes }
      }

      /// 原版 static constexpr Simd128 Splat(cType value) — std::fill_n 铺满
      pub const fn $splat(value: $c) -> Self {
        Self::$from([value; $lanes])
      }

      /// 原版 constexpr sType to_##name() const — bit_cast
      pub const fn $to(&self) -> [$c; $lanes] {
        let mut out = [$b2l([0u8; $w]); $lanes];
        let mut i = 0;
        while i < $lanes {
          let mut b = [0u8; $w];
          let mut j = 0;
          while j < $w {
            b[j] = self.val[i * $w + j];
            j += 1;
          }
          out[i] = $b2l(b);
          i += 1;
        }
        out
      }
    }
  };
}

const fn f64_l2b(v: f64) -> [u8; 8] {
  v.to_bits().to_ne_bytes()
}
const fn f64_b2l(b: [u8; 8]) -> f64 {
  f64::from_bits(u64::from_ne_bytes(b))
}
const fn f32_l2b(v: f32) -> [u8; 4] {
  v.to_bits().to_ne_bytes()
}
const fn f32_b2l(b: [u8; 4]) -> f32 {
  f32::from_bits(u32::from_ne_bytes(b))
}
const fn i64_l2b(v: i64) -> [u8; 8] {
  v.to_ne_bytes()
}
const fn i64_b2l(b: [u8; 8]) -> i64 {
  i64::from_ne_bytes(b)
}
const fn i32_l2b(v: i32) -> [u8; 4] {
  v.to_ne_bytes()
}
const fn i32_b2l(b: [u8; 4]) -> i32 {
  i32::from_ne_bytes(b)
}
const fn i16_l2b(v: i16) -> [u8; 2] {
  v.to_ne_bytes()
}
const fn i16_b2l(b: [u8; 2]) -> i16 {
  i16::from_ne_bytes(b)
}
const fn i8_l2b(v: i8) -> [u8; 1] {
  [v as u8]
}
const fn i8_b2l(b: [u8; 1]) -> i8 {
  b[0] as i8
}

simd_type_methods!(
  f64,
  8,
  2,
  splat_f64x2,
  from_f64x2,
  to_f64x2,
  f64_l2b,
  f64_b2l
);
simd_type_methods!(
  f32,
  4,
  4,
  splat_f32x4,
  from_f32x4,
  to_f32x4,
  f32_l2b,
  f32_b2l
);
simd_type_methods!(
  i64,
  8,
  2,
  splat_i64x2,
  from_i64x2,
  to_i64x2,
  i64_l2b,
  i64_b2l
);
simd_type_methods!(
  i32,
  4,
  4,
  splat_i32x4,
  from_i32x4,
  to_i32x4,
  i32_l2b,
  i32_b2l
);
simd_type_methods!(
  i16,
  2,
  8,
  splat_i16x8,
  from_i16x8,
  to_i16x8,
  i16_l2b,
  i16_b2l
);
simd_type_methods!(i8, 1, 16, splat_i8x16, from_i8x16, to_i8x16, i8_l2b, i8_b2l);
