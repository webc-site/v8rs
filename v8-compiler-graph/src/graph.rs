// 由 tool/src/compiler/turbofan-graph.js 从 v8/src/compiler/turbofan-graph.h + turbofan-graph.cc 生成; 修订就地改, 复跑覆盖

//! Graph (原版 TFGraph) — TurboFan 图属主: 借用 zone / start / end / 装饰器
//!
//! zone 为借用 (1:1 原版 TFGraph(Zone*)); 节点生命周期同 zone ('z)。
//! NewNode 的 Verifier::VerifyNode 校验推迟 (verifier 层), 入口对保留;
//! Print/AsRPO 可视化推迟; NodeMarker (mark_max 单调递增域) 第二梯队接管。

use std::{cell::Cell, ptr, rc::Rc};

use v8_zone::Zone;

use crate::{
  Error,
  node::{Mark, Node, NodeId},
  operator::Operator,
};

/// GraphDecorator — 节点创建行为扩展点 (原版 ZoneObject 虚基类)
pub trait GraphDecorator {
  /// 原版 Decorate(Node*) — 节点入图后回调 (可改节点/图状态; 原版非 const)
  fn decorate(&self, node: &Node);
}

/// Graph — 原版 TFGraph final
pub struct Graph<'z, 'a> {
  /// zone_ (借用; 原版 Zone*)
  zone: &'z Zone<'a>,
  /// start_
  start: *mut Node,
  /// end_
  end: *mut Node,
  /// mark_max_ — NodeMarker 单调递增标记域上界 (Cell: 原版非 const 写)
  mark_max: Cell<Mark>,
  /// next_node_id_ — 新节点 id 发号器 (Cell: NewNode 经 &self 发号)
  next_node_id: Cell<NodeId>,
  /// decorators_ (原版 ZoneVector<GraphDecorator*>; Rc 依数据指针身份增删)
  decorators: Vec<Rc<dyn GraphDecorator>>,
  /// has_simd_
  has_simd: bool,
  /// simd_stores_ (原版 ZoneVector<Node*>)
  simd_stores: Vec<*mut Node>,
}

impl<'z, 'a> Graph<'z, 'a> {
  /// 原版 TFGraph(Zone*)
  pub fn new(zone: &'z Zone<'a>) -> Self {
    Self {
      zone,
      start: ptr::null_mut(),
      end: ptr::null_mut(),
      mark_max: Cell::new(0),
      next_node_id: Cell::new(0),
      decorators: Vec::new(),
      has_simd: false,
      simd_stores: Vec::new(),
    }
  }

  /// 原版 zone()
  pub fn zone(&self) -> &'z Zone<'a> {
    self.zone
  }

