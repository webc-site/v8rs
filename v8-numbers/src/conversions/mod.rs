//! 原版 v8/src/numbers/conversions.h + conversions-inl.h — 内联转换族与模块编排
//!
//! 语义函数按文件族拆分 (子模块):
//! - simple_builder: SimpleStringBuilder + 数字字符查表写出 (conversions.cc 前段)
//! - parse_int: parseInt 整族 (StringToIntHelper / InternalStringToIntDouble)
//! - parse_double: StringToDouble / parseFloat 整族 (InternalStringToDouble)
//! - to_string: Number→String 整族 (DoubleToStringView / toFixed / toPrecision 等)
//! - special_index: IsSpecialIndex
//! - tables: 脚本提取的数据层

use v8_base::numbers::double::Double;
pub mod inl_generated;
pub mod parse_double;
pub mod parse_int;
pub mod simple_builder;
pub mod special_index;
pub mod tables;
pub mod to_string;

pub use parse_double::{
  binary_string_to_double, hex_string_to_double, implicit_octal_string_to_double,
  octal_string_to_double, string_to_double,
};
pub use parse_int::{ConversionFlag, NumChar, string_to_int};
pub use simple_builder::{junk_string_value, signed_zero};
pub use special_index::is_special_index;
pub use tables::*;
pub use to_string::{
  create_exponential_representation, double_to_exponential_string_view,
  double_to_fixed_string_view, double_to_precision_string_view, double_to_radix_string_view,
  double_to_string_view, int_to_string_view,
};

/// Smi 值域下界 — 原版 Smi::kMinValue (64 位平台 31 位 Smi, v8-internal.h)
pub const SMI_MIN_VALUE: i32 = -1 << 30;
/// Smi 值域上界 — 原版 Smi::kMaxValue
pub const SMI_MAX_VALUE: i32 = (1 << 30) - 1;
/// kQuietNaNMask — globals.h:776, 尾数位 51..62
pub const QUIET_NAN_MASK: u64 = 0xFFF << 51;

// ---------------------------------------------------------------------------
// 快速 double ↔ 整数 (conversions.h 92-122)
// ---------------------------------------------------------------------------

/// FastD2IChecked — NaN → i32::MIN, 其余夹取 [i32::MIN, i32::MAX] 后截断
/// (快速例程不保证向零舍入, 但被夹取域覆盖)
#[must_use]
pub fn fast_d2i_checked(x: f64) -> i32 {
  // NaN 与任何比较均为假: NaN 或下溢 → i32::MIN
  if x.is_nan() || x < i32::MIN as f64 {
    return i32::MIN;
  }
  if x > i32::MAX as f64 {
    return i32::MAX;
  }
  x as i32
}

/// FastD2I — 输入保证在 int 域内 (debug 断言语义保留为约定)
#[must_use]
#[inline]
pub fn fast_d2i(x: f64) -> i32 {
  debug_assert!((i32::MIN as f64..=i32::MAX as f64).contains(&x));
  x as i32
}

/// FastD2UI — 无符号 32 位快速转换: 加 2^52 后取低 32 位尾数;
/// 超域 / Inf / NaN 返回 0x8000_0000 (整数不定值)
#[must_use]
pub fn fast_d2_ui(mut x: f64) -> u32 {
  const K_2_POW_52: f64 = 4_503_599_627_370_496.0; // 2^52
  let negative = x < 0.0;
  if negative {
    x = -x;
  }
  if x < K_2_POW_52 {
    x += K_2_POW_52;
    // 小端序下取 double 低 32 位 = 加 2^52 后的低尾数 (目标平台恒小端)
    let result = x.to_bits() as u32;
    if negative {
      return result.wrapping_neg();
    }
    return result;
  }
  0x8000_0000 // 大数 / Inf / NaN → 整数不定值
}

/// FastI2D — int → double (无损)
#[must_use]
#[inline]
pub fn fast_i2d(x: i32) -> f64 {
  f64::from(x)
}

/// FastUI2D — unsigned → double (无损)
#[must_use]
#[inline]
pub fn fast_ui2d(x: u32) -> f64 {
  f64::from(x)
}

// ---------------------------------------------------------------------------
// double → float16 / float32 (conversions-inl.h 60-126)
// ---------------------------------------------------------------------------

