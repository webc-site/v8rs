#!/usr/bin/env bun
// v8/src/compiler/node-marker.cc + node-marker.h
// → v8-compiler-passes/src/node_marker.rs.skel
//
// 规则面: NodeMarkerBase Get/Set 完整语义 (mark_min_/mark_max_ 区间
// 平移 + DCHECK 链)、Mark 类型、NodeMarker<State> 泛型封装、
// "同时仅一 marker 有效" 约束。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/node-marker.h");
const cc = readV8("src/compiler/node-marker.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class NodeMarkerBase \{/, "NodeMarkerBase"],
  [h, /Mark mark = node->mark\(\);\n    if \(mark < mark_min_\) \{\n      return 0;\n    \}/, "Get 下界归零"],
  [h, /node->set_mark\(mark \+ mark_min_\);/, "Set 平移写回"],
  [h, /template <typename State>\nclass NodeMarker : public NodeMarkerBase \{/, "NodeMarker 模板"],
  [h, /return static_cast<State>\(NodeMarkerBase::Get\(node\)\);/, "Get 状态转换"],
  [h, /GraphReducer uses a NodeMarker, so individual Reducers cannot use a\n\/\/ NodeMarker\./, "单 marker 约束注释"],
  [cc, /NodeMarkerBase::NodeMarkerBase\(TFGraph\* graph, uint32_t num_states\)/, "cc 构造"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}

const body = `${banner("src/compiler/node-marker.js", "src/compiler/node-marker.h + src/compiler/node-marker.cc", "NodeMarker — 节点 O(1) 局部状态标记 (mark 区间平移)")}
//! NodeMarker: 每个 Node 存一个全局 mark; NodeMarker 通过
//! [mark_min_, mark_max_) 区间将 mark 平移成本地 State。
//! 约束: 同一图同时只允许用最新创建的 marker (GraphReducer 占用其一)。

#![allow(dead_code)]

/// v8: src/compiler/node-marker.h Mark (uint32 别名)
pub type Mark = u32;

/// v8: src/compiler/node-marker.h NodeMarkerBase
pub struct NodeMarkerBase {
  /// 原 mark_min_ — 本 marker 区间下界
  mark_min: Mark,
  /// 原 mark_max_ — 区间上界
  mark_max: Mark,
}

impl NodeMarkerBase {
  /// v8: src/compiler/node-marker.cc NodeMarkerBase::NodeMarkerBase
  /// 原文: mark_min_ = graph->mark_max_; mark_max_ = graph->mark_max_(+num_states)
  /// 并推进 graph->mark_max_ (独占区间)
  pub fn new(graph: &mut crate::graph::TFGraph, num_states: u32) -> Self {
    let mark_min = graph.mark_max();
    let mark_max = mark_min + num_states;
    graph.set_mark_max(mark_max);
    Self { mark_min, mark_max }
  }

  /// v8: src/compiler/node-marker.h NodeMarkerBase::Get
  /// 原文: mark < mark_min_ → 0 (视为未标); 否则 mark - mark_min_
  pub fn get(&self, node: &crate::node::Node) -> Mark {
    let mark = node.mark();
    if mark < self.mark_min {
      return 0;
    }
    debug_assert!(mark < self.mark_max);
    mark - self.mark_min
  }

  /// v8: src/compiler/node-marker.h NodeMarkerBase::Set
  /// 原文: node->set_mark(mark + mark_min_) (带双 DCHECK)
  pub fn set(&self, node: &mut crate::node::Node, mark: Mark) {
    debug_assert!(mark < self.mark_max - self.mark_min);
    debug_assert!(node.mark() < self.mark_max);
    node.set_mark(mark + self.mark_min);
  }
}

/// v8: src/compiler/node-marker.h NodeMarker<State>
/// 初始化即概念性全零 (Get 未标 → State 0)
pub struct NodeMarker<State> {
  /// 原 NodeMarkerBase 部分
  pub base: NodeMarkerBase,
  _marker: std::marker::PhantomData<State>,
}

impl<State: From<Mark> + Into<Mark>> NodeMarker<State> {
  /// v8: src/compiler/node-marker.h NodeMarker::NodeMarker
  pub fn new(graph: &mut crate::graph::TFGraph, num_states: u32) -> Self {
    Self { base: NodeMarkerBase::new(graph, num_states), _marker: std::marker::PhantomData }
  }

  /// v8: src/compiler/node-marker.h NodeMarker::Get
  /// 原文: static_cast<State>(NodeMarkerBase::Get(node))
  pub fn get(&self, node: &crate::node::Node) -> State {
    State::from(self.base.get(node))
  }

  /// v8: src/compiler/node-marker.h NodeMarker::Set
  /// 原文: NodeMarkerBase::Set(node, static_cast<Mark>(state))
  pub fn set(&self, node: &mut crate::node::Node, state: State) {
    self.base.set(node, state.into());
  }
}
`;

const p = writeSrc("v8-compiler-passes", "node_marker.rs.skel", body);
console.log(`written ${p}\n方法=5 (Base3+Marker2) 区间平移语义=完整 单marker约束=1`);
