use std::collections::HashSet;

use aok::{OK, Void};
use log::info;
use v8_ast::{
  AstStringConstant, Declaration, Error, Expression, HoleCheckState, ScopeTree, Statement,
  ast::*,
  modules::*,
  scopes::*,
  value::{AstRawString, AstValue, AstValueFactory},
  variables::*,
  visitor::AstVisitor,
};
use v8_common::{
  InitializationFlag, IsStaticFlag, MaybeAssignedFlag, ScopeType, VariableKind, VariableLocation,
  VariableMode,
};
use v8_zone::{AccountingAllocator, Zone};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 对应 v8/test/unittests/parser/ast-value-unittest.cc: AstRawStringAsArrayIndex
#[test]
fn test_ast_raw_string_as_array_index() -> Void {
  let test_index = |s: &str, expected_ok: bool, expected_idx: u32| {
    let raw = AstRawString::from_utf8(s);
    let mut idx = 0;
    let ok = raw.as_array_index(&mut idx);
    assert_eq!(ok, expected_ok, "string: {}", s);
    if expected_ok {
      assert_eq!(idx, expected_idx, "string: {}", s);
    }
  };

  // 缓存长度 (length <= 7)
  test_index("0", true, 0);
  test_index("1", true, 1);
  test_index("1234567", true, 1234567);

  // 未缓存长度 (7 < length <= 10)
  test_index("12345678", true, 12345678);
  test_index("1000000000", true, 1000000000);

  // 最大数组索引 (4294967294)
  test_index("4294967294", true, 4294967294);

  // 溢出
  test_index("4294967295", false, 0);
  test_index("5000000000", false, 0);
  test_index("10000000000", false, 0);

  // 非合法索引 (前导 0 或非数字)
  test_index("01", false, 0);
  test_index("foo", false, 0);

  // 双字节包含非 Latin-1
  let non_ascii = AstRawString::from_two_byte(&[0x1234, 0x5678]);
  assert!(!non_ascii.is_one_byte());
  let mut idx = 0;
  assert!(!non_ascii.as_array_index(&mut idx));

  info!("test_ast_raw_string_as_array_index passed");
  OK
}

/// 对应 v8/test/unittests/parser/ast-value-unittest.cc: BigIntToBooleanIsTrue
#[test]
fn test_bigint_to_boolean_is_true() -> Void {
  let factory = AstValueFactory::new();

  // 假值 (全部为 0)
  assert!(!factory.new_bigint("0").to_boolean_is_true());
  assert!(!factory.new_bigint("0b0").to_boolean_is_true());
  assert!(!factory.new_bigint("0o0").to_boolean_is_true());
  assert!(!factory.new_bigint("0x0").to_boolean_is_true());
  assert!(!factory.new_bigint("0b000").to_boolean_is_true());
  assert!(!factory.new_bigint("0o00000").to_boolean_is_true());
  assert!(!factory.new_bigint("0x000000000").to_boolean_is_true());

  // 真值 (非 0)
  assert!(factory.new_bigint("3").to_boolean_is_true());
  assert!(factory.new_bigint("0b1").to_boolean_is_true());
  assert!(factory.new_bigint("0o6").to_boolean_is_true());
  assert!(factory.new_bigint("0xA").to_boolean_is_true());
  assert!(factory.new_bigint("0b0000001").to_boolean_is_true());
  assert!(factory.new_bigint("0o00005000").to_boolean_is_true());
  assert!(factory.new_bigint("0x000000D00C0").to_boolean_is_true());

  info!("test_bigint_to_boolean_is_true passed");
  OK
}

/// AST 构造与 enum_dispatch 派发测试
#[test]
fn test_ast_construction_and_enum_dispatch() -> Void {
  let lit42 = AstNode::from(AstNodeFactory::new_literal(10, AstValue::Number(42.0)));
  let lit10 = AstNode::from(AstNodeFactory::new_literal(15, AstValue::Number(10.0)));

  // enum_dispatch 测试: 直接调用 trait 方法
  assert_eq!(lit42.node_type(), AstNodeType::Literal);
  assert_eq!(lit42.position(), 10);
  assert!(lit42.is_expression());
  assert!(lit42.is_literal());
  assert!(!lit42.is_statement());

  let binop = AstNode::from(AstNodeFactory::new_binary_operation(
    20,
    AstBinaryOp::Add,
    lit42,
    lit10,
  ));

  assert_eq!(binop.node_type(), AstNodeType::BinaryOperation);
  assert_eq!(binop.position(), 20);
  assert!(binop.is_expression());
  assert!(!binop.is_literal());

  if let Some(bo) = binop.as_binary_operation() {
    assert_eq!(bo.op, AstBinaryOp::Add);
    assert_eq!(bo.op.as_str(), "+");
    assert_eq!(bo.op.precedence(), 12);
  } else {
    panic!("expected BinaryOperation");
  }

  info!("test_ast_construction_and_enum_dispatch passed");
  OK
}

/// AstVisitor 遍历器测试
#[test]
fn test_ast_visitor() -> Void {
  // 构造一棵包含语句与表达式的 AST:
  // if (x) { return 42; }
  let proxy_x = AstNode::from(AstNodeFactory::new_variable_proxy(
    5,
    AstRawString::from_utf8("x"),
    VariableProxyBindingMode::Bound,
    HoleCheckMode::Required,
  ));

  let lit_42 = AstNode::from(AstNodeFactory::new_literal(15, AstValue::Number(42.0)));

  let ret_stmt = AstNode::from(AstNodeFactory::new_return_statement(
    12,
    Some(lit_42),
    ReturnType::Normal,
  ));

  let block = AstNode::from(AstNodeFactory::new_block(10, vec![ret_stmt], false));

  let if_stmt = AstNode::from(AstNodeFactory::new_if_statement(0, proxy_x, block, None));

  // 统计各类节点的访问者
  #[derive(Default)]
  struct CounterVisitor {
    node_count: usize,
    if_count: usize,
    return_count: usize,
    literal_count: usize,
    variable_proxy_count: usize,
  }

  impl AstVisitor for CounterVisitor {
    fn visit(&mut self, node: &AstNode) {
      self.node_count += 1;
      match node {
        AstNode::IfStatement(_) => self.if_count += 1,
        AstNode::ReturnStatement(_) => self.return_count += 1,
        AstNode::Literal(_) => self.literal_count += 1,
        AstNode::VariableProxy(_) => self.variable_proxy_count += 1,
        _ => {}
      }
      // 继续递归遍历子节点
      visitor_super_visit(self, node);
    }
  }

  fn visitor_super_visit(visitor: &mut CounterVisitor, node: &AstNode) {
    match node {
      AstNode::IfStatement(n) => {
        visitor.visit(&n.condition);
        visitor.visit(&n.then_statement);
        if let Some(el) = &n.else_statement {
          visitor.visit(el);
        }
      }
      AstNode::Block(n) => {
        for s in &n.statements {
          visitor.visit(s);
        }
      }
      AstNode::ReturnStatement(n) => {
        if let Some(expr) = &n.expression {
          visitor.visit(expr);
        }
      }
      _ => {}
    }
  }

  let mut visitor = CounterVisitor::default();
  visitor.visit(&if_stmt);

  assert_eq!(visitor.if_count, 1);
  assert_eq!(visitor.return_count, 1);
  assert_eq!(visitor.literal_count, 1);
  assert_eq!(visitor.variable_proxy_count, 1);
  // if_stmt + proxy_x + block + ret_stmt + lit_42 = 5 节点
  assert_eq!(visitor.node_count, 5);

  info!("test_ast_visitor passed");
  OK
}

/// 模块导入导出描述符测试
#[test]
fn test_source_text_module_descriptor() -> Void {
  let mut desc = SourceTextModuleDescriptor::new();

  // 添加常规导入
  desc.add_import(
    AstRawString::from_utf8("foo"),
    AstRawString::from_utf8("local_foo"),
    AstRawString::from_utf8("./module.js"),
    ModuleImportPhase::Evaluation,
    ScannerLocation::new(0, 10),
  );

  // 添加常规本地导出 (同一个 local_foo 导出两个名字)
  desc.add_export(
    AstRawString::from_utf8("local_foo"),
    AstRawString::from_utf8("exported_a"),
    ScannerLocation::new(20, 30),
  );
  desc.add_export(
    AstRawString::from_utf8("local_foo"),
    AstRawString::from_utf8("exported_b"),
    ScannerLocation::new(31, 40),
  );

  // 添加星号重导出
  desc.add_star_export(
    AstRawString::from_utf8("./other.js"),
    ModuleImportPhase::Evaluation,
    ScannerLocation::new(50, 60),
  );

  assert_eq!(desc.module_requests().len(), 2);
  assert_eq!(desc.regular_imports().len(), 1);
  assert_eq!(desc.regular_exports().len(), 2);
  assert_eq!(desc.special_exports().len(), 1);

  // 分配槽位
  desc.assign_cell_indices();

  // 导入 slot 应为 -1
  assert_eq!(desc.regular_imports()[0].cell_index, -1);
  assert_eq!(
    desc.regular_imports()[0].cell_index_kind(),
    CellIndexKind::Import
  );

  // 同一个 local_name 的导出共享同一 slot (+1)
  assert_eq!(desc.regular_exports()[0].cell_index, 1);
  assert_eq!(desc.regular_exports()[1].cell_index, 1);
  assert_eq!(
    desc.regular_exports()[0].cell_index_kind(),
    CellIndexKind::Export
  );

  info!("test_source_text_module_descriptor passed");
  OK
}

