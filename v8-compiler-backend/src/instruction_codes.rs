// 由 tool/src/compiler/backend/instruction-codes.h.js 从 v8/src/compiler/backend/instruction-codes.h (公共面) 生成; 修订就地改, 复跑覆盖
// 指令码公共面: FlagsMode/FlagsCondition/位域布局/RecordWriteMode

/// 原版 InstructionCode — 不透明 32 位编码 (u32)
pub type InstructionCode = u32;

/// 原版 kLazyDeoptOnThrowSentinel
pub const LAZY_DEOPT_ON_THROW_SENTINEL: i32 = -1;

/// 原版 RecordWriteMode — 写屏障存储模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RecordWriteMode {
  /// kValueIsMap
  ValueIsMap = 0,
  /// kValueIsPointer
  ValueIsPointer = 1,
  /// kValueIsIndirectPointer
  ValueIsIndirectPointer = 2,
  /// kValueIsEphemeronKey
  ValueIsEphemeronKey = 3,
  /// kValueIsAny
  ValueIsAny = 4,
}

impl RecordWriteMode {
  /// 原版 WriteBarrierKindToRecordWriteMode (kNoWriteBarrier 不应传入)
  #[must_use]
  pub fn from_write_barrier_kind(kind: &str) -> Option<Self> {
    match kind {
      "kMapWriteBarrier" => Some(RecordWriteMode::ValueIsMap),
      "kPointerWriteBarrier" => Some(RecordWriteMode::ValueIsPointer),
      "kIndirectPointerWriteBarrier" => Some(RecordWriteMode::ValueIsIndirectPointer),
      "kEphemeronKeyWriteBarrier" => Some(RecordWriteMode::ValueIsEphemeronKey),
      "kFullWriteBarrier" => Some(RecordWriteMode::ValueIsAny),
      _ => None,
    }
  }
}

/// 原版 FlagsMode — 标志位延续模式 (显式值)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum FlagsMode {
  /// kNone
  None = 0,
  /// kBranch
  Branch = 1,
  /// kDeoptimize
  Deoptimize = 2,
  /// kSet
  Set = 3,
  /// kTrap
  Trap = 4,
  /// kSelect
  Select = 5,
  /// kConditionalTrap
  ConditionalTrap = 6,
  /// kConditionalBranch
  ConditionalBranch = 7,
}

/// 原版 FlagsCondition — 标志位条件
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum FlagsCondition {
  /// kEqual
  Equal = 0,
  /// kNotEqual
  NotEqual = 1,
  /// kSignedLessThan
  SignedLessThan = 2,
  /// kSignedGreaterThanOrEqual
  SignedGreaterThanOrEqual = 3,
  /// kSignedLessThanOrEqual
  SignedLessThanOrEqual = 4,
  /// kSignedGreaterThan
  SignedGreaterThan = 5,
  /// kUnsignedLessThan
  UnsignedLessThan = 6,
  /// kUnsignedGreaterThanOrEqual
  UnsignedGreaterThanOrEqual = 7,
  /// kUnsignedLessThanOrEqual
  UnsignedLessThanOrEqual = 8,
  /// kUnsignedGreaterThan
  UnsignedGreaterThan = 9,
  /// kFloatLessThanOrUnordered
  FloatLessThanOrUnordered = 10,
  /// kFloatGreaterThanOrEqual
  FloatGreaterThanOrEqual = 11,
  /// kFloatLessThanOrEqual
  FloatLessThanOrEqual = 12,
  /// kFloatGreaterThanOrUnordered
  FloatGreaterThanOrUnordered = 13,
  /// kFloatLessThan
  FloatLessThan = 14,
  /// kFloatGreaterThanOrEqualOrUnordered
  FloatGreaterThanOrEqualOrUnordered = 15,
  /// kFloatLessThanOrEqualOrUnordered
  FloatLessThanOrEqualOrUnordered = 16,
  /// kFloatGreaterThan
  FloatGreaterThan = 17,
  /// kUnorderedEqual
  UnorderedEqual = 18,
  /// kUnorderedNotEqual
  UnorderedNotEqual = 19,
  /// kOverflow
  Overflow = 20,
  /// kNotOverflow
  NotOverflow = 21,
  /// kPositiveOrZero
  PositiveOrZero = 22,
  /// kNegative
  Negative = 23,
  /// kIsNaN
  IsNaN = 24,
  /// kIsNotNaN
  IsNotNaN = 25,
}

impl FlagsCondition {
  /// 原版 kStackPointerGreaterThanCondition
  pub const STACK_POINTER_GREATER_THAN_CONDITION: FlagsCondition =
    FlagsCondition::UnsignedGreaterThan;

