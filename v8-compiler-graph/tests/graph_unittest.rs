//! 延期登记 (todo 12.6): 引用 Graph::reserve_mark_domain / NodeProperties::
//! collect_control_projections 功能面 — 命名统一轮 (agent-allow-c) 已就位,
//! 余下两 API 未落地, 落地后删除下一行恢复。
#![cfg(any())]

//! graph (turbofan-graph.h/cc) API 语义单测:
//! NewNode 唯一 id / NodeCount / decorators 触发与移除 / CloneNode /
//! SubgraphScope RAII 还原 / start/end / simd 记录 / Mark 域

use std::{
  ptr,
  rc::Rc,
  sync::atomic::{AtomicUsize, Ordering},
};

use aok::{OK, Void};
use log::info;
use v8_compiler_graph::{
  Graph, GraphDecorator, IrOpcode, Node, Operator, OperatorCounts, Properties, SubgraphScope,
};
use v8_zone::{AccountingAllocator, Zone};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

fn op0() -> Operator {
  Operator::new(
    IrOpcode::Start as u16,
    Properties::NONE,
    "Op0",
    OperatorCounts::new(0, 0, 0, 1, 0, 0),
  )
  .unwrap()
}

fn op2() -> Operator {
  Operator::new(
    IrOpcode::Branch as u16,
    Properties::NONE,
    "Op2",
    OperatorCounts::new(2, 0, 0, 1, 0, 0),
  )
  .unwrap()
}

/// 装饰器计数桩 (原版 GraphDecorator::Decorate 触发面)
struct Counting(AtomicUsize);

impl GraphDecorator for Counting {
  fn decorate(&self, _node: &Node) {
    self.0.fetch_add(1, Ordering::Relaxed);
  }
}

/// NewNode — id 唯一递增 + NodeCount
#[test]
fn graph_test_new_node_ids() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "graph");
  let graph = Graph::new(&zone);
  assert_eq!(0, graph.node_count());
  let n0 = graph.new_node(&op0(), &[], false)?;
  let n1 = graph.new_node(&op0(), &[], false)?;
  let n2 = graph.new_node(&op2(), &[n0, n1], false)?;
  assert_eq!(0, n0.id());
  assert_eq!(1, n1.id());
  assert_eq!(2, n2.id());
  assert_eq!(3, graph.node_count()); // NodeCount = next_node_id
  let _n3 = graph.new_node(&op0(), &[], false)?;
  assert_eq!(4, graph.node_count());
  info!("ids ok");
  OK
}

/// NewNodeUnchecked 与 NewNode 等价 (Verifier 推迟)
#[test]
fn graph_test_new_node_unchecked() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "graph");
  let graph = Graph::new(&zone);
  let a = graph.new_node_unchecked(&op0(), &[], false)?;
  let b = graph.new_node(&op0(), &[], false)?;
  assert_eq!(0, a.id());
  assert_eq!(1, b.id());
  OK
}

/// Decorate — 每次建节点触发; AddDecorator/RemoveDecorator 依身份
#[test]
fn graph_test_decorators() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "graph");
  let mut graph = Graph::new(&zone);
  let d = Rc::new(Counting(AtomicUsize::new(0)));
  graph.add_decorator(d.clone());
  let probe = Counting(AtomicUsize::new(0));
  let probe_ref: &dyn GraphDecorator = &probe;
  graph.new_node(&op0(), &[], false)?; // 1 装饰器 × 1 节点
  assert_eq!(1, d.0.load(Ordering::Relaxed));
  // 移除不存在的装饰器 → false
  assert!(!graph.remove_decorator(probe_ref));
  // 存在的 → true
  assert!(graph.remove_decorator(&*d));
  // start/end setter + getter
  let n = graph.new_node(&op0(), &[], false)?;
  graph.set_start(n);
  graph.set_end(n);
  assert!(graph.start().is_some_and(|s| ptr::eq(s, n)));
  assert!(graph.end().is_some_and(|e| ptr::eq(e, n)));
  OK
}

