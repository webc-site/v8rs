#!/usr/bin/env bun
// tool/src/compiler/representation-change.h.js
// v8/src/compiler/representation-change.h
// → v8-compiler-passes/src/representation_change.rs.skel
// 表示变更器: 解析 *OperatorFor 映射族 / Get*RepresentationFor 分派族 /
// Insert* 插入族三类方法面。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/representation-change.h");

const need = (cond, msg) => {
  if (!cond) throw new Error(`representation-change: ${msg}`);
};

for (const [s, name] of [
  ["class V8_EXPORT_PRIVATE RepresentationChanger final {", "类定义"],
  ["Node* GetRepresentationFor(Node* node, MachineRepresentation output_rep,", "GetRepresentationFor"],
  ["const Operator* Int32OperatorFor(IrOpcode::Value opcode);", "Int32OperatorFor"],
]) need(h.includes(s), `哨兵缺失 ${name}`);

// —— *OperatorFor 映射族 (真解析; 括号内换行归一) ——
const hNorm = h.replace(/\(\s*\n\s*/g, "(").replace(/\)\s*\n\s*/g, ") ");
const opFors = [...hNorm.matchAll(/const Operator\* (\w+OperatorFor)\(IrOpcode::Value opcode\);/g)].map((m) => m[1]);
need(opFors.length === 10, `OperatorFor 族 ${opFors.length} (${opFors.join(",")})`);
for (const n of ["Int32OperatorFor", "Int32OverflowOperatorFor", "AdditiveSafeIntegerOverflowOperatorFor", "Int64OperatorFor", "Int64OverflowOperatorFor", "BigIntOperatorFor", "TaggedSignedOperatorFor", "Uint32OperatorFor", "Uint32OverflowOperatorFor", "Float64OperatorFor"]) {
  need(opFors.includes(n), `${n} 缺席`);
}

// —— Get*RepresentationFor 分派族 ——
const getReps = [...hNorm.matchAll(/Node\* (Get\w+RepresentationFor)\(Node\* node,/g)].map((m) => m[1]);
need(getReps.length === 9, `Get*RepresentationFor ${getReps.length} (${getReps.join(",")})`);

// —— Insert* 插入族 ——
const inserts = [...h.matchAll(/Node\* (Insert\w+)\(Node\* node/g)].map((m) => m[1]);
need(inserts.length >= 14, `Insert 族 ${inserts.length}`);

// —— SetOncePointer 字段 (IEEE754 fp16) ——
const fp16Fields = [...h.matchAll(/SetOncePointer<[\w <>]+>\s+(\w+);/g)].map((m) => m[1]);
need(fp16Fields.length === 4, `SetOnce 字段 ${fp16Fields.length}`);

const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

const body = `${banner("src/compiler/representation-change.h.js", "src/compiler/representation-change.h", "机器表示变更与 Simplified→Machine 转换")}
//! representation-change — GetRepresentationFor 总分派
//!
//! 三层: 10 个 *OperatorFor (目标表示的等价机器算子) /
//! ${getReps.length} 个 Get*RepresentationFor (目标表示分派) /
//! ${inserts.length} 个 Insert* (插入转换节点);
//! 常量折叠先行, 类型错误走 TypeError 收集。

/// 原版 RepresentationChanger — 表示变更器
pub struct RepresentationChanger<'z, 'g> {
  /// cache_
  pub cache: *const TypeCache,
  /// jsgraph_
  pub jsgraph: &'g JSGraph<'z>,
  /// broker_
  pub broker: *mut JSHeapBroker,
  /// verifier_
  pub verifier: Option<Box<SimplifiedLoweringVerifier<'z, 'g>>>,
  /// testing_type_errors_
  pub testing_type_errors: bool,
  /// type_error_
  pub type_error: bool,
  /// ieee754_fp16_raw_bits_to_fp32_raw_bits_code_ 等 4 个 SetOnce 缓存 (${fp16Fields.join("/")})
  pub ieee754_caches: [SetOncePtr<Node>; 4],
}

impl RepresentationChanger<'_, '_> {
  /// 原版 GetRepresentationFor — 总入口 (output_rep/output_type → use_info)
  /// v8: src/compiler/representation-change.cc RepresentationChanger::GetRepresentationFor
  pub fn get_representation_for(&mut self, node: &Node, output_rep: MachineRepresentation, output_type: Type, use_node: &Node, use_info: UseInfo) -> *mut Node {
    todo!("v8: src/compiler/representation-change.cc RepresentationChanger::GetRepresentationFor")
  }

${opFors.map((n) => `  /// 原版 ${n} — opcode → 目标表示机器算子\n  /// v8: src/compiler/representation-change.cc RepresentationChanger::${n}\n  pub fn ${snake(n)}(&self, opcode: IrOpcode) -> &Operator {\n    todo!("v8: src/compiler/representation-change.cc RepresentationChanger::${n}")\n  }`).join("\n\n")}

${getReps.map((n) => `  /// 原版 ${n}\n  /// v8: src/compiler/representation-change.cc RepresentationChanger::${n}\n  fn ${snake(n)}(&mut self, node: &Node, output_rep: MachineRepresentation, output_type: Type, use_node: &Node, use_info: UseInfo) -> *mut Node {\n    todo!("v8: src/compiler/representation-change.cc RepresentationChanger::${n}")\n  }`).join("\n\n")}

  /// 原版 TypeError — 类型错误记录 (testing 模式不 abort)
  /// v8: src/compiler/representation-change.cc RepresentationChanger::TypeError
  fn type_error(&mut self, node: &Node, output_rep: MachineRepresentation, output_type: Type, use: MachineRepresentation) -> *mut Node {
    todo!("v8: src/compiler/representation-change.cc RepresentationChanger::TypeError")
  }

  /// 原版 Insert* 插入族全清单 (各自骨架)
${inserts.map((n) => `  /// v8: src/compiler/representation-change.cc RepresentationChanger::${n}\n  fn ${snake(n)}(&mut self, node: &Node /*, ... */) -> *mut Node {\n    todo!("v8: src/compiler/representation-change.cc RepresentationChanger::${n}")\n  }`).join("\n\n")}
}
`;

const p = writeSrc("v8-compiler-passes", "representation_change.rs.skel", body);
console.log(`written ${p}\nop_for=${opFors.length} get_rep=${getReps.length} insert=${inserts.length} set_once_fields=${fp16Fields.length}`);
