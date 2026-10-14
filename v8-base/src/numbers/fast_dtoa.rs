//! 原版 v8/src/base/numbers/fast-dtoa.cc/h — Grisu3 快速最短/精度十进制转换

use super::{cached_powers::PowersOfTenCache, diy_fp::DiyFp, double::Double};

/// 目标二进制指数范围: w = input × 10^-k 的指数区间
const MINIMAL_TARGET_EXPONENT: i32 = -60;
const MAXIMAL_TARGET_EXPONENT: i32 = -32;

/// FastDtoaMode — 最短表示 / 固定位数
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FastDtoaMode {
  /// 最短可回读表示
  Shortest,
  /// 精确 requested_digits 位
  Precision,
}

/// kFastDtoaMaximalLength = 17 (不含终止符)
pub const FAST_DTOA_MAXIMAL_LENGTH: usize = 17;

/// 调整末位并筛除不精确结果 (原版 RoundWeed, 无符号回绕语义保持)
fn round_weed(
  buffer: &mut [u8],
  last_digit: usize,
  distance_too_high_w: u64,
  unsafe_interval: u64,
  mut rest: u64,
  ten_kappa: u64,
  unit: u64,
) -> bool {
  let small_distance = distance_too_high_w.wrapping_sub(unit);
  let big_distance = distance_too_high_w.wrapping_add(unit);
  debug_assert!(rest <= unsafe_interval);
  // 依次判 3 个终止条件, 顺序不可调换以免溢出
  while rest < small_distance
    && unsafe_interval.wrapping_sub(rest) >= ten_kappa
    && (rest.wrapping_add(ten_kappa) < small_distance
      || small_distance.wrapping_sub(rest)
        >= rest.wrapping_add(ten_kappa).wrapping_sub(small_distance))
  {
    buffer[last_digit] -= 1;
    rest = rest.wrapping_add(ten_kappa);
  }

  if rest < big_distance
    && unsafe_interval.wrapping_sub(rest) >= ten_kappa
    && (rest.wrapping_add(ten_kappa) < big_distance
      || big_distance.wrapping_sub(rest) > rest.wrapping_add(ten_kappa).wrapping_sub(big_distance))
  {
    return false;
  }
  // 安全区间筛查: rest ~ too_high - buffer
  2u64.wrapping_mul(unit) <= rest && rest <= unsafe_interval.wrapping_sub(4u64.wrapping_mul(unit))
}

/// 对已生成位数做末位舍入 (原版 RoundWeedCounted)
fn round_weed_counted(
  buffer: &mut [u8],
  length: usize,
  rest: u64,
  ten_kappa: u64,
  unit: u64,
  kappa: &mut i32,
) -> bool {
  debug_assert!(rest < ten_kappa);
  // 单位过大则无法判定舍入方向 (以下测试顺序避免溢出)
  if unit >= ten_kappa {
    return false;
  }
  if ten_kappa.wrapping_sub(unit) <= unit {
    return false;
  }
  // 2*(rest+unit) <= 10^kappa: 可安全舍去
  if ten_kappa.wrapping_sub(rest) > rest
    && ten_kappa.wrapping_sub(2u64.wrapping_mul(rest)) >= 2u64.wrapping_mul(unit)
  {
    return true;
  }
  // 2*(rest-unit) >= 10^kappa: 可安全进位
  if rest > unit && ten_kappa.wrapping_sub(rest.wrapping_sub(unit)) <= rest.wrapping_sub(unit) {
    buffer[length - 1] += 1;
    let mut i = length - 1;
    while i > 0 {
      if buffer[i] != b'0' + 10 {
        break;
      }
      buffer[i] = b'0';
      buffer[i - 1] += 1;
      i -= 1;
    }
    // 全体为 '9' 时: "99"→"10", kappa 增加
    if buffer[0] == b'0' + 10 {
      buffer[0] = b'1';
      *kappa += 1;
    }
    return true;
  }
  false
}

const TEN_4: u32 = 10000;
const TEN_5: u32 = 100_000;
const TEN_6: u32 = 1_000_000;
const TEN_7: u32 = 10_000_000;
const TEN_8: u32 = 100_000_000;
const TEN_9: u32 = 1_000_000_000;

/// libdivide 生成的除法魔数表
struct DivMagic {
  mul: u32,
  shift: u32,
}

