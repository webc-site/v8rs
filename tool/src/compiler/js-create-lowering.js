#!/usr/bin/env bun
// tool/src/compiler/js-create-lowering.js
// 从 v8/src/compiler/js-create-lowering.h + js-create-lowering.cc
// 提取 JSCreate* Reduce 方法面、TryAllocate* 家族与内联分配参数
// 生成 v8-compiler-frontend/src/js_create_lowering.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/js-create-lowering.h");
const cc = readV8("src/compiler/js-create-lowering.cc");

// —— 1. 锚点断言 ——
const anchors = [
  ["class V8_EXPORT_PRIVATE JSCreateLowering final", "JSCreateLowering 类"],
  ['return "JSCreateLowering";', "reducer_name"],
  ["Reduction ReduceJSCreate(Node* node);", "ReduceJSCreate"],
  ["Reduction ReduceJSCreateArguments(Node* node);", "ReduceJSCreateArguments"],
  ["Node* TryAllocateArguments(Node* effect, Node* control,", "TryAllocateArguments"],
  ["Node* TryAllocateRestArguments(", "TryAllocateRestArguments"],
  ["Node* TryAllocateAliasedArguments(", "TryAllocateAliasedArguments"],
  ["std::optional<Node*> TryAllocateFastLiteral(", "TryAllocateFastLiteral"],
  ["std::optional<Node*> TryAllocateFastLiteralElements(", "TryAllocateFastLiteralElements"],
  ["Node* AllocateElements(", "AllocateElements"],
  ["// The following functions all return nullptr iff there are too many arguments", "TryAllocate 返回 null 语义"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit)) throw new Error(`js-create-lowering.h 锚点缺失: ${name}`);
}
const ccAnchors = [
  ["Reduction JSCreateLowering::Reduce(Node* node) {", "Reduce 分派"],
  ["Reduction JSCreateLowering::ReduceJSCreateArguments(", "ReduceJSCreateArguments 实现"],
  ["JSCreateLowering::TryAllocateFastLiteral(", "TryAllocateFastLiteral 实现"],
  ["kMaxFastLiteralDepth", "快字面量深度上限"],
  ["kMaxFastLiteralProperties", "快字面量属性上限"],
];
for (const [lit, name] of ccAnchors) {
  if (!cc.includes(lit)) throw new Error(`js-create-lowering.cc 锚点缺失: ${name}`);
}

// —— 2. 真解析: JSCreate* Reduce 面 ——
const reduceFns = [...h.matchAll(/Reduction (ReduceJSCreate\w*|ReduceJSGetTemplateObject|ReduceNewArray|ReduceJSCreateObject|ReduceJSCreateStringWrapper)\(/g)]
  .map((m) => m[1]);
const uniqueReduce = [...new Set(reduceFns)];

// —— 3. TryAllocate* 家族 (返回 null = 参数过多放弃内联) ——
const tryFns = [...h.matchAll(/(Node\*|std::optional<Node\*>) (TryAllocate\w+)\(/g)].map((m) => m[2]);
const uniqueTry = [...new Set(tryFns)];

// —— 4. Allocate* 家族 ——
const allocFns = [...h.matchAll(/Node\* (Allocate\w+)\(/g)].map((m) => m[1]);
const uniqueAlloc = [...new Set(allocFns)];

// —— 5. cc 快字面量上限常量使用处 (定义在 compiler/globals.h, 非本族) ——
const uses = (cc.match(/kMaxFastLiteralDepth|kMaxFastLiteralProperties/g) || []).length;
if (uses < 2) throw new Error(`快字面量上限使用处不足: ${uses}`);

const snake = (s) => s.replace(/^Reduce/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

const body = `${banner("src/compiler/js-create-lowering.js", "src/compiler/js-create-lowering.h + js-create-lowering.cc", "JSCreate 降级: 内联分配替换对象创建算子")}

/// JSCreateLowering — 把 JSCreate 级算子降为快速 (内联) 分配
/// v8: src/compiler/js-create-lowering.h JSCreateLowering
pub struct JSCreateLowering { jsgraph: JSGraphRef, broker: JSHeapBrokerRef, zone: ZoneRef }

impl JSCreateLowering {
    /// 总分派 Reduce (按 IrOpcode)
    /// v8: src/compiler/js-create-lowering.cc JSCreateLowering::Reduce
    fn reduce(&mut self, node: &Node) -> Reduction;

    // —— JSCreate* 削减面 (${uniqueReduce.length} 个) ——
${uniqueReduce.map((n) => `    /// v8: src/compiler/js-create-lowering.h JSCreateLowering::${n}\n    fn ${n === "ReduceNewArray" ? "reduce_new_array" : snake(n)}(&mut self, node: &Node /*, 重载差异参数见原版 */) -> Reduction;`).join("\n")}

    // —— TryAllocate 家族 (${uniqueTry.length} 个): 返回 None = 参数过多放弃内联 ——
${uniqueTry.map((n) => `    /// v8: src/compiler/js-create-lowering.h JSCreateLowering::${n}\n    fn ${n.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}(&mut self, effect: NodeRef, control: NodeRef, /* ... */) -> Option<NodeRef>;`).join("\n")}

    // —— Allocate 元素/正则 (${uniqueAlloc.length} 个) ——
${uniqueAlloc.map((n) => `    /// v8: src/compiler/js-create-lowering.h JSCreateLowering::${n}\n    fn ${n.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}(&mut self, effect: NodeRef, control: NodeRef, /* ... */) -> NodeRef;`).join("\n")}
}

// —— 快字面量内联上限 (本 cc 仅使用, 定义在 compiler/globals.h) ——
/// v8: src/compiler/globals.h kMaxFastLiteralDepth (= 3)
pub const K_MAX_FAST_LITERAL_DEPTH: i32 = 3;
/// v8: src/compiler/globals.h kMaxFastLiteralProperties (= JSObject::kMaxInObjectProperties)
pub const K_MAX_FAST_LITERAL_PROPERTIES: i32 = JSObject::MAX_IN_OBJECT_PROPERTIES;

// —— 对数基准 ——
// Reduce ${uniqueReduce.length} / TryAllocate ${uniqueTry.length} / Allocate ${uniqueAlloc.length}
// 上限常量使用 ${uses} 处
`;

const p = writeSrc("v8-compiler-frontend", "js_create_lowering.rs.skel", body);
console.log(`✓ js-create-lowering: Reduce ${uniqueReduce.length} / TryAllocate ${uniqueTry.length} / Allocate ${uniqueAlloc.length} / 上限使用 ${uses} 处 / 锚点 ${anchors.length + ccAnchors.length}`);
