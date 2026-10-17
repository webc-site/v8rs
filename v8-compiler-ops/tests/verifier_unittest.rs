//! Verifier 单元测试

use aok::{OK, Void};
use v8_compiler_graph::Graph;
use v8_compiler_ops::{CommonOperatorBuilder, Error, MachineOperatorBuilder, Verifier};
use v8_zone::{AccountingAllocator, Zone};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_verifier_valid_graph() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let mut graph = Graph::new(&zone);
  let common = CommonOperatorBuilder::new(&zone);

  // 构造规范 TurboFan 图:
  // Start (value_out: 1, effect_out: 1, control_out: 1)
  let start_op = common.start(1)?;
  let start = graph.new_node_unchecked(start_op, &[], false)?;
  graph.set_start(start);

  // Parameter (value_in: 1 [start], value_out: 1)
  let param_op = common.parameter(0, Some("arg0"))?;
  let param = graph.new_node_unchecked(param_op, &[start], false)?;

  // Return (value_in: 2 [hidden, param], effect_in: 1 [start], control_in: 1 [start], control_out: 1)
  let ret_op = common.return_op(1)?;
  let ret = graph.new_node_unchecked(ret_op, &[param, param, start, start], false)?;

  // End (control_in: 1 [ret], control_out: 0)
  let end_op = common.end(1)?;
  let end = graph.new_node_unchecked(end_op, &[ret], false)?;
  graph.set_end(end);

  // 逐节点校验
  Verifier::verify_node(start)?;
  Verifier::verify_node(param)?;
  Verifier::verify_node(ret)?;
  Verifier::verify_node(end)?;

  // 整图遍历校验
  Verifier::verify_graph(&graph)?;

  OK
}

#[test]
fn test_verifier_input_count_mismatch() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let graph = Graph::new(&zone);
  let machine = MachineOperatorBuilder::new(&zone);

  // Int32Add 期望 2 个值输入，但只给 1 个
  let dead_op = CommonOperatorBuilder::new(&zone).dead();
  let dead = graph.new_node_unchecked(dead_op, &[], false)?;

  let add_op = machine.int32_add();
  let invalid_node = graph.new_node_unchecked(add_op, &[dead], false)?;

  let res = Verifier::verify_node(invalid_node);
  assert!(matches!(
    res,
    Err(Error::InputCountMismatch {
      expected: 2,
      actual: 1,
      ..
    })
  ));

  OK
}

#[test]
fn test_verifier_invalid_control_and_effect_input() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let graph = Graph::new(&zone);
  let common = CommonOperatorBuilder::new(&zone);
  let machine = MachineOperatorBuilder::new(&zone);

  let start_op = common.start(1)?;
  let start = graph.new_node_unchecked(start_op, &[], false)?;

  // 纯算术节点 Int32Add: 无 effect 输出，无 control 输出
  let add_op = machine.int32_add();
  let pure_node = graph.new_node_unchecked(add_op, &[start, start], false)?;

  // Return 的 inputs 布局: value[0], value[1], effect, control
  // 故意将 pure_node (无 effect 输出) 接到 effect 槽 (第 2 位)
  let ret_op = common.return_op(1)?;
  let bad_effect_node =
    graph.new_node_unchecked(ret_op, &[start, start, pure_node, start], false)?;

  let res_effect = Verifier::verify_node(bad_effect_node);
  assert!(matches!(
    res_effect,
    Err(Error::InvalidEffectInput { .. }) | Err(Error::InvalidEffectUse { .. })
  ));

  // 故意将 pure_node (无 control 输出) 接到 control 槽 (第 3 位)
  let bad_control_node =
    graph.new_node_unchecked(ret_op, &[start, start, start, pure_node], false)?;
  let res_control = Verifier::verify_node(bad_control_node);
  assert!(matches!(
    res_control,
    Err(Error::InvalidControlInput { .. }) | Err(Error::InvalidControlUse { .. })
  ));

  OK
}