  /// 原版 start()
  pub fn start(&self) -> Option<&'z Node> {
    // SAFETY: start 由 NewNode 落在所借 zone
    unsafe { self.start.as_ref() }
  }

  /// 原版 end()
  pub fn end(&self) -> Option<&'z Node> {
    // SAFETY: end 由 NewNode 落在所借 zone
    unsafe { self.end.as_ref() }
  }

  /// 原版 SetStart
  pub fn set_start(&mut self, start: &'z Node) {
    self.start = ptr::from_ref(start) as *mut Node;
  }

  /// 原版 SetEnd
  pub fn set_end(&mut self, end: &'z Node) {
    self.end = ptr::from_ref(end) as *mut Node;
  }

  /// 原版 NodeCount
  pub fn node_count(&self) -> usize {
    self.next_node_id.get() as usize
  }

  /// 原版 NewNodeUnchecked — 建 Node + 装饰 (Verifier 校验推迟)
  pub fn new_node_unchecked(
    &self,
    op: &Operator,
    inputs: &[&Node],
    incomplete: bool,
  ) -> Result<&'z Node, Error> {
    let id = self.next_node_id()?;
    let node = Node::new(self.zone, id, op, inputs, incomplete)?;
    self.decorate(node);
    Ok(node)
  }

  /// 原版 NewNode — 校验入口 (Verifier::VerifyNode 推迟, 现与 unchecked 等价)
  pub fn new_node(
    &self,
    op: &Operator,
    inputs: &[&Node],
    incomplete: bool,
  ) -> Result<&'z Node, Error> {
    self.new_node_unchecked(op, inputs, incomplete)
  }

  /// 原版 CloneNode — 同输入新 id 副本 + 装饰
  pub fn clone_node(&self, node: &Node) -> Result<&'z Node, Error> {
    let id = self.next_node_id()?;
    let clone = Node::clone_node(self.zone, id, node)?;
    self.decorate(clone);
    Ok(clone)
  }

  /// 原版 Decorate — 顺序触发装饰器
  pub fn decorate(&self, node: &Node) {
    for d in &self.decorators {
      d.decorate(node);
    }
  }

  /// 原版 AddDecorator — Rc 共享, 调用方持克隆即可身份删除
  pub fn add_decorator(&mut self, decorator: Rc<dyn GraphDecorator>) {
    self.decorators.push(decorator);
  }

  /// 原版 RemoveDecorator — 依数据指针身份移除, 返回是否命中
  pub fn remove_decorator(&mut self, decorator: &dyn GraphDecorator) -> bool {
    let key = ptr::from_ref(decorator) as *const ();
    let Some(pos) = self
      .decorators
      .iter()
      .position(|d| ptr::from_ref(d.as_ref() as &dyn GraphDecorator) as *const () == key)
    else {
      return false;
    };
    self.decorators.remove(pos);
    true
  }

  /// 原版 HasSimd
  pub fn has_simd(&self) -> bool {
    self.has_simd
  }

  /// 原版 SetSimd
  pub fn set_simd(&mut self, has_simd: bool) {
    self.has_simd = has_simd;
  }

  /// 原版 RecordSimdStore
  pub fn record_simd_store(&mut self, store: &Node) {
    self.simd_stores.push(ptr::from_ref(store) as *mut Node);
  }

  /// 原版 GetSimdStoreNodes
  pub fn simd_store_nodes(&self) -> Vec<&Node> {
    // SAFETY: 记录的节点均落在所借 zone
    self.simd_stores.iter().map(|p| unsafe { &**p }).collect()
  }

  /// mark_max 只读口 (NodeMarker 单调递增域上界; node-marker 层接管)
  pub fn mark_max(&self) -> Mark {
    self.mark_max.get()
  }

  /// 原版 NextNodeId — 发号器自增 (id 存 24 位域, 原版 DCHECK 不溢出)
  fn next_node_id(&self) -> Result<NodeId, Error> {
    let id = self.next_node_id.get();
    if id >= (1 << 24) - 1 {
      return Err(Error::NodeIdOverflow { id });
    }
    self.next_node_id.set(id + 1);
    Ok(id)
  }
}

/// 原版 TFGraph::SubgraphScope — 内联子图作用域 (离开时还原 start/end)
///
/// RAII: Drop 即 ~SubgraphScope; 作用域内经 `graph()` 换置 start/end
pub struct SubgraphScope<'g, 'z, 'a> {
  graph: &'g mut Graph<'z, 'a>,
  start: *mut Node,
  end: *mut Node,
}

impl<'g, 'z, 'a> SubgraphScope<'g, 'z, 'a> {
  /// 原版 SubgraphScope(TFGraph*) — 仅保存, 不改当前 start/end
  pub fn new(graph: &'g mut Graph<'z, 'a>) -> Self {
    Self {
      start: graph.start,
      end: graph.end,
      graph,
    }
  }

  /// 作用域内访问图 (换置 start/end 走此口)
  pub fn graph(&mut self) -> &mut Graph<'z, 'a> {
    self.graph
  }
}

impl Drop for SubgraphScope<'_, '_, '_> {
  fn drop(&mut self) {
    self.graph.start = self.start;
    self.graph.end = self.end;
  }
}
