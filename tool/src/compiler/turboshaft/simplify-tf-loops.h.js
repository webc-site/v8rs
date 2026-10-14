#!/usr/bin/env bun
// tool/src/compiler/turboshaft/simplify-tf-loops.h.js
// 机械提取 v8/src/compiler/turboshaft/simplify-tf-loops.{h,cc}
// → v8-turboshaft/src/simplify_tf_loops.rs.skel
//
// 提取面: SimplifyTFLoops (Turbofan 侧 AdvancedReducer) 规则锚。
// 注意: 此族在 namespace v8::internal::compiler (无 turboshaft 段)。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/simplify-tf-loops.h");
const cc = readV8("src/compiler/turboshaft/simplify-tf-loops.cc");

// 1. 哨兵
for (const [re, name] of [
  [/class SimplifyTFLoops final : public AdvancedReducer/, "AdvancedReducer 子类"],
  [/reducer_name\(\) const override \{ return "SimplifyTFLoops"; \}/, "reducer_name"],
  [/Reduction Reduce\(Node\* node\) final;/, "Reduce 入口"],
  [/node->opcode\(\) != IrOpcode::kLoop/, "Loop 过滤"],
  [/node->InputCount\(\) <= 2/, "两输入早退"],
  [/mcgraph_->common\(\)->Loop\(2\)/, "新 Loop(2)"],
  [/base::SmallVector<Edge, 4> control_uses;/, "control_uses 缓冲"],
  [/ReplaceWithValue\(use, new_phi, new_phi, new_phi\)/, "phi 三重替换"],
]) {
  if (!(re.test(h) || re.test(cc))) throw new Error(`哨兵缺失: ${name}`);
}

// 2. cc 中的 phi 重写规则
const phiRewrites = [...cc.matchAll(/common\(\)->(Phi|EffectPhi|Loop|Merge)\(/g)].map((m) => m[1]);
const counts = {};
phiRewrites.forEach((k) => (counts[k] = (counts[k] ?? 0) + 1));

let rust = banner(
  "src/compiler/turboshaft/simplify-tf-loops.h.js",
  "src/compiler/turboshaft/simplify-tf-loops.{h,cc}",
  "TF 循环化简: 多输入 Loop 拆二输入 (Turbofan 前置)",
);

rust += `
/// 原版 SimplifyTFLoops — Turbofan AdvancedReducer (非 Turboshaft reducer),
/// 把 >2 输入的 Loop 约束为 2 输入: 新 Loop(2) 套 Merge。
/// v8: src/compiler/turboshaft/simplify-tf-loops.{h,cc}
/// namespace: v8::internal::compiler
pub struct SimplifyTFLoops<'m> {
  pub mcgraph: &'m mut MachineGraph,
}

impl<'m> SimplifyTFLoops<'m> {
  /// v8: simplify-tf-loops.cc Reduce
  /// - opcode != kLoop 或 InputCount <= 2 -> NoChange
  /// - new_loop = NewNode(common->Loop(2), input0, node)
  /// - 原 node RemoveInput(0) 后 ChangeOp 为 Merge(InputCount)
  /// - phi: 对 use_edges 中 Phi/EffectPhi
  ///   (${Object.entries(counts).map(([k, v]) => `${k}×${v}`).join(", ")})
  ///   重建为支配输入 + new_phi(2), ReplaceWithValue(use, new_phi, new_phi, new_phi)
  /// - control_uses (非 phi 控制边) 改接 new_loop
  pub fn reduce(&mut self, node: &mut Node) -> Reduction { todo!() }
}
`;

const p = writeSrc("v8-turboshaft", "simplify_tf_loops.rs.skel", rust);
console.log(`written ${p}`);
console.log(`重写op=${Object.keys(counts).length} 规则锚=8`);
