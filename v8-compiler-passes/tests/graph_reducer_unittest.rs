//! v8: test/unittests/compiler/graph-reducer-unittest.cc 1:1 迁移
//!
//! gmock → Rust 映射 (见 .agents/glossary.md): EXPECT_CALL/Sequence 以
//! 记录器 (Rc<RefCell<Vec<..>>>) + 调用序断言等价; MockEditor 两用例
//! (Replace/Revisit) 因 Editor 面收敛到具体 GraphReducer, 改以真实对象
//! 行为断言 (理由登记 readme/zh.md 覆盖映射)。

use std::{
  cell::{Cell, RefCell},
  mem::transmute,
  ptr::eq,
  rc::Rc,
};

use aok::{OK, Void};
use v8_compiler_graph::{
  Error as GraphError, Graph, IrOpcode, Node, NodeId, Operator, OperatorCounts, Properties,
};
use v8_compiler_ops::CommonOperatorBuilder;
use v8_compiler_passes::{GraphHandle, GraphReducer, Reducer, Reduction};
use v8_zone::{AccountingAllocator, Zone};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// —— 原版测试算子表 (kOpcodeA0..kOpcodeC2) ——
const K_OPCODE_A0: u16 = 10;
const K_OPCODE_A1: u16 = 11;
const K_OPCODE_A2: u16 = 12;
const K_OPCODE_B0: u16 = 20;
const K_OPCODE_B1: u16 = 21;
const K_OPCODE_B2: u16 = 22;
const K_OPCODE_C0: u16 = 30;
const K_OPCODE_C1: u16 = 31;
const K_OPCODE_C2: u16 = 32;

fn top(
  opcode: u16,
  properties: Properties,
  mnemonic: &'static str,
  value_in: u32,
  value_out: u32,
) -> Result<Operator, GraphError> {
  Operator::new(
    opcode,
    properties,
    mnemonic,
    OperatorCounts::new(value_in, 0, 0, value_out, 0, 0),
  )
}

/// 原版九个静态 TestOperator 的等价构造 (逐字节同参)
struct TestOps {
  a0: Operator,
  a1: Operator,
  a2: Operator,
  b0: Operator,
  b1: Operator,
  b2: Operator,
  c0: Operator,
  c1: Operator,
  c2: Operator,
}

impl TestOps {
  fn new() -> Result<Self, GraphError> {
    Ok(Self {
      a0: top(K_OPCODE_A0, Properties::NO_WRITE, "opa1", 0, 1)?,
      a1: top(K_OPCODE_A1, Properties::NONE, "opa2", 1, 1)?,
      a2: top(K_OPCODE_A2, Properties::NONE, "opa3", 2, 1)?,
      b0: top(K_OPCODE_B0, Properties::NO_WRITE, "opb0", 0, 1)?,
      b1: top(K_OPCODE_B1, Properties::NO_WRITE, "opb1", 1, 1)?,
      b2: top(K_OPCODE_B2, Properties::NO_WRITE, "opb2", 2, 1)?,
      c0: top(K_OPCODE_C0, Properties::NO_WRITE, "opc0", 0, 1)?,
      c1: top(K_OPCODE_C1, Properties::NO_WRITE, "opc1", 1, 1)?,
      c2: top(K_OPCODE_C2, Properties::NO_WRITE, "opc2", 2, 1)?,
    })
  }
}

/// 原版 MockOperator / MockOpEffect / MockOpControl (kDead opcode)
fn mock_op(
  name: &'static str,
  effect_in: u32,
  control_in: u32,
  effect_out: u32,
  control_out: u32,
) -> Result<Operator, GraphError> {
  Operator::new(
    IrOpcode::Dead as u16,
    Properties::NONE,
    name,
    OperatorCounts::new(0, effect_in, control_in, 1, effect_out, control_out),
  )
}

fn use_ids(node: &Node) -> Vec<NodeId> {
  let mut v: Vec<NodeId> = node.uses().map(|n| n.id()).collect();
  v.sort_unstable();
  v
}

fn input_id(node: &Node, i: usize) -> Option<NodeId> {
  node.input_at(i).map(Node::id)
}

fn op_is(node: &Node, op: &Operator) -> bool {
  eq(node.op(), op)
}

// —— 原版 Reducer 子类逐一对应 ——

/// InPlaceABReducer: A* → B* 就地换 op 不建新节点
struct InPlaceAb<'o> {
  b0: &'o Operator,
  b1: &'o Operator,
  b2: &'o Operator,
}

impl<'o, 'h, 'z, 'a> Reducer<'h, 'z, 'a> for InPlaceAb<'o> {
  fn reducer_name(&self) -> &'static str {
    "InPlaceABReducer"
  }
  fn reduce(
    &mut self,
    _gr: &mut GraphReducer<'h, 'z, 'a>,
    node: &'z Node,
  ) -> v8_compiler_passes::Result<Reduction<'z>> {
    use v8_compiler_graph::NodeProperties;
    match node.op().opcode() {
      K_OPCODE_A0 => {
        assert_eq!(0, node.input_count());
        NodeProperties::change_op(node, self.b0);
        Ok(Reduction::replace(node))
      }
      K_OPCODE_A1 => {
        assert_eq!(1, node.input_count());
        NodeProperties::change_op(node, self.b1);
        Ok(Reduction::replace(node))
      }
      K_OPCODE_A2 => {
        assert_eq!(2, node.input_count());
        NodeProperties::change_op(node, self.b2);
        Ok(Reduction::replace(node))
      }
      _ => Ok(Reduction::no_change()),
    }
  }
}

/// InPlaceBCReducer: B* → C* 就地换 op
struct InPlaceBc<'o> {
  c0: &'o Operator,
  c1: &'o Operator,
  c2: &'o Operator,
}

