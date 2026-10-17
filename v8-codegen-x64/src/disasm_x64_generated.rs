// 由 tool/src/diagnostics/x64/disasm-x64.cc.js 从 v8/src/diagnostics/x64/disasm-x64.cc 生成; 修订就地改, 复跑覆盖
// x64 反汇编规则面: 枚举/助记表/256 项指令表/寄存器名表

// 原文件为 #if V8_TARGET_ARCH_X64 门控 (x64 专用); 本产物即 x64 目标展开。
// 语义解码实现 (DisassemblerX64::InstructionDecode) 不在机械提取范围, 由 Developer 修订补齐。

/// 操作数编排方式 (原 enum OperandType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OperandType {
  /// 原 `UNSET_OP_ORDER`
  UnsetOpOrder = 0,
  /// 原 `REG_OPER_OP_ORDER`
  RegOperOpOrder = 1,
  /// 原 `OPER_REG_OP_ORDER`
  OperRegOpOrder = 2,
  /// 原 `BYTE_SIZE_OPERAND_FLAG`
  ByteSizeOperandFlag = 4,
  /// 原 `BYTE_REG_OPER_OP_ORDER`
  ByteRegOperOpOrder = 5,
  /// 原 `BYTE_OPER_REG_OP_ORDER`
  ByteOperRegOpOrder = 6,
  /// 原 `OPER_XMMREG_OP_ORDER`
  OperXmmregOpOrder = 7,
  /// 原 `XMMREG_OPER_OP_ORDER`
  XmmregOperOpOrder = 8,
  /// 原 `XMMREG_XMMOPER_OP_ORDER`
  XmmregXmmoperOpOrder = 9,
  /// 原 `XMMOPER_XMMREG_OP_ORDER`
  XmmoperXmmregOpOrder = 10,
}

/// 指令形态 (原 enum InstructionType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InstructionType {
  /// 原 `NO_INSTR`
  NoInstr = 0,
  /// 原 `ZERO_OPERANDS_INSTR`
  ZeroOperandsInstr = 1,
  /// 原 `TWO_OPERANDS_INSTR`
  TwoOperandsInstr = 2,
  /// 原 `JUMP_CONDITIONAL_SHORT_INSTR`
  JumpConditionalShortInstr = 3,
  /// 原 `REGISTER_INSTR`
  RegisterInstr = 4,
  /// 原 `PUSHPOP_INSTR`
  PushpopInstr = 5,
  /// 原 `MOVE_REG_INSTR`
  MoveRegInstr = 6,
  /// 原 `CALL_JUMP_INSTR`
  CallJumpInstr = 7,
  /// 原 `SHORT_IMMEDIATE_INSTR`
  ShortImmediateInstr = 8,
}

/// 前缀字节 (原 enum Prefixes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Prefixes {
  /// 原 `ESCAPE_PREFIX`
  EscapePrefix = 0xF,
  /// 原 `EVEX_PREFIX`
  EvexPrefix = 0x62,
  /// 原 `SEGMENT_FS_OVERRIDE_PREFIX`
  SegmentFsOverridePrefix = 0x64,
  /// 原 `OPERAND_SIZE_OVERRIDE_PREFIX`
  OperandSizeOverridePrefix = 0x66,
  /// 原 `ADDRESS_SIZE_OVERRIDE_PREFIX`
  AddressSizeOverridePrefix = 0x67,
  /// 原 `VEX3_PREFIX`
  Vex3Prefix = 0xC4,
  /// 原 `VEX2_PREFIX`
  Vex2Prefix = 0xC5,
  /// 原 `REX2_PREFIX`
  Rex2Prefix = 0xD5,
  /// 原 `LOCK_PREFIX`
  LockPrefix = 0xF0,
  /// 原 `REPNE_PREFIX`
  RepnePrefix = 0xF2,
  /// 原 `REP_PREFIX`
  RepPrefix = 0xF3,
}

/// 原 REPEQ_PREFIX 是 REP_PREFIX 的别名 (变体名 PascalCase 化)
pub const REPEQ_PREFIX: Prefixes = Prefixes::RepPrefix;

/// 单字节助记项 (原 struct ByteMnemonic)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteMnemonic {
  /// 机器码字节; 原版以 -1 终止表, 此处展开为定长表不再需要终止项
  pub b: u8,
  pub op_order: OperandType,
  pub mnem: &'static str,
}

