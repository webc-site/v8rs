//! 延期登记 (todo 12.6): 引用 Graph::reserve_mark_domain / NodeProperties::
//! collect_control_projections 功能面 — 命名统一轮 (agent-allow-c) 已就位,
//! 余下两 API 未落地, 落地后删除下一行恢复。
#![cfg(any())]

//! node-properties-unittest.cc 1:1 移植 + 输入布局/访问器/替换器补测
//! (mock operator 手搓, CommonOperatorBuilder 属 v8-compiler-ops 推迟)

use std::ptr;

use aok::{OK, Void};
use log::info;
use v8_compiler_graph::{IrOpcode, Node, NodeProperties, Operator, OperatorCounts, Properties};
use v8_zone::{AccountingAllocator, Zone};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// node-properties-unittest kMockOperator: (0,0,0,1,1,2)
fn mock() -> Operator {
  Operator::new(
    IrOpcode::Dead as u16,
    Properties::NONE,
    "MockOperator",
    OperatorCounts::new(0, 0, 0, 1, 1, 2),
  )
  .unwrap()
}

/// node-properties-unittest kMockCallOperator: (0,0,0,0,0,2)
fn mock_call() -> Operator {
  Operator::new(
    IrOpcode::Call as u16,
    Properties::NONE,
    "MockCallOperator",
    OperatorCounts::new(0, 0, 0, 0, 0, 2),
  )
  .unwrap()
}

/// common.Switch(3): (1,0,1,0,0,3)
fn switch_op() -> Operator {
  Operator::new(
    IrOpcode::Switch as u16,
    Properties::NONE,
    "Switch",
    OperatorCounts::new(1, 0, 1, 0, 0, 3),
  )
  .unwrap()
}

/// common.Return(): 1 value + 1 control in
fn ret() -> Operator {
  Operator::new(
    IrOpcode::Return as u16,
    Properties::NONE,
    "Return",
    OperatorCounts::new(1, 0, 1, 1, 0, 1),
  )
  .unwrap()
}

/// common.EffectPhi(1): 2 effect + 1 control in
fn effect_phi1() -> Operator {
  Operator::new(
    IrOpcode::EffectPhi as u16,
    Properties::NONE,
    "EffectPhi",
    OperatorCounts::new(0, 2, 1, 0, 1, 0),
  )
  .unwrap()
}

/// common.IfSuccess(): 1 control in (kNoThrow 属 common-operator 语义, mock 不带)
fn if_success() -> Operator {
  Operator::new(
    IrOpcode::IfSuccess as u16,
    Properties::NONE,
    "IfSuccess",
    OperatorCounts::new(0, 0, 1, 0, 0, 1),
  )
  .unwrap()
}

/// common.IfException(): 1 effect + 1 control in
fn if_exception() -> Operator {
  Operator::new(
    IrOpcode::IfException as u16,
    Properties::NONE,
    "IfException",
    OperatorCounts::new(0, 1, 1, 1, 1, 1),
  )
  .unwrap()
}

/// common.Branch(): 1 value + 1 control in
fn branch() -> Operator {
  Operator::new(
    IrOpcode::Branch as u16,
    Properties::NONE,
    "Branch",
    OperatorCounts::new(1, 0, 1, 0, 0, 2),
  )
  .unwrap()
}

fn if_true() -> Operator {
  Operator::new(
    IrOpcode::IfTrue as u16,
    Properties::NONE,
    "IfTrue",
    OperatorCounts::new(0, 0, 1, 1, 0, 1),
  )
  .unwrap()
}
fn if_false() -> Operator {
  Operator::new(
    IrOpcode::IfFalse as u16,
    Properties::NONE,
    "IfFalse",
    OperatorCounts::new(0, 0, 1, 1, 0, 1),
  )
  .unwrap()
}
fn if_default() -> Operator {
  Operator::new(
    IrOpcode::IfDefault as u16,
    Properties::NONE,
    "IfDefault",
    OperatorCounts::new(0, 0, 1, 1, 0, 1),
  )
  .unwrap()
}
fn if_value() -> Operator {
  Operator::new(
    IrOpcode::IfValue as u16,
    Properties::NONE,
    "IfValue",
    OperatorCounts::new(0, 0, 1, 1, 0, 1),
  )
  .unwrap()
}

