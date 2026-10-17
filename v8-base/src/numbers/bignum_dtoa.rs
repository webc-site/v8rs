//! 原版 v8/src/base/numbers/bignum-dtoa.cc/h — 基于 Bignum 的精确十进制转换 (慢速路径)

use super::{bignum::Bignum, double::Double};

/// BignumDtoaMode — 与 DtoaMode 一一对应
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BignumDtoaMode {
  /// 最短可回读表示
  Shortest,
  /// 小数点后固定位数
  Fixed,
  /// 固定位数 (与指数无关)
  Precision,
}

/// BignumDtoa 输出
pub struct BignumDtoaResult {
  pub length: usize,
  pub point: i32,
}

/// 归一化指数: 将 significand 左移到隐藏位
fn normalized_exponent(mut significand: u64, mut exponent: i32) -> i32 {
  debug_assert_ne!(significand, 0);
  while significand & Double::HIDDEN_BIT == 0 {
    significand <<= 1;
    exponent -= 1;
  }
  exponent
}

/// 估计 k 使 10^(k-1) <= v < 10^k (可能低估 1)
fn estimate_power(exponent: i32) -> i32 {
  const K1_LOG10: f64 = 0.301_029_995_663_981_14;
  // double 有效位 53 (含隐藏位)
  const SIGNIFICAND_SIZE: i32 = 53;
  let estimate = ((f64::from(exponent + SIGNIFICAND_SIZE - 1) * K1_LOG10) - 1e-10).ceil();
  estimate as i32
}

fn initial_scaled_start_values_positive_exponent(
  v: f64,
  estimated_power: i32,
  need_boundary_deltas: bool,
  numerator: &mut Bignum,
  denominator: &mut Bignum,
  delta_minus: &mut Bignum,
  delta_plus: &mut Bignum,
) {
  debug_assert!(estimated_power >= 0);
  // numerator = v
  numerator.assign_uint64(Double::from_double(v).significand());
  numerator.shift_left(Double::from_double(v).exponent());
  // denominator = 10^estimated_power
  denominator.assign_power_uint16(10, estimated_power);

  if need_boundary_deltas {
    // 引入公共分母使边界差为整数
    denominator.shift_left(1);
    numerator.shift_left(1);
    // v = f×2^e ⇒ delta_plus = delta_minus = 2^e
    delta_plus.assign_uint16(1);
    delta_plus.shift_left(Double::from_double(v).exponent());
    delta_minus.assign_uint16(1);
    delta_minus.shift_left(Double::from_double(v).exponent());

    // 若隐藏位外的有效位为 0, 下边界距离为普通值的一半
    let v_bits = Double::from_double(v).as_uint64();
    if v_bits & Double::SIGNIFICAND_MASK == 0 {
      denominator.shift_left(1);
      numerator.shift_left(1);
      delta_plus.shift_left(1);
    }
  }
}

fn initial_scaled_start_values_negative_exponent_positive_power(
  v: f64,
  estimated_power: i32,
  need_boundary_deltas: bool,
  numerator: &mut Bignum,
  denominator: &mut Bignum,
  delta_minus: &mut Bignum,
  delta_plus: &mut Bignum,
) {
  let significand = Double::from_double(v).significand();
  let exponent = Double::from_double(v).exponent();
  // v = f×2^e (e<0), estimated_power >= 0
  numerator.assign_uint64(significand);
  // denominator = 10^estimated_power × 2^-exponent
  denominator.assign_power_uint16(10, estimated_power);
  denominator.shift_left(-exponent);

  if need_boundary_deltas {
    denominator.shift_left(1);
    numerator.shift_left(1);
    // 分母已含 2^e, 边界距离即 1
    delta_plus.assign_uint16(1);
    delta_minus.assign_uint16(1);

    let v_bits = Double::from_double(v).as_uint64();
    if v_bits & Double::SIGNIFICAND_MASK == 0 {
      denominator.shift_left(1);
      numerator.shift_left(1);
      delta_plus.shift_left(1);
    }
  }
}