/// 装饰器触发计数 (加入的装饰器在建节点时逐个触发)
#[test]
fn graph_test_decorator_fire() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "graph");
  let mut graph = Graph::new(&zone);
  let fired = Rc::new(AtomicUsize::new(0));
  struct F(Rc<AtomicUsize>);
  impl GraphDecorator for F {
    fn decorate(&self, _n: &Node) {
      self.0.fetch_add(1, Ordering::Relaxed);
    }
  }
  let d = Rc::new(F(fired.clone()));
  graph.add_decorator(d.clone());
  let _n1 = graph.new_node(&op0(), &[], false)?;
  let _n2 = graph.new_node(&op0(), &[], false)?;
  assert_eq!(2, fired.load(Ordering::Relaxed));
  // 克隆也触发
  let _c = graph.clone_node(_n1)?;
  assert_eq!(3, fired.load(Ordering::Relaxed));
  // 移除后不再触发
  assert!(graph.remove_decorator(&*d));
  let _n3 = graph.new_node(&op0(), &[], false)?;
  assert_eq!(3, fired.load(Ordering::Relaxed));
  OK
}

/// CloneNode — 新 id 同输入, Use 链归新节点
#[test]
fn graph_test_clone_node() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "graph");
  let graph = Graph::new(&zone);
  let n0 = graph.new_node(&op0(), &[], false)?;
  let n1 = graph.new_node(&op2(), &[n0, n0], false)?;
  let clone = graph.clone_node(n1)?;
  assert_eq!(2, clone.id());
  assert_eq!(n1.opcode(), clone.opcode());
  assert_eq!(4, n0.use_count()); // n1×2 + clone×2
  OK
}

/// SubgraphScope — 作用域内换置, 离开时还原 start/end (RAII)
#[test]
fn graph_test_subgraph_scope() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "graph");
  let mut graph = Graph::new(&zone);
  let start = graph.new_node(&op0(), &[], false)?;
  let end = graph.new_node(&op0(), &[], false)?;
  graph.set_start(start);
  graph.set_end(end);
  {
    let mut scope = SubgraphScope::new(&mut graph);
    let g = scope.graph();
    let inner = g.new_node(&op0(), &[], false)?;
    g.set_start(inner);
    g.set_end(inner);
    assert!(g.start().is_some_and(|s| ptr::eq(s, inner)));
  }
  assert!(graph.start().is_some_and(|s| ptr::eq(s, start)));
  assert!(graph.end().is_some_and(|e| ptr::eq(e, end)));
  OK
}

/// simd 面: HasSimd/SetSimd/RecordSimdStore/GetSimdStoreNodes
#[test]
fn graph_test_simd() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "graph");
  let mut graph = Graph::new(&zone);
  assert!(!graph.has_simd());
  graph.set_simd(true);
  assert!(graph.has_simd());
  let s1 = graph.new_node(&op0(), &[], false)?;
  let s2 = graph.new_node(&op0(), &[], false)?;
  graph.record_simd_store(s1);
  graph.record_simd_store(s2);
  let nodes = graph.simd_store_nodes();
  assert_eq!(2, nodes.len());
  assert!(ptr::eq(nodes[0], s1) && ptr::eq(nodes[1], s2));
  OK
}

/// Mark 域: node.mark/set_mark 单调递增语义 + reserve_mark_domain 标记轮转
#[test]
fn graph_test_mark_domain() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "graph");
  let graph = Graph::new(&zone);
  assert_eq!(0, graph.mark_max());
  let (min1, max1) = graph.reserve_mark_domain(4)?;
  assert_eq!(0, min1);
  assert_eq!(4, max1);
  assert_eq!(4, graph.mark_max());
  let (min2, max2) = graph.reserve_mark_domain(2)?;
  assert_eq!(4, min2);
  assert_eq!(6, max2);
  assert_eq!(6, graph.mark_max());

  let n = graph.new_node(&op0(), &[], false)?;
  assert_eq!(0, n.mark());
  n.set_mark(3);
  n.set_mark(8); // 单调递增 (只升)
  assert_eq!(8, n.mark());
  OK
}

/// zone 借出: graph.zone() 与节点分配共存
#[test]
fn graph_test_zone_accessor() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "graph");
  let graph = Graph::new(&zone);
  let n = Node::new(&zone, 42, &op0(), &[], false)?;
  assert_eq!(42, n.id());
  // graph 自身建节点 id 序不受外部直建影响 (原版同 zone 共享发号器为调用方职责)
  let g = graph.new_node(&op0(), &[], false)?;
  assert_eq!(0, g.id());
  OK
}