/// 对应 v8/test/unittests/parser/ast-value-unittest.cc: GetStringNormalizesOneByteContent
#[test]
fn test_get_string_normalizes_one_byte_content() -> Void {
  let mut factory = AstValueFactory::new();

  // 双字节形式但内容全是 Latin-1 (<= 0xFF) 的 "1000000000"
  let two_byte_chars: Vec<u16> = "1000000000".encode_utf16().collect();
  let raw = factory.get_two_byte_string(&two_byte_chars);
  assert!(raw.is_one_byte(), "should be normalized to one-byte");
  let mut index = 0;
  assert!(raw.as_array_index(&mut index));
  assert_eq!(index, 1000000000);

  // 双字节形式包含非 Latin-1 字符 (0x1234)
  let non_one_byte = factory.get_two_byte_string(&[0x1234, 0x5678]);
  assert!(!non_one_byte.is_one_byte());
  let mut dummy_idx = 0;
  assert!(!non_one_byte.as_array_index(&mut dummy_idx));

  info!("test_get_string_normalizes_one_byte_content passed");
  OK
}

/// AstValueFactory 字典去重测试
#[test]
fn test_ast_value_factory_deduplication() -> Void {
  let mut factory = AstValueFactory::new();

  let s1 = factory.get_string("hello");
  let s2 = factory.get_one_byte_string("hello");
  let utf16: Vec<u16> = "hello".encode_utf16().collect();
  let s3 = factory.get_two_byte_string(&utf16);

  assert_eq!(s1, s2);
  assert_eq!(s2, s3);
  assert_eq!(
    factory.string_count(),
    1,
    "all three should share one entry"
  );

  let non_ascii_1 = factory.get_two_byte_string(&[0x3042, 0x3044]); // あい
  let non_ascii_2 = factory.get_two_byte_string(&[0x3042, 0x3044]);
  assert_eq!(non_ascii_1, non_ascii_2);
  assert_eq!(factory.string_count(), 2);

  info!("test_ast_value_factory_deduplication passed");
  OK
}

/// 模块槽位排序分配与隐式间接重导出测试
#[test]
fn test_source_text_module_descriptor_sorting_and_indirect_exports() -> Void {
  let mut desc = SourceTextModuleDescriptor::new();

  // 1. 无序添加导出项: z, a, z (多次), b
  desc.add_export(
    AstRawString::from_utf8("z"),
    AstRawString::from_utf8("exp_z1"),
    ScannerLocation::new(0, 1),
  );
  desc.add_export(
    AstRawString::from_utf8("a"),
    AstRawString::from_utf8("exp_a"),
    ScannerLocation::new(2, 3),
  );
  desc.add_export(
    AstRawString::from_utf8("z"),
    AstRawString::from_utf8("exp_z2"),
    ScannerLocation::new(4, 5),
  );
  desc.add_export(
    AstRawString::from_utf8("b"),
    AstRawString::from_utf8("exp_b"),
    ScannerLocation::new(6, 7),
  );

  // 2. 无序添加导入项: y, x
  assert!(desc.add_import(
    AstRawString::from_utf8("orig_y"),
    AstRawString::from_utf8("y"),
    AstRawString::from_utf8("./mod_y.js"),
    ModuleImportPhase::Evaluation,
    ScannerLocation::new(10, 11),
  ));
  assert!(desc.add_import(
    AstRawString::from_utf8("orig_x"),
    AstRawString::from_utf8("x"),
    AstRawString::from_utf8("./mod_x.js"),
    ModuleImportPhase::Evaluation,
    ScannerLocation::new(12, 13),
  ));

  // 重复导入同一 local_name 应返回 false
  assert!(!desc.add_import(
    AstRawString::from_utf8("other_x"),
    AstRawString::from_utf8("x"),
    AstRawString::from_utf8("./mod_x2.js"),
    ModuleImportPhase::Evaluation,
    ScannerLocation::new(14, 15),
  ));

  // 3. 分配槽位
  desc.assign_cell_indices();

  // 常规导出应按 local_name 码点升序分配:
  // "a" -> +1, "b" -> +2, "z" 两个条目均共享 +3
  let exports = desc.regular_exports();
  assert_eq!(exports.len(), 4);
  assert_eq!(
    exports[0].local_name.as_ref().unwrap().to_string_lossy(),
    "a"
  );
  assert_eq!(exports[0].cell_index, 1);
  assert_eq!(
    exports[1].local_name.as_ref().unwrap().to_string_lossy(),
    "b"
  );
  assert_eq!(exports[1].cell_index, 2);
  assert_eq!(
    exports[2].local_name.as_ref().unwrap().to_string_lossy(),
    "z"
  );
  assert_eq!(exports[2].cell_index, 3);
  assert_eq!(
    exports[3].local_name.as_ref().unwrap().to_string_lossy(),
    "z"
  );
  assert_eq!(exports[3].cell_index, 3);

  // 常规导入应按 local_name 码点升序分配负槽位:
  // "x" -> -1, "y" -> -2
  let imports = desc.regular_imports();
  assert_eq!(imports.len(), 2);
  assert_eq!(
    imports[0].local_name.as_ref().unwrap().to_string_lossy(),
    "x"
  );
  assert_eq!(imports[0].cell_index, -1);
  assert_eq!(
    imports[1].local_name.as_ref().unwrap().to_string_lossy(),
    "y"
  );
  assert_eq!(imports[1].cell_index, -2);

  // 4. 测试隐式间接重导出:
  // import { orig_x as x } from "./mod_x.js";
  // export { x as my_reexport };
  let mut desc2 = SourceTextModuleDescriptor::new();
  desc2.add_import(
    AstRawString::from_utf8("orig_x"),
    AstRawString::from_utf8("x"),
    AstRawString::from_utf8("./mod_x.js"),
    ModuleImportPhase::Evaluation,
    ScannerLocation::new(0, 10),
  );
  desc2.add_export(
    AstRawString::from_utf8("x"),
    AstRawString::from_utf8("my_reexport"),
    ScannerLocation::new(20, 30),
  );
  assert_eq!(desc2.regular_exports().len(), 1);
  assert_eq!(desc2.special_exports().len(), 0);

  desc2.make_indirect_exports_explicit();
  assert_eq!(desc2.regular_exports().len(), 0);
  assert_eq!(desc2.special_exports().len(), 1);
  let sp = &desc2.special_exports()[0];
  assert_eq!(
    sp.export_name.as_ref().unwrap().to_string_lossy(),
    "my_reexport"
  );
  assert_eq!(sp.import_name.as_ref().unwrap().to_string_lossy(), "orig_x");
  assert_eq!(sp.module_request, 0);
  assert!(sp.local_name.is_none());

  info!("test_source_text_module_descriptor_sorting_and_indirect_exports passed");
  OK
}

