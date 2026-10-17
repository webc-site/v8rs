// 由 tool/src/regexp/regexp-bytecodes.h.js 从 v8/src/regexp/regexp-bytecodes.h 生成; 修订就地改, 复跑覆盖
// 四表 opcode + 操作数类型三表 + flag 位表 + GOLDEN_SIZES 布局金标准; 布局算法/编解码在 bytecodes.rs, Dump 在 dump.rs; 同源性: v8-regexp-exec/src/regexp_interpreter.rs (脚本交叉对数)

// ── 操作数类型 (BASIC + LIMITS + SPECIAL 展开, 序数即 BYTECODE_OPERAND_TYPE_LIST 序) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BytecodeOperandType {
  /// int16_t, 宽 2 对齐 2
  Int16 = 0,
  /// int32_t, 宽 4 对齐 4
  Int32 = 1,
  /// uint32_t, 宽 4 对齐 4
  Uint32 = 2,
  /// base::uc16, 宽 2 对齐 2
  Char = 3,
  /// uint32_t, 宽 4 对齐 4
  JumpTarget = 4,
  /// int16_t 限幅, 宽 2 对齐 2
  Offset = 5,
  /// int32_t 限幅, 宽 4 对齐 4
  BoundsCheckOffset = 6,
  /// uint16_t 限幅, 宽 2 对齐 2
  Register = 7,
  /// RegExpMacroAssembler::StackCheckFlag 限幅, 宽 1 对齐 1
  StackCheckFlag = 8,
  /// StandardCharacterSet 限幅, 宽 1 对齐 1
  StandardCharacterSet = 9,
  /// 专列 16/1, 宽 16 对齐 1
  BitTable = 10,
}

pub const K_OPERAND_TYPE_COUNT: usize = 11;
const _: () = assert!(BytecodeOperandType::BitTable as usize + 1 == K_OPERAND_TYPE_COUNT);

pub const OPERAND_TYPE_NAMES: [&str; K_OPERAND_TYPE_COUNT] = [
  "Int16",
  "Int32",
  "Uint32",
  "Char",
  "JumpTarget",
  "Offset",
  "BoundsCheckOffset",
  "Register",
  "StackCheckFlag",
  "StandardCharacterSet",
  "BitTable",
];

/// OperandTypeTraits::kSize (inl.h:30/45/63)
pub const OPERAND_TYPE_SIZES: [u8; K_OPERAND_TYPE_COUNT] = [2, 4, 4, 2, 4, 2, 4, 2, 1, 1, 16];

/// OperandTypeTraits::kAlignment (inl.h:35/54/65: basic=自身宽, BitTable=1)
pub const OPERAND_TYPE_ALIGNMENTS: [u8; K_OPERAND_TYPE_COUNT] = [2, 4, 4, 2, 4, 2, 4, 2, 1, 1, 1];

/// OperandTypeTraits::kIsBasic (inl.h:32/53/64: 仅 BitTable 为 false)
pub const OPERAND_TYPE_IS_BASIC: [bool; K_OPERAND_TYPE_COUNT] = [
  true, true, true, true, true, true, true, true, true, true, false,
];

/// OperandTypeTraits::kMinValue / kMaxValue (inl.h:33-34/51-52; SPECIAL 无 C 型限幅, (0,0) 占位)
pub const OPERAND_TYPE_LIMITS: [(i64, i64); K_OPERAND_TYPE_COUNT] = [
  (-32768, 32767),
  (-2147483648, 2147483647),
  (0, 4294967295),
  (0, 65535),
  (0, 4294967295),
  (-32767, 32767),
  (-32767, 33022),
  (0, 65535),
  (0, 1),
  (42, 119),
  (0, 0),
];

// ── 分析用 flag 位 (BytecodeFlag, h:59-79) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum BytecodeFlag {
  /// 1 << 0
  NoFallthrough = 1 << 0,
  /// 1 << 1
  NoBranchDespiteJumpTargetOperand = 1 << 1,
  /// 1 << 2
  LoadsCC = 1 << 2,
  /// 1 << 3
  UsesCC = 1 << 3,
}

