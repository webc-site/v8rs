// 由 tool/src/ast/variables.h.js 从 v8/src/ast/variables.h 生成; 修订就地改, 复跑覆盖
// 变量层空洞检测枚举、位标志与位图常量

use strum::{Display, FromRepr};

/// 空洞检测状态 (原版 enum class Variable::HoleCheckState)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
#[repr(u8)]
pub enum HoleCheckState {
  Uncached = 0,
  Force = 1,
  Skip = 2,
}

/// 空洞检测位图 (原版 using HoleCheckBitmap)
pub type HoleCheckBitmap = u64;

/// 空洞初始化强制位标志 (原版 enum ForceHoleInitializationFlag)
pub mod force_hole_init {
  /// 原版 kHoleInitializationNotForced = 0
  pub const HOLE_INITIALIZATION_NOT_FORCED: u8 = 0;
  /// 原版 kHasHoleCheckUseInDifferentClosureScope = 1
  pub const HAS_HOLE_CHECK_USE_IN_DIFFERENT_CLOSURE_SCOPE: u8 = 1;
  /// 原版 kHasHoleCheckUseInSameClosureScope = 2
  pub const HAS_HOLE_CHECK_USE_IN_SAME_CLOSURE_SCOPE: u8 = 2;
  /// 原版 kHasHoleCheckUseInUnknownScope = 3
  pub const HAS_HOLE_CHECK_USE_IN_UNKNOWN_SCOPE: u8 = 3;
}

/// 不可缓存空洞检测位图索引 (原版 kUncacheableHoleCheckBitmapIndex)
pub const UNCACHEABLE_HOLE_CHECK_BITMAP_INDEX: u8 = 0;

/// 位图可用位数 (原版 kHoleCheckBitmapBits = numeric_limits<HoleCheckBitmap>::digits)
pub const HOLE_CHECK_BITMAP_BITS: u8 = 64;

// ── 对数不变式断言 ──
const _: () = {
  assert!(HoleCheckState::Uncached as u8 == 0);
  assert!(force_hole_init::HOLE_INITIALIZATION_NOT_FORCED == 0);
  assert!(force_hole_init::HAS_HOLE_CHECK_USE_IN_DIFFERENT_CLOSURE_SCOPE == 1);
  assert!(force_hole_init::HAS_HOLE_CHECK_USE_IN_SAME_CLOSURE_SCOPE == 2);
  assert!(force_hole_init::HAS_HOLE_CHECK_USE_IN_UNKNOWN_SCOPE == 3);
  assert!(UNCACHEABLE_HOLE_CHECK_BITMAP_INDEX == 0);
  assert!(HOLE_CHECK_BITMAP_BITS == 64);
  assert!(HOLE_CHECK_BITMAP_BITS == <HoleCheckBitmap>::BITS as u8);
};