/// 展开原 two_operands_instr (双操作数指令, 42 项)
#[rustfmt::skip]
pub static TWO_OPERANDS_INSTR: [ByteMnemonic; 42] = [
  ByteMnemonic { b: 0x00, op_order: OperandType::ByteOperRegOpOrder, mnem: "add" },
  ByteMnemonic { b: 0x01, op_order: OperandType::OperRegOpOrder, mnem: "add" },
  ByteMnemonic { b: 0x02, op_order: OperandType::ByteRegOperOpOrder, mnem: "add" },
  ByteMnemonic { b: 0x03, op_order: OperandType::RegOperOpOrder, mnem: "add" },
  ByteMnemonic { b: 0x08, op_order: OperandType::ByteOperRegOpOrder, mnem: "or" },
  ByteMnemonic { b: 0x09, op_order: OperandType::OperRegOpOrder, mnem: "or" },
  ByteMnemonic { b: 0x0A, op_order: OperandType::ByteRegOperOpOrder, mnem: "or" },
  ByteMnemonic { b: 0x0B, op_order: OperandType::RegOperOpOrder, mnem: "or" },
  ByteMnemonic { b: 0x10, op_order: OperandType::ByteOperRegOpOrder, mnem: "adc" },
  ByteMnemonic { b: 0x11, op_order: OperandType::OperRegOpOrder, mnem: "adc" },
  ByteMnemonic { b: 0x12, op_order: OperandType::ByteRegOperOpOrder, mnem: "adc" },
  ByteMnemonic { b: 0x13, op_order: OperandType::RegOperOpOrder, mnem: "adc" },
  ByteMnemonic { b: 0x18, op_order: OperandType::ByteOperRegOpOrder, mnem: "sbb" },
  ByteMnemonic { b: 0x19, op_order: OperandType::OperRegOpOrder, mnem: "sbb" },
  ByteMnemonic { b: 0x1A, op_order: OperandType::ByteRegOperOpOrder, mnem: "sbb" },
  ByteMnemonic { b: 0x1B, op_order: OperandType::RegOperOpOrder, mnem: "sbb" },
  ByteMnemonic { b: 0x20, op_order: OperandType::ByteOperRegOpOrder, mnem: "and" },
  ByteMnemonic { b: 0x21, op_order: OperandType::OperRegOpOrder, mnem: "and" },
  ByteMnemonic { b: 0x22, op_order: OperandType::ByteRegOperOpOrder, mnem: "and" },
  ByteMnemonic { b: 0x23, op_order: OperandType::RegOperOpOrder, mnem: "and" },
  ByteMnemonic { b: 0x28, op_order: OperandType::ByteOperRegOpOrder, mnem: "sub" },
  ByteMnemonic { b: 0x29, op_order: OperandType::OperRegOpOrder, mnem: "sub" },
  ByteMnemonic { b: 0x2A, op_order: OperandType::ByteRegOperOpOrder, mnem: "sub" },
  ByteMnemonic { b: 0x2B, op_order: OperandType::RegOperOpOrder, mnem: "sub" },
  ByteMnemonic { b: 0x30, op_order: OperandType::ByteOperRegOpOrder, mnem: "xor" },
  ByteMnemonic { b: 0x31, op_order: OperandType::OperRegOpOrder, mnem: "xor" },
  ByteMnemonic { b: 0x32, op_order: OperandType::ByteRegOperOpOrder, mnem: "xor" },
  ByteMnemonic { b: 0x33, op_order: OperandType::RegOperOpOrder, mnem: "xor" },
  ByteMnemonic { b: 0x38, op_order: OperandType::ByteOperRegOpOrder, mnem: "cmp" },
  ByteMnemonic { b: 0x39, op_order: OperandType::OperRegOpOrder, mnem: "cmp" },
  ByteMnemonic { b: 0x3A, op_order: OperandType::ByteRegOperOpOrder, mnem: "cmp" },
  ByteMnemonic { b: 0x3B, op_order: OperandType::RegOperOpOrder, mnem: "cmp" },
  ByteMnemonic { b: 0x63, op_order: OperandType::RegOperOpOrder, mnem: "movsxl" },
  ByteMnemonic { b: 0x84, op_order: OperandType::ByteRegOperOpOrder, mnem: "test" },
  ByteMnemonic { b: 0x85, op_order: OperandType::RegOperOpOrder, mnem: "test" },
  ByteMnemonic { b: 0x86, op_order: OperandType::ByteRegOperOpOrder, mnem: "xchg" },
  ByteMnemonic { b: 0x87, op_order: OperandType::RegOperOpOrder, mnem: "xchg" },
  ByteMnemonic { b: 0x88, op_order: OperandType::ByteOperRegOpOrder, mnem: "mov" },
  ByteMnemonic { b: 0x89, op_order: OperandType::OperRegOpOrder, mnem: "mov" },
  ByteMnemonic { b: 0x8A, op_order: OperandType::ByteRegOperOpOrder, mnem: "mov" },
  ByteMnemonic { b: 0x8B, op_order: OperandType::RegOperOpOrder, mnem: "mov" },
  ByteMnemonic { b: 0x8D, op_order: OperandType::RegOperOpOrder, mnem: "lea" },
];

