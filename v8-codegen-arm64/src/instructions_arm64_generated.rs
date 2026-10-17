// 由 tool/src/codegen/arm64/instructions-arm64.h.js 从 v8/src/codegen/arm64/instructions-arm64.{h,cc,-constants.cc} 生成; 修订就地改, 复跑覆盖
// arm64 指令枚举与 FP 位型常量

// 字段掩码常量 (LoadStoreMask/ImmPCRel_mask/…) 由 constants-arm64 覆盖面提供;
// 指令解码语义 (Instruction::*) 登记跳过。

/// NEON 访存/运算格式 (原 enum NEONFormat)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NeonFormat {
  /// NF_UNDEF
  NF_UNDEF = 0,
  /// NF_8B
  NF_8B = 1,
  /// NF_16B
  NF_16B = 2,
  /// NF_4H
  NF_4H = 3,
  /// NF_8H
  NF_8H = 4,
  /// NF_2S
  NF_2S = 5,
  /// NF_4S
  NF_4S = 6,
  /// NF_1D
  NF_1D = 7,
  /// NF_2D
  NF_2D = 8,
  /// NF_B
  NF_B = 9,
  /// NF_H
  NF_H = 10,
  /// NF_S
  NF_S = 11,
  /// NF_D
  NF_D = 12,
}

/// NEON 格式索引位宽上限 (原 kNEONFormatMaxBits)
pub const NEON_FORMAT_MAX_BITS: usize = 6;

/// NEONFormatMap (原 struct): bits 位拼接索引 → 格式
#[derive(Debug, Clone, Copy)]
pub struct NeonFormatMap {
  /// 参与索引的指令位号
  pub bits: [u8; 6],
  /// 2^bits 查表
  pub map: [NeonFormat; 1 << 6],
}

/// 分支类型 (原 enum ImmBranchType)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ImmBranchType {
  /// UnknownBranchType
  UnknownBranchType = 0,
  /// CondBranchType
  CondBranchType = 1,
  /// UncondBranchType
  UncondBranchType = 2,
  /// CompareBranchType
  CompareBranchType = 3,
  /// TestBranchType
  TestBranchType = 4,
}

/// 访存地址模式 (原 enum AddrMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AddrMode {
  /// Offset
  Offset = 0,
  /// PreIndex
  PreIndex = 1,
  /// PostIndex
  PostIndex = 2,
}

/// 浮点舍入模式 (原 enum FPRounding; 前 4 值可由 FPCR<RMode> 编码)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FPRounding {
  /// FPTieEven
  FPTieEven = 0x0,
  /// FPPositiveInfinity
  FPPositiveInfinity = 0x1,
  /// FPNegativeInfinity
  FPNegativeInfinity = 0x2,
  /// FPZero
  FPZero = 0x3,
  /// FPTieAway
  FPTieAway = 0x4,
  /// FPRoundOdd
  FPRoundOdd = 0x5,
}

/// 31 号寄存器语义 (原 enum Reg31Mode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Reg31Mode {
  /// Reg31IsStackPointer
  Reg31IsStackPointer = 0,
  /// Reg31IsZeroRegister
  Reg31IsZeroRegister = 1,
}

/// MOPS 内存操作 (原 enum class MemOp : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MemOp {
  /// kCPY
  kCPY = 0,
  /// kSET
  kSET = 1,
}

/// 对齐检查 (原 enum CheckAlignment)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CheckAlignment {
  /// NO_CHECK
  NO_CHECK = 0,
  /// CHECK_ALIGNMENT
  CHECK_ALIGNMENT = 1,
}

/// printf 参数模式 (原 enum PrintfArgPattern; 无 S 型 — float 一律转 double)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PrintfArgPattern {
  /// kPrintfArgW
  kPrintfArgW = 1,
  /// kPrintfArgX
  kPrintfArgX = 2,
  /// kPrintfArgD
  kPrintfArgD = 3,
}

/// printf 参数模式位宽 (原 kPrintfArgPatternBits)
pub const PRINTF_ARG_PATTERN_BITS: usize = 2;

/// debug 参数 (原 enum DebugParameters)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DebugParameters {
  /// NO_PARAM
  NO_PARAM = 0,
  /// BREAK
  BREAK = 1,
  /// LOG_DISASM
  LOG_DISASM = 2,
  /// LOG_REGS
  LOG_REGS = 3,
  /// LOG_VREGS
  LOG_VREGS = 4,
  /// LOG_SYS_REGS
  LOG_SYS_REGS = 5,
  /// LOG_WRITE
  LOG_WRITE = 6,
  /// LOG_NONE
  LOG_NONE = 7,
  /// LOG_STATE
  LOG_STATE = 8,
  /// LOG_ALL
  LOG_ALL = 9,
  /// TRACE_ENABLE
  TRACE_ENABLE = 10,
  /// TRACE_DISABLE
  TRACE_DISABLE = 11,
  /// TRACE_OVERRIDE
  TRACE_OVERRIDE = 12,
}

// —— FP 位型常量 (integer_constants, bit_cast 前的整型原值) ——

/// FP32 +∞ 位型
pub const FP32_POSITIVE_INFINITY_BITS: u32 = 0x7F800000;
/// FP32 -∞ 位型
pub const FP32_NEGATIVE_INFINITY_BITS: u32 = 0xFF800000;
/// FP64 +∞ 位型
pub const FP64_POSITIVE_INFINITY_BITS: u64 = 0x7FF0000000000000;
/// FP64 -∞ 位型
pub const FP64_NEGATIVE_INFINITY_BITS: u64 = 0xFFF0000000000000;
/// FP64 信号 NaN
pub const FP64_SIGNALLING_NAN_BITS: u64 = 0x7FF000007F800001;
/// FP32 信号 NaN
pub const FP32_SIGNALLING_NAN_BITS: u32 = 0x7F800001;
/// FP64 静默 NaN
pub const FP64_QUIET_NAN_BITS: u64 = 0x7FF800007FC00001;
/// FP32 静默 NaN
pub const FP32_QUIET_NAN_BITS: u32 = 0x7FC00001;
/// FP64 默认 NaN (FPCR.DN=1)
pub const FP64_DEFAULT_NAN_BITS: u64 = 0x7FF8000000000000;
/// FP32 默认 NaN
pub const FP32_DEFAULT_NAN_BITS: u32 = 0x7FC00000;
/// FP16 默认 NaN
pub const FP16_DEFAULT_NAN_BITS: u16 = 0x7E00;

// —— 提取不变式断言 ——
const _: () = {
  assert!(NeonFormat::NF_UNDEF as u8 == 0);
  assert!(NeonFormat::NF_D as u8 == 12);
  assert!(ImmBranchType::CondBranchType as u8 == 1);
  assert!(FPRounding::FPTieEven as u8 == 0x0);
  assert!(FPRounding::FPZero as u8 == 0x3);
  assert!(FP64_DEFAULT_NAN_BITS == 0x7FF8000000000000);
  assert!(FP16_DEFAULT_NAN_BITS == 0x7E00);
  assert!(PRINTF_ARG_PATTERN_BITS == 2);
};