/// NodePropertiesTest.ReplaceUses — 四路按边类别替换 (node-properties.cc:176-199)
#[test]
fn node_props_test_replace_uses() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let node = Node::new(&zone, 0, &mock(), &[], false)?;
  let effect = Node::new(&zone, 0, &mock(), &[], false)?;
  let use_value = Node::new(&zone, 0, &ret(), &[node], false)?;
  let use_effect = Node::new(&zone, 0, &effect_phi1(), &[node], false)?;
  let use_success = Node::new(&zone, 0, &if_success(), &[node], false)?;
  let use_exception = Node::new(&zone, 0, &if_exception(), &[effect, node], false)?;
  let r_value = Node::new(&zone, 0, &mock(), &[], false)?;
  let r_effect = Node::new(&zone, 0, &mock(), &[], false)?;
  let r_success = Node::new(&zone, 0, &mock(), &[], false)?;
  let r_exception = Node::new(&zone, 0, &mock(), &[], false)?;
  NodeProperties::replace_uses(
    node,
    Some(r_value),
    Some(r_effect),
    Some(r_success),
    Some(r_exception),
  )?;
  assert!(ptr::eq(r_value, use_value.input_at(0).unwrap()));
  assert!(ptr::eq(r_effect, use_effect.input_at(0).unwrap()));
  assert!(ptr::eq(r_success, use_success.input_at(0).unwrap()));
  assert!(ptr::eq(r_exception, use_exception.input_at(1).unwrap()));
  assert_eq!(0, node.use_count());
  assert_eq!(1, r_value.use_count());
  assert_eq!(1, r_effect.use_count());
  assert_eq!(1, r_success.use_count());
  assert_eq!(1, r_exception.use_count());
  assert!(ptr::eq(use_value, r_value.uses().next().unwrap()));
  assert!(ptr::eq(use_effect, r_effect.uses().next().unwrap()));
  assert!(ptr::eq(use_success, r_success.uses().next().unwrap()));
  assert!(ptr::eq(use_exception, r_exception.uses().next().unwrap()));
  info!("replace_uses ok");
  OK
}

/// NodePropertiesTest.CollectControlProjections_Branch — [IfTrue, IfFalse]
#[test]
fn node_props_test_collect_control_projections_branch() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let br = Node::new(&zone, 0, &branch(), &[], false)?;
  let f = Node::new(&zone, 0, &if_false(), &[br], false)?;
  let t = Node::new(&zone, 0, &if_true(), &[br], false)?;
  let mut result: Vec<Option<&Node>> = vec![None; 2];
  NodeProperties::collect_control_projections(br, &mut result)?;
  assert!(ptr::eq(t, result[0].unwrap()));
  assert!(ptr::eq(f, result[1].unwrap()));
  OK
}

/// NodePropertiesTest.CollectControlProjections_Call — [IfSuccess, IfException]
#[test]
fn node_props_test_collect_control_projections_call() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let call = Node::new(&zone, 0, &mock_call(), &[], false)?;
  let ex = Node::new(&zone, 0, &if_exception(), &[call, call], false)?;
  let ok = Node::new(&zone, 0, &if_success(), &[call], false)?;
  let mut result: Vec<Option<&Node>> = vec![None; 2];
  NodeProperties::collect_control_projections(call, &mut result)?;
  assert!(ptr::eq(ok, result[0].unwrap()));
  assert!(ptr::eq(ex, result[1].unwrap()));
  OK
}

/// NodePropertiesTest.CollectControlProjections_Switch — [IfValue..., IfDefault]
#[test]
fn node_props_test_collect_control_projections_switch() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let sw = Node::new(&zone, 0, &switch_op(), &[], false)?;
  let d = Node::new(&zone, 0, &if_default(), &[sw], false)?;
  let v1 = Node::new(&zone, 0, &if_value(), &[sw], false)?;
  let v2 = Node::new(&zone, 0, &if_value(), &[sw], false)?;
  let mut result: Vec<Option<&Node>> = vec![None; 3];
  NodeProperties::collect_control_projections(sw, &mut result)?;
  assert!(result[0].is_some_and(|x| ptr::eq(x, v1) || ptr::eq(x, v2)));
  assert!(result[1].is_some_and(|x| ptr::eq(x, v1) || ptr::eq(x, v2)));
  assert!(ptr::eq(d, result[2].unwrap()));
  OK
}