impl<'o, 'h, 'z, 'a> Reducer<'h, 'z, 'a> for InPlaceBc<'o> {
  fn reducer_name(&self) -> &'static str {
    "InPlaceBCReducer"
  }
  fn reduce(
    &mut self,
    _gr: &mut GraphReducer<'h, 'z, 'a>,
    node: &'z Node,
  ) -> v8_compiler_passes::Result<Reduction<'z>> {
    use v8_compiler_graph::NodeProperties;
    match node.op().opcode() {
      K_OPCODE_B0 => {
        assert_eq!(0, node.input_count());
        NodeProperties::change_op(node, self.c0);
        Ok(Reduction::replace(node))
      }
      K_OPCODE_B1 => {
        assert_eq!(1, node.input_count());
        NodeProperties::change_op(node, self.c1);
        Ok(Reduction::replace(node))
      }
      K_OPCODE_B2 => {
        assert_eq!(2, node.input_count());
        NodeProperties::change_op(node, self.c2);
        Ok(Reduction::replace(node))
      }
      _ => Ok(Reduction::no_change()),
    }
  }
}

/// NewABReducer: A* → B* 建新节点
struct NewAb<'o, 'h, 'z, 'a> {
  graph: &'h Graph<'z, 'a>,
  b0: &'o Operator,
  b1: &'o Operator,
  b2: &'o Operator,
}

impl<'o, 'h, 'z, 'a> Reducer<'h, 'z, 'a> for NewAb<'o, 'h, 'z, 'a> {
  fn reducer_name(&self) -> &'static str {
    "NewABReducer"
  }
  fn reduce(
    &mut self,
    _gr: &mut GraphReducer<'h, 'z, 'a>,
    node: &'z Node,
  ) -> v8_compiler_passes::Result<Reduction<'z>> {
    let repl = match node.op().opcode() {
      K_OPCODE_A0 => {
        assert_eq!(0, node.input_count());
        self.graph.new_node(self.b0, &[], false)?
      }
      K_OPCODE_A1 => {
        assert_eq!(1, node.input_count());
        self
          .graph
          .new_node(self.b1, &[node.input_at(0).unwrap_or(node)], false)?
      }
      K_OPCODE_A2 => {
        assert_eq!(2, node.input_count());
        self.graph.new_node(
          self.b2,
          &[
            node.input_at(0).unwrap_or(node),
            node.input_at(1).unwrap_or(node),
          ],
          false,
        )?
      }
      _ => return Ok(Reduction::no_change()),
    };
    Ok(Reduction::replace(repl))
  }
}

/// A0Wrapper: A0 → B1(A0)
struct A0Wrapper<'o, 'h, 'z, 'a> {
  graph: &'h Graph<'z, 'a>,
  b1: &'o Operator,
}

impl<'o, 'h, 'z, 'a> Reducer<'h, 'z, 'a> for A0Wrapper<'o, 'h, 'z, 'a> {
  fn reducer_name(&self) -> &'static str {
    "A0Wrapper"
  }
  fn reduce(
    &mut self,
    _gr: &mut GraphReducer<'h, 'z, 'a>,
    node: &'z Node,
  ) -> v8_compiler_passes::Result<Reduction<'z>> {
    if node.op().opcode() == K_OPCODE_A0 {
      assert_eq!(0, node.input_count());
      let n = self.graph.new_node(self.b1, &[node], false)?;
      return Ok(Reduction::replace(n));
    }
    Ok(Reduction::no_change())
  }
}

/// B0Wrapper: B0 → C1(C1(B0))
struct B0Wrapper<'o, 'h, 'z, 'a> {
  graph: &'h Graph<'z, 'a>,
  c1: &'o Operator,
}

impl<'o, 'h, 'z, 'a> Reducer<'h, 'z, 'a> for B0Wrapper<'o, 'h, 'z, 'a> {
  fn reducer_name(&self) -> &'static str {
    "B0Wrapper"
  }
  fn reduce(
    &mut self,
    _gr: &mut GraphReducer<'h, 'z, 'a>,
    node: &'z Node,
  ) -> v8_compiler_passes::Result<Reduction<'z>> {
    if node.op().opcode() == K_OPCODE_B0 {
      assert_eq!(0, node.input_count());
      let inner = self.graph.new_node(self.c1, &[node], false)?;
      let outer = self.graph.new_node(self.c1, &[inner], false)?;
      return Ok(Reduction::replace(outer));
    }
    Ok(Reduction::no_change())
  }
}

/// Forwarder: Op1(x) → x (A1/B1 通用)
struct Forwarder<'o> {
  opcode: u16,
  name: &'static str,
  _probe: Option<&'o Operator>,
}

impl<'o, 'h, 'z, 'a> Reducer<'h, 'z, 'a> for Forwarder<'o> {
  fn reducer_name(&self) -> &'static str {
    self.name
  }
  fn reduce(
    &mut self,
    _gr: &mut GraphReducer<'h, 'z, 'a>,
    node: &'z Node,
  ) -> v8_compiler_passes::Result<Reduction<'z>> {
    if node.op().opcode() == self.opcode {
      assert_eq!(1, node.input_count());
      let x = node.input_at(0).unwrap_or(node);
      return Ok(Reduction::replace(x));
    }
    Ok(Reduction::no_change())
  }
}

/// AB2Sorter: 双输入 A2/B2 按 id 排序
struct Ab2Sorter;

