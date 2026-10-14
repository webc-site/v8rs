// 由 tool/src/numbers/conversions-inl.h.js 从 v8/src/numbers/conversions-inl.h (+conversions.h 依赖常量) 生成; 修订就地改, 复跑覆盖
// 双精度转换魔数与签名骨架

/// 双精度→整数转换魔数与界 (原版 conversions-inl.h/conversions.h 显式展开)
pub mod conv_consts {
  /// 2^52: 借位对齐魔数 (原版 k2Pow52)
  pub const TWO_POW_52: f64 = 4503599627370496.0;

  /// double 舍入到 float 上界的最大值 (原版 kRoundingThreshold)
  pub const FLOAT_ROUNDING_THRESHOLD: f64 = 3.4028235677973362e+38;
  /// 负向镜像
  pub const NEG_FLOAT_ROUNDING_THRESHOLD: f64 = -3.4028235677973362e+38;

  /// FastD2UI 越界/∞/NaN 返回的整数不定值 (原版 0x80000000u)
  pub const INTEGER_INDEFINITE: u32 = 0x8000_0000;

  /// DoubleToUint32IfEqualToSelf: 指数 52 + 隐含位的高 32 位特征 (原版 kValidTopBits)
  pub const VALID_TOP_BITS: u32 = 0x4330_0000;
  /// 低 32 位掩码 (原版 kBottomBitMask)
  pub const BOTTOM_BIT_MASK: u64 = 0x0000_0000_FFFF_FFFF;

  // —— FP64/FP16 位型常量 (定义于 conversions.h, 依赖再展开) ——
  pub const FP64_EXPONENT_BITS: i32 = 11;
  pub const FP64_MANTISSA_BITS: i32 = 52;
  pub const FP64_EXPONENT_BIAS: u64 = 1023;
  pub const FP64_SIGN_MASK: u64 = 0x8000_0000_0000_0000;
  pub const FP64_INFINITY: u64 = 0x7FF0_0000_0000_0000;
  pub const FP16_INFINITY_AND_NAN_INFIMUM: u64 = 0x40F0_0000_0000_0000;
  pub const FP16_MIN_EXPONENT: u64 = 1009;
  pub const FP16_DENORMAL_THRESHOLD: u64 = 0x3F10_0000_0000_0000;
  pub const FP16_MANTISSA_BITS: i32 = 10;
  pub const FP16_Q_NAN: u16 = 0x7E00;
  pub const FP16_INFINITY: u16 = 0x7C00;
  pub const FP64_TO16_ROUNDING_ADDEND: u64 = 0x0001_FFFF_FFFF_FFFF;
  pub const FP64_TO16_REBIAS_EXPONENT_AND_ROUND: u64 = 0xC101_FFFF_FFFF_FFFF;
  pub const FP64_TO16_DENORMAL_MAGIC: u64 = 0x41B0_0000_0000_0000;
}

/// 转换函数签名骨架 (语义实现见 crate 内 conversions/ 模块)
pub mod conversions_inl_skel {
  use crate::conversions::{
    double_to_float16 as conv_double_to_float16, double_to_float32 as conv_double_to_float32,
    double_to_int32 as conv_double_to_int32, double_to_integer as conv_double_to_integer,
    double_to_smi_integer as conv_double_to_smi_integer,
    double_to_string_view as conv_double_to_string_view, double_to_uint32 as conv_double_to_uint32,
    double_to_uint32_if_equal_to_self as conv_double_to_uint32_if_equal_to_self,
    double_to_webidl_int64 as conv_double_to_webidl_int64,
    double_to_webidl_uint64 as conv_double_to_webidl_uint64, fast_d2_ui as conv_fast_d2_ui,
    fast_d2i as conv_fast_d2i, is_int32_double as conv_is_int32_double,
    is_minus_zero as conv_is_minus_zero, is_smi_double as conv_is_smi_double,
    is_uint32_double as conv_is_uint32_double,
    parse_double::string_to_double as conv_string_to_double, parse_int::ConversionFlag,
    tables::DOUBLE_TO_STRING_MIN_BUFFER_SIZE, try_double_to_size as conv_try_double_to_size,
  };

  /// 快速 double→u32, 不保证向零舍入 (原版 FastD2UI; 越界回 INTEGER_INDEFINITE)
  #[inline]
  #[must_use]
  pub fn fast_d2ui(x: f64) -> u32 {
    conv_fast_d2_ui(x)
  }