/// AstVisitor 默认进入与回溯、剪枝测试
#[test]
fn test_ast_visitor_enter_leave_and_pruning() -> Void {
  // 构建: if (true) { return 1; }
  let lit_true = AstNode::from(AstNodeFactory::new_literal(1, AstValue::Boolean(true)));
  let lit_1 = AstNode::from(AstNodeFactory::new_literal(3, AstValue::Smi(1)));
  let ret = AstNode::from(AstNodeFactory::new_return_statement(
    2,
    Some(lit_1),
    ReturnType::Normal,
  ));
  let block = AstNode::from(AstNodeFactory::new_block(2, vec![ret], false));
  let if_node = AstNode::from(AstNodeFactory::new_if_statement(0, lit_true, block, None));

  // 测试 1: 跟踪进入 (pre-order) 与离开 (post-order) 序列
  #[derive(Default)]
  struct TraceVisitor {
    trace: Vec<(&'static str, AstNodeType)>,
  }

  impl AstVisitor for TraceVisitor {
    fn enter_node(&mut self, node: &AstNode) -> bool {
      self.trace.push(("enter", node.node_type()));
      true
    }

    fn leave_node(&mut self, node: &AstNode) {
      self.trace.push(("leave", node.node_type()));
    }
  }

  let mut tracer = TraceVisitor::default();
  tracer.visit(&if_node);

  // 验证先序进入和后序离开对称性
  let enters: Vec<AstNodeType> = tracer
    .trace
    .iter()
    .filter(|(action, _)| *action == "enter")
    .map(|(_, ty)| *ty)
    .collect();
  let leaves: Vec<AstNodeType> = tracer
    .trace
    .iter()
    .filter(|(action, _)| *action == "leave")
    .map(|(_, ty)| *ty)
    .collect();

  assert_eq!(enters.len(), 5);
  assert_eq!(leaves.len(), 5);
  assert_eq!(enters[0], AstNodeType::IfStatement);
  assert_eq!(leaves.last(), Some(&AstNodeType::IfStatement));

  // 测试 2: 剪枝 (pruning) — 当遇到 Block 时返回 false 阻止递归子节点
  #[derive(Default)]
  struct PruneVisitor {
    visited: Vec<AstNodeType>,
  }

  impl AstVisitor for PruneVisitor {
    fn enter_node(&mut self, node: &AstNode) -> bool {
      self.visited.push(node.node_type());
      !matches!(node, AstNode::Block(_))
    }
  }

  let mut pruner = PruneVisitor::default();
  pruner.visit(&if_node);
  // 应包含 IfStatement, Literal (true), Block, 但 ReturnStatement 和 Smi(1) 被剪枝跳过
  assert_eq!(pruner.visited.len(), 3);
  assert!(!pruner.visited.contains(&AstNodeType::ReturnStatement));

  info!("test_ast_visitor_enter_leave_and_pruning passed");
  OK
}

/// ClassLiteral 静态元素与 SuperCallReference 完整遍历测试
#[test]
fn test_class_literal_and_super_call_reference_traversal() -> Void {
  // 构建 ClassLiteral 包含属性与静态块
  let prop_key = AstNode::from(AstNodeFactory::new_literal(
    1,
    AstValue::String(AstRawString::from_utf8("static_field")),
  ));
  let prop_val = AstNode::from(AstNodeFactory::new_literal(2, AstValue::Smi(100)));
  let static_prop = ClassLiteralProperty::new(
    ClassPropertyKind::Field,
    prop_key,
    prop_val,
    true,
    false,
    false,
  );

  let static_block_stmt = AstNode::from(AstNodeFactory::new_empty_statement(3));
  let static_block = AstNodeFactory::new_block(3, vec![static_block_stmt], false);

  let static_elements = vec![
    ClassLiteralStaticElement::property(static_prop),
    ClassLiteralStaticElement::static_block(static_block),
  ];

  let class_node = AstNode::ClassLiteral(AstNodeFactory::new_class_literal(
    0,
    ClassLiteralOptions {
      name: Some(AstRawString::from_utf8("MyClass")),
      static_elements,
      ..ClassLiteralOptions::default()
    },
  ));

  // 构建 SuperCallReference 包含 new_target_var 与 this_function_var
  let nt_proxy = AstNodeFactory::new_variable_proxy(
    10,
    AstRawString::from_utf8(".new.target"),
    VariableProxyBindingMode::Bound,
    HoleCheckMode::Required,
  );
  let tf_proxy = AstNodeFactory::new_variable_proxy(
    11,
    AstRawString::from_utf8(".this_function"),
    VariableProxyBindingMode::Bound,
    HoleCheckMode::Required,
  );

  let super_call_ref = AstNode::from(AstNodeFactory::new_super_call_reference(
    10,
    Some(nt_proxy),
    Some(tf_proxy),
  ));

  #[derive(Default)]
  struct NodeCollector {
    types: Vec<AstNodeType>,
  }

  impl AstVisitor for NodeCollector {
    fn enter_node(&mut self, node: &AstNode) -> bool {
      self.types.push(node.node_type());
      true
    }

    fn visit_variable_proxy(&mut self, _node: &VariableProxy) {
      self.types.push(AstNodeType::VariableProxy);
    }
  }

  let mut collector = NodeCollector::default();
  collector.visit(&class_node);

  // 验证 ClassLiteral 的静态元素中的属性 key/value 以及 static_block 均被完整遍历
  assert!(collector.types.contains(&AstNodeType::ClassLiteral));
  assert!(collector.types.contains(&AstNodeType::Literal));
  assert!(collector.types.contains(&AstNodeType::EmptyStatement));

  let mut super_collector = NodeCollector::default();
  super_collector.visit(&super_call_ref);
  assert_eq!(super_collector.types.len(), 3);
  assert_eq!(super_collector.types[0], AstNodeType::SuperCallReference);
  assert_eq!(super_collector.types[1], AstNodeType::VariableProxy);
  assert_eq!(super_collector.types[2], AstNodeType::VariableProxy);

  info!("test_class_literal_and_super_call_reference_traversal passed");
  OK
}

/// 特殊索引 is_special_index 测试
#[test]
fn test_is_special_index_coverage() -> Void {
  // 单字节特殊索引
  let nan = AstRawString::from_utf8("NaN");
  assert!(nan.is_special_index());
  let inf = AstRawString::from_utf8("Infinity");
  assert!(inf.is_special_index());
  let neg_inf = AstRawString::from_utf8("-Infinity");
  assert!(neg_inf.is_special_index());
  let num_str = AstRawString::from_utf8("123");
  assert!(num_str.is_special_index());

  // 双字节特殊索引 (包含非 Latin-1 的不可能为特殊索引)
  let non_ascii = AstRawString::from_two_byte(&[0x1234, 0x5678]);
  assert!(!non_ascii.is_special_index());

  // 双字节但纯数字
  let two_byte_num = AstRawString::from_two_byte(&[b'4' as u16, b'2' as u16]);
  // 规范化后为单字节
  assert!(two_byte_num.is_one_byte());
  assert!(two_byte_num.is_special_index());

  info!("test_is_special_index_coverage passed");
  OK
}

/// Declaration, Statement, Expression 子类枚举转换测试
#[test]
fn test_subclass_node_enums_conversion() -> Void {
  // 1. Declaration
  let empty_stmt = AstNodeFactory::new_empty_statement(0);
  assert!(Declaration::from_node(AstNode::from(empty_stmt)).is_none());

  let var_node = AstNode::from(AstNodeFactory::new_variable_declaration(
    1,
    Some(AstRawString::from_utf8("foo")),
    VariableMode::Let,
  ));
  let decl = Declaration::from_node(var_node.clone());
  assert!(decl.is_some());
  let decl = decl.unwrap();
  assert_eq!(decl.node_type(), AstNodeType::VariableDeclaration);
  assert_eq!(decl.into_node(), var_node);

  // 2. Statement
  let stmt_node = AstNode::from(AstNodeFactory::new_empty_statement(5));
  let stmt = Statement::from_node(stmt_node.clone());
  assert!(stmt.is_some());
  let stmt = stmt.unwrap();
  assert_eq!(stmt.node_type(), AstNodeType::EmptyStatement);
  assert_eq!(stmt.into_node(), stmt_node);

  // 3. Expression
  let expr_node = AstNode::from(AstNodeFactory::new_literal(10, AstValue::Smi(123)));
  let expr = Expression::from_node(expr_node.clone());
  assert!(expr.is_some());
  let expr = expr.unwrap();
  assert_eq!(expr.node_type(), AstNodeType::Literal);
  assert_eq!(expr.into_node(), expr_node);

  // 表达式转为语句应为 None
  assert!(Statement::from_node(expr_node).is_none());

  info!("test_subclass_node_enums_conversion passed");
  OK
}

/// AstStringConstant 常量表测试
#[test]
fn test_ast_string_constants_and_factory() -> Void {
  assert_eq!(AstStringConstant::COUNT, 177);
  assert_eq!(AstStringConstant::ONE_CHAR_COUNT, 128);

  // 单字符 ASCII 常量
  assert_eq!(AstStringConstant::Space.contents(), " ");
  assert_eq!(AstStringConstant::Plus.contents(), "+");
  assert_eq!(AstStringConstant::Zero.contents(), "0");

  // 命名常量
  assert_eq!(AstStringConstant::Anonymous.contents(), "anonymous");
  assert_eq!(AstStringConstant::DotResult.contents(), ".result");
  assert_eq!(AstStringConstant::Constructor.contents(), "constructor");
  assert_eq!(AstStringConstant::Prototype.contents(), "prototype");

  // 通过 AstValueFactory 验证去重
  let mut factory = AstValueFactory::new();
  let s_ctor = factory.get_string(AstStringConstant::Constructor.contents());
  assert_eq!(s_ctor.to_string_lossy(), "constructor");
  assert!(s_ctor.is_one_byte());

  info!("test_ast_string_constants_and_factory passed");
  OK
}

/// Variable 空洞检测分析与位图测试
#[test]
fn test_variable_hole_check_bitmap_and_state() -> Void {
  let mut var = Variable::new(
    AstRawString::from_utf8("hoist_me"),
    VariableMode::Let,
    VariableKind::NormalVariable,
    InitializationFlag::NeedsInitialization,
    MaybeAssignedFlag::NotAssigned,
    IsStaticFlag::NotStatic,
  );

  assert_eq!(var.hole_check_state(), HoleCheckState::Uncached);
  var.set_hole_check_state(HoleCheckState::Force);
  assert_eq!(var.hole_check_state(), HoleCheckState::Force);

  assert!(!var.is_hole_initialization_forced());
  var.force_hole_initialization(force_hole_init::HAS_HOLE_CHECK_USE_IN_SAME_CLOSURE_SCOPE);
  assert!(var.is_hole_initialization_forced());
  assert!(var.has_hole_check_use_in_same_closure_scope());

  // 位图测试: 默认不可缓存索引 0
  assert_eq!(var.hole_check_bitmap_index(), 0);
  let bitmap = 0u64;
  assert_eq!(var.remember_hole_check_in_bitmap(bitmap), 0);
  assert!(!var.has_remembered_hole_check(bitmap));

  // 分配位图索引 3
  var.assign_hole_check_bitmap_index(3);
  assert_eq!(var.hole_check_bitmap_index(), 3);
  let updated_bitmap = var.remember_hole_check_in_bitmap(bitmap);
  assert_eq!(updated_bitmap, 1u64 << 3);
  assert!(var.has_remembered_hole_check(updated_bitmap));
  assert!(!var.has_remembered_hole_check(1u64 << 2));

  info!("test_variable_hole_check_bitmap_and_state passed");
  OK
}

/// 构造全部 60 类节点的最小实例, 全部经工厂构造器 (顺序 = AstNodeType 判别值 0..COUNT)
fn all_node_samples() -> Vec<AstNode> {
  // 叶子节点工厂: 每个表达式位均需独立实例 (AstNode 非 Copy)
  let lit = || AstNode::from(AstNodeFactory::new_literal(1, AstValue::Smi(0)));
  let stmt = || AstNode::from(AstNodeFactory::new_empty_statement(1));
  let name = || AstRawString::from_utf8("x");
  let block = |pos: i32| AstNodeFactory::new_block(pos, Vec::new(), false);
  let vp = || {
    AstNode::from(AstNodeFactory::new_variable_proxy(
      1,
      AstRawString::from_utf8("x"),
      VariableProxyBindingMode::Bound,
      HoleCheckMode::Required,
    ))
  };

  vec![
    AstNode::VariableDeclaration(AstNodeFactory::new_variable_declaration(
      0,
      Some(name()),
      VariableMode::Var,
    )),
    AstNode::FunctionDeclaration(AstNodeFactory::new_function_declaration(0, None)),
    AstNode::DoWhileStatement(AstNodeFactory::new_do_while_statement(0, stmt(), lit())),
    AstNode::WhileStatement(AstNodeFactory::new_while_statement(0, lit(), stmt())),
    AstNode::ForStatement(AstNodeFactory::new_for_statement(
      0,
      None,
      None,
      None,
      stmt(),
    )),
    AstNode::ForInStatement(AstNodeFactory::new_for_in_statement(
      0,
      vp(),
      lit(),
      stmt(),
      None,
    )),
    AstNode::ForOfStatement(AstNodeFactory::new_for_of_statement(
      0,
      vp(),
      lit(),
      stmt(),
      IteratorType::Normal,
      None,
    )),
    AstNode::Block(block(0)),
    AstNode::SwitchStatement(AstNodeFactory::new_switch_statement(
      0,
      lit(),
      vec![CaseClause::new(None, Vec::new())],
    )),
    AstNode::ExpressionStatement(AstNodeFactory::new_expression_statement(0, lit())),
    AstNode::EmptyStatement(AstNodeFactory::new_empty_statement(0)),
    AstNode::SloppyBlockFunctionStatement(AstNodeFactory::new_sloppy_block_function_statement(
      0,
      stmt(),
    )),
    AstNode::IfStatement(AstNodeFactory::new_if_statement(0, lit(), stmt(), None)),
    AstNode::ContinueStatement(AstNodeFactory::new_continue_statement(0, 2)),
    AstNode::BreakStatement(AstNodeFactory::new_break_statement(0, 2)),
    AstNode::ReturnStatement(AstNodeFactory::new_return_statement(
      0,
      None,
      ReturnType::Normal,
    )),
    AstNode::WithStatement(AstNodeFactory::new_with_statement(0, lit(), stmt())),
    AstNode::TryCatchStatement(AstNodeFactory::new_try_catch_statement(
      0,
      block(1),
      block(2),
      None,
      CatchPrediction::Caught,
    )),
    AstNode::TryFinallyStatement(AstNodeFactory::new_try_finally_statement(
      0,
      block(1),
      block(2),
    )),
    AstNode::DebuggerStatement(AstNodeFactory::new_debugger_statement(0)),
    AstNode::InitializeClassMembersStatement(
      AstNodeFactory::new_initialize_class_members_statement(0, Vec::new()),
    ),
    AstNode::InitializeClassStaticElementsStatement(
      AstNodeFactory::new_initialize_class_static_elements_statement(0, Vec::new()),
    ),
    AstNode::AutoAccessorGetterBody(AstNodeFactory::new_auto_accessor_getter_body(0, None)),
    AstNode::AutoAccessorSetterBody(AstNodeFactory::new_auto_accessor_setter_body(0, None)),
    AstNode::RegExpLiteral(AstNodeFactory::new_reg_exp_literal(0, name(), 0)),
    AstNode::ObjectLiteral(AstNodeFactory::new_object_literal(0, Vec::new())),
    AstNode::ArrayLiteral(AstNodeFactory::new_array_literal(0, Vec::new())),
    AstNode::Assignment(AstNodeFactory::new_assignment(
      0,
      AstAssignOp::Assign,
      vp(),
      lit(),
    )),
    AstNode::Await(AstNodeFactory::new_await(0, lit())),
    AstNode::BinaryOperation(AstNodeFactory::new_binary_operation(
      0,
      AstBinaryOp::Add,
      lit(),
      lit(),
    )),
    AstNode::NaryOperation(AstNodeFactory::new_nary_operation(
      0,
      AstBinaryOp::Add,
      lit(),
      Vec::new(),
    )),
    AstNode::Call(AstNodeFactory::new_call(
      0,
      vp(),
      Vec::new(),
      CallType::OrdinaryCall,
      SpreadPosition::NoSpread,
    )),
    AstNode::SuperCallForwardArgs(AstNodeFactory::new_super_call_forward_args(0, vp())),
    AstNode::CallNew(AstNodeFactory::new_call_new(
      0,
      lit(),
      Vec::new(),
      SpreadPosition::NoSpread,
    )),
    AstNode::CallRuntime(AstNodeFactory::new_call_runtime(0, name(), Vec::new())),
    AstNode::ClassLiteral(AstNodeFactory::new_class_literal(
      0,
      ClassLiteralOptions::default(),
    )),
    AstNode::CompareOperation(AstNodeFactory::new_compare_operation(
      0,
      AstCompareOp::Eq,
      lit(),
      lit(),
    )),
    AstNode::CompoundAssignment(AstNodeFactory::new_compound_assignment(
      0,
      AstCompoundAssignOp::AssignAdd,
      vp(),
      lit(),
      None,
    )),
    AstNode::ConditionalChain(AstNodeFactory::new_conditional_chain(0, Vec::new(), lit())),
    AstNode::Conditional(AstNodeFactory::new_conditional(0, lit(), lit(), lit())),
    AstNode::CountOperation(AstNodeFactory::new_count_operation(
      0,
      AstCountOp::Inc,
      lit(),
      true,
    )),
    AstNode::EmptyParentheses(AstNodeFactory::new_empty_parentheses(0)),
    {
      let mut factory = AstNodeFactory::new();
      AstNode::FunctionLiteral(factory.new_function_literal(0, FunctionLiteralOptions::default()))
    },
    AstNode::GetTemplateObject(AstNodeFactory::new_get_template_object(
      0,
      Vec::new(),
      Vec::new(),
    )),
    AstNode::ImportCallExpression(AstNodeFactory::new_import_call_expression(
      0,
      lit(),
      ModuleImportPhase::Evaluation,
      None,
    )),
    AstNode::Literal(AstNodeFactory::new_literal(0, AstValue::Smi(0))),
    AstNode::NativeFunctionLiteral(AstNodeFactory::new_native_function_literal(0, name())),
    AstNode::OptionalChain(AstNodeFactory::new_optional_chain(0, lit())),
    AstNode::Property(AstNodeFactory::new_property(0, vp(), lit(), false, false)),
    AstNode::Spread(AstNodeFactory::new_spread(0, lit())),
    AstNode::SuperCallReference(AstNodeFactory::new_super_call_reference(0, None, None)),
    AstNode::SuperPropertyReference(AstNodeFactory::new_super_property_reference(0, None)),
    AstNode::TemplateLiteral(AstNodeFactory::new_template_literal(
      0,
      Vec::new(),
      Vec::new(),
    )),
    AstNode::ThisExpression(AstNodeFactory::new_this_expression(0)),
    AstNode::Throw(AstNodeFactory::new_throw(0, lit())),
    AstNode::UnaryOperation(AstNodeFactory::new_unary_operation(
      0,
      AstUnaryOp::Not,
      lit(),
    )),
    AstNode::VariableProxy(AstNodeFactory::new_variable_proxy(
      1,
      AstRawString::from_utf8("x"),
      VariableProxyBindingMode::Bound,
      HoleCheckMode::Required,
    )),
    AstNode::Yield(AstNodeFactory::new_yield(
      0,
      None,
      OnAbruptResume::NoControl,
    )),
    AstNode::YieldStar(AstNodeFactory::new_yield_star(0, lit())),
    AstNode::FailureExpression(AstNodeFactory::new_failure_expression(0)),
  ]
}

/// 节点枚举完整性对数: 60 变体逐项与 AST_NODE_LIST 顺序 1:1 (判别值/派发/名称三方对齐)
#[test]
fn test_node_type_dispatch_roundtrip_all_60() -> Void {
  let samples = all_node_samples();
  assert_eq!(samples.len(), AstNodeType::COUNT, "样本数与枚举总数一致");

  // 原版 AST_NODE_LIST + FAILURE_NODE_LIST 展开顺序 (手工对数基准, 与上游 1:1)
  const EXPECTED_ORDER: [&str; 60] = [
    "VariableDeclaration",
    "FunctionDeclaration",
    "DoWhileStatement",
    "WhileStatement",
    "ForStatement",
    "ForInStatement",
    "ForOfStatement",
    "Block",
    "SwitchStatement",
    "ExpressionStatement",
    "EmptyStatement",
    "SloppyBlockFunctionStatement",
    "IfStatement",
    "ContinueStatement",
    "BreakStatement",
    "ReturnStatement",
    "WithStatement",
    "TryCatchStatement",
    "TryFinallyStatement",
    "DebuggerStatement",
    "InitializeClassMembersStatement",
    "InitializeClassStaticElementsStatement",
    "AutoAccessorGetterBody",
    "AutoAccessorSetterBody",
    "RegExpLiteral",
    "ObjectLiteral",
    "ArrayLiteral",
    "Assignment",
    "Await",
    "BinaryOperation",
    "NaryOperation",
    "Call",
    "SuperCallForwardArgs",
    "CallNew",
    "CallRuntime",
    "ClassLiteral",
    "CompareOperation",
    "CompoundAssignment",
    "ConditionalChain",
    "Conditional",
    "CountOperation",
    "EmptyParentheses",
    "FunctionLiteral",
    "GetTemplateObject",
    "ImportCallExpression",
    "Literal",
    "NativeFunctionLiteral",
    "OptionalChain",
    "Property",
    "Spread",
    "SuperCallReference",
    "SuperPropertyReference",
    "TemplateLiteral",
    "ThisExpression",
    "Throw",
    "UnaryOperation",
    "VariableProxy",
    "Yield",
    "YieldStar",
    "FailureExpression",
  ];

  for (idx, node) in samples.iter().enumerate() {
    // 1. 派发返回的节点类型判别值 == 索引 (enum_dispatch → AstNodeTrait → AstNodeBase 三层一致)
    let t = node.node_type();
    assert_eq!(t as u8 as usize, idx, "样本 {idx} 派发类型判别值错位");
    // 2. FromRepr 反查一致
    assert_eq!(AstNodeType::from_repr(idx as u8), Some(t));
    // 3. 名称与上游展开顺序逐项对齐
    assert_eq!(
      t.to_string(),
      EXPECTED_ORDER[idx],
      "样本 {idx} 名称与上游顺序不一致"
    );
    // 4. 位置与基类
    assert_eq!(node.position(), node.base().position);
  }

  // 类别计数对齐 (上游 DECLARATION=2 / STATEMENT=22 / EXPRESSION=35 / 失败=1)
  let decls = samples
    .iter()
    .filter(|n| n.node_type().is_declaration())
    .count();
  let stmts = samples
    .iter()
    .filter(|n| n.node_type().is_statement())
    .count();
  let exprs = samples
    .iter()
    .filter(|n| n.node_type().is_expression())
    .count();
  assert_eq!(decls, AstNodeType::DECLARATION_COUNT);
  assert_eq!(stmts, AstNodeType::STATEMENT_COUNT);
  assert_eq!(exprs, AstNodeType::EXPRESSION_COUNT);
  assert_eq!(
    decls + stmts + exprs + 1,
    AstNodeType::COUNT,
    "仅 FailureExpression 无类别"
  );

  info!("test_node_type_dispatch_roundtrip_all_60 passed");
  OK
}

/// Visitor 全覆盖: 对 60 个变体逐一实例化并访问, walk_node 分发臂 60/60 全命中
#[test]
fn test_visitor_walk_dispatch_full_coverage() -> Void {
  #[derive(Default)]
  struct AllArmsVisitor {
    dispatched: Vec<AstNodeType>,
  }

  impl AstVisitor for AllArmsVisitor {
    fn enter_node(&mut self, node: &AstNode) -> bool {
      self.dispatched.push(node.node_type());
      true
    }
  }

  let samples = all_node_samples();
  let mut visitor = AllArmsVisitor::default();
  for node in &samples {
    visitor.visit(node);
  }

  // 60 个分发臂全部执行: 每个类型都经 visit 进入过 (含子节点递归到达)
  for expected in 0..AstNodeType::COUNT {
    let t = AstNodeType::from_repr(expected as u8).unwrap_or(AstNodeType::FailureExpression);
    assert!(
      visitor.dispatched.contains(&t),
      "walk_node 分发臂 {expected} 未命中"
    );
  }
  let distinct: HashSet<_> = visitor.dispatched.iter().collect();
  assert_eq!(distinct.len(), AstNodeType::COUNT, "应访问到全部 60 类节点");

  info!("test_visitor_walk_dispatch_full_coverage passed");
  OK
}

/// FunctionLiteral id 分配测试 (原版解析器 id 计数器语义)
#[test]
fn test_function_literal_id_allocation() -> Void {
  assert_eq!(FUNCTION_LITERAL_ID_TOP_LEVEL, 0);
  assert_eq!(FUNCTION_LITERAL_ID_UNASSIGNED, -1);

  let mut factory = AstNodeFactory::new();
  let f1 = factory.new_function_literal(0, FunctionLiteralOptions::default());
  let f2 = factory.new_function_literal(
    0,
    FunctionLiteralOptions {
      is_async: true,
      is_generator: true,
      ..FunctionLiteralOptions::default()
    },
  );
  assert_eq!(f1.function_literal_id, 1, "顶层 id 0 保留, 首个分配为 1");
  assert_eq!(f2.function_literal_id, 2);
  assert!(!f1.is_toplevel());

  info!("test_function_literal_id_allocation passed");
  OK
}

/// 第二阶段节点字段补全对数: FunctionLiteral/ClassLiteral/VariableProxy/Assignment 补全字段行为
#[test]
fn test_node_field_completion() -> Void {
  // FunctionLiteral 补全字段 (对照上游 FunctionLiteral 构造参数全集)
  let mut factory = AstNodeFactory::new();
  let mut preparse = PreparseData::new();
  preparse.byte_data = vec![1, 2, 3];
  let fl = factory.new_function_literal(
    10,
    FunctionLiteralOptions {
      name: Some(AstRawString::from_utf8("myFunc")),
      syntax_kind: FunctionSyntaxKind::NamedExpression,
      parameter_count: 2,
      function_length: 1,
      expected_property_count: 3,
      has_duplicate_parameters: ParameterFlag::HasDuplicateParameters,
      has_braces: false,
      preparse_data: Some(Box::new(preparse)),
      ..FunctionLiteralOptions::default()
    },
  );
  assert_eq!(fl.syntax_kind, FunctionSyntaxKind::NamedExpression);
  assert!(!fl.is_anonymous_expression());
  assert_eq!(fl.parameter_count, 2);
  assert_eq!(fl.function_length, 1);
  assert_eq!(fl.expected_property_count, 3);
  assert!(fl.has_duplicate_parameters);
  assert!(!fl.has_braces);
  assert_eq!(fl.function_token_position, NO_SOURCE_POSITION);
  assert!(!fl.can_suspend(), "初始 suspend_count 为 0");
  assert_eq!(
    fl.preparse_data.as_ref().unwrap().byte_data,
    vec![1, 2, 3],
    "produced_preparse_data 透传"
  );
  assert!(fl.has_shared_name());

  // 语法类别枚举对数 (上游 5 值)
  assert_eq!(FunctionSyntaxKind::COUNT, 5);
  assert!(FunctionSyntaxKind::from_repr(0) == Some(FunctionSyntaxKind::AnonymousExpression));
  assert!(FunctionSyntaxKind::from_repr(4) == Some(FunctionSyntaxKind::Wrapped));
  assert!(FunctionSyntaxKind::AnonymousExpression.is_anonymous_expression());
  assert!(!FunctionSyntaxKind::Declaration.is_anonymous_expression());

  // CatchPrediction 对数 (上游 HandlerTable 5 值)
  assert_eq!(
    CatchPrediction::from_repr(4),
    Some(CatchPrediction::UncaughtAsyncAwait)
  );

  // AstAssignOp 对数: 16 值, 纯 '=' + 复合 15
  assert_eq!(AstAssignOp::COUNT, 16);
  assert_eq!(AstAssignOp::from_repr(0), Some(AstAssignOp::Assign));
  assert_eq!(AstAssignOp::Assign.as_str(), "=");
  assert_eq!(AstAssignOp::Assign.binary_op(), None);
  assert_eq!(AstAssignOp::AssignAdd.binary_op(), Some(AstBinaryOp::Add));
  assert_eq!(
    AstAssignOp::from_compound(AstCompoundAssignOp::AssignMul),
    AstAssignOp::AssignMul
  );

  // ClassLiteral 完整成员
  let ctor = factory.new_function_literal(5, FunctionLiteralOptions::default());
  let sinit = factory.new_function_literal(6, FunctionLiteralOptions::default());
  let imem = factory.new_function_literal(7, FunctionLiteralOptions::default());
  let cls = AstNodeFactory::new_class_literal(
    0,
    ClassLiteralOptions {
      name: Some(AstRawString::from_utf8("C")),
      constructor: Some(ctor),
      static_initializer: Some(sinit),
      instance_members_initializer: Some(imem),
      end_position: 42,
      has_static_computed_names: true,
      is_anonymous: true,
      ..ClassLiteralOptions::default()
    },
  );
  assert!(cls.constructor.is_some());
  assert!(cls.static_initializer.is_some());
  assert!(cls.instance_members_initializer.is_some());
  assert_eq!(cls.end_position, 42);
  assert!(cls.has_static_computed_names);
  assert!(cls.is_anonymous_expression());
  assert!(cls.public_members.is_empty() && cls.private_members.is_empty());
  assert!(cls.scope.is_none() && cls.home_object.is_none() && cls.static_home_object.is_none());

  // VariableProxy 消解字段 (原版 IsAssigned/IsResolved/var_)
  let mut proxy = AstNodeFactory::new_variable_proxy(
    3,
    AstRawString::from_utf8("p"),
    VariableProxyBindingMode::Unbound,
    HoleCheckMode::Required,
  );
  assert!(!proxy.is_resolved && !proxy.is_assigned && proxy.var.is_none());
  proxy.bind_to(VariableId::from_raw(7));
  assert!(proxy.is_resolved);
  assert_eq!(proxy.var, Some(VariableId::from_raw(7)));

  // Assignment 补全: op + lookup_hoisting_mode
  let a = AstNodeFactory::new_assignment(
    0,
    AstAssignOp::AssignNullish,
    AstNode::ThisExpression(AstNodeFactory::new_this_expression(0)),
    AstNode::Literal(AstNodeFactory::new_literal(1, AstValue::Smi(1))),
  );
  assert_eq!(a.op, AstAssignOp::AssignNullish);

  info!("test_node_field_completion passed");
  OK
}

/// Zone 分配路径: 节点驻留 + ZoneList/ZoneVector 物化 + 无后端错误
#[test]
fn test_zone_allocation_path() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "ast_zone_test");
  let factory = AstNodeFactory::with_zone(&zone);

  // 1. 节点驻留 (原版 NewAstNode): 值在 zone 内存活且可变
  let lit = AstNodeFactory::new_literal(7, AstValue::Smi(42));
  let zlit = factory.alloc_node(lit).expect("zone 分配 8 字节对齐内节点");
  assert!(zlit.value.to_boolean_is_true());
  assert_eq!(zlit.base.position, 7);
  zlit.base.position = 8;
  assert_eq!(zlit.base.position, 8);

  // 2. ZoneList 物化 (上游 ZonePtrList 语句流形态)
  let mut list = factory.new_zone_list::<i32>(2).expect("zone list 分配");
  assert!(list.is_empty());
  for i in 0..5 {
    list.add(&i).expect("zone list add");
  }
  assert_eq!(list.length(), 5);
  assert_eq!(*list.at(4), 4);

  // 3. ZoneVector 物化 (移动 push_back 语句流)
  let mut vec = factory.zone_vec::<u8>().expect("zone vec 分配");
  for b in 0..100u8 {
    vec.push_back(b).expect("zone vec push");
  }
  assert_eq!(vec.size(), 100);
  assert_eq!(*vec.back().unwrap(), 99);

  // 4. 切片物化 (参数名集合物化, 原版 ZoneVector(vector, zone))
  let params = [AstRawString::from_utf8("a"), AstRawString::from_utf8("b")];
  let pv = factory
    .zone_vec_from_slice(&params)
    .expect("zone vec from slice");
  assert_eq!(pv.size(), 2);
  assert_eq!(pv.at(1).to_string_lossy(), "b");

  // 5. 辅助结构驻留 (ClassLiteralProperty 等非节点 zone 对象)
  let prop = ClassLiteralProperty::new(
    ClassPropertyKind::Field,
    AstNode::Literal(AstNodeFactory::new_literal(1, AstValue::Smi(0))),
    AstNode::Literal(AstNodeFactory::new_literal(2, AstValue::Smi(0))),
    false,
    false,
    false,
  );
  let zprop = factory.alloc_node(prop).expect("辅助结构驻留");
  assert_eq!(zprop.kind, ClassPropertyKind::Field);

  // 6. 无 zone 后端工厂调用分配路径 → NoZoneBackend
  let plain = AstNodeFactory::new();
  assert!(matches!(plain.zone_vec::<u8>(), Err(Error::NoZoneBackend)));

  // 7. zone 记账: 已分配内存 > 0
  assert!(zone.allocation_size() > 0);

  info!("test_zone_allocation_path passed");
  OK
}

