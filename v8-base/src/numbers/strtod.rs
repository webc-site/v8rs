//! 原版 v8/src/base/numbers/strtod.cc/h — 十进制数字串 → double (Grisu 快速 + Bignum 兜底)
//!
//! 原版对 IA32/模拟器直接禁用快速路径; 目标平台为 macOS/aarch64 与 Linux/x86_64,
//! 机械展开后仅保留非 IA32 分支.

use super::{
  bignum::Bignum,
  cached_powers::{
    DECIMAL_EXPONENT_DISTANCE, MAX_DECIMAL_EXPONENT, MIN_DECIMAL_EXPONENT, PowersOfTenCache,
  },
};
use crate::{immediate_crash::debug_unreachable, numbers::diy_fp::DiyFp};

// 2^53 = 9007199254740992: 至多 15 位十进制整数可精确进入 double
const MAX_EXACT_DOUBLE_INTEGER_DECIMAL_DIGITS: usize = 15;
// 2^64 = 18446744073709551616 > 10^19
const MAX_UINT64_DECIMAL_DIGITS: i32 = 19;

// 10^309 以上记为 +inf; 10^-324 以下记为 0 (2.5e-324 仍读作最小正规数)
const MAX_DECIMAL_POWER: i32 = 309;
const MIN_DECIMAL_POWER: i32 = -324;

const MAX_UINT64: u64 = 0xFFFF_FFFF_FFFF_FFFF;

/// 可精确表示的 10^n (n=0..22)
static EXACT_POWERS_OF_TEN: [f64; 23] = [
  1.0,
  10.0,
  100.0,
  1000.0,
  10_000.0,
  100_000.0,
  1_000_000.0,
  10_000_000.0,
  100_000_000.0,
  1_000_000_000.0,
  10_000_000_000.0,
  100_000_000_000.0,
  1_000_000_000_000.0,
  10_000_000_000_000.0,
  100_000_000_000_000.0,
  1_000_000_000_000_000.0,
  10_000_000_000_000_000.0,
  100_000_000_000_000_000.0,
  1_000_000_000_000_000_000.0,
  10_000_000_000_000_000_000.0,
  100_000_000_000_000_000_000.0,
  1_000_000_000_000_000_000_000.0,
  // 10^22 = 0x21E19E0C9BAB2400000 = 0x878678326EAC9 × 2^22
  10_000_000_000_000_000_000_000.0,
];
const EXACT_POWERS_OF_TEN_SIZE: usize = EXACT_POWERS_OF_TEN.len();

/// 十进制表示的最大有效位数 (实际 772, 放宽到 780)
const MAX_SIGNIFICANT_DECIMAL_DIGITS: usize = 780;

/// 去除前导零, 返回起始下标
fn trim_leading_zeros(buffer: &[u8]) -> usize {
  for (i, &c) in buffer.iter().enumerate() {
    if c != b'0' {
      return i;
    }
  }
  buffer.len()
}

/// 去除末尾零, 返回截断后的切片
fn trim_trailing_zeros(buffer: &[u8]) -> &[u8] {
  let mut i = buffer.len();
  while i > 0 {
    if buffer[i - 1] != b'0' {
      return &buffer[..i];
    }
    i -= 1;
  }
  &buffer[..0]
}

/// 截断到最大有效位: 保留 779 位并以 '1' 结尾保证正确舍入
fn trim_to_max_significant_digits(
  buffer: &[u8],
  exponent: i32,
  significant_buffer: &mut [u8],
) -> i32 {
  significant_buffer[..MAX_SIGNIFICANT_DECIMAL_DIGITS - 1]
    .copy_from_slice(&buffer[..MAX_SIGNIFICANT_DECIMAL_DIGITS - 1]);
  // 输入已去尾零: 末位必非 '0'
  debug_assert_ne!(buffer[buffer.len() - 1], b'0');
  // 末位置非零即可保证正确舍入
  significant_buffer[MAX_SIGNIFICANT_DECIMAL_DIGITS - 1] = b'1';
  exponent + (buffer.len() as i32 - MAX_SIGNIFICANT_DECIMAL_DIGITS as i32)
}

