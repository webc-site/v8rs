#!/usr/bin/env bun
// v8/src/compiler/dead-code-elimination.cc + dead-code-elimination.h
// → v8-compiler-passes/src/dead_code_elimination.rs.skel
//
// 规则面: Reduce 分派表 (14 opcode) + 12 私有方法签名 +
// DeadValue 默认表示 (kNone); TrimMergeOrPhi 语义锚。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/dead-code-elimination.h");
const cc = readV8("src/compiler/dead-code-elimination.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class V8_EXPORT_PRIVATE DeadCodeElimination final/, "DeadCodeElimination"],
  [h, /Reduction ReduceDeoptimizeOrReturnOrTerminateOrTailCall\(Node\* node\);/, "长名方法"],
  [h, /MachineRepresentation rep = MachineRepresentation::kNone\)/, "DeadValue 默认"],
  [cc, /Reduction DeadCodeElimination::Reduce\(Node\* node\) \{/, "Reduce 实现"],
  [cc, /Reduction DeadCodeElimination::PropagateDeadControl\(/, "PropagateDeadControl"],
  [cc, /void DeadCodeElimination::TrimMergeOrPhi\(Node\* node, int size\) \{/, "TrimMergeOrPhi"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}

// —— 分派表 ——
const switchSrc = cc.slice(
  cc.indexOf("Reduction DeadCodeElimination::Reduce(Node* node) {"),
  cc.indexOf("Reduction DeadCodeElimination::ReduceEnd"),
);
const cases = [...switchSrc.matchAll(/case IrOpcode::k(\w+):/g)].map((m) => m[1]);
if (cases.join() !== "End,Loop,Merge,LoopExit,Unreachable,IfException,Phi,EffectPhi,Deoptimize,Return,Terminate,TailCall,Throw,Branch,Switch") {
  throw new Error(`分派表漂移: ${cases}`);
}
// 私有方法 (cc 实现对账)
const ccMethods = [...cc.matchAll(/^Reduction DeadCodeElimination::(\w+)\(|^void DeadCodeElimination::(\w+)\(|^Node\* DeadCodeElimination::(\w+)\(/gm)].map((m) => m[1] ?? m[2] ?? m[3]);
const expect = ["Reduce", "ReduceEnd", "ReduceLoopOrMerge", "ReduceLoopExit", "ReduceNode", "ReducePhi", "ReduceEffectPhi", "ReducePureNode",
  "ReduceUnreachableOrIfException", "ReduceEffectNode", "ReduceDeoptimizeOrReturnOrTerminateOrTailCall", "ReduceBranchOrSwitch",
  "RemoveLoopExit", "PropagateDeadControl", "TrimMergeOrPhi", "DeadValue"];
const missing = expect.filter((m) => !ccMethods.includes(m));
if (missing.length) throw new Error(`cc 缺实现: ${missing}`);

const body = `${banner("src/compiler/dead-code-elimination.js", "src/compiler/dead-code-elimination.h + src/compiler/dead-code-elimination.cc", "DeadCodeElimination — 死控制/死值传播与清除")}
//! DeadCodeElimination: 以 DeadValue/Unreachable 传播, 收缩 Loop/Merge
//! 死前驱, Trim 残留 Phi 输入。
//!
//! 规则面: 分派表 ${cases.length} case + ${expect.length} 方法。
//! 语义面 (推迟): 各 case 的传播方向与图改写。

#![allow(dead_code, unused_variables)]

/// v8: src/compiler/dead-code-elimination.h DeadCodeElimination
pub struct DeadCodeElimination {
  graph: *mut crate::graph::TFGraph,
  common: *mut crate::common_operator::CommonOperatorBuilder,
  dead: crate::node::Node,
  zone: *mut crate::zone::Zone,
}

impl DeadCodeElimination {
  /// v8: src/compiler/dead-code-elimination.h DeadCodeElimination::DeadCodeElimination
  pub fn new(editor: &mut dyn crate::graph_reducer::Editor, graph: *mut crate::graph::TFGraph, common: *mut crate::common_operator::CommonOperatorBuilder, temp_zone: *mut crate::zone::Zone) -> Self {
    todo!("dead_ = Dead (graph->NewNode(common->Dead()))")
  }

  /// v8: src/compiler/dead-code-elimination.cc DeadCodeElimination::reducer_name
  pub const REDUCER_NAME: &'static str = "DeadCodeElimination";

  /// v8: src/compiler/dead-code-elimination.cc DeadCodeElimination::Reduce
  /// 分派表 (原文顺序): ${cases.join(", ")}; default → ReduceNode
  pub fn reduce(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    match node.opcode() {
      ${cases.map((c) => `crate::IrOpcode::${c} => todo!("k${c}")`).join(",\n      ")}
      _ => self.reduce_node(node),
    }
  }

  /// v8: src/compiler/dead-code-elimination.cc DeadCodeElimination::ReduceEnd
  pub fn reduce_end(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("End 死输入 Trim")
  }
  /// v8: src/compiler/dead-code-elimination.cc DeadCodeElimination::ReduceLoopOrMerge
  pub fn reduce_loop_or_merge(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("死前驱剔除; 全死 → 死控制传播")
  }
  /// v8: src/compiler/dead-code-elimination.cc DeadCodeElimination::ReduceLoopExit
  pub fn reduce_loop_exit(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("LoopExit 旁路")
  }
  /// v8: src/compiler/dead-code-elimination.cc DeadCodeElimination::ReduceNode
  pub fn reduce_node(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("纯节点 → DeadValue; 效应节点 → Unreachable")
  }
  /// v8: src/compiler/dead-code-elimination.cc DeadCodeElimination::ReducePhi
  pub fn reduce_phi(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("死输入置 DeadValue; 单活输入抽取")
  }
  /// v8: src/compiler/dead-code-elimination.cc DeadCodeElimination::ReduceEffectPhi
  pub fn reduce_effect_phi(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("死效应输入替换")
  }
  /// v8: src/compiler/dead-code-elimination.cc DeadCodeElimination::ReducePureNode
  pub fn reduce_pure_node(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("DeadValue(rep) 替换")
  }
  /// v8: src/compiler/dead-code-elimination.cc DeadCodeElimination::ReduceUnreachableOrIfException
  pub fn reduce_unreachable_or_if_exception(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("死控制直通")
  }
  /// v8: src/compiler/dead-code-elimination.cc DeadCodeElimination::ReduceEffectNode
  pub fn reduce_effect_node(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("效应链死亡 → Unreachable 接管")
  }
  /// v8: src/compiler/dead-code-elimination.cc DeadCodeElimination::ReduceDeoptimizeOrReturnOrTerminateOrTailCall
  pub fn reduce_deoptimize_or_return_or_terminate_or_tail_call(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("控制死 → 止于 Unreachable")
  }
  /// v8: src/compiler/dead-code-elimination.cc DeadCodeElimination::ReduceBranchOrSwitch
  pub fn reduce_branch_or_switch(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("死条件分支消解")
  }
  /// v8: src/compiler/dead-code-elimination.cc DeadCodeElimination::RemoveLoopExit
  pub fn remove_loop_exit(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("LoopExit 摘除直连")
  }
  /// v8: src/compiler/dead-code-elimination.cc DeadCodeElimination::PropagateDeadControl
  pub fn propagate_dead_control(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("控制输入为 Dead → 本节点 Dead")
  }
  /// v8: src/compiler/dead-code-elimination.cc DeadCodeElimination::TrimMergeOrPhi
  /// 原文: 前驱重编号 + 输入 TrimInputCount(1 + size)
  pub fn trim_merge_or_phi(&mut self, node: &crate::node::Node, size: i32) {
    todo!("输入收缩到 size")
  }
  /// v8: src/compiler/dead-code-elimination.cc DeadCodeElimination::DeadValue
  /// 原默认参: rep = MachineRepresentation::kNone
  pub fn dead_value(&mut self, none_node: crate::node::Node, rep: crate::machine_type::MachineRepresentation) -> crate::node::Node {
    todo!("DeadValue(rep)(none_node)")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "dead_code_elimination.rs.skel", body);
console.log(`written ${p}\n分派case=${cases.length} cc方法=${ccMethods.length} DeadValue默认=kNone`);