// ── opcode 全集 (INVALID+BASIC+SPECIAL+PEEPHOLE 四表展开序) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Bytecode {
  // INVALID_BYTECODE_LIST ↓0..=0
  Break = 0,
  // BASIC_BYTECODE_LIST ↓1..=35
  PushCurrentPosition = 1,
  PushBacktrack = 2,
  WriteCurrentPositionToRegister = 3,
  ReadCurrentPositionFromRegister = 4,
  WriteStackPointerToRegister = 5,
  ReadStackPointerFromRegister = 6,
  SetRegister = 7,
  ClearRegisters = 8,
  AdvanceRegister = 9,
  PopCurrentPosition = 10,
  PushRegister = 11,
  PopRegister = 12,
  Fail = 13,
  Succeed = 14,
  AdvanceCurrentPosition = 15,
  GoTo = 16,
  LoadCurrentCharacter = 17,
  CheckPosition = 18,
  CheckSpecialClassRanges = 19,
  CheckCharacter = 20,
  CheckNotCharacter = 21,
  CheckCharacterAfterAnd = 22,
  CheckNotCharacterAfterAnd = 23,
  CheckNotCharacterAfterMinusAnd = 24,
  CheckCharacterInRange = 25,
  CheckCharacterNotInRange = 26,
  CheckCharacterLT = 27,
  CheckCharacterGT = 28,
  IfRegisterLT = 29,
  IfRegisterGE = 30,
  IfRegisterEqPos = 31,
  CheckAtStart = 32,
  CheckNotAtStart = 33,
  CheckFixedLengthLoop = 34,
  SetCurrentPositionFromEnd = 35,
  // SPECIAL_BYTECODE_LIST ↓36..=53
  Backtrack = 36,
  LoadCurrentCharacterUnchecked = 37,
  CheckBitInTable = 38,
  Load2CurrentChars = 39,
  Load2CurrentCharsUnchecked = 40,
  Load4CurrentChars = 41,
  Load4CurrentCharsUnchecked = 42,
  Check4Chars = 43,
  CheckNot4Chars = 44,
  AndCheck4Chars = 45,
  AndCheckNot4Chars = 46,
  AdvanceCpAndGoto = 47,
  CheckNotBackRef = 48,
  CheckNotBackRefNoCase = 49,
  CheckNotBackRefNoCaseUnicode = 50,
  CheckNotBackRefBackward = 51,
  CheckNotBackRefNoCaseBackward = 52,
  CheckNotBackRefNoCaseUnicodeBackward = 53,
  // PEEPHOLE_BYTECODE_LIST ↓54..=60
  SkipUntilBitInTable = 54,
  SkipUntilCharAnd = 55,
  SkipUntilChar = 56,
  SkipUntilCharOrChar = 57,
  SkipUntilGtOrNotBitInTable = 58,
  SkipUntilOneOfMasked = 59,
  SkipUntilOneOfMasked3 = 60,
  // 原版 kLast = -1 +1×61 (h:340-343) — 末项即最后实码
}

pub const K_BYTECODE_COUNT: u8 = 61;
pub const K_BYTECODE_ALIGNMENT: usize = 4;
/// 原版 kCount = ToByte(kLast)+1 (h:355); kLast 即末位
pub const K_LAST: Bytecode = Bytecode::SkipUntilOneOfMasked3;
const _: () = assert!(K_LAST as u8 + 1 == K_BYTECODE_COUNT);