/// 作用域声明与沿链查找 (原版 DeclareLocal/DeclareVariableName/Lookup)
#[test]
fn test_scopes_declaration_and_lookup() -> Void {
  let mut tree = ScopeTree::new();
  let script = tree.new_scope(ScopeType::ScriptScope, None);
  assert!(tree.is_script_scope(script) && tree.is_declaration_scope(script));
  assert_eq!(tree.len(), 1);

  let func = tree.new_scope(ScopeType::FunctionScope, Some(script));
  let block = tree.new_scope(ScopeType::BlockScope, Some(func));
  assert_eq!(tree.inner_scopes(func), &[block]);
  assert_eq!(tree.outer_scope(block), Some(func));
  assert!(tree.is_outer_scope_of(script, block));

  // 词法声明: let x (函数作用域)
  let x = tree
    .declare_variable_name(
      func,
      AstRawString::from_utf8("x"),
      VariableMode::Let,
      VariableKind::NormalVariable,
    )
    .expect("首次 let x 声明");
  assert_eq!(tree.variable(x).mode(), VariableMode::Let);
  assert_eq!(tree.variable(x).scope(), func);

  // 重复 let 声明 → Redeclaration
  let dup = tree.declare_variable_name(
    func,
    AstRawString::from_utf8("x"),
    VariableMode::Let,
    VariableKind::NormalVariable,
  );
  assert!(matches!(dup.unwrap_err(), Error::Redeclaration { .. }));

  // let 后 var → 词法冲突同样报错 (上游同判)
  let var_after_let = tree.declare_variable_name(
    func,
    AstRawString::from_utf8("x"),
    VariableMode::Var,
    VariableKind::NormalVariable,
  );
  assert!(matches!(
    var_after_let.unwrap_err(),
    Error::Redeclaration { .. }
  ));

  // sloppy 块函数豁免: 已有 + 新增均为 SLOPPY_BLOCK_FUNCTION_VARIABLE → OK
  let sb = AstRawString::from_utf8("sbf");
  tree
    .declare_variable_name(
      func,
      sb.clone(),
      VariableMode::Let,
      VariableKind::SloppyBlockFunctionVariable,
    )
    .expect("首次 sloppy 块函数声明");
  tree
    .declare_variable_name(
      func,
      sb,
      VariableMode::Var,
      VariableKind::SloppyBlockFunctionVariable,
    )
    .expect("sloppy 块函数重声明豁免 (web 兼容)");

  // var 重声明 OK: var x 与 let 冲突? let x 后 var x → 词法冲突, 上游同判 Err
  // var y 声明在块作用域 → 提升至函数作用域
  let y = tree
    .declare_variable_name(
      block,
      AstRawString::from_utf8("y"),
      VariableMode::Var,
      VariableKind::NormalVariable,
    )
    .expect("var y 块内声明");
  assert_eq!(tree.variable(y).scope(), func, "var 提升至声明作用域");

  // 沿链查找: 块内查 x 命中函数作用域
  let found = tree.lookup_local(block, &AstRawString::from_utf8("x"));
  assert_eq!(found, None, "块内无本地 x");
  let tree_mut = &mut tree;
  let found = tree_mut.lookup(block, &AstRawString::from_utf8("x"));
  assert_eq!(found, Some(x));

  // var y 在块内也能查到 (提升后)
  let found = tree_mut.lookup(block, &AstRawString::from_utf8("y"));
  assert_eq!(found, Some(y));

  // script 根未命中 → 动态全局回退 (DeclareDynamicGlobal)
  let g = tree_mut.lookup(block, &AstRawString::from_utf8("globalVar"));
  let g = g.expect("script 根回退");
  assert_eq!(tree.variable(g).mode(), VariableMode::DynamicGlobal);
  assert_eq!(tree.variable(g).location(), VariableLocation::Unallocated);

  // context 链长: 块 → script (无 context 需求时为 0)
  assert_eq!(tree.context_chain_length(block, script), 0);

  // catch 绑定变量
  let catch = tree.new_scope(ScopeType::CatchScope, Some(func));
  let e = tree.declare_catch_variable_name(catch, AstRawString::from_utf8("e"));
  assert_eq!(tree.catch_variable(catch), Some(e));
  assert!(tree.is_catch_scope(catch));
  assert_eq!(tree.variable(e).mode(), VariableMode::Var);

  // 形参 + receiver 声明
  let p = tree.declare_parameter(
    func,
    AstRawString::from_utf8("a"),
    VariableMode::Var,
    VariableKind::ParameterVariable,
  );
  assert_eq!(tree.params(func), &[p]);

  // DeclarationScope 专属声明: 函数名 / 生成器对象 / new.target / 临时变量
  let fname = tree
    .declare_function_var(func, AstRawString::from_utf8("f"))
    .expect("函数名变量声明");
  assert_eq!(tree.variable(fname).mode(), VariableMode::Var);
  let gen_obj =
    tree.declare_generator_object_var(func, AstRawString::from_utf8(".generator_object"));
  assert_eq!(tree.variable(gen_obj).mode(), VariableMode::Var);
  let nt = tree.declare_new_target_var(func, AstRawString::from_utf8(".new.target"));
  assert_eq!(tree.variable(nt).mode(), VariableMode::Const);
  assert_eq!(tree.new_target_var(func), Some(nt));
  let tmp = tree.new_temporary(func, AstRawString::from_utf8(".tmp"));
  assert_eq!(tree.variable(tmp).mode(), VariableMode::Temporary);

  // 声明作用域/闭包作用域判定
  assert!(tree.is_closure_scope(func));
  assert!(!tree.is_closure_scope(block));
  assert_eq!(tree.get_declaration_scope(block), func);

  info!("test_scopes_declaration_and_lookup passed");
  OK
}