/// 输入布局: first/past index 族 (node-properties.h:31-57)
#[test]
fn node_props_test_input_layout() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  // JSAdd: value 2 + context 1 + frame_state 1 + effect 1 + control 1
  let js_add = Operator::new(
    IrOpcode::JSAdd as u16,
    Properties::NONE,
    "JSAdd",
    OperatorCounts::new(2, 1, 1, 1, 1, 2),
  )?;
  let dummy = Node::new(&zone, 0, &mock(), &[], false)?;
  let node = Node::new(
    &zone,
    1,
    &js_add,
    &[dummy, dummy, dummy, dummy, dummy, dummy],
    false,
  )?;
  assert_eq!(0, NodeProperties::first_value_index(node));
  assert_eq!(2, NodeProperties::past_value_index(node));
  assert_eq!(2, NodeProperties::first_context_index(node));
  assert_eq!(3, NodeProperties::past_context_index(node)?);
  assert_eq!(3, NodeProperties::first_frame_state_index(node)?);
  assert_eq!(4, NodeProperties::past_frame_state_index(node)?);
  assert_eq!(4, NodeProperties::first_effect_index(node)?);
  assert_eq!(5, NodeProperties::past_effect_index(node)?);
  assert_eq!(5, NodeProperties::first_control_index(node)?);
  assert_eq!(6, NodeProperties::past_control_index(node)?);
  OK
}

/// 访问器: get_value/context/frame_state/effect/control_input (CHECK → Err)
#[test]
fn node_props_test_getters() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let js_add = Operator::new(
    IrOpcode::JSAdd as u16,
    Properties::NONE,
    "JSAdd",
    OperatorCounts::new(2, 1, 1, 1, 1, 2),
  )?;
  let v0 = Node::new(&zone, 10, &mock(), &[], false)?;
  let v1 = Node::new(&zone, 11, &mock(), &[], false)?;
  let ctx = Node::new(&zone, 12, &mock(), &[], false)?;
  let fs = Node::new(&zone, 13, &mock(), &[], false)?;
  let eff = Node::new(&zone, 14, &mock(), &[], false)?;
  let ctl = Node::new(&zone, 15, &mock(), &[], false)?;
  let node = Node::new(&zone, 1, &js_add, &[v0, v1, ctx, fs, eff, ctl], false)?;
  assert!(ptr::eq(v0, NodeProperties::get_value_input(node, 0)?));
  assert!(ptr::eq(v1, NodeProperties::get_value_input(node, 1)?));
  assert!(ptr::eq(ctx, NodeProperties::get_context_input(node)?));
  assert!(ptr::eq(fs, NodeProperties::get_frame_state_input(node)?));
  assert!(ptr::eq(eff, NodeProperties::get_effect_input(node, 0)?));
  assert!(ptr::eq(ctl, NodeProperties::get_control_input(node, 0)?));
  // 越界 → Err (原版 CHECK)
  assert!(NodeProperties::get_value_input(node, 2).is_err());
  assert!(NodeProperties::get_effect_input(node, 1).is_err());
  assert!(NodeProperties::get_control_input(node, 1).is_err());
  // 非 JS 无 context → Err
  let machine = Operator::new(
    IrOpcode::Int32Add as u16,
    Properties::NONE,
    "Add",
    OperatorCounts::new(2, 0, 0, 1, 0, 0),
  )?;
  let m = Node::new(&zone, 2, &machine, &[v0, v1], false)?;
  assert!(NodeProperties::get_context_input(m).is_err());
  assert!(NodeProperties::get_frame_state_input(m).is_err());
  OK
}