/// 展开原 zero_operands_instr (零操作数指令, 17 项)
#[rustfmt::skip]
pub static ZERO_OPERANDS_INSTR: [ByteMnemonic; 17] = [
  ByteMnemonic { b: 0xC3, op_order: OperandType::UnsetOpOrder, mnem: "ret" },
  ByteMnemonic { b: 0xC9, op_order: OperandType::UnsetOpOrder, mnem: "leave" },
  ByteMnemonic { b: 0xF4, op_order: OperandType::UnsetOpOrder, mnem: "hlt" },
  ByteMnemonic { b: 0xFC, op_order: OperandType::UnsetOpOrder, mnem: "cld" },
  ByteMnemonic { b: 0xCC, op_order: OperandType::UnsetOpOrder, mnem: "int3" },
  ByteMnemonic { b: 0x60, op_order: OperandType::UnsetOpOrder, mnem: "pushad" },
  ByteMnemonic { b: 0x61, op_order: OperandType::UnsetOpOrder, mnem: "popad" },
  ByteMnemonic { b: 0x9C, op_order: OperandType::UnsetOpOrder, mnem: "pushfd" },
  ByteMnemonic { b: 0x9D, op_order: OperandType::UnsetOpOrder, mnem: "popfd" },
  ByteMnemonic { b: 0x9E, op_order: OperandType::UnsetOpOrder, mnem: "sahf" },
  ByteMnemonic { b: 0x99, op_order: OperandType::UnsetOpOrder, mnem: "cdq" },
  ByteMnemonic { b: 0x9B, op_order: OperandType::UnsetOpOrder, mnem: "fwait" },
  ByteMnemonic { b: 0xAB, op_order: OperandType::UnsetOpOrder, mnem: "stos" },
  ByteMnemonic { b: 0xA4, op_order: OperandType::UnsetOpOrder, mnem: "movs" },
  ByteMnemonic { b: 0xA5, op_order: OperandType::UnsetOpOrder, mnem: "movs" },
  ByteMnemonic { b: 0xA6, op_order: OperandType::UnsetOpOrder, mnem: "cmps" },
  ByteMnemonic { b: 0xA7, op_order: OperandType::UnsetOpOrder, mnem: "cmps" },
];

/// 展开原 call_jump_instr (call/jmp 直移, 2 项)
#[rustfmt::skip]
pub static CALL_JUMP_INSTR: [ByteMnemonic; 2] = [
  ByteMnemonic { b: 0xE8, op_order: OperandType::UnsetOpOrder, mnem: "call" },
  ByteMnemonic { b: 0xE9, op_order: OperandType::UnsetOpOrder, mnem: "jmp" },
];

/// 展开原 short_immediate_instr (短立即数指令, 8 项)
#[rustfmt::skip]
pub static SHORT_IMMEDIATE_INSTR: [ByteMnemonic; 8] = [
  ByteMnemonic { b: 0x05, op_order: OperandType::UnsetOpOrder, mnem: "add" },
  ByteMnemonic { b: 0x0D, op_order: OperandType::UnsetOpOrder, mnem: "or" },
  ByteMnemonic { b: 0x15, op_order: OperandType::UnsetOpOrder, mnem: "adc" },
  ByteMnemonic { b: 0x1D, op_order: OperandType::UnsetOpOrder, mnem: "sbb" },
  ByteMnemonic { b: 0x25, op_order: OperandType::UnsetOpOrder, mnem: "and" },
  ByteMnemonic { b: 0x2D, op_order: OperandType::UnsetOpOrder, mnem: "sub" },
  ByteMnemonic { b: 0x35, op_order: OperandType::UnsetOpOrder, mnem: "xor" },
  ByteMnemonic { b: 0x3D, op_order: OperandType::UnsetOpOrder, mnem: "cmp" },
];

