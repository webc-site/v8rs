// 由 tool/src/bigint/util.js 从 v8/src/bigint/util.h 生成; 修订就地改, 复跑覆盖
// 纯规则文件, 整文件脚本所有, 可幂等复跑

//! 通用小工具 (与 BigInt 无关的部分)

/// x 向上取整到 y 的倍数 (y 须为 2 的幂, 原版按位与实现)
#[inline]
pub const fn round_up(x: u32, y: u32) -> u32 {
  (x + y - 1) & y.wrapping_neg()
}

/// 尾随零个数; 0 返回 32 (对齐原版 _BitScanForward 失败路径语义)
#[inline]
pub const fn count_trailing_zeros(value: u32) -> u32 {
  if value == 0 {
    32
  } else {
    value.trailing_zeros()
  }
}

/// 是否 2 的幂
#[inline]
pub const fn is_power_of_two(value: u32) -> bool {
  value > 0 && (value & (value - 1)) == 0
}

/// DIV_CEIL (v8/src/base/macros.h) 的 u64 形态: 向上取整除法; x=0 返回 0
#[inline]
pub const fn div_ceil_u64(x: u64, y: u64) -> u64 {
  if x == 0 { 0 } else { (x - 1) / y + 1 }
}