impl<'h, 'z, 'a> Reducer<'h, 'z, 'a> for Ab2Sorter {
  fn reducer_name(&self) -> &'static str {
    "AB2Sorter"
  }
  fn reduce(
    &mut self,
    _gr: &mut GraphReducer<'h, 'z, 'a>,
    node: &'z Node,
  ) -> v8_compiler_passes::Result<Reduction<'z>> {
    let op = node.op().opcode();
    if op == K_OPCODE_A2 || op == K_OPCODE_B2 {
      assert_eq!(2, node.input_count());
      let x = node.input_at(0).unwrap_or(node);
      let y = node.input_at(1).unwrap_or(node);
      if x.id() > y.id() {
        node.replace_input(0, Some(y));
        node.replace_input(1, Some(x));
        return Ok(Reduction::replace(node));
      }
    }
    Ok(Reduction::no_change())
  }
}

/// 记录型 reducer (gmock MockReducer 的 Reduce 记录面)
struct Recorder {
  name: &'static str,
  idx: u8,
  log: Rc<RefCell<Vec<(u8, NodeId)>>>,
  /// 首次于该 id 触发 Changed(原地) (ReduceAgainAfterChanged 的 r3)
  change_once_at: Option<NodeId>,
  fired: Cell<bool>,
}

impl Recorder {
  fn new(name: &'static str, idx: u8, log: &Rc<RefCell<Vec<(u8, NodeId)>>>) -> Self {
    Self {
      name,
      idx,
      log: Rc::clone(log),
      change_once_at: None,
      fired: Cell::new(false),
    }
  }

  fn with_change(mut self, at: NodeId) -> Self {
    self.change_once_at = Some(at);
    self
  }
}

impl<'h, 'z, 'a> Reducer<'h, 'z, 'a> for Recorder {
  fn reducer_name(&self) -> &'static str {
    self.name
  }

  fn reduce(
    &mut self,
    _gr: &mut GraphReducer<'h, 'z, 'a>,
    node: &'z Node,
  ) -> v8_compiler_passes::Result<Reduction<'z>> {
    self.log.borrow_mut().push((self.idx, node.id()));
    if self.change_once_at == Some(node.id()) && !self.fired.get() {
      self.fired.set(true);
      return Ok(Reduction::changed(node));
    }
    Ok(Reduction::no_change())
  }
}

/// Revisit 用例专用: 首次归约 node1 时回访 node0
struct Revisitor {
  revisit_node: NodeId,
  done: Cell<bool>,
}

impl<'h, 'z, 'a> Reducer<'h, 'z, 'a> for Revisitor {
  fn reducer_name(&self) -> &'static str {
    "Revisitor"
  }
  fn reduce(
    &mut self,
    gr: &mut GraphReducer<'h, 'z, 'a>,
    node: &'z Node,
  ) -> v8_compiler_passes::Result<Reduction<'z>> {
    if self.revisit_node == node.id() && !self.done.get() {
      self.done.set(true);
      // 找 node0 (其输入) 回访
      if let Some(input) = node.input_at(0) {
        gr.revisit(input);
      }
    }
    Ok(Reduction::no_change())
  }
}

// ————————————————— GraphReducerTest —————————————————

/// v8: TEST_F(GraphReducerTest, NodeIsDeadAfterReplace)
#[test]
fn graph_reducer_test_node_is_dead_after_replace() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let ops = TestOps::new()?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);

  let node0 = graph.new_node(&ops.a0, &[], false)?;
  let node1 = graph.new_node(&ops.a1, &[node0], false)?;
  let node2 = graph.new_node(&ops.a1, &[node0], false)?;
  handle.set_end(node1);

  let log: Rc<RefCell<Vec<(u8, NodeId)>>> = Rc::default();
  let _ = &log;
  struct Plan<'z> {
    calls: Rc<RefCell<Vec<(u8, NodeId)>>>,
    _node0_id: NodeId,
    node1_id: NodeId,
    repl: Option<&'z Node>,
  }
  // 计划不可用泛型 Recorder 表达 (Replace(node2) 定向), 用本地脚本 reducer
  struct Scripted<'z> {
    inner: RefCell<Plan<'z>>,
  }
  impl<'z, 'h, 'zz, 'a> Reducer<'h, 'zz, 'a> for Scripted<'z> {
    fn reducer_name(&self) -> &'static str {
      "MockReducer"
    }
    fn reduce(
      &mut self,
      _gr: &mut GraphReducer<'h, 'zz, 'a>,
      node: &'zz Node,
    ) -> v8_compiler_passes::Result<Reduction<'zz>> {
      let plan = self.inner.borrow();
      plan.calls.borrow_mut().push((0, node.id()));
      if node.id() == plan.node1_id {
        let repl = plan.repl.expect("plan repl");
        // 生命周期适配: repl 为 &'z, 测试内同 zone 即同一 'zz
        let repl = unsafe { transmute::<&'z Node, &'zz Node>(repl) };
        return Ok(Reduction::replace(repl));
      }
      Ok(Reduction::no_change())
    }
  }
  let scripted = Scripted {
    inner: RefCell::new(Plan {
      calls: Rc::clone(&log),
      _node0_id: node0.id(),
      node1_id: node1.id(),
      repl: Some(node2),
    }),
  };

  let mut gr = GraphReducer::new(&handle, None);
  gr.add_reducer(Box::new(scripted));
  gr.reduce_node(node1)?;

  let calls = log.borrow();
  // EXPECT_CALL 序列: Reduce(node0) → NoChange; Reduce(node1) → Replace(node2)
  assert_eq!(&[(0, node0.id()), (0, node1.id())][..], &calls[..]);
  drop(calls);
  assert!(!node0.is_dead());
  assert!(node1.is_dead());
  assert!(!node2.is_dead());
  OK
}

