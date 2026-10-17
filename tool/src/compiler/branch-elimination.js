#!/usr/bin/env bun
// v8/src/compiler/branch-elimination.cc + branch-elimination.h
// → v8-compiler-passes/src/branch_elimination.rs.skel
//
// 规则面: Phase 枚举 (kEARLY/kLATE)、Reduce 分派表 (opcode → 方法,
// 逐 case 提取)、BranchCondition 结构语义 (三元组相等)。
// 语义面 (推迟): 条件沿控制路径传播 (依赖 control-path-state)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/branch-elimination.h");
const cc = readV8("src/compiler/branch-elimination.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /struct BranchCondition \{/, "BranchCondition"],
  [h, /class V8_EXPORT_PRIVATE BranchElimination final/, "BranchElimination"],
  [h, /enum Phase \{/, "Phase 枚举"],
  [h, /Reduction Reduce\(Node\* node\) final;/, "Reduce 入口"],
  [h, /bool TryEliminateBranchWithPhiCondition\(Node\* branch, Node\* phi, Node\* merge\);/, "Phi 条件消解"],
  [cc, /Reduction BranchElimination::Reduce\(Node\* node\) \{/, "Reduce 实现"],
  [cc, /void BranchElimination::SimplifyBranchCondition\(Node\* branch\) \{/, "SimplifyBranchCondition"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}

// —— Phase 枚举 ——
const phaseBlock = h.match(/enum Phase \{([\s\S]*?)\n  \};/)[1];
const phases = [...phaseBlock.matchAll(/^\s*k(\w+),?\s*$/gm)].map((m) => m[1]);
if (phases.join() !== "EARLY,LATE") throw new Error(`Phase 漂移: ${phases}`);

// —— Reduce 分派表: switch (node->opcode()) 区段逐 case 提取 ——
const switchSrc = cc.slice(
  cc.indexOf("Reduction BranchElimination::Reduce(Node* node) {"),
  cc.indexOf("Reduction BranchElimination::ReduceBranch"),
);
const cases = [...switchSrc.matchAll(/case IrOpcode::k(\w+):/g)].map((m) => m[1]);
if (cases.join() !== "Dead,DeoptimizeIf,DeoptimizeUnless,Merge,Loop,Branch,IfFalse,IfTrue,TrapIf,TrapUnless,Start") {
  throw new Error(`分派表漂移: ${cases}`);
}
// default 分支: ControlOutputCount > 0 → ReduceOtherControl
if (!/default:\n      if \(node->op\(\)->ControlOutputCount\(\) > 0\) \{/.test(switchSrc)) {
  throw new Error("default 分派缺失");
}
// 私有方法清单 (cc 实现数对账)
const ccMethods = [...cc.matchAll(/^(?:Reduction|void|bool) BranchElimination::(\w+)\(/gm)].map((m) => m[1]);
const expectMethods = ["Reduce", "ReduceBranch", "ReduceDeoptimizeConditional", "ReduceIf", "ReduceLoop",
  "ReduceMerge", "ReduceStart", "ReduceOtherControl", "SimplifyBranchCondition",
  "TryEliminateBranchWithPhiCondition"];
const missing = expectMethods.filter((m) => !ccMethods.includes(m));
if (missing.length) throw new Error(`cc 缺实现: ${missing}`);

const body = `${banner("src/compiler/branch-elimination.js", "src/compiler/branch-elimination.h + src/compiler/branch-elimination.cc", "BranchElimination — 控制路径条件传播, 分支/条件去重")}
//! BranchElimination: 记录控制路径上已知条件的取值, 消解冗余 Branch /
//! DeoptimizeIf / TrapIf。
//!
//! 规则面: Phase 枚举, Reduce 分派表 ${cases.length} case + default,
//! BranchCondition 相等语义。语义面 (推迟): ControlPathState 泛型基类
//! (control-path-state.h) 的状态合并/回滚。

#![allow(dead_code, non_camel_case_types)]

/// v8: src/compiler/branch-elimination.h BranchElimination::Phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
  /// kEARLY
  Early,
  /// kLATE
  Late,
}

/// v8: src/compiler/branch-elimination.h BranchCondition
/// 条件节点 + 分支节点 + 真假值 三元组
#[derive(Clone, Copy)]
pub struct BranchCondition {
  /// 原 node — 条件
  pub node: crate::node::Node,
  /// 原 branch — 分支
  pub branch: crate::node::Node,
  /// 原 is_true
  pub is_true: bool,
}

impl BranchCondition {
  /// v8: src/compiler/branch-elimination.h BranchCondition::IsSet
  pub fn is_set(&self) -> bool {
    !self.node.is_null()
  }
}

impl PartialEq for BranchCondition {
  /// v8: src/compiler/branch-elimination.h BranchCondition::operator==
  fn eq(&self, other: &Self) -> bool {
    self.node == other.node && self.branch == other.branch && self.is_true == other.is_true
  }
}

/// v8: src/compiler/branch-elimination.h BranchElimination
pub struct BranchElimination {
  /// 原 js_graph_
  js_graph: *mut crate::js_graph::JSGraph,
  /// 原 dead_
  dead: crate::node::Node,
  /// 原 phase_
  phase: Phase,
}

impl BranchElimination {
  /// v8: src/compiler/branch-elimination.h BranchElimination::BranchElimination
  /// 原默认参: phase = kLATE
  pub fn new(editor: &mut dyn crate::graph_reducer::Editor, js_graph: *mut crate::js_graph::JSGraph, zone: &mut crate::zone::Zone, phase: Phase) -> Self {
    let _ = (editor, zone);
    todo!("ControlPathState 基类构造")
  }

  /// v8: src/compiler/branch-elimination.cc BranchElimination::reducer_name
  pub const REDUCER_NAME: &'static str = "BranchElimination";

  /// v8: src/compiler/branch-elimination.cc BranchElimination::Reduce
  /// 分派表 (原文顺序): ${cases.join(", ")}; default: 控制输出>0 → ReduceOtherControl
  pub fn reduce(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    match node.opcode() {
      ${cases.map((c) => `crate::IrOpcode::${c} => todo!("case k${c}")`).join(",\n      ")}
      _ if node.op().control_output_count() > 0 => self.reduce_other_control(node),
      _ => crate::graph_reducer::Reduction::NoChange,
    }
  }

  /// v8: src/compiler/branch-elimination.cc BranchElimination::ReduceBranch
  pub fn reduce_branch(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("条件已知真/假 → 消解分支; 相反条件 → DeoptimizeIf 改写")
  }
  /// v8: src/compiler/branch-elimination.cc BranchElimination::ReduceDeoptimizeConditional
  pub fn reduce_deoptimize_conditional(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("DeoptimizeIf/Unless 条件已知 → 无条件 Deoptimize 或消除")
  }
  /// v8: src/compiler/branch-elimination.cc BranchElimination::ReduceIf
  /// 原文: is_true_branch 区分 IfTrue/IfFalse
  pub fn reduce_if(&mut self, node: &crate::node::Node, is_true_branch: bool) -> crate::graph_reducer::Reduction {
    todo!("记录 BranchCondition 到路径状态")
  }
  /// v8: src/compiler/branch-elimination.cc BranchElimination::ReduceTrapConditional
  pub fn reduce_trap_conditional(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("TrapIf/Unless 对称处理")
  }
  /// v8: src/compiler/branch-elimination.cc BranchElimination::ReduceLoop
  pub fn reduce_loop(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("循环头状态重置 (沿回边不可用条件丢弃)")
  }
  /// v8: src/compiler/branch-elimination.cc BranchElimination::ReduceMerge
  pub fn reduce_merge(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("多前驱条件交集合并")
  }
  /// v8: src/compiler/branch-elimination.cc BranchElimination::ReduceStart
  pub fn reduce_start(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("Start 状态初始化")
  }
  /// v8: src/compiler/branch-elimination.cc BranchElimination::ReduceOtherControl
  pub fn reduce_other_control(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("其余控制节点状态继承")
  }
  /// v8: src/compiler/branch-elimination.cc BranchElimination::SimplifyBranchCondition
  pub fn simplify_branch_condition(&mut self, branch: &crate::node::Node) {
    todo!("条件规范化 (如 Word32Equal 反转)")
  }
  /// v8: src/compiler/branch-elimination.cc BranchElimination::TryEliminateBranchWithPhiCondition
  pub fn try_eliminate_branch_with_phi_condition(&mut self, branch: &crate::node::Node, phi: &crate::node::Node, merge: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("Phi 条件各输入一致性 → 常量化分支")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "branch_elimination.rs.skel", body);
console.log(`written ${p}\nPhase=${phases.length} 分派case=${cases.length} cc方法=${ccMethods.length}`);
