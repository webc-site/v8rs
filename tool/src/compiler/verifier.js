#!/usr/bin/env bun
// tool/src/compiler/verifier.js
// v8/src/compiler/verifier.h + verifier.cc
// → v8-compiler-passes/src/verifier.rs.skel
// 图/调度校验器: 解析 Typing/CheckInputs/CodeType 三枚举与 Visitor 方法面。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/verifier.h");
const cc = readV8("src/compiler/verifier.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`verifier: ${msg}`);
};

for (const [s, name] of [
  ["class Verifier {", "Verifier"],
  ["enum Typing { TYPED, UNTYPED };", "Typing 枚举"],
  ["enum CheckInputs { kValuesOnly, kAll };", "CheckInputs 枚举"],
  ["enum CodeType { kDefault, kWasm };", "CodeType 枚举"],
  ["static void Run(TFGraph* graph, Typing typing = TYPED,", "Run 签名"],
  ["class V8_EXPORT_PRIVATE ScheduleVerifier {", "ScheduleVerifier"],
  ["void Verifier::Run(TFGraph* graph, Typing typing,", "cc Run"],
  ["void ScheduleVerifier::Run(Schedule* schedule)", "cc ScheduleVerifier Run"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// —— cc Visitor 的 Check 方法清单 (真解析) ——
const checkMethods = [...cc.matchAll(/^\s{2}void (Check\w*)\(/gm)].map((mm) => mm[1]);
for (const needM of ["Check", "CheckSwitch", "CheckNotTyped", "CheckTypeIs", "CheckTypeMaybe", "CheckValueInputIs", "CheckOutput"]) {
  need(checkMethods.includes(needM), `Visitor ${needM} 缺席`);
}

// —— cc 内 Check 的 opcode 分派 case 数 ——
const checkBody = cc.slice(cc.indexOf("void Verifier::Visitor::Check(Node* node, const AllNodes& all)"));
const cases = [...checkBody.slice(0, checkBody.indexOf("\n}", 100)).matchAll(/case IrOpcode::k(\w+):/g)].map((mm) => mm[1]);
need(cases.length > 20, `Check case 数 ${cases.length}`);

// —— cc 顶层定义清单 (CheckOutput 为头文件内联实现) ——
const ccDefs = [
  "void Verifier::VerifyNode(Node* node)",
  "void Verifier::VerifyEdgeInputReplacement(",
  "void Verifier::Visitor::CheckSwitch(Node* node, const AllNodes& all)",
  "void Verifier::Visitor::Check(Node* node, const AllNodes& all)",
  "static void CheckInputsDominate(Schedule* schedule, BasicBlock* block,",
];
for (const d of ccDefs) need(cc.includes(d), `cc 缺 ${d}`);

const body = `${banner("src/compiler/verifier.js", "src/compiler/verifier.h + verifier.cc", "DEBUG 图良构校验与调度校验")}
//! verifier — 图不变式校验 (输入/输出计数, effect/control/framestate 链,
//! phi 前置, 支配性); Release 构建全部为空操作。

/// 原版 Verifier::Typing — 是否检查类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Typing {
  /// TYPED
  #[default]
  Typed,
  /// UNTYPED
  Untyped,
}

/// 原版 Verifier::CheckInputs — 输入检查范围
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CheckInputs {
  /// kValuesOnly
  ValuesOnly,
  /// kAll
  #[default]
  All,
}

/// 原版 Verifier::CodeType — 代码类别 (默认 JS / wasm)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CodeType {
  /// kDefault
  #[default]
  Default,
  /// kWasm
  Wasm,
}

/// 原版 Verifier — 静态门面 (AllStatic)
pub struct Verifier;

impl Verifier {
  /// 原版 Run — 全图遍历校验 (默认 TYPED + kAll + kDefault)
  /// v8: src/compiler/verifier.cc Verifier::Run
  pub fn run(graph: &TFGraph, typing: Typing, check_inputs: CheckInputs, code_type: CodeType) {
    todo!("v8: src/compiler/verifier.cc Verifier::Run")
  }

  /// 原版 VerifyNode — 单节点不变式 (仅 DEBUG 构建有体)
  /// v8: src/compiler/verifier.cc Verifier::VerifyNode
  pub fn verify_node(node: &Node) {
    // Release: 原版空体
  }

  /// 原版 VerifyEdgeInputReplacement — 替换节点输出满足边需求
  /// v8: src/compiler/verifier.cc Verifier::VerifyEdgeInputReplacement
  pub fn verify_edge_input_replacement(edge: &Edge, replacement: &Node) {
    // Release: 原版空体
  }
}

/// 原版 Verifier::Visitor — 逐节点检查面 (cc 实现)
/// Check 方法清单 (${checkMethods.length} 个): ${checkMethods.join(", ")}
/// Check 分派 opcode case: ${cases.length} 个
pub struct VerifierVisitor<'g> {
  /// typing (Visitor 模板参数)
  pub typing: Typing,
  /// code_type
  pub code_type: CodeType,
  /// check_inputs
  pub check_inputs: CheckInputs,
  /// graph
  pub graph: &'g TFGraph,
}

impl VerifierVisitor<'_> {
  /// 原版 Visitor::Check — 单节点分派 (opcode case 展开)
  /// v8: src/compiler/verifier.cc Verifier::Visitor::Check
  pub fn check(&mut self, node: &Node, all: &AllNodes) {
    todo!("v8: src/compiler/verifier.cc Verifier::Visitor::Check (${cases.length} case)")
  }

  /// 原版 Visitor::CheckSwitch — Switch 节点专属检查
  /// v8: src/compiler/verifier.cc Verifier::Visitor::CheckSwitch
  pub fn check_switch(&mut self, node: &Node, all: &AllNodes) {
    todo!("v8: src/compiler/verifier.cc Verifier::Visitor::CheckSwitch")
  }

  /// 原版 Visitor::CheckOutput — 使用方需求校验
  /// v8: src/compiler/verifier.cc Verifier::Visitor::CheckOutput
  pub fn check_output(&mut self, node: &Node, r#use: &Node, count: usize, kind: &str) {
    todo!("v8: src/compiler/verifier.cc Verifier::Visitor::CheckOutput")
  }
}

/// 原版 ScheduleVerifier — 调度校验 (支配性/phi 前置)
pub struct ScheduleVerifier;

impl ScheduleVerifier {
  /// 原版 Run
  /// v8: src/compiler/verifier.cc ScheduleVerifier::Run
  pub fn run(schedule: &Schedule) {
    todo!("v8: src/compiler/verifier.cc ScheduleVerifier::Run")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "verifier.rs.skel", body);
console.log(`written ${p}\ncheck_methods=${checkMethods.length} verify_cases=${cases.length} enums=3 cc_defs=${ccDefs.length}`);
