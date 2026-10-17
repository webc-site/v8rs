// 由 tool/src/deoptimizer/x64/deoptimizer-x64.cc.js 从 v8/src/deoptimizer/x64/deoptimizer-x64.cc 生成; 修订就地改, 复跑覆盖
// x64 反优化平台常量

// x64 目标展开; 语义面 (ZapCode/PatchToJump/寄存器读写) 登记跳过。

/// 急切反优化出口尺寸 (原 kEagerDeoptExitSize)
pub const EAGER_DEOPT_EXIT_SIZE: i32 = 4;

/// 惰性反优化出口尺寸双分支:
/// - CET IBT 开启 (额外 endbr64): 8
/// - 关闭: 4
pub const LAZY_DEOPT_EXIT_SIZE_CET_IBT: i32 = 8;
pub const LAZY_DEOPT_EXIT_SIZE: i32 = 4;

/// 影子栈自适应偏移双分支 (CET shadow stack):
/// - 开启: 7
/// - 关闭: 0
pub const ADAPT_SHADOW_STACK_OFFSET_TO_SUBTRACT_CET: i32 = 7;
pub const ADAPT_SHADOW_STACK_OFFSET_TO_SUBTRACT: i32 = 0;

/// PatchToJump 覆写缓冲 (原 kSize)
pub const PATCH_TO_JUMP_BUFFER_SIZE: i32 = 32;
/// ZapCode 缓冲余量 (原 kExtra)
pub const ZAP_CODE_EXTRA: i32 = 32;

// 原版 ASSERT_OFFSET 断言锚: tier0 入口表 + DeoptimizationEntry_Eager/Lazy
// × kSystemPointerSize <= 0x7F (IsolateData 布局保证)。

// —— 提取不变式断言 ——
const _: () = {
  assert!(EAGER_DEOPT_EXIT_SIZE == 4);
  assert!(LAZY_DEOPT_EXIT_SIZE == 4);
};
