// 由 tool/src/baseline/arm64/baseline-assembler-arm64-inl.h.js 从 v8/src/baseline/arm64/baseline-assembler-arm64-inl.h 生成; 修订就地改, 复跑覆盖

/// arm64 ScratchRegisterScope 特化成员声明 (机械 1:1)
pub const ARM64_SCRATCH_REGISTER_SCOPE_DECLARATIONS: &[&str] = &[
  "[public/method] explicit ScratchRegisterScope(BaselineAssembler* assembler) : assembler_(assembler), prev_scope_(assembler->scratch_register_scope_), wrapped_scope_(assembler->masm())",
  "[public/method] ~ScratchRegisterScope()",
  "[public/method] Register AcquireScratch()",
  "[private/field] BaselineAssembler* assembler_",
  "[private/field] ScratchRegisterScope* prev_scope_",
  "[private/field] UseScratchRegisterScope wrapped_scope_",
];

/// 跳转目标预留指令数 (V8_COMPRESS_POINTERS 开启时为 0, 本目标按非压缩取 1;
/// 原版 #if 分支, 修订阶段按构建特性二选一)
pub const INSTRUCTIONS_PER_JUMP_TARGET: i32 = 1;
/// 每 label 指令数 (原版 instructions_per_label = 1 + instructions_per_jump_target)
pub const INSTRUCTIONS_PER_LABEL: i32 = 1 + INSTRUCTIONS_PER_JUMP_TARGET;

/// 推送/弹出助手模板登记 (CountPushHelper×3 + PushAllHelper×5 + PopAllHelper×3)
pub const HELPER_TEMPLATE_COUNT: usize = 11;
// —— 提取不变式断言 ——
const _: () = {
  assert!(ARM64_SCRATCH_REGISTER_SCOPE_DECLARATIONS.len() == 6);
  assert!(HELPER_TEMPLATE_COUNT == 11);
  assert!(INSTRUCTIONS_PER_LABEL == 2);
};
