//! 原版 v8/src/base/numbers/fixed-dtoa.cc/h — 定点 (toFixed) 快速十进制转换

use super::double::Double;

/// 53 位有效数字 (含隐藏位)
const DOUBLE_SIGNIFICAND_SIZE: i32 = 53;

/// 定长写入 number 的十进制位 (低位补零)
fn fill_digits_32_fixed_length(
  mut number: u32,
  requested_length: i32,
  buffer: &mut [u8],
  length: &mut usize,
) {
  let mut i = requested_length - 1;
  while i >= 0 {
    buffer[*length + i as usize] = b'0' + (number % 10) as u8;
    number /= 10;
    i -= 1;
  }
  *length += requested_length as usize;
}

/// 写入 number 的全部十进制位 (逆序填入后原地翻转)
fn fill_digits_32(mut number: u32, buffer: &mut [u8], length: &mut usize) {
  let mut number_length = 0;
  while number != 0 {
    let digit = number % 10;
    number /= 10;
    buffer[*length + number_length] = b'0' + digit as u8;
    number_length += 1;
  }
  let mut i = *length;
  let mut j = *length + number_length - 1;
  while i < j {
    buffer.swap(i, j);
    i += 1;
    j -= 1;
  }
  *length += number_length;
}

/// u128 版本 UInt128 (原版包装 absl::uint128)
struct UInt128 {
  value: u128,
}

impl UInt128 {
  fn new(high: u64, low: u64) -> Self {
    Self {
      value: (u128::from(high) << 64) | u128::from(low),
    }
  }

  fn multiply(&mut self, multiplicand: u32) {
    self.value *= u128::from(multiplicand);
  }

  fn shift(&mut self, shift_amount: i32) {
    self.value >>= shift_amount;
  }

  /// 返回 *this / 2^power, 并把 *this 更新为 *this mod 2^power
  fn div_mod_power_of_2(&mut self, power: i32) -> i32 {
    let result = self.value >> power;
    self.value -= result << power;
    result as i32
  }

  fn is_zero(&self) -> bool {
    self.value == 0
  }

  fn bit_at(&self, position: i32) -> i32 {
    ((self.value >> position) & 1) as i32
  }
}

fn fill_digits_64_fixed_length(
  mut number: u64,
  _requested_length: i32,
  buffer: &mut [u8],
  length: &mut usize,
) {
  const TEN_7: u64 = 10_000_000;
  // 效率考虑: 拆成 3 个 u32 段分别输出
  let part2 = (number % TEN_7) as u32;
  number /= TEN_7;
  let part1 = (number % TEN_7) as u32;
  let part0 = (number / TEN_7) as u32;

  fill_digits_32_fixed_length(part0, 3, buffer, length);
  fill_digits_32_fixed_length(part1, 7, buffer, length);
  fill_digits_32_fixed_length(part2, 7, buffer, length);
}

fn fill_digits_64(mut number: u64, buffer: &mut [u8], length: &mut usize) {
  const TEN_7: u64 = 10_000_000;
  let part2 = (number % TEN_7) as u32;
  number /= TEN_7;
  let part1 = (number % TEN_7) as u32;
  let part0 = (number / TEN_7) as u32;

  if part0 != 0 {
    fill_digits_32(part0, buffer, length);
    fill_digits_32_fixed_length(part1, 7, buffer, length);
    fill_digits_32_fixed_length(part2, 7, buffer, length);
  } else if part1 != 0 {
    fill_digits_32(part1, buffer, length);
    fill_digits_32_fixed_length(part2, 7, buffer, length);
  } else {
    fill_digits_32(part2, buffer, length);
  }
}

/// 向上舍入: 末位 +1 并处理连九进位 (原版 RoundUp)
fn dtoa_round_up(buffer: &mut [u8], length: &mut usize, decimal_point: &mut i32) {
  // 空 buffer 表示 0
  if *length == 0 {
    buffer[0] = b'1';
    *decimal_point = 1;
    *length = 1;
    return;
  }
  buffer[*length - 1] += 1;
  let mut i = *length - 1;
  while i > 0 {
    if buffer[i] != b'0' + 10 {
      return;
    }
    buffer[i] = b'0';
    buffer[i - 1] += 1;
    i -= 1;
  }
  // 首位变成 '0'+10: 全部为 9, 改为 '1' 并移动小数点
  if buffer[0] == b'0' + 10 {
    buffer[0] = b'1';
    *decimal_point += 1;
  }
}