/// 按 uint64 容量读取数字; 返回 (值, 读取位数)
fn read_uint64(buffer: &[u8]) -> (u64, usize) {
  let mut result = 0u64;
  let mut i = 0;
  while i < buffer.len() && result <= (MAX_UINT64 / 10 - 1) {
    let digit = buffer[i] - b'0';
    debug_assert!(digit <= 9);
    result = 10 * result + u64::from(digit);
    i += 1;
  }
  (result, i)
}

/// 从 buffer 读入 DiyFp (可能舍入, 误差 <= 1/2 ulp)
fn read_diy_fp(buffer: &[u8]) -> (DiyFp, usize) {
  let (significand, read_digits) = read_uint64(buffer);
  if buffer.len() == read_digits {
    (DiyFp::new(significand, 0), 0)
  } else {
    // 舍入 significand
    let mut significand = significand;
    if buffer[read_digits] >= b'5' {
      significand += 1;
    }
    (DiyFp::new(significand, 0), buffer.len() - read_digits)
  }
}

/// 快速路径: 直接浮点乘除即可精确 (返回 None 表示需要更精确算法)
fn double_strtod(trimmed: &[u8], exponent: i32) -> Option<f64> {
  if trimmed.len() <= MAX_EXACT_DOUBLE_INTEGER_DECIMAL_DIGITS {
    let (value, read_digits) = read_uint64(trimmed);
    debug_assert_eq!(read_digits, trimmed.len());
    let result = value as f64;
    let table_size = EXACT_POWERS_OF_TEN_SIZE as i32;
    if exponent < 0 && -exponent < table_size {
      // 10^-exponent 可精确进入 double
      return Some(result / EXACT_POWERS_OF_TEN[-exponent as usize]);
    }
    if 0 <= exponent && exponent < table_size {
      // 10^exponent 可精确进入 double
      return Some(result * EXACT_POWERS_OF_TEN[exponent as usize]);
    }
    let remaining_digits = MAX_EXACT_DOUBLE_INTEGER_DECIMAL_DIGITS as i32 - trimmed.len() as i32;
    if 0 <= exponent && exponent - remaining_digits < table_size {
      // 短输入先乘 10^remaining_digits, 剩余指数亦落在精确表内
      let step1 = result * EXACT_POWERS_OF_TEN[remaining_digits as usize];
      return Some(step1 * EXACT_POWERS_OF_TEN[(exponent - remaining_digits) as usize]);
    }
  }
  None
}

/// 10^exponent 的精确 DiyFp (1 <= exponent < kDecimalExponentDistance)
fn adjustment_power_of_ten(exponent: i32) -> DiyFp {
  use crate::numbers::diy_fp::DiyFp;
  debug_assert!(exponent > 0);
  debug_assert!(exponent < DECIMAL_EXPONENT_DISTANCE);
  // 按十进制距离 8 直接硬编码其余幂
  debug_assert_eq!(DECIMAL_EXPONENT_DISTANCE, 8);
  match exponent {
    1 => DiyFp::new(0xA000_0000_0000_0000, -60),
    2 => DiyFp::new(0xC800_0000_0000_0000, -57),
    3 => DiyFp::new(0xFA00_0000_0000_0000, -54),
    4 => DiyFp::new(0x9C40_0000_0000_0000, -50),
    5 => DiyFp::new(0xC350_0000_0000_0000, -47),
    6 => DiyFp::new(0xF424_0000_0000_0000, -44),
    7 => DiyFp::new(0x9896_8000_0000_0000, -40),
    _ => unsafe { debug_unreachable() },
  }
}