pub const ALL_BYTECODES: [Bytecode; K_BYTECODE_COUNT as usize] = [
  Bytecode::Break,
  Bytecode::PushCurrentPosition,
  Bytecode::PushBacktrack,
  Bytecode::WriteCurrentPositionToRegister,
  Bytecode::ReadCurrentPositionFromRegister,
  Bytecode::WriteStackPointerToRegister,
  Bytecode::ReadStackPointerFromRegister,
  Bytecode::SetRegister,
  Bytecode::ClearRegisters,
  Bytecode::AdvanceRegister,
  Bytecode::PopCurrentPosition,
  Bytecode::PushRegister,
  Bytecode::PopRegister,
  Bytecode::Fail,
  Bytecode::Succeed,
  Bytecode::AdvanceCurrentPosition,
  Bytecode::GoTo,
  Bytecode::LoadCurrentCharacter,
  Bytecode::CheckPosition,
  Bytecode::CheckSpecialClassRanges,
  Bytecode::CheckCharacter,
  Bytecode::CheckNotCharacter,
  Bytecode::CheckCharacterAfterAnd,
  Bytecode::CheckNotCharacterAfterAnd,
  Bytecode::CheckNotCharacterAfterMinusAnd,
  Bytecode::CheckCharacterInRange,
  Bytecode::CheckCharacterNotInRange,
  Bytecode::CheckCharacterLT,
  Bytecode::CheckCharacterGT,
  Bytecode::IfRegisterLT,
  Bytecode::IfRegisterGE,
  Bytecode::IfRegisterEqPos,
  Bytecode::CheckAtStart,
  Bytecode::CheckNotAtStart,
  Bytecode::CheckFixedLengthLoop,
  Bytecode::SetCurrentPositionFromEnd,
  Bytecode::Backtrack,
  Bytecode::LoadCurrentCharacterUnchecked,
  Bytecode::CheckBitInTable,
  Bytecode::Load2CurrentChars,
  Bytecode::Load2CurrentCharsUnchecked,
  Bytecode::Load4CurrentChars,
  Bytecode::Load4CurrentCharsUnchecked,
  Bytecode::Check4Chars,
  Bytecode::CheckNot4Chars,
  Bytecode::AndCheck4Chars,
  Bytecode::AndCheckNot4Chars,
  Bytecode::AdvanceCpAndGoto,
  Bytecode::CheckNotBackRef,
  Bytecode::CheckNotBackRefNoCase,
  Bytecode::CheckNotBackRefNoCaseUnicode,
  Bytecode::CheckNotBackRefBackward,
  Bytecode::CheckNotBackRefNoCaseBackward,
  Bytecode::CheckNotBackRefNoCaseUnicodeBackward,
  Bytecode::SkipUntilBitInTable,
  Bytecode::SkipUntilCharAnd,
  Bytecode::SkipUntilChar,
  Bytecode::SkipUntilCharOrChar,
  Bytecode::SkipUntilGtOrNotBitInTable,
  Bytecode::SkipUntilOneOfMasked,
  Bytecode::SkipUntilOneOfMasked3,
];

/// detail::kBytecodeNames (inl.h:307-310, #CamelName)
pub const BYTECODE_NAMES: [&str; K_BYTECODE_COUNT as usize] = [
  "Break",
  "PushCurrentPosition",
  "PushBacktrack",
  "WriteCurrentPositionToRegister",
  "ReadCurrentPositionFromRegister",
  "WriteStackPointerToRegister",
  "ReadStackPointerFromRegister",
  "SetRegister",
  "ClearRegisters",
  "AdvanceRegister",
  "PopCurrentPosition",
  "PushRegister",
  "PopRegister",
  "Fail",
  "Succeed",
  "AdvanceCurrentPosition",
  "GoTo",
  "LoadCurrentCharacter",
  "CheckPosition",
  "CheckSpecialClassRanges",
  "CheckCharacter",
  "CheckNotCharacter",
  "CheckCharacterAfterAnd",
  "CheckNotCharacterAfterAnd",
  "CheckNotCharacterAfterMinusAnd",
  "CheckCharacterInRange",
  "CheckCharacterNotInRange",
  "CheckCharacterLT",
  "CheckCharacterGT",
  "IfRegisterLT",
  "IfRegisterGE",
  "IfRegisterEqPos",
  "CheckAtStart",
  "CheckNotAtStart",
  "CheckFixedLengthLoop",
  "SetCurrentPositionFromEnd",
  "Backtrack",
  "LoadCurrentCharacterUnchecked",
  "CheckBitInTable",
  "Load2CurrentChars",
  "Load2CurrentCharsUnchecked",
  "Load4CurrentChars",
  "Load4CurrentCharsUnchecked",
  "Check4Chars",
  "CheckNot4Chars",
  "AndCheck4Chars",
  "AndCheckNot4Chars",
  "AdvanceCpAndGoto",
  "CheckNotBackRef",
  "CheckNotBackRefNoCase",
  "CheckNotBackRefNoCaseUnicode",
  "CheckNotBackRefBackward",
  "CheckNotBackRefNoCaseBackward",
  "CheckNotBackRefNoCaseUnicodeBackward",
  "SkipUntilBitInTable",
  "SkipUntilCharAnd",
  "SkipUntilChar",
  "SkipUntilCharOrChar",
  "SkipUntilGtOrNotBitInTable",
  "SkipUntilOneOfMasked",
  "SkipUntilOneOfMasked3",
];

