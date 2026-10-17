#!/usr/bin/env bun
// tool/src/compiler/wasm-graph-assembler.js
// v8/src/compiler/wasm-graph-assembler.h + wasm-graph-assembler.cc
// → v8-compiler-passes/src/wasm_graph_assembler.rs.skel
// wasm 图组装器: 解析 builtin 调用面 (跳表/直调) 与节点工厂清单。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/wasm-graph-assembler.h");
const cc = readV8("src/compiler/wasm-graph-assembler.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`wasm-graph-assembler: ${msg}`);
};

for (const [s, name] of [
  ["class WasmGraphAssembler : public GraphAssembler {", "类定义"],
  ["Node* CallBuiltinThroughJumptable(Builtin builtin,", "跳表调用"],
  ["Node* GetBuiltinPointerTarget(Builtin builtin) {", "指针目标"],
  ["BranchSemantics::kMachine", "机器分支语义"],
]) need(h.includes(s), `哨兵缺失 ${name}`);

// —— builtin 调用面 (真解析: 模板直调 + 跳表 + LoadImmutable + 辅助) ——
const builtinCalls = [...h.matchAll(/Node\* Call(Builtin\w*)\(/g)].map((m) => m[1]);
need(builtinCalls.includes("Builtin") && builtinCalls.includes("BuiltinThroughJumptable"), `builtin 面 ${builtinCalls}`);
// 节点工厂清单 (cc 71 个定义对数)
const ccFns = [...cc.matchAll(/^Node\* WasmGraphAssembler::(\w+)\(/gm)].map((m) => m[1]);
need(ccFns.length >= 60, `cc 工厂 ${ccFns.length}`);
for (const f of ["LoadImmutableFromObject", "LoadFromObject", "BuildDecodeTrustedPointer", "ArrayGet", "AssertNotNull", "Branch"]) {
  need(ccFns.includes(f) || h.includes(`Node* ${f}(`), `${f} 缺席`);
}
// 头内 inline 工厂
const hFns = [...h.matchAll(/^  Node\* (\w+)\(/gm)].map((m) => m[1]);

const body = `${banner("src/compiler/wasm-graph-assembler.js", "src/compiler/wasm-graph-assembler.h + wasm-graph-assembler.cc", "wasm 专用图组装器")}
//! wasm-graph-assembler — GraphAssembler 扩展 (机器分支语义)
//!
//! builtin 调用双形态: 直调 (近跳转更短) 与跳表槽;
//! cc 定义节点工厂 ${ccFns.length} 个 (MemBuffer/MemStart/LoadMem/StoreMem/异常解码等)。

/// 原版 WasmGraphAssembler — wasm 图组装器
pub struct WasmGraphAssembler<'z> {
  /// simplified_ (builder)
  pub simplified: SimplifiedOperatorBuilder<'z>,
  // 继承面: GraphAssembler(mcgraph, zone, BranchSemantics::kMachine)
}

impl WasmGraphAssembler<'_> {
  /// 原版 CallBuiltinThroughJumptable — 经本模块跳表槽近调用 builtin
  /// v8: src/compiler/wasm-graph-assembler.h WasmGraphAssembler::CallBuiltinThroughJumptable
  pub fn call_builtin_through_jumptable(&mut self, builtin: Builtin, properties: OperatorProperties, args: &[&Node]) -> &Node {
    todo!("v8: src/compiler/wasm-graph-assembler.h WasmGraphAssembler::CallBuiltinThroughJumptable (kCallWasmRuntimeStub)")
  }

  /// 原版 GetBuiltinPointerTarget — BuiltinPtr 常量 (Smi 编号)
  /// v8: src/compiler/wasm-graph-assembler.h WasmGraphAssembler::GetBuiltinPointerTarget
  pub fn get_builtin_pointer_target(&mut self, builtin: Builtin) -> &Node {
    todo!("v8: src/compiler/wasm-graph-assembler.h WasmGraphAssembler::GetBuiltinPointerTarget")
  }

  /// 原版 CallBuiltin — 直调 builtin
  /// v8: src/compiler/wasm-graph-assembler.h WasmGraphAssembler::CallBuiltin
  pub fn call_builtin(&mut self, name: Builtin, properties: OperatorProperties, args: &[&Node]) -> &Node {
    todo!("v8: src/compiler/wasm-graph-assembler.h WasmGraphAssembler::CallBuiltin")
  }

  /// 原版 LoadFromObject — 对象加载
  /// v8: src/compiler/wasm-graph-assembler.cc WasmGraphAssembler::LoadFromObject
  pub fn load_from_object(&mut self, type_: MachineType, object: &Node, offset: i32) -> &Node {
    todo!("v8: src/compiler/wasm-graph-assembler.cc WasmGraphAssembler::LoadFromObject")
  }

  /// 原版 BuildDecodeTrustedPointer — 可信指针解码 (sandbox)
  /// v8: src/compiler/wasm-graph-assembler.cc WasmGraphAssembler::BuildDecodeTrustedPointer
  pub fn build_decode_trusted_pointer(&mut self, object: &Node, offset: i32, tag: IndirectPointerTag) -> &Node {
    todo!("v8: src/compiler/wasm-graph-assembler.cc WasmGraphAssembler::BuildDecodeTrustedPointer")
  }
}
// cc 工厂全清单 (${ccFns.length}): ${ccFns.slice(0, 40).join(", ")}, ...
`;

const p = writeSrc("v8-compiler-passes", "wasm_graph_assembler.rs.skel", body);
console.log(`written ${p}\nbuiltin_calls=${builtinCalls.length} cc_factories=${ccFns.length} inline_factories=${hFns.length}`);