#[test]
fn test_verifier_edge_replacement() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let graph = Graph::new(&zone);
  let common = CommonOperatorBuilder::new(&zone);
  let machine = MachineOperatorBuilder::new(&zone);

  let start_op = common.start(1)?;
  let start = graph.new_node_unchecked(start_op, &[], false)?;

  let ret_op = common.return_op(1)?;
  let ret = graph.new_node_unchecked(ret_op, &[start, start, start, start], false)?;

  let pure_op = machine.int32_add();
  let pure_node = graph.new_node_unchecked(pure_op, &[start, start], false)?;

  // 查找 ret 的 control 输入边 (输入下标 3)
  for edge in ret.input_edges() {
    if edge.index() == 3 {
      // 用 pure_node 替换 control 边应被拒绝
      let check_replace = Verifier::verify_edge_input_replacement(&edge, pure_node);
      assert!(matches!(
        check_replace,
        Err(Error::EdgeReplacementMismatch {
          kind: "Control",
          ..
        })
      ));

      // 用 start 替换 control 边 (start 有 control 输出) 应被允许
      assert!(Verifier::verify_edge_input_replacement(&edge, start).is_ok());
    }
  }

  OK
}

#[test]
fn test_verifier_pair_binary_node() -> Void {
  use v8_compiler_graph::{IrOpcode, OperatorProperties};

  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let graph = Graph::new(&zone);
  let common = CommonOperatorBuilder::new(&zone);
  let machine = MachineOperatorBuilder::new(&zone);

  let dead_op = common.dead();
  let dead = graph.new_node_unchecked(dead_op, &[], false)?;

  // Int32PairAdd: 4 值输入 → 总输入 4 (无 effect/control/context)
  let pair_add = machine.int32_pair_add();
  assert_eq!(OperatorProperties::get_total_input_count(pair_add)?, 4);
  let valid_add = graph.new_node_unchecked(pair_add, &[dead, dead, dead, dead], false)?;
  Verifier::verify_node(valid_add)?;

  // 缺一条值输入 (3) 应报 InputCountMismatch{expected:4, actual:3}
  let bad_add = graph.new_node_unchecked(pair_add, &[dead, dead, dead], false)?;
  let res = Verifier::verify_node(bad_add);
  assert!(matches!(
    res,
    Err(Error::InputCountMismatch {
      expected: 4,
      actual: 3,
      mnemonic: "Int32PairAdd",
      ..
    })
  ));

  // Word32PairShl: 3 值输入 (pair0, pair1, shift) → 2 值输出
  let pair_shl = machine.word32_pair_shl();
  assert_eq!(pair_shl.value_output_count(), 2);
  assert_eq!(OperatorProperties::get_total_input_count(pair_shl)?, 3);
  let valid_shl = graph.new_node_unchecked(pair_shl, &[dead, dead, dead], false)?;
  Verifier::verify_node(valid_shl)?;
  assert_eq!(valid_shl.op().opcode(), IrOpcode::Word32PairShl as u16);

  // 多给一条 (4) 应被拒绝
  let bad_shl = graph.new_node_unchecked(pair_shl, &[dead, dead, dead, dead], false)?;
  let res = Verifier::verify_node(bad_shl);
  assert!(matches!(
    res,
    Err(Error::InputCountMismatch {
      expected: 3,
      actual: 4,
      mnemonic: "Word32PairShl",
      ..
    })
  ));

  OK
}

#[test]
fn test_verifier_js_feedback_vector_node() -> Void {
  use v8_compiler_graph::FeedbackSource;
  use v8_compiler_ops::JSOperatorBuilder;

  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let graph = Graph::new(&zone);
  let common = CommonOperatorBuilder::new(&zone);
  let js = JSOperatorBuilder::new(&zone);

  let start_op = common.start(1)?;
  let start = graph.new_node_unchecked(start_op, &[], false)?;

  let add_op = js.js_add(FeedbackSource::invalid())?;
  // JSAdd 输入布局 (7条): Left, Right, FeedbackVector, Context, FrameState, Effect, Control
  // 正确 7 边接入:
  let valid_js_add = graph.new_node_unchecked(
    add_op,
    &[start, start, start, start, start, start, start],
    false,
  )?;
  Verifier::verify_node(valid_js_add)?;

  // 错误输入数: 缺失 FeedbackVector (仅 6 条边) 应被 Verifier 拒绝
  let invalid_js_add =
    graph.new_node_unchecked(add_op, &[start, start, start, start, start, start], false)?;
  let res = Verifier::verify_node(invalid_js_add);
  assert!(matches!(
    res,
    Err(Error::InputCountMismatch {
      expected: 7,
      actual: 6,
      mnemonic: "JSAdd",
      ..
    })
  ));

  OK
}
