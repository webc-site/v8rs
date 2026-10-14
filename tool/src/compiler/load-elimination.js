#!/usr/bin/env bun
// v8/src/compiler/load-elimination.cc + load-elimination.h
// → v8-compiler-passes/src/load_elimination.rs.skel
//
// 规则面: kMaxTrackedElements=8、AbstractElements 环形表 (next_index 取模)
// 语义、FieldInfo 相等四元组、Reduce 分派表 (~16 opcode)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/load-elimination.h");
const cc = readV8("src/compiler/load-elimination.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class V8_EXPORT_PRIVATE LoadElimination final/, "LoadElimination"],
  [h, /static const size_t kMaxTrackedElements = 8;/, "kMaxTrackedElements"],
  [h, /that->next_index_ = \(that->next_index_ \+ 1\) % arraysize\(elements_\);/, "环形索引"],
  [h, /name\.address\(\) == other\.name\.address\(\)/, "FieldInfo 句柄地址相等"],
  [cc, /Reduction LoadElimination::Reduce\(Node\* node\) \{/, "Reduce 实现"],
  [cc, /case IrOpcode::kLoadField:/, "分派 LoadField"],
  [cc, /case IrOpcode::kTransitionElementsKind:/, "分派 Transition"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}

// 分派表 (Reduce 主 switch)
const switchSrc = cc.slice(
  cc.indexOf("Reduction LoadElimination::Reduce(Node* node) {"),
  cc.indexOf("LoadElimination::AbstractState const* LoadElimination::ComputeLoopState"),
);
const cases = [...switchSrc.matchAll(/case IrOpcode::k(\w+):/g)].map((m) => m[1]);
if (cases.join() !== "MapGuard,CheckMaps,CompareMaps,EnsureWritableFastElements,MaybeGrowFastElements,TransitionElementsKind,TransitionElementsKindOrCheckMap,LoadField,StoreField,LoadElement,StoreElement,TransitionAndStoreElement,StoreTypedElement,EffectPhi,Dead,Start") {
  throw new Error(`分派表漂移: ${cases}`);
}
if (cases[0] !== "MapGuard" || !cases.includes("LoadField") || !cases.includes("StoreField") || !cases.includes("LoadElement") || !cases.includes("StoreTypedElement")) {
  throw new Error(`分派首/关键项漂移: ${cases}`);
}
// 内部抽象状态类清单
const stateClasses = [...h.matchAll(/^  class (\w+) final : public ZoneObject \{/gm)].map((m) => m[1]);
if (stateClasses.join() !== "AbstractElements,AbstractField,AbstractMaps,AbstractState,AbstractStateForEffectNodes") {
  throw new Error(`抽象状态类漂移: ${stateClasses}`);
}

const body = `${banner("src/compiler/load-elimination.js", "src/compiler/load-elimination.h + src/compiler/load-elimination.cc", "LoadElimination — Turbofan 层字段/元素/映射 load 消除 (三抽象状态)")}
//! LoadElimination: AbstractElements/AbstractFields/AbstractMaps 沿效应链
//! 传播, 冗余 load 直接替换。
//!
//! 规则面: 分派 ${cases.length} case + 三抽象状态 + kMaxTrackedElements。
//! 语义面 (推迟): 别名分析 (AliasStateInfo) 与 kill 规则。

#![allow(dead_code, unused_variables)]

/// v8: src/compiler/load-elimination.h kMaxTrackedElements
pub const K_MAX_TRACKED_ELEMENTS: usize = 8;

/// v8: src/compiler/load-elimination.h LoadElimination::AbstractElements::Element
#[derive(Clone, Copy)]
pub struct Element {
  /// 原 object / index / value / representation
  pub object: Option<crate::node::Node>,
  pub index: Option<crate::node::Node>,
  pub value: Option<crate::node::Node>,
  pub representation: crate::machine_type::MachineRepresentation,
}

/// v8: src/compiler/load-elimination.h LoadElimination::AbstractElements
/// 环形表: elements_[kMaxTrackedElements], next_index_ 循环覆写
/// (Extend: 写 next_index_ 后 (next_index_+1) % 8)
pub struct AbstractElements {
  /// 原 elements_
  pub elements: [Element; K_MAX_TRACKED_ELEMENTS],
  /// 原 next_index_
  pub next_index: usize,
}

impl AbstractElements {
  /// v8: src/compiler/load-elimination.h AbstractElements::Extend
  /// 原文: 克隆后写 next_index_ 槽, 指针环形推进 (FIFO 覆写)
  pub fn extend(&self, object: crate::node::Node, index: crate::node::Node, value: crate::node::Node, representation: crate::machine_type::MachineRepresentation, zone: &mut crate::zone::Zone) -> *const AbstractElements {
    let _ = zone;
    todo!("zone 克隆 + 环形写入")
  }
  /// v8: src/compiler/load-elimination.cc AbstractElements::Lookup
  pub fn lookup(&self, object: &crate::node::Node, index: &crate::node::Node, representation: crate::machine_type::MachineRepresentation) -> Option<crate::node::Node> {
    let _ = (object, index, representation);
    todo!("命中查找 (object/index/rep 三匹配)")
  }
  /// v8: src/compiler/load-elimination.cc AbstractElements::Kill
  pub fn kill(&self, object: &crate::node::Node, index: &crate::node::Node, zone: &mut crate::zone::Zone) -> *const AbstractElements {
    let _ = (object, index, zone);
    todo!("同 object 条目清除")
  }
  /// v8: src/compiler/load-elimination.cc AbstractElements::Equals
  pub fn equals(&self, that: &AbstractElements) -> bool {
    todo!("逐槽比较")
  }
  /// v8: src/compiler/load-elimination.cc AbstractElements::Merge
  pub fn merge(&self, that: &AbstractElements, zone: &mut crate::zone::Zone) -> *const AbstractElements {
    let _ = (that, zone);
    todo!("交集合并")
  }
}

/// v8: src/compiler/load-elimination.h LoadElimination::FieldInfo
/// 相等 = value + representation + name 句柄地址 + const_field_info
#[derive(Clone, Copy)]
pub struct FieldInfo {
  /// 原 value
  pub value: Option<crate::node::Node>,
  /// 原 representation (默认 kNone)
  pub representation: crate::machine_type::MachineRepresentation,
  /// 原 name (MaybeHandle<Name>, 地址比较)
  pub name: Option<usize>,
  /// 原 const_field_info (ConstFieldInfo::None() 默认)
  pub const_field_info: crate::ConstFieldInfo,
}

/// v8: src/compiler/load-elimination.h LoadElimination::AbstractFields / AbstractMaps
/// 见 .h:130-260; 字段按 map 区分槽位, maps 为对象 → map 集

/// v8: src/compiler/load-elimination.h LoadElimination
pub struct LoadElimination {
  /// 原 broker_
  pub broker: *mut crate::broker::JSHeapBroker,
  /// 原 node_states_
  pub node_states: crate::node_aux_data::NodeAuxData<*const AbstractState>,
  /// 原 jsgraph_
  pub jsgraph: *mut crate::js_graph::JSGraph,
}

impl LoadElimination {
  /// v8: src/compiler/load-elimination.cc LoadElimination::reducer_name
  pub const REDUCER_NAME: &'static str = "LoadElimination";

  /// v8: src/compiler/load-elimination.cc LoadElimination::Reduce
  /// 分派表 (${cases.length} case): ${cases.join(", ")}
  pub fn reduce(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    match node.opcode() {
      ${cases.map((c) => `crate::IrOpcode::${c} => todo!("k${c}")`).join(",\n      ")}
      _ => crate::graph_reducer::Reduction::NoChange,
    }
  }
}

/// 语义面占位: AbstractState (三抽象状态复合) + ComputeLoopState/MergeSets
pub struct AbstractState {
  pub _semantic: (),
}
`;

const p = writeSrc("v8-compiler-passes", "load_elimination.rs.skel", body);
console.log(`written ${p}\n分派case=${cases.length} 抽象状态=${stateClasses.length} 环形槽=8`);
