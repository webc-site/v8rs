#!/usr/bin/env bun
// v8/src/compiler/loop-variable-optimizer.cc + loop-variable-optimizer.h
// → v8-compiler-passes/src/loop_variable_optimizer.rs.skel
//
// 规则面: ConstraintKind/ArithmeticType 枚举、Bound 结构、InductionVariable
///字段 + LoopVariableOptimizer 方法面; induction_vars_ 以 phi NodeId 索引。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/loop-variable-optimizer.h");
const cc = readV8("src/compiler/loop-variable-optimizer.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class InductionVariable : public ZoneObject \{/, "InductionVariable"],
  [h, /enum ConstraintKind \{ kStrict, kNonStrict \};/, "ConstraintKind"],
  [h, /enum ArithmeticType \{ kAddition, kSubtraction \};/, "ArithmeticType"],
  [h, /struct Bound \{/, "Bound"],
  [h, /class LoopVariableOptimizer \{/, "LoopVariableOptimizer"],
  [h, /void ChangeToInductionVariablePhis\(\);/, "ChangeToInductionVariablePhis"],
  [cc, /void LoopVariableOptimizer::Run\(\) \{/, "Run 实现"],
  [cc, /void LoopVariableOptimizer::VisitBackedge\(Node\* from, Node\* loop\) \{/, "VisitBackedge"],
  [cc, /void LoopVariableOptimizer::VisitIf\(Node\* node, bool polarity\) \{/, "VisitIf"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}
// cc 方法面
const ccMethods = [...cc.matchAll(/^(?:void|const InductionVariable\*|InductionVariable\*) LoopVariableOptimizer::(\w+)\(/gm)].map((m) => m[1]);
if (ccMethods.length < 7) throw new Error(`cc 方法 ${ccMethods.length} < 7`);

const body = `${banner("src/compiler/loop-variable-optimizer.js", "src/compiler/loop-variable-optimizer.h + src/compiler/loop-variable-optimizer.cc", "LoopVariableOptimizer — 归纳变量识别 (phi→增量+界约束)")}
//! 归纳变量: phi = init + n*increment, 附上下界约束; 供 bound 收紧用。
//!
//! 规则面: 枚举 2 + Bound + 方法面 ${ccMethods.length}。
//! 语义面 (推迟): TryGetInductionVariable 的算术模式匹配 (cc)。

#![allow(dead_code, unused_variables)]

/// v8: src/compiler/loop-variable-optimizer.h InductionVariable::ConstraintKind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintKind {
  /// kStrict — 严格不等 (<)
  Strict,
  /// kNonStrict — 非严格 (<=)
  NonStrict,
}

/// v8: src/compiler/loop-variable-optimizer.h InductionVariable::ArithmeticType
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithmeticType {
  /// kAddition
  Addition,
  /// kSubtraction
  Subtraction,
}

/// v8: src/compiler/loop-variable-optimizer.h InductionVariable::Bound
pub struct Bound {
  /// 原 bound
  pub bound: crate::node::Node,
  /// 原 kind
  pub kind: ConstraintKind,
}

/// v8: src/compiler/loop-variable-optimizer.h InductionVariable
pub struct InductionVariable {
  /// 原 phi_ — 归纳变量 phi
  pub phi: crate::node::Node,
  /// 原 effect_phi_ — 同循环节点
  pub effect_phi: crate::node::Node,
  /// 原 arith_ — 增量算术节点
  pub arith: crate::node::Node,
  /// 原 increment_
  pub increment: crate::node::Node,
  /// 原 init_value_
  pub init_value: crate::node::Node,
  /// 原 lower_bounds_ / upper_bounds_
  pub lower_bounds: Vec<Bound>,
  pub upper_bounds: Vec<Bound>,
  /// 原 arithmeticType_
  pub arithmetic_type: ArithmeticType,
}

impl InductionVariable {
  /// v8: src/compiler/loop-variable-optimizer.h InductionVariable::phi 等 getter 5 项
  pub fn phi(&self) -> crate::node::Node {
    self.phi
  }
  /// v8: src/compiler/loop-variable-optimizer.h InductionVariable::Type
  pub fn ty(&self) -> ArithmeticType {
    self.arithmetic_type
  }
  /// v8: src/compiler/loop-variable-optimizer.h InductionVariable::AddUpperBound
  pub fn add_upper_bound(&mut self, bound: crate::node::Node, kind: ConstraintKind) {
    self.upper_bounds.push(Bound { bound, kind });
  }
  /// v8: src/compiler/loop-variable-optimizer.h InductionVariable::AddLowerBound
  pub fn add_lower_bound(&mut self, bound: crate::node::Node, kind: ConstraintKind) {
    self.lower_bounds.push(Bound { bound, kind });
  }
}

/// v8: src/compiler/loop-variable-optimizer.h LoopVariableOptimizer
pub struct LoopVariableOptimizer {
  /// 原 graph_ / common_ / zone_
  pub graph: *mut crate::graph::TFGraph,
  pub common: *mut crate::common_operator::CommonOperatorBuilder,
  pub zone: *mut crate::zone::Zone,
  /// 原 induction_vars_ — phi NodeId → 归纳变量
  pub induction_vars: std::collections::BTreeMap<i32, *mut InductionVariable>,
}

impl LoopVariableOptimizer {
  /// v8: src/compiler/loop-variable-optimizer.cc LoopVariableOptimizer::Run
  /// 原文: 逐 loop 头 DetectInductionVariables
  pub fn run(&mut self) {
    todo!("遍历 Loop 节点")
  }

  /// v8: src/compiler/loop-variable-optimizer.h induction_variables
  pub fn induction_variables(&self) -> &std::collections::BTreeMap<i32, *mut InductionVariable> {
    &self.induction_vars
  }

  /// v8: src/compiler/loop-variable-optimizer.cc ChangeToInductionVariablePhis
  pub fn change_to_induction_variable_phis(&mut self) {
    todo!("phi op → InductionVariablePhi")
  }
  /// v8: src/compiler/loop-variable-optimizer.cc ChangeToPhisAndInsertGuards
  pub fn change_to_phis_and_insert_guards(&mut self) {
    todo!("还原 phi + 插入界守卫")
  }

  /// v8: src/compiler/loop-variable-optimizer.cc VisitBackedge
  pub fn visit_backedge(&mut self, from: crate::node::Node, to: crate::node::Node) {
    let _ = (from, to);
    todo!("回边处更新约束")
  }
  /// v8: src/compiler/loop-variable-optimizer.cc FindInductionVariable
  pub fn find_induction_variable(&self, node: &crate::node::Node) -> *const InductionVariable {
    let _ = node;
    todo!("phi id 查表")
  }
  /// v8: src/compiler/loop-variable-optimizer.cc TryGetInductionVariable
  pub fn try_get_induction_variable(&mut self, phi: crate::node::Node) -> *mut InductionVariable {
    let _ = phi;
    todo!("phi 算术模式匹配构造")
  }
  /// v8: src/compiler/loop-variable-optimizer.cc DetectInductionVariables
  pub fn detect_induction_variables(&mut self, loop_node: crate::node::Node) {
    let _ = loop_node;
    todo!("循环体 phi 扫描")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "loop_variable_optimizer.rs.skel", body);
console.log(`written ${p}\n枚举=2 (Constraint2/Arithmetic2) Bound=1 cc方法=${ccMethods.length}`);