/// 条件码后缀表 (原 conditional_code_suffix[16], 索引即条件码)
#[rustfmt::skip]
pub static CONDITIONAL_CODE_SUFFIX: [&str; 16] = [
  "o",
  "no",
  "c",
  "nc",
  "z",
  "nz",
  "na",
  "a",
  "s",
  "ns",
  "pe",
  "po",
  "l",
  "ge",
  "le",
  "g",
];

/// CMOV 指令助记表 (原 cmov_instructions[16], 索引即条件码)
#[rustfmt::skip]
pub static CMOV_MNEMONICS: [&str; 16] = [
  "cmovo",
  "cmovno",
  "cmovc",
  "cmovnc",
  "cmovz",
  "cmovnz",
  "cmovna",
  "cmova",
  "cmovs",
  "cmovns",
  "cmovpe",
  "cmovpo",
  "cmovl",
  "cmovge",
  "cmovle",
  "cmovg",
];

/// SSE compare 伪操作表 (原 cmp_pseudo_op[16])
#[rustfmt::skip]
pub static CMP_PSEUDO_OP: [&str; 16] = [
  "eq",
  "lt",
  "le",
  "unord",
  "neq",
  "nlt",
  "nle",
  "ord",
  "eq_uq",
  "nge",
  "ngt",
  "false",
  "neq_oq",
  "ge",
  "gt",
  "true",
];

/// 指令描述 (原 struct InstructionDesc)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstructionDesc {
  /// 助记符; 原版短跳转项为 nullptr (由条件码现算), 此处用 empty_str 标记
  pub mnem: &'static str,
  pub ty: InstructionType,
  pub op_order: OperandType,
  /// 定长 8 位操作
  pub byte_size_operation: bool,
}

