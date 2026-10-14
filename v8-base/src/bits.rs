//! v8/src/base/bits.h + bits.cc 的 1:1 转写 — 位运算工具集
//! 原版命名空间 `v8::base::bits` 对应本模块 `crate::bits`
//! 泛型模板函数按具体宽度落地 (Rust 无模板); 0 输入的 UB 边界显式定义:
//! - count_leading_zeros(0) / count_trailing_zeros(0) 返回位宽 (与原版一致)
//! - count_trailing_zeros_nonzero(0) / which_power_of_two(非2的幂) 仅 debug_assert
//!
//! 附注: 本快照 bits.h 无 Align/FloorToPowerOf2/RoundToPowerOf2/Log2 (旧版 macros.h
//! 提供, 现已上游移除); 为满足下游对齐/取整需求以同名辅助函数保留语义。

use std::mem::size_of;

/// 通用位运算 trait: 对应原版各模板函数, 对无符号与有符号整数实现
pub trait BitOps: Copy + Sized {
  /// 位宽 (8 * sizeof)
  const BITS: u32;

  /// CountPopulation: 置位个数
  fn count_population(self) -> u32;
  /// CountLeadingZeros: 最高位 1 之前的 0 的个数; 0 输入返回 BITS
  fn count_leading_zeros(self) -> u32;
  /// CountTrailingZeros: 最低位 1 之后的 0 的个数; 0 输入返回 BITS
  fn count_trailing_zeros(self) -> u32;
  /// ReverseBits: 位序反转
  fn reverse_bits(self) -> Self;
  /// ReverseBytes: 字节序反转
  fn reverse_bytes(self) -> Self;
  /// IsPowerOfTwo
  fn is_power_of_two(self) -> bool;
  /// ClearLsb: Brian Kernighan 方法清除最低位 1
  fn clear_lsb(self) -> Self;
}

/// 无符号整数附加运算
pub trait BitOpsUnsigned: BitOps {
  /// CountTrailingZerosNonZero: 要求非 0 (仅 debug 断言)
  fn count_trailing_zeros_nonzero(self) -> u32;
  /// WhichPowerOfTwo: 输入必须是 2 的幂
  fn which_power_of_two(self) -> u32;
  /// BitWidth: 有效位数 (digits - CountLeadingZeros)
  fn bit_width(self) -> u32;
  /// RotateRight32/64 泛化版
  fn rotate_right(self, shift: u32) -> Self;
  /// RotateLeft32/64 泛化版
  fn rotate_left(self, shift: u32) -> Self;
  /// FloorToPowerOf2: 不大于 value 的最大 2 的幂 (value==0 返回 0)
  fn floor_to_power_of_two(self) -> Self;
  /// RoundToPowerOf2: 不小于 value 的最小 2 的幂 (value==0 返回 0)
  fn round_to_power_of_two(self) -> Self;
  /// RoundUpToPowerOf2 别名: 等同 round_to_power_of_two
  #[inline]
  fn round_up_to_power_of_two(self) -> Self {
    self.round_to_power_of_two()
  }
  /// BitAt: 仅第 index 位为 1 的掩码 (对应 T{1} << index; index >= BITS 返回 0)
  fn bit_at(index: u32) -> Self;
  /// 清除第 index 位: 对应原版 `bits &= ~(T{1} << pos)` (BitsIterator::operator++)
  fn clear_bit(self, index: u32) -> Self;
}