fn initial_scaled_start_values_negative_exponent_negative_power(
  v: f64,
  estimated_power: i32,
  need_boundary_deltas: bool,
  numerator: &mut Bignum,
  denominator: &mut Bignum,
  delta_minus: &mut Bignum,
  delta_plus: &mut Bignum,
) {
  const MINIMAL_NORMALIZED_EXPONENT: u64 = 0x0010_0000_0000_0000;
  let significand = Double::from_double(v).significand();
  let exponent = Double::from_double(v).exponent();
  // 不改分母而是把分子/差值乘 10^-estimated_power
  // numerator 被借用为 power_ten
  numerator.assign_power_uint16(10, -estimated_power);

  if need_boundary_deltas {
    // delta_plus = delta_minus = 10^-estimated_power (此处原文注释即指 power 值)
    delta_plus.assign_bignum(numerator);
    delta_minus.assign_bignum(numerator);
  }

  // numerator = significand × 2 × 10^-estimated_power
  // (numerator 已被借用为 power_ten, 原位继续乘)
  numerator.multiply_by_uint64(significand);

  // denominator = 2 × 2^-exponent (exponent < 0)
  denominator.assign_uint16(1);
  denominator.shift_left(-exponent);

  if need_boundary_deltas {
    numerator.shift_left(1);
    denominator.shift_left(1);
    // 此时边界差值已获得正确量级 (先前赋值完成)

    // 下边界距离减半的特殊情形 (含最小规格化数例外)
    let v_bits = Double::from_double(v).as_uint64();
    if v_bits & Double::SIGNIFICAND_MASK == 0
      && v_bits & Double::EXPONENT_MASK != MINIMAL_NORMALIZED_EXPONENT
    {
      numerator.shift_left(1);
      denominator.shift_left(1);
      delta_plus.shift_left(1);
    }
  }
}

/// 计算 v/10^estimated_power 的精确 bignum 分式 (含可选边界差值)
fn initial_scaled_start_values(
  v: f64,
  estimated_power: i32,
  need_boundary_deltas: bool,
  numerator: &mut Bignum,
  denominator: &mut Bignum,
  delta_minus: &mut Bignum,
  delta_plus: &mut Bignum,
) {
  if Double::from_double(v).exponent() >= 0 {
    initial_scaled_start_values_positive_exponent(
      v,
      estimated_power,
      need_boundary_deltas,
      numerator,
      denominator,
      delta_minus,
      delta_plus,
    );
  } else if estimated_power >= 0 {
    initial_scaled_start_values_negative_exponent_positive_power(
      v,
      estimated_power,
      need_boundary_deltas,
      numerator,
      denominator,
      delta_minus,
      delta_plus,
    );
  } else {
    initial_scaled_start_values_negative_exponent_negative_power(
      v,
      estimated_power,
      need_boundary_deltas,
      numerator,
      denominator,
      delta_minus,
      delta_plus,
    );
  }
}

/// 把分式调整到 1-10 区间, 返回 decimal_point
fn fixup_multiply_10(
  estimated_power: i32,
  is_even: bool,
  decimal_point: &mut i32,
  numerator: &mut Bignum,
  denominator: &Bignum,
  delta_minus: &mut Bignum,
  delta_plus: &mut Bignum,
) {
  let compare = Bignum::plus_compare(numerator, delta_plus, denominator);
  // is_even 时 .5 情形舍向偶数: 边界取 >=
  let in_range = if is_even { compare >= 0 } else { compare > 0 };
  if in_range {
    // 已在 1 <= n/d < 10 内: 仅修正幂
    *decimal_point = estimated_power + 1;
  } else {
    *decimal_point = estimated_power;
    numerator.times10();
    if Bignum::equal(delta_minus, delta_plus) {
      delta_minus.times10();
      delta_plus.assign_bignum(delta_minus);
    } else {
      delta_minus.times10();
      delta_plus.times10();
    }
  }
}

