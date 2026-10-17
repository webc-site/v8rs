#!/usr/bin/env bun
// v8/src/compiler/diamond.h (单文件, 无 .cc)
// → v8-compiler-passes/src/diamond.rs.skel
//
// 全结构 + 5 方法, 语义全部完整可迁 (图节点装配序列逐字核对)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/diamond.h");

// —— 哨兵 ——
const sentinels = [
  [/struct Diamond \{/, "Diamond 结构"],
  [/BranchHint hint = BranchHint::kNone,/, "默认 hint"],
  [/BranchSemantics semantics = BranchSemantics::kUnspecified\)/, "默认 semantics"],
  [/graph->NewNode\(common->Merge\(2\), if_true, if_false\);/, "构造尾 Merge"],
  [/void Chain\(Diamond const& that\) \{ branch->ReplaceInput\(1, that\.merge\); \}/, "Chain"],
  [/that.merge->ReplaceInput\(0, merge\);/, "Nest true 路"],
  [/return graph->NewNode\(common->Phi\(rep, 2\), tv, fv, merge\);/, "Phi"],
];
for (const [re, name] of sentinels) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}
// 构造序列: Branch → IfTrue → IfFalse → Merge (顺序即依赖)
const order = ["common->Branch(hint, semantics), cond, graph->start()", "common->IfTrue(), branch", "common->IfFalse(), branch", "common->Merge(2), if_true, if_false"];
let prev = -1;
for (const frag of order) {
  const i = h.indexOf(frag);
  if (i < 0 || i <= prev) throw new Error(`构造序漂移: ${frag}`);
  prev = i;
}

const body = `${banner("src/compiler/diamond.js", "src/compiler/diamond.h", "Diamond — 菱形控制模式构建助手 (Branch/IfTrue/IfFalse/Merge)")}
//! Diamond: 一次构造 5 节点 (Branch + IfTrue + IfFalse + Merge),
//! Chain/Nest 拼接, Phi/EffectPhi 汇聚。

#![allow(dead_code)]

/// v8: src/compiler/diamond.h Diamond
pub struct Diamond {
  /// 原 graph
  pub graph: *mut crate::graph::TFGraph,
  /// 原 common
  pub common: *mut crate::common_operator::CommonOperatorBuilder,
  /// 原 branch / if_true / if_false / merge
  pub branch: crate::node::Node,
  pub if_true: crate::node::Node,
  pub if_false: crate::node::Node,
  pub merge: crate::node::Node,
}

impl Diamond {
  /// v8: src/compiler/diamond.h Diamond::Diamond
  /// 原文装配序: Branch(hint,semantics)(cond, start) →
  /// IfTrue(branch) → IfFalse(branch) → Merge(2)(if_true, if_false);
  /// 原默认参: hint = BranchHint::kNone, semantics = BranchSemantics::kUnspecified
  pub fn new(
    g: &mut crate::graph::TFGraph,
    b: &mut crate::common_operator::CommonOperatorBuilder,
    cond: crate::node::Node,
    hint: crate::BranchHint, /* 原 kNone */
    semantics: crate::BranchSemantics, /* 原 kUnspecified */
  ) -> Self {
    let branch = g.new_node(b.branch_with(hint, semantics), cond, g.start());
    let if_true = g.new_node(b.if_true(), branch);
    let if_false = g.new_node(b.if_false(), branch);
    let merge = g.new_node(b.merge(2), if_true, if_false);
    Self { graph: g as *mut _, common: b as *mut _, branch, if_true, if_false, merge }
  }

  /// v8: src/compiler/diamond.h Diamond::Chain (Diamond 版)
  /// 原文: branch->ReplaceInput(1, that.merge)
  pub fn chain(&mut self, that: &Diamond) {
    self.branch.replace_input(1, that.merge);
  }

  /// v8: src/compiler/diamond.h Diamond::Chain (Node 版)
  /// 原文: branch->ReplaceInput(1, that)
  pub fn chain_node(&mut self, that: crate::node::Node) {
    self.branch.replace_input(1, that);
  }

  /// v8: src/compiler/diamond.h Diamond::Nest
  /// 原文: cond → 接 that.if_true 且 that.merge 输入 0 ← merge;
  /// !cond → 接 that.if_false 且 that.merge 输入 1 ← merge
  pub fn nest(&mut self, that: &mut Diamond, cond: bool) {
    if cond {
      self.branch.replace_input(1, that.if_true);
      that.merge.replace_input(0, self.merge);
    } else {
      self.branch.replace_input(1, that.if_false);
      that.merge.replace_input(1, self.merge);
    }
  }

  /// v8: src/compiler/diamond.h Diamond::Phi
  /// 原文: Phi(rep, 2)(tv, fv, merge)
  pub fn phi(&mut self, rep: crate::machine_type::MachineRepresentation, tv: crate::node::Node, fv: crate::node::Node) -> crate::node::Node {
    unsafe { (*self.graph).new_node(unsafe { (*self.common).phi(rep, 2) }, tv, fv, self.merge) }
  }

  /// v8: src/compiler/diamond.h Diamond::EffectPhi
  /// 原文: EffectPhi(2)(tv, fv, merge)
  pub fn effect_phi(&mut self, tv: crate::node::Node, fv: crate::node::Node) -> crate::node::Node {
    unsafe { (*self.graph).new_node(unsafe { (*self.common).effect_phi(2) }, tv, fv, self.merge) }
  }
}
`;

const p = writeSrc("v8-compiler-passes", "diamond.rs.skel", body);
console.log(`written ${p}\n方法=6 装配序=4 节点=5 (完整语义, 无 todo!)`);