/// DoubleToFloat16 — 二进制16 截断 (roundTiesToEven), 返回 fp16 位型
/// 移植自 rygorous 算法; 位常量见 [tables]
#[must_use]
pub fn double_to_float16(value: f64) -> u16 {
  let mut input = value.to_bits();
  let out: u16;

  // 取绝对值
  let sign = input & FP64_SIGN_MASK;
  input ^= sign;

  if input >= FP16_INFINITY_AND_NAN_INFIMUM {
    // 无穷或 NaN
    out = if input > FP64_INFINITY {
      FP16Q_NAN // NaN → qNaN
    } else {
      FP16_INFINITY // Inf → Inf
    };
  } else {
    // (非)规格化数或零
    if input < FP16_DENORMAL_THRESHOLD {
      // 非规格化或零: 魔数浮点加对齐低 10 位尾数 (依赖舍入到偶)
      let temp = f64::from_bits(input) + f64::from_bits(FP64_TO16_DENORMAL_MAGIC);
      out = (temp.to_bits().wrapping_sub(FP64_TO16_DENORMAL_MAGIC)) as u16;
    } else {
      // 规格化: 记录舍入前尾数奇偶
      let mant_odd = (input >> (FP64_MANTISSA_BITS - FP16_MANTISSA_BITS)) & 1;
      // 指数重偏置 + 舍入到偶 (两段加法)
      input = input.wrapping_add(FP64_TO16_REBIAS_EXPONENT_AND_ROUND);
      input = input.wrapping_add(mant_odd);
      out = (input >> (FP64_MANTISSA_BITS - FP16_MANTISSA_BITS)) as u16;
    }
  }

  out | (sign >> 48) as u16
}

/// DoubleToFloat32 — ECMA-262 20.2.2.17 (ToFloat32 语义)
#[must_use]
pub fn double_to_float32(x: f64) -> f32 {
  const F32_MAX: f32 = f32::MAX;
  if x > f32::MAX as f64 {
    // 最大可向下舍入的 double: 尾数在 float 域后紧跟 0 位
    const ROUNDING_THRESHOLD: f64 = 3.402_823_567_797_336_2e38;
    if x <= ROUNDING_THRESHOLD {
      return F32_MAX;
    }
    return f32::INFINITY;
  }
  if x < f32::MIN as f64 {
    const ROUNDING_THRESHOLD: f64 = -3.402_823_567_797_336_2e38;
    if x >= ROUNDING_THRESHOLD {
      return f32::MIN;
    }
    return f32::NEG_INFINITY;
  }
  x as f32
}

// ---------------------------------------------------------------------------
// double → 整数 (ECMA-262 ToInt32/ToInteger 等)
// ---------------------------------------------------------------------------

/// DoubleToInteger — ECMA-262 ToIntegerOrInfinity (-0 归一到 +0)
#[must_use]
pub fn double_to_integer(x: f64) -> f64 {
  if x.is_nan() || x == 0.0 {
    return 0.0;
  }
  if !x.is_finite() {
    return x;
  }
  // 截断后 +0.0 消除 -0
  (if x > 0.0 { x.floor() } else { x.ceil() }) + 0.0
}

/// DoubleToInt32 — ECMA-262 ToInt32 主体
#[must_use]
#[inline]
pub fn double_to_int32(x: f64) -> i32 {
  if (i32::MIN as f64..=i32::MAX as f64).contains(&x) {
    return x as i32;
  }
  if !x.is_finite() || x == 0.0 {
    return 0;
  }
  let rem = x.trunc() % 4294967296.0;
  let mut int64 = rem as i64;
  if int64 >= 2147483648 {
    int64 -= 4294967296;
  } else if int64 < -2147483648 {
    int64 += 4294967296;
  }
  int64 as i32
}

/// DoubleToWebIDLInt64 — WebIDL ConvertToInt 通用分支 (步 1 与 8-12)
#[must_use]
pub fn double_to_webidl_int64(x: f64) -> i64 {
  const MAX_SAFE: f64 = 9_007_199_254_740_991.0; // 2^53 - 1
  const MIN_SAFE: f64 = -9_007_199_254_740_991.0;
  if x.is_finite() && (MIN_SAFE..=MAX_SAFE).contains(&x) {
    return x as i64;
  }
  let d = Double::from_double(x);
  let exponent = d.exponent();
  let bits: u64 = if exponent < 0 {
    if exponent <= -Double::SIGNIFICAND_SIZE {
      return 0;
    }
    d.significand() >> -exponent
  } else {
    if exponent > 63 {
      return 0;
    }
    d.significand() << exponent
  };
  if d.sign() < 0 {
    (bits as i64).wrapping_neg()
  } else {
    bits as i64
  }
}