/// 256 项单字节指令终态表 — 机械执行原 InstructionTable::Clear/Init/
/// CopyTable/SetTableRange/AddJumpConditionalShort 后的展开结果。
#[rustfmt::skip]
pub static INSTRUCTION_TABLE: [InstructionDesc; 256] = [
  InstructionDesc { mnem: "add", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::OperRegOpOrder, byte_size_operation: true }, InstructionDesc { mnem: "add", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::OperRegOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "add", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: true }, InstructionDesc { mnem: "add", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "add", ty: InstructionType::ShortImmediateInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "or", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::OperRegOpOrder, byte_size_operation: true }, InstructionDesc { mnem: "or", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::OperRegOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "or", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: true }, InstructionDesc { mnem: "or", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "or", ty: InstructionType::ShortImmediateInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "adc", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::OperRegOpOrder, byte_size_operation: true }, InstructionDesc { mnem: "adc", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::OperRegOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "adc", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: true }, InstructionDesc { mnem: "adc", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "adc", ty: InstructionType::ShortImmediateInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "sbb", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::OperRegOpOrder, byte_size_operation: true }, InstructionDesc { mnem: "sbb", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::OperRegOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "sbb", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: true }, InstructionDesc { mnem: "sbb", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "sbb", ty: InstructionType::ShortImmediateInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "and", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::OperRegOpOrder, byte_size_operation: true }, InstructionDesc { mnem: "and", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::OperRegOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "and", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: true }, InstructionDesc { mnem: "and", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "and", ty: InstructionType::ShortImmediateInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "sub", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::OperRegOpOrder, byte_size_operation: true }, InstructionDesc { mnem: "sub", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::OperRegOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "sub", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: true }, InstructionDesc { mnem: "sub", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "sub", ty: InstructionType::ShortImmediateInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "xor", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::OperRegOpOrder, byte_size_operation: true }, InstructionDesc { mnem: "xor", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::OperRegOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "xor", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: true }, InstructionDesc { mnem: "xor", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "xor", ty: InstructionType::ShortImmediateInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "cmp", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::OperRegOpOrder, byte_size_operation: true }, InstructionDesc { mnem: "cmp", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::OperRegOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "cmp", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: true }, InstructionDesc { mnem: "cmp", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "cmp", ty: InstructionType::ShortImmediateInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "push", ty: InstructionType::PushpopInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "push", ty: InstructionType::PushpopInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "push", ty: InstructionType::PushpopInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "push", ty: InstructionType::PushpopInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "push", ty: InstructionType::PushpopInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "push", ty: InstructionType::PushpopInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "push", ty: InstructionType::PushpopInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "push", ty: InstructionType::PushpopInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "pop", ty: InstructionType::PushpopInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "pop", ty: InstructionType::PushpopInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "pop", ty: InstructionType::PushpopInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "pop", ty: InstructionType::PushpopInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "pop", ty: InstructionType::PushpopInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "pop", ty: InstructionType::PushpopInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "pop", ty: InstructionType::PushpopInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "pop", ty: InstructionType::PushpopInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "pushad", ty: InstructionType::ZeroOperandsInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "popad", ty: InstructionType::ZeroOperandsInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "movsxl", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "", ty: InstructionType::JumpConditionalShortInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "", ty: InstructionType::JumpConditionalShortInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "", ty: InstructionType::JumpConditionalShortInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "", ty: InstructionType::JumpConditionalShortInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "", ty: InstructionType::JumpConditionalShortInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "", ty: InstructionType::JumpConditionalShortInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "", ty: InstructionType::JumpConditionalShortInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "", ty: InstructionType::JumpConditionalShortInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "", ty: InstructionType::JumpConditionalShortInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "", ty: InstructionType::JumpConditionalShortInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "", ty: InstructionType::JumpConditionalShortInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "", ty: InstructionType::JumpConditionalShortInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "", ty: InstructionType::JumpConditionalShortInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "", ty: InstructionType::JumpConditionalShortInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "", ty: InstructionType::JumpConditionalShortInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "", ty: InstructionType::JumpConditionalShortInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "test", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: true }, InstructionDesc { mnem: "test", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "xchg", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: true }, InstructionDesc { mnem: "xchg", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "mov", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::OperRegOpOrder, byte_size_operation: true }, InstructionDesc { mnem: "mov", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::OperRegOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "mov", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: true }, InstructionDesc { mnem: "mov", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "lea", ty: InstructionType::TwoOperandsInstr, op_order: OperandType::RegOperOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "cdq", ty: InstructionType::ZeroOperandsInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "fwait", ty: InstructionType::ZeroOperandsInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "pushfd", ty: InstructionType::ZeroOperandsInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "popfd", ty: InstructionType::ZeroOperandsInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "sahf", ty: InstructionType::ZeroOperandsInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "movs", ty: InstructionType::ZeroOperandsInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "movs", ty: InstructionType::ZeroOperandsInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "cmps", ty: InstructionType::ZeroOperandsInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "cmps", ty: InstructionType::ZeroOperandsInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "stos", ty: InstructionType::ZeroOperandsInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "mov", ty: InstructionType::MoveRegInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "mov", ty: InstructionType::MoveRegInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "mov", ty: InstructionType::MoveRegInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "mov", ty: InstructionType::MoveRegInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "mov", ty: InstructionType::MoveRegInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "mov", ty: InstructionType::MoveRegInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "mov", ty: InstructionType::MoveRegInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "mov", ty: InstructionType::MoveRegInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "ret", ty: InstructionType::ZeroOperandsInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "leave", ty: InstructionType::ZeroOperandsInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "int3", ty: InstructionType::ZeroOperandsInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "call", ty: InstructionType::CallJumpInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "jmp", ty: InstructionType::CallJumpInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "hlt", ty: InstructionType::ZeroOperandsInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
  InstructionDesc { mnem: "cld", ty: InstructionType::ZeroOperandsInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false }, InstructionDesc { mnem: "(bad)", ty: InstructionType::NoInstr, op_order: OperandType::UnsetOpOrder, byte_size_operation: false },
];

/// 判定短跳转项 (原版 mnem 为 nullptr, 条件码现算)
#[inline]
#[must_use]
pub const fn is_jump_conditional_short(desc: &InstructionDesc) -> bool {
  desc.mnem.is_empty() && matches!(desc.ty, InstructionType::JumpConditionalShortInstr)
}

// —— 寄存器名表 (原文件尾部, X-macro MAKE_REG_NAME 展开) ——

/// 原 cpu_regs = GENERAL_REGISTERS(MAKE_REG_NAME) 展开
#[rustfmt::skip]
pub static CPU_REGS: [&str; 16] = [
  "rax",
  "rcx",
  "rdx",
  "rbx",
  "rsp",
  "rbp",
  "rsi",
  "rdi",
  "r8",
  "r9",
  "r10",
  "r11",
  "r12",
  "r13",
  "r14",
  "r15",
];

/// 原 byte_cpu_regs[16] (REX 前 8 位寄存器名带后缀)
#[rustfmt::skip]
pub static BYTE_CPU_REGS: [&str; 16] = [
  "al",
  "cl",
  "dl",
  "bl",
  "spl",
  "bpl",
  "sil",
  "dil",
  "r8l",
  "r9l",
  "r10l",
  "r11l",
  "r12l",
  "r13l",
  "r14l",
  "r15l",
];

/// 原 xmm_regs = DOUBLE_REGISTERS_AVX512(MAKE_REG_NAME) 展开
#[rustfmt::skip]
pub static XMM_REGS: [&str; 32] = [
  "xmm0",
  "xmm1",
  "xmm2",
  "xmm3",
  "xmm4",
  "xmm5",
  "xmm6",
  "xmm7",
  "xmm8",
  "xmm9",
  "xmm10",
  "xmm11",
  "xmm12",
  "xmm13",
  "xmm14",
  "xmm15",
  "xmm16",
  "xmm17",
  "xmm18",
  "xmm19",
  "xmm20",
  "xmm21",
  "xmm22",
  "xmm23",
  "xmm24",
  "xmm25",
  "xmm26",
  "xmm27",
  "xmm28",
  "xmm29",
  "xmm30",
  "xmm31",
];

/// 原 ymm_regs = YMM_REGISTERS_AVX512(MAKE_REG_NAME) 展开
#[rustfmt::skip]
pub static YMM_REGS: [&str; 32] = [
  "ymm0",
  "ymm1",
  "ymm2",
  "ymm3",
  "ymm4",
  "ymm5",
  "ymm6",
  "ymm7",
  "ymm8",
  "ymm9",
  "ymm10",
  "ymm11",
  "ymm12",
  "ymm13",
  "ymm14",
  "ymm15",
  "ymm16",
  "ymm17",
  "ymm18",
  "ymm19",
  "ymm20",
  "ymm21",
  "ymm22",
  "ymm23",
  "ymm24",
  "ymm25",
  "ymm26",
  "ymm27",
  "ymm28",
  "ymm29",
  "ymm30",
  "ymm31",
];

/// 原 zmm_regs[32]
#[rustfmt::skip]
pub static ZMM_REGS: [&str; 32] = [
  "zmm0",
  "zmm1",
  "zmm2",
  "zmm3",
  "zmm4",
  "zmm5",
  "zmm6",
  "zmm7",
  "zmm8",
  "zmm9",
  "zmm10",
  "zmm11",
  "zmm12",
  "zmm13",
  "zmm14",
  "zmm15",
  "zmm16",
  "zmm17",
  "zmm18",
  "zmm19",
  "zmm20",
  "zmm21",
  "zmm22",
  "zmm23",
  "zmm24",
  "zmm25",
  "zmm26",
  "zmm27",
  "zmm28",
  "zmm29",
  "zmm30",
  "zmm31",
];

// —— 编译期可用原语: str 逐字节等值 (const 上下文 PartialEq<&str> 不可用) ——
#[inline]
#[must_use]
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(INSTRUCTION_TABLE.len() == 256);
  assert!(str_eq(INSTRUCTION_TABLE[0x00].mnem, "add"));
  assert!(matches!(INSTRUCTION_TABLE[0x50].ty, InstructionType::PushpopInstr));
  assert!(matches!(INSTRUCTION_TABLE[0x70].ty, InstructionType::JumpConditionalShortInstr));
  assert!(str_eq(INSTRUCTION_TABLE[0xB8].mnem, "mov"));
  assert!(str_eq(INSTRUCTION_TABLE[0xE8].mnem, "call"));
  assert!(str_eq(CMOV_MNEMONICS[0], "cmovo"));
  assert!(str_eq(CMP_PSEUDO_OP[0], "eq"));
  assert!(str_eq(CONDITIONAL_CODE_SUFFIX[0], "o"));
  assert!(str_eq(CPU_REGS[0], "rax"));
  assert!(str_eq(BYTE_CPU_REGS[0], "al"));
  assert!(str_eq(XMM_REGS[0], "xmm0"));
  assert!(str_eq(YMM_REGS[0], "ymm0"));
  assert!(str_eq(ZMM_REGS[31], "zmm31"));
};