const DIV: [DivMagic; 9] = [
  DivMagic { mul: 0, shift: 0 }, // 1 未用 (算法不支持)
  DivMagic {
    mul: 0x9999_999a,
    shift: 3,
  }, // 10
  DivMagic {
    mul: 0x47ae_147b,
    shift: 6,
  }, // 100
  DivMagic {
    mul: 0x0624_dd30,
    shift: 9,
  }, // 1000
  DivMagic {
    mul: 0xa36e_2eb2,
    shift: 13,
  }, // 10000
  DivMagic {
    mul: 0x4f8b_588f,
    shift: 16,
  }, // 100000
  DivMagic {
    mul: 0x0c6f_7a0c,
    shift: 19,
  }, // 1000000
  DivMagic {
    mul: 0xad7f_29ac,
    shift: 23,
  }, // 10000000
  DivMagic {
    mul: 0x5798_ee24,
    shift: 26,
  }, // 100000000
];

/// val / divisor 且 val %= divisor (libdivide 分支无关 u32 算法)
fn fast_divmod(val: &mut u32, divisor: u32, d: &DivMagic) -> u32 {
  if divisor == 1 {
    let digit = *val;
    *val = 0;
    digit
  } else {
    let q = ((u64::from(*val) * u64::from(d.mul)) >> 32) as u32;
    let t = ((*val).wrapping_sub(q)) / 2 + q;
    let digit = t >> d.shift;
    *val = val.wrapping_sub(digit.wrapping_mul(divisor));
    digit
  }
}

/// 不超过 number 的最大 10 幂; number_bits 为 number 的二进制位数上限
fn biggest_power_ten(number: u32, number_bits: i32) -> (u32, i32) {
  // C 的 case 区间 fallthrough 等价改写: 从高到低逐档判定
  if number_bits >= 30 && number >= TEN_9 {
    return (TEN_9, 9);
  }
  if number_bits >= 27 && number >= TEN_8 {
    return (TEN_8, 8);
  }
  if number_bits >= 24 && number >= TEN_7 {
    return (TEN_7, 7);
  }
  if number_bits >= 20 && number >= TEN_6 {
    return (TEN_6, 6);
  }
  if number_bits >= 17 && number >= TEN_5 {
    return (TEN_5, 5);
  }
  if number_bits >= 14 && number >= TEN_4 {
    return (TEN_4, 4);
  }
  if number_bits >= 10 && number >= 1000 {
    return (1000, 3);
  }
  if number_bits >= 7 && number >= 100 {
    return (100, 2);
  }
  if number_bits >= 4 && number >= 10 {
    return (10, 1);
  }
  if number_bits >= 1 && number >= 1 {
    return (1, 0);
  }
  // case 0: 返回 10^-1
  (0, -1)
}

/// DigitGen 输出: buffer 中写入的位数
struct DigitGenOut {
  len: usize,
  kappa: i32,
}

/// 生成 low/high 安全区间内 w 的最短十进制位串 (原版 DigitGen)
fn digit_gen(
  low: DiyFp,
  w: DiyFp,
  mut high: DiyFp,
  buffer: &mut [u8],
) -> Option<(DigitGenOut, bool)> {
  debug_assert!(low.e() == w.e() && w.e() == high.e());
  debug_assert!(low.f().wrapping_add(1) <= high.f().wrapping_sub(1));
  debug_assert!(MINIMAL_TARGET_EXPONENT <= w.e() && w.e() <= MAXIMAL_TARGET_EXPONENT);
  let unit: u64 = 1;
  let too_low = DiyFp::new(low.f().wrapping_sub(unit), low.e());
  high.set_f(high.f().wrapping_add(unit));
  let too_high = DiyFp::new(high.f(), high.e());
  let mut unsafe_interval = DiyFp::minus(too_high, too_low);
  let one = DiyFp::new(1u64 << -w.e(), w.e());
  // 除以 one 为移位, 取模为按位与
  let mut integrals = (too_high.f() >> -one.e()) as u32;
  let mut fractionals = too_high.f() & (one.f().wrapping_sub(1));
  let (mut divisor, mut divisor_exponent) =
    biggest_power_ten(integrals, DiyFp::SIGNIFICAND_SIZE - (-one.e()));
  let mut kappa = divisor_exponent + 1;
  let mut len: usize = 0;
  while kappa > 0 {
    let digit = fast_divmod(&mut integrals, divisor, &DIV[divisor_exponent as usize]);
    buffer[len] = b'0' + digit as u8;
    len += 1;
    kappa -= 1;
    let rest = (u64::from(integrals) << -one.e()).wrapping_add(fractionals);
    if rest < unsafe_interval.f() {
      // 提前停止等价于向下舍入, 落在不安全区间内
      let out = DigitGenOut { len, kappa };
      let ok = round_weed(
        buffer,
        len - 1,
        DiyFp::minus(too_high, w).f(),
        unsafe_interval.f(),
        rest,
        (u64::from(divisor)) << -one.e(),
        unit,
      );
      return Some((out, ok));
    }
    if kappa <= 0 {
      break;
    }
    divisor /= 10;
    divisor_exponent -= 1;
  }

  // 整数部分已生成: 余下的小数位用乘 10 除 one 生成
  debug_assert!(one.e() >= -60);
  debug_assert!(fractionals < one.f());
  debug_assert!(u64::MAX / 10 >= one.f());
  let mut unit = unit;
  loop {
    fractionals = fractionals.wrapping_mul(10);
    unit = unit.wrapping_mul(10);
    unsafe_interval.set_f(unsafe_interval.f().wrapping_mul(10));
    let digit = (fractionals >> -one.e()) as i32;
    buffer[len] = b'0' + digit as u8;
    len += 1;
    fractionals &= one.f().wrapping_sub(1);
    kappa -= 1;
    if fractionals < unsafe_interval.f() {
      let out = DigitGenOut { len, kappa };
      let ok = round_weed(
        buffer,
        len - 1,
        DiyFp::minus(too_high, w).f().wrapping_mul(unit),
        unsafe_interval.f(),
        fractionals,
        one.f(),
        unit,
      );
      return Some((out, ok));
    }
  }
}

