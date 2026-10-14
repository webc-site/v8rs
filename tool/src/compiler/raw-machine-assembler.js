#!/usr/bin/env bun
// tool/src/compiler/raw-machine-assembler.js
// v8/src/compiler/raw-machine-assembler.h + raw-machine-assembler.cc
// → v8-compiler-passes/src/raw_machine_assembler.rs.skel
// 裸机汇编器: 解析节点工厂方法清单 / Export 面 / RawMachineLabel。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/raw-machine-assembler.h");
const cc = readV8("src/compiler/raw-machine-assembler.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`raw-machine-assembler: ${msg}`);
};

for (const [s, name] of [
  ["void Goto(RawMachineLabel* label);", "Goto(RawMachineLabel)"],
  ["class V8_EXPORT_PRIVATE RawMachineAssembler {", "类定义"],
  ["Schedule* ExportForTest();", "ExportForTest"],
  ["TFGraph* ExportForOptimization();", "ExportForOptimization"],
  ["RawMachineAssembler::RawMachineAssembler(", "cc ctor"],
  ["Schedule* RawMachineAssembler::ExportForTest() {", "cc ExportForTest"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// —— 节点工厂方法面 (真解析: Node* Xxx(...) 声明) ——
const factories = [...h.matchAll(/^  Node\* (\w+)\(/gm)].map((m) => m[1]);
need(factories.length >= 90, `工厂方法 ${factories.length}`);
for (const f of ["NullConstant", "IntPtrConstant", "Int32Constant", "Int64Constant", "Float64Constant", "HeapConstant", "Load", "Store", "CallN", "CallNWithFrameState", "CallCFunction", "Phi", "Parameter", "Word32Shl", "Int32Add", "Float64Add"]) {
  need(factories.includes(f), `工厂 ${f} 缺席`);
}
// Machine 算子转发面 (M()-> 形态)
const forwards = [...h.matchAll(/^  Node\* M\(\)->(\w+)\(/gm)].length;

// 控制流面 (void 返回: Branch/Goto/Switch/Deoptimize)
const controlVoid = [...new Set([...h.matchAll(/^  void (Branch|Goto|Switch|Deoptimize|Comment|Bind|Return|TailCallN)\((Node\* \w+|CallDescriptor\*|RawMachineLabel\*)/gm)].map((m) => m[1]))];
need(controlVoid.includes("Branch") && controlVoid.includes("Goto") && controlVoid.includes("Return"), `控制流面 ${controlVoid.join(",")}`);

// —— Label 体系 ——
need(h.includes("class RawMachineLabel;") || h.includes("void Goto(RawMachineLabel* label);"), "Label 面");

const body = `${banner("src/compiler/raw-machine-assembler.js", "src/compiler/raw-machine-assembler.h + raw-machine-assembler.cc", "边建图边入调度的机器级汇编器")}
//! raw-machine-assembler — 裸机图工厂 (CSA/wasm 编译底座)
//!
//! 节点立即入当前块 (无显式 effect/control 接线契约);
//! 工厂方法 ${factories.length} 个 (常量/存取/算术/控制/调用族);
//! Export*: ForTest 出调度, ForOptimization 出可再调度的图。

/// 原版 RawMachineAssembler — 汇编器主体
pub struct RawMachineAssembler<'z, 'g> {
  /// isolate_
  pub isolate: *mut Isolate,
  /// graph_
  pub graph: &'g mut TFGraph<'z>,
  /// schedule_ (即时入调度)
  pub schedule: Schedule<'z>,
  /// machine_
  pub machine: MachineOperatorBuilder<'z>,
  /// common_
  pub common: CommonOperatorBuilder<'z>,
  /// simplified_
  pub simplified: SimplifiedOperatorBuilder<'z>,
  /// call_descriptor_
  pub call_descriptor: *const CallDescriptor,
  /// parameters_ (Parameter 节点缓存)
  pub parameters: ZoneVector<*mut Node>,
  /// current_block_ (basic_block_)
  pub current_block: *mut BasicBlock<'z>,
  /// current_virtual_consts_
  pub current_virtual_consts: ZoneMap<usize, *mut Node>,
}

impl RawMachineAssembler<'_, '_> {
  /// 原版 ExportForTest — 封版导出调度 (此后实例失效)
  /// v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::ExportForTest
  pub fn export_for_test(&mut self) -> &Schedule<'_> {
    todo!("v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::ExportForTest")
  }

  /// 原版 ExportForOptimization — 导出可 TF 优化重调度的图
  /// v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::ExportForOptimization
  pub fn export_for_optimization(&mut self) -> &TFGraph<'_> {
    todo!("v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::ExportForOptimization")
  }

  /// 原版 Bind — 切当前块 (Label 目标)
  /// v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::Bind
  pub fn bind(&mut self, label: &RawMachineLabel<'_>) {
    todo!("v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::Bind")
  }

  /// 原版 Goto — 无条件跳 (含悬空 Phi 补全)
  /// v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::Goto
  pub fn goto_(&mut self, label: &RawMachineLabel<'_>) {
    todo!("v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::Goto")
  }

  /// 原版 Branch — 条件跳
  /// v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::Branch
  pub fn branch(&mut self, cond: &Node, true_dst: &RawMachineLabel<'_>, false_dst: &RawMachineLabel<'_>) {
    todo!("v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::Branch")
  }

  /// 原版 Parameter — 参数节点 (按索引缓存)
  /// v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::Parameter
  pub fn parameter(&mut self, index: usize) -> &Node {
    todo!("v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::Parameter")
  }

  // —— 节点工厂面 (${factories.length} 个, 样例锚) ——
  /// v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::Int32Constant
  pub fn int32_constant(&mut self, value: i32) -> &Node {
    todo!("v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::Int32Constant (工厂面 ${factories.length} 个)")
  }

  /// v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::Load
  pub fn load(&mut self, rep: MachineType, base: &Node, index: &Node) -> &Node {
    todo!("v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::Load")
  }

  /// v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::Store
  pub fn store(&mut self, write_barrier_kind: WriteBarrierKind, rep: MachineType, base: &Node, index: &Node, value: &Node) -> &Node {
    todo!("v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::Store")
  }

  /// v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::Call
  pub fn call(&mut self, descriptor: &CallDescriptor, target: &Node, args: &[&Node]) -> &Node {
    todo!("v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::Call")
  }

  /// v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::Phi
  pub fn phi(&mut self, rep: MachineRepresentation, args: &[&Node]) -> &Node {
    todo!("v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::Phi")
  }

  /// v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::Return
  pub fn return_(&mut self, count: usize, inputs: &[&Node]) -> &Node {
    todo!("v8: src/compiler/raw-machine-assembler.cc RawMachineAssembler::Return")
  }
}

/// 原版 RawMachineLabel (RawMachineAssembler::Label) — 块标号
/// v8: src/compiler/raw-machine-assembler.h RawMachineAssembler::Label
pub struct RawMachineLabel<'z> {
  /// block_
  pub block: Option<*mut BasicBlock<'z>>,
  /// used_
  pub used: bool,
  /// merge_index_ / phi_merge_
  pub merge_index: ZoneVector<*mut Node>,
}
`;

const p = writeSrc("v8-compiler-passes", "raw_machine_assembler.rs.skel", body);
console.log(`written ${p}\nfactories=${factories.length} m_forwards=${forwards}`);