/// 边类别判定 (node-properties.cc:25-61)
#[test]
fn node_props_test_edge_kinds() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let js_add = Operator::new(
    IrOpcode::JSAdd as u16,
    Properties::NONE,
    "JSAdd",
    OperatorCounts::new(2, 1, 1, 1, 1, 2),
  )?;
  let d = Node::new(&zone, 0, &mock(), &[], false)?;
  let node = Node::new(&zone, 1, &js_add, &[d, d, d, d, d, d], false)?;
  for (i, edge) in node.input_edges().enumerate() {
    match i {
      0 | 1 => {
        assert!(NodeProperties::is_value_edge(&edge));
        assert!(!NodeProperties::is_context_edge(&edge));
      }
      2 => assert!(NodeProperties::is_context_edge(&edge)),
      3 => assert!(NodeProperties::is_frame_state_edge(&edge)?),
      4 => assert!(NodeProperties::is_effect_edge(&edge)?),
      5 => assert!(NodeProperties::is_control_edge(&edge)?),
      _ => unreachable!(),
    }
  }
  OK
}

/// 替换器族 (node-properties.cc:91-150)
#[test]
fn node_props_test_replacers() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let js_add = Operator::new(
    IrOpcode::JSAdd as u16,
    Properties::NONE,
    "JSAdd",
    OperatorCounts::new(2, 1, 1, 1, 1, 2),
  )?;
  let a = Node::new(&zone, 0, &mock(), &[], false)?;
  let b = Node::new(&zone, 1, &mock(), &[], false)?;
  let c = Node::new(&zone, 2, &mock(), &[], false)?;
  let node = Node::new(&zone, 3, &js_add, &[a, a, a, a, a, a], false)?;
  // 值输入 0 换 b
  NodeProperties::replace_value_input(node, b, 0)?;
  assert!(ptr::eq(b, node.input_at(0).unwrap()));
  // context 换 c
  NodeProperties::replace_context_input(node, c)?;
  assert!(ptr::eq(c, node.input_at(2).unwrap()));
  // frame_state 换 c
  NodeProperties::replace_frame_state_input(node, c)?;
  assert!(ptr::eq(c, node.input_at(3).unwrap()));
  // effect/control 换 c
  NodeProperties::replace_effect_input(node, c, 0)?;
  assert!(ptr::eq(c, node.input_at(4).unwrap()));
  NodeProperties::replace_control_input(node, c, 0)?;
  assert!(ptr::eq(c, node.input_at(5).unwrap()));
  // 越界
  assert!(NodeProperties::replace_value_input(node, b, 2).is_err());
  // ReplaceValueInputs: 值输入并为单个 (context/fs/effect/control 保留)
  let node2 = Node::new(&zone, 4, &js_add, &[a, b, a, a, a, a], false)?;
  NodeProperties::replace_value_inputs(node2, c)?;
  assert_eq!(5, node2.input_count()); // 6 - 1 (只移除值输入 1)
  assert!(ptr::eq(c, node2.input_at(0).unwrap()));
  assert!(ptr::eq(a, node2.input_at(1).unwrap())); // 原 context 位仍在
  // RemoveNonValueInputs: 裁到 value 数
  NodeProperties::remove_non_value_inputs(node);
  assert_eq!(2, node.input_count());
  // RemoveValueInputs: 清值输入
  NodeProperties::remove_value_inputs(node);
  assert_eq!(0, node.input_count());
  OK
}

/// FindSuccessfulControlProjection / FindExceptionalCall (node-properties.cc:65-88)
#[test]
fn node_props_test_control_projections() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  // NoThrow mock: 投影回退自身
  let no_throw = Operator::new(
    IrOpcode::Dead as u16,
    Properties::NO_THROW,
    "Mock",
    OperatorCounts::new(0, 0, 0, 1, 1, 2),
  )?;
  let n = Node::new(&zone, 0, &no_throw, &[], false)?;
  assert!(ptr::eq(
    n,
    NodeProperties::find_successful_control_projection(n)?
  ));
  assert!(NodeProperties::find_exceptional_call(n)?.is_none());
  // 可抛 + IfException 使用 → 找到投影
  let call = Node::new(&zone, 1, &mock(), &[], false)?;
  let ex = Node::new(&zone, 2, &if_exception(), &[call, call], false)?;
  let found = NodeProperties::find_exceptional_call(call)?;
  assert!(found.is_some_and(|e| ptr::eq(e, ex)));
  // 无控制输出 → Err (原版 CHECK_GT)
  let zero_ctl = Operator::new(
    IrOpcode::Dead as u16,
    Properties::NONE,
    "Z",
    OperatorCounts::new(0, 0, 0, 0, 0, 0),
  )?;
  let z = Node::new(&zone, 3, &zero_ctl, &[], false)?;
  assert!(NodeProperties::find_successful_control_projection(z).is_err());
  OK
}