/// Grisu 风格带误差界转换: true 表示结果必为正确 double
fn diy_fp_strtod(buffer: &[u8], exponent: i32) -> (f64, bool) {
  use crate::numbers::{diy_fp::DiyFp, double::Double};
  let (mut input, remaining_decimals) = read_diy_fp(buffer);
  // 丢弃数位后误差 <= .5 ulp; 用公共分母避免分数
  const K_DENOMINATOR_LOG: u32 = 3;
  const K_DENOMINATOR: i64 = 1 << K_DENOMINATOR_LOG;
  let exponent = exponent + remaining_decimals as i32;
  let mut error: i64 = if remaining_decimals == 0 {
    0
  } else {
    K_DENOMINATOR / 2
  };

  let old_e = input.e();
  input.normalize();
  error <<= (old_e - input.e()) as u32;

  debug_assert!(exponent <= MAX_DECIMAL_EXPONENT);
  if exponent < MIN_DECIMAL_EXPONENT {
    return (0.0, true);
  }
  let (cached_power, cached_decimal_exponent) =
    PowersOfTenCache::get_cached_power_for_decimal_exponent(exponent);

  if cached_decimal_exponent != exponent {
    let adjustment_exponent = exponent - cached_decimal_exponent;
    let adjustment_power = adjustment_power_of_ten(adjustment_exponent);
    input.multiply(adjustment_power);
    if MAX_UINT64_DECIMAL_DIGITS - buffer.len() as i32 >= adjustment_exponent {
      // input × 调整幂仍适合 64 位整数
      debug_assert_eq!(DiyFp::SIGNIFICAND_SIZE, 64);
    } else {
      // 调整幂精确: 误差仍只有 0.5
      error += K_DENOMINATOR / 2;
    }
  }

  input.multiply(cached_power);
  // a×b 误差 = error_a + error_b + error_a*error_b/2^64 + 0.5
  // cached_power 误差 < 0.5 ulp ⇒ error_b = 0.5; error_ab 上取整为 1
  let error_b = K_DENOMINATOR / 2;
  let error_ab = if error == 0 { 0 } else { 1 };
  let fixed_error = K_DENOMINATOR / 2;
  error += error_b + error_ab + fixed_error;

  let old_e = input.e();
  input.normalize();
  error <<= (old_e - input.e()) as u32;

  // 检查加减误差是否改变 double 有效位
  let order_of_magnitude = DiyFp::SIGNIFICAND_SIZE + input.e();
  let effective_significand_size =
    Double::significand_size_for_order_of_magnitude(order_of_magnitude);
  let mut precision_digits_count = DiyFp::SIGNIFICAND_SIZE - effective_significand_size;
  if precision_digits_count + K_DENOMINATOR_LOG as i32 >= DiyFp::SIGNIFICAND_SIZE {
    // 仅极小非规格化数触达: 整体右移
    let shift_amount =
      precision_digits_count + K_DENOMINATOR_LOG as i32 - DiyFp::SIGNIFICAND_SIZE + 1;
    input.set_f(input.f() >> shift_amount as u32);
    input.set_e(input.e() + shift_amount);
    // error 丢失 1 位精度, input.f() 丢失 kDenominator 份
    error = (error >> shift_amount as u32) + 1 + K_DENOMINATOR;
    precision_digits_count -= shift_amount;
  }
  debug_assert_eq!(DiyFp::SIGNIFICAND_SIZE, 64);
  debug_assert!(precision_digits_count < 64);
  let one64: u64 = 1;
  let precision_bits_mask = (one64 << precision_digits_count) - 1;
  let mut precision_bits = input.f() & precision_bits_mask;
  let mut half_way = one64 << (precision_digits_count - 1);
  precision_bits = precision_bits.wrapping_mul(K_DENOMINATOR as u64);
  half_way = half_way.wrapping_mul(K_DENOMINATOR as u64);
  let mut rounded_input = DiyFp::new(
    input.f() >> precision_digits_count,
    input.e() + precision_digits_count,
  );
  if precision_bits >= half_way + error as u64 {
    rounded_input.set_f(rounded_input.f() + 1);
  }
  // 若最后位太接近 .5 则不够精确: 返回 false 让调用方兜底 (结果保证为正确值或其下邻)
  let result = Double::from_diy_fp(rounded_input).value();
  if half_way.wrapping_sub(error as u64) < precision_bits
    && precision_bits < half_way + error as u64
  {
    (result, false)
  } else {
    (result, true)
  }
}

