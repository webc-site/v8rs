// 由 tool/src/diagnostics/unwinding-info-win64.h.js 从 v8/src/diagnostics/unwinding-info-win64.{h,cc} 生成; 修订就地改, 复跑覆盖
// Win64/WinARM64 展开信息常量面

// Windows 专用模块 (V8_OS_WIN_X64 / V8_OS_WIN_ARM64 双分支, 均 x64/arm64 目标内),
// 接入 crate 时建议整体 #[cfg(windows)]。语义面 (XdataEncoder/ABI 编码) 登记跳过。

/// 原 CRASH_HANDLER_FUNCTION_NAME 宏
pub const CRASH_HANDLER_FUNCTION_NAME: &str = "CrashForExceptionInNonABICompliantCodeRange";

/// OS 页大小 (原 kOSPageSize)
pub const OS_PAGE_SIZE: usize = 4096;

/// RUNTIME_FUNCTION 默认数 (原 kDefaultRuntimeFunctionCount; X64 一条覆盖 4GB)
pub const DEFAULT_RUNTIME_FUNCTION_COUNT: u32 = 1;

// —— V8_OS_WIN_X64 分支 ——

/// 原 kPushRbpInstructionLength
pub const X64_PUSH_RBP_INSTRUCTION_LENGTH: usize = 1;
/// 原 kMovRbpRspInstructionLength
pub const X64_MOV_RBP_RSP_INSTRUCTION_LENGTH: usize = 3;
/// 原 kRbpPrefixCodes
pub const X64_RBP_PREFIX_CODES: usize = 2;
/// 原 kRbpPrefixLength = push + mov
pub const X64_RBP_PREFIX_LENGTH: usize = 4;

// —— V8_OS_WIN_ARM64 分支 ——

/// 18 位无符号编码函数长度 (除 2 个恒 0 LSB), 原 kMaxFunctionLength
pub const ARM64_MAX_FUNCTION_LENGTH: usize = 1048572;

/// ARM64 unwind 8 位操作码 (原 enum UnwindOp8Bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum UnwindOp8Bit {
  /// 原 `OpNop`
  Nop = 0xE3,
  /// 原 `OpAllocS`
  AllocS = 0x0,
  /// 原 `OpSaveFpLr`
  SaveFpLr = 0x40,
  /// 原 `OpSaveFpLrX`
  SaveFpLrX = 0x80,
  /// 原 `OpSetFp`
  SetFp = 0xE1,
  /// 原 `OpAddFp`
  AddFp = 0xE2,
  /// 原 `OpEnd`
  End = 0xE4,
}

impl UnwindOp8Bit {
  /// 原 Combine8BitUnwindCodes: 四个 8 位操作码打包为一个 UNWIND_CODE 字
  #[inline]
  #[must_use]
  pub const fn combine(c0: Self, c1: Self, c2: Self, c3: Self) -> u32 {
    c0 as u32 | ((c1 as u32) << 8) | ((c2 as u32) << 16) | ((c3 as u32) << 24)
  }
}

/// 原 UNWIND_CODE typedef
pub type UnwindCode = u32;

/// 原 kNumberOfUnwindCodes
pub const ARM64_NUMBEROFUNWINDCODES: usize = 2;
/// 原 kMaxExceptionThunkSize
pub const ARM64_MAXEXCEPTIONTHUNKSIZE: usize = 12;
/// 原 kDefaultNumberOfUnwindCodeWords
pub const ARM64_DEFAULTNUMBEROFUNWINDCODEWORDS: usize = 1;
/// 原 kFunctionLengthShiftSize
pub const ARM64_FUNCTION_LENGTH_SHIFT_SIZE: usize = 2;
/// 原 kFunctionLengthMask
pub const ARM64_FUNCTION_LENGTH_MASK: usize = 0x3;
/// 原 kAllocStackShiftSize
pub const ARM64_ALLOC_STACK_SHIFT_SIZE: usize = 4;
/// 原 kAllocStackShiftMask
pub const ARM64_ALLOC_STACK_SHIFT_MASK: usize = 0xF;

// —— 提取不变式断言 ——
const _: () = {
  assert!(OS_PAGE_SIZE == 4096);
  assert!(DEFAULT_RUNTIME_FUNCTION_COUNT == 1);
  assert!(X64_RBP_PREFIX_LENGTH == 4);
  assert!(ARM64_MAX_FUNCTION_LENGTH == 1048572);
  assert!(ARM64_FUNCTION_LENGTH_MASK == (1 << ARM64_FUNCTION_LENGTH_SHIFT_SIZE) - 1);
  assert!(ARM64_ALLOC_STACK_MASK == (1 << ARM64_ALLOC_STACK_SHIFT_SIZE) - 1);
  assert!(UnwindOp8Bit::End as u32 == 0xE4);
  assert!(UnwindOp8Bit::combine(UnwindOp8Bit::Nop, UnwindOp8Bit::Nop, UnwindOp8Bit::Nop, UnwindOp8Bit::Nop) == 0xE3E3E3E3);
};
