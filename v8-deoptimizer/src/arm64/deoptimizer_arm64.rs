// 由 tool/src/deoptimizer/arm64/deoptimizer-arm64.cc.js 从 v8/src/deoptimizer/arm64/deoptimizer-arm64.cc 生成; 修订就地改, 复跑覆盖
// arm64 反优化平台常量

// arm64 目标展开; 语义面 (ZapCode/PatchToJump/寄存器读写) 登记跳过。

/// 影子栈自适应偏移 (arm64 无 CET 影子栈, 恒 0)
pub const ADAPT_SHADOW_STACK_OFFSET_TO_SUBTRACT: i32 = 0;

/// 定长指令尺寸 (原 kInstrSize, constants-arm64.h)
pub const INSTR_SIZE: i32 = 4;
/// 急切反优化出口尺寸 (原 kEagerDeoptExitSize = kInstrSize, cc:15)
pub const EAGER_DEOPT_EXIT_SIZE: i32 = 4;
/// 惰性反优化出口尺寸 (原 kLazyDeoptExitSize = 2*kInstrSize, V8_ENABLE_CONTROL_FLOW_INTEGRITY 分支 cc:17)
pub const LAZY_DEOPT_EXIT_SIZE_CFI: i32 = 8;
/// 惰性反优化出口尺寸 (原 kLazyDeoptExitSize = 1*kInstrSize, 目标配置 CFI 关, cc:19)
pub const LAZY_DEOPT_EXIT_SIZE: i32 = 4;

/// PatchToJump 覆写缓冲 (原 kSize; 4 字节 b 指令 + 余量)
pub const PATCH_TO_JUMP_BUFFER_SIZE: i32 = 128;

// 原版登记: ZapCode 平台 TODO(422364570) 未支持, 为空实现。
// PatchToJump: offset = (new_pc - pc) / kInstrSize, 发射 b 指令。

// —— 提取不变式断言 ——
const _: () = {
  assert!(ADAPT_SHADOW_STACK_OFFSET_TO_SUBTRACT == 0);
};
