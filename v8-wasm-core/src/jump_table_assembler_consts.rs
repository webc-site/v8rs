// 由 tool/src/wasm/jump-table-assembler.h.js 从 v8/src/wasm/jump-table-assembler.h 生成; 修订就地改, 复跑覆盖
// 跳转表槽位布局常量 (x64+arm64 分支展开; 其余架构跳过: ia32/arm/s390x/ppc/mips64/riscv)

/// x64_cet_ibt (V8_TARGET_ARCH_X64 + V8_ENABLE_CET_IBT)
pub mod x64_cet_ibt {
  /// 原版 kJumpTableLineSize
  pub const JUMP_TABLE_LINE_SIZE: i32 = 16;
  /// 原版 kJumpTableSlotSize
  pub const JUMP_TABLE_SLOT_SIZE: i32 = 16;
  /// 原版 kJumpTableSlotEntryMarkerSize
  pub const JUMP_TABLE_SLOT_ENTRY_MARKER_SIZE: i32 = 8;
  /// 原版 kFarJumpTableSlotSize
  pub const FAR_JUMP_TABLE_SLOT_SIZE: i32 = 16;
  /// 原版 kLazyCompileTableSlotSize
  pub const LAZY_COMPILE_TABLE_SLOT_SIZE: i32 = 10;
}

/// x64 (V8_TARGET_ARCH_X64, 无 CET_IBT)
pub mod x64 {
  /// 原版 kJumpTableLineSize
  pub const JUMP_TABLE_LINE_SIZE: i32 = 8;
  /// 原版 kJumpTableSlotSize
  pub const JUMP_TABLE_SLOT_SIZE: i32 = 8;
  /// 原版 kJumpTableSlotEntryMarkerSize
  pub const JUMP_TABLE_SLOT_ENTRY_MARKER_SIZE: i32 = 0;
  /// 原版 kFarJumpTableSlotSize
  pub const FAR_JUMP_TABLE_SLOT_SIZE: i32 = 16;
  /// 原版 kLazyCompileTableSlotSize
  pub const LAZY_COMPILE_TABLE_SLOT_SIZE: i32 = 10;
}

/// arm64_cfi (V8_TARGET_ARCH_ARM64 + V8_ENABLE_CONTROL_FLOW_INTEGRITY; kInstrSize=4)
pub mod arm64_cfi {
  /// 原版 kJumpTableLineSize
  pub const JUMP_TABLE_LINE_SIZE: i32 = 8;
  /// 原版 kJumpTableSlotSize
  pub const JUMP_TABLE_SLOT_SIZE: i32 = 8;
  /// 原版 kFarJumpTableSlotSize
  pub const FAR_JUMP_TABLE_SLOT_SIZE: i32 = 16;
  /// 原版 kLazyCompileTableSlotSize
  pub const LAZY_COMPILE_TABLE_SLOT_SIZE: i32 = 16;
}

/// arm64 (V8_TARGET_ARCH_ARM64, 无 CFI)
pub mod arm64 {
  /// 原版 kJumpTableLineSize
  pub const JUMP_TABLE_LINE_SIZE: i32 = 4;
  /// 原版 kJumpTableSlotSize
  pub const JUMP_TABLE_SLOT_SIZE: i32 = 4;
  /// 原版 kFarJumpTableSlotSize
  pub const FAR_JUMP_TABLE_SLOT_SIZE: i32 = 16;
  /// 原版 kLazyCompileTableSlotSize
  pub const LAZY_COMPILE_TABLE_SLOT_SIZE: i32 = 16;
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(x64_cet_ibt::JUMP_TABLE_SLOT_SIZE == 16);
  assert!(x64::JUMP_TABLE_SLOT_SIZE == 8);
  assert!(x64::JUMP_TABLE_LINE_SIZE == 8);
  assert!(arm64_cfi::JUMP_TABLE_SLOT_SIZE == 8);
  assert!(arm64::JUMP_TABLE_SLOT_SIZE == 4);
  assert!(arm64::LAZY_COMPILE_TABLE_SLOT_SIZE == 16);
};