/// 从左到右生成数字, 直到得到最短可回读表示
fn generate_shortest_digits(
  numerator: &mut Bignum,
  denominator: &Bignum,
  delta_minus: &mut Bignum,
  delta_plus: &mut Bignum,
  is_even: bool,
  buffer: &mut [u8],
  length: &mut usize,
) {
  // 优化: 两差值相等时共用一个 (Rust 用别名标志模拟指针别名)
  let plus_alias = Bignum::equal(delta_minus, delta_plus);
  *length = 0;
  loop {
    let digit = numerator.divide_modulo_int_bignum(denominator);
    debug_assert!(digit <= 9);
    buffer[*length] = b'0' + digit as u8;
    *length += 1;

    // 余数是否已进入下/上边界安全区
    let in_delta_room_minus = if is_even {
      Bignum::less_equal(numerator, delta_minus)
    } else {
      Bignum::less(numerator, delta_minus)
    };
    let compare = if plus_alias {
      Bignum::plus_compare(numerator, delta_minus, denominator)
    } else {
      Bignum::plus_compare(numerator, delta_plus, denominator)
    };
    let in_delta_room_plus = if is_even { compare >= 0 } else { compare > 0 };

    if !in_delta_room_minus && !in_delta_room_plus {
      // 为下一位做准备
      numerator.times10();
      delta_minus.times10();
      // delta_plus 与 delta_minus 值相同时不重复乘 (原版靠指针相等跳过)
      if !plus_alias {
        delta_plus.times10();
      }
    } else if in_delta_room_minus && in_delta_room_plus {
      // 2*numerator 与 denominator 比较决定舍入方向
      let compare = Bignum::plus_compare(numerator, numerator, denominator);
      if compare < 0 {
        // 剩余 < .5: 舍去 (什么都不做)
      } else if compare > 0 {
        // 剩余 > .5: 进位 (末位不可能是 '9', 否则早已退出循环)
        debug_assert_ne!(buffer[*length - 1], b'9');
        buffer[*length - 1] += 1;
      } else {
        // 恰为 .5: 舍向偶数 (与 Gay 一致)
        if !(buffer[*length - 1] - b'0').is_multiple_of(2) {
          debug_assert_ne!(buffer[*length - 1], b'9');
          buffer[*length - 1] += 1;
        }
      }
      return;
    } else if in_delta_room_minus {
      // 舍去
      return;
    } else {
      // 进位
      debug_assert_ne!(buffer[*length - 1], b'9');
      buffer[*length - 1] += 1;
      return;
    }
  }
}

/// 生成 count 位并按余数舍入 (恰 .5 向上)
fn generate_counted_digits(
  count: i32,
  decimal_point: &mut i32,
  numerator: &mut Bignum,
  denominator: &Bignum,
  buffer: &mut [u8],
  length: &mut usize,
) {
  debug_assert!(count > 0, "count==0 由 DoubleToAscii 提前拦截");
  let count = count as usize;
  for slot in buffer.iter_mut().take(count - 1) {
    let digit = numerator.divide_modulo_int_bignum(denominator);
    debug_assert!(digit <= 9);
    *slot = b'0' + digit as u8;
    numerator.times10();
  }
  // 生成最后一位
  let mut digit = numerator.divide_modulo_int_bignum(denominator);
  if Bignum::plus_compare(numerator, numerator, denominator) >= 0 {
    digit += 1;
  }
  buffer[count - 1] = b'0' + digit as u8;
  // 修正连续 '9' 进位产生的坏位
  let mut i = count - 1;
  while i > 0 {
    if buffer[i] != b'0' + 10 {
      break;
    }
    buffer[i] = b'0';
    buffer[i - 1] += 1;
    i -= 1;
  }
  if buffer[0] == b'0' + 10 {
    // 向最高位以上进位
    buffer[0] = b'1';
    *decimal_point += 1;
  }
  *length = count;
}

