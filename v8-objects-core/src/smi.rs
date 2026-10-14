use crate::{
  tagged::{Object, Tagged},
  tagged_constants::{SMI_TAG, smi31, smi32},
};

/// 小整数 (Smi) 标记类型
///
/// V8 的 Smi 是直接内联存储在指针中的有符号整数（不走堆分配）。
/// 原始指针数值的最低位 Tag 为 0。
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Smi;

impl Smi {
  /// Smi 可表示的最小值 (-2^31)
  pub const MIN_VALUE: i32 = smi32::MIN_VALUE;

  /// Smi 可表示的最大值 (2^31 - 1)
  pub const MAX_VALUE: i32 = smi32::MAX_VALUE;

  /// 31 位 Smi 可表示的最小值 (-2^30)
  pub const MIN_VALUE_31: i32 = smi31::MIN_VALUE;

  /// 31 位 Smi 可表示的最大值 (2^30 - 1)
  pub const MAX_VALUE_31: i32 = smi31::MAX_VALUE;

  /// 构造数值为 0 的 Smi Tagged 指针
  #[inline]
  pub const fn zero() -> Tagged<Self> {
    Tagged::new(SMI_TAG)
  }

  /// 将 32 位有符号整数编码为 Tagged<Smi> 指针
  ///
  /// 在 64 位架构下，32 位数值左移 32 位，最低位存储 Smi Tag (0)
  #[inline]
  pub const fn from_int(value: i32) -> Tagged<Self> {
    let raw = ((value as i64 as u64) << smi32::SHIFT_BITS) | (SMI_TAG as u64);
    Tagged::new(raw as usize)
  }

  /// 从有符号长整数安全构造 Smi，若溢出则返回 None
  #[inline]
  pub const fn try_from_i64(value: i64) -> Option<Tagged<Self>> {
    if Self::is_valid(value) {
      Some(Self::from_int(value as i32))
    } else {
      None
    }
  }

  /// 解包 Smi 指针并还原为 32 位有符号整数
  ///
  /// # 符号扩展
  /// 算术右移 32 位，高 32 位严格进行符号位保留扩展。
  #[inline]
  pub const fn value(smi: Tagged<Self>) -> i32 {
    debug_assert!(smi.is_smi(), "value() 必须在 Smi 指针上调用");
    ((smi.ptr() as i64) >> smi32::SHIFT_BITS) as i32
  }

  /// 检查 64 位有符号整数是否可以无损放入 32 位 Smi
  #[inline]
  pub const fn is_valid(value: i64) -> bool {
    Self::is_valid_32(value)
  }

  /// 检查整数是否可以表示为 32 位 Smi
  #[inline]
  pub const fn is_valid_32(value: i64) -> bool {
    value >= Self::MIN_VALUE as i64 && value <= Self::MAX_VALUE as i64
  }

  /// 检查无符号 64 位整数是否可以表示为 32 位 Smi
  #[inline]
  pub const fn is_valid_u64_32(value: u64) -> bool {
    value <= Self::MAX_VALUE as u64
  }

  /// 检查整数是否可以表示为 31 位 Smi (指针压缩或 32 位模式)
  #[inline]
  pub const fn is_valid_31(value: i64) -> bool {
    value >= Self::MIN_VALUE_31 as i64 && value <= Self::MAX_VALUE_31 as i64
  }

  /// 检查无符号整数是否可以表示为 31 位 Smi
  #[inline]
  pub const fn is_valid_u64_31(value: u64) -> bool {
    value <= Self::MAX_VALUE_31 as u64
  }

  /// 构造 31 位模式编码的 Tagged<Smi> (用于指针压缩测试与场景)
  #[inline]
  pub const fn from_int31(value: i32) -> Tagged<Self> {
    debug_assert!(
      value >= Self::MIN_VALUE_31 && value <= Self::MAX_VALUE_31,
      "数值超出 31 位 Smi 范围"
    );
    let raw = ((value as u32) << smi31::SHIFT_BITS) | (SMI_TAG as u32);
    Tagged::new(raw as usize)
  }

  /// 解码 31 位模式的 Smi 数值
  #[inline]
  pub const fn value_31(smi: Tagged<Self>) -> i32 {
    debug_assert!(smi.is_smi());
    (smi.ptr() as i32) >> smi31::SHIFT_BITS
  }

  /// 限制并转换为正整数 Smi
  #[inline]
  pub const fn to_uint32_smi(smi: Tagged<Self>) -> Tagged<Self> {
    let val = Self::value(smi);
    if val <= 0 {
      Self::zero()
    } else {
      Self::from_int(val)
    }
  }

  /// Given {value} in [0, 2^31-1], force it into Smi range (对应 From31BitPattern)
  ///
  /// 本 crate 为 64 位非压缩模型 (kSmiValueSize=32), 原版移位表达式
  /// `value << (32 - 32) >> (32 - 32)` 为恒等: 输入域 [0, 2^31-1] 已全部落在
  /// 32 位 Smi 值域内, 无需改变任何位。31 位压缩配置的截断语义由
  /// [`Self::from_int31`] 显式承担。
  #[inline]
  pub const fn from_31bit_pattern(value: i32) -> Tagged<Self> {
    debug_assert!(value >= 0, "From31BitPattern 输入约定 [0, 2^31-1]");
    Self::from_int(value)
  }

