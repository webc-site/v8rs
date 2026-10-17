//! v8/src/base/overflowing-math.h 的 1:1 转写
//! 原版宏 OP_WITH_WRAPAROUND / ASSERT_SIGNED_INTEGER_TYPE 机械展开为逐类型实现

/// AddWithWraparound (i16/i32/i64): 环绕加法, 无 UB
#[inline]
pub const fn add_with_wraparound_i16(a: i16, b: i16) -> i16 {
  a.wrapping_add(b)
}

/// AddWithWraparound (i32)
#[inline]
pub const fn add_with_wraparound_i32(a: i32, b: i32) -> i32 {
  a.wrapping_add(b)
}

/// AddWithWraparound (i64)
#[inline]
pub const fn add_with_wraparound_i64(a: i64, b: i64) -> i64 {
  a.wrapping_add(b)
}

/// SubWithWraparound (i16)
#[inline]
pub const fn sub_with_wraparound_i16(a: i16, b: i16) -> i16 {
  a.wrapping_sub(b)
}

/// SubWithWraparound (i32)
#[inline]
pub const fn sub_with_wraparound_i32(a: i32, b: i32) -> i32 {
  a.wrapping_sub(b)
}

/// SubWithWraparound (i64)
#[inline]
pub const fn sub_with_wraparound_i64(a: i64, b: i64) -> i64 {
  a.wrapping_sub(b)
}

/// MulWithWraparound (i16): 原版特化 — C++ 隐式提升规则的特例路径
#[inline]
pub const fn mul_with_wraparound_i16(a: i16, b: i16) -> i16 {
  let a_unsigned = a as u32;
  let b_unsigned = b as u32;
  let result = a_unsigned.wrapping_mul(b_unsigned);
  result as u16 as i16
}

/// MulWithWraparound (i32)
#[inline]
pub const fn mul_with_wraparound_i32(a: i32, b: i32) -> i32 {
  a.wrapping_mul(b)
}

/// MulWithWraparound (i64)
#[inline]
pub const fn mul_with_wraparound_i64(a: i64, b: i64) -> i64 {
  a.wrapping_mul(b)
}

/// NegateWithWraparound (i16): min 值返回自身
#[inline]
pub const fn negate_with_wraparound_i16(a: i16) -> i16 {
  if a == i16::MIN { a } else { -a }
}

/// NegateWithWraparound (i32)
#[inline]
pub const fn negate_with_wraparound_i32(a: i32) -> i32 {
  if a == i32::MIN { a } else { -a }
}

/// NegateWithWraparound (i64)
#[inline]
pub const fn negate_with_wraparound_i64(a: i64) -> i64 {
  if a == i64::MIN { a } else { -a }
}

/// AddWithWraparound (i8)
#[inline]
pub const fn add_with_wraparound_i8(a: i8, b: i8) -> i8 {
  a.wrapping_add(b)
}

/// SubWithWraparound (i8)
#[inline]
pub const fn sub_with_wraparound_i8(a: i8, b: i8) -> i8 {
  a.wrapping_sub(b)
}

/// MulWithWraparound (i8)
#[inline]
pub const fn mul_with_wraparound_i8(a: i8, b: i8) -> i8 {
  a.wrapping_mul(b)
}

/// NegateWithWraparound (i8)
#[inline]
pub const fn negate_with_wraparound_i8(a: i8) -> i8 {
  if a == i8::MIN { a } else { -a }
}

/// ShlWithWraparound (i8)
#[inline]
pub const fn shl_with_wraparound_i8(a: i8, b: i8) -> i8 {
  let k_mask: u8 = (i8::BITS - 1) as u8;
  ((a as u8) << ((b as u8) & k_mask)) as i8
}

/// ShlWithWraparound (i16)
#[inline]
pub const fn shl_with_wraparound_i16(a: i16, b: i16) -> i16 {
  let k_mask: u16 = (i16::BITS - 1) as u16;
  ((a as u16) << ((b as u16) & k_mask)) as i16
}

/// ShlWithWraparound (i32): 移位量按位宽取掩码 (b & (8*size-1))
#[inline]
pub const fn shl_with_wraparound_i32(a: i32, b: i32) -> i32 {
  let k_mask: u32 = i32::BITS - 1;
  ((a as u32) << ((b as u32) & k_mask)) as i32
}

/// ShlWithWraparound (i64)
#[inline]
pub const fn shl_with_wraparound_i64(a: i64, b: i64) -> i64 {
  let k_mask: u64 = (i64::BITS - 1) as u64;
  ((a as u64) << ((b as u64) & k_mask)) as i64
}

// ---------------------------------------------------------------------------
// 浮点除法 (wasm 语义): 避免 C++ UB 的 0 除处理
// ---------------------------------------------------------------------------

/// Divide (f32): y==0 时按 IEEE 语义返回 NaN/±inf
#[inline]
pub fn divide_f32(x: f32, y: f32) -> f32 {
  if y != 0.0 {
    return x / y;
  }
  if x == 0.0 || x.is_nan() {
    return f32::NAN;
  }
  // (x >= 0) == (signbit(y) == 0): y 为 ±0 时符号由 signbit 决定
  let same_sign = (x >= 0.0) == !y.is_sign_negative();
  if same_sign {
    f32::INFINITY
  } else {
    f32::NEG_INFINITY
  }
}

/// Divide (f64)
#[inline]
pub fn divide_f64(x: f64, y: f64) -> f64 {
  if y != 0.0 {
    return x / y;
  }
  if x == 0.0 || x.is_nan() {
    return f64::NAN;
  }
  let same_sign = (x >= 0.0) == !y.is_sign_negative();
  if same_sign {
    f64::INFINITY
  } else {
    f64::NEG_INFINITY
  }
}

/// Recip (f32)
#[inline]
pub fn recip_f32(a: f32) -> f32 {
  divide_f32(1.0, a)
}

/// Recip (f64)
#[inline]
pub fn recip_f64(a: f64) -> f64 {
  divide_f64(1.0, a)
}

/// RecipSqrt (f32): sqrt 在 a<0 时为 NaN, 与原版 std::sqrt 行为一致;
/// a==0 时按 signbit 返回 ±inf
#[inline]
pub fn recip_sqrt_f32(a: f32) -> f32 {
  if a != 0.0 {
    return 1.0 / a.sqrt();
  }
  // 此处 a 必为 ±0: 原版用 signbit 区分
  if a.is_sign_negative() {
    f32::NEG_INFINITY
  } else {
    f32::INFINITY
  }
}

/// RecipSqrt (f64)
#[inline]
pub fn recip_sqrt_f64(a: f64) -> f64 {
  if a != 0.0 {
    return 1.0 / a.sqrt();
  }
  // 此处 a 必为 ±0: 原版用 signbit 区分
  if a.is_sign_negative() {
    f64::NEG_INFINITY
  } else {
    f64::INFINITY
  }
}