/// 生成小数点后 requested_digits 位 (可能省略尾部 '0')
fn bignum_to_fixed(
  requested_digits: i32,
  decimal_point: &mut i32,
  numerator: &mut Bignum,
  denominator: &mut Bignum,
  buffer: &mut [u8],
  length: &mut usize,
) {
  // 需多看一位: 例如 v=0.5 且 requested_digits=0 仍要舍入
  if -*decimal_point > requested_digits {
    // 数值过小: 无数字生成
    *decimal_point = -requested_digits;
    *length = 0;
  } else if -*decimal_point == requested_digits {
    // 仅需验证舍入方向 (0.04 / 0.06 例)
    debug_assert_eq!(*decimal_point, -requested_digits);
    // 分母乘 10 便于比较
    denominator.times10();
    if Bignum::plus_compare(numerator, numerator, denominator) >= 0 {
      // 分数 >= 0.5: 写入舍入后的 1 位
      buffer[0] = b'1';
      *length = 1;
      *decimal_point += 1;
    } else {
      *length = 0;
    }
  } else {
    // 需要的位数含小数点前的位
    let needed_digits = *decimal_point + requested_digits;
    generate_counted_digits(
      needed_digits,
      decimal_point,
      numerator,
      denominator,
      buffer,
      length,
    );
  }
}

/// 原版 BignumDtoa: v 必须 >0 且非特殊值; buffer 写数字并置 `0` 结尾
pub fn bignum_dtoa(
  v: f64,
  mode: BignumDtoaMode,
  requested_digits: i32,
  buffer: &mut [u8],
) -> BignumDtoaResult {
  debug_assert!(v > 0.0);
  debug_assert!(!Double::from_double(v).is_special());
  let significand = Double::from_double(v).significand();
  let is_even = significand & 1 == 0;
  let exponent = Double::from_double(v).exponent();
  let normalized_exponent = normalized_exponent(significand, exponent);
  // estimated_power 可能低估 1
  let estimated_power = estimate_power(normalized_exponent);

  // FIXED 捷径: 数值太小无需取位
  if mode == BignumDtoaMode::Fixed && -estimated_power - 1 > requested_digits {
    buffer[0] = 0;
    return BignumDtoaResult {
      length: 0,
      point: -requested_digits,
    };
  }

  let mut numerator = Bignum::new();
  let mut denominator = Bignum::new();
  let mut delta_minus = Bignum::new();
  let mut delta_plus = Bignum::new();
  // 最小 double = 4e-324: 分母 < 324×4 二进制位; 最大 e308 亦 < 308×4
  // MAX_SIGNIFICANT_BITS(3584) >= 324*4: 编译期常量性质, 注释保留原版 DCHECK 语义
  let need_boundary_deltas = mode == BignumDtoaMode::Shortest;
  initial_scaled_start_values(
    v,
    estimated_power,
    need_boundary_deltas,
    &mut numerator,
    &mut denominator,
    &mut delta_minus,
    &mut delta_plus,
  );
  let mut decimal_point = 0i32;
  fixup_multiply_10(
    estimated_power,
    is_even,
    &mut decimal_point,
    &mut numerator,
    &denominator,
    &mut delta_minus,
    &mut delta_plus,
  );
  let mut length = 0usize;
  match mode {
    BignumDtoaMode::Shortest => generate_shortest_digits(
      &mut numerator,
      &denominator,
      &mut delta_minus,
      &mut delta_plus,
      is_even,
      buffer,
      &mut length,
    ),
    BignumDtoaMode::Fixed => bignum_to_fixed(
      requested_digits,
      &mut decimal_point,
      &mut numerator,
      &mut denominator,
      buffer,
      &mut length,
    ),
    BignumDtoaMode::Precision => generate_counted_digits(
      requested_digits,
      &mut decimal_point,
      &mut numerator,
      &denominator,
      buffer,
      &mut length,
    ),
  }
  buffer[length] = 0;
  BignumDtoaResult {
    length,
    point: decimal_point,
  }
}
