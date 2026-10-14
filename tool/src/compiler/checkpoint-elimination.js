#!/usr/bin/env bun
// v8/src/compiler/checkpoint-elimination.cc + checkpoint-elimination.h
// → v8-compiler-passes/src/checkpoint_elimination.rs.skel
//
// 全族一个 reducer 一个私有方法。规则面: ReduceCheckpoint 的 effect 链
// 回溯消解语义 (逐字核对 cc:54-60); Reduce 分派单 case。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/checkpoint-elimination.h");
const cc = readV8("src/compiler/checkpoint-elimination.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class V8_EXPORT_PRIVATE CheckpointElimination final/, "CheckpointElimination"],
  [h, /Reduction ReduceCheckpoint\(Node\* node\);/, "ReduceCheckpoint 声明"],
  [cc, /Reduction CheckpointElimination::ReduceCheckpoint\(Node\* node\) \{/, "ReduceCheckpoint 实现"],
  [cc, /case IrOpcode::kCheckpoint:/, "分派 case"],
  [cc, /bool IsRedundantCheckpoint\(Node\* node\) \{/, "IsRedundantCheckpoint 自由函数"],
  [cc, /return Replace\(NodeProperties::GetEffectInput\(node\)\);/, "Replace 语义"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}
// effect 链回溯: 沿 effect 边找前一个 CheckPoint → ReplaceEffectInput
if (!/NodeProperties::GetEffectInput\(node\)/.test(cc)) throw new Error("effect 输入回溯缺失");
if (!/return Replace\(NodeProperties::GetEffectInput\(node\)\);/.test(cc)) throw new Error("Replace 语义缺失");
if (!/GetFunctionInfo\(effect\) == function_info/.test(cc)) throw new Error("FunctionInfo 相等判据缺失");

const body = `${banner("src/compiler/checkpoint-elimination.js", "src/compiler/checkpoint-elimination.h + src/compiler/checkpoint-elimination.cc", "CheckpointElimination — 冗余 CheckPoint 消除")}
//! CheckpointElimination: 若 CheckPoint 的 effect 输入链上最近节点已是
//! CheckPoint (无中间可观察副作用), 则后者冗余, 改接前一个。
//!
//! 规则面: 分派表 1 case + effect 链回溯规则。

#![allow(dead_code)]

/// v8: src/compiler/checkpoint-elimination.h CheckpointElimination
pub struct CheckpointElimination {
  /// 原 editor_ (AdvancedReducer 基类)
  pub editor: *mut crate::graph_reducer::Editor,
}

impl CheckpointElimination {
  /// v8: src/compiler/checkpoint-elimination.cc CheckpointElimination::CheckpointElimination
  pub fn new(editor: *mut crate::graph_reducer::Editor) -> Self {
    Self { editor }
  }

  /// v8: src/compiler/checkpoint-elimination.cc CheckpointElimination::reducer_name
  pub const REDUCER_NAME: &'static str = "CheckpointElimination";

  /// v8: src/compiler/checkpoint-elimination.cc CheckpointElimination::Reduce
  /// 原文: switch 单 case kCheckPoint → ReduceCheckpoint; 其余 NoChange
  pub fn reduce(&mut self, node: &mut crate::node::Node) -> crate::graph_reducer::Reduction {
    match node.opcode() {
      crate::IrOpcode::Checkpoint => self.reduce_checkpoint(node),
      _ => crate::graph_reducer::Reduction::NoChange,
    }
  }

  /// v8: src/compiler/checkpoint-elimination.cc CheckpointElimination::ReduceCheckpoint
  /// 原文: if (IsRedundantCheckpoint(node)) return Replace(GetEffectInput(node));
  /// IsRedundantCheckpoint (cc:38-52): function_info 非空; 沿 effect 前驱
  /// 走 (途经须 kNoWrite 且 EffectInputCount==1), 首个 kCheckpoint 的
  /// GetFunctionInfo == 本节点 function_info 才冗余
  fn reduce_checkpoint(&mut self, node: &mut crate::node::Node) -> crate::graph_reducer::Reduction {
    let effect = crate::node_properties::get_effect_input(node);
    let _ = effect;
    todo!("effect 链回溯至前一 CheckPoint 并跳接")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "checkpoint_elimination.rs.skel", body);
console.log(`written ${p}\n方法=3 (new/reduce/reduce_checkpoint) 分派case=1`);