/// 动态查找回退: with 降级 / sloppy eval 降级 (原版 LookupWith/LookupSloppyEval)
#[test]
fn test_scopes_dynamic_lookup_fallbacks() -> Void {
  // 1. with 作用域: 外层命中后转 LOOKUP 动态
  let mut tree = ScopeTree::new();
  let script = tree.new_scope(ScopeType::ScriptScope, None);
  let outer = tree.new_scope(ScopeType::FunctionScope, Some(script));
  let with = tree.new_scope(ScopeType::WithScope, Some(outer));
  tree.declare_local(
    outer,
    AstRawString::from_utf8("w"),
    VariableMode::Var,
    VariableKind::NormalVariable,
    InitializationFlag::CreatedInitialized,
  );
  assert!(tree.has_context_extension_slot(with));
  assert_eq!(tree.context_header_length(with), MIN_CONTEXT_EXTENDED_SLOTS);

  let found = tree.lookup(with, &AstRawString::from_utf8("w"));
  let dynamic = found.expect("with 内命中");
  let var = tree.variable(dynamic);
  assert_eq!(var.mode(), VariableMode::Dynamic);
  assert_eq!(var.location(), VariableLocation::Lookup);
  assert_eq!(var.index(), -1);

  // 2. sloppy eval 声明作用域: 全局对象属性降级 DynamicGlobal
  let mut tree2 = ScopeTree::new();
  let script2 = tree2.new_scope(ScopeType::ScriptScope, None);
  let caller = tree2.new_scope(ScopeType::FunctionScope, Some(script2));
  tree2.set_sloppy_eval_can_extend_vars(caller, true);
  // script 里 var g (全局对象属性)
  let (g, _) = tree2.declare_local(
    script2,
    AstRawString::from_utf8("g"),
    VariableMode::Var,
    VariableKind::NormalVariable,
    InitializationFlag::CreatedInitialized,
  );
  let found = tree2.lookup(caller, &AstRawString::from_utf8("g"));
  let downgraded = found.expect("sloppy eval 降级");
  assert_eq!(
    tree2.variable(downgraded).mode(),
    VariableMode::DynamicGlobal,
    "全局对象属性经 sloppy eval 作用域降级"
  );
  assert_ne!(downgraded, g, "降级产生新动态变量");

  // 3. eval 调用向上传播标记
  let mut tree3 = ScopeTree::new();
  let s = tree3.new_scope(ScopeType::ScriptScope, None);
  let f = tree3.new_scope(ScopeType::FunctionScope, Some(s));
  let inner = tree3.new_scope(ScopeType::BlockScope, Some(f));
  tree3.record_inner_scope_eval_call(inner);
  assert!(tree3.inner_scope_calls_eval(inner));
  assert!(tree3.inner_scope_calls_eval(f), "eval 标记向上传播");
  assert!(tree3.inner_scope_calls_eval(s));

  info!("test_scopes_dynamic_lookup_fallbacks passed");
  OK
}

