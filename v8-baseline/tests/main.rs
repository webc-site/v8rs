use aok::{OK, Void};
use v8_baseline::{
  ARM64_FALLBACK_BUILTIN_CALL_JUMP_MODE_FOR_BASELINE, ARM64_SCRATCH_REGISTER_SCOPE_DECLARATIONS,
  BASELINE_ASSEMBLER_DECLARATIONS, BASELINE_BATCH_COMPILER_DECLARATIONS,
  BASELINE_COMPILER_DECLARATIONS, BYTECODE_OFFSET_ITERATOR_DECLARATIONS,
  BYTECODE_OFFSET_TABLE_BUILDER_DECLARATIONS, BuiltinCallJumpMode,
  COMPARE_BRANCH_CANDIDATE_DECLARATIONS, CompressionMode, EFFECT_STATE_DECLARATIONS,
  ENSURE_ACCUMULATOR_PRESERVED_SCOPE_DECLARATIONS, ENTRY_SIGNATURES, Error, HELPER_TEMPLATE_COUNT,
  INITIAL_QUEUE_SIZE, INSTRUCTIONS_PER_JUMP_TARGET, INSTRUCTIONS_PER_LABEL, LOOP_UNROLL_SIZE,
  NUM_SCRATCH_REGISTERS, SAVE_ACCUMULATOR_SCOPE_DECLARATIONS, SCRATCH_REGISTERS, SKIP_REGISTRATION,
  StackCheckBehavior, VISIT_BYTECODE_METHODS, VISIT_INTRINSIC_METHODS,
  X64_FALLBACK_BUILTIN_CALL_JUMP_MODE_FOR_BASELINE, X64_SCRATCH_REGISTER_SCOPE_DECLARATIONS,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_baseline_entry_and_batch_compiler() -> Void {
  assert_eq!(ENTRY_SIGNATURES.len(), 3);
  assert!(ENTRY_SIGNATURES[0].contains("CanCompileWithBaseline"));
  assert!(ENTRY_SIGNATURES[1].contains("GenerateBaselineCode"));
  assert!(ENTRY_SIGNATURES[2].contains("EmitReturnBaseline"));

  assert_eq!(INITIAL_QUEUE_SIZE, 32);
  assert_eq!(BASELINE_BATCH_COMPILER_DECLARATIONS.len(), 22);
  assert!(
    BASELINE_BATCH_COMPILER_DECLARATIONS
      .iter()
      .any(|d| d.contains("EnqueueFunction"))
  );
  assert!(
    BASELINE_BATCH_COMPILER_DECLARATIONS
      .iter()
      .any(|d| d.contains("InstallBatch"))
  );
  OK
}

#[test]
fn test_baseline_assembler_and_scopes() -> Void {
  assert_eq!(CompressionMode::Default as u8, 0);
  assert_eq!(CompressionMode::ForceDecompression as u8, 1);
  assert!(CompressionMode::Default.is_default());
  assert!(!CompressionMode::Default.is_force_decompression());
  assert!(CompressionMode::ForceDecompression.is_force_decompression());
  assert!(!CompressionMode::ForceDecompression.is_default());

  let parsed: CompressionMode = "ForceDecompression"
    .parse()
    .map_err(|e| Error::Message(format!("{e}")))?;
  assert_eq!(parsed, CompressionMode::ForceDecompression);

  assert_eq!(SKIP_REGISTRATION.len(), 14);
  assert_eq!(BASELINE_ASSEMBLER_DECLARATIONS.len(), 99);
  assert_eq!(ENSURE_ACCUMULATOR_PRESERVED_SCOPE_DECLARATIONS.len(), 5);
  OK
}

#[test]
fn test_baseline_compiler_and_bytecodes() -> Void {
  assert_eq!(StackCheckBehavior::EnableStackCheck as u8, 0);
  assert_eq!(StackCheckBehavior::DisableStackCheck as u8, 1);
  assert!(StackCheckBehavior::EnableStackCheck.is_enabled());
  assert!(!StackCheckBehavior::EnableStackCheck.is_disabled());
  assert!(StackCheckBehavior::DisableStackCheck.is_disabled());

  assert_eq!(BuiltinCallJumpMode::Absolute as u8, 0);
  assert_eq!(BuiltinCallJumpMode::PcRelative as u8, 1);
  assert_eq!(BuiltinCallJumpMode::Indirect as u8, 2);
  assert_eq!(BuiltinCallJumpMode::ForMksnapshot as u8, 3);
  assert!(BuiltinCallJumpMode::Indirect.is_indirect());
  assert!(!BuiltinCallJumpMode::Absolute.is_indirect());

  assert_eq!(VISIT_BYTECODE_METHODS.len(), 217);
  assert_eq!(VISIT_BYTECODE_METHODS[0], "VisitWide");
  assert_eq!(VISIT_BYTECODE_METHODS[216], "VisitIllegal");

  assert_eq!(VISIT_INTRINSIC_METHODS.len(), 17);
  assert_eq!(BYTECODE_OFFSET_TABLE_BUILDER_DECLARATIONS.len(), 4);
  assert_eq!(BASELINE_COMPILER_DECLARATIONS.len(), 76);
  assert_eq!(COMPARE_BRANCH_CANDIDATE_DECLARATIONS.len(), 2);
  assert_eq!(EFFECT_STATE_DECLARATIONS.len(), 6);
  assert_eq!(SAVE_ACCUMULATOR_SCOPE_DECLARATIONS.len(), 4);
  OK
}

#[test]
fn test_bytecode_offset_iterator() -> Void {
  assert_eq!(BYTECODE_OFFSET_ITERATOR_DECLARATIONS.len(), 25);
  assert!(
    BYTECODE_OFFSET_ITERATOR_DECLARATIONS
      .iter()
      .any(|d| d.contains("AdvanceToBytecodeOffset"))
  );
  assert!(
    BYTECODE_OFFSET_ITERATOR_DECLARATIONS
      .iter()
      .any(|d| d.contains("AdvanceToPCOffset"))
  );
  OK
}

#[test]
fn test_arch_specific_x64_and_arm64() -> Void {
  // x64
  assert_eq!(SCRATCH_REGISTERS.len(), 5);
  assert_eq!(SCRATCH_REGISTERS, ["r8", "r9", "r11", "r12", "r15"]);
  assert_eq!(NUM_SCRATCH_REGISTERS, 5);
  assert_eq!(X64_SCRATCH_REGISTER_SCOPE_DECLARATIONS.len(), 6);
  assert_eq!(
    X64_FALLBACK_BUILTIN_CALL_JUMP_MODE_FOR_BASELINE,
    BuiltinCallJumpMode::Indirect
  );
  assert_eq!(LOOP_UNROLL_SIZE, 8);

  // arm64
  assert_eq!(ARM64_SCRATCH_REGISTER_SCOPE_DECLARATIONS.len(), 6);
  assert_eq!(INSTRUCTIONS_PER_JUMP_TARGET, 1);
  assert_eq!(INSTRUCTIONS_PER_LABEL, 2);
  assert_eq!(HELPER_TEMPLATE_COUNT, 11);
  assert_eq!(
    ARM64_FALLBACK_BUILTIN_CALL_JUMP_MODE_FOR_BASELINE,
    BuiltinCallJumpMode::Indirect
  );
  OK
}

#[test]
fn test_error_display() -> Void {
  let err = Error::UnsupportedArchitecture("mips".into());
  assert_eq!(format!("{err}"), "Unsupported baseline architecture: mips");
  let err = Error::InvalidBytecodeOffset { offset: 12 };
  assert_eq!(format!("{err}"), "Invalid bytecode offset: 12");
  let err = Error::CompilationFailed("overflow".into());
  assert_eq!(format!("{err}"), "Compilation failed: overflow");
  let err = Error::Message("custom error".into());
  assert_eq!(format!("{err}"), "Baseline error: custom error");
  OK
}

#[test]
fn test_baseline_compiler_real_execution() -> Void {
  use v8_baseline::{
    BaselineCompiler, BytecodeOffsetIterator, BytecodeOffsetTableBuilder,
    can_compile_with_baseline, generate_baseline_code,
  };

  assert!(can_compile_with_baseline(true, true));
  assert!(!can_compile_with_baseline(false, true));
  assert!(!can_compile_with_baseline(true, false));

  // 构造真实字节码序列: LdaZero (0x0c), Nop (0x00), Return (0xab)
  let bytecode = [0x0c, 0x00, 0xab];

  let mut compiler = BaselineCompiler::new(&bytecode);
  compiler.mark_jump_target(2);
  assert!(compiler.is_jump_target(2));
  assert!(!compiler.is_jump_target(1));

  let label_id = compiler.new_label();
  assert_eq!(label_id, 0);

  let compiled = compiler
    .generate_code()
    .map_err(|e| Error::Message(format!("{e}")))?;
  assert!(!compiled.code_bytes.is_empty());
  assert!(!compiled.bytecode_offset_table.is_empty());
  assert!(compiled.estimated_size >= bytecode.len());

  // 验证直接通过 generate_baseline_code 函数生成
  let compiled2 = generate_baseline_code(&bytecode).map_err(|e| Error::Message(format!("{e}")))?;
  assert_eq!(compiled.code_bytes, compiled2.code_bytes);
  assert_eq!(
    compiled.bytecode_offset_table,
    compiled2.bytecode_offset_table
  );

  // 验证 BytecodeOffsetTableBuilder 与 BytecodeOffsetIterator 对接
  let mut builder = BytecodeOffsetTableBuilder::new();
  builder.add_position(4);
  builder.add_position(11);
  builder.add_position(12);
  assert!(!builder.is_empty());
  assert_eq!(builder.len(), 3);

  let mut iter = BytecodeOffsetIterator::new(builder.bytes());
  assert_eq!(iter.current_pc_start_offset(), 0);
  assert_eq!(iter.current_pc_end_offset(), 4);
  assert_eq!(iter.current_bytecode_offset(), 1);

  iter.advance();
  assert_eq!(iter.current_pc_start_offset(), 4);
  assert_eq!(iter.current_pc_end_offset(), 11);
  assert_eq!(iter.current_bytecode_offset(), 2);

  iter.advance();
  assert_eq!(iter.current_pc_start_offset(), 11);
  assert_eq!(iter.current_pc_end_offset(), 12);
  assert_eq!(iter.current_bytecode_offset(), 3);

  // 迭代至终点后验证 done 标志与防溢出
  iter.advance();
  assert!(iter.done());
  iter.advance();
  assert!(iter.done());

  // 验证 Iterator trait 链式消费
  let entries: Vec<_> = BytecodeOffsetIterator::new(builder.bytes()).collect();
  assert_eq!(entries.len(), 3);
  assert_eq!(entries[0].pc_start_offset, 0);
  assert_eq!(entries[0].pc_end_offset, 4);
  assert_eq!(entries[1].pc_start_offset, 4);
  assert_eq!(entries[1].pc_end_offset, 11);
  assert_eq!(entries[2].pc_start_offset, 11);
  assert_eq!(entries[2].pc_end_offset, 12);

  // 验证 advance_to_bytecode_offset 与 advance_to_pc_offset
  let mut target_iter = BytecodeOffsetIterator::new(builder.bytes());
  target_iter.advance_to_bytecode_offset(2);
  assert_eq!(target_iter.current_bytecode_offset(), 2);
  assert_eq!(target_iter.current_pc_end_offset(), 11);

  let mut pc_iter = BytecodeOffsetIterator::new(builder.bytes());
  pc_iter.advance_to_pc_offset(10);
  assert_eq!(pc_iter.current_pc_end_offset(), 11);

  // 边界保护: 空映射表
  let empty_iter = BytecodeOffsetIterator::new(&[]);
  assert!(empty_iter.done());

  // 边界保护: 动态扩展跳转标记位
  compiler.mark_jump_target(100);
  assert!(compiler.is_jump_target(100));

  OK
}