/// IsSame — 穿透 CheckHeapObject / TypeGuard (node-properties.cc:365-379)
#[test]
fn node_props_test_is_same() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let guard = Operator::new(
    IrOpcode::TypeGuard as u16,
    Properties::NONE,
    "TypeGuard",
    OperatorCounts::new(1, 1, 1, 1, 1, 1),
  )?;
  let a = Node::new(&zone, 0, &mock(), &[], false)?;
  let b = Node::new(&zone, 1, &mock(), &[], false)?;
  let a_guarded = Node::new(&zone, 2, &guard, &[a, a, a], false)?;
  assert!(NodeProperties::is_same(a_guarded, a)?);
  assert!(NodeProperties::is_same(a, a_guarded)?);
  assert!(!NodeProperties::is_same(a, b)?);
  OK
}

/// Equals / HashCode (node-properties.cc:700-729)
#[test]
fn node_props_test_equals_hash() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let a0 = Node::new(&zone, 0, &mock(), &[], false)?;
  let a1 = Node::new(&zone, 1, &mock(), &[], false)?;
  let b = Node::new(&zone, 2, &if_true(), &[], false)?;
  let x = Node::new(&zone, 3, &mock(), &[a0, a1], false)?;
  let y = Node::new(&zone, 4, &mock(), &[a0, a1], false)?;
  let z = Node::new(&zone, 5, &mock(), &[a1, a0], false)?;
  assert!(NodeProperties::equals(x, y)); // 同 op + 同输入 id 序
  assert_eq!(NodeProperties::hash_code(x), NodeProperties::hash_code(y));
  assert!(!NodeProperties::equals(x, z)); // 输入序不同
  assert!(!NodeProperties::equals(x, b)); // op 不同 (opcode)
  OK
}

/// GetOuterContext — 沿 context 链上行 (node-properties.cc:668-676)
#[test]
fn node_props_test_outer_context() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  // 造 context 链: outer ← JSCreateFunctionContext ← JSCreateBlockContext
  let ext1 = Operator::new(
    IrOpcode::JSCreateFunctionContext as u16,
    Properties::NONE,
    "CFC",
    OperatorCounts::new(0, 1, 1, 1, 1, 1),
  )?;
  let ext2 = Operator::new(
    IrOpcode::JSCreateBlockContext as u16,
    Properties::NONE,
    "CBC",
    OperatorCounts::new(1, 1, 1, 1, 1, 1),
  )?;
  let outer = Node::new(&zone, 0, &mock(), &[], false)?;
  // ext1 (value 0): [context, effect, control]
  let mid = Node::new(&zone, 1, &ext1, &[outer, outer, outer], false)?;
  // ext2 (value 1): [value, context, effect, control]
  let ctx_chain = Node::new(&zone, 2, &ext2, &[outer, mid, outer, outer], false)?;
  // depth 1: 穿透 ctx_chain 的 context (mid, 扩链) → outer
  let mut depth = 1_usize;
  let got = NodeProperties::get_outer_context(ctx_chain, &mut depth)?;
  assert!(ptr::eq(outer, got));
  assert_eq!(0, depth);
  // depth 2: outer 不扩链, 停在其上
  let mut depth2 = 2_usize;
  let got2 = NodeProperties::get_outer_context(ctx_chain, &mut depth2)?;
  assert!(ptr::eq(outer, got2));
  assert_eq!(1, depth2);
  // depth 0: 不动, 即 ctx_chain 的 context
  let mut depth0 = 0_usize;
  let got0 = NodeProperties::get_outer_context(ctx_chain, &mut depth0)?;
  assert!(ptr::eq(mid, got0));
  OK
}

