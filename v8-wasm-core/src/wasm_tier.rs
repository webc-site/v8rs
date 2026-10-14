// 由 tool/src/wasm/wasm-tier.h.js 从 v8/src/wasm/wasm-tier.h 生成; 修订就地改, 复跑覆盖
// Wasm 执行层枚举 (DRUMBRAKE 按默认关闭展开, 变体登记注释)

/// Wasm 执行层 (对应 enum class ExecutionTier : int8_t)。
///
/// 原版在 V8_ENABLE_DRUMBRAKE 下于 None 与 Liftoff 之间多一个 kInterpreter;
/// 本产物按默认目标配置 (DRUMBRAKE 关闭) 展开, 序号随之下移。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i8)]
pub enum ExecutionTier {
  /// kNone
  None = 0,
  // kInterpreter = 1, // 仅 V8_ENABLE_DRUMBRAKE
  /// kLiftoff (基线层)
  Liftoff = 1,
  /// kTurbofan (优化层)
  Turbofan = 2,
}

impl ExecutionTier {
  /// 名称串 (对应 ExecutionTierToString)
  #[inline]
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
      Self::None => "none",
      Self::Liftoff => "liftoff",
      Self::Turbofan => "turbofan",
    }
  }
}

/// 调试用编译档位 (对应 enum ForDebugging : int8_t)
/// {kForDebugging} 默认降档代码; {kWithBreakpoints} 含断点; {kForStepping} 全量断点
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i8)]
pub enum ForDebugging {
  /// kNotForDebugging
  NotForDebugging = 0,
  /// kForDebugging
  ForDebugging = 1,
  /// kWithBreakpoints
  WithBreakpoints = 2,
  /// kForStepping
  ForStepping = 3,
}

/// 调试状态 (对应 enum DebugState : bool)
pub type DebugState = bool;

/// kNotDebugging
pub const NOT_DEBUGGING: DebugState = false;
/// kDebugging
pub const DEBUGGING: DebugState = true;

// —— 提取不变式断言 ——
const _: () = {
  assert!(ExecutionTier::None as i8 == 0);
  assert!(ExecutionTier::Liftoff as i8 == 1);
  assert!(ExecutionTier::Turbofan as i8 == 2);
  assert!(ForDebugging::NotForDebugging as i8 == 0);
  assert!(ForDebugging::ForDebugging as i8 == 1);
  assert!(ForDebugging::WithBreakpoints as i8 == 2);
  assert!(ForDebugging::ForStepping as i8 == 3);
  assert!(DEBUGGING);
  assert!(!NOT_DEBUGGING);
};