/// 生成 (至多) requested_digits 位 (原版 DigitGenCounted)
fn digit_gen_counted(
  w: DiyFp,
  mut requested_digits: i32,
  buffer: &mut [u8],
) -> Option<(usize, i32, bool)> {
  debug_assert!(MINIMAL_TARGET_EXPONENT <= w.e() && w.e() <= MAXIMAL_TARGET_EXPONENT);
  // MINIMAL_TARGET_EXPONENT(-60) >= -60 / MAXIMAL(-32) <= -32: 常量性质,
  // 注释保留原版 DCHECK 语义
  // w 的误差 < 1 ulp; 缩放 w 时误差同步缩放
  let mut w_error: u64 = 1;
  let one = DiyFp::new(1u64 << -w.e(), w.e());
  let mut integrals = (w.f() >> -one.e()) as u32;
  let mut fractionals = w.f() & (one.f().wrapping_sub(1));
  let (mut divisor, mut divisor_exponent) =
    biggest_power_ten(integrals, DiyFp::SIGNIFICAND_SIZE - (-one.e()));
  let mut kappa = divisor_exponent + 1;
  let mut length: usize = 0;

  while kappa > 0 {
    let digit = fast_divmod(&mut integrals, divisor, &DIV[divisor_exponent as usize]);
    buffer[length] = b'0' + digit as u8;
    length += 1;
    requested_digits -= 1;
    kappa -= 1;
    if requested_digits == 0 {
      break;
    }
    divisor /= 10;
    divisor_exponent -= 1;
  }

  if requested_digits == 0 {
    let rest = (u64::from(integrals) << -one.e()).wrapping_add(fractionals);
    let mut kappa_local = kappa;
    let ok = round_weed_counted(
      buffer,
      length,
      rest,
      u64::from(divisor) << -one.e(),
      w_error,
      &mut kappa_local,
    );
    return Some((length, kappa_local, ok));
  }

  debug_assert!(one.e() >= -60);
  debug_assert!(fractionals < one.f());
  debug_assert!(u64::MAX / 10 >= one.f());
  while requested_digits > 0 && fractionals > w_error {
    fractionals = fractionals.wrapping_mul(10);
    w_error = w_error.wrapping_mul(10);
    let digit = (fractionals >> -one.e()) as i32;
    buffer[length] = b'0' + digit as u8;
    length += 1;
    requested_digits -= 1;
    fractionals &= one.f().wrapping_sub(1);
    kappa -= 1;
  }
  if requested_digits != 0 {
    return Some((length, kappa, false));
  }
  let mut kappa_local = kappa;
  let ok = round_weed_counted(
    buffer,
    length,
    fractionals,
    one.f(),
    w_error,
    &mut kappa_local,
  );
  Some((length, kappa_local, ok))
}