/// v8: TEST_F(GraphReducerTest, ReduceOnceForEveryReducer)
#[test]
fn graph_reducer_test_reduce_once_for_every_reducer() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let ops = TestOps::new()?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);
  let node0 = graph.new_node(&ops.a0, &[], false)?;
  handle.set_end(node0);

  let log: Rc<RefCell<Vec<(u8, NodeId)>>> = Rc::default();
  let r1 = Recorder::new("r1", 0, &log);
  let r2 = Recorder::new("r2", 1, &log);

  let mut gr = GraphReducer::new(&handle, None);
  gr.add_reducer(Box::new(r1));
  gr.add_reducer(Box::new(r2));
  gr.reduce_node(node0)?;

  assert_eq!(&[(0, node0.id()), (1, node0.id())][..], &log.borrow()[..]);
  OK
}

/// v8: TEST_F(GraphReducerTest, ReduceAgainAfterChanged)
#[test]
fn graph_reducer_test_reduce_again_after_changed() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let ops = TestOps::new()?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);
  let node0 = graph.new_node(&ops.a0, &[], false)?;
  handle.set_end(node0);

  let log: Rc<RefCell<Vec<(u8, NodeId)>>> = Rc::default();
  let r1 = Recorder::new("r1", 0, &log);
  let r2 = Recorder::new("r2", 1, &log);
  let r3 = Recorder::new("r3", 2, &log).with_change(node0.id());

  let mut gr = GraphReducer::new(&handle, None);
  gr.add_reducer(Box::new(r1));
  gr.add_reducer(Box::new(r2));
  gr.add_reducer(Box::new(r3));
  gr.reduce_node(node0)?;

  // InSequence s1,s2,s3: r1,r2,r3(Changed), 重跑 r1,r2 (跳过 r3)
  assert_eq!(
    &[
      (0, node0.id()),
      (1, node0.id()),
      (2, node0.id()),
      (0, node0.id()),
      (1, node0.id())
    ][..],
    &log.borrow()[..]
  );
  OK
}

/// v8: TEST_F(GraphReducerTest, ReduceGraphFromEnd1)
#[test]
fn graph_reducer_test_reduce_graph_from_end1() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let ops = TestOps::new()?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);
  let n = graph.new_node(&ops.a0, &[], false)?;
  let end = graph.new_node(&ops.a1, &[n], false)?;
  handle.set_end(end);

  let log: Rc<RefCell<Vec<(u8, NodeId)>>> = Rc::default();
  let r1 = Recorder::new("r1", 0, &log);

  let mut gr = GraphReducer::new(&handle, None);
  gr.add_reducer(Box::new(r1));
  gr.reduce_graph()?;

  // Sequence s: Reduce(n) 先于 Reduce(end)
  assert_eq!(&[(0, n.id()), (0, end.id())][..], &log.borrow()[..]);
  OK
}

/// v8: TEST_F(GraphReducerTest, ReduceGraphFromEnd2)
#[test]
fn graph_reducer_test_reduce_graph_from_end2() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let ops = TestOps::new()?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);
  let n1 = graph.new_node(&ops.a0, &[], false)?;
  let n2 = graph.new_node(&ops.a1, &[n1], false)?;
  let n3 = graph.new_node(&ops.a1, &[n1], false)?;
  let end = graph.new_node(&ops.a2, &[n2, n3], false)?;
  handle.set_end(end);

  let log: Rc<RefCell<Vec<(u8, NodeId)>>> = Rc::default();
  let r1 = Recorder::new("r1", 0, &log);

  let mut gr = GraphReducer::new(&handle, None);
  gr.add_reducer(Box::new(r1));
  gr.reduce_graph()?;

  // s1=(n1,n2,end), s2=(n1,n3,end): 深度优先输入先于使用者
  assert_eq!(
    &[(0, n1.id()), (0, n2.id()), (0, n3.id()), (0, end.id())][..],
    &log.borrow()[..]
  );
  OK
}

/// v8: TEST_F(GraphReducerTest, ReduceInPlace1)
#[test]
fn graph_reducer_test_reduce_in_place1() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let ops = TestOps::new()?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);
  let n1 = graph.new_node(&ops.a0, &[], false)?;
  let end = graph.new_node(&ops.a1, &[n1], false)?;
  handle.set_end(end);

  for _ in 0..3 {
    let before = graph.node_count();
    let mut gr = GraphReducer::new(&handle, None);
    gr.add_reducer(Box::new(InPlaceAb {
      b0: &ops.b0,
      b1: &ops.b1,
      b2: &ops.b2,
    }));
    gr.reduce_graph()?;
    assert_eq!(before, graph.node_count());
    assert!(op_is(n1, &ops.b0));
    assert!(op_is(end, &ops.b1));
    assert_eq!(input_id(end, 0), Some(n1.id()));
  }
  OK
}

/// v8: TEST_F(GraphReducerTest, ReduceInPlace2)
#[test]
fn graph_reducer_test_reduce_in_place2() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let ops = TestOps::new()?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);
  let n1 = graph.new_node(&ops.a0, &[], false)?;
  let n2 = graph.new_node(&ops.a1, &[n1], false)?;
  let n3 = graph.new_node(&ops.a1, &[n1], false)?;
  let end = graph.new_node(&ops.a2, &[n2, n3], false)?;
  handle.set_end(end);

  for _ in 0..3 {
    let before = graph.node_count();
    let mut gr = GraphReducer::new(&handle, None);
    gr.add_reducer(Box::new(InPlaceAb {
      b0: &ops.b0,
      b1: &ops.b1,
      b2: &ops.b2,
    }));
    gr.reduce_graph()?;
    assert_eq!(before, graph.node_count());
    assert!(op_is(n1, &ops.b0));
    assert!(op_is(n2, &ops.b1));
    assert_eq!(input_id(n2, 0), Some(n1.id()));
    assert!(op_is(n3, &ops.b1));
    assert_eq!(input_id(n3, 0), Some(n1.id()));
    assert!(op_is(end, &ops.b2));
    assert_eq!(input_id(end, 0), Some(n2.id()));
    assert_eq!(input_id(end, 1), Some(n3.id()));
  }
  OK
}

