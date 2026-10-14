// 由 tool/src/execution/arm64/frame-constants-arm64.js 从 v8/src/execution/arm64/frame-constants-arm64.h + src/execution/arm64/frame-constants-arm64.cc 生成; 修订就地改, 复跑覆盖
// arm64 帧常量规则代码 (常量表/位标志/宏展开/static_assert 转译/签名骨架)
// 目标: arm64 (V8_TARGET_ARCH_ARM64); 公共基: kSystemPointerSize=8, kSimd128Size=16, kQRegSize=16, kCPSlotSize=0 (无常量池槽)
// 宏展开: TYPED_FRAME_PUSHED_VALUE_OFFSET(x) = TypedFrameConstants::kFirstPushedFrameValueOffset - x*kSystemPointerSize = -16 - 8x
// 注: 本文件无 OS/平台分支; kLastPushedGpRegisterOffset 含 RoundUp<16> 头部对齐展开 (MacroAssembler::EnterFrame 16 字节垫)

/// 公共帧常量基值 (展开自 v8/src/execution/frame-constants.h TypedFrameConstants, arm64 口径)
pub const SYSTEM_POINTER_SIZE: i32 = 8;
pub const SIMD128_SIZE: i32 = 16;
/// 128-bit Q 寄存器尺寸 (原版 kQRegSize)
pub const Q_REG_SIZE: i32 = 16;
/// TypedFrameConstants::kFirstPushedFrameValueOffset (kFixedFrameSizeFromFp=8)
pub const TYPED_FIRST_PUSHED_VALUE_OFFSET: i32 = -16;

/// 原版: class EntryFrameConstants (frame-constants-arm64.h)
pub mod entry_frame_constants {
  /// This is the offset to where JSEntry pushes the current value of
  /// Isolate::c_entry_fp onto the stack.
  pub const NEXT_EXIT_FRAME_FP_OFFSET: i32 = -24;
  /// The offsets for storing the FP and PC of fast API calls.
  pub const NEXT_FAST_CALL_FRAME_FP_OFFSET: i32 = -40;
  pub const NEXT_FAST_CALL_FRAME_PC_OFFSET: i32 = -48;
  pub const FIXED_FRAME_SIZE: i32 = 48;
  /// The following constants are defined so we can static-assert their values
  /// near the relevant JSEntry assembly code, not because they're actually very
  /// useful.
  pub const CALLEE_SAVED_REGISTER_BYTES_PUSHED_BEFORE_FP_LR_PAIR: i32 = 144;
  pub const CALLEE_SAVED_REGISTER_BYTES_PUSHED_AFTER_FP_LR_PAIR: i32 = 0;
  pub const OFFSET_TO_CALLEE_SAVED_REGISTERS: i32 = 0;
  /// These offsets refer to the immediate caller (a native frame), not to the
  /// previous JS exit frame like kCallerFPOffset above.
  pub const DIRECT_CALLER_FP_OFFSET: i32 = 0;
  pub const DIRECT_CALLER_PC_OFFSET: i32 = 8;
  pub const DIRECT_CALLER_SP_OFFSET: i32 = 160;
}

/// 原版: class WasmLiftoffSetupFrameConstants (frame-constants-arm64.h)
pub mod wasm_liftoff_setup_frame_constants {
  /// Number of gp parameters, without the instance.
  pub const NUMBER_OF_SAVED_GP_PARAM_REGS: i32 = 6;
  pub const NUMBER_OF_SAVED_FP_PARAM_REGS: i32 = 8;
  ///   x0, x2, x3, x4, x5, x6: param1, param2, ..., param6
  /// in the following FP-relative order: [x6, x5, x4, x3, x2, x0].
  /// The instance slot is in position '0', the first spill slot is at '1'.
  pub const INSTANCE_SPILL_OFFSET: i32 = -16;
  pub const CALLING_PC_OFFSET: i32 = -200;
  pub const PARAMETER_SPILLS_OFFSET: [i32; 6] = [-64, -56, -48, -40, -32, -24];
  /// SP-relative.
  pub const WASM_INSTANCE_DATA_OFFSET: i32 = 16;
  pub const DECLARED_FUNCTION_INDEX_OFFSET: i32 = 8;
  pub const NATIVE_MODULE_OFFSET: i32 = 0;
}

