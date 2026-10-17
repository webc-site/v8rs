// 由 tool/src/execution/x64/frame-constants-x64.js 从 v8/src/execution/x64/frame-constants-x64.h + src/execution/x64/frame-constants-x64.cc 生成; 修订就地改, 复跑覆盖
// x64 帧常量规则代码 (常量表/位标志/宏展开/签名骨架)
// 目标: x64 (V8_TARGET_ARCH_X64); 公共基: kSystemPointerSize=8, kSimd128Size=16, kCPSlotSize=0 (无常量池槽)
// 宏展开: TYPED_FRAME_PUSHED_VALUE_OFFSET(x) = TypedFrameConstants::kFirstPushedFrameValueOffset - x*kSystemPointerSize = -16 - 8x
// 分支: EntryFrameConstants 按 V8_TARGET_OS_WIN 双分支展开; 默认=非 WIN (主目标), WIN 变体入 windows 子模块
// 对齐注记: gn arg v8_x64_16byte_stack_alignment 默认 false → V8_X64_16BYTE_STACK_ALIGNMENT_BOOL=false

/// 公共帧常量基值 (展开自 v8/src/execution/frame-constants.h TypedFrameConstants, x64 口径)
pub const SYSTEM_POINTER_SIZE: i32 = 8;
pub const SIMD128_SIZE: i32 = 16;
/// TypedFrameConstants::kFirstPushedFrameValueOffset (kFixedFrameSizeFromFp=8)
pub const TYPED_FIRST_PUSHED_VALUE_OFFSET: i32 = -16;

/// 原版: class EntryFrameConstants (frame-constants-x64.h)
pub mod entry_frame_constants {
  /// xmm register pushes take 16 bytes on the stack.
  /// On other OS, the callee-saved registers are (in push order):
  /// r12, r13, r14, r15, rbx
  pub const XMM_REGISTER_SIZE: i32 = 16;
  /// Isolate::c_entry_fp onto the stack.
  /// On x64, there are 5 pushq() and 3 Push() calls between setting up rbp and
  /// pushing the c_entry_fp.
  pub const NEXT_EXIT_FRAME_FP_OFFSET: i32 = -64;
  /// This are the offsets to where JSEntry pushes the current values of
  /// IsolateData::fast_c_call_caller_fp and IsolateData::fast_c_call_caller_pc.
  pub const NEXT_FAST_CALL_FRAME_FP_OFFSET: i32 = -72;
  pub const NEXT_FAST_CALL_FRAME_PC_OFFSET: i32 = -80;
}

/// 原版: class WasmLiftoffSetupFrameConstants (frame-constants-x64.h)
pub mod wasm_liftoff_setup_frame_constants {
  /// Number of gp parameters, without the instance.
  pub const NUMBER_OF_SAVED_GP_PARAM_REGS: i32 = 5;
  pub const NUMBER_OF_SAVED_FP_PARAM_REGS: i32 = 6;
  /// There's one spilled value (which doesn't need visiting) below the instance.
  pub const CALLING_PC_OFFSET: i32 = -16;
  pub const INSTANCE_SPILL_OFFSET: i32 = -24;
  pub const PARAMETER_SPILLS_OFFSET: [i32; 5] = [-32, -40, -48, -56, -64];
  /// SP-relative.
  pub const WASM_INSTANCE_DATA_OFFSET: i32 = 16;
  pub const DECLARED_FUNCTION_INDEX_OFFSET: i32 = 8;
  pub const NATIVE_MODULE_OFFSET: i32 = 0;
}

/// 原版: class WasmLiftoffFrameConstants (frame-constants-x64.h)
pub mod wasm_liftoff_frame_constants {
  pub const FEEDBACK_VECTOR_OFFSET: i32 = 24;
  pub const INSTANCE_DATA_OFFSET: i32 = 16;
}

/// 原版: class WasmDebugBreakFrameConstants (frame-constants-x64.h)
pub mod wasm_debug_break_frame_constants {
  use super::{SIMD128_SIZE, SYSTEM_POINTER_SIZE};