/// 变量分配: 栈槽 / context 槽 / 形参 / catch 强制 context (原版 Allocate 族)
#[test]
fn test_scopes_variable_allocation() -> Void {
  // 1. 函数作用域: 形参 + 已用 var 本地 → 栈槽
  let mut tree = ScopeTree::new();
  let script = tree.new_scope(ScopeType::ScriptScope, None);
  let func = tree.new_scope(ScopeType::FunctionScope, Some(script));
  let p0 = tree.declare_parameter(
    func,
    AstRawString::from_utf8("p0"),
    VariableMode::Var,
    VariableKind::ParameterVariable,
  );
  let p1 = tree.declare_parameter(
    func,
    AstRawString::from_utf8("p1"),
    VariableMode::Var,
    VariableKind::ParameterVariable,
  );
  let (v, _) = tree.declare_local(
    func,
    AstRawString::from_utf8("used_local"),
    VariableMode::Var,
    VariableKind::NormalVariable,
    InitializationFlag::CreatedInitialized,
  );
  let (unused, _) = tree.declare_local(
    func,
    AstRawString::from_utf8("unused_local"),
    VariableMode::Var,
    VariableKind::NormalVariable,
    InitializationFlag::CreatedInitialized,
  );
  tree.variable_mut(v).set_is_used();
  // script 作用域: 词法 let (分配阶段 → context 槽)
  let lx = tree
    .declare_variable_name(
      script,
      AstRawString::from_utf8("lex"),
      VariableMode::Let,
      VariableKind::NormalVariable,
    )
    .expect("script 词法声明");
  tree.allocate_variables_recursively(script);

  // receiver: this 声明并占 Parameter(-1)
  let vars: Vec<VariableId> = tree.variables(func).map(|(_, id)| id).collect();
  let this_var = vars
    .iter()
    .copied()
    .find(|id| tree.variable(*id).name().to_string_lossy() == "this")
    .expect("receiver 已声明");
  assert_eq!(
    tree.variable(this_var).location(),
    VariableLocation::Parameter
  );
  assert_eq!(tree.variable(this_var).index(), -1);

  assert_eq!(tree.variable(p0).location(), VariableLocation::Parameter);
  assert_eq!(tree.variable(p0).index(), 0);
  assert_eq!(tree.variable(p1).index(), 1);
  assert_eq!(tree.variable(v).location(), VariableLocation::Local);
  assert_eq!(
    tree.variable(unused).location(),
    VariableLocation::Unallocated,
    "未用变量不分配"
  );
  assert_eq!(tree.num_stack_slots(func), 1);
  assert_eq!(tree.num_heap_slots(func), 0, "函数作用域无 context 槽");
  assert!(!tree.needs_context(func));

  let lv = tree.variable(lx);
  assert!(lv.is_used(), "declare_variable_name 标记 is_used");
  assert_eq!(lv.location(), VariableLocation::Context);
  assert_eq!(lv.index(), MIN_CONTEXT_SLOTS);
  assert_eq!(tree.num_heap_slots(script), MIN_CONTEXT_SLOTS + 1);
  assert!(tree.needs_context(script));
  // 函数到 script 的 context 链长: [func, script) 半开区间, func 无 context → 0
  // (script 自身的 context 属于外层访问者)
  assert_eq!(tree.context_chain_length(func, script), 0);
  // 从 block (func 子作用域) 出发同样为 0
  let block_probe = tree.new_scope(ScopeType::BlockScope, Some(func));
  assert_eq!(tree.context_chain_length(block_probe, func), 0);
  // 最近持 context 的外层: func 无 context → 命中 script
  assert_eq!(tree.get_outer_scope_with_context(block_probe), Some(script));

  // 3. catch 作用域变量强制 context, 槽位 = THROWN_OBJECT_INDEX
  let mut tree2 = ScopeTree::new();
  let s2 = tree2.new_scope(ScopeType::ScriptScope, None);
  let f2 = tree2.new_scope(ScopeType::FunctionScope, Some(s2));
  let c2 = tree2.new_scope(ScopeType::CatchScope, Some(f2));
  let e2 = tree2.declare_catch_variable_name(c2, AstRawString::from_utf8("e"));
  tree2.allocate_variables_recursively(s2);
  let ev = tree2.variable(e2);
  assert_eq!(ev.location(), VariableLocation::Context);
  assert_eq!(ev.index(), THROWN_OBJECT_INDEX);
  assert_eq!(tree2.num_heap_slots(c2), MIN_CONTEXT_SLOTS + 1);

  // 4. eval 内层调用 → 本地强制 context (MustAllocateInContext: inner_scope_calls_eval)
  let mut tree3 = ScopeTree::new();
  let s3 = tree3.new_scope(ScopeType::ScriptScope, None);
  let f3 = tree3.new_scope(ScopeType::FunctionScope, Some(s3));
  let (k, _) = tree3.declare_local(
    f3,
    AstRawString::from_utf8("k"),
    VariableMode::Var,
    VariableKind::NormalVariable,
    InitializationFlag::CreatedInitialized,
  );
  tree3.variable_mut(k).set_is_used();
  tree3.record_inner_scope_eval_call(f3);
  tree3.allocate_variables_recursively(s3);
  assert_eq!(
    tree3.variable(k).location(),
    VariableLocation::Context,
    "eval 可达的本地变量强制 context 分配"
  );

  info!("test_scopes_variable_allocation passed");
  OK
}