macro_rules! impl_unsigned_ops {
  ($t:ty) => {
    impl BitOps for $t {
      const BITS: u32 = size_of::<$t>() as u32 * 8;
      #[inline]
      fn count_population(self) -> u32 {
        self.count_ones()
      }
      #[inline]
      fn count_leading_zeros(self) -> u32 {
        self.leading_zeros()
      }
      #[inline]
      fn count_trailing_zeros(self) -> u32 {
        self.trailing_zeros()
      }
      #[inline]
      fn reverse_bits(self) -> Self {
        self.reverse_bits()
      }
      #[inline]
      fn reverse_bytes(self) -> Self {
        self.swap_bytes()
      }
      #[inline]
      fn is_power_of_two(self) -> bool {
        self > 0 && (self & (self - 1)) == 0
      }
      #[inline]
      fn clear_lsb(self) -> Self {
        self & (self - 1)
      }
    }
    impl BitOpsUnsigned for $t {
      #[inline]
      fn count_trailing_zeros_nonzero(self) -> u32 {
        debug_assert_ne!(self, 0);
        self.trailing_zeros()
      }
      #[inline]
      fn which_power_of_two(self) -> u32 {
        debug_assert!(self.is_power_of_two());
        self.trailing_zeros()
      }
      #[inline]
      fn bit_width(self) -> u32 {
        Self::BITS - self.leading_zeros()
      }
      #[inline]
      fn rotate_right(self, shift: u32) -> Self {
        // std 内建 rotate 对任意 shift 等价于 shift % BITS, 与原版前置条件语义一致
        self.rotate_right(shift)
      }
      #[inline]
      fn rotate_left(self, shift: u32) -> Self {
        self.rotate_left(shift)
      }
      #[inline]
      fn bit_at(index: u32) -> Self {
        // 0 输入 UB 边界: index >= BITS 时显式返回 0 掩码
        if index >= Self::BITS { 0 } else { 1 << index }
      }
      #[inline]
      fn clear_bit(self, index: u32) -> Self {
        self & !Self::bit_at(index)
      }
      #[inline]
      fn floor_to_power_of_two(self) -> Self {
        if self == 0 {
          return 0;
        }
        1 << (Self::BITS - 1 - self.leading_zeros())
      }
      #[inline]
      fn round_to_power_of_two(self) -> Self {
        if self <= 1 {
          return self;
        }
        let exp = Self::BITS - (self - 1).leading_zeros();
        // 超出位宽时饱和到最大 2 的幂 (显式定义原版 CHECK 失败的边界)
        if exp >= Self::BITS {
          return 1 << (Self::BITS - 1);
        }
        1 << exp
      }
    }
  };
}

impl_unsigned_ops!(u8);
impl_unsigned_ops!(u16);
impl_unsigned_ops!(u32);
impl_unsigned_ops!(u64);
// 原版约束 is_unsigned_v<T> && sizeof(T) <= 8 (bits.h:27/101) → 指针宽度同属全集
impl_unsigned_ops!(usize);
// IsPowerOfTwo/WhichPowerOfTwo/ClearLsb 仅要求 is_integral (bits.h:188/196/237),
// 128 位无符号亦在原版可用面内
impl_unsigned_ops!(u128);

macro_rules! impl_signed_ops {
  ($t:ty, $u:ty) => {
    impl BitOps for $t {
      const BITS: u32 = size_of::<$t>() as u32 * 8;
      #[inline]
      fn count_population(self) -> u32 {
        (self as $u).count_ones()
      }
      #[inline]
      fn count_leading_zeros(self) -> u32 {
        (self as $u).leading_zeros()
      }
      #[inline]
      fn count_trailing_zeros(self) -> u32 {
        (self as $u).trailing_zeros()
      }
      #[inline]
      fn reverse_bits(self) -> Self {
        (self as $u).reverse_bits() as $t
      }
      #[inline]
      fn reverse_bytes(self) -> Self {
        (self as $u).swap_bytes() as $t
      }
      #[inline]
      fn is_power_of_two(self) -> bool {
        self > 0 && (self & (self - 1)) == 0
      }
      #[inline]
      fn clear_lsb(self) -> Self {
        self & (self - 1)
      }
    }
  };
}

impl_signed_ops!(i8, u8);
impl_signed_ops!(i16, u16);
impl_signed_ops!(i32, u32);
impl_signed_ops!(i64, u64);
// 与 Unsigned/Signed 转换全集 (bits.h:86/91 make_signed_t/make_unsigned_t) 对齐:
// 指针宽度与 128 位有符号同属 is_integral 面
impl_signed_ops!(isize, usize);
impl_signed_ops!(i128, u128);

/// bits::Unsigned: 有符号 → 无符号 (对应原版 `Unsigned(T value)`)
pub trait UnsignedConv {
  type Unsigned: BitOpsUnsigned;
  fn unsigned(self) -> Self::Unsigned;
}

/// bits::Signed: 无符号 → 有符号 (对应原版 `Signed(T value)`)
pub trait SignedConv {
  type Signed: BitOps;
  fn signed(self) -> Self::Signed;
}

macro_rules! impl_sign_conv {
  ($s:ty, $u:ty) => {
    impl UnsignedConv for $s {
      type Unsigned = $u;
      #[inline]
      fn unsigned(self) -> $u {
        self as $u
      }
    }
    impl SignedConv for $u {
      type Signed = $s;
      #[inline]
      fn signed(self) -> $s {
        self as $s
      }
    }
  };
}