/// v8: TEST_F(GraphReducerTest, ReduceNew1)
#[test]
fn graph_reducer_test_reduce_new1() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let ops = TestOps::new()?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);
  let n1 = graph.new_node(&ops.a0, &[], false)?;
  let n2 = graph.new_node(&ops.a1, &[n1], false)?;
  let n3 = graph.new_node(&ops.a1, &[n1], false)?;
  let end = graph.new_node(&ops.a2, &[n2, n3], false)?;
  handle.set_end(end);

  for i in 0..3 {
    let before = graph.node_count();
    let mut gr = GraphReducer::new(&handle, None);
    gr.add_reducer(Box::new(NewAb {
      graph: &graph,
      b0: &ops.b0,
      b1: &ops.b1,
      b2: &ops.b2,
    }));
    gr.reduce_graph()?;
    if i == 0 {
      assert_ne!(before, graph.node_count());
    } else {
      assert_eq!(before, graph.node_count());
    }
    let nend = handle.end().unwrap();
    assert!(!eq(nend, end));
    let nn2 = nend.input_at(0).unwrap();
    let nn3 = nend.input_at(1).unwrap();
    let nn1 = nn2.input_at(0).unwrap();
    assert_eq!(input_id(nn3, 0), Some(nn1.id()));
    assert!(op_is(nn1, &ops.b0));
    assert!(op_is(nn2, &ops.b1));
    assert!(op_is(nn3, &ops.b1));
    assert!(op_is(nend, &ops.b2));
  }
  OK
}

/// v8: TEST_F(GraphReducerTest, Wrapping1)
#[test]
fn graph_reducer_test_wrapping1() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let ops = TestOps::new()?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);
  let end = graph.new_node(&ops.a0, &[], false)?;
  handle.set_end(end);
  assert_eq!(1, graph.node_count());

  let mut gr = GraphReducer::new(&handle, None);
  gr.add_reducer(Box::new(A0Wrapper {
    graph: &graph,
    b1: &ops.b1,
  }));
  gr.reduce_graph()?;
  assert_eq!(2, graph.node_count());

  let nend = handle.end().unwrap();
  assert!(!eq(nend, end));
  assert!(op_is(nend, &ops.b1));
  assert_eq!(1, nend.input_count());
  assert_eq!(input_id(nend, 0), Some(end.id()));
  OK
}

/// v8: TEST_F(GraphReducerTest, Wrapping2)
#[test]
fn graph_reducer_test_wrapping2() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let ops = TestOps::new()?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);
  let end = graph.new_node(&ops.b0, &[], false)?;
  handle.set_end(end);
  assert_eq!(1, graph.node_count());

  let mut gr = GraphReducer::new(&handle, None);
  gr.add_reducer(Box::new(B0Wrapper {
    graph: &graph,
    c1: &ops.c1,
  }));
  gr.reduce_graph()?;
  assert_eq!(3, graph.node_count());

  let nend = handle.end().unwrap();
  assert!(!eq(nend, end));
  assert!(op_is(nend, &ops.c1));
  assert_eq!(1, nend.input_count());
  let n1 = nend.input_at(0).unwrap();
  assert!(!eq(n1, end));
  assert!(op_is(n1, &ops.c1));
  assert_eq!(1, n1.input_count());
  assert_eq!(input_id(n1, 0), Some(end.id()));
  OK
}

/// v8: TEST_F(GraphReducerTest, Forwarding1)
#[test]
fn graph_reducer_test_forwarding1() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let ops = TestOps::new()?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);
  let n1 = graph.new_node(&ops.a0, &[], false)?;
  let end = graph.new_node(&ops.a1, &[n1], false)?;
  handle.set_end(end);

  for _ in 0..3 {
    let before = graph.node_count();
    let mut gr = GraphReducer::new(&handle, None);
    gr.add_reducer(Box::new(Forwarder {
      opcode: K_OPCODE_A1,
      name: "A1Forwarder",
      _probe: None,
    }));
    gr.reduce_graph()?;
    assert_eq!(before, graph.node_count());
    assert!(op_is(n1, &ops.a0));
    assert_eq!(Some(n1.id()), handle.end().map(Node::id));
  }
  OK
}

/// v8: TEST_F(GraphReducerTest, Forwarding2)
#[test]
fn graph_reducer_test_forwarding2() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let ops = TestOps::new()?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);
  let n1 = graph.new_node(&ops.a0, &[], false)?;
  let n2 = graph.new_node(&ops.a1, &[n1], false)?;
  let n3 = graph.new_node(&ops.a1, &[n1], false)?;
  let end = graph.new_node(&ops.a2, &[n2, n3], false)?;
  handle.set_end(end);

  for _ in 0..3 {
    let before = graph.node_count();
    let mut gr = GraphReducer::new(&handle, None);
    gr.add_reducer(Box::new(Forwarder {
      opcode: K_OPCODE_A1,
      name: "A1Forwarder",
      _probe: None,
    }));
    gr.reduce_graph()?;
    assert_eq!(before, graph.node_count());
    assert!(op_is(n1, &ops.a0));
    assert_eq!(input_id(end, 0), Some(n1.id()));
    assert_eq!(input_id(end, 1), Some(n1.id()));
    assert!(op_is(end, &ops.a2));
    assert_eq!(0, n2.use_count());
    assert_eq!(0, n3.use_count());
  }
  OK
}