/// fractionals 是二进制点位于 -exponent 的定点数; 生成 fractional_count 位并舍入
fn fill_fractionals(
  mut fractionals: u64,
  exponent: i32,
  fractional_count: i32,
  buffer: &mut [u8],
  length: &mut usize,
  decimal_point: &mut i32,
) {
  debug_assert!((-128..=0).contains(&exponent));
  if -exponent <= 64 {
    // 单个 64 位数已足够
    debug_assert!(fractionals >> 56 == 0);
    let mut point = -exponent;
    for _ in 0..fractional_count {
      if fractionals == 0 {
        break;
      }
      // 乘 5 并下移小数点, 避免溢出 (5^3=125<2^7, 每 3 轮内不溢出)
      fractionals *= 5;
      point -= 1;
      let digit = (fractionals >> point) as i32;
      buffer[*length] = b'0' + digit as u8;
      *length += 1;
      fractionals -= (digit as u64) << point;
    }
    // 小数点后第一位为 1 则进位
    if point > 0 && ((fractionals >> (point - 1)) & 1) == 1 {
      dtoa_round_up(buffer, length, decimal_point);
    }
  } else {
    debug_assert!(64 < -exponent && -exponent <= 128);
    let mut fractionals_128 = UInt128::new(fractionals, 0);
    fractionals_128.shift(-exponent - 64);
    let mut point = 128;
    for _ in 0..fractional_count {
      if fractionals_128.is_zero() {
        break;
      }
      fractionals_128.multiply(5);
      point -= 1;
      let digit = fractionals_128.div_mod_power_of_2(point);
      buffer[*length] = b'0' + digit as u8;
      *length += 1;
    }
    if fractionals_128.bit_at(point - 1) == 1 {
      dtoa_round_up(buffer, length, decimal_point);
    }
  }
}

/// 去除首尾 0; 首 0 移除时同步调整小数点
fn trim_zeros(buffer: &mut [u8], length: &mut usize, decimal_point: &mut i32) {
  while *length > 0 && buffer[*length - 1] == b'0' {
    *length -= 1;
  }
  let mut first_non_zero = 0;
  while first_non_zero < *length && buffer[first_non_zero] == b'0' {
    first_non_zero += 1;
  }
  if first_non_zero != 0 {
    for i in first_non_zero..*length {
      buffer[i - first_non_zero] = buffer[i];
    }
    *length -= first_non_zero;
    *decimal_point -= first_non_zero as i32;
  }
}

/// FastFixedDtoa 输出
pub struct FastFixedDtoaResult {
  pub length: usize,
  pub decimal_point: i32,
}

/// 原版 FastFixedDtoa: v 的非负定点十进制表示 (至多 20 位小数)
#[must_use]
pub fn fast_fixed_dtoa(
  v: f64,
  fractional_count: i32,
  buffer: &mut [u8],
) -> Option<FastFixedDtoaResult> {
  let mut length = 0usize;
  // 原版为出参 decimal_point, 每个分支都会赋值, 故不预置初值
  let mut decimal_point: i32;
  let significand = Double::from_double(v).significand();
  let exponent = Double::from_double(v).exponent();
  // v = significand × 2^exponent; 指数 >20 时 (73 位) 不支持
  if exponent > 20 {
    return None;
  }
  if fractional_count > 20 {
    return None;
  }
  if exponent + DOUBLE_SIGNIFICAND_SIZE > 64 {
    // 先除以 10^17 (= 5^17 × 2^17): 商给首位, 余数适合 64 位
    const FIVE_17: u64 = 0xB1_A2BC_2EC5;
    let mut divisor = FIVE_17;
    let divisor_power = 17;
    let mut dividend = significand;
    let quotient: u32;
    let remainder: u64;
    if exponent > divisor_power {
      // 仅允许指数 <= 20, 故 (17-e) <= 3
      dividend <<= exponent - divisor_power;
      quotient = (dividend / divisor) as u32;
      remainder = (dividend % divisor) << divisor_power;
    } else {
      divisor <<= divisor_power - exponent;
      quotient = (dividend / divisor) as u32;
      remainder = (dividend % divisor) << exponent;
    }
    fill_digits_32(quotient, buffer, &mut length);
    fill_digits_64_fixed_length(remainder, divisor_power, buffer, &mut length);
    decimal_point = length as i32;
  } else if exponent >= 0 {
    // 0 <= exponent <= 11
    let shifted = significand << exponent;
    fill_digits_64(shifted, buffer, &mut length);
    decimal_point = length as i32;
  } else if exponent > -DOUBLE_SIGNIFICAND_SIZE {
    // 需要切割数值
    let integrals = significand >> -exponent;
    let fractionals = significand - (integrals << -exponent);
    if integrals > u64::from(u32::MAX) {
      fill_digits_64(integrals, buffer, &mut length);
    } else {
      fill_digits_32(integrals as u32, buffer, &mut length);
    }
    decimal_point = length as i32;
    fill_fractionals(
      fractionals,
      exponent,
      fractional_count,
      buffer,
      &mut length,
      &mut decimal_point,
    );
  } else if exponent < -128 {
    // 该配置 (<=20 位) 下所有位必为 0
    debug_assert!(fractional_count <= 20);
    buffer[0] = 0;
    length = 0;
    decimal_point = -fractional_count;
  } else {
    decimal_point = 0;
    fill_fractionals(
      significand,
      exponent,
      fractional_count,
      buffer,
      &mut length,
      &mut decimal_point,
    );
  }
  trim_zeros(buffer, &mut length, &mut decimal_point);
  buffer[length] = 0;
  if length == 0 {
    // 与 dtoa (Gay) 保持一致: 空串时小数点取 -fractional_count
    decimal_point = -fractional_count;
  }
  Some(FastFixedDtoaResult {
    length,
    decimal_point,
  })
}
