// 由 tool/src/objects/swiss-hash-table-helpers.tq.js 从 v8/src/objects/swiss-hash-table-helpers.{tq,h} 生成; 修订就地改, 复跑覆盖
// swiss 表控制字节与探测常量 (SIMD 分支)
//! swiss_table 辅助常量 (Torque 侧 generates 的 C++ 取值)。
//! 目标平台: x64 (SSE2) / arm64 (NEON) → SIMD Group 路径 (kWidth=16);
//! portable Group (kWidth=8) 配置分支登记跳过。

/// 控制字节: 空槽 (原版 Ctrl::kEmpty = -128 即 0b10000000)。
pub const K_EMPTY: u8 = 0x80;
/// 控制字节: 已删除槽 (原版 Ctrl::kDeleted = -2 即 0b11111110)。
pub const K_DELETED: u8 = 0xFE;
/// H2 (7 位哈希摘要) 位宽。
pub const K_H2_BITS: u32 = 7;
/// H2 掩码 (原版 (1 << kH2Bits) - 1)。
pub const K_H2_MASK: u32 = 0x7F;
/// 组宽 (SIMD 路径: 每组槽数 = 16)。
pub const K_GROUP_WIDTH: usize = 16;
/// 是否 SIMD 路径 (kWidth == 16)。
pub const K_USE_SIMD: bool = true;

/// portable 路径组宽 (登记: 目标配置不使用)。
pub const PORTABLE_GROUP_WIDTH: usize = 8;

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_EMPTY == 0x80);
  assert!(K_DELETED == 0xFE);
  assert!(K_H2_BITS == 7);
  assert!(K_H2_MASK == 0x7F);
  assert!(K_GROUP_WIDTH == 16);
  assert!(K_USE_SIMD);
};
