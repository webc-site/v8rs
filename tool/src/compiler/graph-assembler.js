#!/usr/bin/env bun
// tool/src/compiler/graph-assembler.js
// 从 v8/src/compiler/graph-assembler.h + graph-assembler.cc
// 提取 JSGRAPH_SINGLETON_CONSTANT_LIST 单例表、Label 类型枚举与 GraphAssembler 签名面
// 生成 v8-compiler-frontend/src/graph_assembler.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/graph-assembler.h");
const cc = readV8("src/compiler/graph-assembler.cc");

// —— 1. 锚点断言 ——
const anchors = [
  ["#define JSGRAPH_SINGLETON_CONSTANT_LIST(V)", "单例常量宏表"],
  ["enum class GraphAssemblerLabelType { kDeferred, kNonDeferred, kLoop };", "Label 类型枚举"],
  ["constexpr size_t kGraphAssemblerLabelDynamicCount = ~0u;", "动态计数哨兵 ~0"],
  ["class GraphAssemblerLabel {", "Label 类"],
  ["class V8_EXPORT_PRIVATE GraphAssembler {", "GraphAssembler 类"],
  ["class V8_EXPORT_PRIVATE JSGraphAssembler : public GraphAssembler {", "JSGraphAssembler 类"],
  ["detail::GraphAssemblerLabelForReps<Reps...> MakeLabel(Reps... reps) {", "MakeLabel 工厂"],
  ["GraphAssemblerLabelType::kLoop", "MakeLoopLabel kLoop"],
  ["GraphAssemblerLabelType::kDeferred", "MakeDeferredLabel kDeferred"],
  ["void Bind(GraphAssemblerLabel<VarCount>* label);", "Bind"],
  ["void Goto(detail::GraphAssemblerLabelForVars<Vars...>* label, Vars...);", "Goto"],
  ["class V8_NODISCARD LoopScope final {", "LoopScope"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit)) throw new Error(`graph-assembler.h 锚点缺失: ${name}`);
}

// —— 2. 真解析: JSGRAPH_SINGLETON_CONSTANT_LIST ——
const listSrc = h.slice(
  h.indexOf("#define JSGRAPH_SINGLETON_CONSTANT_LIST(V)"),
  h.indexOf("class GraphAssembler;"),
);
const singletons = [...listSrc.matchAll(/^  V\((\w+),\s*(\w+)\)/gm)].map((m) => ({ name: m[1], ty: m[2] }));
const wasmOnly = [...listSrc.matchAll(/^  IF_WASM\(V,\s*(\w+),\s*(\w+)\)/gm)].map((m) => ({ name: m[1], ty: m[2] }));
if (singletons.length !== 23 || wasmOnly.length !== 2) {
  throw new Error(`单例表提取异常: V ${singletons.length} / IF_WASM ${wasmOnly.length}`);
}

// —— 3. 真解析: GraphAssemblerLabelType 枚举值 ——
const enumMatch = h.match(/enum class GraphAssemblerLabelType \{([^}]+)\};/);
const labelTypes = enumMatch[1].split(",").map((s) => s.trim());
if (labelTypes.length !== 3) throw new Error(`LabelType 应 3 值: ${labelTypes}`);

