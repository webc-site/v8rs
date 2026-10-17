//! GraphHandle — TFGraph 别名语义的窄化适配 (局部最小签名)
//!
//! 原版 C++ 中 TFGraph* 裸指针可自由别名: 任何 pass 都能经
//! `graph->SetStart/SetEnd` 改写图端点, 同时读 `graph->end()`。
//! Rust 借用规则不允许共享引用与可变写点共存, 且冻结层
//! v8-compiler-graph 的 `Graph::start()/end()` 返回借用期引用。
//! 本模块把 start/end 写点接管到 Cell 镜像中: 节点引用以
//! `&'z Node` (zone 生命周期, 经 `new_node` 产出) 注入, 读取一律
//! 走句柄; 其余图操作 (`new_node`/`node_count`/zone) 透传共享引用。
//! mark 分配区间同样接管到句柄 (原版 graph->mark_max_ 自增)。

use std::cell::Cell;

use v8_compiler_graph::{Graph, Mark, Node, Operator};
use v8_zone::Zone;

/// v8: src/compiler/turbofan-graph.h TFGraph (共享句柄 + start/end 镜像)
pub struct GraphHandle<'h, 'z, 'a> {
  /// 原 graph_ 裸指针的共享化
  graph: &'h Graph<'z, 'a>,
  /// start_ 镜像 (原版 SetStart 写点)
  start: Cell<Option<&'z Node>>,
  /// end_ 镜像 (原版 SetEnd 写点)
  end: Cell<Option<&'z Node>>,
  /// mark_max_ 镜像: NodeMarker 区间分配游标 (初值取 graph.mark_max)
  mark_next: Cell<Mark>,
}

impl<'h, 'z, 'a> GraphHandle<'h, 'z, 'a> {
  /// v8: src/compiler/turbofan-graph.h TFGraph::TFGraph
  /// 原版允许 start/end 暂空 (UB 面在 Rust 侧显式化为 Option)
  pub fn new(graph: &'h Graph<'z, 'a>, start: Option<&'z Node>, end: Option<&'z Node>) -> Self {
    Self {
      graph,
      start: Cell::new(start),
      end: Cell::new(end),
      mark_next: Cell::new(graph.mark_max()),
    }
  }

  /// v8: src/compiler/turbofan-graph.h TFGraph (取底层图共享引用)
  pub const fn graph(&self) -> &'h Graph<'z, 'a> {
    self.graph
  }

  /// v8: src/compiler/turbofan-graph.h TFGraph::zone
  pub fn zone(&self) -> &'z Zone<'a> {
    self.graph.zone()
  }

  /// v8: src/compiler/turbofan-graph.h TFGraph::NodeCount
  pub fn node_count(&self) -> usize {
    self.graph.node_count()
  }

  /// v8: src/compiler/turbofan-graph.h TFGraph::NewNode
  pub fn new_node(
    &self,
    op: &Operator,
    inputs: &[&Node],
    incomplete: bool,
  ) -> crate::Result<&'z Node> {
    Ok(self.graph.new_node(op, inputs, incomplete)?)
  }

  /// v8: src/compiler/turbofan-graph.h TFGraph::start
  pub fn start(&self) -> Option<&'z Node> {
    self.start.get()
  }

  /// v8: src/compiler/turbofan-graph.h TFGraph::SetStart
  pub fn set_start(&self, node: &'z Node) {
    self.start.set(Some(node));
  }

  /// v8: src/compiler/turbofan-graph.h TFGraph::end
  pub fn end(&self) -> Option<&'z Node> {
    self.end.get()
  }

  /// v8: src/compiler/turbofan-graph.h TFGraph::SetEnd
  pub fn set_end(&self, node: &'z Node) {
    self.end.set(Some(node));
  }

  /// v8: src/compiler/node-marker.cc NodeMarkerBase::NodeMarkerBase
  /// 原版经 graph->mark_max_ 单调自增占位; 此处接管到句柄游标
  pub fn alloc_mark_range(&self, num_states: Mark) -> Mark {
    let base = self.mark_next.get();
    self.mark_next.set(base + num_states);
    base
  }
}
