#!/usr/bin/env bun
// tool/src/compiler/simplified-lowering.js
// v8/src/compiler/simplified-lowering.h + simplified-lowering.cc
// → v8-compiler-passes/src/simplified_lowering.rs.skel
// 简化降低: 解析 Do* 公有面 / Float64*/Int32* 辅助族 / SetOnce 缓存族 /
// cc 内 RepresentationSelector 的访问器计数。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/simplified-lowering.h");
const cc = readV8("src/compiler/simplified-lowering.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`simplified-lowering: ${msg}`);
};

for (const [s, name] of [
  ["class V8_EXPORT_PRIVATE SimplifiedLowering final {", "类定义"],
  ["void DoMax(Node* node, Operator const* op, MachineRepresentation rep);", "DoMax"],
  ["void LowerAllNodes();", "LowerAllNodes"],
  ["void SimplifiedLowering::DoMax(", "cc DoMax"],
  ["void SimplifiedLowering::LowerAllNodes() {", "cc LowerAllNodes"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// —— Do* 公有面 (真解析 h; 括号内换行归一) ——
const hNorm = h.replace(/\(\s*\n\s*/g, "(");
const dos = [...hNorm.matchAll(/^  void (Do\w+)\(Node\* node[,)]/gm)].map((m) => m[1]);
for (const needM of ["DoMax", "DoMin", "DoJSToNumberOrNumericTruncatesToFloat64", "DoJSToNumberOrNumericTruncatesToWord32", "DoIntegral32ToBit", "DoOrderedNumberToBit", "DoNumberToBit", "DoIntegerToUint8Clamped", "DoNumberToUint8Clamped", "DoSigned32ToUint8Clamped", "DoUnsigned32ToUint8Clamped"]) {
  need(dos.includes(needM), `Do* 缺 ${needM} (实得 ${dos.length})`);
}

// —— 私有数值辅助族 (真解析 h) ——
const float64s = [...h.matchAll(/^  Node\* (Float64\w+)\(Node\* const node\);/gm)].map((m) => m[1]);
const int32s = [...h.matchAll(/^  Node\* (Int32\w+)\(Node\* const node\);/gm)].map((m) => m[1]);
const uint32s = [...h.matchAll(/^  Node\* (Uint32\w+)\(Node\* const node\);/gm)].map((m) => m[1]);
need(float64s.length === 2 && int32s.length === 4 && uint32s.length === 2,
  `数值辅助 Float64=${float64s.length} Int32=${int32s.length} Uint32=${uint32s.length}`);

// —— SetOnce 缓存族 ——
const setOnces = [...h.matchAll(/SetOncePointer<[\w <>]+>\s+(\w+);/g)].map((m) => m[1]);
need(setOnces.length === 6, `SetOnce 字段 ${setOnces.length}`);

// —— cc: RepresentationSelector 内部类 ——
need(cc.includes("class RepresentationSelector {"), "RepresentationSelector 缺席");
// 访问器计数: Get*PerNodeData / setter 面 (真解析)
const selStart = cc.indexOf("class RepresentationSelector {");
need(selStart > 0, "Selector 类定位失败");
// setter 面 (SetOutput/SetAndCheckInput 等模板族, 全文件 2 空格缩进)
const setters = [...cc.matchAll(/^\s{2}void (Set\w+)\(/gm)].map((m) => m[1]);
need(setters.length >= 2, `Selector setter ${setters.length}`);
// Visit 分派面 (VisitInputs/VisitReturn/VisitBinop/... 28 个 Visit*)
const visits = [...cc.matchAll(/^\s{2}void (Visit\w*)\(/gm)].map((m) => m[1]);
need(visits.length >= 25, `Selector visit ${visits.length}`);

const body = `${banner("src/compiler/simplified-lowering.js", "src/compiler/simplified-lowering.h + simplified-lowering.cc", "Simplified→Machine 表示选择与降低")}
//! simplified-lowering — 两层: SimplifiedLowering 门面 + RepresentationSelector
//!
//! 门面 Do* ${dos.length} 个 (Max/Min/ToBit/Uint8Clamped 族);
//! 数值辅助: Float64(${float64s.join("/")}) Int32(${int32s.join("/")}) Uint32(${uint32s.join("/")});
//! SetOnce 缓存 ${setOnces.length} 个; Selector: setter 面 ${setters.length} / visit 面 ${visits.length}。
/// 原版 SimplifiedLowering — 门面 (jsgraph + 源位置 + origin + 观察)
pub struct SimplifiedLowering<'z, 'g> {
  /// jsgraph_
  pub jsgraph: &'g JSGraph<'z>,
  /// broker_
  pub broker: *mut JSHeapBroker,
  /// zone_
  pub zone: &'z Zone<'z>,
  /// type_cache_
  pub type_cache: *const TypeCache,
  /// to_number_code_ 等 SetOnce 缓存 (${setOnces.join("/")})  pub caches: SimplifiedLoweringCaches,
  /// source_positions_
  pub source_positions: *mut SourcePositionTable,
  /// node_origins_
  pub node_origins: *mut NodeOriginTable,
  /// tick_counter_
  pub tick_counter: *mut TickCounter,
  /// linkage_
  pub linkage: *mut Linkage,
  /// info_
  pub info: *mut OptimizedCompilationInfo,
  /// observe_node_manager_
  pub observe_node_manager: *mut ObserveNodeManager,
}

/// SetOnce 缓存包 (${setOnces.length} 字段)
pub struct SimplifiedLoweringCaches {
${setOnces.map((f) => `  /// ${f}\n  pub ${f}: SetOncePtr<()>,`).join("\n")}
}

impl SimplifiedLowering<'_, '_> {
  /// 原版 LowerAllNodes — 建 RepresentationSelector 跑图
  /// v8: src/compiler/simplified-lowering.cc SimplifiedLowering::LowerAllNodes
  pub fn lower_all_nodes(&mut self) {
    todo!("v8: src/compiler/simplified-lowering.cc SimplifiedLowering::LowerAllNodes")
  }

${dos.map((d) => `  /// 原版 ${d}\n  /// v8: src/compiler/simplified-lowering.cc SimplifiedLowering::${d}\n  pub fn ${d.replace(/^Do/, "do_").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}(&mut self, node: &Node /*, op/rep/selector 按签名 */) {\n    todo!("v8: src/compiler/simplified-lowering.cc SimplifiedLowering::${d}")\n  }`).join("\n\n")}

  /// 原版 Float64Round/Float64Sign — ${float64s.join(", ")}
  /// v8: src/compiler/simplified-lowering.cc SimplifiedLowering::Float64Round
  fn float64_round(&mut self, node: &Node) -> *mut Node {
    todo!("v8: src/compiler/simplified-lowering.cc SimplifiedLowering::Float64Round (同族 ${float64s.join("/")})")
  }

  /// 原版 Int32Abs/Int32Div/Int32Mod/Int32Sign — 降低辅助
  /// v8: src/compiler/simplified-lowering.cc SimplifiedLowering::Int32Abs
  fn int32_abs(&mut self, node: &Node) -> *mut Node {
    todo!("v8: src/compiler/simplified-lowering.cc SimplifiedLowering::Int32Abs (同族 ${int32s.join("/")})")
  }

  /// 原版 Uint32Div/Uint32Mod
  /// v8: src/compiler/simplified-lowering.cc SimplifiedLowering::Uint32Div
  fn uint32_div(&mut self, node: &Node) -> *mut Node {
    todo!("v8: src/compiler/simplified-lowering.cc SimplifiedLowering::Uint32Div (同族 ${uint32s.join("/")})")
  }

  /// 原版 ChangeOp — NodeProperties::ChangeOp 的观察通知包装
  /// v8: src/compiler/simplified-lowering.cc SimplifiedLowering::ChangeOp
  fn change_op(&mut self, node: &Node, new_op: &Operator) {
    todo!("v8: src/compiler/simplified-lowering.cc SimplifiedLowering::ChangeOp")
  }
}

/// 原版 RepresentationSelector — 表示选择器 (simplified-lowering.cc 内部类)
/// setter 面 (${setters.length}): ${setters.join(", ")}
/// visit 面 (${visits.length}): ${visits.slice(0, 20).join(", ")}...
pub struct RepresentationSelector<'z, 'g> {
  /// 关联的 lowering
  pub lowering: *mut SimplifiedLowering<'z, 'g>,
}

impl RepresentationSelector<'_, '_> {
  /// 原版 Run — 两遍: 遍历定表示 → 遍历改节点
  /// v8: src/compiler/simplified-lowering.cc RepresentationSelector::Run
  pub fn run(&mut self, phase_name: &str) {
    todo!("v8: src/compiler/simplified-lowering.cc RepresentationSelector::Run (propagation + retry / lowering)")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "simplified_lowering.rs.skel", body);
console.log(`written ${p}\ndo=${dos.length} float64=${float64s.length} int32=${int32s.length} uint32=${uint32s.length} set_once=${setOnces.length} setters=${setters.length} visits=${visits.length}`);