  /// 位标志: bit n = 寄存器编码 n (展开自 kPushedGpRegs)
  pub const PUSHED_GP_REGS_BITS: u32 = 0x93cf;
  /// 位标志: bit n = 寄存器编码 n (展开自 kPushedFpRegs)
  pub const PUSHED_FP_REGS_BITS: u32 = 0xff;
  pub const NUM_PUSHED_GP_REGISTERS: i32 = 10;
  pub const NUM_PUSHED_FP_REGISTERS: i32 = 8;
  pub const LAST_PUSHED_GP_REGISTER_OFFSET: i32 = -88;
  pub const LAST_PUSHED_FP_REGISTER_OFFSET: i32 = -216;
  /// Offsets are fp-relative.
  #[inline]
  pub const fn get_pushed_gp_register_offset(reg_code: i32) -> i32 {
    let lower_regs = PUSHED_GP_REGS_BITS & ((1u32 << reg_code) - 1);
    LAST_PUSHED_GP_REGISTER_OFFSET + (lower_regs.count_ones() as i32) * SYSTEM_POINTER_SIZE
  }
  #[inline]
  pub const fn get_pushed_fp_register_offset(reg_code: i32) -> i32 {
    let lower_regs = PUSHED_FP_REGS_BITS & ((1u32 << reg_code) - 1);
    LAST_PUSHED_FP_REGISTER_OFFSET + (lower_regs.count_ones() as i32) * SIMD128_SIZE
  }
}

/// EntryFrameConstants 的 V8_TARGET_OS_WIN 完整展开 (分支内 5 条 + 类公共重算 3 条)
pub mod entry_frame_constants_windows {
  /// 原版 kXMMRegisterSize (类公共条目, 按 WIN 符号表重算)
  pub const XMM_REGISTER_SIZE: i32 = 16;
  /// 原版 kNextFastCallFrameFPOffset (类公共条目, 按 WIN 符号表重算)
  pub const NEXT_FAST_CALL_FRAME_FP_OFFSET: i32 = -248;
  /// 原版 kNextFastCallFramePCOffset (类公共条目, 按 WIN 符号表重算)
  pub const NEXT_FAST_CALL_FRAME_PC_OFFSET: i32 = -256;
  /// 原版 kCalleeSaveXMMRegisters (WIN 分支内条目)
  pub const CALLEE_SAVE_XMM_REGISTERS: i32 = 10;
  /// 原版 kXMMRegistersBlockSize (WIN 分支内条目)
  pub const XMM_REGISTERS_BLOCK_SIZE: i32 = 160;
  /// 原版 kNextExitFrameFPOffset (WIN 分支内条目)
  pub const NEXT_EXIT_FRAME_FP_OFFSET: i32 = -240;
  /// 原版 kArgcOffset (WIN 分支内条目)
  pub const ARGC_OFFSET: i32 = 48;
  /// 原版 kArgvOffset (WIN 分支内条目)
  pub const ARGV_OFFSET: i32 = 56;
}

// ── 签名骨架 (原版 frame-constants-x64.cc 逐条摘录, 零改写) ──
// Register JavaScriptFrame::fp_register() { return rbp; }
// Register JavaScriptFrame::context_register() { return rsi; }
// Register JavaScriptFrame::constant_pool_pointer_register() { UNREACHABLE(); }
// int UnoptimizedFrameConstants::RegisterStackSlotCount(int register_count) {
//   return V8_X64_16BYTE_STACK_ALIGNMENT_BOOL ? RoundUp(register_count, 2)
//                                             : register_count;
// }
// int BuiltinContinuationFrameConstants::PaddingSlotCount(int register_count) {
//   return V8_X64_16BYTE_STACK_ALIGNMENT_BOOL
//              ? (RoundUp(kFixedSlotCount + register_count, 2) -
//                 (kFixedSlotCount + register_count))
//              : 0;
// }
// intptr_t MaglevFrame::StackGuardFrameSize(int register_input_count) {
//   // Include one extra slot for the single argument into StackGuardWithGap +
//   // register input count.
//   return StandardFrameConstants::kFixedFrameSizeFromFp +
//          (1 + register_input_count) * kSystemPointerSize;
// }

// —— 提取不变式断言 ——
const _: () = {
  assert!(SYSTEM_POINTER_SIZE == 8);
  assert!(entry_frame_constants::NEXT_EXIT_FRAME_FP_OFFSET == -64);
  assert!(entry_frame_constants_windows::NEXT_EXIT_FRAME_FP_OFFSET == -240);
  assert!(entry_frame_constants_windows::XMM_REGISTERS_BLOCK_SIZE == 160);
  assert!(entry_frame_constants_windows::NEXT_FAST_CALL_FRAME_FP_OFFSET == -248);
  assert!(entry_frame_constants_windows::NEXT_FAST_CALL_FRAME_PC_OFFSET == -256);
  assert!(wasm_debug_break_frame_constants::PUSHED_GP_REGS_BITS == 0x93cf);
  assert!(wasm_debug_break_frame_constants::NUM_PUSHED_GP_REGISTERS == 10);
  assert!(wasm_debug_break_frame_constants::NUM_PUSHED_FP_REGISTERS == 8);
  assert!(wasm_debug_break_frame_constants::LAST_PUSHED_GP_REGISTER_OFFSET == -88);
  assert!(wasm_debug_break_frame_constants::LAST_PUSHED_FP_REGISTER_OFFSET == -216);
};