  /// 逆运算还原 31-bit pattern
  #[inline]
  pub const fn to_31bit_pattern(smi: Tagged<Self>) -> u32 {
    (Self::value(smi) as u32) & 0x7FFF_FFFF
  }
}

/// 31 位 Smi 静态检查与编码辅助 (对应 C++ SmiTagging<4>)
pub struct SmallSmi;

impl SmallSmi {
  #[inline]
  pub const fn is_valid_i64(value: i64) -> bool {
    Smi::is_valid_31(value)
  }

  #[inline]
  pub const fn is_valid_u64(value: u64) -> bool {
    Smi::is_valid_u64_31(value)
  }
}

/// 32 位 Smi 静态检查与编码辅助 (对应 C++ SmiTagging<8>)
pub struct LargeSmi;

impl LargeSmi {
  #[inline]
  pub const fn is_valid_i64(value: i64) -> bool {
    Smi::is_valid_32(value)
  }

  #[inline]
  pub const fn is_valid_u64(value: u64) -> bool {
    Smi::is_valid_u64_32(value)
  }
}

// ============================================================================
// 转换辅助 + 字典序比较
// 来源: v8/src/objects/smi.h + objects.cc Smi::LexicographicCompare(6482-6559)
// (由 tool/src/objects/smi.h.js 生成差集骨架后就地修订)
// 命名映射登记: C++ FromIntptr/FromEnum → 已有 try_from_i64/from_int 直接承担, 不再另设
// ============================================================================

impl Smi {
  /// 任意 Tagged<Object> → int (对应 ToInt; 必须实际是 Smi)
  #[inline]
  pub const fn to_int(object: Tagged<Object>) -> i32 {
    Self::value(object.cast())
  }

  /// 正 Smi → u32 (对应 ToUInt, debug 断言非负)
  #[inline]
  pub const fn to_uint(object: Tagged<Object>) -> u32 {
    let v = Self::to_int(object);
    debug_assert!(v >= 0, "ToUInt 只接受正 Smi");
    v as u32
  }

  /// [0, kSmiMaxValue] → Smi (对应 FromUInt)
  #[inline]
  pub const fn from_uint(value: u32) -> Tagged<Self> {
    debug_assert!(Self::is_valid_u64_32(value as u64), "u32 超出 Smi 值域");
    Self::from_int(value as i32)
  }

  /// 未初始化反序列化哨兵 = kNullAddress (对应 uninitialized_deserialization_value)
  ///
  /// 该值必须为 0: 未初始化的内嵌指针字段读出来要被当作 nullptr 跳过。
  #[inline]
  pub const fn uninitialized_deserialization_value() -> Tagged<Self> {
    Tagged::new(0)
  }

  /// 字典序比较两个 Smi 的十进制字符串表示 (对应 LexicographicCompare;
  /// 原版 isolate 参数仅用于禁 GC/JS 执行, 与算法无关, 省略)
  ///
  /// 返回值为 -1/0/1 的 Smi (x < y / x == y / x > y)。
  pub const fn lexicographic_compare(x: Tagged<Self>, y: Tagged<Self>) -> Tagged<Self> {
    let x_value = Self::value(x);
    let y_value = Self::value(y);

    // 数值相等则字符串表示相等
    if x_value == y_value {
      return Self::from_int(0);
    }

    // 一方为 0 时, 数值序即字符串字典序
    if x_value == 0 || y_value == 0 {
      return Self::from_int(if x_value < y_value { -1 } else { 1 });
    }

    // 仅一方为负: '-' 字符码小于任何数字, 负数在前
    // 用无符号值避免 32 位 Smi 下 -MIN_INT 的溢出问题
    let mut x_scaled = x_value as u32;
    let mut y_scaled = y_value as u32;
    if x_value < 0 {
      if y_value >= 0 {
        return Self::from_int(-1);
      }
      y_scaled = y_scaled.wrapping_neg();
      x_scaled = x_scaled.wrapping_neg();
    } else if y_value < 0 {
      return Self::from_int(1);
    }

    // 位数相同 → 数值序即字典序; 位数不同 → 短的一方乘 10 的幂补齐,
    // 长的一方右移一位 (比较长度之外的最后一位无意义)
    let x_log10 = Self::log10(x_scaled);
    let y_log10 = Self::log10(y_scaled);
    let mut tie = 0;

    if x_log10 < y_log10 {
      x_scaled *= POWERS_OF_10[(y_log10 - x_log10 - 1) as usize];
      y_scaled /= 10;
      tie = -1;
    } else if y_log10 < x_log10 {
      y_scaled *= POWERS_OF_10[(x_log10 - y_log10 - 1) as usize];
      x_scaled /= 10;
      tie = 1;
    }

    if x_scaled < y_scaled {
      return Self::from_int(-1);
    }
    if x_scaled > y_scaled {
      return Self::from_int(1);
    }
    Self::from_int(tie)
  }

  /// 十进制位数 - 1 (graphics.stanford.edu IntegerLog10 位技巧)
  #[inline]
  const fn log10(v: u32) -> i32 {
    let log2 = 31 - v.leading_zeros() as i32;
    let mut log10 = ((log2 + 1) * 1233) >> 12;
    log10 -= (v < POWERS_OF_10[log10 as usize]) as i32;
    log10
  }
}

/// 10 的幂表 (objects.cc:6482 机械提取核对)
const POWERS_OF_10: [u32; 10] = [
  1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
];