  /// 从数值反查 (原版 static_cast 语义)
  #[inline]
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
    match val {
      0 => FlagsCondition::Equal,
      1 => FlagsCondition::NotEqual,
      2 => FlagsCondition::SignedLessThan,
      3 => FlagsCondition::SignedGreaterThanOrEqual,
      4 => FlagsCondition::SignedLessThanOrEqual,
      5 => FlagsCondition::SignedGreaterThan,
      6 => FlagsCondition::UnsignedLessThan,
      7 => FlagsCondition::UnsignedGreaterThanOrEqual,
      8 => FlagsCondition::UnsignedLessThanOrEqual,
      9 => FlagsCondition::UnsignedGreaterThan,
      10 => FlagsCondition::FloatLessThanOrUnordered,
      11 => FlagsCondition::FloatGreaterThanOrEqual,
      12 => FlagsCondition::FloatLessThanOrEqual,
      13 => FlagsCondition::FloatGreaterThanOrUnordered,
      14 => FlagsCondition::FloatLessThan,
      15 => FlagsCondition::FloatGreaterThanOrEqualOrUnordered,
      16 => FlagsCondition::FloatLessThanOrEqualOrUnordered,
      17 => FlagsCondition::FloatGreaterThan,
      18 => FlagsCondition::UnorderedEqual,
      19 => FlagsCondition::UnorderedNotEqual,
      20 => FlagsCondition::Overflow,
      21 => FlagsCondition::NotOverflow,
      22 => FlagsCondition::PositiveOrZero,
      23 => FlagsCondition::Negative,
      24 => FlagsCondition::IsNaN,
      25 => FlagsCondition::IsNotNaN,
      _ => FlagsCondition::Equal,
    }
  }

  /// 原版 NegateFlagsCondition — 条件取反 (异或 1)
  #[inline]
  #[must_use]
  pub const fn negate(self) -> Self {
    Self::from_repr((self as u8) ^ 1)
  }
}

/// 原版 MemoryAccessMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MemoryAccessMode {
  /// kMemoryAccessDirect
  Direct = 0,
  /// kMemoryAccessTrapping
  Trapping = 1,
}

/// 原版 AtomicWidth (64 位架构双宽; 32 位全按 32 处理)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AtomicWidth {
  /// kWord32 (尺寸 4)
  Word32 = 0,
  /// kWord64 (尺寸 8)
  Word64 = 1,
}

impl AtomicWidth {
  /// 原版 AtomicWidthSize
  #[inline]
  #[must_use]
  pub const fn size(self) -> usize {
    match self {
      AtomicWidth::Word32 => 4,
      AtomicWidth::Word64 => 8,
    }
  }
}

/// 原版 LaneSize — SIMD 通道宽
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LaneSize {
  /// kL8 (位宽 8)
  L8 = 0,
  /// kL16 (位宽 16)
  L16 = 1,
  /// kL32 (位宽 32)
  L32 = 2,
  /// kL64 (位宽 64)
  L64 = 3,
}

impl LaneSize {
  /// 原版 LaneSizeBits
  #[inline]
  #[must_use]
  pub const fn bits(self) -> i32 {
    match self {
      LaneSize::L8 => 8,
      LaneSize::L16 => 16,
      LaneSize::L32 => 32,
      LaneSize::L64 => 64,
    }
  }

  /// 原版 LaneSizeFromBits
  #[must_use]
  pub const fn from_bits(width: u8) -> Option<Self> {
    match width {
      8 => Some(LaneSize::L8),
      16 => Some(LaneSize::L16),
      32 => Some(LaneSize::L32),
      64 => Some(LaneSize::L64),
      _ => None,
    }
  }
}

/// 原版 VectorLength
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VectorLength {
  /// kV64
  V64 = 0,
  /// kV128
  V128 = 1,
  /// kV256
  V256 = 2,
  /// kV512
  V512 = 3,
}

impl VectorLength {
  /// 原版 VectorLengthBits
  #[inline]
  #[must_use]
  pub const fn bits(self) -> i32 {
    match self {
      VectorLength::V64 => 64,
      VectorLength::V128 => 128,
      VectorLength::V256 => 256,
      VectorLength::V512 => 512,
    }
  }
}

