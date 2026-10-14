//! 原版 v8/src/base/numerics/byte_conversions.h + basic_ops_impl.h
//!
//! 原文件假定小端机器; Rust 侧用 from_/to_le/be/ne_bytes 保持位级等价
//! (小端平台上 FromNativeEndian 与原版逐字节一致).

use core::mem::size_of;

/// ByteSwap — 反转整数字节序 (原版 internal::SwapBytes)
#[must_use]
pub fn byte_swap<T: SwapBytes>(value: T) -> T {
  value.swap_bytes_impl()
}

/// 可字节序反转的整数 trait (对应 std::is_integral_v 约束)
pub trait SwapBytes {
  fn swap_bytes_impl(self) -> Self;
}

macro_rules! impl_swap_bytes {
  ($($t:ty),*) => {
    $(impl SwapBytes for $t {
      fn swap_bytes_impl(self) -> Self { self.swap_bytes() }
    })*
  };
}
impl_swap_bytes!(u8, u16, u32, u64, i8, i16, i32, i64);

/// U8FromNativeEndian — 单字节仅拷贝
#[must_use]
pub fn u8_from_native_endian(bytes: &[u8; 1]) -> u8 {
  bytes[0]
}
/// U8FromLittleEndian
#[must_use]
pub fn u8_from_little_endian(bytes: &[u8; 1]) -> u8 {
  bytes[0]
}
/// U8FromBigEndian
#[must_use]
pub fn u8_from_big_endian(bytes: &[u8; 1]) -> u8 {
  bytes[0]
}
/// I8FromNativeEndian
#[must_use]
pub fn i8_from_native_endian(bytes: &[u8; 1]) -> i8 {
  bytes[0] as i8
}
/// I8FromLittleEndian
#[must_use]
pub fn i8_from_little_endian(bytes: &[u8; 1]) -> i8 {
  bytes[0] as i8
}
/// I8FromBigEndian
#[must_use]
pub fn i8_from_big_endian(bytes: &[u8; 1]) -> i8 {
  bytes[0] as i8
}
/// U8ToNativeEndian
#[must_use]
pub const fn u8_to_native_endian(val: u8) -> [u8; 1] {
  [val]
}
/// U8ToLittleEndian
#[must_use]
pub const fn u8_to_little_endian(val: u8) -> [u8; 1] {
  [val]
}
/// U8ToBigEndian
#[must_use]
pub const fn u8_to_big_endian(val: u8) -> [u8; 1] {
  [val]
}
/// I8ToNativeEndian
#[must_use]
pub const fn i8_to_native_endian(val: i8) -> [u8; 1] {
  [val as u8]
}
/// I8ToLittleEndian
#[must_use]
pub const fn i8_to_little_endian(val: i8) -> [u8; 1] {
  [val as u8]
}
/// I8ToBigEndian
#[must_use]
pub const fn i8_to_big_endian(val: i8) -> [u8; 1] {
  [val as u8]
}

/// 多字节整数/浮点的 From/To 系列 (宏批量生成, 语义同原版 memcpy)
macro_rules! gen_endian_functions {
  ($(($from_native:ident, $from_le:ident, $from_be:ident, $to_native:ident, $to_le:ident, $to_be:ident, $t:ty)),* $(,)?) => {
    $(
      #[doc = concat!("From 系列: ", stringify!($t), " 原生端")]
      #[must_use]
      pub fn $from_native(bytes: &[u8; size_of::<$t>()]) -> $t {
        <$t>::from_ne_bytes(*bytes)
      }
      #[doc = concat!("From 系列: ", stringify!($t), " 小端")]
      #[must_use]
      pub fn $from_le(bytes: &[u8; size_of::<$t>()]) -> $t {
        <$t>::from_le_bytes(*bytes)
      }
      #[doc = concat!("From 系列: ", stringify!($t), " 大端")]
      #[must_use]
      pub fn $from_be(bytes: &[u8; size_of::<$t>()]) -> $t {
        <$t>::from_be_bytes(*bytes)
      }
      #[doc = concat!("To 系列: ", stringify!($t), " 原生端")]
      #[must_use]
      pub fn $to_native(val: $t) -> [u8; size_of::<$t>()] {
        val.to_ne_bytes()
      }
      #[doc = concat!("To 系列: ", stringify!($t), " 小端")]
      #[must_use]
      pub fn $to_le(val: $t) -> [u8; size_of::<$t>()] {
        val.to_le_bytes()
      }
      #[doc = concat!("To 系列: ", stringify!($t), " 大端")]
      #[must_use]
      pub fn $to_be(val: $t) -> [u8; size_of::<$t>()] {
        val.to_be_bytes()
      }
    )*
  };
}

gen_endian_functions!(
  (
    u16_from_native_endian,
    u16_from_little_endian,
    u16_from_big_endian,
    u16_to_native_endian,
    u16_to_little_endian,
    u16_to_big_endian,
    u16
  ),
  (
    u32_from_native_endian,
    u32_from_little_endian,
    u32_from_big_endian,
    u32_to_native_endian,
    u32_to_little_endian,
    u32_to_big_endian,
    u32
  ),
  (
    u64_from_native_endian,
    u64_from_little_endian,
    u64_from_big_endian,
    u64_to_native_endian,
    u64_to_little_endian,
    u64_to_big_endian,
    u64
  ),
  (
    i16_from_native_endian,
    i16_from_little_endian,
    i16_from_big_endian,
    i16_to_native_endian,
    i16_to_little_endian,
    i16_to_big_endian,
    i16
  ),
  (
    i32_from_native_endian,
    i32_from_little_endian,
    i32_from_big_endian,
    i32_to_native_endian,
    i32_to_little_endian,
    i32_to_big_endian,
    i32
  ),
  (
    i64_from_native_endian,
    i64_from_little_endian,
    i64_from_big_endian,
    i64_to_native_endian,
    i64_to_little_endian,
    i64_to_big_endian,
    i64
  ),
  (
    float_from_native_endian,
    float_from_little_endian,
    float_from_big_endian,
    float_to_native_endian,
    float_to_little_endian,
    float_to_big_endian,
    f32
  ),
  (
    double_from_native_endian,
    double_from_little_endian,
    double_from_big_endian,
    double_to_native_endian,
    double_to_little_endian,
    double_to_big_endian,
    f64
  ),
);
