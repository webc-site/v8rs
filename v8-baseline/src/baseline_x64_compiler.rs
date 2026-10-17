// 由 tool/src/baseline/x64/baseline-compiler-x64-inl.h.js 从 v8/src/baseline/x64/baseline-compiler-x64-inl.h 生成; 修订就地改, 复跑覆盖

use crate::baseline_compiler::BuiltinCallJumpMode;

/// 短内建调用未启用时的回退模式 (原版 kFallbackBuiltinCallJumpModeForBaseline = kIndirect)
pub const X64_FALLBACK_BUILTIN_CALL_JUMP_MODE_FOR_BASELINE: BuiltinCallJumpMode =
  BuiltinCallJumpMode::Indirect;

/// PrologueFillFrame 循环展开宽度 (原版 kLoopUnrollSize)
pub const LOOP_UNROLL_SIZE: i32 = 8;

// —— 提取不变式断言 ——
const _: () = {
  assert!(X64_FALLBACK_BUILTIN_CALL_JUMP_MODE_FOR_BASELINE as u8 == 2);
  assert!(LOOP_UNROLL_SIZE == 8);
};
