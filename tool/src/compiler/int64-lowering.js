#!/usr/bin/env bun
// v8/src/compiler/int64-lowering.cc + int64-lowering.h
// → v8-compiler-passes/src/int64_lowering.rs.skel
//
// 规则面: State 枚举 (3)、Replacement{low,high} 结构、方法面 12 签名
// (含 GetParameterCountAfterLowering 静态); 32 位目标降 64 位
// (low/high 拆分) 的各 Lower* 锚。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/int64-lowering.h");
const cc = readV8("src/compiler/int64-lowering.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class V8_EXPORT_PRIVATE Int64Lowering \{/, "Int64Lowering 实体分支"],
  [h, /enum class State : uint8_t \{ kUnvisited, kOnStack, kVisited \};/, "State 枚举"],
  [h, /struct Replacement \{/, "Replacement"],
  [h, /static int GetParameterCountAfterLowering\(/, "GetParameterCountAfterLowering"],
  [h, /void LowerWord64AtomicNarrowOp\(Node\* node, const Operator\* op\);/, "LowerWord64AtomicNarrowOp"],
  [cc, /void Int64Lowering::LowerGraph\(\) \{/, "LowerGraph 实现"],
  [cc, /void Int64Lowering::LowerNode\(Node\* node\) \{/, "LowerNode 实现"],
  [cc, /void Int64Lowering::ReplaceNode\(Node\* old, Node\* new_low, Node\* new_high\) \{/, "ReplaceNode 实现"],
  [cc, /void Int64Lowering::PreparePhiReplacement\(Node\* phi\) \{/, "PreparePhiReplacement 实现"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}

// 方法面 (cc 实现对账)
const ccMethods = [...cc.matchAll(/^(?:void|bool|int|const CallDescriptor\*) Int64Lowering::(\w+)\(/gm)].map((m) => m[1]);
if (!/^Int64Lowering::Int64Lowering\(TFGraph\* graph, MachineOperatorBuilder\* machine,/m.test(cc)) throw new Error("构造缺失");
const expect = ["LowerGraph", "GetParameterCountAfterLowering", "GetIndexNodes", "LowerNode",
  "DefaultLowering", "LowerComparison", "LowerWord64AtomicBinop", "LowerWord64AtomicNarrowOp",
  "LowerLoadOperator", "LowerStoreOperator", "LowerCallDescriptor", "ReplaceNode", "PreparePhiReplacement"];
const missing = expect.filter((m) => !ccMethods.includes(m));
if (missing.length) throw new Error(`cc 缺实现: ${missing}`);

const body = `${banner("src/compiler/int64-lowering.js", "src/compiler/int64-lowering.h + src/compiler/int64-lowering.cc", "Int64Lowering — 64 位算子拆 low/high 双 32 位 (32 位目标)")}
//! Int64Lowering: 64 位值拆 (low, high) 对, Word64 算子逐个降级。
//!
//! 规则面: State 枚举 + Replacement + 方法面 ${expect.length}。
//! 语义面 (推迟): LowerNode 的 switch (各 Word64 op → Word32 对)。

#![allow(dead_code, unused_variables)]

/// v8: src/compiler/int64-lowering.h Int64Lowering::State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
  /// kUnvisited
  Unvisited,
  /// kOnStack
  OnStack,
  /// kVisited
  Visited,
}

/// v8: src/compiler/int64-lowering.h Int64Lowering::Replacement
/// 64 位值的 32 位拆分对
pub struct Replacement {
  /// 原 low
  pub low: crate::node::Node,
  /// 原 high
  pub high: crate::node::Node,
}

/// v8: src/compiler/int64-lowering.h Int64Lowering
pub struct Int64Lowering {
  /// 原 graph_
  pub graph: *mut crate::graph::TFGraph,
  /// 原 machine_ / common_ / simplified_
  pub machine: *mut crate::machine_operator::MachineOperatorBuilder,
  pub common: *mut crate::common_operator::CommonOperatorBuilder,
  pub simplified: *mut crate::simplified_operator::SimplifiedOperatorBuilder,
  /// 原 zone_
  pub zone: *mut crate::zone::Zone,
  /// 原 signature_
  pub signature: *mut crate::Signature<crate::machine_type::MachineRepresentation>,
  /// 原 replacements_ — 拆分表 (推迟容器)
  pub _semantic: (),
}

impl Int64Lowering {
  /// v8: src/compiler/int64-lowering.cc Int64Lowering::LowerGraph
  /// 原文: 从 graph->end() 起 DFS (PushNode), 随后逐节点 LowerNode
  pub fn lower_graph(&mut self) {
    todo!("栈遍历 + 逐节点降级")
  }

  /// v8: src/compiler/int64-lowering.cc Int64Lowering::GetParameterCountAfterLowering
  /// 原文: 64 位参数计 2 槽
  pub fn get_parameter_count_after_lowering(
    signature: &crate::Signature<crate::machine_type::MachineRepresentation>,
  ) -> i32 {
    let mut parameter_count = 0;
    for i in 0..signature.parameter_count() {
      parameter_count += if signature.get_param(i) == crate::machine_type::MachineRepresentation::Word64 { 2 } else { 1 };
    }
    parameter_count
  }

  /// v8: src/compiler/int64-lowering.cc Int64Lowering::GetIndexNodes
  pub fn get_index_nodes(&mut self, index: crate::node::Node) -> (crate::node::Node, crate::node::Node) {
    todo!("索引 → (low, high) 对")
  }
  /// v8: src/compiler/int64-lowering.cc Int64Lowering::ReplaceNode
  pub fn replace_node(&mut self, old: crate::node::Node, new_low: crate::node::Node, new_high: crate::node::Node) {
    todo!("replacements_ 表登记")
  }
  /// v8: src/compiler/int64-lowering.cc Int64Lowering::PreparePhiReplacement
  pub fn prepare_phi_replacement(&mut self, phi: crate::node::Node) {
    todo!("Phi 预建 low/high 空节点")
  }
  /// v8: src/compiler/int64-lowering.cc Int64Lowering::LowerNode
  pub fn lower_node(&mut self, node: crate::node::Node) {
    todo!("Word64 op 分派 (算术/比较/load/store/call/phi)")
  }
  /// v8: src/compiler/int64-lowering.cc Int64Lowering::DefaultLowering
  /// 原默认参: low_word_only = false
  pub fn default_lowering(&mut self, node: crate::node::Node, low_word_only: bool) -> bool {
    let _ = low_word_only;
    todo!("普通节点输入替换 (low 或 low+high)")
  }
  /// v8: src/compiler/int64-lowering.cc Int64Lowering::LowerComparison
  pub fn lower_comparison(&mut self, node: crate::node::Node, signed_op: &crate::operator::Operator, unsigned_op: &crate::operator::Operator) {
    let _ = (signed_op, unsigned_op);
    todo!("比较分解: high 等价或 low 有符号/无符号比较")
  }
  /// v8: src/compiler/int64-lowering.cc Int64Lowering::LowerWord64AtomicBinop
  pub fn lower_word64_atomic_binop(&mut self, node: crate::node::Node, op: &crate::operator::Operator) {
    let _ = op;
    todo!("原子二元 op 分半两次 32 位原子")
  }
  /// v8: src/compiler/int64-lowering.cc Int64Lowering::LowerWord64AtomicNarrowOp
  pub fn lower_word64_atomic_narrow_op(&mut self, node: crate::node::Node, op: &crate::operator::Operator) {
    let _ = op;
    todo!("窄化原子 op (仅 low 半)")
  }
  /// v8: src/compiler/int64-lowering.cc Int64Lowering::LowerLoadOperator
  pub fn lower_load_operator(&mut self, node: crate::node::Node, rep: crate::machine_type::MachineRepresentation, load_op: &crate::operator::Operator) {
    let _ = (rep, load_op);
    todo!("Word64 load → 两次 Word32 load (偏移 +4)")
  }
  /// v8: src/compiler/int64-lowering.cc Int64Lowering::LowerStoreOperator
  pub fn lower_store_operator(&mut self, node: crate::node::Node, rep: crate::machine_type::MachineRepresentation, store_op: &crate::operator::Operator) {
    let _ = (rep, store_op);
    todo!("Word64 store → 两次 Word32 store")
  }
  /// v8: src/compiler/int64-lowering.cc Int64Lowering::LowerCallDescriptor
  pub fn lower_call_descriptor(&mut self, descriptor: &crate::CallDescriptor) -> *mut crate::CallDescriptor {
    let _ = descriptor;
    todo!("签名 64 位参数扩 2 槽")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "int64_lowering.rs.skel", body);
console.log(`written ${p}\nState=3 方法面=${expect.length} Replacement=low/high对`);