/// DoubleToWebIDLUint64 — WebIDL 无符号变体
#[must_use]
pub fn double_to_webidl_uint64(x: f64) -> u64 {
  double_to_webidl_int64(x) as u64
}

/// DoubleToUint32 — ECMA-262 ToUint32
#[must_use]
#[inline]
pub fn double_to_uint32(x: f64) -> u32 {
  double_to_int32(x) as u32
}

// ---------------------------------------------------------------------------
// double 谓词族 (conversions-inl.h 188-238)
// ---------------------------------------------------------------------------

/// IsMinusZero — 负零判定 (位型比较)
#[must_use]
pub fn is_minus_zero(value: f64) -> bool {
  value.to_bits() == (-0.0f64).to_bits()
}

/// DoubleToSmiInteger — 可表示为 Smi 则返回整值
#[must_use]
pub fn double_to_smi_integer(value: f64) -> Option<i32> {
  if !is_smi_double(value) {
    return None;
  }
  Some(fast_d2i(value))
}

/// IsSmiDouble — double 精确落在 Smi 值域 (排除 -0)
#[must_use]
pub fn is_smi_double(value: f64) -> bool {
  (SMI_MIN_VALUE as f64..=SMI_MAX_VALUE as f64).contains(&value)
    && !is_minus_zero(value)
    && value == fast_i2d(fast_d2i(value))
}

/// IsInt32Double — 精确 Integer32 ([-2^31, 2^31-1], 排除 -0)
#[must_use]
pub fn is_int32_double(value: f64) -> bool {
  (i32::MIN as f64..=i32::MAX as f64).contains(&value)
    && !is_minus_zero(value)
    && value == fast_i2d(fast_d2i(value))
}

/// IsUint32Double — 精确 UInteger32 ([0, 2^32-1], 排除 -0)
#[must_use]
pub fn is_uint32_double(value: f64) -> bool {
  !is_minus_zero(value)
    && (0.0..=u32::MAX as f64).contains(&value)
    && value == fast_ui2d(fast_d2_ui(value))
}

/// DoubleToUint32IfEqualToSelf — 双向转换相等性检验 (ECMA-262 15.4.2.2 类)
#[must_use]
pub fn double_to_uint32_if_equal_to_self(value: f64) -> Option<u32> {
  const K_2_POW_52: f64 = 4_503_599_627_370_496.0;
  const VALID_TOP_BITS: u32 = 0x4330_0000;
  const BOTTOM_BIT_MASK: u64 = 0xFFFF_FFFF;

  // 加 2^52 把有效 uint32 值排进指数/尾数低位 (同时归一 ±0)
  let shifted = value + K_2_POW_52;
  let result = shifted.to_bits();
  if (result >> 32) as u32 == VALID_TOP_BITS {
    let v = (result & BOTTOM_BIT_MASK) as u32;
    if f64::from(v) == value {
      return Some(v);
    }
  }
  None
}

/// TryNumberToSize 的 double 分支 — 可表示为 usize 则返回
/// (原版 Tagged<Object> 入口推迟到对象层, 数值语义在此保留)
#[must_use]
pub fn try_double_to_size(value: f64) -> Option<usize> {
  // 先转换上限再比较: 上限转 double 可能丢尾数成 limit+1, 故用 <
  let max_size = usize::MAX as f64;
  if !(0.0..max_size).contains(&value) {
    return None;
  }
  let size = value as usize;
  // V8_ENABLE_SANDBOX 分支 (默认配置展开; v8-internal.h:286):
  // 沙箱下缓冲区上界 32GB-1, 超出即拒绝 (防越出沙箱守护区)
  const K_MAX_SAFE_BUFFER_SIZE_FOR_SANDBOX: usize = (32 << 30) - 1;
  if size > K_MAX_SAFE_BUFFER_SIZE_FOR_SANDBOX {
    return None;
  }
  Some(size)
}