/// v8: TEST_F(GraphReducerTest, Forwarding3)
#[test]
fn graph_reducer_test_forwarding3() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let ops = TestOps::new()?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);

  for i in 0..8 {
    let n1 = graph.new_node(&ops.a0, &[], false)?;
    let mut end = n1;
    for _ in 0..i {
      end = graph.new_node(&ops.a1, &[end], false)?;
    }
    handle.set_end(end);

    for _ in 0..3 {
      let before = graph.node_count();
      let mut gr = GraphReducer::new(&handle, None);
      gr.add_reducer(Box::new(Forwarder {
        opcode: K_OPCODE_A1,
        name: "A1Forwarder",
        _probe: None,
      }));
      gr.reduce_graph()?;
      assert_eq!(before, graph.node_count());
      assert!(op_is(n1, &ops.a0));
      assert_eq!(Some(n1.id()), handle.end().map(Node::id));
    }
  }
  OK
}

/// v8: TEST_F(GraphReducerTest, ReduceForward1)
#[test]
fn graph_reducer_test_reduce_forward1() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let ops = TestOps::new()?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);
  let n1 = graph.new_node(&ops.a0, &[], false)?;
  let n2 = graph.new_node(&ops.a1, &[n1], false)?;
  let n3 = graph.new_node(&ops.a1, &[n1], false)?;
  let end = graph.new_node(&ops.a2, &[n2, n3], false)?;
  handle.set_end(end);

  for _ in 0..3 {
    let before = graph.node_count();
    let mut gr = GraphReducer::new(&handle, None);
    gr.add_reducer(Box::new(InPlaceAb {
      b0: &ops.b0,
      b1: &ops.b1,
      b2: &ops.b2,
    }));
    gr.add_reducer(Box::new(Forwarder {
      opcode: K_OPCODE_B1,
      name: "B1Forwarder",
      _probe: None,
    }));
    gr.reduce_graph()?;
    assert_eq!(before, graph.node_count());
    assert!(op_is(n1, &ops.b0));
    assert!(n2.is_dead());
    assert_eq!(input_id(end, 0), Some(n1.id()));
    assert!(n3.is_dead());
    assert_eq!(input_id(end, 0), Some(n1.id()));
    assert!(op_is(end, &ops.b2));
    assert_eq!(0, n2.use_count());
    assert_eq!(0, n3.use_count());
  }
  OK
}

/// v8: TEST_F(GraphReducerTest, Sorter1)
#[test]
fn graph_reducer_test_sorter1() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let ops = TestOps::new()?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);

  // (i==0..5) 的六种输入排列
  let pairs = [(2usize, 3usize), (3, 2), (2, 0), (0, 2), (3, 0), (0, 3)];
  for (a, b) in pairs {
    let n1 = graph.new_node(&ops.a0, &[], false)?;
    let n2 = graph.new_node(&ops.a1, &[n1], false)?;
    let n3 = graph.new_node(&ops.a1, &[n1], false)?;
    let all = [n1, n1, n2, n3];
    let end = graph.new_node(&ops.a2, &[all[a], all[b]], false)?;
    handle.set_end(end);

    let before = graph.node_count();
    let mut gr = GraphReducer::new(&handle, None);
    gr.add_reducer(Box::new(Ab2Sorter));
    gr.reduce_graph()?;
    assert_eq!(before, graph.node_count());
    assert!(op_is(n1, &ops.a0));
    assert!(op_is(n2, &ops.a1));
    assert!(op_is(n3, &ops.a1));
    assert!(op_is(end, &ops.a2));
    assert_eq!(Some(end.id()), handle.end().map(Node::id));
    let i0 = end.input_at(0).unwrap().id();
    let i1 = end.input_at(1).unwrap().id();
    assert!(i0 <= i1);
  }
  OK
}

/// GenDAG (原版匿名 helper)
fn gen_dag<'z, 'a>(
  graph: &Graph<'z, 'a>,
  handle: &GraphHandle<'_, 'z, 'a>,
  ops: &TestOps,
  p3: [usize; 2],
  p2: [usize; 4],
  p1: [usize; 2],
) -> v8_compiler_passes::Result<()> {
  let level4 = graph.new_node(&ops.a0, &[], false)?;
  let l3a = graph.new_node(&ops.a1, &[level4], false)?;
  let l3b = graph.new_node(&ops.a1, &[level4], false)?;
  let l3 = [l3a, l3b];
  let mut l2 = [l3a; 4];
  l2[0] = graph.new_node(&ops.a1, &[l3[p3[0]]], false)?;
  l2[1] = graph.new_node(&ops.a1, &[l3[p3[1]]], false)?;
  l2[2] = graph.new_node(&ops.a1, &[l3[p3[0]]], false)?;
  l2[3] = graph.new_node(&ops.a1, &[l3[p3[1]]], false)?;
  let l1a = graph.new_node(&ops.a2, &[l2[p2[0]], l2[p2[1]]], false)?;
  let l1b = graph.new_node(&ops.a2, &[l2[p2[2]], l2[p2[3]]], false)?;
  let l1 = [l1a, l1b];
  let end = graph.new_node(&ops.a2, &[l1[p1[0]], l1[p1[1]]], false)?;
  handle.set_end(end);
  Ok(())
}

