#!/usr/bin/env bun
// tool/src/compiler/simplified-lowering-verifier.js
// v8/src/compiler/simplified-lowering-verifier.h + simplified-lowering-verifier.cc
// → v8-compiler-passes/src/simplified_lowering_verifier.rs.skel
// 降低校验器: 解析 PerNodeData / Check* / Join* 方法面与 cc 对数。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/simplified-lowering-verifier.h");
const cc = readV8("src/compiler/simplified-lowering-verifier.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`simplified-lowering-verifier: ${msg}`);
};

for (const [s, name] of [
  ["class SimplifiedLoweringVerifier final {", "类定义"],
  ["struct PerNodeData {", "PerNodeData"],
  ["void VisitNode(Node* node, OperationTyper& op_typer);", "VisitNode"],
  ["Truncation GeneralizeTruncation(const Truncation& truncation,", "GeneralizeTruncation"],
]) need(h.includes(s), `哨兵缺失 ${name}`);

// —— PerNodeData 字段 (真解析) ——
const pnd = h.slice(h.indexOf("struct PerNodeData {"));
const pndFields = [...pnd.slice(0, pnd.indexOf("};")).matchAll(/^\s{4}(std::optional<Type>|Truncation) (\w+)(?: = [^;]+)?;/gm)].map((m) => m[2]);
need(pndFields.length === 2 && pndFields.includes("type") && pndFields.includes("truncation"), `PerNodeData ${pndFields}`);

// —— Check/Join/Set 面 ——
const checks = [...h.matchAll(/^\s{2}void (Check\w+)\(Node\* node/gm)].map((m) => m[1]);
const joins = [...h.matchAll(/^\s{2}Truncation (Join\w+)\(/gm)].map((m) => m[1]);
need(checks.length >= 2, `Check 面 ${checks.length}`);
need(joins.length >= 1, `Join 面 ${joins.length}`);

// —— cc 定义对数 ——
const ccDefs = [
  "void SimplifiedLoweringVerifier::VisitNode(",
  "void SimplifiedLoweringVerifier::CheckType(",
  "void SimplifiedLoweringVerifier::CheckAndSet(",
  "void SimplifiedLoweringVerifier::ReportInvalidTypeCombination(",
  "Truncation SimplifiedLoweringVerifier::GeneralizeTruncation(",
];
for (const d of ccDefs) need(cc.includes(d), `cc 缺 ${d}`);
const ccJoin = [...cc.matchAll(/Truncation SimplifiedLoweringVerifier::Join/g)].length;
need(ccJoin >= 1, "cc JoinTruncation 缺席");

// —— 默认截断语义 (真解析: Any + kDistinguishZeros) ——
need(h.includes("Truncation::Any(IdentifyZeros::kDistinguishZeros)"), "默认截断哨兵");

const body = `${banner("src/compiler/simplified-lowering-verifier.js", "src/compiler/simplified-lowering-verifier.h + simplified-lowering-verifier.cc", "SL 后的类型/截断一致性校验")}
//! simplified-lowering-verifier — 降低后按 OperationTyper 复推类型比对
//!
//! PerNodeData = (type?, truncation); 缺省截断 Any[kDistinguishZeros];
//! 机器常量的静态类型不采 (机器/非机器子图共享缓存)。

/// 原版 PerNodeData — 每节点 (type, truncation)
pub struct PerNodeData {
  /// type = std::nullopt
  pub type_: Option<Type>,
  /// truncation = Truncation::Any(kDistinguishZeros)
  pub truncation: Truncation,
}

/// 原版 SimplifiedLoweringVerifier — 校验器
pub struct SimplifiedLoweringVerifier<'z, 'g> {
  /// hints_ — 插入的 SLVerifierHint 节点
  pub hints: ZoneVector<*mut Node>,
  /// machine_uses_of_constants_ — 机器常量的使用方
  pub machine_uses_of_constants: ZoneUnorderedMap<*mut Node, ZoneVector<*mut Node>>,
  /// data_ — NodeId 索引
  pub data: ZoneVector<PerNodeData>,
  /// graph_
  pub graph: &'g TFGraph<'z>,
  /// zone_
  pub zone: &'z Zone<'z>,
}

impl SimplifiedLoweringVerifier<'_, '_> {
  /// 原版 VisitNode — 逐节点重推类型/截断并核对
  /// v8: src/compiler/simplified-lowering-verifier.cc SimplifiedLoweringVerifier::VisitNode
  pub fn visit_node(&mut self, node: &Node, op_typer: &mut OperationTyper<'_>) {
    todo!("v8: src/compiler/simplified-lowering-verifier.cc SimplifiedLoweringVerifier::VisitNode")
  }

  /// 原版 GetType — 静态类型优先 (机器常量除外), 否则查推断表
  /// v8: src/compiler/simplified-lowering-verifier.h SimplifiedLoweringVerifier::GetType
  pub fn get_type(&self, node: &Node) -> Option<Type> {
    todo!("v8: src/compiler/simplified-lowering-verifier.h SimplifiedLoweringVerifier::GetType")
  }

${checks.map((c) => `  /// 原版 ${c}\n  /// v8: src/compiler/simplified-lowering-verifier.cc SimplifiedLoweringVerifier::${c}\n  fn ${c.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}(&mut self, node: &Node /*, type */) {\n    todo!("v8: src/compiler/simplified-lowering-verifier.cc SimplifiedLoweringVerifier::${c}")\n  }`).join("\n\n")}

  /// 原版 InputType — 输入类型 (缺省 None)
  /// v8: src/compiler/simplified-lowering-verifier.h SimplifiedLoweringVerifier::InputType
  fn input_type(&self, node: &Node, input_index: usize) -> Type {
    todo!("v8: src/compiler/simplified-lowering-verifier.h SimplifiedLoweringVerifier::InputType")
  }

  /// 原版 GeneralizeTruncation — 依类型泛化截断 (避免组合爆炸)
  /// v8: src/compiler/simplified-lowering-verifier.cc SimplifiedLoweringVerifier::GeneralizeTruncation
  fn generalize_truncation(&self, truncation: &Truncation, type_: &Type) -> Truncation {
    todo!("v8: src/compiler/simplified-lowering-verifier.cc SimplifiedLoweringVerifier::GeneralizeTruncation")
  }

  /// 原版 JoinTruncation — 截断合并 (${joins.length} 重载)
  /// v8: src/compiler/simplified-lowering-verifier.cc SimplifiedLoweringVerifier::JoinTruncation
  fn join_truncation(&self, t1: &Truncation, t2: &Truncation /*, t3 */) -> Truncation {
    todo!("v8: src/compiler/simplified-lowering-verifier.cc SimplifiedLoweringVerifier::JoinTruncation")
  }

  /// 原版 ReportInvalidTypeCombination — 校验失败报告
  /// v8: src/compiler/simplified-lowering-verifier.cc SimplifiedLoweringVerifier::ReportInvalidTypeCombination
  fn report_invalid_type_combination(&self, node: &Node, types: &[Type]) {
    todo!("v8: src/compiler/simplified-lowering-verifier.cc SimplifiedLoweringVerifier::ReportInvalidTypeCombination")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "simplified_lowering_verifier.rs.skel", body);
console.log(`written ${p}\nper_node_data=2 checks=${checks.length} joins=${joins.length} cc_defs=${ccDefs.length}+join=${ccJoin}`);
