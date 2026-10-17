//! MachineOperatorBuilder 单元测试

use aok::{OK, Void};
use v8_compiler_graph::{IrOpcode, MachineRepresentation, MachineType, Properties};
use v8_compiler_ops::{MachineOperatorBuilder, StoreRepresentation, WriteBarrierKind};
use v8_zone::{AccountingAllocator, Zone};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_machine_pure_operators() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let machine = MachineOperatorBuilder::new(&zone);

  // Word32And
  let w32_and = machine.word32_and();
  assert_eq!(w32_and.opcode(), IrOpcode::Word32And as u16);
  assert!(w32_and.properties().contains(Properties::PURE));
  assert!(w32_and.properties().contains(Properties::COMMUTATIVE));
  assert_eq!(w32_and.value_input_count(), 2);
  assert_eq!(w32_and.value_output_count(), 1);

  // Word64And
  let w64_and = machine.word64_and();
  assert_eq!(w64_and.opcode(), IrOpcode::Word64And as u16);
  assert!(w64_and.properties().contains(Properties::PURE));
  assert_eq!(w64_and.value_input_count(), 2);

  // Int32Add & Int32Sub
  let i32_add = machine.int32_add();
  assert_eq!(i32_add.opcode(), IrOpcode::Int32Add as u16);
  assert!(i32_add.properties().contains(Properties::COMMUTATIVE));

  let i32_sub = machine.int32_sub();
  assert_eq!(i32_sub.opcode(), IrOpcode::Int32Sub as u16);
  assert!(!i32_sub.properties().contains(Properties::COMMUTATIVE));

  // Float64Add
  let f64_add = machine.float64_add();
  assert_eq!(f64_add.opcode(), IrOpcode::Float64Add as u16);
  assert!(f64_add.properties().contains(Properties::COMMUTATIVE));
  assert_eq!(f64_add.value_input_count(), 2);

  // Float64Equal & Float64LessThan
  let f64_eq = machine.float64_equal();
  assert_eq!(f64_eq.opcode(), IrOpcode::Float64Equal as u16);
  assert!(f64_eq.properties().contains(Properties::COMMUTATIVE));

  let f64_lt = machine.float64_less_than();
  assert_eq!(f64_lt.opcode(), IrOpcode::Float64LessThan as u16);
  assert!(!f64_lt.properties().contains(Properties::COMMUTATIVE));

  OK
}

#[test]
fn test_machine_pair_binary_operators() -> Void {
  use gxhash::HashSet;

  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let machine = MachineOperatorBuilder::new(&zone);

  // 表项与 machine-operator.cc:879-884 (MACHINE_PURE_OP_LIST 尾部) 逐字段对齐:
  // V(Name, kNoProperties(→kPure), value_in, control_in, value_out)
  let table: [(&str, u16, u32); 6] = [
    ("Int32PairAdd", IrOpcode::Int32PairAdd as u16, 4),
    ("Int32PairSub", IrOpcode::Int32PairSub as u16, 4),
    ("Int32PairMul", IrOpcode::Int32PairMul as u16, 4),
    ("Word32PairShl", IrOpcode::Word32PairShl as u16, 3),
    ("Word32PairShr", IrOpcode::Word32PairShr as u16, 3),
    ("Word32PairSar", IrOpcode::Word32PairSar as u16, 3),
  ];
  let getters = [
    machine.int32_pair_add(),
    machine.int32_pair_sub(),
    machine.int32_pair_mul(),
    machine.word32_pair_shl(),
    machine.word32_pair_shr(),
    machine.word32_pair_sar(),
  ];
  assert_eq!(table.len(), getters.len());

  let mut opcodes = HashSet::<u16>::default();
  for ((name, opcode, value_in), op) in table.iter().zip(getters) {
    assert_eq!(op.mnemonic(), *name);
    assert_eq!(op.opcode(), *opcode);
    assert!(op.properties().contains(Properties::PURE));
    // kNoProperties: 除 kPure 外无任何附加属性 (非交换、非结合、无控制输入)
    assert!(!op.properties().contains(Properties::COMMUTATIVE));
    assert!(!op.properties().contains(Properties::ASSOCIATIVE));
    assert_eq!(op.value_input_count(), *value_in);
    assert_eq!(op.control_input_count(), 0);
    assert_eq!(op.value_output_count(), 2); // 双值输出 = pair 语义
    assert!(opcodes.insert(*opcode), "Pair 表项 opcode 重复: {name}");
  }
  assert_eq!(opcodes.len(), 6);

  // 枚举互斥: Pair opcode 与对应单值 opcode 是不同变体 (Word32PairShl 等与 ShiftKind 版无关联参数)
  for (pair, single) in [
    (IrOpcode::Int32PairAdd, IrOpcode::Int32Add),
    (IrOpcode::Int32PairSub, IrOpcode::Int32Sub),
    (IrOpcode::Int32PairMul, IrOpcode::Int32Mul),
    (IrOpcode::Word32PairShl, IrOpcode::Word32Shl),
    (IrOpcode::Word32PairShr, IrOpcode::Word32Shr),
    (IrOpcode::Word32PairSar, IrOpcode::Word32Sar),
  ] {
    assert_ne!(pair as u16, single as u16);
  }

  OK
}

#[test]
fn test_machine_load_store() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let machine = MachineOperatorBuilder::new(&zone);

  // Load (LoadRepresentation = MachineType)
  let load_i32 = machine.load(MachineType::int32())?;
  assert_eq!(load_i32.opcode(), IrOpcode::Load as u16);
  assert!(load_i32.properties().contains(Properties::ELIMINATABLE));
  assert_eq!(load_i32.value_input_count(), 2); // base, index
  assert_eq!(load_i32.effect_input_count(), 1);
  assert_eq!(load_i32.control_input_count(), 1);
  assert_eq!(load_i32.value_output_count(), 1);
  assert_eq!(load_i32.effect_output_count(), 1);
  assert_eq!(load_i32.control_output_count(), 0);

  // Store (StoreRepresentation)
  let store_rep = StoreRepresentation::new(
    MachineRepresentation::Word32,
    WriteBarrierKind::NoWriteBarrier,
  );
  assert_eq!(store_rep.representation(), MachineRepresentation::Word32);
  assert_eq!(
    store_rep.write_barrier_kind(),
    WriteBarrierKind::NoWriteBarrier
  );

  let store = machine.store(store_rep)?;
  assert_eq!(store.opcode(), IrOpcode::Store as u16);
  assert!(store.properties().contains(Properties::NO_DEOPT));
  assert!(store.properties().contains(Properties::NO_READ));
  assert!(store.properties().contains(Properties::NO_THROW));
  assert_eq!(store.value_input_count(), 3); // base, index, value
  assert_eq!(store.effect_input_count(), 1);
  assert_eq!(store.control_input_count(), 1);
  assert_eq!(store.value_output_count(), 0);
  assert_eq!(store.effect_output_count(), 1);
  assert_eq!(store.control_output_count(), 0);

  // Unaligned Load / Store
  let unaligned_load = machine.unaligned_load(MachineType::float64())?;
  assert_eq!(unaligned_load.opcode(), IrOpcode::UnalignedLoad as u16);
  assert_eq!(unaligned_load.value_input_count(), 2);

  let unaligned_store = machine.unaligned_store(MachineRepresentation::Float64)?;
  assert_eq!(unaligned_store.opcode(), IrOpcode::UnalignedStore as u16);
  assert_eq!(unaligned_store.value_input_count(), 3);

  OK
}
