#!/usr/bin/env bun
// v8/src/compiler/common-operator-reducer.cc + common-operator-reducer.h
// → v8-compiler-passes/src/common_operator_reducer.rs.skel
//
// 规则面: Reduce 分派表 (12 opcode) + 10 私有 Reduce 方法 + Change×2 +
// DecideCondition/BranchSemanticsOf 签名; Decision 语义 (12.1 锚)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/common-operator-reducer.h");
const cc = readV8("src/compiler/common-operator-reducer.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class V8_EXPORT_PRIVATE CommonOperatorReducer final/, "CommonOperatorReducer"],
  [h, /Reduction ReduceStaticAssert\(Node\* node\);/, "ReduceStaticAssert"],
  [h, /Decision DecideCondition\(Node\* const cond, BranchSemantics branch_semantics\);/, "DecideCondition"],
  [h, /BranchSemantics default_branch_semantics_;/, "默认分支语义字段"],
  [cc, /Reduction CommonOperatorReducer::Reduce\(Node\* node\) \{/, "Reduce 实现"],
  [cc, /Decision CommonOperatorReducer::DecideCondition\(/, "DecideCondition 实现"],
  [cc, /case IrOpcode::kStaticAssert:/, "分派 StaticAssert"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}

// —— 分派表 (Reduce switch 区) ——
const switchSrc = cc.slice(
  cc.indexOf("Reduction CommonOperatorReducer::Reduce(Node* node) {"),
  cc.indexOf("Decision CommonOperatorReducer::DecideCondition"),
);
const cases = [...switchSrc.matchAll(/case IrOpcode::k(\w+):/g)].map((m) => m[1]);
if (cases.join() !== "Branch,DeoptimizeIf,DeoptimizeUnless,Merge,EffectPhi,Phi,Return,Select,Switch,StaticAssert,TrapIf,TrapUnless") {
  throw new Error(`分派表漂移: ${cases}`);
}
// DecideCondition 内的常量判读 case
const dcSrc = cc.slice(cc.indexOf("Decision CommonOperatorReducer::DecideCondition"), cc.indexOf("Reduction CommonOperatorReducer::ReduceBranch"));
const dcCases = [...dcSrc.matchAll(/case IrOpcode::k(\w+):/g)].map((m) => m[1]);
if (dcCases.join() !== "Int32Constant,HeapConstant") throw new Error(`DecideCondition case 漂移: ${dcCases}`);
// 私有方法 (cc 实现对账)
const ccMethods = [...cc.matchAll(/^(?:Reduction|Decision|BranchSemantics) CommonOperatorReducer::(\w+)\(/gm)].map((m) => m[1]);
const expect = ["Reduce", "ReduceBranch", "ReduceDeoptimizeConditional", "ReduceMerge", "ReduceEffectPhi", "ReducePhi", "ReduceReturn", "ReduceSelect", "ReduceSwitch", "ReduceStaticAssert", "ReduceTrapConditional", "Change", "DecideCondition"];
const missing = expect.filter((m) => !ccMethods.includes(m));
if (missing.length) throw new Error(`cc 缺实现: ${missing}`);
// Change 重载 2 个
if (ccMethods.filter((m) => m === "Change").length !== 2) throw new Error("Change 重载 != 2");

const body = `${banner("src/compiler/common-operator-reducer.js", "src/compiler/common-operator-reducer.h + src/compiler/common-operator-reducer.cc", "CommonOperatorReducer — 通用算子强度削减 (Branch/Phi/Select/Return…)")}
//! CommonOperatorReducer: 常量条件折叠、Phi 化简、Merge/EffectPhi 收缩等。
//!
//! 规则面: 分派表 ${cases.length} case + DecideCondition 常量判读 case 2 +
//! ${expect.length} 方法 + Change 双重载。语义面 (推迟): 图改写。

#![allow(dead_code, non_camel_case_types, unused_variables)]

/// v8: src/compiler/common-operator-reducer.h CommonOperatorReducer
pub struct CommonOperatorReducer {
  graph: *mut crate::graph::TFGraph,
  broker: *mut crate::broker::JSHeapBroker,
  common: *mut crate::common_operator::CommonOperatorBuilder,
  machine: *mut crate::machine_operator::MachineOperatorBuilder,
  dead: crate::node::Node,
  zone: *mut crate::zone::Zone,
  /// 原 default_branch_semantics_
  default_branch_semantics: crate::BranchSemantics,
}

impl CommonOperatorReducer {
  /// v8: src/compiler/common-operator-reducer.h CommonOperatorReducer::CommonOperatorReducer
  pub fn new(
    editor: &mut dyn crate::graph_reducer::Editor,
    graph: *mut crate::graph::TFGraph,
    broker: *mut crate::broker::JSHeapBroker,
    common: *mut crate::common_operator::CommonOperatorBuilder,
    machine: *mut crate::machine_operator::MachineOperatorBuilder,
    temp_zone: *mut crate::zone::Zone,
    default_branch_semantics: crate::BranchSemantics,
  ) -> Self {
    todo!("dead 节点创建")
  }

  /// v8: src/compiler/common-operator-reducer.cc CommonOperatorReducer::reducer_name
  pub const REDUCER_NAME: &'static str = "CommonOperatorReducer";

  /// v8: src/compiler/common-operator-reducer.cc CommonOperatorReducer::Reduce
  /// 分派表 (原文顺序): ${cases.join(", ")}
  pub fn reduce(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    match node.opcode() {
      ${cases.map((c) => `crate::IrOpcode::${c} => todo!("k${c}")`).join(",\n      ")}
      _ => crate::graph_reducer::Reduction::NoChange,
    }
  }

  /// v8: src/compiler/common-operator-reducer.cc CommonOperatorReducer::ReduceBranch
  pub fn reduce_branch(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("常量条件 → IfTrue/IfFalse 直通; SwapInputs 对称化")
  }
  /// v8: src/compiler/common-operator-reducer.cc CommonOperatorReducer::ReduceDeoptimizeConditional
  pub fn reduce_deoptimize_conditional(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("DeoptimizeIf/Unless 条件判读 → 消除或无条件化")
  }
  /// v8: src/compiler/common-operator-reducer.cc CommonOperatorReducer::ReduceMerge
  pub fn reduce_merge(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("单输入 Merge 抽取")
  }
  /// v8: src/compiler/common-operator-reducer.cc CommonOperatorReducer::ReduceEffectPhi
  pub fn reduce_effect_phi(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("单输入 EffectPhi 抽取")
  }
  /// v8: src/compiler/common-operator-reducer.cc CommonOperatorReducer::ReducePhi
  pub fn reduce_phi(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("常量 Phi 折叠 / 输入去重")
  }
  /// v8: src/compiler/common-operator-reducer.cc CommonOperatorReducer::ReduceReturn
  pub fn reduce_return(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("Return 输入规范化")
  }
  /// v8: src/compiler/common-operator-reducer.cc CommonOperatorReducer::ReduceSelect
  pub fn reduce_select(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("Select 条件判读 → vtrue/vfalse")
  }
  /// v8: src/compiler/common-operator-reducer.cc CommonOperatorReducer::ReduceSwitch
  pub fn reduce_switch(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("Switch 常量 case 直跳")
  }
  /// v8: src/compiler/common-operator-reducer.cc CommonOperatorReducer::ReduceStaticAssert
  pub fn reduce_static_assert(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("静态断言常量判读 → 移除或保留")
  }
  /// v8: src/compiler/common-operator-reducer.cc CommonOperatorReducer::ReduceTrapConditional
  pub fn reduce_trap_conditional(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("TrapIf/Unless 条件判读")
  }

  /// v8: src/compiler/common-operator-reducer.cc CommonOperatorReducer::Change (单输入)
  pub fn change(&mut self, node: &crate::node::Node, op: &crate::operator::Operator, a: crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("换 op + 单输入替换")
  }
  /// v8: src/compiler/common-operator-reducer.cc CommonOperatorReducer::Change (双输入)
  pub fn change2(&mut self, node: &crate::node::Node, op: &crate::operator::Operator, a: crate::node::Node, b: crate::node::Node) -> crate::graph_reducer::Reduction {
    todo!("换 op + 双输入替换")
  }

  /// v8: src/compiler/common-operator-reducer.cc CommonOperatorReducer::DecideCondition
  /// 原文: Int32Constant(0)→Unknown/False; Int32Constant(bit set)→True;
  /// HeapConstant(oddbool true/false)→True/False; 其余 Unknown
  pub fn decide_condition(&self, cond: &crate::node::Node, branch_semantics: crate::BranchSemantics) -> crate::Decision {
    todo!("常量判读三分支")
  }

  /// v8: src/compiler/common-operator-reducer.h CommonOperatorReducer::BranchSemanticsOf
  /// 原文: BranchParametersOf(op).semantics() 非 kUnspecified 用之,
  /// 否则 default_branch_semantics_
  pub fn branch_semantics_of(&self, branch: &crate::node::Node) -> crate::BranchSemantics {
    let bs = crate::common_operator::branch_parameters_of(branch.op()).semantics();
    if bs != crate::BranchSemantics::Unspecified { bs } else { self.default_branch_semantics }
  }
}
`;

const p = writeSrc("v8-compiler-passes", "common_operator_reducer.rs.skel", body);
console.log(`written ${p}\n分派case=${cases.length} cc方法=${ccMethods.length} (含 Change 重载 2)`);