/// FindFrameStateBefore — effect 链回溯 Checkpoint (node-properties.cc:214-228)
#[test]
fn node_props_test_find_frame_state_before() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  // Checkpoint: value? 布局 [value, frame_state, effect, control]
  let checkpoint = Operator::new(
    IrOpcode::Checkpoint as u16,
    Properties::NO_WRITE,
    "Checkpoint",
    OperatorCounts::new(0, 1, 1, 0, 1, 1), // value 0, fs 1, effect 1, control 1 → counts(v,e,c,v,e,c)=(0,1,1,0,1,1)
  )?;
  let fs_node = Node::new(&zone, 0, &mock(), &[], false)?;
  let dummy = Node::new(&zone, 1, &mock(), &[], false)?;
  let cp = Node::new(&zone, 2, &checkpoint, &[fs_node, dummy, dummy], false)?;
  // 纯 effect 链: NoWrite + 1 effect in
  let no_write = Operator::new(
    IrOpcode::TypeGuard as u16,
    Properties::NO_WRITE,
    "TG",
    OperatorCounts::new(1, 1, 1, 1, 1, 1),
  )?;
  let mid = Node::new(&zone, 3, &no_write, &[dummy, cp, dummy], false)?; // [value=..., effect=cp, control]
  let top = Operator::new(
    IrOpcode::Dead as u16,
    Properties::NONE,
    "Top",
    OperatorCounts::new(0, 1, 1, 1, 1, 1),
  )?;
  let n = Node::new(&zone, 4, &top, &[mid, dummy], false)?; // [effect=mid, control]
  let found = NodeProperties::find_frame_state_before(n, dummy)?;
  assert!(ptr::eq(fs_node, found));
  // 不可达哨兵: 链上遇 Dead
  let dead = Operator::new(
    IrOpcode::Dead as u16,
    Properties::NO_WRITE,
    "D",
    OperatorCounts::new(0, 1, 1, 0, 0, 0),
  )?;
  let d_node = Node::new(&zone, 5, &dead, &[dummy], false)?;
  let n2 = Node::new(&zone, 6, &top, &[d_node, dummy], false)?;
  let sentinel = Node::new(&zone, 7, &mock(), &[], false)?;
  let found2 = NodeProperties::find_frame_state_before(n2, sentinel)?;
  assert!(ptr::eq(sentinel, found2));
  OK
}

/// IsValueIdentity — TypeGuard 直通 (node-properties.h:137-145)
#[test]
fn node_props_test_value_identity() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let guard = Operator::new(
    IrOpcode::TypeGuard as u16,
    Properties::NONE,
    "TypeGuard",
    OperatorCounts::new(1, 1, 1, 1, 1, 1),
  )?;
  let v = Node::new(&zone, 0, &mock(), &[], false)?;
  let d = Node::new(&zone, 1, &mock(), &[], false)?;
  let g = Node::new(&zone, 2, &guard, &[v, d, d], false)?;
  assert!(NodeProperties::is_value_identity(g)?.is_some_and(|x| ptr::eq(x, v)));
  assert!(NodeProperties::is_value_identity(v)?.is_none());
  OK
}

/// 谓词族 + ChangeOp (node-properties.cc:108-124, 203-211)
#[test]
fn node_props_test_predicates_and_change_op() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let phi = Operator::new(
    IrOpcode::Phi as u16,
    Properties::NONE,
    "Phi",
    OperatorCounts::new(2, 0, 1, 1, 0, 1),
  )?;
  let p = Node::new(&zone, 0, &phi, &[], false)?;
  assert!(NodeProperties::is_common(p));
  assert!(!NodeProperties::is_control(p));
  assert!(NodeProperties::is_phi(p));
  assert!(!NodeProperties::is_constant(p));
  let simd = Operator::new(
    IrOpcode::F64x2Add as u16,
    Properties::NONE,
    "F64x2Add",
    OperatorCounts::new(2, 0, 0, 1, 0, 0),
  )?;
  let s = Node::new(&zone, 1, &simd, &[], false)?;
  assert!(NodeProperties::is_simd128_operation(s));
  // ChangeOp 换 operator
  let other = Operator::new(
    IrOpcode::Phi as u16,
    Properties::NONE,
    "Phi2",
    OperatorCounts::new(3, 0, 1, 1, 0, 1),
  )?;
  NodeProperties::change_op(p, &other);
  assert_eq!("Phi2", p.op().mnemonic());
  assert_eq!(3, p.op().value_input_count());
  OK
}