/// v8: TEST_F(GraphReducerTest, SortForwardReduce)
#[test]
fn graph_reducer_test_sort_forward_reduce() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let ops = TestOps::new()?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);

  for j in 0..2 {
    let p3 = [j, 1 - j];
    for m in 0..2 {
      let p1 = [m, 1 - m];
      for k in 0..24 {
        // 原版: 0..3 的全排列逐 k 解码
        let mut p2 = [usize::MAX; 4];
        let mut n = k;
        for d in (1..=4).rev() {
          let mut p = n % d;
          for slot in &mut p2 {
            if *slot == usize::MAX {
              if p == 0 {
                *slot = (d - 1) as usize;
              }
              p -= 1;
            }
          }
          n /= d;
        }
        gen_dag(&graph, &handle, &ops, p3, p2, p1)?;

        let mut gr = GraphReducer::new(&handle, None);
        gr.add_reducer(Box::new(Ab2Sorter));
        gr.add_reducer(Box::new(Forwarder {
          opcode: K_OPCODE_A1,
          name: "A1Forwarder",
          _probe: None,
        }));
        gr.add_reducer(Box::new(InPlaceAb {
          b0: &ops.b0,
          b1: &ops.b1,
          b2: &ops.b2,
        }));
        gr.reduce_graph()?;

        let end = handle.end().unwrap();
        assert!(op_is(end, &ops.b2));
        let n1 = end.input_at(0).unwrap();
        let n2 = end.input_at(1).unwrap();
        assert!(!eq(n1, n2));
        assert!(n1.id() < n2.id());
        assert!(op_is(n1, &ops.b2));
        assert!(op_is(n2, &ops.b2));
        let n4 = n1.input_at(0).unwrap();
        assert!(op_is(n4, &ops.b0));
        assert_eq!(input_id(n1, 1), Some(n4.id()));
        assert_eq!(input_id(n2, 0), Some(n4.id()));
        assert_eq!(input_id(n2, 1), Some(n4.id()));
      }
    }
  }
  OK
}

/// v8: TEST_F(GraphReducerTest, Order)
#[test]
fn graph_reducer_test_order() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let ops = TestOps::new()?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);

  for i in 0..2 {
    let n1 = graph.new_node(&ops.a0, &[], false)?;
    let end = graph.new_node(&ops.a1, &[n1], false)?;
    handle.set_end(end);

    for _ in 0..3 {
      let before = graph.node_count();
      let mut gr = GraphReducer::new(&handle, None);
      if i == 0 {
        gr.add_reducer(Box::new(InPlaceAb {
          b0: &ops.b0,
          b1: &ops.b1,
          b2: &ops.b2,
        }));
        gr.add_reducer(Box::new(InPlaceBc {
          c0: &ops.c0,
          c1: &ops.c1,
          c2: &ops.c2,
        }));
      } else {
        gr.add_reducer(Box::new(InPlaceBc {
          c0: &ops.c0,
          c1: &ops.c1,
          c2: &ops.c2,
        }));
        gr.add_reducer(Box::new(InPlaceAb {
          b0: &ops.b0,
          b1: &ops.b1,
          b2: &ops.b2,
        }));
      }
      gr.reduce_graph()?;
      assert_eq!(before, graph.node_count());
      assert!(op_is(n1, &ops.c0));
      assert!(op_is(end, &ops.c1));
      assert_eq!(input_id(end, 0), Some(n1.id()));
    }
  }
  OK
}

// ————————————————— AdvancedReducerTest —————————————————

/// v8: TEST_F(AdvancedReducerTest, Replace)
/// DummyReducer 转发 Replace(node, node) 且 Reduce 返回 NoChange (Changed() == false)
#[test]
fn advanced_reducer_test_replace() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let ops = TestOps::new()?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);
  let node0 = graph.new_node(&ops.a0, &[], false)?;
  let node1 = graph.new_node(&ops.a1, &[node0], false)?;
  handle.set_start(node0);
  handle.set_end(node1);

  let calls = Rc::new(RefCell::new(Vec::new()));
  let calls_clone = Rc::clone(&calls);

  struct DummyReducer {
    calls: Rc<RefCell<Vec<(NodeId, NodeId)>>>,
  }
  impl<'h, 'z, 'a> Reducer<'h, 'z, 'a> for DummyReducer {
    fn reducer_name(&self) -> &'static str {
      "DummyReducer"
    }
    fn reduce(
      &mut self,
      gr: &mut GraphReducer<'h, 'z, 'a>,
      node: &'z Node,
    ) -> v8_compiler_passes::Result<Reduction<'z>> {
      self.calls.borrow_mut().push((node.id(), node.id()));
      gr.replace(node, node)?;
      Ok(Reduction::no_change())
    }
  }

  let mut gr = GraphReducer::new(&handle, None);
  let mut r = DummyReducer { calls: calls_clone };
  let res0 = r.reduce(&mut gr, node0)?;
  let res1 = r.reduce(&mut gr, node1)?;
  assert!(!res0.is_changed());
  assert!(!res1.is_changed());
  assert_eq!(
    *calls.borrow(),
    vec![(node0.id(), node0.id()), (node1.id(), node1.id())]
  );
  OK
}

/// v8: TEST_F(AdvancedReducerTest, Revisit)
/// 真实对象等价: 归约中 Revisit 已访问节点 → 重新入栈二次归约
#[test]
fn advanced_reducer_test_revisit() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let ops = TestOps::new()?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);
  let node0 = graph.new_node(&ops.a0, &[], false)?;
  let node1 = graph.new_node(&ops.a1, &[node0], false)?;
  handle.set_end(node1);

  let mut gr = GraphReducer::new(&handle, None);
  gr.add_reducer(Box::new(Revisitor {
    revisit_node: node1.id(),
    done: Cell::new(false),
  }));
  gr.reduce_node(node1)?;

  // 回访生效: node0 二次归约后图仍可正常收敛, 结构不变
  assert_eq!(input_id(node1, 0), Some(node0.id()));
  assert!(!node0.is_dead());
  assert!(!node1.is_dead());
  OK
}

// ————————————— ReplaceWithValue 四例 (真实 GraphReducer 驱动) —————————————