  /// 快速 double→i32, 保证在 int 域内 (原版 FastD2I)
  #[inline]
  #[must_use]
  pub fn fast_d2i(x: f64) -> i32 {
    conv_fast_d2i(x)
  }

  /// double→f16 位型 (rygorous 技巧, 原版 DoubleToFloat16)
  #[inline]
  #[must_use]
  pub fn double_to_float16(value: f64) -> u16 {
    conv_double_to_float16(value)
  }

  /// double→f32 (超界按 kRoundingThreshold 舍入, 原版 DoubleToFloat32)
  #[inline]
  #[must_use]
  pub fn double_to_float32(x: f64) -> f32 {
    conv_double_to_float32(x)
  }

  /// ToIntegerOrInfinity: -0 归 +0 (原版 DoubleToInteger)
  #[inline]
  #[must_use]
  pub fn double_to_integer(x: f64) -> f64 {
    conv_double_to_integer(x)
  }

  /// ToInt32 (原版 DoubleToInt32; 指数上限 31)
  #[inline]
  #[must_use]
  pub fn double_to_int32(x: f64) -> i32 {
    conv_double_to_int32(x)
  }

  /// ToUint32 (原版 DoubleToUint32)
  #[inline]
  #[must_use]
  pub fn double_to_uint32(x: f64) -> u32 {
    conv_double_to_uint32(x)
  }

  /// WebIDL ConvertToInt 有符号 (原版 DoubleToWebIDLInt64; 指数上限 63)
  #[inline]
  #[must_use]
  pub fn double_to_webidl_int64(x: f64) -> i64 {
    conv_double_to_webidl_int64(x)
  }

  /// WebIDL ConvertToInt 无符号 (原版 DoubleToWebIDLUint64)
  #[inline]
  #[must_use]
  pub fn double_to_webidl_uint64(x: f64) -> u64 {
    conv_double_to_webidl_uint64(x)
  }

  /// double 是否恰为 Smi 并回填 (原版 DoubleToSmiInteger)
  #[inline]
  #[must_use]
  pub fn double_to_smi_integer(value: f64) -> Option<i32> {
    conv_double_to_smi_integer(value)
  }

  /// Smi 判定 (原版 IsSmiDouble)
  #[inline]
  #[must_use]
  pub fn is_smi_double(value: f64) -> bool {
    conv_is_smi_double(value)
  }

  /// int32 判定 (原版 IsInt32Double)
  #[inline]
  #[must_use]
  pub fn is_int32_double(value: f64) -> bool {
    conv_is_int32_double(value)
  }

  /// uint32 判定 (原版 IsUint32Double)
  #[inline]
  #[must_use]
  pub fn is_uint32_double(value: f64) -> bool {
    conv_is_uint32_double(value)
  }

  /// 恰等 uint32 判定, 高 32 位特征检查 (原版 DoubleToUint32IfEqualToSelf)
  #[inline]
  #[must_use]
  pub fn double_to_uint32_if_equal_to_self(value: f64) -> Option<u32> {
    conv_double_to_uint32_if_equal_to_self(value)
  }

  /// 负零判定 (原版 IsMinusZero)
  #[inline]
  #[must_use]
  pub fn is_minus_zero(value: f64) -> bool {
    conv_is_minus_zero(value)
  }

  /// TryDoubleToSize — 可表示为 usize 则返回 (原版 TryNumberToSize double 分支)
  #[inline]
  #[must_use]
  pub fn try_double_to_size(value: f64) -> Option<usize> {
    conv_try_double_to_size(value)
  }

  /// DoubleToStringView — 零分配双精度转换为十进制字符串切片 (原版 DoubleToString)
  #[inline]
  #[must_use]
  pub fn double_to_string_view(value: f64, buffer: &mut [u8]) -> &str {
    conv_double_to_string_view(value, buffer)
  }

  /// DoubleToString — 双精度转换为十进制字符串 (原版 DoubleToString)
  #[must_use]
  pub fn double_to_string(value: f64) -> String {
    let mut buf = [0u8; DOUBLE_TO_STRING_MIN_BUFFER_SIZE];
    conv_double_to_string_view(value, &mut buf).to_string()
  }

  /// StringToDouble — 字符串解析为双精度浮点数 (原版 StringToDouble)
  #[inline]
  #[must_use]
  pub fn string_to_double(s: &str) -> f64 {
    conv_string_to_double(s.as_bytes(), ConversionFlag::NoConversionFlag, 0.0)
  }
}