/// 原版: class WasmLiftoffFrameConstants (frame-constants-arm64.h)
pub mod wasm_liftoff_frame_constants {
  pub const FEEDBACK_VECTOR_OFFSET: i32 = 24;
  pub const INSTANCE_DATA_OFFSET: i32 = 16;
}

/// 原版: class WasmDebugBreakFrameConstants (frame-constants-arm64.h)
pub mod wasm_debug_break_frame_constants {
  use super::{SIMD128_SIZE, SYSTEM_POINTER_SIZE};

  /// x16: ip0, x17: ip1, x18: platform register, x26: root, x28: base, x29: fp,
  /// x30: lr, x31: xzr.
  /// 位标志: bit n = 寄存器编码 n (展开自 kPushedGpRegs)
  pub const PUSHED_GP_REGS_BITS: u32 = 0xbf8ffff;
  /// We push FpRegs as 128-bit SIMD registers, so 16-byte frame alignment
  /// is guaranteed regardless of register count.
  /// 位标志: bit n = 寄存器编码 n (展开自 kPushedFpRegs)
  pub const PUSHED_FP_REGS_BITS: u32 = 0xfff7fff;
  pub const NUM_PUSHED_GP_REGISTERS: i32 = 24;
  pub const NUM_PUSHED_FP_REGISTERS: i32 = 27;
  pub const LAST_PUSHED_GP_REGISTER_OFFSET: i32 = -208;
  pub const LAST_PUSHED_FP_REGISTER_OFFSET: i32 = -640;
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

// ── 签名骨架 (原版 frame-constants-arm64.cc 逐条摘录, 零改写) ──
// Register JavaScriptFrame::fp_register() { return v8::internal::fp; }
// Register JavaScriptFrame::context_register() { return cp; }
// Register JavaScriptFrame::constant_pool_pointer_register() { UNREACHABLE(); }
// int UnoptimizedFrameConstants::RegisterStackSlotCount(int register_count) {
//   static_assert(InterpreterFrameConstants::kFixedFrameSize % 16 == 0);
//   // Round up to a multiple of two, to make the frame a multiple of 16 bytes.
//   return RoundUp(register_count, 2);
// }
// int BuiltinContinuationFrameConstants::PaddingSlotCount(int register_count) {
//   // Round the total slot count up to a multiple of two, to make the frame a
//   // multiple of 16 bytes.
//   int slot_count = kFixedSlotCount + register_count;
//   int rounded_slot_count = RoundUp(slot_count, 2);
//   return rounded_slot_count - slot_count;
// }
// intptr_t MaglevFrame::StackGuardFrameSize(int register_input_count) {
//   // Include any paddings from kFixedFrameSizeFromFp, an extra slot + padding
//   // for the single argument into StackGuardWithGap and finally padded register
//   // input count.
//   int slot_count = RoundUp(StandardFrameConstants::kFixedSlotCountFromFp, 2) +
//                    2 /* argument */ + RoundUp(register_input_count, 2);
//   return slot_count * kSystemPointerSize;
// }

// —— 提取不变式断言 (含原版 static_assert 转译) ——
const _: () = {
  assert!(SYSTEM_POINTER_SIZE == 8);
  // 原版: static_assert(kQRegSize == 2 * kSystemPointerSize);
  assert!(Q_REG_SIZE == 2 * SYSTEM_POINTER_SIZE);
  // 原版: static_assert(kNumPushedGpRegisters % 2 == 0);
  assert!(wasm_debug_break_frame_constants::NUM_PUSHED_GP_REGISTERS % 2 == 0);
  assert!(entry_frame_constants::NEXT_EXIT_FRAME_FP_OFFSET == -24);
  assert!(entry_frame_constants::DIRECT_CALLER_SP_OFFSET == 160);
  assert!(wasm_liftoff_setup_frame_constants::CALLING_PC_OFFSET == -200);
  assert!(wasm_debug_break_frame_constants::PUSHED_GP_REGS_BITS == 0xbf8ffff);
  assert!(wasm_debug_break_frame_constants::PUSHED_FP_REGS_BITS == 0xfff7fff);
  assert!(wasm_debug_break_frame_constants::NUM_PUSHED_GP_REGISTERS == 24);
  assert!(wasm_debug_break_frame_constants::NUM_PUSHED_FP_REGISTERS == 27);
  assert!(wasm_debug_break_frame_constants::LAST_PUSHED_GP_REGISTER_OFFSET == -208);
  assert!(wasm_debug_break_frame_constants::LAST_PUSHED_FP_REGISTER_OFFSET == -640);
};