/// 逐条操作数名 (BytecodeOperandNames::kNames, inl.h:157-169 SplitNames)
pub(crate) const BYTECODE_OPERAND_NAMES: [&[&str]; K_BYTECODE_COUNT as usize] = [
  &[],
  &[],
  &["label"],
  &["register_index", "cp_offset"],
  &["register_index"],
  &["register_index"],
  &["register_index"],
  &["register_index", "value"],
  &["from_register", "to_register"],
  &["register_index", "by"],
  &[],
  &["register_index", "stack_check"],
  &["register_index"],
  &[],
  &[],
  &["by"],
  &["label"],
  &["cp_offset", "bounds_check_offset", "on_failure"],
  &["cp_offset", "on_failure"],
  &["character_set", "on_no_match"],
  &["character", "on_equal"],
  &["character", "on_not_equal"],
  &["character", "mask", "on_equal"],
  &["character", "mask", "on_not_equal"],
  &["character", "minus", "mask", "on_not_equal"],
  &["from", "to", "on_in_range"],
  &["from", "to", "on_not_in_range"],
  &["limit", "on_less"],
  &["limit", "on_greater"],
  &["register_index", "comparand", "on_less_than"],
  &["register_index", "comparand", "on_greater_or_equal"],
  &["register_index", "on_eq"],
  &["cp_offset", "on_at_start"],
  &["cp_offset", "on_not_at_start"],
  &["on_tos_equals_current_position"],
  &["by"],
  &["return_code"],
  &["cp_offset"],
  &["on_bit_set", "table"],
  &["cp_offset", "bounds_check_offset", "on_failure"],
  &["cp_offset"],
  &["cp_offset", "bounds_check_offset", "on_failure"],
  &["cp_offset"],
  &["characters", "on_equal"],
  &["characters", "on_not_equal"],
  &["characters", "mask", "on_equal"],
  &["characters", "mask", "on_not_equal"],
  &["by", "on_goto"],
  &["start_reg", "on_not_equal"],
  &["start_reg", "on_not_equal"],
  &["start_reg", "on_not_equal"],
  &["start_reg", "on_not_equal"],
  &["start_reg", "on_not_equal"],
  &["start_reg", "on_not_equal"],
  &[
    "cp_offset",
    "advance_by",
    "table",
    "bounds_check_offset",
    "on_match",
    "on_no_match",
  ],
  &[
    "cp_offset",
    "advance_by",
    "character",
    "mask",
    "bounds_check_offset",
    "on_match",
    "on_no_match",
  ],
  &[
    "cp_offset",
    "advance_by",
    "character",
    "bounds_check_offset",
    "on_match",
    "on_no_match",
  ],
  &[
    "cp_offset",
    "advance_by",
    "char1",
    "char2",
    "bounds_check_offset",
    "on_match",
    "on_no_match",
  ],
  &[
    "cp_offset",
    "advance_by",
    "character",
    "table",
    "bounds_check_offset",
    "on_match",
    "on_no_match",
  ],
  &[
    "cp_offset",
    "advance_by",
    "both_chars",
    "both_mask",
    "max_offset",
    "chars1",
    "mask1",
    "chars2",
    "mask2",
    "on_match1",
    "on_match2",
    "on_failure",
  ],
  &[
    "bc0_cp_offset",
    "bc0_advance_by",
    "bc0_table",
    "bc1_bounds_check_offset",
    "bc1_on_failure",
    "bc1_cp_offset",
    "bc2_characters",
    "bc2_mask",
    "bc3_by",
    "bc4_bounds_check_offset",
    "bc4_cp_offset",
    "bc5_characters",
    "bc5_mask",
    "bc5_on_equal",
    "bc6_characters",
    "bc6_mask",
    "bc6_on_equal",
    "bc7_characters",
    "bc7_mask",
    "fallthrough_jump_target",
  ],
];

