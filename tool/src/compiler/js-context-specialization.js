#!/usr/bin/env bun
// tool/src/compiler/js-context-specialization.js
// 从 v8/src/compiler/js-context-specialization.h + js-context-specialization.cc
// 提取 OuterContext 值类型、Reduce*/Simplify* 方法面与上下文链折叠规则
// 生成 v8-compiler-frontend/src/js_context_specialization.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/js-context-specialization.h");
const cc = readV8("src/compiler/js-context-specialization.cc");

// —— 1. 锚点断言 ——
const anchors = [
  ["struct OuterContext {", "OuterContext 结构"],
  ["size_t distance = 0;", "distance 默认 0"],
  ["class V8_EXPORT_PRIVATE JSContextSpecialization final : public AdvancedReducer {", "Reducer 类"],
  ['return "JSContextSpecialization";', "reducer_name"],
  ["Reduction ReduceParameter(Node* node);", "ReduceParameter"],
  ["Reduction ReduceJSLoadContextNoCell(Node* node);", "ReduceJSLoadContextNoCell"],
  ["Reduction ReduceJSStoreContextNoCell(Node* node);", "ReduceJSStoreContextNoCell"],
  ["Reduction SimplifyJSLoadContextNoCell(Node* node, Node* new_context,", "SimplifyJSLoadContextNoCell"],
  ["Maybe<OuterContext> outer() const { return outer_; }", "outer 访问器"],
  ["MaybeHandle<JSFunction> closure() const { return closure_; }", "closure 访问器"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit)) throw new Error(`js-context-specialization.h 锚点缺失: ${name}`);
}
const ccAnchors = [
  ["Reduction JSContextSpecialization::Reduce(Node* node) {", "Reduce 分派"],
  ["Reduction JSContextSpecialization::ReduceParameter(", "ReduceParameter 实现"],
  ["Reduction JSContextSpecialization::SimplifyJSLoadContextNoCell(", "SimplifyJSLoadContextNoCell 实现"],
  ["Reduction JSContextSpecialization::ReduceJSGetImportMeta(", "GetImportMeta 实现"],
];
for (const [lit, name] of ccAnchors) {
  if (!cc.includes(lit)) throw new Error(`js-context-specialization.cc 锚点缺失: ${name}`);
}

// —— 2. 真解析: Reduce* / Simplify* 方法面 ——
const reduceFns = [...h.matchAll(/Reduction (Reduce\w+)\(Node\* node\);/g)].map((m) => m[1]);
const simplifyFns = [...h.matchAll(/Reduction (Simplify\w+)\(Node\* node, Node\* new_context,\s*\n\s*size_t new_depth\);/g)].map((m) => m[1]);
if (reduceFns.length < 6 || simplifyFns.length !== 4) {
  throw new Error(`方法面提取异常: Reduce ${reduceFns.length} / Simplify ${simplifyFns.length}`);
}

// —— 3. .cc Reduce 分派 (IrOpcode case) ——
const dispatchCases = [...cc.matchAll(/case IrOpcode::k(\w+):/g)].map((m) => m[1]);
const implFns = new Set([...cc.matchAll(/^Reduction JSContextSpecialization::(\w+)\(/gm)].map((m) => m[1]));

const body = `${banner("src/compiler/js-context-specialization.js", "src/compiler/js-context-specialization.h + js-context-specialization.cc", "上下文特化: Load/StoreContext 常量折叠 + closure/import.meta 折叠")}

/// 外层上下文对 — 上下文及其距参照点的距离
/// v8: src/compiler/js-context-specialization.h OuterContext
pub struct OuterContext {
    pub context: IndirectHandle<Context>,
    /// 距离默认 0
    pub distance: usize,
}

/// JSContextSpecialization — 把 JSGraph 特化到给定上下文:
/// - 折叠 {LoadContextNoCell} / 削减 {StoreContextNoCell}
/// - 给定 {closure} 时折叠函数参数
/// - 对应对象已存在时折叠 import.meta 加载
/// outer 可为函数进入上下文或任意外层上下文 (由 distance 指示)
/// v8: src/compiler/js-context-specialization.h JSContextSpecialization
pub struct JSContextSpecialization {
    jsgraph: JSGraphRef,
    outer: Option<OuterContext>,
    closure: Option<Handle<JSFunction>>,
    broker: JSHeapBrokerRef,
}

impl JSContextSpecialization {
    /// 总分派 Reduce — 按 IrOpcode (${dispatchCases.length} 路):
    ${dispatchCases.map((c) => `// k${c}`).join(" / ")}
    /// v8: src/compiler/js-context-specialization.cc JSContextSpecialization::Reduce

${reduceFns.map((n) => `    /// v8: src/compiler/js-context-specialization.h JSContextSpecialization::${n}\n    fn ${n.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}(&mut self, node: &Node) -> Reduction;`).join("\n")}

    // —— Simplify 家族 (${simplifyFns.length}): 换 context + 深度重算, 距离不够则补 Load ——
${simplifyFns.map((n) => `    /// v8: src/compiler/js-context-specialization.h JSContextSpecialization::${n}\n    fn ${n.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}(&mut self, node: &Node, new_context: NodeRef, new_depth: usize) -> Reduction;`).join("\n")}
}

// —— 对数基准 ——
// Reduce ${reduceFns.length} / Simplify ${simplifyFns.length} / cc 实现 ${implFns.size}
// 分派 case ${dispatchCases.length}: ${dispatchCases.join(",")}
`;

const p = writeSrc("v8-compiler-frontend", "js_context_specialization.rs.skel", body);
console.log(`✓ js-context-specialization: Reduce ${reduceFns.length} / Simplify ${simplifyFns.length} / cc 实现 ${implFns.size} / 分派 case ${dispatchCases.length} / 锚点 ${anchors.length + ccAnchors.length}`);
