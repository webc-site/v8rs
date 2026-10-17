// 由 tool/src/baseline/arm64/baseline-compiler-arm64-inl.h.js 从 v8/src/baseline/arm64/baseline-compiler-arm64-inl.h 生成; 修订就地改, 复跑覆盖

use crate::baseline_compiler::BuiltinCallJumpMode;

/// 短内建调用未启用时的回退模式 (原版 kFallbackBuiltinCallJumpModeForBaseline = kIndirect)
pub const ARM64_FALLBACK_BUILTIN_CALL_JUMP_MODE_FOR_BASELINE: BuiltinCallJumpMode =
  BuiltinCallJumpMode::Indirect;

// —— 提取不变式断言 ——
const _: () = {
  assert!(ARM64_FALLBACK_BUILTIN_CALL_JUMP_MODE_FOR_BASELINE as u8 == 2);
};
