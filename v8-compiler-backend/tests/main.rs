use aok::{OK, Void};
use log::info;
use v8_compiler_backend::{
  AtomicWidth, FlagsCondition, FlagsMode, LAZY_DEOPT_ON_THROW_SENTINEL, LaneSize, MemoryAccessMode,
  RecordWriteMode, VectorLength, code_fields, instruction_codes_arm64, instruction_codes_x64,
};
use v8_turboshaft::{Graph, Origin};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_instruction_codes_common() -> Void {
  info!("Testing instruction codes common definitions");

  assert_eq!(LAZY_DEOPT_ON_THROW_SENTINEL, -1);

  // FlagsMode
  assert_eq!(FlagsMode::None as u8, 0);
  assert_eq!(FlagsMode::Branch as u8, 1);
  assert_eq!(FlagsMode::Deoptimize as u8, 2);
  assert_eq!(FlagsMode::Set as u8, 3);
  assert_eq!(FlagsMode::Trap as u8, 4);
  assert_eq!(FlagsMode::Select as u8, 5);
  assert_eq!(FlagsMode::ConditionalTrap as u8, 6);
  assert_eq!(FlagsMode::ConditionalBranch as u8, 7);

  // FlagsCondition
  assert_eq!(FlagsCondition::Equal as u8, 0);
  assert_eq!(FlagsCondition::NotEqual as u8, 1);
  assert_eq!(FlagsCondition::Equal.negate(), FlagsCondition::NotEqual);
  assert_eq!(
    FlagsCondition::SignedLessThan.negate(),
    FlagsCondition::SignedGreaterThanOrEqual
  );
  assert_eq!(
    FlagsCondition::UnsignedLessThan.negate(),
    FlagsCondition::UnsignedGreaterThanOrEqual
  );
  assert_eq!(
    FlagsCondition::STACK_POINTER_GREATER_THAN_CONDITION,
    FlagsCondition::from_repr(FlagsCondition::STACK_POINTER_GREATER_THAN_CONDITION as u8)
  );

  // RecordWriteMode
  assert_eq!(
    RecordWriteMode::from_write_barrier_kind("kMapWriteBarrier"),
    Some(RecordWriteMode::ValueIsMap)
  );
  assert_eq!(
    RecordWriteMode::from_write_barrier_kind("kPointerWriteBarrier"),
    Some(RecordWriteMode::ValueIsPointer)
  );
  assert_eq!(
    RecordWriteMode::from_write_barrier_kind("kFullWriteBarrier"),
    Some(RecordWriteMode::ValueIsAny)
  );
  assert_eq!(
    RecordWriteMode::from_write_barrier_kind("kUnknownBarrier"),
    None
  );

  // MemoryAccessMode & AtomicWidth
  assert_eq!(MemoryAccessMode::Direct as u8, 0);
  assert_eq!(MemoryAccessMode::Trapping as u8, 1);
  assert_eq!(AtomicWidth::Word32.size(), 4);
  assert_eq!(AtomicWidth::Word64.size(), 8);

  // LaneSize
  assert_eq!(LaneSize::L8.bits(), 8);
  assert_eq!(LaneSize::L16.bits(), 16);
  assert_eq!(LaneSize::L32.bits(), 32);
  assert_eq!(LaneSize::L64.bits(), 64);
  assert_eq!(LaneSize::from_bits(8), Some(LaneSize::L8));
  assert_eq!(LaneSize::from_bits(32), Some(LaneSize::L32));
  assert_eq!(LaneSize::from_bits(128), None);

  // VectorLength
  assert_eq!(VectorLength::V64.bits(), 64);
  assert_eq!(VectorLength::V128.bits(), 128);
  assert_eq!(VectorLength::V256.bits(), 256);
  assert_eq!(VectorLength::V512.bits(), 512);

  // code_fields
  assert_eq!(code_fields::ARCH_OPCODE_FIELD.0, 0);
  assert_eq!(code_fields::ARCH_OPCODE_FIELD.1, 10);
  assert_eq!(code_fields::ADDRESSING_MODE_FIELD.0, 10);
  assert_eq!(code_fields::ADDRESSING_MODE_FIELD.1, 5);
  assert_eq!(code_fields::FLAGS_MODE_FIELD.0, 15);
  assert_eq!(code_fields::FLAGS_MODE_FIELD.1, 3);
  assert_eq!(code_fields::FLAGS_CONDITION_FIELD.0, 18);
  assert_eq!(code_fields::FLAGS_CONDITION_FIELD.1, 5);
  assert_eq!(code_fields::MISC_FIELD.0, 23);
  assert_eq!(code_fields::MISC_FIELD.1, 9);

  OK
}

