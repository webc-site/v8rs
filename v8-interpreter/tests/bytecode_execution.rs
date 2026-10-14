use aok::{OK, Void};
use log::info;
use v8_ast::{
  FunctionLiteralOptions,
  ast::{
    AstAssignOp, AstBinaryOp, AstCompareOp, AstNode, AstNodeFactory, AstUnaryOp, HoleCheckMode,
    ReturnType, VariableProxyBindingMode,
  },
  value::{AstRawString, AstValue},
};
use v8_interpreter::{BytecodeDispatcher, BytecodeGenerator, Value, execute_bytecode};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

fn var_proxy(name: &str) -> AstNode {
  AstNode::VariableProxy(AstNodeFactory::new_variable_proxy(
    0,
    AstRawString::from_utf8(name),
    VariableProxyBindingMode::Bound,
    HoleCheckMode::Elided,
  ))
}

fn smi(val: i32) -> AstNode {
  AstNode::Literal(AstNodeFactory::new_literal(0, AstValue::Smi(val)))
}

fn assign(name: &str, val: AstNode) -> AstNode {
  AstNode::Assignment(AstNodeFactory::new_assignment(
    0,
    AstAssignOp::Assign,
    var_proxy(name),
    val,
  ))
}

fn add(left: AstNode, right: AstNode) -> AstNode {
  AstNode::BinaryOperation(AstNodeFactory::new_binary_operation(
    0,
    AstBinaryOp::Add,
    left,
    right,
  ))
}

fn sub(left: AstNode, right: AstNode) -> AstNode {
  AstNode::BinaryOperation(AstNodeFactory::new_binary_operation(
    0,
    AstBinaryOp::Sub,
    left,
    right,
  ))
}

fn mul(left: AstNode, right: AstNode) -> AstNode {
  AstNode::BinaryOperation(AstNodeFactory::new_binary_operation(
    0,
    AstBinaryOp::Mul,
    left,
    right,
  ))
}

fn ret(val: AstNode) -> AstNode {
  AstNode::ReturnStatement(AstNodeFactory::new_return_statement(
    0,
    Some(val),
    ReturnType::Normal,
  ))
}

fn block(stmts: Vec<AstNode>) -> AstNode {
  AstNode::Block(AstNodeFactory::new_block(0, stmts, false))
}

#[test]
fn test_while_loop_execution_42() -> Void {
  info!("Testing AST while-loop execution producing exactly 42");

  // AST:
  // x = 0;
  // i = 0;
  // while (i < 6) {
  //   x = x + 7;
  //   i = i + 1;
  // }
  // return x;
  let ast = block(vec![
    assign("x", smi(0)),
    assign("i", smi(0)),
    AstNode::WhileStatement(AstNodeFactory::new_while_statement(
      0,
      AstNode::CompareOperation(AstNodeFactory::new_compare_operation(
        0,
        AstCompareOp::LessThan,
        var_proxy("i"),
        smi(6),
      )),
      block(vec![
        assign("x", add(var_proxy("x"), smi(7))),
        assign("i", add(var_proxy("i"), smi(1))),
      ]),
    )),
    ret(var_proxy("x")),
  ]);

  let bytecode = BytecodeGenerator::generate(&ast)?;
  let result = execute_bytecode(&bytecode)?;
  assert_eq!(result, Value::Smi(42));

  OK
}

#[test]
fn test_for_loop_execution_42() -> Void {
  info!("Testing AST for-loop execution producing exactly 42");

  // AST:
  // sum = 6;
  // for (i = 0; i < 9; i = i + 1) {
  //   sum = sum + 4;
  // }
  // return sum; // 6 + 9 * 4 = 42
  let ast = block(vec![
    assign("sum", smi(6)),
    AstNode::ForStatement(AstNodeFactory::new_for_statement(
      0,
      Some(assign("i", smi(0))),
      Some(AstNode::CompareOperation(
        AstNodeFactory::new_compare_operation(0, AstCompareOp::LessThan, var_proxy("i"), smi(9)),
      )),
      Some(assign("i", add(var_proxy("i"), smi(1)))),
      block(vec![assign("sum", add(var_proxy("sum"), smi(4)))]),
    )),
    ret(var_proxy("sum")),
  ]);

  let bytecode = BytecodeGenerator::generate(&ast)?;
  let result = execute_bytecode(&bytecode)?;
  assert_eq!(result, Value::Smi(42));

  OK
}

#[test]
fn test_arithmetic_precedence_and_evaluation() -> Void {
  info!("Testing AST arithmetic operations producing 42");

  // AST: return (10 + 20) * 2 - 18 = 42
  let expr = sub(mul(add(smi(10), smi(20)), smi(2)), smi(18));
  let ast = ret(expr);

  let bytecode = BytecodeGenerator::generate(&ast)?;
  let result = execute_bytecode(&bytecode)?;
  assert_eq!(result, Value::Smi(42));

  OK
}

