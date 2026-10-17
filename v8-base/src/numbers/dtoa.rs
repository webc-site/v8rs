//! 原版 v8/src/base/numbers/dtoa.cc/h — DoubleToAscii 调度器 (fast → bignum 兜底)

use super::{
  bignum_dtoa::{BignumDtoaMode, BignumDtoaResult, bignum_dtoa},
  double::Double,
  fast_dtoa::{FastDtoaMode, fast_dtoa},
  fixed_dtoa::fast_fixed_dtoa,
};

/// DtoaMode — 最短 / 定点小数位 / 固定位数
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DtoaMode {
  /// 最短可回读表示 (0.29999… 输出 0.3)
  Shortest,
  /// 小数点后固定位数
  Fixed,
  /// 与指数无关的固定位数
  Precision,
}

/// kBase10MaximalLength = 17 (不含符号/小数点/终止符)
pub const BASE10_MAXIMAL_LENGTH: usize = 17;

/// DoubleToAscii 输出 (sign/length/point)
pub struct DoubleToAsciiResult {
  pub sign: i32,
  pub length: usize,
  pub point: i32,
}

fn dtoa_to_bignum_dtoa_mode(mode: DtoaMode) -> BignumDtoaMode {
  match mode {
    DtoaMode::Shortest => BignumDtoaMode::Shortest,
    DtoaMode::Fixed => BignumDtoaMode::Fixed,
    DtoaMode::Precision => BignumDtoaMode::Precision,
  }
}

/// 原版 DoubleToAscii: buffer 写数字并 NUL 结尾;
/// SHORTEST 模式 buffer 需 >= kBase10MaximalLength+1, 其余需 >= requested_digits+1
pub fn double_to_ascii(
  mut v: f64,
  mode: DtoaMode,
  requested_digits: i32,
  buffer: &mut [u8],
) -> DoubleToAsciiResult {
  debug_assert!(!Double::from_double(v).is_special());
  debug_assert!(mode == DtoaMode::Shortest || requested_digits >= 0);

  let sign = if Double::from_double(v).sign() < 0 {
    v = -v;
    1
  } else {
    0
  };

  if v == 0.0 {
    buffer[0] = b'0';
    buffer[1] = 0;
    return DoubleToAsciiResult {
      sign,
      length: 1,
      point: 1,
    };
  }

  if mode == DtoaMode::Precision && requested_digits == 0 {
    buffer[0] = 0;
    return DoubleToAsciiResult {
      sign,
      length: 0,
      point: 0,
    };
  }

  // 快速路径
  let fast = match mode {
    DtoaMode::Shortest => {
      fast_dtoa(v, FastDtoaMode::Shortest, 0, buffer).map(|r| (r.length, r.decimal_point))
    }
    DtoaMode::Fixed => {
      fast_fixed_dtoa(v, requested_digits, buffer).map(|r| (r.length, r.decimal_point))
    }
    DtoaMode::Precision => fast_dtoa(v, FastDtoaMode::Precision, requested_digits, buffer)
      .map(|r| (r.length, r.decimal_point)),
  };
  if let Some((length, point)) = fast {
    return DoubleToAsciiResult {
      sign,
      length,
      point,
    };
  }

  // 快速失败: 慢速 Bignum 路径
  let BignumDtoaResult { length, point } =
    bignum_dtoa(v, dtoa_to_bignum_dtoa_mode(mode), requested_digits, buffer);
  buffer[length] = 0;
  DoubleToAsciiResult {
    sign,
    length,
    point,
  }
}
