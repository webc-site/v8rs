// 由 tool/src/regexp/x64/regexp-macro-assembler-x64.h.js 从 v8/src/regexp/x64/regexp-macro-assembler-x64.h 生成; 修订就地改, 复跑覆盖
// x64 正则匹配栈帧布局 (rbp 相对偏移; SysV/Win 两态展开)

/// sysv (V8_TARGET_OS_WIN 关; AMD64 ABI, 参数走寄存器)
pub mod sysv {
  pub const FRAME_POINTER_OFFSET: i32 = 0;
  /// 原版 kReturnAddressOffset
  pub const RETURN_ADDRESS_OFFSET: i32 = 8;
  /// 原版 kFrameTypeOffset (帧类型标记, 位于 rbp 下方)
  pub const FRAME_TYPE_OFFSET: i32 = -8;
  /// 原版 kFrameAlign
  pub const FRAME_ALIGN: i32 = 16;
  /// 原版 kInputStringOffset
  pub const INPUT_STRING_OFFSET: i32 = -16;
  /// 原版 kStartIndexOffset
  pub const START_INDEX_OFFSET: i32 = -24;
  /// 原版 kInputStartOffset
  pub const INPUT_START_OFFSET: i32 = -32;
  /// 原版 kInputEndOffset
  pub const INPUT_END_OFFSET: i32 = -40;
  /// 原版 kRegisterOutputOffset
  pub const REGISTER_OUTPUT_OFFSET: i32 = -48;
  /// 原版 kNumOutputRegistersOffset
  pub const NUM_OUTPUT_REGISTERS_OFFSET: i32 = -56;
  /// 原版 kDirectCallOffset
  pub const DIRECT_CALL_OFFSET: i32 = 16;
  /// 原版 kIsolateOffset
  pub const ISOLATE_OFFSET: i32 = 24;
  /// 原版 kBackupRbxOffset
  pub const BACKUP_RBX_OFFSET: i32 = -64;
  /// 原版 kBackupR12Offset
  pub const BACKUP_R12_OFFSET: i32 = -72;
  /// 原版 kNumCalleeSaveRegisters
  pub const NUM_CALLEE_SAVE_REGISTERS: i32 = 2;
  /// 原版 kLastCalleeSaveRegister
  pub const LAST_CALLEE_SAVE_REGISTER: i32 = -72;
  /// 原版 kSuccessfulCapturesOffset
  pub const SUCCESSFUL_CAPTURES_OFFSET: i32 = -80;
  /// 原版 kStringStartMinusOneOffset
  pub const STRING_START_MINUS_ONE_OFFSET: i32 = -88;
  /// 原版 kBacktrackCountOffset
  pub const BACKTRACK_COUNT_OFFSET: i32 = -96;
  /// 原版 kRegExpStackBasePointerOffset
  pub const REGEXP_STACK_BASE_POINTER_OFFSET: i32 = -104;
  /// 原版 kRegisterZeroOffset
  pub const REGISTER_ZERO_OFFSET: i32 = -112;

  /// 正则内部寄存器 (register_index) 在栈上的 rbp 偏移
  /// 对应原版 register_location(index) 寻址：kRegisterZeroOffset - index * 8
  #[inline]
  pub const fn register_location_offset(register_index: i32) -> i32 {
    REGISTER_ZERO_OFFSET - register_index * 8
  }
}