/// 逐条操作数类型 (BytecodeOperandsTraits::kOperandTypes, inl.h:141-142)
pub(crate) const BYTECODE_OPERAND_TYPES: [&[BytecodeOperandType]; K_BYTECODE_COUNT as usize] = [
  &[],                                                             // Break
  &[],                                                             // PushCurrentPosition
  &[BytecodeOperandType::JumpTarget],                              // PushBacktrack
  &[BytecodeOperandType::Register, BytecodeOperandType::Offset],   // WriteCurrentPositionToRegister
  &[BytecodeOperandType::Register], // ReadCurrentPositionFromRegister
  &[BytecodeOperandType::Register], // WriteStackPointerToRegister
  &[BytecodeOperandType::Register], // ReadStackPointerFromRegister
  &[BytecodeOperandType::Register, BytecodeOperandType::Int32], // SetRegister
  &[BytecodeOperandType::Register, BytecodeOperandType::Register], // ClearRegisters
  &[BytecodeOperandType::Register, BytecodeOperandType::Offset], // AdvanceRegister
  &[],                              // PopCurrentPosition
  &[
    BytecodeOperandType::Register,
    BytecodeOperandType::StackCheckFlag,
  ], // PushRegister
  &[BytecodeOperandType::Register], // PopRegister
  &[],                              // Fail
  &[],                              // Succeed
  &[BytecodeOperandType::Offset],   // AdvanceCurrentPosition
  &[BytecodeOperandType::JumpTarget], // GoTo
  &[
    BytecodeOperandType::Offset,
    BytecodeOperandType::BoundsCheckOffset,
    BytecodeOperandType::JumpTarget,
  ], // LoadCurrentCharacter
  &[BytecodeOperandType::Offset, BytecodeOperandType::JumpTarget], // CheckPosition
  &[
    BytecodeOperandType::StandardCharacterSet,
    BytecodeOperandType::JumpTarget,
  ], // CheckSpecialClassRanges
  &[BytecodeOperandType::Char, BytecodeOperandType::JumpTarget], // CheckCharacter
  &[BytecodeOperandType::Char, BytecodeOperandType::JumpTarget], // CheckNotCharacter
  &[
    BytecodeOperandType::Char,
    BytecodeOperandType::Uint32,
    BytecodeOperandType::JumpTarget,
  ], // CheckCharacterAfterAnd
  &[
    BytecodeOperandType::Char,
    BytecodeOperandType::Uint32,
    BytecodeOperandType::JumpTarget,
  ], // CheckNotCharacterAfterAnd
  &[
    BytecodeOperandType::Char,
    BytecodeOperandType::Char,
    BytecodeOperandType::Char,
    BytecodeOperandType::JumpTarget,
  ], // CheckNotCharacterAfterMinusAnd
  &[
    BytecodeOperandType::Char,
    BytecodeOperandType::Char,
    BytecodeOperandType::JumpTarget,
  ], // CheckCharacterInRange
  &[
    BytecodeOperandType::Char,
    BytecodeOperandType::Char,
    BytecodeOperandType::JumpTarget,
  ], // CheckCharacterNotInRange
  &[BytecodeOperandType::Char, BytecodeOperandType::JumpTarget], // CheckCharacterLT
  &[BytecodeOperandType::Char, BytecodeOperandType::JumpTarget], // CheckCharacterGT
  &[
    BytecodeOperandType::Register,
    BytecodeOperandType::Int32,
    BytecodeOperandType::JumpTarget,
  ], // IfRegisterLT
  &[
    BytecodeOperandType::Register,
    BytecodeOperandType::Int32,
    BytecodeOperandType::JumpTarget,
  ], // IfRegisterGE
  &[
    BytecodeOperandType::Register,
    BytecodeOperandType::JumpTarget,
  ], // IfRegisterEqPos
  &[BytecodeOperandType::Offset, BytecodeOperandType::JumpTarget], // CheckAtStart
  &[BytecodeOperandType::Offset, BytecodeOperandType::JumpTarget], // CheckNotAtStart
  &[BytecodeOperandType::JumpTarget], // CheckFixedLengthLoop
  &[BytecodeOperandType::Offset],   // SetCurrentPositionFromEnd
  &[BytecodeOperandType::Int16],    // Backtrack
  &[BytecodeOperandType::Offset],   // LoadCurrentCharacterUnchecked
  &[
    BytecodeOperandType::JumpTarget,
    BytecodeOperandType::BitTable,
  ], // CheckBitInTable
  &[
    BytecodeOperandType::Offset,
    BytecodeOperandType::BoundsCheckOffset,
    BytecodeOperandType::JumpTarget,
  ], // Load2CurrentChars
  &[BytecodeOperandType::Offset],   // Load2CurrentCharsUnchecked
  &[
    BytecodeOperandType::Offset,
    BytecodeOperandType::BoundsCheckOffset,
    BytecodeOperandType::JumpTarget,
  ], // Load4CurrentChars
  &[BytecodeOperandType::Offset],   // Load4CurrentCharsUnchecked
  &[BytecodeOperandType::Uint32, BytecodeOperandType::JumpTarget], // Check4Chars
  &[BytecodeOperandType::Uint32, BytecodeOperandType::JumpTarget], // CheckNot4Chars
  &[
    BytecodeOperandType::Uint32,
    BytecodeOperandType::Uint32,
    BytecodeOperandType::JumpTarget,
  ], // AndCheck4Chars
  &[
    BytecodeOperandType::Uint32,
    BytecodeOperandType::Uint32,
    BytecodeOperandType::JumpTarget,
  ], // AndCheckNot4Chars
  &[BytecodeOperandType::Offset, BytecodeOperandType::JumpTarget], // AdvanceCpAndGoto
  &[
    BytecodeOperandType::Register,
    BytecodeOperandType::JumpTarget,
  ], // CheckNotBackRef
  &[
    BytecodeOperandType::Register,
    BytecodeOperandType::JumpTarget,
  ], // CheckNotBackRefNoCase
  &[
    BytecodeOperandType::Register,
    BytecodeOperandType::JumpTarget,
  ], // CheckNotBackRefNoCaseUnicode
  &[
    BytecodeOperandType::Register,
    BytecodeOperandType::JumpTarget,
  ], // CheckNotBackRefBackward
  &[
    BytecodeOperandType::Register,
    BytecodeOperandType::JumpTarget,
  ], // CheckNotBackRefNoCaseBackward
  &[
    BytecodeOperandType::Register,
    BytecodeOperandType::JumpTarget,
  ], // CheckNotBackRefNoCaseUnicodeBackward
  &[
    BytecodeOperandType::Offset,
    BytecodeOperandType::Offset,
    BytecodeOperandType::BitTable,
    BytecodeOperandType::BoundsCheckOffset,
    BytecodeOperandType::JumpTarget,
    BytecodeOperandType::JumpTarget,
  ], // SkipUntilBitInTable
  &[
    BytecodeOperandType::Offset,
    BytecodeOperandType::Offset,
    BytecodeOperandType::Char,
    BytecodeOperandType::Uint32,
    BytecodeOperandType::BoundsCheckOffset,
    BytecodeOperandType::JumpTarget,
    BytecodeOperandType::JumpTarget,
  ], // SkipUntilCharAnd
  &[
    BytecodeOperandType::Offset,
    BytecodeOperandType::Offset,
    BytecodeOperandType::Char,
    BytecodeOperandType::BoundsCheckOffset,
    BytecodeOperandType::JumpTarget,
    BytecodeOperandType::JumpTarget,
  ], // SkipUntilChar
  &[
    BytecodeOperandType::Offset,
    BytecodeOperandType::Offset,
    BytecodeOperandType::Char,
    BytecodeOperandType::Char,
    BytecodeOperandType::BoundsCheckOffset,
    BytecodeOperandType::JumpTarget,
    BytecodeOperandType::JumpTarget,
  ], // SkipUntilCharOrChar
  &[
    BytecodeOperandType::Offset,
    BytecodeOperandType::Offset,
    BytecodeOperandType::Char,
    BytecodeOperandType::BitTable,
    BytecodeOperandType::BoundsCheckOffset,
    BytecodeOperandType::JumpTarget,
    BytecodeOperandType::JumpTarget,
  ], // SkipUntilGtOrNotBitInTable
  &[
    BytecodeOperandType::Offset,
    BytecodeOperandType::Offset,
    BytecodeOperandType::Uint32,
    BytecodeOperandType::Uint32,
    BytecodeOperandType::BoundsCheckOffset,
    BytecodeOperandType::Uint32,
    BytecodeOperandType::Uint32,
    BytecodeOperandType::Uint32,
    BytecodeOperandType::Uint32,
    BytecodeOperandType::JumpTarget,
    BytecodeOperandType::JumpTarget,
    BytecodeOperandType::JumpTarget,
  ], // SkipUntilOneOfMasked
  &[
    BytecodeOperandType::Offset,
    BytecodeOperandType::Offset,
    BytecodeOperandType::BitTable,
    BytecodeOperandType::BoundsCheckOffset,
    BytecodeOperandType::JumpTarget,
    BytecodeOperandType::Offset,
    BytecodeOperandType::Uint32,
    BytecodeOperandType::Uint32,
    BytecodeOperandType::Offset,
    BytecodeOperandType::BoundsCheckOffset,
    BytecodeOperandType::Offset,
    BytecodeOperandType::Uint32,
    BytecodeOperandType::Uint32,
    BytecodeOperandType::JumpTarget,
    BytecodeOperandType::Uint32,
    BytecodeOperandType::Uint32,
    BytecodeOperandType::JumpTarget,
    BytecodeOperandType::Uint32,
    BytecodeOperandType::Uint32,
    BytecodeOperandType::JumpTarget,
  ], // SkipUntilOneOfMasked3
];

