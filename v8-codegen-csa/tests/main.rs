use aok::{OK, Void};
use log::info;
use v8_codegen_csa::{
  AllocationFlag, BinaryOp, BuiltinKind, CallInterfaceDescriptor, CodeAssembler,
  CodeAssemblerLabel, CodeAssemblerState, CodeStubAssembler, CompareBinaryOp, Error,
  ExtractFixedArrayFlag, HoleConversionMode, NodeId, NodeOp, Object, TypeTag, UnaryOp, Word32T,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_tnode_basics() -> Void {
  info!("测试 TNode 基础类型标记与 NodeId 行为");
  let invalid_node = v8_codegen_csa::TNode::<Word32T>::invalid();
  assert!(!invalid_node.is_valid());
  assert_eq!(invalid_node.node_id(), NodeId::INVALID);

  let valid_node = v8_codegen_csa::TNode::<Word32T>::new(NodeId(42));
  assert!(valid_node.is_valid());
  assert_eq!(valid_node.node_id().raw(), 42);
  assert_eq!(Word32T::type_name(), "Word32T");
  assert_eq!(Word32T::machine_type(), Some("Word32"));
  assert!(!Word32T::is_tagged());
  assert!(Object::is_tagged());

  let cast_node: v8_codegen_csa::TNode<v8_codegen_csa::Int32T> = valid_node.unchecked_cast();
  assert_eq!(cast_node.node_id().raw(), 42);

  OK
}

#[test]
fn test_code_assembler_operators_count_and_predicates() -> Void {
  info!("测试 CodeAssembler 算子数量与分类谓词");
  assert_eq!(CompareBinaryOp::COUNT, 31);
  assert_eq!(BinaryOp::COUNT, 65);
  assert_eq!(UnaryOp::COUNT, 71);

  assert!(CompareBinaryOp::Float64Equal.is_float_compare());
  assert!(!CompareBinaryOp::Int32LessThan.is_float_compare());
  assert!(CompareBinaryOp::Int32LessThan.is_signed_int_compare());
  assert!(CompareBinaryOp::Uint32LessThan.is_unsigned_int_compare());

  assert!(BinaryOp::Int32Add.is_arithmetic());
  assert!(BinaryOp::WordOr.is_bitwise());
  assert!(!BinaryOp::Int32Add.is_bitwise());

  assert!(UnaryOp::BitcastTaggedToWord.is_bitcast());
  assert!(UnaryOp::ChangeInt32ToInt64.is_truncate_or_change());
  assert!(!UnaryOp::Float64Abs.is_bitcast());

  OK
}

#[test]
fn test_code_assembler_graph_construction() -> Void {
  info!("测试 CodeAssembler 图构造与标号状态机");
  let mut state = CodeAssemblerState::new();
  let mut csa = CodeAssembler::new(&mut state);

  let c1 = csa.int32_constant(100);
  let c2 = csa.int32_constant(200);
  let sum = csa.int32_add(c1, c2);
  let cmp = csa.int32_less_than(c1, sum);

  let mut label_then = csa.alloc_label("then_block");
  let mut label_else = csa.alloc_label("else_block");
  csa.branch(cmp, &mut label_then, &mut label_else);

  assert!(csa.bind(&mut label_then).is_ok());
  assert!(label_then.is_bound());
  assert_eq!(label_then.merged_count(), 1);

  // 二次绑定应抛出错误
  let double_bind_res = csa.bind(&mut label_then);
  assert_eq!(double_bind_res, Err(Error::LabelAlreadyBound("then_block")));

  assert!(csa.node_count() >= 4);

  OK
}

#[test]
fn test_code_stub_assembler_smi_and_type_checks() -> Void {
  info!("测试 CodeStubAssembler Smi 原语与类型检查谓词");
  let mut state = CodeAssemblerState::new();
  let mut csa = CodeStubAssembler::new(&mut state);

  // 1. Smi 原语
  let raw_intptr = csa.raw_assembler().intptr_constant(5);
  let smi_val = csa.smi_tag(raw_intptr);
  let untagged = csa.smi_untag(smi_val);
  assert!(untagged.is_valid());

  let smi_a = csa.raw_assembler().smi_constant(10);
  let smi_b = csa.raw_assembler().smi_constant(20);
  let smi_sum = csa.smi_add(smi_a, smi_b);
  let smi_diff = csa.smi_sub(smi_sum, smi_a);
  let smi_eq = csa.smi_equal(smi_diff, smi_a);
  assert!(smi_eq.is_valid());

  // 2. 标志与位集
  assert_eq!(AllocationFlag::Pretenured.bits(), 2);
  assert!(ExtractFixedArrayFlag::ALL_FIXED_ARRAYS.contains(ExtractFixedArrayFlag::FIXED_ARRAYS));
  assert_eq!(HoleConversionMode::DontConvert as u8, 0);

  // 3. 内存分配
  let alloc_size = csa.raw_assembler().intptr_constant(64);
  let heap_obj = csa.allocate_in_young_generation(alloc_size);
  assert!(heap_obj.is_valid());

  // 4. 类型检查谓词
  let is_rcv = csa.is_js_receiver(heap_obj);
  assert!(is_rcv.is_valid());

  OK
}

#[test]
fn test_node_record_inspection() -> Void {
  info!("测试 NodeRecord 节点结构与图记录");
  let mut state = CodeAssemblerState::new();
  let mut csa = CodeAssembler::new(&mut state);

  let node_a = csa.int32_constant(42);
  let node_b = csa.int32_constant(58);
  let node_c = csa.int32_add(node_a, node_b);
  assert!(node_c.is_valid());

  let nodes = state.nodes();
  assert_eq!(nodes.len(), 3);
  assert_eq!(nodes[0].op, NodeOp::ConstantInt32(42));
  assert_eq!(nodes[1].op, NodeOp::ConstantInt32(58));
  assert_eq!(nodes[2].op, NodeOp::Binary(BinaryOp::Int32Add));
  assert_eq!(nodes[2].inputs[0], node_a.node_id());
  assert_eq!(nodes[2].inputs[1], node_b.node_id());
  OK
}

#[test]
fn test_builtin_and_stub_calls() -> Void {
  info!("测试 CallBuiltin 与 CallStub 调用图节点");
  let mut state = CodeAssemblerState::new();
  let mut csa = CodeAssembler::new(&mut state);

  let arg1 = csa.int32_constant(1).node_id();
  let arg2 = csa.int32_constant(2).node_id();
  let builtin_node = csa.call_builtin(42, BuiltinKind::TFJ, &[arg1, arg2]);
  assert!(builtin_node.is_valid());

  let desc = CallInterfaceDescriptor::new(Default::default());
  let stub_node = csa.call_stub(&desc, &[arg1]);
  assert!(stub_node.is_valid());

  let mut lbl: CodeAssemblerLabel = csa.alloc_label("stub_entry");
  assert!(csa.bind(&mut lbl).is_ok());
  assert!(lbl.is_bound());

  OK
}