/// win (V8_TARGET_OS_WIN 开; 参数在调用者栈上, 备份 rsi/rdi/rbx/r12)
pub mod win {
  pub const FRAME_POINTER_OFFSET: i32 = 0;
  /// 原版 kReturnAddressOffset
  pub const RETURN_ADDRESS_OFFSET: i32 = 8;
  /// 原版 kFrameTypeOffset (帧类型标记, 位于 rbp 下方)
  pub const FRAME_TYPE_OFFSET: i32 = -8;
  /// 原版 kFrameAlign
  pub const FRAME_ALIGN: i32 = 16;
  /// 原版 kInputStringOffset
  pub const INPUT_STRING_OFFSET: i32 = 16;
  /// 原版 kStartIndexOffset
  pub const START_INDEX_OFFSET: i32 = 24;
  /// 原版 kInputStartOffset
  pub const INPUT_START_OFFSET: i32 = 32;
  /// 原版 kInputEndOffset
  pub const INPUT_END_OFFSET: i32 = 40;
  /// 原版 kRegisterOutputOffset
  pub const REGISTER_OUTPUT_OFFSET: i32 = 48;
  /// 原版 kNumOutputRegistersOffset
  pub const NUM_OUTPUT_REGISTERS_OFFSET: i32 = 56;
  /// 原版 kDirectCallOffset
  pub const DIRECT_CALL_OFFSET: i32 = 64;
  /// 原版 kIsolateOffset
  pub const ISOLATE_OFFSET: i32 = 72;
  /// 原版 kBackupRsiOffset
  pub const BACKUP_RSI_OFFSET: i32 = -16;
  /// 原版 kBackupRdiOffset
  pub const BACKUP_RDI_OFFSET: i32 = -24;
  /// 原版 kBackupRbxOffset
  pub const BACKUP_RBX_OFFSET: i32 = -32;
  /// 原版 kBackupR12Offset
  pub const BACKUP_R12_OFFSET: i32 = -40;
  /// 原版 kNumCalleeSaveRegisters
  pub const NUM_CALLEE_SAVE_REGISTERS: i32 = 4;
  /// 原版 kLastCalleeSaveRegister
  pub const LAST_CALLEE_SAVE_REGISTER: i32 = -40;
  /// 原版 kSuccessfulCapturesOffset
  pub const SUCCESSFUL_CAPTURES_OFFSET: i32 = -48;
  /// 原版 kStringStartMinusOneOffset
  pub const STRING_START_MINUS_ONE_OFFSET: i32 = -56;
  /// 原版 kBacktrackCountOffset
  pub const BACKTRACK_COUNT_OFFSET: i32 = -64;
  /// 原版 kRegExpStackBasePointerOffset
  pub const REGEXP_STACK_BASE_POINTER_OFFSET: i32 = -72;
  /// 原版 kRegisterZeroOffset
  pub const REGISTER_ZERO_OFFSET: i32 = -80;

  /// 正则内部寄存器 (register_index) 在栈上的 rbp 偏移
  /// 对应原版 register_location(index) 寻址：kRegisterZeroOffset - index * 8
  #[inline]
  pub const fn register_location_offset(register_index: i32) -> i32 {
    REGISTER_ZERO_OFFSET - register_index * 8
  }
}

/// 首次匹配缓冲 (原版 kRegExpCodeSize)
pub const REGEXP_CODE_SIZE: i32 = 1024;

/// 专用寄存器分配 (原版静态访问器与核心约定; 名称 → x64 寄存器)
pub const REGISTERS: &[(&str, &str)] = &[
  ("current_character", "rdx"),
  ("current_position", "rdi"),
  ("input_end", "rsi"),
  ("frame_pointer", "rbp"),
  ("stack_pointer", "rsp"),
  ("backtrack_stackpointer", "rbx"),
  ("code_object_pointer", "r8"),
  ("regexp_stack", "r12"),
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(sysv::FRAME_TYPE_OFFSET == -(sysv::RETURN_ADDRESS_OFFSET));
  assert!(sysv::INPUT_STRING_OFFSET == sysv::FRAME_TYPE_OFFSET - 8);
  assert!(sysv::NUM_OUTPUT_REGISTERS_OFFSET == sysv::INPUT_STRING_OFFSET - 5 * 8);
  assert!(sysv::NUM_CALLEE_SAVE_REGISTERS == 2);
  assert!(sysv::SUCCESSFUL_CAPTURES_OFFSET == sysv::LAST_CALLEE_SAVE_REGISTER - 8);
  assert!(sysv::STRING_START_MINUS_ONE_OFFSET == sysv::SUCCESSFUL_CAPTURES_OFFSET - 8);
  assert!(sysv::BACKTRACK_COUNT_OFFSET == sysv::STRING_START_MINUS_ONE_OFFSET - 8);
  assert!(sysv::REGEXP_STACK_BASE_POINTER_OFFSET == sysv::BACKTRACK_COUNT_OFFSET - 8);
  assert!(sysv::REGISTER_ZERO_OFFSET == sysv::REGEXP_STACK_BASE_POINTER_OFFSET - 8);

  assert!(win::INPUT_STRING_OFFSET == win::FRAME_ALIGN);
  assert!(win::NUM_CALLEE_SAVE_REGISTERS == 4);
  assert!(win::SUCCESSFUL_CAPTURES_OFFSET == win::LAST_CALLEE_SAVE_REGISTER - 8);
  assert!(win::STRING_START_MINUS_ONE_OFFSET == win::SUCCESSFUL_CAPTURES_OFFSET - 8);
  assert!(win::BACKTRACK_COUNT_OFFSET == win::STRING_START_MINUS_ONE_OFFSET - 8);
  assert!(win::REGEXP_STACK_BASE_POINTER_OFFSET == win::BACKTRACK_COUNT_OFFSET - 8);
  assert!(win::REGISTER_ZERO_OFFSET == win::REGEXP_STACK_BASE_POINTER_OFFSET - 8);

  assert!(REGEXP_CODE_SIZE == 1024);
  assert!(!REGISTERS.is_empty());
};