#[test]
fn test_if_else_control_flow() -> Void {
  info!("Testing AST if-else control flow");

  // AST:
  // cond = 1;
  // if (cond == 1) {
  //   res = 42;
  // } else {
  //   res = 0;
  // }
  // return res;
  let ast = block(vec![
    assign("cond", smi(1)),
    AstNode::IfStatement(AstNodeFactory::new_if_statement(
      0,
      AstNode::CompareOperation(AstNodeFactory::new_compare_operation(
        0,
        AstCompareOp::Eq,
        var_proxy("cond"),
        smi(1),
      )),
      block(vec![assign("res", smi(42))]),
      Some(block(vec![assign("res", smi(0))])),
    )),
    ret(var_proxy("res")),
  ]);

  let bytecode = BytecodeGenerator::generate(&ast)?;
  let result = execute_bytecode(&bytecode)?;
  assert_eq!(result, Value::Smi(42));

  OK
}

#[test]
fn test_break_in_loop() -> Void {
  info!("Testing break inside while-loop");

  // AST:
  // i = 0;
  // while (i < 100) {
  //   i = i + 1;
  //   if (i == 42) {
  //     break;
  //   }
  // }
  // return i;
  let ast = block(vec![
    assign("i", smi(0)),
    AstNode::WhileStatement(AstNodeFactory::new_while_statement(
      0,
      AstNode::CompareOperation(AstNodeFactory::new_compare_operation(
        0,
        AstCompareOp::LessThan,
        var_proxy("i"),
        smi(100),
      )),
      block(vec![
        assign("i", add(var_proxy("i"), smi(1))),
        AstNode::IfStatement(AstNodeFactory::new_if_statement(
          0,
          AstNode::CompareOperation(AstNodeFactory::new_compare_operation(
            0,
            AstCompareOp::Eq,
            var_proxy("i"),
            smi(42),
          )),
          AstNode::BreakStatement(AstNodeFactory::new_break_statement(0, 0)),
          None,
        )),
      ]),
    )),
    ret(var_proxy("i")),
  ]);

  let bytecode = BytecodeGenerator::generate(&ast)?;
  let result = execute_bytecode(&bytecode)?;
  assert_eq!(result, Value::Smi(42));

  OK
}

#[test]
fn test_unary_and_logical_operations() -> Void {
  info!("Testing unary operations");

  // AST:
  // a = -(-42);
  // return a;
  let ast = block(vec![
    assign(
      "a",
      AstNode::UnaryOperation(AstNodeFactory::new_unary_operation(
        0,
        AstUnaryOp::Sub,
        AstNode::UnaryOperation(AstNodeFactory::new_unary_operation(
          0,
          AstUnaryOp::Sub,
          smi(42),
        )),
      )),
    ),
    ret(var_proxy("a")),
  ]);

  let bytecode = BytecodeGenerator::generate(&ast)?;
  let result = execute_bytecode(&bytecode)?;
  assert_eq!(result, Value::Smi(42));

  OK
}

#[test]
fn test_function_literal_with_parameters() -> Void {
  info!("Testing FunctionLiteral with parameters execution");

  let mut factory = AstNodeFactory::new();
  let fn_lit = factory.new_function_literal(
    0,
    FunctionLiteralOptions {
      params: vec![AstRawString::from_utf8("a"), AstRawString::from_utf8("b")],
      body: vec![ret(add(var_proxy("a"), var_proxy("b")))],
      ..Default::default()
    },
  );

  let bytecode = BytecodeGenerator::generate_function(&fn_lit)?;
  let mut dispatcher = BytecodeDispatcher::new();
  let res = dispatcher.execute_with_args(&bytecode, &[Value::Smi(18), Value::Smi(24)])?;
  assert_eq!(res, Value::Smi(42));

  OK
}

#[test]
fn test_global_variables_execution() -> Void {
  info!("Testing LdaGlobal execution");

  let ast = block(vec![ret(add(var_proxy("globalOffset"), smi(42)))]);

  let bytecode = BytecodeGenerator::generate(&ast)?;
  let mut dispatcher = BytecodeDispatcher::new();
  dispatcher.set_global("globalOffset", Value::Smi(100));

  let res = dispatcher.execute(&bytecode)?;
  assert_eq!(res, Value::Smi(142));

  OK
}

#[test]
fn test_large_number_modulo_wrapping() -> Void {
  info!("Testing large number modulo wrapping for bitwise operations");

  let large_val = Value::Number(4025962439.0);
  assert_eq!(large_val.to_i32(), -269004857);
  assert_eq!(large_val.to_u32(), 4025962439);

  let shift_res = large_val.shift_right_logical(&Value::Smi(0));
  assert_eq!(shift_res, Value::Number(4025962439.0));

  OK
}