/// 原版 InstructionCode 位域布局 (低 23 位五字段 + 9 位 Misc 覆盖区):
/// - ArchOpcodeField: offset 0, 宽 10
/// - AddressingModeField: offset 10, 宽 5
/// - FlagsModeField: offset 15, 宽 3
/// - FlagsConditionField: offset 18, 宽 5
/// - MiscField: offset 23, 宽 9
///
/// MiscField 覆盖区按指令类再分:
/// - RecordWriteModeField: offset 23, 宽 3
/// - AtomicWidthField: offset 26, 宽 1
/// - AtomicMemoryOrderField: offset 27, 宽 1
/// - AccessModeField: offset 28, 宽 1
/// - LaneSizeField: offset 23, 宽 2
/// - VectorLengthField: offset 25, 宽 2
/// - SharedBaseField: offset 29, 宽 1
/// - DeoptImmedArgsCountField: offset 23, 宽 2
/// - DeoptFrameStateOffsetField: offset 25, 宽 7
/// - MiscField: offset 23, 宽 9
/// - StackCheckField: offset 23, 宽 2
/// - BranchHintField: offset 25, 宽 1
pub mod code_fields {
  /// 原版 ArchOpcodeField — offset 0, 宽 10
  pub const ARCH_OPCODE_FIELD: (u32, u32) = (0, 10);
  /// 原版 AddressingModeField — offset 10, 宽 5
  pub const ADDRESSING_MODE_FIELD: (u32, u32) = (10, 5);
  /// 原版 FlagsModeField — offset 15, 宽 3
  pub const FLAGS_MODE_FIELD: (u32, u32) = (15, 3);
  /// 原版 FlagsConditionField — offset 18, 宽 5
  pub const FLAGS_CONDITION_FIELD: (u32, u32) = (18, 5);
  /// 原版 RecordWriteModeField — offset 23, 宽 3
  pub const RECORD_WRITE_MODE_FIELD: (u32, u32) = (23, 3);
  /// 原版 AtomicWidthField — offset 26, 宽 1
  pub const ATOMIC_WIDTH_FIELD: (u32, u32) = (26, 1);
  /// 原版 AtomicMemoryOrderField — offset 27, 宽 1
  pub const ATOMIC_MEMORY_ORDER_FIELD: (u32, u32) = (27, 1);
  /// 原版 AccessModeField — offset 28, 宽 1
  pub const ACCESS_MODE_FIELD: (u32, u32) = (28, 1);
  /// 原版 LaneSizeField — offset 23, 宽 2
  pub const LANE_SIZE_FIELD: (u32, u32) = (23, 2);
  /// 原版 VectorLengthField — offset 25, 宽 2
  pub const VECTOR_LENGTH_FIELD: (u32, u32) = (25, 2);
  /// 原版 SharedBaseField — offset 29, 宽 1
  pub const SHARED_BASE_FIELD: (u32, u32) = (29, 1);
  /// 原版 DeoptImmedArgsCountField — offset 23, 宽 2
  pub const DEOPT_IMMED_ARGS_COUNT_FIELD: (u32, u32) = (23, 2);
  /// 原版 DeoptFrameStateOffsetField — offset 25, 宽 7
  pub const DEOPT_FRAME_STATE_OFFSET_FIELD: (u32, u32) = (25, 7);
  /// 原版 MiscField — offset 23, 宽 9
  pub const MISC_FIELD: (u32, u32) = (23, 9);
  /// 原版 StackCheckField — offset 23, 宽 2
  pub const STACK_CHECK_FIELD: (u32, u32) = (23, 2);
  /// 原版 BranchHintField — offset 25, 宽 1
  pub const BRANCH_HINT_FIELD: (u32, u32) = (25, 1);
  /// 原版 ParamField — offset 0, 宽 5
  pub const PARAM_FIELD: (u32, u32) = (0, 5);
  /// 原版 FPParamField — offset 5, 宽 5
  pub const FPPARAM_FIELD: (u32, u32) = (5, 5);
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(FlagsMode::None as u8 == 0);
  assert!(FlagsMode::ConditionalBranch as u8 == 7);
  assert!(FlagsCondition::Equal as u8 == 0);
  assert!(FlagsCondition::IsNotNaN as u8 == 25);
  assert!(FlagsCondition::Equal.negate() as u8 == FlagsCondition::NotEqual as u8);
  assert!(FlagsCondition::Overflow.negate() as u8 == FlagsCondition::NotOverflow as u8);
  assert!(RecordWriteMode::ValueIsMap as u8 == 0);
  assert!(AtomicWidth::Word64.size() == 8);
  assert!(LaneSize::L8.bits() == 8);
  assert!(VectorLength::V512.bits() == 512);
  assert!(code_fields::ARCH_OPCODE_FIELD.0 == 0);
  assert!(code_fields::MISC_FIELD.0 == 23 && code_fields::MISC_FIELD.1 == 9);
};