/// 未消解引用与递归消解 (原版 unresolved_list_ + ResolveVariablesRecursively)
#[test]
fn test_scopes_resolve_variables() -> Void {
  let mut tree = ScopeTree::new();
  let script = tree.new_scope(ScopeType::ScriptScope, None);
  let func = tree.new_scope(ScopeType::FunctionScope, Some(script));

  // 函数内声明 var src; 块内引用 dst (读) 与 src (写)
  let src = tree
    .declare_variable_name(
      func,
      AstRawString::from_utf8("src"),
      VariableMode::Var,
      VariableKind::NormalVariable,
    )
    .expect("src 声明");
  let block = tree.new_scope(ScopeType::BlockScope, Some(func));
  tree.add_unresolved(block, AstRawString::from_utf8("src"), 10, true);
  tree.add_unresolved(block, AstRawString::from_utf8("dst"), 20, false);
  assert_eq!(tree.unresolved(block).len(), 2);

  // script 里声明 var dst (跨闭包引用)
  let dst = tree
    .declare_variable_name(
      script,
      AstRawString::from_utf8("dst"),
      VariableMode::Var,
      VariableKind::NormalVariable,
    )
    .expect("dst 声明");

  tree.resolve_variables_recursively(script);

  // 消解后: src 标记 is_used + maybe_assigned
  let sv = tree.variable(src);
  assert!(sv.is_used());
  assert_eq!(sv.maybe_assigned_flag(), MaybeAssignedFlag::MaybeAssigned);
  // dst (script var): is_used, 且因内层函数引用 → 强制 context? 上游 ResolveVariable 不强制,
  // is_used 即标记; 分配阶段 script var 为全局属性不分配
  assert!(tree.variable(dst).is_used());
  // 未消解列表清空
  assert_eq!(tree.unresolved(block).len(), 0);

  // delete_unresolved: 再登记后按位置删除
  tree.add_unresolved(block, AstRawString::from_utf8("tmp"), 30, false);
  assert!(tree.delete_unresolved(block, 30));
  assert!(!tree.delete_unresolved(block, 99));

  // 变量代理绑定回写路径 (节点 ↔ 树)
  let mut proxy = AstNodeFactory::new_variable_proxy(
    10,
    AstRawString::from_utf8("src"),
    VariableProxyBindingMode::Unbound,
    HoleCheckMode::Required,
  );
  let resolved = tree
    .resolve_variable(block, &AstRawString::from_utf8("src"), true)
    .expect("块内消解 src");
  proxy.bind_to(resolved);
  assert_eq!(proxy.var, Some(resolved));
  assert!(proxy.is_resolved);

  info!("test_scopes_resolve_variables passed");
  OK
}

