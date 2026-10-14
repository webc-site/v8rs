#!/usr/bin/env bun
// tool/src/compiler/js-graph.js
// 从 v8/src/compiler/js-graph.h + js-graph.cc
// 提取 CACHED_GLOBAL_LIST/CACHED_CENTRY_LIST 缓存表、CEntryStub 常量协议与 HoleType→常量分派
// 生成 v8-compiler-frontend/src/js_graph.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/js-graph.h");
const cc = readV8("src/compiler/js-graph.cc");

// —— 1. 锚点断言 ——
const anchors = [
  ["class V8_EXPORT_PRIVATE JSGraph : public MachineGraph {", "JSGraph 类"],
  ["#define CACHED_GLOBAL_LIST(V)", "全局缓存宏表"],
  ["#define CACHED_CENTRY_LIST(V)", "CEntry 缓存宏表"],
  ["TNode<Hole> PaddingConstant() { return TheHoleConstant(); }", "Padding=TheHole 别名"],
  ["TNode<Number> NoContextConstant() { return ZeroConstant(); }", "NoContext=Zero 别名"],
  ["Node* HeapConstantNoHole(Handle<HeapObject> value);", "HeapConstantNoHole"],
  ["Node* HeapConstantMaybeHole(Handle<HeapObject> value);", "HeapConstantMaybeHole"],
  ["Node* HeapConstantHole(Handle<HeapObject> value);", "HeapConstantHole"],
  ["DCHECK(result_size >= 1 && result_size <= 3);", "CEntry result_size 1..3"],
  ["ptr = builtin_exit_frame ? &CEntryStub1WithBuiltinExitFrameConstant_", "builtin_exit_frame 缓存选择"],
  ["Node* SmiConstant(int32_t immediate) {", "SmiConstant"],
  ["CHECK(ref.IsSmi() || ref.IsHeapNumber() || ref.HoleType() == HoleType::kNone);", "NoHole 安全检查 (crbug 1486789)"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit) && !cc.includes(lit)) throw new Error(`js-graph 锚点缺失: ${name}`);
}

// —— 2. 真解析: 两个缓存表 ——
const parseList = (src, macro, tail) => {
  const i = src.indexOf(`#define ${macro}(V)`);
  if (i < 0) throw new Error(`未找到 ${macro}`);
  const seg = src.slice(i, src.indexOf(tail, i));
  // 两型: V(name, Type) 与 CEntry 专用的单参 V(name)
  const v = [...seg.matchAll(/^  V\((\w+)(?:,\s*(\w+))?\)/gm)].map((m) => ({ name: m[1], ty: m[2] }));
  const w = [...seg.matchAll(/^  IF_WASM\(V,\s*(\w+),\s*(\w+)\)/gm)].map((m) => ({ name: m[1], ty: m[2] }));
  return { v, w };
};
const globals = parseList(h, "CACHED_GLOBAL_LIST", "// Cached global node accessor methods.");
const centries = parseList(h, "CACHED_CENTRY_LIST", "// Canonicalized global node fields.");
if (globals.v.length < 30 || globals.w.length !== 2 || centries.v.length !== 4) {
  throw new Error(`缓存表提取异常: global ${globals.v.length}+${globals.w.length} / centry ${centries.v.length}`);
}

// —— 3. 真解析: HoleType → 常量分派 (js-graph.cc Constant) ——
const constSrc = cc.slice(cc.indexOf("Node* JSGraph::Constant(ObjectRef ref"), cc.indexOf("  OddballType oddball_type"));
const holeCases = [...constSrc.matchAll(/case HoleType::k(\w+):/g)].map((m) => m[1]);
const holeReturns = [...constSrc.matchAll(/case HoleType::k(\w+):\s*\n\s*return (\w+)\(\);/g)].map((m) => [m[1], m[2]]);
if (holeCases.length < 12 || holeReturns.length < 8) throw new Error(`HoleType 分派提取异常: ${holeCases.length}/${holeReturns.length}`);

