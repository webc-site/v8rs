// 由 tool/src/regexp/arm64/regexp-macro-assembler-arm64.h.js 从 v8/src/regexp/arm64/regexp-macro-assembler-arm64.h 生成; 修订就地改, 复跑覆盖
// arm64 正则匹配栈帧布局与专用寄存器分配

/// arm64 正则栈帧布局 (rbp 相对偏移; kSystemPointerSize=8, kWRegSize=4)
pub const K_SYSTEM_POINTER_SIZE: i32 = 8;
pub const K_W_REG_SIZE: i32 = 4;
/// 原版 kFramePointerOffset
pub const FRAME_POINTER_OFFSET: i32 = 0;
/// 原版 kReturnAddressOffset
pub const RETURN_ADDRESS_OFFSET: i32 = 8;

/// 原版 kNumCalleeSavedRegisters (x19-x28)
pub const NUM_CALLEE_SAVED_REGISTERS: i32 = 10;
/// 原版 kCalleeSavedRegistersOffset
pub const CALLEE_SAVED_REGISTERS_OFFSET: i32 = 16;
/// 原版 kFrameTypeOffset
pub const FRAME_TYPE_OFFSET: i32 = -8;

/// 原版 kPaddingAfterFrameType
pub const PADDING_AFTER_FRAME_TYPE: i32 = 8;
/// 原版 kIsolateOffset
pub const ISOLATE_OFFSET: i32 = -24;
/// 原版 kDirectCallOffset
pub const DIRECT_CALL_OFFSET: i32 = -32;
/// 原版 kNumOutputRegistersOffset
pub const NUM_OUTPUT_REGISTERS_OFFSET: i32 = -40;
/// 原版 kInputStringOffset
pub const INPUT_STRING_OFFSET: i32 = -48;
/// 原版 kSuccessfulCapturesOffset
pub const SUCCESSFUL_CAPTURES_OFFSET: i32 = -56;
/// 原版 kBacktrackCountOffset
pub const BACKTRACK_COUNT_OFFSET: i32 = -64;
/// 原版 kRegExpStackBasePointerOffset
pub const REGEXP_STACK_BASE_POINTER_OFFSET: i32 = -72;
/// 原版 kStackLocalPadding
pub const STACK_LOCAL_PADDING: i32 = -80;

/// 原版 kNumberOfStackLocals
pub const NUMBER_OF_STACK_LOCALS: i32 = 4;
/// 原版 kFirstRegisterOnStackOffset
pub const FIRST_REGISTER_ON_STACK_OFFSET: i32 = -84;
/// 原版 kFirstCaptureOnStackOffset
pub const FIRST_CAPTURE_ON_STACK_OFFSET: i32 = -88;

/// 首次匹配缓冲 (原版 kInitialBufferSize)
pub const INITIAL_BUFFER_SIZE: i32 = 1024;

/// 展开初始化循环的寄存器数上限 (原版 kNumRegistersToUnroll)
pub const NUM_REGISTERS_TO_UNROLL: i32 = 16;

/// x0-x7 缓存捕获寄存器数 (每硬件寄存器存一对 position; 原版 kNumCachedRegisters)
pub const NUM_CACHED_REGISTERS: i32 = 16;

/// 专用寄存器分配 (原版静态访问器; 名称 → arm64 寄存器)
pub const REGISTERS: &[(&str, &str)] = &[
  ("current_input_offset", "w21"),
  ("current_character", "w22"),
  ("input_end", "x25"),
  ("input_start", "x26"),
  ("start_offset", "w27"),
  ("output_array", "x28"),
  ("frame_pointer", "fp"),
  ("backtrack_stackpointer", "x23"),
  ("code_pointer", "x20"),
  ("regexp_stack", "x19"),
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(FRAME_TYPE_OFFSET == -K_SYSTEM_POINTER_SIZE);
  assert!(INPUT_STRING_OFFSET == -6 * K_SYSTEM_POINTER_SIZE);
  assert!(FIRST_REGISTER_ON_STACK_OFFSET == STACK_LOCAL_PADDING - K_W_REG_SIZE);
  assert!(NUM_CACHED_REGISTERS == 16);
  assert!(!REGISTERS.is_empty());
};
