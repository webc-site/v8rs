// 由 tool/src/numbers/conversions.cc.js 从 v8/src/numbers/conversions.cc + conversions.h (表与常量族) 生成; 修订就地改, 复跑覆盖

//! conversions 的纯数据层: 数字字符查表 + FP64/FP16/FP32 位型常量 + 格式缓冲容量
//! 语义函数见同目录其余模块; 本文件全部由脚本提取, 值不变式见尾部断言

/// kRadix100Table — 两位数字 n → 字符对 ('3','5' ⇔ 35), 尾数快速写出的查表
pub const RADIX_100_TABLE: [u8; 200] = *b"0001020304050607080910111213141516171819\
2021222324252627282930313233343536373839\
4041424344454647484950515253545556575859\
6061626364656667686970717273747576777879\
8081828384858687888990919293949596979899";

/// kRadix100HeadTable — 两位数字 n → 头部字符 (1 位数第二字节为 0 哨兵), ConvertHeadDigits 用
pub const RADIX_100_HEAD_TABLE: [u8; 200] =
  *b"\x00\x001\x002\x003\x004\x005\x006\x007\x008\x009\x0010111213141516171819\
2021222324252627282930313233343536373839\
4041424344454647484950515253545556575859\
6061626364656667686970717273747576777879\
8081828384858687888990919293949596979899";

/// FP64 指数位宽
pub const FP64_EXPONENT_BITS: i32 = 11;

/// FP64 尾数位宽
pub const FP64_MANTISSA_BITS: i32 = 52;

/// FP64 指数偏置
pub const FP64_EXPONENT_BIAS: u64 = 1023;

/// FP64 符号位掩码
pub const FP64_SIGN_MASK: u64 = 1u64 << (FP64_EXPONENT_BITS + FP64_MANTISSA_BITS);

/// FP64 无穷位型
pub const FP64_INFINITY: u64 = 2047u64 << FP64_MANTISSA_BITS;

/// FP64 中 FP16 无穷/NaN 的下界位型
pub const FP16_INFINITY_AND_NAN_INFIMUM: u64 = (FP64_EXPONENT_BIAS + 16) << FP64_MANTISSA_BITS;

/// FP64 中 FP16 最小指数 (偏置后)
pub const FP16_MIN_EXPONENT: u64 = FP64_EXPONENT_BIAS - 14;

/// FP64 中 FP16 非规格化阈值位型
pub const FP16_DENORMAL_THRESHOLD: u64 = FP16_MIN_EXPONENT << FP64_MANTISSA_BITS;

/// FP16 尾数位宽
pub const FP16_MANTISSA_BITS: i32 = 10;

/// FP16 quiet NaN 位型
pub const FP16Q_NAN: u16 = 0x7e00;

/// FP16 无穷位型
pub const FP16_INFINITY: u16 = 0x7c00;

/// double→FP16 舍入加数 (低 41 位尾数非零时置位第 11 位)
pub const FP64_TO16_ROUNDING_ADDEND: u64 =
  (1u64 << ((FP64_MANTISSA_BITS - FP16_MANTISSA_BITS) - 1)) - 1;

/// double→FP16 指数重偏置 + 就近舍入
pub const FP64_TO16_REBIAS_EXPONENT_AND_ROUND: u64 =
  ((15u64.wrapping_sub(FP64_EXPONENT_BIAS)) << FP64_MANTISSA_BITS) + FP64_TO16_ROUNDING_ADDEND;

/// double→FP16 非规格化对齐魔数 (依赖舍入到偶)
pub const FP64_TO16_DENORMAL_MAGIC: u64 =
  (FP16_MIN_EXPONENT + ((FP64_MANTISSA_BITS - FP16_MANTISSA_BITS) as u64)) << FP64_MANTISSA_BITS;

/// FP32 去符号掩码
pub const FP32_WITHOUT_SIGN_MASK: u32 = 0x7fffffff;

/// FP32 中 FP16 可表示零的最小位型
pub const FP32_MIN_FP16_ZERO_REPRESENTABLE: u32 = 0x33000000;

/// FP32 中 FP16 最大可表示位型
pub const FP32_MAX_FP16_REPRESENTABLE: u32 = 0x47800000;

/// FP32 中 FP16 非规格化阈值位型
pub const FP32_SUBNORMAL_THRESHOLD_OF_FP16: u32 = 0x38800000;

/// toFixed/toPrecision/toExponential 的位数上限
pub const MAX_FRACTION_DIGITS: i32 = 100;

/// toFixed 定点表示的整数部分最大位数
pub const DOUBLE_TO_FIXED_MAX_DIGITS_BEFORE_POINT: i32 = 21;

/// toFixed 结果缓冲容量
pub const DOUBLE_TO_FIXED_MAX_CHARS: i32 =
  DOUBLE_TO_FIXED_MAX_DIGITS_BEFORE_POINT + MAX_FRACTION_DIGITS + 2;

/// toPrecision 结果缓冲容量
pub const DOUBLE_TO_PRECISION_MAX_CHARS: i32 = MAX_FRACTION_DIGITS + 8;

/// toExponential 结果缓冲容量
pub const DOUBLE_TO_EXPONENTIAL_MAX_CHARS: i32 = MAX_FRACTION_DIGITS + 8;

/// 任意进制转换缓冲容量 (原版 int; 同 kDoubleToStringMinBufferSize, usize 域)
pub const DOUBLE_TO_RADIX_MAX_CHARS: usize = 2200;

/// DoubleToStringView 结果缓冲最小容量 (原版 int; Rust 消费面为数组维/len 比较, usize 域)
pub const DOUBLE_TO_STRING_MIN_BUFFER_SIZE: usize = 100;

// —— 提取不变式: 两表各 200 项, RADIX_100 组内两字符即两位十进制 ——

/// 头表第 n 组写入位数: n>=10 为 2, 否则 1 (ConvertHeadDigits 语义)
#[inline]
pub const fn radix_100_head_digit_count(n: u8) -> u8 {
  1 + (n >= 10) as u8
}

const _: () = {
  assert!(RADIX_100_TABLE[0] == b'0' && RADIX_100_TABLE[1] == b'0');
  assert!(RADIX_100_TABLE[198] == b'9' && RADIX_100_TABLE[199] == b'9');
  assert!(RADIX_100_HEAD_TABLE[0] == 0 && RADIX_100_HEAD_TABLE[1] == 0);
  assert!(RADIX_100_HEAD_TABLE[20] == b'1' && RADIX_100_HEAD_TABLE[21] == b'0');
  assert!(FP64_SIGN_MASK == 1u64 << 63);
  assert!(FP64_INFINITY == 2047u64 << 52);
  assert!(DOUBLE_TO_FIXED_MAX_CHARS == 123);
  assert!(DOUBLE_TO_RADIX_MAX_CHARS == 2200);
};
