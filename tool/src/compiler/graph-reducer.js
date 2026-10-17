#!/usr/bin/env bun
// v8/src/compiler/graph-reducer.cc + graph-reducer.h
// → v8-compiler-passes/src/graph_reducer.rs.skel
//
// 规则面: NodeId 别名、Decision/State 枚举、Reduction 值语义
// (FollowedBy 完整迁)、Reducer/AdvancedReducer::Editor 签名、
// GraphReducer 字段与 14 方法; Recurse/Revisit 队列机制锚。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/graph-reducer.h");
const cc = readV8("src/compiler/graph-reducer.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /using NodeId = uint32_t;/, "NodeId"],
  [h, /enum class Decision : uint8_t \{ kUnknown, kTrue, kFalse \};/, "Decision"],
  [h, /class Reduction final \{/, "Reduction"],
  [h, /Reduction FollowedBy\(Reduction next\) const \{/, "FollowedBy"],
  [h, /class V8_EXPORT_PRIVATE Reducer \{/, "Reducer"],
  [h, /class AdvancedReducer : public Reducer \{/, "AdvancedReducer"],
  [h, /class Editor \{/, "Editor"],
  [h, /void RelaxEffectsAndControls\(Node\* node\) \{/, "RelaxEffectsAndControls"],
  [h, /class V8_EXPORT_PRIVATE GraphReducer/, "GraphReducer"],
  [cc, /void GraphReducer::ReduceTop\(\) \{/, "ReduceTop 实现"],
  [cc, /bool GraphReducer::Recurse\(Node\* node\) \{/, "Recurse 实现"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}
// FollowedBy 语义
if (!/if \(next\.Changed\(\)\) return next;\n    return \*this;/.test(h)) throw new Error("FollowedBy 语义漂移");
// cc 方法面
const ccMethods = [...cc.matchAll(/^(?:Reduction|void|bool|GraphReducer) GraphReducer::(\w+)\(/gm)].map((m) => m[1]);
const expect = ["ReduceNode", "ReduceGraph", "Reduce", "ReduceTop", "Replace",
  "ReplaceWithValue", "Recurse", "Revisit", "Pop", "Push"];
const missing = expect.filter((m) => !ccMethods.includes(m));
if (missing.length) throw new Error(`cc 缺实现: ${missing}`);
// State 枚举 (cc 中 forward declare 在 h: enum class State : uint8_t;)
const st = cc.match(/enum class GraphReducer::State : uint8_t \{([\s\S]*?)\};/);
if (!st) throw new Error("State 枚举定义缺失 (cc)");
const states = [...st[1].matchAll(/k(\w+)/g)].map((m) => m[1]);

const body = `${banner("src/compiler/graph-reducer.js", "src/compiler/graph-reducer.h + src/compiler/graph-reducer.cc", "GraphReducer — 迭代图归约机 (Reducer 体系 + revisit/stack 驱动)")}
//! GraphReducer: 栈驱动自底向上归约, revisit 队列回访, Editor 提供
//! 图编辑原语。
//!
//! 规则面: 枚举 2 + Reduction 值语义 + 方法面 ${expect.length}。
//! 语义面 (推迟): ReduceTop 的输入遍历循环 (cc 主循环)。

#![allow(dead_code, unused_variables, non_camel_case_types)]

/// v8: src/compiler/graph-reducer.h NodeId
pub type NodeId = u32;

/// v8: src/compiler/graph-reducer.h Decision
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Decision {
  /// kUnknown
  Unknown,
  /// kTrue
  True,
  /// kFalse
  False,
}

/// v8: src/compiler/graph-reducer.h Reduction
/// 归约结果: replacement 为 null 即 NoChange
#[derive(Clone, Copy)]
pub struct Reduction {
  replacement: Option<crate::node::Node>,
}

impl Reduction {
  /// v8: src/compiler/graph-reducer.h Reduction::Reduction (默认 nullptr)
  pub const fn new(replacement: Option<crate::node::Node>) -> Self {
    Self { replacement }
  }
  /// v8: src/compiler/graph-reducer.h Reduction::replacement
  pub const fn replacement(&self) -> Option<crate::node::Node> {
    self.replacement
  }
  /// v8: src/compiler/graph-reducer.h Reduction::Changed
  pub const fn changed(&self) -> bool {
    self.replacement.is_some()
  }
  /// v8: src/compiler/graph-reducer.h Reduction::FollowedBy
  /// 原文: if (next.Changed()) return next; return *this;
  pub const fn followed_by(&self, next: Reduction) -> Reduction {
    if next.changed() { next } else { *self }
  }

  /// v8: src/compiler/graph-reducer.h Reducer::NoChange
  pub const fn NoChange() -> Self {
    Self::new(None)
  }
  /// v8: src/compiler/graph-reducer.h Reducer::Replace
  pub const fn Replace(node: crate::node::Node) -> Self {
    Self::new(Some(node))
  }
  /// v8: src/compiler/graph-reducer.h Reducer::Changed (与 Replace 同构)
  pub const fn Changed(node: crate::node::Node) -> Self {
    Self::new(Some(node))
  }
}

/// v8: src/compiler/graph-reducer.h AdvancedReducer::Editor
/// 图编辑原语: Replace×2 / Revisit / ReplaceWithValue
pub trait Editor {
  /// v8: src/compiler/graph-reducer.h Editor::Replace
  fn replace(&mut self, node: crate::node::Node, replacement: crate::node::Node);
  /// v8: src/compiler/graph-reducer.h Editor::Replace (max_id 版)
  fn replace_max(&mut self, node: crate::node::Node, replacement: crate::node::Node, max_id: NodeId);
  /// v8: src/compiler/graph-reducer.h Editor::Revisit
  fn revisit(&mut self, node: crate::node::Node);
  /// v8: src/compiler/graph-reducer.h Editor::ReplaceWithValue
  /// effect 为 null 时用 node 的 effect 输入; 控制用假定不可抛
  fn replace_with_value(&mut self, node: crate::node::Node, value: crate::node::Node, effect: Option<crate::node::Node>, control: Option<crate::node::Node>);
}

/// v8: src/compiler/graph-reducer.h AdvancedReducer (节选助手)
pub struct AdvancedReducer<'a> {
  /// 原 editor_
  pub editor: &'a mut dyn Editor,
}

impl AdvancedReducer<'_> {
  /// v8: src/compiler/graph-reducer.h AdvancedReducer::RelaxEffectsAndControls
  /// 原文: ReplaceWithValue(node, node, nullptr, nullptr)
  pub fn relax_effects_and_controls(&mut self, node: crate::node::Node) {
    self.editor.replace_with_value(node, node, None, None);
  }
  /// v8: src/compiler/graph-reducer.h AdvancedReducer::RelaxControls
  /// 原文: ReplaceWithValue(node, node, node, control)
  pub fn relax_controls(&mut self, node: crate::node::Node, control: Option<crate::node::Node>) {
    self.editor.replace_with_value(node, node, Some(node), control);
  }
  /// v8: src/compiler/graph-reducer.h AdvancedReducer::MergeControlToEnd
  /// 原文: MergeControlToEnd + Revisit(graph->end())
  pub fn merge_control_to_end(&mut self, graph: &mut crate::graph::TFGraph, common: &mut crate::common_operator::CommonOperatorBuilder, node: crate::node::Node) {
    crate::node_properties::merge_control_to_end(graph, common, node);
    self.editor.revisit(graph.end());
  }
}

/// v8: src/compiler/graph-reducer.cc GraphReducer::State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum State {
  /// ${states.map((s) => `k${s}`).join(" / ")}
  ${states.map((s) => `${s[0].toUpperCase() + s.slice(1)},`).join("\n  ")}
}

/// v8: src/compiler/graph-reducer.h GraphReducer::NodeState
struct NodeState {
  node: crate::node::Node,
  input_index: i32,
}

/// v8: src/compiler/graph-reducer.h GraphReducer
pub struct GraphReducer {
  /// 原 graph_
  pub graph: *mut crate::graph::TFGraph,
  /// 原 dead_ (构造默认参 nullptr)
  pub dead: Option<crate::node::Node>,
  /// 原 state_ — NodeMarker<State>
  pub state: crate::node_marker::NodeMarkerMarker,
  /// 原 reducers_
  pub reducers: Vec<*mut dyn ReducerTrait>,
  /// 原 revisit_ — ZoneQueue
  pub revisit: std::collections::VecDeque<crate::node::Node>,
  /// 原 stack_ — ZoneStack<NodeState>
  pub stack: Vec<NodeState>,
}

/// Reducer trait 面 (Reduce/Finalize/reducer_name)
pub trait ReducerTrait {
  /// v8: src/compiler/graph-reducer.h Reducer::reducer_name
  fn reducer_name(&self) -> &'static str;
  /// v8: src/compiler/graph-reducer.h Reducer::Reduce
  fn reduce(&mut self, node: &crate::node::Node) -> Reduction;
  /// v8: src/compiler/graph-reducer.h Reducer::Finalize
  fn finalize(&mut self) {}
}

impl GraphReducer {
  /// v8: src/compiler/graph-reducer.cc GraphReducer::AddReducer
  pub fn add_reducer(&mut self, reducer: *mut dyn ReducerTrait) {
    self.reducers.push(reducer);
  }
  /// v8: src/compiler/graph-reducer.cc GraphReducer::ReduceNode
  pub fn reduce_node(&mut self, node: crate::node::Node) {
    todo!("Recurse(node) + 循环 ReduceTop")
  }
  /// v8: src/compiler/graph-reducer.cc GraphReducer::ReduceGraph
  /// 原文: end 为根 ReduceNode + revisit 队列清空
  pub fn reduce_graph(&mut self) {
    todo!("graph.end() 归约 + drain revisit")
  }
  /// v8: src/compiler/graph-reducer.cc GraphReducer::Reduce (单节点)
  fn reduce(&mut self, node: &crate::node::Node) -> Reduction {
    todo!("逐 reducer 依次 Reduce + FollowedBy 链")
  }
  /// v8: src/compiler/graph-reducer.cc GraphReducer::ReduceTop
  fn reduce_top(&mut self) {
    todo!("栈顶输入遍历/归约/回写 (主循环)")
  }
  /// v8: src/compiler/graph-reducer.cc GraphReducer::Replace (Editor 实现)
  fn replace(&mut self, node: crate::node::Node, replacement: crate::node::Node) {
    todo!("全 use 改写")
  }
  /// v8: src/compiler/graph-reducer.cc GraphReducer::Replace (max_id 版)
  fn replace_max(&mut self, node: crate::node::Node, replacement: crate::node::Node, max_id: NodeId) {
    let _ = max_id;
    todo!("id 上界过滤改写")
  }
  /// v8: src/compiler/graph-reducer.cc GraphReducer::ReplaceWithValue
  fn replace_with_value(&mut self, node: crate::node::Node, value: crate::node::Node, effect: Option<crate::node::Node>, control: Option<crate::node::Node>) {
    let _ = (value, effect, control);
    todo!("值/效应/控制三路 use 分拆改写")
  }
  /// v8: src/compiler/graph-reducer.cc GraphReducer::Pop
  fn pop(&mut self) {
    self.stack.pop();
  }
  /// v8: src/compiler/graph-reducer.cc GraphReducer::Push
  fn push(&mut self, node: crate::node::Node) {
    self.stack.push(NodeState { node, input_index: 0 });
  }
  /// v8: src/compiler/graph-reducer.cc GraphReducer::Recurse
  /// 原文: 状态 kUnvisited → kOnStack 入栈 true; kVisited → false
  fn recurse(&mut self, node: crate::node::Node) -> bool {
    todo!("状态机分派")
  }
  /// v8: src/compiler/graph-reducer.cc GraphReducer::Revisit
  /// 原文: kVisited → kRevisit 入队
  fn revisit(&mut self, node: crate::node::Node) {
    todo!("回访队列")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "graph_reducer.rs.skel", body);
console.log(`written ${p}\nState=${states.length} 枚举=2 方法面=${expect.length} Reduction值语义=5`);
