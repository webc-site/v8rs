#!/usr/bin/env bun
// tool/src/compiler/node-observer.js
// v8/src/compiler/node-observer.h + node-observer.cc
// → v8-compiler-passes/src/node_observer.rs.skel
// %ObserveNode 内建的基础设施: 解析 Observation 枚举 / ObservableNodeState
// 三元组相等语义 / NodeObserver 回调签名 / ObserveNodeManager 管理面。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/node-observer.h");
const cc = readV8("src/compiler/node-observer.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`node-observer: ${msg}`);
};

for (const [s, name] of [
  ["class ObservableNodeState {", "ObservableNodeState"],
  ["enum class Observation {", "Observation 枚举"],
  ["class NodeObserver : public ZoneObject {", "NodeObserver"],
  ["class ObserveNodeManager : public ZoneObject {", "ObserveNodeManager"],
  ["struct ObserveNodeInfo {", "ObserveNodeInfo"],
  ["void ObserveNodeManager::StartObserving(Node* node, NodeObserver* observer)", "cc StartObserving"],
  ["void ObserveNodeManager::OnNodeChanged(const char* reducer_name,", "cc OnNodeChanged"],
  ["ObservableNodeState::ObservableNodeState(const Node* node, Zone* zone)", "cc ObservableNodeState ctor"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// —— Observation 枚举成员 ——
const obsBody = h.slice(h.indexOf("enum class Observation {"), h.indexOf("};", h.indexOf("enum class Observation")));
const observations = [...obsBody.matchAll(/k(\w+),/g)].map((m) => m[1]);
need(observations.length === 2 && observations.includes("Continue") && observations.includes("Stop"),
  `Observation 成员 ${observations}`);

// —— NodeObserver 虚回调签名 ——
const callbacks = [...h.matchAll(/virtual Observation (\w+)\(([\s\S]*?)\)\s*\{/g)].map((m) => ({
  name: m[1],
  args: m[2].replace(/\s+/g, " ").trim(),
}));
need(callbacks.length === 2, `回调数 ${callbacks.length}`);
need(callbacks.some((c) => c.name === "OnNodeCreated" && c.args.includes("const Node* node")), "OnNodeCreated 签名");
need(callbacks.some((c) => c.name === "OnNodeChanged" && c.args.includes("const char* reducer_name") && c.args.includes("const ObservableNodeState& old_state")), "OnNodeChanged 签名");

// —— ObservableNodeState 字段 ——
const stateFields = [...h.matchAll(/^\s{2}(uint32_t|const Operator\*|Type) (id_|op_|type_);/gm)];
need(stateFields.length === 3, `state 字段 ${stateFields.length}`);

// —— cc 定义对数 ——
const ccDefs = ["ObservableNodeState::ObservableNodeState", "ObserveNodeManager::StartObserving", "ObserveNodeManager::OnNodeChanged"];
for (const d of ccDefs) need(cc.includes(d), `cc 缺 ${d}`);

const body = `${banner("src/compiler/node-observer.js", "src/compiler/node-observer.h + node-observer.cc", "%ObserveNode 观察: 节点创建/变更回调基础设施")}
//! node-observer — %ObserveNode(expr) 内建的观察面
//!
//! 无副作用语义; 观察点: 节点创建 / operator 或 type 变更 / 被替换。
//! ObservableNodeState 以 (id, op, type) 三元组判等 (operator== 全等才等)。

/// 原版 NodeObserver::Observation — 回调后是否继续观察
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Observation {
  /// kContinue — 继续观察后续变更
  Continue,
  /// kStop — 停止观察
  Stop,
}

/// 原版 ObservableNodeState — 被观察节点的状态快照 (id + op + type 三元组)
pub struct ObservableNodeState {
  /// id_
  pub id: u32,
  /// op_ (变更前 operator)
  pub op: *const Operator,
  /// type_
  pub type_: Type,
}

impl PartialEq for ObservableNodeState {
  /// 原版 operator== — id/op/type 三元组全等
  /// v8: src/compiler/node-observer.h operator==(ObservableNodeState, ObservableNodeState)
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id && self.op == other.op && self.type_ == other.type_
  }
}

impl ObservableNodeState {
  /// 原版 opcode() — op_->opcode()
  pub fn opcode(&self) -> i16 {
    todo!("v8: src/compiler/node-observer.h ObservableNodeState::opcode")
  }
}

/// 原版 NodeObserver — 观察回调 trait (ZoneObject; has_observed_changes 原子位)
pub trait NodeObserver {
  /// 原版 OnNodeCreated — 节点创建回调 (默认 kContinue)
  /// v8: src/compiler/node-observer.h NodeObserver::OnNodeCreated
  fn on_node_created(&mut self, node: &Node) -> Observation {
    Observation::Continue
  }

  /// 原版 OnNodeChanged — operator/type 变更回调 (默认 kContinue)
  /// v8: src/compiler/node-observer.h NodeObserver::OnNodeChanged
  fn on_node_changed(&mut self, reducer_name: &str, node: &Node, old_state: &ObservableNodeState) -> Observation {
    Observation::Continue
  }

  /// 原版 set_has_observed_changes / has_observed_changes
  /// v8: src/compiler/node-observer.h NodeObserver::set_has_observed_changes
  fn has_observed_changes(&self) -> bool;
}

/// 原版 NodeObservation — (observer, state) 对
pub struct NodeObservation<'n> {
  /// observer
  pub observer: &'n mut dyn NodeObserver,
  /// state
  pub state: ObservableNodeState,
}

/// 原版 ObserveNodeManager — NodeId → NodeObservation 管理器
pub struct ObserveNodeManager<'z, 'n> {
  /// observations_ — ZoneMap<NodeId, NodeObservation*>
  pub observations: ZoneMap<NodeId, NodeObservation<'n>>,
  /// zone_
  pub zone: &'z Zone<'z>,
}

impl ObserveNodeManager<'_, '_> {
  /// 原版 StartObserving — 登记观察点
  /// v8: src/compiler/node-observer.cc ObserveNodeManager::StartObserving
  pub fn start_observing(&mut self, node: &Node, observer: &mut dyn NodeObserver) {
    todo!("v8: src/compiler/node-observer.cc ObserveNodeManager::StartObserving")
  }

  /// 原版 OnNodeChanged — 旧节点状态对比新节点, 触发回调
  /// v8: src/compiler/node-observer.cc ObserveNodeManager::OnNodeChanged
  pub fn on_node_changed(&mut self, reducer_name: &str, old_node: &Node, new_node: &Node) {
    todo!("v8: src/compiler/node-observer.cc ObserveNodeManager::OnNodeChanged")
  }
}

/// 原版 ObserveNodeInfo — (manager, observer) 可空对 (编译信息携带)
pub struct ObserveNodeInfo<'m, 'n> {
  /// observe_node_manager
  pub observe_node_manager: Option<&'m mut ObserveNodeManager<'m, 'n>>,
  /// node_observer
  pub node_observer: Option<&'n mut dyn NodeObserver>,
}

impl ObserveNodeInfo<'_, '_> {
  /// 原版 StartObserving — manager 在位时转发登记
  /// v8: src/compiler/node-observer.h ObserveNodeInfo::StartObserving
  pub fn start_observing(&mut self, node: &Node) {
    if let Some(m) = self.observe_node_manager.as_deref_mut() {
      if let Some(o) = self.node_observer.as_deref_mut() {
        m.start_observing(node, o);
      }
    }
  }
}
`;

const p = writeSrc("v8-compiler-passes", "node_observer.rs.skel", body);
console.log(`written ${p}\nobservation=${observations.length} callbacks=${callbacks.length} state_fields=${stateFields.length} cc_defs=${ccDefs.length}`);