#[test]
fn test_instruction_codes_x64() -> Void {
  info!("Testing x64 instruction codes");
  use instruction_codes_x64::{AddressingMode, ArchOpcode, HAS_MEMORY_ACCESS_MODE};

  assert_eq!(ArchOpcode::ArchTailCallCodeObject as u16, 0);
  assert_eq!(ArchOpcode::ArchJmp as u16, 10);
  assert_eq!(ArchOpcode::ArchNop as u16, 13);
  const { assert!(instruction_codes_x64::LAST_ARCH_OPCODE > 300) };

  assert_eq!(AddressingMode::ModeNone as u16, 0);
  assert_eq!(AddressingMode::ModeMR as u16, 1);
  assert_eq!(AddressingMode::ModeMRI as u16, 2);
  const { assert!(instruction_codes_x64::LAST_ADDRESSING_MODE >= 20) };

  assert!(HAS_MEMORY_ACCESS_MODE.contains(&ArchOpcode::AtomicLoadInt8));
  assert!(!HAS_MEMORY_ACCESS_MODE.contains(&ArchOpcode::ArchNop));

  OK
}

#[test]
fn test_instruction_codes_arm64() -> Void {
  info!("Testing arm64 instruction codes");
  use instruction_codes_arm64::{AddressingMode, ArchOpcode, HAS_MEMORY_ACCESS_MODE};

  assert_eq!(ArchOpcode::ArchTailCallCodeObject as u16, 0);
  assert_eq!(ArchOpcode::ArchJmp as u16, 10);
  assert_eq!(ArchOpcode::ArchNop as u16, 13);
  const { assert!(instruction_codes_arm64::LAST_ARCH_OPCODE > 200) };

  assert_eq!(AddressingMode::ModeNone as u16, 0);
  assert_eq!(AddressingMode::ModeMRI as u16, 1);
  assert_eq!(AddressingMode::ModeMRR as u16, 2);
  const { assert!(instruction_codes_arm64::LAST_ADDRESSING_MODE >= 12) };

  assert!(HAS_MEMORY_ACCESS_MODE.contains(&ArchOpcode::AtomicLoadInt8));
  assert!(!HAS_MEMORY_ACCESS_MODE.contains(&ArchOpcode::ArchNop));

  OK
}

#[test]
fn test_turboshaft_instruction_selection() -> Void {
  use v8_compiler_backend::{InstructionOperand, InstructionSelector};
  use v8_turboshaft::BlockKind;

  // 1. 空图选择
  let graph = Graph::new(Origin::PureTurboshaft);
  let code = v8_compiler_backend::select_instructions_from_turboshaft(&graph);
  assert_eq!(code, 0);

  // 2. 具备真实基本块的图选择 (覆盖 BranchTarget, Merge, LoopHeader)
  let mut graph = Graph::new(Origin::PureTurboshaft);
  let b0 = graph.new_block(BlockKind::BranchTarget);
  let b1 = graph.new_block(BlockKind::Merge);
  let b2 = graph.new_block(BlockKind::LoopHeader);
  assert_eq!(b0.id(), 0);
  assert_eq!(b1.id(), 1);
  assert_eq!(b2.id(), 2);

  let selector = InstructionSelector::new(&graph);
  let seq = selector.select();
  assert_eq!(seq.blocks.len(), 3);
  assert_eq!(seq.total_instruction_count(), 3);

  // 检验首个 BranchTarget 块发射的指令及其操作数
  let block0 = &seq.blocks[0];
  assert_eq!(block0.block_index, 0);
  assert_eq!(block0.instructions.len(), 1);
  let inst0 = &block0.instructions[0];
  assert_eq!(inst0.flags_mode, FlagsMode::Branch);
  assert_eq!(inst0.flags_condition, FlagsCondition::Equal);
  assert_eq!(inst0.inputs, vec![InstructionOperand::Immediate(0)]);

  // 检验 LoopHeader 块发射的指令
  let block2 = &seq.blocks[2];
  assert_eq!(block2.block_index, 2);
  assert_eq!(block2.instructions.len(), 1);

  // 检验操作数 Copy 语义与无额外分配
  let op = InstructionOperand::Immediate(42);
  let op_copy = op;
  assert_eq!(op, op_copy);

  // 检验通过快捷函数获取首条指令码
  let code2 = v8_compiler_backend::select_instructions_from_turboshaft(&graph);
  assert_eq!(code2, inst0.opcode);
  assert_ne!(code2, 0);

  OK
}