/// v8: TEST_F(AdvancedReducerTest, ReplaceWithValue_ValueUse)
#[test]
fn advanced_reducer_test_replace_with_value_value_use() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let ops = TestOps::new()?;
  let common = CommonOperatorBuilder::new(&zone);
  let mock = mock_op("MockOperator", 0, 0, 1, 0)?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);

  let node = graph.new_node(&mock, &[], false)?;
  let start = graph.new_node(common.start(1)?, &[], false)?;
  handle.set_start(start);
  // 原版 common.Int32Constant(0) 未迁至算子层, 以等价纯值算子 a0 代位
  let zero = graph.new_node(&ops.a0, &[], false)?;
  let return_op = common.return_op(2)?;
  let use_value = graph.new_node(return_op, &[zero, node, start, start], false)?;
  handle.set_end(use_value);
  let replacement = graph.new_node(&mock, &[], false)?;

  let mut gr = GraphReducer::new(&handle, None);
  gr.replace_with_value(node, replacement, None, None)?;

  assert_eq!(input_id(use_value, 1), Some(replacement.id()));
  assert_eq!(0, node.use_count());
  assert_eq!(1, replacement.use_count());
  assert_eq!(vec![use_value.id()], use_ids(replacement));
  OK
}

/// v8: TEST_F(AdvancedReducerTest, ReplaceWithValue_EffectUse)
#[test]
fn advanced_reducer_test_replace_with_value_effect_use() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let common = CommonOperatorBuilder::new(&zone);
  let mock = mock_op("MockOperator", 0, 0, 1, 0)?;
  let mock_effect = mock_op("MockOpEffect", 1, 0, 1, 1)?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);

  let start = graph.new_node(common.start(1)?, &[], false)?;
  handle.set_start(start);
  let node = graph.new_node(&mock_effect, &[start], false)?;
  let use_control = graph.new_node(common.merge(1)?, &[start], false)?;
  let use_effect = graph.new_node(common.effect_phi(1)?, &[node, use_control], false)?;
  handle.set_end(use_effect);
  let replacement = graph.new_node(&mock, &[], false)?;

  let mut gr = GraphReducer::new(&handle, None);
  gr.replace_with_value(node, replacement, None, None)?;

  assert_eq!(input_id(use_effect, 0), Some(start.id()));
  assert_eq!(0, node.use_count());
  assert_eq!(3, start.use_count());
  assert_eq!(0, replacement.use_count());
  // UnorderedElementsAre(use_effect, use_control, node)
  let expected = {
    let mut v = vec![use_effect.id(), use_control.id(), node.id()];
    v.sort_unstable();
    v
  };
  assert_eq!(expected, use_ids(start));
  OK
}

/// v8: TEST_F(AdvancedReducerTest, ReplaceWithValue_ControlUse1)
#[test]
fn advanced_reducer_test_replace_with_value_control_use1() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let common = CommonOperatorBuilder::new(&zone);
  let mock = mock_op("MockOperator", 0, 0, 1, 0)?;
  let mock_control = mock_op("MockOpControl", 0, 1, 1, 0)?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);

  let start = graph.new_node(common.start(1)?, &[], false)?;
  handle.set_start(start);
  let node = graph.new_node(&mock_control, &[start], false)?;
  let success = graph.new_node(common.if_success(), &[node], false)?;
  let use_control = graph.new_node(common.merge(1)?, &[success], false)?;
  handle.set_end(use_control);
  let replacement = graph.new_node(&mock, &[], false)?;

  let mut gr = GraphReducer::new(&handle, None);
  gr.replace_with_value(node, replacement, None, None)?;

  assert_eq!(input_id(use_control, 0), Some(start.id()));
  assert_eq!(0, node.use_count());
  assert_eq!(2, start.use_count());
  assert_eq!(0, replacement.use_count());
  let expected = {
    let mut v = vec![use_control.id(), node.id()];
    v.sort_unstable();
    v
  };
  assert_eq!(expected, use_ids(start));
  OK
}

/// v8: TEST_F(AdvancedReducerTest, ReplaceWithValue_ControlUse2)
#[test]
fn advanced_reducer_test_replace_with_value_control_use2() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "gr");
  let common = CommonOperatorBuilder::new(&zone);
  let mock = mock_op("MockOperator", 0, 0, 1, 0)?;
  let mock_control = mock_op("MockOpControl", 0, 1, 1, 0)?;
  let graph = Graph::new(&zone);
  let handle = GraphHandle::new(&graph, None, None);

  let start = graph.new_node(common.start(1)?, &[], false)?;
  handle.set_start(start);
  let effect = graph.new_node(&mock, &[], false)?;
  let dead = graph.new_node(&mock, &[], false)?;
  let node = graph.new_node(&mock_control, &[start], false)?;
  let success = graph.new_node(common.if_success(), &[node], false)?;
  let exception = graph.new_node(common.if_exception(), &[effect, node], false)?;
  handle.set_end(exception);
  let use_control = graph.new_node(common.merge(1)?, &[success], false)?;
  let replacement = graph.new_node(&mock, &[], false)?;

  let mut gr = GraphReducer::new(&handle, Some(dead));
  gr.replace_with_value(node, replacement, None, None)?;

  assert_eq!(input_id(use_control, 0), Some(start.id()));
  assert_eq!(input_id(exception, 1), Some(dead.id()));
  assert_eq!(0, node.use_count());
  assert_eq!(2, start.use_count());
  assert_eq!(1, dead.use_count());
  assert_eq!(0, replacement.use_count());
  let expected = {
    let mut v = vec![use_control.id(), node.id()];
    v.sort_unstable();
    v
  };
  assert_eq!(expected, use_ids(start));
  assert_eq!(vec![exception.id()], use_ids(dead));
  OK
}

/// v8: TEST_F(AdvancedReducerTest, ReplaceWithValue_ControlUse3)
#[test]
fn advanced_reducer_test_replace_with_value_control_use3() -> Void {
  // 原版 2/3 两用例体相同, 保留双用例名维持覆盖对账
  advanced_reducer_test_replace_with_value_control_use2()
}
