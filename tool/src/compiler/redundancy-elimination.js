#!/usr/bin/env bun
// tool/src/compiler/redundancy-elimination.js
// v8/src/compiler/redundancy-elimination.h + redundancy-elimination.cc
// → v8-compiler-passes/src/redundancy_elimination.rs.skel
// 效应路径检查消除: 解析 EffectPathChecks 链表语义与 Reduce* 面。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/redundancy-elimination.h");
const cc = readV8("src/compiler/redundancy-elimination.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`redundancy-elimination: ${msg}`);
};

for (const [s, name] of [
  ["class V8_EXPORT_PRIVATE RedundancyElimination final : public AdvancedReducer {", "类定义"],
  ["class EffectPathChecks final {", "EffectPathChecks"],
  ["class PathChecksForEffectNodes final {", "PathChecksForEffectNodes"],
  ['return "RedundancyElimination";', "reducer_name"],
  ["Reduction RedundancyElimination::Reduce(Node* node) {", "cc Reduce"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// —— Reduce* / Take / Update 面 ——
const reducers = [...h.matchAll(/^\s{2}Reduction (Reduce\w+|Take\w+|Update\w+)\(Node\* node[^;]*;/gm)].map((m) => m[1]);
for (const r of ["ReduceCheckNode", "ReduceEffectPhi", "ReduceSpeculativeNumberOperation", "ReduceStart", "TakeChecksFromFirstEffect", "UpdateChecks"]) {
  need(reducers.includes(r), `${r} 缺席`);
}

// —— EffectPathChecks 静态/成员面 ——
const epcMethods = [...h.matchAll(/^\s{4}static EffectPathChecks(?: const)?\* (\w+)\(/gm)].map((m) => m[1]);
need(epcMethods.includes("Copy") && epcMethods.includes("Empty"), "Copy/Empty 缺席");

// —— cc 定义对数 (前缀匹配, 容忍换行参数) ——
const ccDefs = ["Reduce", "ReduceCheckNode", "ReduceEffectPhi", "ReduceSpeculativeNumberComparison", "ReduceSpeculativeNumberOperation", "ReduceStart", "ReduceOtherNode", "TakeChecksFromFirstEffect", "UpdateChecks"].filter((d) => cc.includes(`RedundancyElimination::${d}(Node* node`));
need(ccDefs.length >= 9, `cc 定义数 ${ccDefs.length}`);

// —— cc: 检查节点可消除的判定 (effect 路径上等价检查) ——
const lookupImpl = cc.includes("RedundancyElimination::EffectPathChecks::LookupCheck") ? 1 : 0;
need(lookupImpl === 1, "cc LookupCheck 缺席");

const body = `${banner("src/compiler/redundancy-elimination.js", "src/compiler/redundancy-elimination.h + redundancy-elimination.cc", "效应路径上的冗余检查消除")}
//! redundancy-elimination — 同效应路径等价检查复用
//!
//! EffectPathChecks: 检查节点单向链 (copy-on-write, Merge 取最长公共尾);
//! PathChecksForEffectNodes: NodeId → 当前路径检查集。

/// 原版 RedundancyElimination::Check — 链表节点
pub struct Check<'n> {
  /// node
  pub node: &'n Node,
  /// next
  pub next: Option<&'n Check<'n>>,
}

/// 原版 EffectPathChecks — 效应路径检查集 (不可变链 + 长度)
pub struct EffectPathChecks<'n> {
  /// head_
  pub head: Option<&'n Check<'n>>,
  /// size_
  pub size: usize,
}

impl EffectPathChecks<'_> {
  /// 原版 Copy — 拷贝 (链共享, O(1) 语义)
  /// v8: src/compiler/redundancy-elimination.cc EffectPathChecks::Copy
  pub fn copy(zone: &Zone<'_>, checks: &Self) -> &Self {
    todo!("v8: src/compiler/redundancy-elimination.cc EffectPathChecks::Copy")
  }

  /// 原版 Empty — 空集
  /// v8: src/compiler/redundancy-elimination.cc EffectPathChecks::Empty
  pub fn empty(zone: &Zone<'_>) -> &Self {
    todo!("v8: src/compiler/redundancy-elimination.cc EffectPathChecks::Empty")
  }

  /// 原版 Equals — 链逐节点等价
  /// v8: src/compiler/redundancy-elimination.cc EffectPathChecks::Equals
  pub fn equals(&self, that: &Self) -> bool {
    todo!("v8: src/compiler/redundancy-elimination.cc EffectPathChecks::Equals")
  }

  /// 原版 Merge — 最长公共尾合并
  /// v8: src/compiler/redundancy-elimination.cc EffectPathChecks::Merge
  pub fn merge(&self, that: &Self) {
    todo!("v8: src/compiler/redundancy-elimination.cc EffectPathChecks::Merge")
  }

  /// 原版 AddCheck — 头插返回新集
  /// v8: src/compiler/redundancy-elimination.cc EffectPathChecks::AddCheck
  pub fn add_check<'z>(&self, zone: &'z Zone<'z>, node: &Node) -> &EffectPathChecks<'z> {
    todo!("v8: src/compiler/redundancy-elimination.cc EffectPathChecks::AddCheck")
  }

  /// 原版 LookupCheck — 等价检查节点查询
  /// v8: src/compiler/redundancy-elimination.cc EffectPathChecks::LookupCheck
  pub fn lookup_check(&self, node: &Node, jsgraph: &JSGraph<'_>) -> Option<&Node> {
    todo!("v8: src/compiler/redundancy-elimination.cc EffectPathChecks::LookupCheck")
  }

  /// 原版 LookupBoundsCheckFor — BoundsCheck 专用查询
  /// v8: src/compiler/redundancy-elimination.cc EffectPathChecks::LookupBoundsCheckFor
  pub fn lookup_bounds_check_for(&self, node: &Node) -> Option<&Node> {
    todo!("v8: src/compiler/redundancy-elimination.cc EffectPathChecks::LookupBoundsCheckFor")
  }
}

/// 原版 RedundancyElimination — AdvancedReducer
pub struct RedundancyElimination<'z, 'g> {
  /// node_checks_ — PathChecksForEffectNodes
  pub node_checks: PathChecksForEffectNodes<'z>,
  /// jsgraph_
  pub jsgraph: &'g JSGraph<'z>,
  /// zone_
  pub zone: &'z Zone<'z>,
}

impl RedundancyElimination<'_, '_> {
  /// 原版 reducer_name
  pub const REDUCER_NAME: &'static str = "RedundancyElimination";

${reducers.map((r) => `  /// 原版 ${r}\n  /// v8: src/compiler/redundancy-elimination.cc RedundancyElimination::${r}\n  fn ${r.toLowerCase()}(&mut self, node: &Node) -> Reduction {\n    todo!("v8: src/compiler/redundancy-elimination.cc RedundancyElimination::${r}")\n  }`).join("\n\n")}
}

/// 原版 PathChecksForEffectNodes — NodeId 索引的逐节点检查集
pub struct PathChecksForEffectNodes<'z> {
  /// info_for_node_
  pub info_for_node: ZoneVector<Option<&'z EffectPathChecks<'z>>>,
}

impl PathChecksForEffectNodes<'_> {
  /// 原版 Get / Set
  /// v8: src/compiler/redundancy-elimination.h PathChecksForEffectNodes::Get
  pub fn get(&self, node: &Node) -> Option<&EffectPathChecks<'_>> {
    todo!("v8: src/compiler/redundancy-elimination.cc PathChecksForEffectNodes::Get")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "redundancy_elimination.rs.skel", body);
console.log(`written ${p}\nreduce_face=${reducers.length} epc_static=${epcMethods.length} cc_defs=${ccDefs.length}`);
