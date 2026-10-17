#!/usr/bin/env bun
// tool/src/compiler/select-lowering.js
// v8/src/compiler/select-lowering.h + select-lowering.cc
// → v8-compiler-passes/src/select_lowering.rs.skel
// Select→菱形降低: 解析 Reducer 面与 cc 定义对数。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/select-lowering.h");
const cc = readV8("src/compiler/select-lowering.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`select-lowering: ${msg}`);
};

for (const [s, name] of [
  ["class SelectLowering final : public Reducer {", "SelectLowering 定义"],
  ['return "SelectLowering";', "reducer_name"],
  ["Reduction SelectLowering::Reduce(Node* node)", "cc Reduce"],
  ["Reduction SelectLowering::LowerSelect(Node* node)", "cc LowerSelect"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// Reduce 的 opcode 门 (真解析)
const gate = cc.match(/if \(node->opcode\(\) != IrOpcode::k(\w+)\)/);
need(gate && gate[1] === "Select", `Reduce 门 ${gate && gate[1]}`);

const body = `${banner("src/compiler/select-lowering.js", "src/compiler/select-lowering.h + select-lowering.cc", "Select 节点降低为 Branch/Diamond")}
//! select-lowering — Select(cond,v1,v2) → 菱形 (Branch+IfTrue/IfFalse+Phi)
//!
//! 对 start 之后的控制流敏感: 记 start 节点, 插入位置在其后。

/// 原版 SelectLowering — Reducer (图组装器 + start 锚)
pub struct SelectLowering<'z, 'g> {
  /// graph_assembler_
  pub graph_assembler: &'z mut JSGraphAssembler<'z, 'g>,
  /// start_
  pub start: *mut Node,
}

impl SelectLowering<'_, '_> {
  /// 原版 reducer_name
  pub const REDUCER_NAME: &'static str = "SelectLowering";

  /// 原版 Reduce — 仅 IrOpcode::Select 进入 (${gate[1]} 门核对)
  /// v8: src/compiler/select-lowering.cc SelectLowering::Reduce
  pub fn reduce(&mut self, node: &Node) -> Reduction {
    if node.opcode() != IrOpcode::Select {
      return Reduction::NoChange;
    }
    self.lower_select(node)
  }

  /// 原版 LowerSelect — 构造菱形并替换
  /// v8: src/compiler/select-lowering.cc SelectLowering::LowerSelect
  fn lower_select(&mut self, node: &Node) -> Reduction {
    todo!("v8: src/compiler/select-lowering.cc SelectLowering::LowerSelect")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "select_lowering.rs.skel", body);
console.log(`written ${p}\nreduce_gate=${gate[1]}`);
