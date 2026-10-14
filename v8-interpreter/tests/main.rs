use aok::{OK, Void};
use log::info;
use v8_common::LanguageMode;
use v8_interpreter::{
  Bytecode, BytecodeArrayIterator, BytecodeNode, BytecodeSourceInfo, CreateArrayLiteralFlags,
  CreateClosureFlags, CreateObjectLiteralFlags, INVALID_INDEX, INVALID_OFFSET, LookupHoistingMode,
  OperandScale, OperandSize, Register, RegisterList, StoreLookupSlotFlags, TestTypeOfLiteralFlag,
  bytecodes::Bytecodes, constant_pool_entry_for,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_bytecodes_properties_and_scales() -> Void {
  info!("Testing Bytecodes properties and scales");

  assert_eq!(Bytecode::BYTECODE_COUNT, 217);
  assert_eq!(Bytecode::SHORT_STAR_COUNT, 16);

  // 基础前缀
  assert_eq!(Bytecode::Wide as u8, 0);
  assert_eq!(Bytecode::ExtraWide as u8, 1);
  assert!(Bytecodes::is_prefix_scaling_bytecode(Bytecode::Wide));
  assert!(Bytecodes::is_prefix_scaling_bytecode(Bytecode::ExtraWide));
  assert!(!Bytecodes::is_prefix_scaling_bytecode(
    Bytecode::LdaUndefined
  ));

  // 累加器读写属性
  assert!(Bytecodes::writes_accumulator(Bytecode::LdaUndefined));
  assert!(!Bytecodes::reads_accumulator(Bytecode::LdaUndefined));
  assert!(Bytecodes::reads_accumulator(Bytecode::Return));
  assert!(!Bytecodes::writes_accumulator(Bytecode::Return));

  // 尺寸计算：LdaUndefined 无操作数，尺寸恒为 1
  assert_eq!(Bytecode::LdaUndefined.size(OperandScale::Single), 1);
  assert_eq!(Bytecode::LdaUndefined.size(OperandScale::Double), 1);
  assert_eq!(Bytecode::LdaUndefined.size(OperandScale::Quadruple), 1);

  // LdaSmi 有 1 个 Imm 操作数：Single 为 1+1=2，Double 为 1+2=3，Quadruple 为 1+4=5
  assert_eq!(Bytecode::LdaSmi.size(OperandScale::Single), 2);
  assert_eq!(Bytecode::LdaSmi.size(OperandScale::Double), 3);
  assert_eq!(Bytecode::LdaSmi.size(OperandScale::Quadruple), 5);

  OK
}

#[test]
fn test_short_star_conversions() -> Void {
  info!("Testing ShortStar conversions");

  let star0 = Register::from_short_star(Bytecode::Star0);
  assert_eq!(star0.index(), 0);
  assert_eq!(star0.try_to_short_star(), Some(Bytecode::Star0));

  let star1 = Register::from_short_star(Bytecode::Star1);
  assert_eq!(star1.index(), 1);
  assert_eq!(star1.try_to_short_star(), Some(Bytecode::Star1));

  let star15 = Register::from_short_star(Bytecode::Star15);
  assert_eq!(star15.index(), 15);
  assert_eq!(star15.try_to_short_star(), Some(Bytecode::Star15));

  let reg16 = Register::new(16);
  assert_eq!(reg16.try_to_short_star(), None);

  OK
}

#[test]
fn test_interpreter_registers() -> Void {
  info!("Testing Interpreter Registers");

  let r0 = Register::new(0);
  assert_eq!(r0.index(), 0);
  assert!(r0.is_valid());
  assert!(!r0.is_parameter());

  let inv = Register::invalid_value();
  assert!(!inv.is_valid());

  // 参数: 0 为 receiver (<this>)
  let recv = Register::receiver();
  assert!(recv.is_parameter());
  assert!(recv.is_receiver());
  assert_eq!(recv.to_parameter_index(), 0);
  assert_eq!(recv.index(), -1);

  // a0
  let a0 = Register::from_parameter_index(1);
  assert!(a0.is_parameter());
  assert!(!a0.is_receiver());
  assert_eq!(a0.to_parameter_index(), 1);
  assert_eq!(a0.index(), -2);

  // 操作数编解码互逆性
  for idx in [-10, -5, -2, -1, 0, 1, 10, 100] {
    let reg = Register::new(idx);
    let op = reg.to_operand();
    assert_eq!(Register::from_operand(op), reg);
  }

  // 尺寸选择
  assert_eq!(r0.size_of_operand(), OperandSize::Byte);

  OK
}

#[test]
fn test_register_list() -> Void {
  info!("Testing RegisterList");

  let empty = RegisterList::new();
  assert!(empty.is_empty());
  assert_eq!(empty.register_count(), 0);

  let single = RegisterList::single(Register::new(5));
  assert_eq!(single.register_count(), 1);
  assert_eq!(single.first_register().index(), 5);
  assert_eq!(single.last_register().index(), 5);

  let list = RegisterList::from_range(Register::new(2), 4);
  assert_eq!(list.register_count(), 4);
  assert_eq!(list.first_register().index(), 2);
  assert_eq!(list.last_register().index(), 5);

  let truncated = list.truncate(2)?;
  assert_eq!(truncated.register_count(), 2);
  assert_eq!(truncated.last_register().index(), 3);

  let popped = list.pop_left()?;
  assert_eq!(popped.register_count(), 3);
  assert_eq!(popped.first_register().index(), 3);

  OK
}

#[test]
fn test_bytecode_nodes() -> Void {
  info!("Testing BytecodeNode");

  let node_undef = BytecodeNode::new(Bytecode::LdaUndefined, BytecodeSourceInfo::none())?;
  assert_eq!(node_undef.bytecode(), Bytecode::LdaUndefined);
  assert_eq!(node_undef.operand_count(), 0);
  assert_eq!(node_undef.operand_scale(), OperandScale::Single);

  // LdaSmi 单操作数 (立即数 10 -> Single scale)
  let node_smi = BytecodeNode::with_operands(Bytecode::LdaSmi, &[10], BytecodeSourceInfo::none())?;
  assert_eq!(node_smi.bytecode(), Bytecode::LdaSmi);
  assert_eq!(node_smi.operand_count(), 1);
  assert_eq!(node_smi.operand(0)?, 10);
  assert_eq!(node_smi.operand_scale(), OperandScale::Single);

  // LdaSmi 单操作数 (立即数 1000 -> Double scale)
  let node_smi_wide =
    BytecodeNode::with_operands(Bytecode::LdaSmi, &[1000], BytecodeSourceInfo::none())?;
  assert_eq!(node_smi_wide.operand_scale(), OperandScale::Double);

  OK
}

#[test]
fn test_bytecode_array_iterator_simple() -> Void {
  info!("Testing BytecodeArrayIterator simple stream");

  // 指令流:
  // 1. LdaUndefined (0x2c)
  // 2. Star0 (0xd7)
  // 3. Return (0xa8)
  let bytecodes = [
    Bytecode::LdaUndefined as u8,
    Bytecode::Star0 as u8,
    Bytecode::Return as u8,
  ];

  let mut it = BytecodeArrayIterator::new(&bytecodes);
  assert!(!it.done());
  assert_eq!(it.current_bytecode(), Bytecode::LdaUndefined);
  assert_eq!(it.current_offset(), 0);
  assert_eq!(it.current_operand_scale(), OperandScale::Single);

  it.advance();
  assert!(!it.done());
  assert_eq!(it.current_bytecode(), Bytecode::Star0);
  assert_eq!(it.current_offset(), 1);

  it.advance();
  assert!(!it.done());
  assert_eq!(it.current_bytecode(), Bytecode::Return);
  assert_eq!(it.current_offset(), 2);

  it.advance();
  assert!(it.done());

  OK
}

#[test]
fn test_bytecode_array_iterator_scaled_and_operands() -> Void {
  info!("Testing BytecodeArrayIterator Wide prefix and operands");

  // 构造序列:
  // 1. LdaSmi 42 -> [LdaSmi, 42]
  // 2. Wide LdaSmi 0x1234 -> [Wide, LdaSmi, 0x34, 0x12]
  let bytecodes = [
    Bytecode::LdaSmi as u8,
    42,
    Bytecode::Wide as u8,
    Bytecode::LdaSmi as u8,
    0x34,
    0x12,
  ];

  let mut it = BytecodeArrayIterator::new(&bytecodes);
  assert_eq!(it.current_bytecode(), Bytecode::LdaSmi);
  assert_eq!(it.current_offset(), 0);
  assert_eq!(it.get_immediate_operand(0)?, 42);

  it.advance();
  assert_eq!(it.current_bytecode(), Bytecode::LdaSmi);
  assert_eq!(it.current_offset(), 2);
  assert_eq!(it.current_operand_scale(), OperandScale::Double);
  assert_eq!(it.current_bytecode_size(), 4); // 1 prefix + 1 opcode + 2 imm = 4
  assert_eq!(it.get_immediate_operand(0)?, 0x1234);

  it.advance();
  assert!(it.done());

  OK
}

#[test]
fn test_bytecode_flags_and_tokens() -> Void {
  info!("Testing Bytecode Flags and Tokens");

  // CreateArrayLiteralFlags
  let arr_flags = CreateArrayLiteralFlags::encode(true, 7);
  assert_eq!(arr_flags & CreateArrayLiteralFlags::FLAGS_MASK, 7);
  assert_ne!(
    arr_flags & CreateArrayLiteralFlags::FAST_CLONE_SUPPORTED_BIT,
    0
  );

  // CreateObjectLiteralFlags
  let obj_flags = CreateObjectLiteralFlags::encode(3, true);
  assert_eq!(obj_flags & CreateObjectLiteralFlags::FLAGS_MASK, 3);
  assert_ne!(
    obj_flags & CreateObjectLiteralFlags::FAST_CLONE_SUPPORTED_BIT,
    0
  );

  // CreateClosureFlags: 仅在 !pretenure && is_function_scope 时置 fast 位
  let closure_flags_pretenure = CreateClosureFlags::encode(true, true);
  assert_ne!(
    closure_flags_pretenure & CreateClosureFlags::PRETENURED_BIT,
    0
  );
  assert_eq!(
    closure_flags_pretenure & CreateClosureFlags::FAST_NEW_CLOSURE_BIT,
    0
  );

  let closure_flags_fast = CreateClosureFlags::encode(false, true);
  assert_eq!(closure_flags_fast & CreateClosureFlags::PRETENURED_BIT, 0);
  assert_ne!(
    closure_flags_fast & CreateClosureFlags::FAST_NEW_CLOSURE_BIT,
    0
  );

  // TestTypeOfLiteralFlag
  let flag = TestTypeOfLiteralFlag::Number;
  assert_eq!(TestTypeOfLiteralFlag::decode(flag.encode())?, flag);
  assert_eq!(flag.as_str(), "number");

  // StoreLookupSlotFlags
  let store_flags =
    StoreLookupSlotFlags::encode(LanguageMode::Strict, LookupHoistingMode::LegacySloppy);
  assert_eq!(
    StoreLookupSlotFlags::get_language_mode(store_flags),
    LanguageMode::Strict
  );
  assert!(StoreLookupSlotFlags::is_lookup_hoisting_mode(store_flags));

  OK
}

#[test]
fn test_bytecode_array_iterator_jumps() -> Void {
  info!("Testing BytecodeArrayIterator jumps and targets");

  // 1. 普通非前缀跳转: Jump 10 (长度 2: opcode + UImm 10)
  let jump_bytes = [Bytecode::Jump as u8, 10];
  let it = BytecodeArrayIterator::new(&jump_bytes);
  assert_eq!(it.current_bytecode(), Bytecode::Jump);
  assert_eq!(it.current_offset(), 0);
  assert_eq!(it.get_relative_jump_target_offset()?, 10);
  assert_eq!(it.get_absolute_offset(10)?, 10);
  assert_eq!(it.get_jump_target_offset()?, 10);

  // 2. Wide 前缀跳转: Wide Jump 0x0100 (256)
  // 指令流: [Wide, Jump, 0x00, 0x01]
  // prefix_size = 1, current_offset = 0
  // relative_offset = 256
  // target = current_offset + relative_offset + prefix_size = 0 + 256 + 1 = 257
  let wide_jump_bytes = [Bytecode::Wide as u8, Bytecode::Jump as u8, 0x00, 0x01];
  let wide_it = BytecodeArrayIterator::new(&wide_jump_bytes);
  assert_eq!(wide_it.current_bytecode(), Bytecode::Jump);
  assert_eq!(wide_it.current_offset(), 0);
  assert_eq!(wide_it.current_operand_scale(), OperandScale::Double);
  assert_eq!(wide_it.get_relative_jump_target_offset()?, 256);
  assert_eq!(wide_it.get_absolute_offset(256)?, 257);
  assert_eq!(wide_it.get_jump_target_offset()?, 257);

  // 3. JumpLoop 回跳:
  // 放在 offset 10 之后: 前置 10 个 LdaUndefined, 再接 JumpLoop 4
  let mut loop_stream = vec![Bytecode::LdaUndefined as u8; 10];
  loop_stream.push(Bytecode::JumpLoop as u8);
  loop_stream.push(4); // 回跳 4
  let mut loop_it = BytecodeArrayIterator::new(&loop_stream);
  for _ in 0..10 {
    loop_it.advance();
  }
  assert_eq!(loop_it.current_bytecode(), Bytecode::JumpLoop);
  assert_eq!(loop_it.current_offset(), 10);
  assert_eq!(loop_it.get_relative_jump_target_offset()?, -4);
  assert_eq!(loop_it.get_jump_target_offset()?, 6);

  OK
}

#[test]
fn test_bytecode_source_info() -> Void {
  info!("Testing BytecodeSourceInfo states");

  let mut none_info = BytecodeSourceInfo::none();
  assert!(!none_info.is_valid());
  assert_eq!(none_info.source_position(), -1);
  assert!(none_info.is_breakable());
  assert!(!none_info.is_statement());
  assert!(!none_info.is_expression());

  let stmt_info = BytecodeSourceInfo::statement(42, false);
  assert!(stmt_info.is_valid());
  assert_eq!(stmt_info.source_position(), 42);
  assert!(!stmt_info.is_breakable());
  assert!(stmt_info.is_statement());
  assert!(!stmt_info.is_expression());

  let expr_info = BytecodeSourceInfo::expression(88);
  assert!(expr_info.is_valid());
  assert_eq!(expr_info.source_position(), 88);
  assert!(expr_info.is_breakable());
  assert!(!expr_info.is_statement());
  assert!(expr_info.is_expression());

  none_info = stmt_info;
  assert!(none_info.is_valid());
  none_info.set_invalid();
  assert!(!none_info.is_valid());

  OK
}

#[test]
fn test_bytecode_jump_table() -> Void {
  info!("Testing BytecodeJumpTable formulas and sentinels");

  assert_eq!(INVALID_INDEX, usize::MAX);
  assert_eq!(INVALID_OFFSET, usize::MAX);
  assert_eq!(constant_pool_entry_for(100, 3, 5), 102);
  assert_eq!(constant_pool_entry_for(200, 0, 10), 210);

  OK
}

#[test]
fn test_bytecode_node_scaling_and_errors() -> Void {
  info!("Testing BytecodeNode scaling updates and error handling");

  // 节点操作数与尺度更新
  let mut node = BytecodeNode::with_operands(Bytecode::LdaSmi, &[10], BytecodeSourceInfo::none())?;
  assert_eq!(node.operand_scale(), OperandScale::Single);

  node.update_operand0(1000)?;
  assert_eq!(node.operand(0)?, 1000);
  assert_eq!(node.operand_scale(), OperandScale::Double);

  node.update_operand0(100_000)?;
  assert_eq!(node.operand(0)?, 100_000);
  assert_eq!(node.operand_scale(), OperandScale::Quadruple);

  // 边界错误排查
  let err_operands = BytecodeNode::new(Bytecode::LdaSmi, BytecodeSourceInfo::none());
  assert!(err_operands.is_err());

  let err_idx = node.operand(5);
  assert!(err_idx.is_err());

  let err_set = node.set_operand(5, 0);
  assert!(err_set.is_err());

  OK
}

#[test]
fn test_bytecode_source_info_operations_v8() -> Void {
  info!("Testing BytecodeSourceInfo operations (V8 1:1 replication)");
  let mut x = BytecodeSourceInfo::statement(0, true);
  assert_eq!(x.source_position(), 0);
  assert!(x.is_statement());
  assert!(x.is_valid());
  x.set_invalid();
  assert!(!x.is_statement());
  assert!(!x.is_valid());

  x.make_statement_position(1, true);
  let mut y = BytecodeSourceInfo::statement(1, true);
  assert_eq!(x, y);

  x.set_invalid();
  assert_ne!(x, y);

  y.make_statement_position(1, true);
  assert_eq!(y.source_position(), 1);
  assert!(y.is_statement());

  y.make_statement_position(2, true);
  assert_eq!(y.source_position(), 2);
  assert!(y.is_statement());

  y.set_invalid();
  y.make_expression_position(3);
  assert_eq!(y.source_position(), 3);
  assert!(!y.is_statement());
  assert!(y.is_expression());

  y.make_statement_position(3, true);
  assert_eq!(y.source_position(), 3);
  assert!(y.is_statement());

  y.force_expression_position(5);
  assert_eq!(y.source_position(), 5);
  assert!(y.is_expression());

  // 原版 V8 语义: is_breakable 不参与 operator==
  let s_break = BytecodeSourceInfo::statement(10, true);
  let s_nobreak = BytecodeSourceInfo::statement(10, false);
  assert_eq!(s_break, s_nobreak);

  OK
}

#[test]
fn test_register_list_indexing() -> Void {
  info!("Testing RegisterList get and register_at");
  let list = RegisterList::from_range(Register::new(10), 3);
  assert_eq!(list.get(0), Some(Register::new(10)));
  assert_eq!(list.get(1), Some(Register::new(11)));
  assert_eq!(list.get(2), Some(Register::new(12)));
  assert_eq!(list.get(3), None);

  assert_eq!(list.register_at(0)?, Register::new(10));
  assert_eq!(list.register_at(1)?, Register::new(11));
  assert_eq!(list.register_at(2)?, Register::new(12));
  assert!(list.register_at(3).is_err());

  OK
}

#[test]
fn test_bytecode_array_iterator_advanced_features() -> Void {
  info!("Testing BytecodeArrayIterator advanced features and V8 test patterns");
  let bytes = [
    Bytecode::LdaUndefined as u8,
    Bytecode::Star0 as u8,
    Bytecode::Return as u8,
  ];

  let mut it = BytecodeArrayIterator::new(&bytes);
  assert_eq!(it.current_bytecode(), Bytecode::LdaUndefined);
  assert_eq!(it.current_offset(), 0);
  assert_eq!(it.next_offset(), 1);
  assert_eq!(it.next_bytecode(), Bytecode::Star0);

  it.advance();
  assert_eq!(it.current_bytecode(), Bytecode::Star0);
  assert_eq!(it.current_offset(), 1);
  assert_eq!(it.get_star_target_register()?, Register::new(0));
  assert_eq!(it.next_offset(), 2);
  assert_eq!(it.next_bytecode(), Bytecode::Return);

  // 测试 AdvanceTo, SetOffset, Reset
  let stream = [
    Bytecode::LdaUndefined as u8, // offset 0 (len 1)
    Bytecode::LdaZero as u8,      // offset 1 (len 1)
    Bytecode::Star0 as u8,        // offset 2 (len 1)
    Bytecode::Star1 as u8,        // offset 3 (len 1)
    Bytecode::Return as u8,       // offset 4 (len 1)
  ];
  let mut nav_it = BytecodeArrayIterator::new(&stream);
  nav_it.advance_to(3)?;
  assert_eq!(nav_it.current_offset(), 3);
  assert_eq!(nav_it.current_bytecode(), Bytecode::Star1);

  nav_it.set_offset(1)?;
  assert_eq!(nav_it.current_offset(), 1);
  assert_eq!(nav_it.current_bytecode(), Bytecode::LdaZero);

  nav_it.reset();
  assert_eq!(nav_it.current_offset(), 0);
  assert_eq!(nav_it.current_bytecode(), Bytecode::LdaUndefined);

  OK
}

#[test]
fn test_wide_and_extrawide_jumploop_accuracy() -> Void {
  info!("Testing Wide & ExtraWide JumpLoop absolute offset computation");
  // 1. Wide JumpLoop:
  // 前置 20 个 LdaUndefined (offset 0..20)
  // loop_header 位于 offset 0.
  // current_offset = 20.
  // V8 delta = 20 - 0 = 20; 带 Wide 前缀时 delta += 1 => 21.
  let mut wide_stream = vec![Bytecode::LdaUndefined as u8; 20];
  wide_stream.push(Bytecode::Wide as u8);
  wide_stream.push(Bytecode::JumpLoop as u8);
  wide_stream.push(21);
  wide_stream.push(0);

  let mut wide_it = BytecodeArrayIterator::new(&wide_stream);
  wide_it.advance_to(20)?;
  assert_eq!(wide_it.current_bytecode(), Bytecode::JumpLoop);
  assert_eq!(wide_it.current_offset(), 20);
  assert_eq!(wide_it.current_operand_scale(), OperandScale::Double);
  assert_eq!(wide_it.get_relative_jump_target_offset()?, -21);
  assert_eq!(wide_it.get_jump_target_offset()?, 0);

  // 2. ExtraWide JumpLoop:
  // 前置 300 个 LdaUndefined (offset 0..300)
  // loop_header 位于 offset 0.
  // current_offset = 300.
  // V8 delta = 300 - 0 = 300; 带 ExtraWide 前缀时 delta += 1 => 301.
  let mut extra_stream = vec![Bytecode::LdaUndefined as u8; 300];
  extra_stream.push(Bytecode::ExtraWide as u8);
  extra_stream.push(Bytecode::JumpLoop as u8);
  extra_stream.extend_from_slice(&301u32.to_le_bytes());

  let mut extra_it = BytecodeArrayIterator::new(&extra_stream);
  extra_it.advance_to(300)?;
  assert_eq!(extra_it.current_bytecode(), Bytecode::JumpLoop);
  assert_eq!(extra_it.current_offset(), 300);
  assert_eq!(extra_it.current_operand_scale(), OperandScale::Quadruple);
  assert_eq!(extra_it.get_relative_jump_target_offset()?, -301);
  assert_eq!(extra_it.get_jump_target_offset()?, 0);

  OK
}

#[test]
fn test_v8_sample_bytecodes_accumulator_semantics() -> Void {
  info!("Testing V8 sample bytecodes accumulator semantics (1:1 with bytecodes-unittest.cc)");
  use v8_interpreter::ImplicitRegisterUse;

  assert!(Bytecodes::reads_accumulator(Bytecode::Star));
  assert!(!Bytecodes::writes_accumulator(Bytecode::Star));
  assert_eq!(
    Bytecode::Star.implicit_register_use(),
    ImplicitRegisterUse::READ_ACCUMULATOR
  );

  assert!(!Bytecodes::reads_accumulator(Bytecode::Ldar));
  assert!(Bytecodes::writes_accumulator(Bytecode::Ldar));
  assert_eq!(
    Bytecode::Ldar.implicit_register_use(),
    ImplicitRegisterUse::WRITE_ACCUMULATOR
  );

  assert!(Bytecodes::reads_accumulator(Bytecode::Add));
  assert!(Bytecodes::writes_accumulator(Bytecode::Add));
  assert_eq!(
    Bytecode::Add.implicit_register_use(),
    ImplicitRegisterUse::READ_WRITE_ACCUMULATOR
  );

  OK
}

#[test]
fn test_v8_registers_parameters_no_overlap() -> Void {
  info!(
    "Testing Registers and Parameters operand space non-overlap (1:1 with bytecodes-unittest.cc)"
  );
  let register_count = 128;
  let parameter_count = 100;
  let mut seen_operands = [false; 256];

  for i in 0..register_count {
    let r = Register::new(i);
    let op = r.to_operand();
    let idx = (op as u8) as usize;
    assert!(
      !seen_operands[idx],
      "Duplicate operand index for register {i}"
    );
    seen_operands[idx] = true;
    let restored = Register::from_operand(op);
    assert_eq!(restored.index(), i);
  }

  for i in 0..parameter_count {
    let r = Register::from_parameter_index(i);
    let op = r.to_operand();
    let idx = (op as u8) as usize;
    assert!(
      !seen_operands[idx],
      "Duplicate operand index for parameter {i}"
    );
    seen_operands[idx] = true;
    let restored = Register::from_operand(op);
    assert_eq!(restored.to_parameter_index(), i);
  }

  OK
}

#[test]
fn test_v8_bytecode_array_iterator_operands_and_params() -> Void {
  info!(
    "Testing BytecodeArrayIterator operand decoding and parameter helper (1:1 with bytecode-array-iterator-unittest.cc)"
  );

  let param = Register::from_parameter_index(2);
  let reg0 = Register::new(0);

  let stream = [
    Bytecode::CallRuntimeForPair as u8,
    0x23,
    0x01,                       // RuntimeId (16-bit)
    (param.to_operand() as u8), // Reg
    1,                          // RegCount
    (reg0.to_operand() as u8),  // RegOutPair
  ];

  let it = BytecodeArrayIterator::new(&stream);
  assert_eq!(it.current_bytecode(), Bytecode::CallRuntimeForPair);
  assert_eq!(it.get_runtime_id_operand(0)?, 0x0123);
  assert_eq!(it.get_register_operand(1)?, param);
  assert_eq!(it.get_register_operand_range(1)?, 1);
  assert_eq!(it.get_unsigned_operand(2)?, 1);
  assert_eq!(it.get_register_operand(3)?, reg0);
  assert_eq!(it.get_register_operand_range(3)?, 2);

  let pair = it.get_register_pair_operand(3)?;
  assert_eq!(pair.0, reg0);
  assert_eq!(pair.1, Register::new(1));

  let it_param0 = it.get_parameter(0);
  assert_eq!(it_param0, Register::from_parameter_index(1));
  let it_param1 = it.get_parameter(1);
  assert_eq!(it_param1, Register::from_parameter_index(2));

  OK
}

#[test]
fn test_v8_bytecodes_scales_and_debug_breaks() -> Void {
  info!("Testing Bytecodes scaling rules and debug break parity (1:1 with bytecodes-unittest.cc)");

  assert_eq!(
    Bytecodes::operand_scale_to_prefix_bytecode(OperandScale::Double),
    Some(Bytecode::Wide)
  );
  assert_eq!(
    Bytecodes::operand_scale_to_prefix_bytecode(OperandScale::Quadruple),
    Some(Bytecode::ExtraWide)
  );
  assert_eq!(
    Bytecodes::prefix_bytecode_to_operand_scale(Bytecode::Wide),
    Some(OperandScale::Double)
  );
  assert_eq!(
    Bytecodes::prefix_bytecode_to_operand_scale(Bytecode::ExtraWide),
    Some(OperandScale::Quadruple)
  );

  assert_eq!(Bytecodes::scale_for_signed_operand(0), OperandScale::Single);
  assert_eq!(
    Bytecodes::scale_for_signed_operand(127),
    OperandScale::Single
  );
  assert_eq!(
    Bytecodes::scale_for_signed_operand(-128),
    OperandScale::Single
  );
  assert_eq!(
    Bytecodes::scale_for_signed_operand(128),
    OperandScale::Double
  );
  assert_eq!(
    Bytecodes::scale_for_signed_operand(-129),
    OperandScale::Double
  );
  assert_eq!(
    Bytecodes::scale_for_signed_operand(32767),
    OperandScale::Double
  );
  assert_eq!(
    Bytecodes::scale_for_signed_operand(-32768),
    OperandScale::Double
  );
  assert_eq!(
    Bytecodes::scale_for_signed_operand(32768),
    OperandScale::Quadruple
  );
  assert_eq!(
    Bytecodes::scale_for_signed_operand(-32769),
    OperandScale::Quadruple
  );

  assert_eq!(
    Bytecodes::scale_for_unsigned_operand(0),
    OperandScale::Single
  );
  assert_eq!(
    Bytecodes::scale_for_unsigned_operand(255),
    OperandScale::Single
  );
  assert_eq!(
    Bytecodes::scale_for_unsigned_operand(256),
    OperandScale::Double
  );
  assert_eq!(
    Bytecodes::scale_for_unsigned_operand(65535),
    OperandScale::Double
  );
  assert_eq!(
    Bytecodes::scale_for_unsigned_operand(65536),
    OperandScale::Quadruple
  );

  for b in 0..Bytecode::BYTECODE_COUNT {
    if let Some(bc) = Bytecode::from_repr(b as u8)
      && !Bytecodes::is_debug_break(bc)
      && !Bytecodes::is_prefix_scaling_bytecode(bc)
    {
      let dbg = Bytecodes::get_debug_break(bc);
      assert_eq!(
        bc.size(OperandScale::Single),
        dbg.size(OperandScale::Single),
        "Size mismatch between {bc:?} and {dbg:?}"
      );
    }
  }

  OK
}

#[test]
fn test_v8_bytecode_node_equality() -> Void {
  info!("Testing BytecodeNode equality (1:1 with bytecode-node-unittest.cc)");

  let s1 = BytecodeSourceInfo::statement(3, true);
  let s2 = BytecodeSourceInfo::statement(3, false);
  let s_diff = BytecodeSourceInfo::statement(77, true);

  let n1 = BytecodeNode::with_operands(Bytecode::ForInNext, &[0x71, 0xA5, 0x5A, 0xFC], s1)?;
  let n2 = BytecodeNode::with_operands(Bytecode::ForInNext, &[0x71, 0xA5, 0x5A, 0xFC], s2)?;
  let n_other =
    BytecodeNode::with_operands(Bytecode::ForInNext, &[0x71, 0xA5, 0x5A, 0xFC], s_diff)?;
  let n_no_info = BytecodeNode::with_operands(
    Bytecode::ForInNext,
    &[0x71, 0xA5, 0x5A, 0xFC],
    BytecodeSourceInfo::none(),
  )?;

  assert_eq!(n1, n1);
  assert_eq!(n1, n2);
  assert_ne!(n1, n_other);
  assert_ne!(n1, n_no_info);

  OK
}

#[test]
fn test_v8_bytecode_operands_scalability() -> Void {
  info!("Testing BytecodeOperands scalability (1:1 with bytecode-operands-unittest.cc)");
  use v8_interpreter::{BytecodeOperands, OperandType};

  // Scalable signed byte (Imm + all register operands)
  for t in [
    OperandType::Imm,
    OperandType::Reg,
    OperandType::RegList,
    OperandType::RegPair,
    OperandType::RegOut,
    OperandType::RegOutList,
    OperandType::RegOutPair,
    OperandType::RegOutTriple,
    OperandType::RegInOut,
  ] {
    assert!(BytecodeOperands::is_scalable_signed_byte(t));
    assert!(!BytecodeOperands::is_scalable_unsigned_byte(t));
  }

  // Scalable unsigned byte
  for t in [
    OperandType::ConstantPoolIndex,
    OperandType::FeedbackSlot,
    OperandType::ContextSlot,
    OperandType::CoverageSlot,
    OperandType::UImm,
    OperandType::RegCount,
  ] {
    assert!(BytecodeOperands::is_scalable_unsigned_byte(t));
    assert!(!BytecodeOperands::is_scalable_signed_byte(t));
  }

  // Non-scalable
  for t in [
    OperandType::None,
    OperandType::Flag8,
    OperandType::Flag16,
    OperandType::IntrinsicId,
    OperandType::RuntimeId,
    OperandType::NativeContextIndex,
    OperandType::AbortReason,
    OperandType::EmbeddedFeedback,
  ] {
    assert!(!BytecodeOperands::is_scalable_signed_byte(t));
    assert!(!BytecodeOperands::is_scalable_unsigned_byte(t));
  }

  OK
}
