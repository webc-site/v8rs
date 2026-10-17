#!/usr/bin/env bun
// v8/src/compiler/csa-load-elimination.cc + csa-load-elimination.h
// → v8-compiler-passes/src/csa_load_elimination.rs.skel
//
// 规则面: Reduce 分派表 (8 opcode 组) + FieldInfo 结构语义 +
// HalfState 六路 entry (fresh/constant/arbitrary × known/unknown)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/csa-load-elimination.h");
const cc = readV8("src/compiler/csa-load-elimination.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class V8_EXPORT_PRIVATE CsaLoadElimination final/, "CsaLoadElimination"],
  [h, /struct FieldInfo \{/, "FieldInfo"],
  [h, /class HalfState final : public ZoneObject \{/, "HalfState"],
  [h, /fresh_entries_\(zone, InnerMap\(zone\)\)/, "六路 entry"],
  [cc, /Reduction CsaLoadElimination::Reduce\(Node\* node\) \{/, "Reduce 实现"],
  [cc, /case IrOpcode::kLoadFromObject:/, "分派 Load"],
  [cc, /case IrOpcode::kAbortCSADcheck:/, "分派 AbortCSADcheck"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}

// —— 分派表 ——
const switchSrc = cc.slice(
  cc.indexOf("Reduction CsaLoadElimination::Reduce(Node* node) {"),
  cc.indexOf("Reduction CsaLoadElimination::ReduceLoadFromObject"),
);
const cases = [...switchSrc.matchAll(/case IrOpcode::k(\w+):/g)].map((m) => m[1]);
if (cases.join() !== "LoadFromObject,LoadImmutableFromObject,StoreToObject,InitializeImmutableInObject,DebugBreak,AbortCSADcheck,Call,EffectPhi,Dead,Start") {
  throw new Error(`分派表漂移: ${cases}`);
}
// 六路 entry 字段
const entries = [...h.matchAll(/^          (\w+)_entries_\(zone, InnerMap\(zone\)\),?[{}\s]*$/gm)].map((m) => m[1]);
if (entries.join() !== "fresh,constant,arbitrary,fresh_unknown,constant_unknown,arbitrary_unknown") {
  throw new Error(`entry 路 ${entries}`);
}
// cc 方法面
const ccMethods = [...cc.matchAll(/^(?:Reduction|CsaLoadElimination::HalfState\*|CsaLoadElimination::HalfState const\*) CsaLoadElimination::(\w+)/gm)].map((m) => m[1]);
if (ccMethods.length < 8) throw new Error(`cc 方法 ${ccMethods.length} < 8`);

const body = `${banner("src/compiler/csa-load-elimination.js", "src/compiler/csa-load-elimination.h + src/compiler/csa-load-elimination.cc", "CsaLoadElimination — CSA 层对象字段 load 消除 (HalfState 六路)")}
//! CsaLoadElimination: 针对 CSA (LoadFromObject/StoreToObject) 的
//! 冗余 load 消除, 按 offset 索引的 HalfState 传播。
//!
//! 规则面: 分派表 ${cases.length} 组 + FieldInfo + HalfState 六路 entry。
//! 语义面 (推迟): InnerMap 合并 (KillField 等, 见设计文档 bit.ly/36MfD6Y)。

#![allow(dead_code)]

/// v8: src/compiler/csa-load-elimination.h CsaLoadElimination::FieldInfo
/// 值节点 + 表示; 相等 = 两者均等; 空 = value 为 null
#[derive(Clone, Copy)]
pub struct FieldInfo {
  /// 原 value
  pub value: Option<crate::node::Node>,
  /// 原 representation (默认 kNone)
  pub representation: crate::machine_type::MachineRepresentation,
}

impl FieldInfo {
  /// v8: src/compiler/csa-load-elimination.h FieldInfo::operator==
  pub fn eq(&self, other: &Self) -> bool {
    self.value == other.value && self.representation == other.representation
  }
  /// v8: src/compiler/csa-load-elimination.h FieldInfo::IsEmpty
  pub fn is_empty(&self) -> bool {
    self.value.is_none()
  }
}

/// v8: src/compiler/csa-load-elimination.h CsaLoadElimination::HalfState
/// 六路: fresh/constant/arbitrary × 已知对象/未知对象 (unknown)
pub struct HalfState {
  /// 原 fresh_entries_ — 新建对象字段表
  pub fresh_entries: crate::persistent_map::PersistentMap<crate::HalfKey, FieldInfo>,
  /// 原 constant_entries_ — 常量对象
  pub constant_entries: crate::persistent_map::PersistentMap<crate::HalfKey, FieldInfo>,
  /// 原 arbitrary_entries_ — 任意对象
  pub arbitrary_entries: crate::persistent_map::PersistentMap<crate::HalfKey, FieldInfo>,
  /// 原 fresh_unknown_entries_ / constant_unknown_entries_ / arbitrary_unknown_entries_
  pub fresh_unknown_entries: crate::persistent_map::PersistentMap<crate::UnknownKey, FieldInfo>,
  pub constant_unknown_entries: crate::persistent_map::PersistentMap<crate::UnknownKey, FieldInfo>,
  pub arbitrary_unknown_entries: crate::persistent_map::PersistentMap<crate::UnknownKey, FieldInfo>,
}

impl HalfState {
  /// v8: src/compiler/csa-load-elimination.h HalfState::Equals
  /// 六路逐一比较
  pub fn equals(&self, that: &HalfState) -> bool {
    self.fresh_entries == that.fresh_entries
      && self.constant_entries == that.constant_entries
      && self.arbitrary_entries == that.arbitrary_entries
      && self.fresh_unknown_entries == that.fresh_unknown_entries
      && self.constant_unknown_entries == that.constant_unknown_entries
      && self.arbitrary_unknown_entries == that.arbitrary_unknown_entries
  }
}

/// v8: src/compiler/csa-load-elimination.h CsaLoadElimination
pub struct CsaLoadElimination {
  /// 原 empty_state_
  pub empty_state: HalfState,
  /// 原 node_states_ — NodeAuxData<HalfState*>
  pub node_states: crate::node_aux_data::NodeAuxData<*mut HalfState>,
  /// 原 jsgraph_
  pub jsgraph: *mut crate::js_graph::JSGraph,
}

impl CsaLoadElimination {
  /// v8: src/compiler/csa-load-elimination.cc CsaLoadElimination::reducer_name
  pub const REDUCER_NAME: &'static str = "CsaLoadElimination";

  /// v8: src/compiler/csa-load-elimination.cc CsaLoadElimination::Reduce
  /// 分派表 (原文顺序): ${cases.join(", ")}
  pub fn reduce(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    match node.opcode() {
      ${cases.map((c) => `crate::IrOpcode::${c} => todo!("k${c}")`).join(",\n      ")}
      _ => crate::graph_reducer::Reduction::NoChange,
    }
  }

  /// v8: src/compiler/csa-load-elimination.cc CsaLoadElimination::ReduceLoadFromObject
  pub fn reduce_load_from_object(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("HalfState 命中同 offset → Replace(旧值)")
  }
  /// v8: src/compiler/csa-load-elimination.cc CsaLoadElimination::ReduceStoreToObject
  pub fn reduce_store_to_object(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("写即覆盖 + 可传递kill")
  }
  /// v8: src/compiler/csa-load-elimination.cc CsaLoadElimination::ReduceEffectPhi
  pub fn reduce_effect_phi(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("多前驱 HalfState 合并")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "csa_load_elimination.rs.skel", body);
console.log(`written ${p}\n分派case=${cases.length} 六路entry=${entries.length} cc方法=${ccMethods.length}`);