// —— 4. .cc 定义面计数 (显式定义 + GET_CACHED_FIELD 宏展开访问器) ——
const ccFns = [...cc.matchAll(/^(?:Node\*|void) JSGraph::\w+\(/gm)];
const cachedFieldExpanders = (cc.match(/JSGraph::name\(\)/g) || []).length;
if (ccFns.length < 15) throw new Error(`js-graph.cc 定义面不足: ${ccFns.length}`);

const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

const body = `${banner("src/compiler/js-graph.js", "src/compiler/js-graph.h + js-graph.cc", "JSGraph 门面: 规范化全局常量缓存 + CEntryStub 协议 + Constant 分派")}
//! JSGraph — TFGraph 的 JS 门面: 算子构建器 + 规范化全局常量 + helper。

/// JS 图门面
/// v8: src/compiler/js-graph.h JSGraph
pub struct JSGraph { /* MachineGraph + isolate/javascript/simplified 三构建器 */ }

impl JSGraph {
    /// CEntryStub 常量 — 按 (result_size ∈ [1,3], argv_mode, builtin_exit_frame) 缓存
    /// kStack 模式 + 非 builtin_exit_frame: 1/2/3 三档缓存
    /// builtin_exit_frame: CEntryStub1WithBuiltinExitFrame 单独缓存
    /// 非 kStack (argv on heap): 不缓存, 每次新建
    /// v8: src/compiler/js-graph.cc JSGraph::CEntryStubConstant
    pub fn centry_stub_constant(&mut self, result_size: i32, argv_mode: ArgvMode, builtin_exit_frame: bool) -> NodeRef;

    /// 别名两枚 (原版内联):
    /// v8: src/compiler/js-graph.h JSGraph::PaddingConstant (= TheHoleConstant)
    /// v8: src/compiler/js-graph.h JSGraph::NoContextConstant (= ZeroConstant)

    // HeapConstant 三档纪律:
    /// 默认用; 禁止发射 hole
    /// v8: src/compiler/js-graph.h JSGraph::HeapConstantNoHole
    pub fn heap_constant_no_hole(&mut self, value: Handle<HeapObject>) -> NodeRef;
    /// 可能发 hole 时用
    /// v8: src/compiler/js-graph.h JSGraph::HeapConstantMaybeHole
    pub fn heap_constant_maybe_hole(&mut self, value: Handle<HeapObject>) -> NodeRef;
    /// 只发 hole 时用
    /// v8: src/compiler/js-graph.h JSGraph::HeapConstantHole
    pub fn heap_constant_hole(&mut self, value: Handle<HeapObject>) -> NodeRef;

    // Constant 两档纪律:
    /// CHECK 非 hole (安全关键, crbug.com/1486789); 字符串先 UnpackIfThin
    /// v8: src/compiler/js-graph.cc JSGraph::ConstantNoHole
    pub fn constant_no_hole(&mut self, r: ObjectRef, broker: &JSHeapBroker) -> NodeRef;
    /// v8: src/compiler/js-graph.cc JSGraph::ConstantMaybeHole
    pub fn constant_maybe_hole(&mut self, r: ObjectRef, broker: &JSHeapBroker) -> NodeRef;
    /// Smi → ConstantMaybeHole(immediate)
    /// v8: src/compiler/js-graph.h JSGraph::SmiConstant
    pub fn smi_constant(&mut self, immediate: i32) -> NodeRef;
    /// 可变 HeapNumber 常量 (不同于读 double 值)
    /// v8: src/compiler/js-graph.h JSGraph::ConstantMutableHeapNumber
    pub fn constant_mutable_heap_number(&mut self, r: HeapNumberRef, broker: &JSHeapBroker) -> NodeRef;
    /// v8: src/compiler/js-graph.h JSGraph::BooleanConstant
    pub fn boolean_constant(&mut self, is_true: bool) -> NodeRef;
}

// —— HoleType → 规范化常量分派 (js-graph.cc JSGraph::Constant 内嵌 switch) ——
// 命中 ${holeReturns.length} 路, 其余 UNREACHABLE:
${holeReturns.map(([k, v]) => `// k${k} => ${snake(v.replace(/Constant$/, ""))}_constant() /// v8: src/compiler/js-graph.cc JSGraph::Constant case HoleType::k${k}`).join("\n")}
// UNREACHABLE: ${holeCases.filter((k) => !holeReturns.some(([a]) => a === k)).join(" / ")}
// 之后按 OddballType (Undefined/Null/...) 走 roots 常量

// —— CACHED_GLOBAL_LIST: ${globals.v.length} 项 + IF_WASM ${globals.w.length} 项 ——
${globals.v.map((g) => `// ${g.name}: ${g.ty} /// v8: src/compiler/js-graph.h CACHED_GLOBAL_LIST`).join("\n")}
// wasm-only:
${globals.w.map((g) => `// ${g.name}: ${g.ty}`).join("\n")}

// —— CACHED_CENTRY_LIST: 4 项 ——
${centries.v.map((c) => `// ${c.name} /// v8: src/compiler/js-graph.h CACHED_CENTRY_LIST`).join("\n")}

// GetCachedNodes: 把全部已缓存节点加进 list
/// v8: src/compiler/js-graph.h JSGraph::GetCachedNodes

// —— 对数基准 ——
// CACHED_GLOBAL ${globals.v.length}+${globals.w.length}(wasm) / CENTRY ${centries.v.length}
// HoleType case ${holeCases.length} (映射 ${holeReturns.length})
// cc 显式定义 ${ccFns.length} + GET_CACHED_FIELD 宏展开访问器模板 ${cachedFieldExpanders} 处
`;

const p = writeSrc("v8-compiler-frontend", "js_graph.rs.skel", body);
console.log(`✓ js-graph: GLOBAL ${globals.v.length}+${globals.w.length}(wasm) / CENTRY ${centries.v.length} / HoleType ${holeCases.length}(${holeReturns.length} 映射) / cc 定义 ${ccFns.length}+宏${cachedFieldExpanders} / 锚点 ${anchors.length}`);