// —— 4. .cc 定义面计数 ——
const ccFns = [...cc.matchAll(/^[A-Za-z].*(?:GraphAssembler|JSGraphAssembler|GraphAssemblerLabel)\w*::\w+\(/gm)];
if (ccFns.length < 20) throw new Error(`graph-assembler.cc 定义面不足: ${ccFns.length}`);

const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
const cacheTy = { InstructionStream: "InstructionStream", Map: "Map", String: "String", Boolean: "Boolean", Number: "Number", Object: "Object", Null: "Null", Undefined: "Undefined", Hole: "Hole" };

const body = `${banner("src/compiler/graph-assembler.js", "src/compiler/graph-assembler.h + graph-assembler.cc", "图组装器: Label 状态机 + 单例常量缓存 + JSGraphAssembler 扩展")}

/// Label 类型
/// v8: src/compiler/graph-assembler.h GraphAssemblerLabelType
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphAssemblerLabelType {
    /// 延迟块 (冷路径)
    Deferred,
    NonDeferred,
    /// 循环头 (需要 Phi 回边合并)
    Loop,
}

/// 动态 (运行期才知道) 变量数哨兵
/// v8: src/compiler/graph-assembler.h detail::kGraphAssemblerLabelDynamicCount
pub const K_GRAPH_ASSEMBLER_LABEL_DYNAMIC_COUNT: usize = usize::MAX; // ~0u

/// Label — 组装器基本块句柄 (合并 Phi + effect/control)
/// v8: src/compiler/graph-assembler.h GraphAssemblerLabel
pub struct GraphAssemblerLabel<const VAR_COUNT: usize = K_GRAPH_ASSEMBLER_LABEL_DYNAMIC_COUNT> {
    ty: GraphAssemblerLabelType,
    /// 循环嵌套层级
    loop_nesting_level: i32,
    merged_count: i32,
    effect: Option<NodeRef>,
    control: Option<NodeRef>,
    merged_phis: [Option<NodeRef>; VAR_COUNT],
}

impl GraphAssemblerLabel {
    /// v8: src/compiler/graph-assembler.h GraphAssemblerLabel::IsDeferred
    pub fn is_deferred(&self) -> bool { self.ty == GraphAssemblerLabelType::Deferred }
    /// v8: src/compiler/graph-assembler.h GraphAssemblerLabel::IsLoop
    pub fn is_loop(&self) -> bool { self.ty == GraphAssemblerLabelType::Loop }
}

/// GraphAssembler — 在 TFGraph 上以效果/控制传递方式组装子图
/// v8: src/compiler/graph-assembler.h GraphAssembler
pub struct GraphAssembler {
    pub effect: NodeRef,
    pub control: NodeRef,
}

impl GraphAssembler {
    /// 三工厂:
    /// v8: src/compiler/graph-assembler.h GraphAssembler::MakeLabel (kNonDeferred)
    /// v8: src/compiler/graph-assembler.h GraphAssembler::MakeLoopLabel (kLoop)
    /// v8: src/compiler/graph-assembler.h GraphAssembler::MakeDeferredLabel (kDeferred)
    pub fn make_label(&mut self, reps: MachineRepresentation) -> GraphAssemblerLabel;
    pub fn make_loop_label(&mut self, reps: MachineRepresentation) -> GraphAssemblerLabel;
    pub fn make_deferred_label(&mut self, reps: MachineRepresentation) -> GraphAssemblerLabel;

    /// v8: src/compiler/graph-assembler.h GraphAssembler::Bind
    pub fn bind(&mut self, label: &mut GraphAssemblerLabel);
    /// v8: src/compiler/graph-assembler.h GraphAssembler::Goto
    pub fn goto_(&mut self, label: &mut GraphAssemblerLabel, vars: &[NodeRef]);
    /// v8: src/compiler/graph-assembler.h GraphAssembler::MergeState
    pub fn merge_state(&mut self, label: &mut GraphAssemblerLabel, vars: &[NodeRef]);

    /// LoopScope — RAII 循环域
    /// v8: src/compiler/graph-assembler.h GraphAssembler::LoopScope
    pub fn loop_scope(&mut self) -> LoopScope<'_>;
}

/// JS 侧扩展 — 挂 JSGraph 的 singleton 常量缓存
/// v8: src/compiler/graph-assembler.h JSGraphAssembler
pub struct JSGraphAssembler { inner: GraphAssembler }

// —— JSGRAPH_SINGLETON_CONSTANT_LIST (${singletons.length} 项 + IF_WASM ${wasmOnly.length} 项) ——
// (名, 类型) 1:1:
${singletons.map((s) => `// ${s.name}: ${cacheTy[s.ty] ?? s.ty} /// v8: src/compiler/graph-assembler.h JSGRAPH_SINGLETON_CONSTANT_LIST`).join("\n")}
// wasm-only (V8_ENABLE_WEBASSEMBLY):
${wasmOnly.map((s) => `// ${s.name}: ${cacheTy[s.ty] ?? s.ty}`).join("\n")}

impl JSGraphAssembler {
${singletons.slice(0, 6).map((s) => `    /// v8: src/compiler/graph-assembler.h JSGRAPH_SINGLETON_CONSTANT_LIST ${s.name}\n    pub fn ${snake(s.name)}(&mut self) -> NodeRef;`).join("\n")}
    // ... 其余 ${singletons.length - 6} 项同型
}

// —— 对数基准 ——
// 单例表 ${singletons.length}+${wasmOnly.length} (wasm) / LabelType ${labelTypes.length} 值 / cc 定义面 ${ccFns.length}
`;

const p = writeSrc("v8-compiler-frontend", "graph_assembler.rs.skel", body);
console.log(`✓ graph-assembler: 单例 ${singletons.length}+${wasmOnly.length}(wasm) / LabelType ${labelTypes.length} / cc 定义 ${ccFns.length} / 锚点 ${anchors.length}`);
