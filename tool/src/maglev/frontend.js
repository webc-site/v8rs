#!/usr/bin/env bun
// tool/src/maglev/frontend.js — 建图前端: builder/reducer/内联/剥循环/编译单元/KNA
// 覆盖: src/maglev/maglev-graph-builder.h/.cc + maglev-reducer.h + maglev-reducer-inl.h +
//       src/maglev/maglev-inlining.h/.cc + maglev-loop-peeler.h/.cc +
//       src/maglev/maglev-compilation-info.h/.cc + maglev-compilation-unit.h/.cc +
//       src/maglev/maglev-interpreter-frame-state.h/.cc +
//       src/maglev/maglev-known-node-aspects.h/.cc + maglev-kna-processor.h/.cc +
//       src/maglev/maglev-map-inference.h
// 产出: v8-maglev-frontend/src/frontend.rs.skel
// 复跑幂等: 纯文本扫描 + 确定性输出。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/maglev/frontend.js";
const FILES = [
  "src/maglev/maglev-graph-builder.h", "src/maglev/maglev-graph-builder.cc",
  "src/maglev/maglev-reducer.h", "src/maglev/maglev-reducer-inl.h",
  "src/maglev/maglev-inlining.h", "src/maglev/maglev-inlining.cc",
  "src/maglev/maglev-loop-peeler.h", "src/maglev/maglev-loop-peeler.cc",
  "src/maglev/maglev-compilation-info.h", "src/maglev/maglev-compilation-info.cc",
  "src/maglev/maglev-compilation-unit.h", "src/maglev/maglev-compilation-unit.cc",
  "src/maglev/maglev-interpreter-frame-state.h", "src/maglev/maglev-interpreter-frame-state.cc",
  "src/maglev/maglev-known-node-aspects.h", "src/maglev/maglev-known-node-aspects.cc",
  "src/maglev/maglev-kna-processor.h", "src/maglev/maglev-kna-processor.cc",
  "src/maglev/maglev-map-inference.h",
];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[frontend.js] ${m}`); };

const gb = txt["src/maglev/maglev-graph-builder.h"];
for (const needle of [
  "class MaglevGraphBuilder {",
  "KnownNodeAspects& known_node_aspects() {",
  "void TakeKnownNodeAspectsAndVOsFromParent();",
  "void MoveKnownNodeAspectsAndVOsToParent();",
  "BytecodeOffset osr_offset);",
]) {
  if (!gb.includes(needle)) die(`builder 缺针 ${needle}`);
}
const rd = txt["src/maglev/maglev-reducer.h"];
for (const needle of [
  "concept ReducerBaseWithKNA = requires(BaseT* b) { b->known_node_aspects(); }",
  "concept ReducerBaseWithKNASetter",
  "class MaglevReducer {",
]) {
  if (!rd.includes(needle)) die(`reducer 缺针 ${needle}`);
}
const kna = txt["src/maglev/maglev-known-node-aspects.h"];
for (const needle of [
  "class KnownNodeAspects {",
  "V8_EXPORT_PRIVATE void MergeForLoop(const KnownNodeAspects& backedge,",
  "bool IsCompatibleWithLoopHeader(const KnownNodeAspects& other) const;",
]) {
  if (!kna.includes(needle)) die(`kna 缺针 ${needle}`);
}
const kp = txt["src/maglev/maglev-kna-processor.h"];
if (!kp.includes("class RecomputeKnownNodeAspectsProcessor {")) die("缺 RecomputeKnownNodeAspectsProcessor");
const mi = txt["src/maglev/maglev-map-inference.h"];
if (!mi.includes("class MapInference {")) die("缺 MapInference");
const ci = txt["src/maglev/maglev-compilation-info.h"];
if (!ci.includes("class MaglevCompilationInfo final {")) die("缺 MaglevCompilationInfo");
const cu = txt["src/maglev/maglev-compilation-unit.h"];
for (const needle of ["class MaglevCompilationUnit : public ZoneObject {",
  "static MaglevCompilationUnit* New(Zone* zone, MaglevCompilationInfo* info,",
  "bool is_inline() const { return inlining_depth_ != 0; }"]) {
  if (!cu.includes(needle)) die(`unit 缺针 ${needle}`);
}
const fs = txt["src/maglev/maglev-interpreter-frame-state.h"];
for (const needle of ["KnownNodeAspects* known_node_aspects() { return known_node_aspects_; }",
  "KnownNodeAspects* TakeKnownNodeAspects() {",
  "void ClearKnownNodeAspects() { known_node_aspects_ = nullptr; }"]) {
  if (!fs.includes(needle)) die(`frame-state 缺针 ${needle}`);
}
const lp = txt["src/maglev/maglev-loop-peeler.h"];
if (!lp.includes("class MaglevLoopPeeler {")) die("缺 MaglevLoopPeeler");

const out = [];
out.push(banner(GEN, FILES.join(", "), "建图前端: builder/reducer/内联/剥循环/单元/KNA"));
out.push(`//! 依赖: v8-maglev-ir (Opcode/NodeType/Graph), v8-interpreter (字节码), broker 只读面。`);
out.push(`//! 破环登记 (todo.md 五; 泛型单态化, 禁 dyn):`);
out.push(`//!   pub trait MaglevBrokerEnv { /* StaticTypeForMap/Constant 的 Map/broker 只读面 */ }`);
out.push(`//!   pub trait MaglevIsolateEnv { /* LocalIsolate/Zone 分配面 */ }`);
out.push(``);
out.push(`// ── MaglevGraphBuilder (字节码→图; 父子 KNA/VO 借用: Take/Move 成对) ──`);
out.push(`//   BuildGraph(info, osr_offset); known_node_aspects(); 内联时 BorrowParent* 守卫`);
out.push(``);
out.push(`// ── MaglevReducer (concept 约束族: WithKNA/WithKNASetter/WithEagerDeopt/WithLoopEffectTracking…) ──`);
out.push(`//   修订转 Rust trait bound; MaglevReducer 本体在 passes 侧被各 Processor 组合`);
out.push(``);
out.push(`// ── 内联 + 剥循环 (Phase kInlining/kLoopPeeling) ──`);
out.push(`//   MaglevCompilationUnit::New/NewInner/NewDummy (顶层/内联/哑单元); is_inline()=depth!=0`);
out.push(`//   MaglevLoopPeeler; MaglevLoopPeelInfo 登记在 Graph 侧`);
out.push(``);
out.push(`// ── KNA (KnownNodeAspects: map 推断 + 循环头 MergeForLoop/IsCompatibleWithLoopHeader) ──`);
out.push(`//   RecomputeKnownNodeAspectsProcessor (优化器/KNA 重算共用); MapInference (单点 map 折叠)`);
out.push(`//   InterpreterFrameState: Take/Clone/Clear/MergeNodeAspects (merge 块状态机)`);
out.push(``);
out.push(`// ── MaglevCompilationInfo (顶层编译参数; OSR 经 toplevel unit) ──`);
out.push(``);
out.push(`// 对数: builder 5 + reducer 3 + kna 4 + unit 3 + frame 3 + peeler 1 + info/map 2 全过`);

writeSrc("v8-maglev-frontend", "frontend.rs.skel", out.join("\n") + "\n");
console.log(`OK frontend.js → v8-maglev-frontend/src/frontend.rs.skel`);
