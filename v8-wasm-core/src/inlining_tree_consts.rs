// 由 tool/src/wasm/inlining-tree.h.js 从 v8/src/wasm/inlining-tree.h 生成; 修订就地改, 复跑覆盖
// Wasm 内联决策树常量

/// 内联树的存储形态 (对应原版 InliningTree::Mode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InliningMode {
  /// kUninitialized
  Uninitialized = 0,
  /// kVector
  Vector = 1,
  /// kMap
  Map = 2,
}

/// 内联函数数量上限 (原版 kMaxInlinedCount)
pub const MAX_INLINED_COUNT: i32 = 60;

/// 内联嵌套深度上限 (原版 kMaxInliningNestingDepth)
pub const MAX_INLINING_NESTING_DEPTH: u32 = 7;

/// budget 调整: Turboshaft 每调用点扣减 (原版 kTurboshaftAdjustment)
pub const TURBOSHAFT_ADJUSTMENT: i32 = 2;

/// budget 低于此值不再内联 (原版 kLowestUsefulValue)
pub const LOWEST_USEFUL_VALUE: i32 = 2;

/// budget 折算系数 (原版 kTurboshaftCorrectionFactor)
pub const TURBOSHAFT_CORRECTION_FACTOR: f64 = 1.2;

/// 一次调用的预估节省 wire-bytes (原版 kOneLessCall, 估算值)
pub const ONE_LESS_CALL: i32 = 6;

// —— 提取不变式断言 ——
const _: () = {
  assert!(MAX_INLINED_COUNT == 60);
  assert!(MAX_INLINING_NESTING_DEPTH == 7);
  assert!(TURBOSHAFT_CORRECTION_FACTOR > 1.0);
  // 原版 inlining-tree.h: kLowestUsefulValue = kTurboshaftAdjustment = 2 (相等, 非小于)
  assert!(LOWEST_USEFUL_VALUE == 2);
  assert!(TURBOSHAFT_ADJUSTMENT == 2);
};