impl_sign_conv!(i8, u8);
impl_sign_conv!(i16, u16);
impl_sign_conv!(i32, u32);
impl_sign_conv!(i64, u64);
impl_sign_conv!(i128, u128);
impl_sign_conv!(isize, usize);

/// 自由函数入口, 对应原版 `bits::Unsigned(value)`
#[inline]
pub fn unsigned<T: UnsignedConv>(value: T) -> T::Unsigned {
  value.unsigned()
}

/// 自由函数入口, 对应原版 `bits::Signed(value)`
#[inline]
pub fn signed<T: SignedConv>(value: T) -> T::Signed {
  value.signed()
}

// ---------------------------------------------------------------------------
// 独立自由函数: 对应原版非模板/固定宽度函数
// ---------------------------------------------------------------------------

/// CountLeadingZeros32
#[inline]
pub const fn count_leading_zeros32(value: u32) -> u32 {
  value.leading_zeros()
}

/// CountLeadingZeros64
#[inline]
pub const fn count_leading_zeros64(value: u64) -> u32 {
  value.leading_zeros()
}

/// CountTrailingZeros32
#[inline]
pub const fn count_trailing_zeros32(value: u32) -> u32 {
  value.trailing_zeros()
}

/// CountTrailingZeros64
#[inline]
pub const fn count_trailing_zeros64(value: u64) -> u32 {
  value.trailing_zeros()
}

/// WhichPowerOfTwo 泛型入口 (原版 requires is_integral, 有/无符号皆可):
/// 输入必须是 2 的幂, 非 2 的幂仅 debug_assert (原版 DCHECK 语义)
#[inline]
pub fn which_power_of_two<T: BitOps>(value: T) -> u32 {
  debug_assert!(value.is_power_of_two());
  value.count_trailing_zeros()
}

/// CountLeadingSignBits: 正数的前导 0 位数, 负数的前导 1 位数
#[inline]
pub const fn count_leading_sign_bits32(value: i32) -> u32 {
  if value < 0 {
    (!(value as u32)).leading_zeros()
  } else {
    (value as u32).leading_zeros()
  }
}

/// CountLeadingSignBits (64 位)
#[inline]
pub const fn count_leading_sign_bits64(value: i64) -> u32 {
  if value < 0 {
    (!(value as u64)).leading_zeros()
  } else {
    (value as u64).leading_zeros()
  }
}

/// RotateRight32 (前置条件 0 <= shift < 32)
#[inline]
pub const fn rotate_right32(value: u32, shift: u32) -> u32 {
  (value >> shift) | (value << ((32 - shift) & 31))
}

/// RotateLeft32 (前置条件 0 <= shift < 32)
#[inline]
pub const fn rotate_left32(value: u32, shift: u32) -> u32 {
  (value << shift) | (value >> ((32 - shift) & 31))
}

/// RotateRight64 (前置条件 0 <= shift < 64)
#[inline]
pub const fn rotate_right64(value: u64, shift: u32) -> u64 {
  (value >> shift) | (value << ((64 - shift) & 63))
}

/// RotateLeft64 (前置条件 0 <= shift < 64)
#[inline]
pub const fn rotate_left64(value: u64, shift: u32) -> u64 {
  (value << shift) | (value >> ((64 - shift) & 63))
}

/// SignedAddOverflow32: 返回 (是否溢出, 环绕结果)
#[inline]
pub fn signed_add_overflow32(lhs: i32, rhs: i32) -> (bool, i32) {
  let (res, overflow) = lhs.overflowing_add(rhs);
  (overflow, res)
}

/// SignedSubOverflow32
#[inline]
pub fn signed_sub_overflow32(lhs: i32, rhs: i32) -> (bool, i32) {
  let (res, overflow) = lhs.overflowing_sub(rhs);
  (overflow, res)
}

/// SignedMulOverflow32
#[inline]
pub fn signed_mul_overflow32(lhs: i32, rhs: i32) -> (bool, i32) {
  let (res, overflow) = lhs.overflowing_mul(rhs);
  (overflow, res)
}

/// SignedAddOverflow64
#[inline]
pub fn signed_add_overflow64(lhs: i64, rhs: i64) -> (bool, i64) {
  let (res, overflow) = lhs.overflowing_add(rhs);
  (overflow, res)
}

/// SignedSubOverflow64
#[inline]
pub fn signed_sub_overflow64(lhs: i64, rhs: i64) -> (bool, i64) {
  let (res, overflow) = lhs.overflowing_sub(rhs);
  (overflow, res)
}