/// detail::kBytecodeFlags (inl.h:318-322)
pub(crate) const BYTECODE_FLAGS: [u32; K_BYTECODE_COUNT as usize] = [
  0, // Break
  0, // PushCurrentPosition
  2, // PushBacktrack
  0, // WriteCurrentPositionToRegister
  0, // ReadCurrentPositionFromRegister
  0, // WriteStackPointerToRegister
  0, // ReadStackPointerFromRegister
  0, // SetRegister
  0, // ClearRegisters
  0, // AdvanceRegister
  0, // PopCurrentPosition
  0, // PushRegister
  0, // PopRegister
  1, // Fail
  1, // Succeed
  0, // AdvanceCurrentPosition
  1, // GoTo
  4, // LoadCurrentCharacter
  0, // CheckPosition
  8, // CheckSpecialClassRanges
  8, // CheckCharacter
  8, // CheckNotCharacter
  8, // CheckCharacterAfterAnd
  8, // CheckNotCharacterAfterAnd
  8, // CheckNotCharacterAfterMinusAnd
  8, // CheckCharacterInRange
  8, // CheckCharacterNotInRange
  8, // CheckCharacterLT
  8, // CheckCharacterGT
  0, // IfRegisterLT
  0, // IfRegisterGE
  0, // IfRegisterEqPos
  0, // CheckAtStart
  0, // CheckNotAtStart
  0, // CheckFixedLengthLoop
  0, // SetCurrentPositionFromEnd
  1, // Backtrack
  4, // LoadCurrentCharacterUnchecked
  8, // CheckBitInTable
  4, // Load2CurrentChars
  4, // Load2CurrentCharsUnchecked
  4, // Load4CurrentChars
  4, // Load4CurrentCharsUnchecked
  8, // Check4Chars
  8, // CheckNot4Chars
  8, // AndCheck4Chars
  8, // AndCheckNot4Chars
  1, // AdvanceCpAndGoto
  0, // CheckNotBackRef
  0, // CheckNotBackRefNoCase
  0, // CheckNotBackRefNoCaseUnicode
  0, // CheckNotBackRefBackward
  0, // CheckNotBackRefNoCaseBackward
  0, // CheckNotBackRefNoCaseUnicodeBackward
  4, // SkipUntilBitInTable
  4, // SkipUntilCharAnd
  4, // SkipUntilChar
  4, // SkipUntilCharOrChar
  4, // SkipUntilGtOrNotBitInTable
  4, // SkipUntilOneOfMasked
  4, // SkipUntilOneOfMasked3
];

