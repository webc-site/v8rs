use aok::{OK, Void};
use log::info;
use v8_compiler_graph::{MachineRepresentation, MachineType};
use v8_turboshaft::{
  BlockIndex, BlockKind, FastHash, Graph, MaskBuilder, MemoryRepresentation, OpIndex, OpMaskField,
  Opcode, Origin, RegisterRepresentation, fast_hash_combine, fast_hash_combine2, fast_hash_range,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_representations_properties() -> Void {
  info!("Testing register and memory representations");

  // 寄存器表示属性与转换
  let w32 = RegisterRepresentation::Word32;
  let w64 = RegisterRepresentation::Word64;
  let tagged = RegisterRepresentation::Tagged;

  assert!(w32.to_maybe().is_word());
  assert!(w64.to_maybe().is_word());
  assert!(!tagged.to_maybe().is_word());
  assert!(tagged.to_maybe().is_tagged_or_compressed());

  assert_eq!(w32.to_maybe().bit_width(), 32);
  assert_eq!(w64.to_maybe().bit_width(), 64);
  assert_eq!(w32.to_maybe().max_unsigned_value(), u32::MAX as u64);
  assert_eq!(w64.to_maybe().max_unsigned_value(), u64::MAX);

  // 指针压缩映射测试
  assert_eq!(tagged.map_tagged_to_word(), RegisterRepresentation::Word32);
  assert_eq!(w64.map_tagged_to_word(), RegisterRepresentation::Word64);

  // 内存表示尺寸与类型映射
  assert_eq!(MemoryRepresentation::Int8.size_in_bytes(), 1);
  assert_eq!(MemoryRepresentation::Int16.size_in_bytes(), 2);
  assert_eq!(MemoryRepresentation::Int32.size_in_bytes(), 4);
  assert_eq!(MemoryRepresentation::Int64.size_in_bytes(), 8);
  assert_eq!(MemoryRepresentation::Simd128.size_in_bytes(), 16);
  assert_eq!(MemoryRepresentation::Simd256.size_in_bytes(), 32);

  assert!(MemoryRepresentation::Int32.is_signed());
  assert!(!MemoryRepresentation::Uint32.is_signed());
  assert!(MemoryRepresentation::AnyTagged.is_compressible_tagged());
  assert!(!MemoryRepresentation::Int64.is_compressible_tagged());

  assert_eq!(
    MemoryRepresentation::Int32.to_register_representation(),
    RegisterRepresentation::Word32
  );
  assert_eq!(
    MemoryRepresentation::Int64.to_register_representation(),
    RegisterRepresentation::Word64
  );

  assert_eq!(
    MemoryRepresentation::Int32.to_machine_type(),
    MachineType::int32()
  );
  assert_eq!(
    MemoryRepresentation::Float64.to_machine_type(),
    MachineType::float64()
  );

  // MachineRepresentation 桥接
  assert_eq!(
    RegisterRepresentation::from_machine_representation(MachineRepresentation::Word32),
    RegisterRepresentation::Word32
  );
  assert_eq!(
    MemoryRepresentation::from_machine_representation(MachineRepresentation::Word64),
    MemoryRepresentation::Uint64
  );

  OK
}

#[test]
fn test_opcodes() -> Void {
  info!("Testing turboshaft opcodes");

  // 块终结符测试
  assert!(Opcode::CheckException.is_block_terminator());
  assert!(Opcode::Goto.is_block_terminator());
  assert!(Opcode::Return.is_block_terminator());
  assert!(Opcode::Branch.is_block_terminator());
  assert!(Opcode::WasmTrap.is_block_terminator());
  assert!(!Opcode::Call.is_block_terminator());

  // MayThrow 测试
  assert!(Opcode::Call.may_throw());
  assert!(Opcode::FastApiCall.may_throw());
  assert!(Opcode::GenericBinop.may_throw());
  assert!(!Opcode::Goto.may_throw());

  OK
}

#[test]
fn test_opmasks() -> Void {
  info!("Testing opmask construction and matching");

  // 模拟操作数据掩码构造
  let opcode = Opcode::WordBinop;
  static FIELDS: [OpMaskField; 1] = [OpMaskField {
    name: "kind",
    offset: 1,
    size: 1,
  }];

  let builder = MaskBuilder { fields: &FIELDS };
  let mask = builder.r#for(opcode.index(), &[5]);

  assert_eq!(mask.mask & 0xFF, 0xFF);
  assert_eq!((mask.value & 0xFF) as u8, opcode.index());
  assert_eq!(((mask.value >> 8) & 0xFF) as u8, 5);

  // 匹配操作掩码
  let matching_word = (opcode.index() as u64) | (5u64 << 8) | (0xDEADu64 << 16);
  assert!(mask.matches(matching_word));

  let mismatch_word = (opcode.index() as u64) | (6u64 << 8);
  assert!(!mask.matches(mismatch_word));

  OK
}

#[test]
fn test_fast_hash() -> Void {
  info!("Testing turboshaft fast hash");

  // 原版分发表不变式 (fast-hash.h): size_t 直传, optional 位移编码
  assert_eq!(None::<u64>.fast_hash(), 0);
  assert_eq!(Some(0u64).fast_hash(), 1);
  assert_eq!(12345u64.fast_hash(), 12345);
  assert_eq!(
    Some(5u64).fast_hash(),
    (5u64.fast_hash() << 1) + 1,
    "optional fast_hash: (hash(v) << 1) + 1"
  );

  // 变参展开: 首元素权重 1, 逐位 17 倍后移
  assert_eq!(fast_hash_combine(&[]), 0);
  assert_eq!(fast_hash_combine(&[5]), 5);
  assert_eq!(fast_hash_combine(&[3, 5]), 17 * 5 + 3);

  // 两参 combine: 17 * acc + value
  assert_eq!(fast_hash_combine2(0, 7), 7);

  // 左折叠 range: combine2(combine2(0,1),2) = 17*1+2 = 19
  assert_eq!(fast_hash_range(&[1, 2]), 19);

  let h1 = 12345u64.fast_hash();
  let h2 = 67890u64.fast_hash();
  assert_eq!(fast_hash_combine(&[h1, h2]), 17 * h2 + h1);

  OK
}

#[test]
fn test_graph_and_blocks() -> Void {
  info!("Testing turboshaft graph and block management");

  // 原版 Graph 初始空态: 无绑定块, 无操作槽
  let mut graph = Graph::new(Origin::PureTurboshaft);
  assert_eq!(graph.origin, Origin::PureTurboshaft);
  assert_eq!(graph.block_count(), 0);
  assert_eq!(graph.op_id_count(), 0);
  assert_eq!(graph.next_block_index(), BlockIndex::new(0));

  // NewBlock: 追加已绑定块, BlockIndex 递增; 块 0 为入口 (start_block)
  let b0 = graph.new_block(BlockKind::Merge);
  assert_eq!(b0, BlockIndex::new(0));
  assert_eq!(graph.block_count(), 1);
  assert!(graph.start_block().is_some_and(|b| b.is_merge()));
  assert_eq!(graph.start_block().map(|b| b.kind.as_str()), Some("MERGE"));

  let b1 = graph.new_block(BlockKind::LoopHeader);
  assert_eq!(b1, BlockIndex::new(1));
  assert!(graph.get_block(b1).is_some_and(|b| b.is_loop_header()));
  assert!(graph.get_block(BlockIndex::new(9)).is_none());
  assert_eq!(graph.blocks().len(), 2);
  assert_eq!(graph.blocks().iter().filter(|b| b.is_merge()).count(), 1);

  // 操作索引运算
  let op0 = OpIndex::from_offset(0);
  assert!(op0.valid());
  assert_eq!(op0.offset(), 0);
  assert_eq!(op0.id(), 0);
  assert!(!op0.is_external_id());

  let ext_op = OpIndex::encode_external_id(42);
  assert!(ext_op.is_external_id());
  assert_eq!(ext_op.decode_external_id(), 42);

  OK
}