/// SignedMulOverflow64
#[inline]
pub fn signed_mul_overflow64(lhs: i64, rhs: i64) -> (bool, i64) {
  let (res, overflow) = lhs.overflowing_mul(rhs);
  (overflow, res)
}

/// UnsignedAddOverflow32
#[inline]
pub fn unsigned_add_overflow32(lhs: u32, rhs: u32) -> (bool, u32) {
  let (res, overflow) = lhs.overflowing_add(rhs);
  (overflow, res)
}

/// SignedMulHigh32: 有符号 32x32 乘积的高 32 位
#[inline]
pub fn signed_mul_high32(lhs: i32, rhs: i32) -> i32 {
  let value = lhs as i64 * rhs as i64;
  (value >> 32) as u32 as i32
}

/// UnsignedMulHigh32: 无符号 32x32 乘积的高 32 位
#[inline]
pub fn unsigned_mul_high32(lhs: u32, rhs: u32) -> u32 {
  let value = lhs as u64 * rhs as u64;
  (value >> 32) as u32
}

/// SignedMulHigh64: Hacker's Delight 8.2 节算法 (算术右移, 中间量按环绕语义)
#[inline]
pub fn signed_mul_high64(u: i64, v: i64) -> i64 {
  let u0 = (u as u64) & 0xFFFF_FFFF;
  let u1 = u >> 32;
  let v0 = (v as u64) & 0xFFFF_FFFF;
  let v1 = v >> 32;

  let w0 = u0.wrapping_mul(v0);
  let t = u1.wrapping_mul(v0 as i64).wrapping_add((w0 >> 32) as i64);
  let w1 = t & 0xFFFF_FFFF;
  let w2 = t >> 32;
  // 原版 `u0 * v1 + w1`: 混合符号乘加按 uint64 回绕 (debug 下非 wrapping 会 panic)
  let w1 = u0.wrapping_mul(v1 as u64).wrapping_add(w1 as u64) as i64;

  u1.wrapping_mul(v1).wrapping_add(w2).wrapping_add(w1 >> 32)
}

/// UnsignedMulHigh64: Hacker's Delight 8.2 节算法 (无符号环绕)
#[inline]
pub fn unsigned_mul_high64(u: u64, v: u64) -> u64 {
  let u0 = u & 0xFFFF_FFFF;
  let u1 = u >> 32;
  let v0 = v & 0xFFFF_FFFF;
  let v1 = v >> 32;

  let w0 = u0.wrapping_mul(v0);
  let t = u1.wrapping_mul(v0).wrapping_add(w0 >> 32);
  let w1 = t & 0xFFFF_FFFF;
  let w2 = t >> 32;
  let w1 = u0.wrapping_mul(v1).wrapping_add(w1);

  u1.wrapping_mul(v1).wrapping_add(w2).wrapping_add(w1 >> 32)
}

/// SignedMulHighAndAdd32: 高 32 位加上 acc (环绕加法)
#[inline]
pub fn signed_mul_high_and_add32(lhs: i32, rhs: i32, acc: i32) -> i32 {
  (acc as u32).wrapping_add(signed_mul_high32(lhs, rhs) as u32) as i32
}

/// SignedDiv32: rhs==0 返回 0; lhs==minint && rhs==-1 返回 minint
#[inline]
pub fn signed_div32(lhs: i32, rhs: i32) -> i32 {
  if rhs == 0 {
    return 0;
  }
  if rhs == -1 {
    return if lhs == i32::MIN { lhs } else { -lhs };
  }
  lhs / rhs
}

/// SignedDiv64
#[inline]
pub fn signed_div64(lhs: i64, rhs: i64) -> i64 {
  if rhs == 0 {
    return 0;
  }
  if rhs == -1 {
    return if lhs == i64::MIN { lhs } else { -lhs };
  }
  lhs / rhs
}

/// SignedMod32: rhs==0 或 rhs==-1 返回 0
#[inline]
pub fn signed_mod32(lhs: i32, rhs: i32) -> i32 {
  if rhs == 0 || rhs == -1 {
    return 0;
  }
  lhs % rhs
}

/// SignedMod64
#[inline]
pub fn signed_mod64(lhs: i64, rhs: i64) -> i64 {
  if rhs == 0 || rhs == -1 {
    return 0;
  }
  lhs % rhs
}