#[test]
fn test_visitors_generated_traversal_printer_and_reindexer() -> Void {
  use v8_ast::visitors_generated::*;

  // 1. ast_printer_skel 验证
  assert_eq!(unsafe { ast_printer_skel::print(0) }, "");

  let empty_stmt = AstNode::from(AstNodeFactory::new_empty_statement(42));
  let printed = print_ast(&empty_stmt);
  assert!(printed.contains("Node("));

  let block_stmt = AstNode::from(AstNodeFactory::new_block(
    10,
    vec![empty_stmt.clone()],
    false,
  ));
  let printed_block = print_ast(&block_stmt);
  assert!(printed_block.contains("{\n"));
  assert!(printed_block.contains("}\n"));

  let ptr = &block_stmt as *const AstNode as usize;
  let printed_via_skel = unsafe { ast_printer_skel::print(ptr) };
  assert_eq!(printed_via_skel, printed_block);

  // 2. ast_visitor_skel & AstTraversalVisitor 验证
  unsafe {
    ast_visitor_skel::traverse(0); // 空指针不崩
    ast_visitor_skel::traverse(ptr);
  }

  let mut traverser = DefaultAstTraversalVisitor::default();
  traverser.visit(&block_stmt);
  assert!(traverser.visited_count >= 2);

  let stmts = vec![empty_stmt.clone(), block_stmt.clone()];
  traverser.visit_statements(&stmts);
  traverser.visit_expressions(&stmts);
  traverser.visit_declarations(&stmts);

  // 3. 函数字面量重排器验证
  ast_visitor_skel::reindex_function_literal_ids(50, 10);
  let reindexer = FunctionLiteralIdReindexer::new(40);
  assert_eq!(reindexer.reindex_id(10), 50);

  // 4. 源码范围收集器验证
  ast_visitor_skel::collect_source_ranges();
  let mut collector = SourceRangeCollector::new();
  collector.record_range(SourceRangeKind::Body, 0, 10);
  collector.record_range(SourceRangeKind::Continuation, 15, 20);
  collector.record_range(SourceRangeKind::Continuation, 15, 25); // 重复 pos 忽略
  assert_eq!(collector.ranges.len(), 2);

  collector.maybe_remove_continuation_range(15);
  assert_eq!(collector.ranges.len(), 1);
  assert_eq!(collector.ranges[0].0, SourceRangeKind::Body);

  info!("test_visitors_generated_traversal_printer_and_reindexer passed");
  OK
}
