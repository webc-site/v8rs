//! NodeMarker — 节点 O(1) 局部状态标记 (mark 区间平移)
//!
//! v8: src/compiler/node-marker.h + node-marker.cc 1:1。
//! 每个 Node 只有一个全局 mark 域; NodeMarker 以 [mark_min, mark_max)
//! 区间把全局 mark 平移为本地 State。区间经 GraphHandle 游标独占分配。
//! 约束 (原版注释): 同一图同一时刻只允许使用最新创建的 marker;
//! GraphReducer 占用其一, 单个 Reducer 不能再建 NodeMarker。

use std::marker::PhantomData;

use v8_compiler_graph::{Mark, Node};

use crate::graph_handle::GraphHandle;

/// 本地 State 与全局 Mark 的双向平移 (原版 static_cast<State>)
pub trait MarkerState: Copy + PartialEq + Eq {
  /// 原版 static_cast<State>(mark): 区间值域由构造保证
  fn from_mark(mark: Mark) -> Self;
  /// 原版 static_cast<Mark>(state)
  fn to_mark(self) -> Mark;
}

/// v8: src/compiler/node-marker.h NodeMarkerBase
pub struct NodeMarkerBase {
  /// 原 mark_min_ — 本 marker 区间下界
  mark_min: Mark,
  /// 原 mark_max_ — 区间上界
  mark_max: Mark,
}

impl NodeMarkerBase {
  /// v8: src/compiler/node-marker.cc NodeMarkerBase::NodeMarkerBase
  /// 原文: mark_min_ = graph->mark_max_; mark_max_ = graph->mark_max_ += num_states
  pub fn new(handle: &GraphHandle<'_, '_, '_>, num_states: Mark) -> Self {
    debug_assert!(num_states != 0, "num_states 为零是用户错误");
    let mark_min = handle.alloc_mark_range(num_states);
    let mark_max = mark_min + num_states;
    debug_assert!(mark_min < mark_max, "区间回绕");
    Self { mark_min, mark_max }
  }

  /// v8: src/compiler/node-marker.h NodeMarkerBase::Get
  /// 原文: mark < mark_min_ → 0; 否则 mark - mark_min_
  pub fn get(&self, node: &Node) -> Mark {
    let mark = node.mark();
    if mark < self.mark_min {
      return 0;
    }
    debug_assert!(mark < self.mark_max);
    mark - self.mark_min
  }

  /// v8: src/compiler/node-marker.h NodeMarkerBase::Set
  /// 原文: DCHECK_LT(mark, mark_max_ - mark_min_); node->set_mark(mark + mark_min_)
  /// (冻结层 Node::set_mark 经内部可变性取 &self)
  pub fn set(&self, node: &Node, mark: Mark) {
    debug_assert!(mark < self.mark_max - self.mark_min);
    debug_assert!(node.mark() < self.mark_max);
    node.set_mark(mark + self.mark_min);
  }
}

/// v8: src/compiler/node-marker.h NodeMarker<State>
/// 初始化即概念性全零 (Get 未标 → State 0)
pub struct NodeMarker<S: MarkerState> {
  /// 原 NodeMarkerBase 继承部分
  pub base: NodeMarkerBase,
  _state: PhantomData<S>,
}

impl<S: MarkerState> NodeMarker<S> {
  /// v8: src/compiler/node-marker.h NodeMarker::NodeMarker
  pub fn new(handle: &GraphHandle<'_, '_, '_>, num_states: Mark) -> Self {
    Self {
      base: NodeMarkerBase::new(handle, num_states),
      _state: PhantomData,
    }
  }

  /// v8: src/compiler/node-marker.h NodeMarker::Get
  pub fn get(&self, node: &Node) -> S {
    S::from_mark(self.base.get(node))
  }

  /// v8: src/compiler/node-marker.h NodeMarker::Set
  pub fn set(&self, node: &Node, state: S) {
    self.base.set(node, state.to_mark());
  }
}