/// UnsignedDiv32: rhs==0 返回 0
#[inline]
pub const fn unsigned_div32(lhs: u32, rhs: u32) -> u32 {
  match lhs.checked_div(rhs) {
    Some(v) => v,
    // rhs==0: 原版定义返回 0
    None => 0,
  }
}

/// UnsignedDiv64
#[inline]
pub const fn unsigned_div64(lhs: u64, rhs: u64) -> u64 {
  match lhs.checked_div(rhs) {
    Some(v) => v,
    // rhs==0: 原版定义返回 0
    None => 0,
  }
}

/// UnsignedMod32
#[inline]
pub const fn unsigned_mod32(lhs: u32, rhs: u32) -> u32 {
  match lhs.checked_rem(rhs) {
    Some(v) => v,
    // rhs==0: 原版定义返回 0
    None => 0,
  }
}

/// UnsignedMod64
#[inline]
pub const fn unsigned_mod64(lhs: u64, rhs: u64) -> u64 {
  match lhs.checked_rem(rhs) {
    Some(v) => v,
    // rhs==0: 原版定义返回 0
    None => 0,
  }
}

/// WraparoundAdd32: 无 UB 的环绕加法
#[inline]
pub const fn wraparound_add32(lhs: i32, rhs: i32) -> i32 {
  (lhs as u32).wrapping_add(rhs as u32) as i32
}

/// WraparoundNeg32
#[inline]
pub const fn wraparound_neg32(x: i32) -> i32 {
  (x as u32).wrapping_neg() as i32
}

/// ByteReverse16
#[inline]
pub const fn byte_reverse16(value: u16) -> u16 {
  value.swap_bytes()
}

/// ByteReverse32
#[inline]
pub const fn byte_reverse32(value: u32) -> u32 {
  value.swap_bytes()
}

/// ByteReverse64
#[inline]
pub const fn byte_reverse64(value: u64) -> u64 {
  value.swap_bytes()
}

/// SignedSaturatedAdd64: 饱和加法
#[inline]
pub fn signed_saturated_add64(lhs: i64, rhs: i64) -> i64 {
  // 下溢: lhs + rhs < min
  if rhs < 0 && lhs < i64::MIN - rhs {
    return i64::MIN;
  }
  // 上溢: lhs + rhs > max
  if rhs >= 0 && lhs > i64::MAX - rhs {
    return i64::MAX;
  }
  lhs + rhs
}

/// SignedSaturatedSub64: 饱和减法
#[inline]
pub fn signed_saturated_sub64(lhs: i64, rhs: i64) -> i64 {
  if rhs > 0 && lhs < i64::MIN + rhs {
    return i64::MIN;
  }
  if rhs <= 0 && lhs > i64::MAX + rhs {
    return i64::MAX;
  }
  lhs - rhs
}

/// BitWidth32: std::numeric_limits<T>::digits - CountLeadingZeros(x)
#[inline]
pub const fn bit_width32(x: u32) -> u32 {
  32 - x.leading_zeros()
}

/// BitWidth64
#[inline]
pub const fn bit_width64(x: u64) -> u32 {
  64 - x.leading_zeros()
}

/// Align: value 向上对齐到 alignment (必须为 2 的幂) — 旧版 macros.h 语义
#[inline]
pub const fn align(value: usize, alignment: usize) -> usize {
  debug_assert!(alignment > 0 && (alignment & (alignment - 1)) == 0);
  (value.wrapping_add(alignment - 1)) & !(alignment - 1)
}

/// Log2: value 必须是 2 的幂, 返回其指数 — 旧版 macros.h 语义
#[inline]
pub const fn log2(value: usize) -> u32 {
  debug_assert!(value.is_power_of_two());
  value.trailing_zeros()
}

/// RoundUpToPowerOfTwo 泛型入口: 不小于 value 的最小 2 的幂 (value==0 返回 0)
#[inline]
pub fn round_up_to_power_of_two<T: BitOpsUnsigned>(value: T) -> T {
  value.round_to_power_of_two()
}

/// RoundToPowerOfTwo 泛型入口: 不小于 value 的最小 2 的幂 (value==0 返回 0)
#[inline]
pub fn round_to_power_of_two<T: BitOpsUnsigned>(value: T) -> T {
  value.round_to_power_of_two()
}

/// FloorToPowerOfTwo 泛型入口: 不大于 value 的最大 2 的幂 (value==0 返回 0)
#[inline]
pub fn floor_to_power_of_two<T: BitOpsUnsigned>(value: T) -> T {
  value.floor_to_power_of_two()
}