/// Grisu3: 最短表示 (原版 Grisu3)
fn grisu3(v: f64, buffer: &mut [u8]) -> Option<(usize, i32)> {
  let w = Double::from_double(v).as_normalized_diy_fp();
  // boundaries: 任何严格位于其间的数都会舍入到 v
  let (boundary_minus, boundary_plus) = Double::from_double(v).normalized_boundaries();
  debug_assert!(boundary_plus.e() == w.e());
  let ten_mk_minimal_binary_exponent = MINIMAL_TARGET_EXPONENT - (w.e() + DiyFp::SIGNIFICAND_SIZE);
  let ten_mk_maximal_binary_exponent = MAXIMAL_TARGET_EXPONENT - (w.e() + DiyFp::SIGNIFICAND_SIZE);
  let (ten_mk, mk) = PowersOfTenCache::get_cached_power_for_binary_exponent_range(
    ten_mk_minimal_binary_exponent,
    ten_mk_maximal_binary_exponent,
  );
  debug_assert!(
    MINIMAL_TARGET_EXPONENT <= w.e() + ten_mk.e() + DiyFp::SIGNIFICAND_SIZE
      && MAXIMAL_TARGET_EXPONENT >= w.e() + ten_mk.e() + DiyFp::SIGNIFICAND_SIZE
  );
  // ten_mk 仅 64 位精度; Times 舍入后 scaled_w 误差 < 1ulp
  let scaled_w = DiyFp::times(w, ten_mk);
  debug_assert!(scaled_w.e() == boundary_plus.e() + ten_mk.e() + DiyFp::SIGNIFICAND_SIZE);
  let scaled_boundary_minus = DiyFp::times(boundary_minus, ten_mk);
  let scaled_boundary_plus = DiyFp::times(boundary_plus, ten_mk);

  let (out, result) = digit_gen(
    scaled_boundary_minus,
    scaled_w,
    scaled_boundary_plus,
    buffer,
  )?;
  let decimal_exponent = -mk + out.kappa;
  result.then_some((out.len, decimal_exponent))
}

/// Grisu3 计数版 (原版 Grisu3Counted)
fn grisu3_counted(v: f64, requested_digits: i32, buffer: &mut [u8]) -> Option<(usize, i32)> {
  let w = Double::from_double(v).as_normalized_diy_fp();
  let ten_mk_minimal_binary_exponent = MINIMAL_TARGET_EXPONENT - (w.e() + DiyFp::SIGNIFICAND_SIZE);
  let ten_mk_maximal_binary_exponent = MAXIMAL_TARGET_EXPONENT - (w.e() + DiyFp::SIGNIFICAND_SIZE);
  let (ten_mk, mk) = PowersOfTenCache::get_cached_power_for_binary_exponent_range(
    ten_mk_minimal_binary_exponent,
    ten_mk_maximal_binary_exponent,
  );
  debug_assert!(
    MINIMAL_TARGET_EXPONENT <= w.e() + ten_mk.e() + DiyFp::SIGNIFICAND_SIZE
      && MAXIMAL_TARGET_EXPONENT >= w.e() + ten_mk.e() + DiyFp::SIGNIFICAND_SIZE
  );
  let scaled_w = DiyFp::times(w, ten_mk);
  let (length, kappa, result) = digit_gen_counted(scaled_w, requested_digits, buffer)?;
  let decimal_exponent = -mk + kappa;
  if result {
    Some((length, decimal_exponent))
  } else {
    None
  }
}

/// FastDtoa 结果: buffer 内位数与小数点位置
pub struct FastDtoaResult {
  pub length: usize,
  pub decimal_point: i32,
}

/// 原版 FastDtoa: v 必须为严格正的有限 double; 成功时 buffer 写位数+`0`
#[must_use]
pub fn fast_dtoa(
  v: f64,
  mode: FastDtoaMode,
  requested_digits: i32,
  buffer: &mut [u8],
) -> Option<FastDtoaResult> {
  debug_assert!(v > 0.0);
  debug_assert!(!Double::from_double(v).is_special());

  let (result, length, decimal_exponent) = match mode {
    FastDtoaMode::Shortest => {
      let (len, dec_exp) = grisu3(v, buffer)?;
      (true, len, dec_exp)
    }
    FastDtoaMode::Precision => {
      let (len, dec_exp) = grisu3_counted(v, requested_digits, buffer)?;
      (true, len, dec_exp)
    }
  };
  if !result {
    return None;
  }
  let decimal_point = length as i32 + decimal_exponent;
  buffer[length] = 0;
  Some(FastDtoaResult {
    length,
    decimal_point,
  })
}