/// Bignum 精确兜底: guess 须为正确 double 或其下邻
fn bignum_strtod(buffer: &[u8], exponent: i32, guess: f64) -> f64 {
  use crate::numbers::double::Double;
  if guess == f64::INFINITY {
    return guess;
  }

  let upper_boundary = Double::from_double(guess).upper_boundary();

  debug_assert!(buffer.len() as i32 + exponent <= MAX_DECIMAL_POWER + 1);
  debug_assert!(buffer.len() as i32 + exponent > MIN_DECIMAL_POWER);
  debug_assert!(buffer.len() <= MAX_SIGNIFICANT_DECIMAL_DIGITS);
  // Bignum 容量足够: 指数单独存放, 移位最多少一个 bigit (<64 位); ln(10)约 3.3219
  // (MAX_DECIMAL_POWER+1)*333/100 < MAX_SIGNIFICANT_BITS: 常量性质 (2777 < 3584),
  // 注释保留原版 DCHECK 语义
  let mut input = Bignum::new();
  let mut boundary = Bignum::new();
  input.assign_decimal_string(buffer);
  boundary.assign_uint64(upper_boundary.f());
  if exponent >= 0 {
    input.multiply_by_power_of_ten(exponent);
  } else {
    boundary.multiply_by_power_of_ten(-exponent);
  }
  if upper_boundary.e() > 0 {
    boundary.shift_left(upper_boundary.e());
  } else {
    input.shift_left(-upper_boundary.e());
  }
  let comparison = Bignum::compare(&input, &boundary);
  if comparison < 0 {
    guess
  } else if comparison > 0 {
    Double::from_double(guess).next_double()
  } else if Double::from_double(guess).significand() & 1 == 0 {
    // 舍向偶数
    guess
  } else {
    Double::from_double(guess).next_double()
  }
}

/// 原版 Strtod: buffer 只含 [0-9] 数字, 不以 '0' 开头且非空
pub fn strtod(buffer: &[u8], exponent: i32) -> f64 {
  let left_trimmed_start = trim_leading_zeros(buffer);
  let left_trimmed = &buffer[left_trimmed_start..];
  let trimmed = trim_trailing_zeros(left_trimmed);
  let exponent = exponent + (left_trimmed.len() - trimmed.len()) as i32;
  if trimmed.is_empty() {
    return 0.0;
  }
  if trimmed.len() > MAX_SIGNIFICANT_DECIMAL_DIGITS {
    let mut significant_buffer = [0u8; MAX_SIGNIFICANT_DECIMAL_DIGITS];
    let significant_exponent =
      trim_to_max_significant_digits(trimmed, exponent, &mut significant_buffer);
    return strtod(&significant_buffer, significant_exponent);
  }
  if exponent + trimmed.len() as i32 > MAX_DECIMAL_POWER {
    return f64::INFINITY;
  }
  if exponent + trimmed.len() as i32 <= MIN_DECIMAL_POWER {
    return 0.0;
  }
  // 快速路径优先
  if let Some(result) = double_strtod(trimmed, exponent) {
    return result;
  }
  let (guess, accurate) = diy_fp_strtod(trimmed, exponent);
  if accurate {
    return guess;
  }
  // 与原版一致: 结果可能为下邻, 用 Bignum 校验
  bignum_strtod(trimmed, exponent, guess)
}
