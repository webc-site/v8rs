#!/usr/bin/env bun
// v8/src/compiler/add-type-assertions-reducer.h + add-type-assertions-reducer.cc
// → v8-compiler-passes/src/add_type_assertions_reducer.rs.skel
//
// 全族仅一个入口函数 + 匿名命名空间 Impl (3 方法)。规则面:
// ProcessBlock 中跳过断言插入的 opcode 清单 (7 项, 逐字核对) +
// InsertAssertion/Run/ProcessBlock 签名; 语义面 (遍历 effect 链插
// AssertType 节点) 留 todo! + /// v8: 锚。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/add-type-assertions-reducer.h");
const cc = readV8("src/compiler/add-type-assertions-reducer.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /void AddTypeAssertions\(JSGraph\* jsgraph, Schedule\* schedule, Zone\* phase_zone\);/, "入口声明"],
  [cc, /struct AddTypeAssertionsImpl \{/, "Impl 结构"],
  [cc, /void AddTypeAssertionsImpl::Run\(\) \{/, "Run"],
  [cc, /void AddTypeAssertionsImpl::ProcessBlock\(BasicBlock\* block\) \{/, "ProcessBlock"],
  [cc, /void AddTypeAssertionsImpl::InsertAssertion\(Node\* asserted,/, "InsertAssertion"],
  [cc, /type\.CanBeAsserted\(\)/, "CanBeAsserted 门槛"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}

// —— 跳过清单: ProcessBlock 中 continue 的 opcode (顺序即语义) ——
const skipBlock = cc.match(
  /if \(node->opcode\(\) == IrOpcode::kAssertType \|\|[\s\S]*?\) \{\n      continue;\n    \}/,
)[0];
const skipOpcodes = [...skipBlock.matchAll(/IrOpcode::k(\w+)/g)].map((m) => m[1]);
if (skipOpcodes.join() !== "AssertType,Allocate,ObjectState,ObjectId,Phi,Unreachable") {
  throw new Error(`跳过清单漂移: ${skipOpcodes}`);
}
// 条件余项: !NodeProperties::IsTyped(node) 混在 kPhi 与 kUnreachable 之间
if (!skipBlock.includes("|| !NodeProperties::IsTyped(node) ||")) {
  throw new Error("IsTyped 混合条件缺失");
}

const body = `${banner("src/compiler/add-type-assertions-reducer.js", "src/compiler/add-type-assertions-reducer.h + src/compiler/add-type-assertions-reducer.cc", "AddTypeAssertions — 为可断言类型节点插 AssertType (调度后遍)")}
//! AddTypeAssertions: 遍历 RPO 序基本块, 对 effect 输出/输入各 1 的节点
//! 之前的 pending 节点插类型断言。
//!
//! 规则面: 跳过 opcode 清单 ${skipOpcodes.length} 项 + IsTyped 否定。
//! 语义面 (推迟): NewNode(AssertType)/ReplaceEffectInput 图改写。

#![allow(dead_code)]

/// v8: src/compiler/add-type-assertions-reducer.cc AddTypeAssertionsImpl
struct AddTypeAssertionsImpl<'a> {
  jsgraph: &'a mut crate::js_graph::JSGraph,
  schedule: &'a crate::schedule::Schedule,
  phase_zone: &'a mut crate::zone::Zone,
}

impl AddTypeAssertionsImpl<'_> {
  /// v8: src/compiler/add-type-assertions-reducer.cc AddTypeAssertionsImpl::Run
  fn run(&mut self) {
    for block in self.schedule.rpo_order() {
      self.process_block(block);
    }
  }

  /// v8: src/compiler/add-type-assertions-reducer.cc AddTypeAssertionsImpl::ProcessBlock
  /// 跳过清单 (原文顺序): ${skipOpcodes.join(", ")} 或未定型
  fn process_block(&mut self, block: &crate::schedule::BasicBlock) {
    let mut pending: Vec<crate::node::Node> = Vec::new();
    let mut inside_of_region = false;
    let _ = (&mut pending, &mut inside_of_region, block);
    todo!("ProcessBlock: effect 链匹配 + CanBeAsserted 门槛")
  }

  /// v8: src/compiler/add-type-assertions-reducer.cc AddTypeAssertionsImpl::InsertAssertion
  fn insert_assertion(&mut self, asserted: &crate::node::Node, effect_successor: &crate::node::Node) {
    todo!("InsertAssertion: AssertType 节点拼接 + ReplaceEffectInput")
  }
}

/// v8: src/compiler/add-type-assertions-reducer.h AddTypeAssertions
pub fn add_type_assertions(
  jsgraph: &mut crate::js_graph::JSGraph,
  schedule: &crate::schedule::Schedule,
  phase_zone: &mut crate::zone::Zone,
) {
  AddTypeAssertionsImpl { jsgraph, schedule, phase_zone }.run();
}
`;

const p = writeSrc("v8-compiler-passes", "add_type_assertions_reducer.rs.skel", body);
console.log(`written ${p}\n跳过 opcode=${skipOpcodes.length} 方法=3 入口=1`);