/// 全集最大操作数个数 (原版 SkipUntilOneOfMasked3 20 项)
pub const K_MAX_OPERAND_COUNT: usize = 20;

/// 脚本侧独立复刻 inl.h:107-136 的 4 字节块对齐排布所得尺寸基线 —
/// bytecodes.rs 编译期算法表与其逐项 assert (双实现交叉验证)
pub(crate) const GOLDEN_SIZES: [u8; K_BYTECODE_COUNT as usize] = [
  4,  // Break
  4,  // PushCurrentPosition
  8,  // PushBacktrack
  8,  // WriteCurrentPositionToRegister
  4,  // ReadCurrentPositionFromRegister
  4,  // WriteStackPointerToRegister
  4,  // ReadStackPointerFromRegister
  8,  // SetRegister
  8,  // ClearRegisters
  8,  // AdvanceRegister
  4,  // PopCurrentPosition
  8,  // PushRegister
  4,  // PopRegister
  4,  // Fail
  4,  // Succeed
  4,  // AdvanceCurrentPosition
  8,  // GoTo
  12, // LoadCurrentCharacter
  8,  // CheckPosition
  8,  // CheckSpecialClassRanges
  8,  // CheckCharacter
  8,  // CheckNotCharacter
  12, // CheckCharacterAfterAnd
  12, // CheckNotCharacterAfterAnd
  12, // CheckNotCharacterAfterMinusAnd
  12, // CheckCharacterInRange
  12, // CheckCharacterNotInRange
  8,  // CheckCharacterLT
  8,  // CheckCharacterGT
  12, // IfRegisterLT
  12, // IfRegisterGE
  8,  // IfRegisterEqPos
  8,  // CheckAtStart
  8,  // CheckNotAtStart
  8,  // CheckFixedLengthLoop
  4,  // SetCurrentPositionFromEnd
  4,  // Backtrack
  4,  // LoadCurrentCharacterUnchecked
  24, // CheckBitInTable
  12, // Load2CurrentChars
  4,  // Load2CurrentCharsUnchecked
  12, // Load4CurrentChars
  4,  // Load4CurrentCharsUnchecked
  12, // Check4Chars
  12, // CheckNot4Chars
  16, // AndCheck4Chars
  16, // AndCheckNot4Chars
  8,  // AdvanceCpAndGoto
  8,  // CheckNotBackRef
  8,  // CheckNotBackRefNoCase
  8,  // CheckNotBackRefNoCaseUnicode
  8,  // CheckNotBackRefBackward
  8,  // CheckNotBackRefNoCaseBackward
  8,  // CheckNotBackRefNoCaseUnicodeBackward
  36, // SkipUntilBitInTable
  24, // SkipUntilCharAnd
  20, // SkipUntilChar
  24, // SkipUntilCharOrChar
  36, // SkipUntilGtOrNotBitInTable
  48, // SkipUntilOneOfMasked
  92, // SkipUntilOneOfMasked3
];
