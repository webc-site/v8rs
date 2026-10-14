#!/usr/bin/env bun
// tool/src/compiler/js-generic-lowering.js
// 从 v8/src/compiler/js-generic-lowering.h + js-generic-lowering.cc
// 提取 JS_OP_LIST 展开的 Lower* 方法面 (以 .cc 定义清单为准) 与替换 helper 协议
// 生成 v8-compiler-frontend/src/js_generic_lowering.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/js-generic-lowering.h");
const cc = readV8("src/compiler/js-generic-lowering.cc");

// —— 1. 锚点断言 ——
const anchors = [
  ["class JSGenericLowering final : public AdvancedReducer {", "JSGenericLowering 类"],
  ['return "JSGenericLowering";', "reducer_name"],
  ["#define DECLARE_LOWER(x, ...) void Lower##x(Node* node);", "JS_OP_LIST DECLARE_LOWER 宏"],
  ["JS_OP_LIST(DECLARE_LOWER)", "JS_OP_LIST 展开"],
  ["void ReplaceWithBuiltinCall(Node* node, Builtin builtin);", "ReplaceWithBuiltinCall"],
  ["void ReplaceWithRuntimeCall(Node* node, Runtime::FunctionId f, int args = -1);", "ReplaceWithRuntimeCall"],
  ["void ReplaceUnaryOpWithBuiltinCall(Node* node, Builtin builtin);", "ReplaceUnaryOp"],
  ["void ReplaceBinaryOpWithBuiltinCall(", "ReplaceBinaryOp (双内建: 无反馈/带反馈)"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit)) throw new Error(`js-generic-lowering.h 锚点缺失: ${name}`);
}
const ccAnchors = [
  ["Reduction JSGenericLowering::Reduce(Node* node) {", "Reduce 分派实现"],
  ["void JSGenericLowering::ReplaceWithBuiltinCall(", "ReplaceWithBuiltinCall 实现"],
  ["void JSGenericLowering::ReplaceWithRuntimeCall(", "ReplaceWithRuntimeCall 实现"],
];
for (const [lit, name] of ccAnchors) {
  if (!cc.includes(lit)) throw new Error(`js-generic-lowering.cc 锚点缺失: ${name}`);
}

// —— 2. 真解析: .cc 的 Lower* 定义清单 (JS_OP_LIST 实际展开面) ——
const lowerDefs = [...cc.matchAll(/^void JSGenericLowering::(Lower\w+)\(Node\* node\) \{/gm)].map((m) => m[1]);
if (lowerDefs.length < 80) throw new Error(`Lower* 定义不足: ${lowerDefs.length}`);

// —— 3. Reduce 分派: 逐 opcode switch 面 ——
const switchCases = [...cc.matchAll(/case IrOpcode::k(\w+):/g)].map((m) => m[1]);

// —— 4. ReplaceWithBuiltinCall 重载数 ——
const builtinOverloads = (cc.match(/^void JSGenericLowering::ReplaceWithBuiltinCall\(/gm) || []).length;
const runtimeOverloads = (cc.match(/^void JSGenericLowering::ReplaceWithRuntimeCall\(/gm) || []).length;

const body = `${banner("src/compiler/js-generic-lowering.js", "src/compiler/js-generic-lowering.h + js-generic-lowering.cc", "JS 算子通用降级: 逐 opcode 换 runtime/IC 内建调用")}

/// JSGenericLowering — 把 JS 级算子降为 "generic" 情形的 runtime/IC 调用
/// v8: src/compiler/js-generic-lowering.h JSGenericLowering
pub struct JSGenericLowering { jsgraph: JSGraphRef, broker: JSHeapBrokerRef }

impl JSGenericLowering {
    /// 总分派 — 按 IrOpcode 逐个 Lower${"##"}x (JS_OP_LIST 全展开)
    /// v8: src/compiler/js-generic-lowering.cc JSGenericLowering::Reduce
    fn reduce(&mut self, node: &Node) -> Reduction;

    // —— Lower* 家族: .cc 定义 ${lowerDefs.length} 个 (JS_OP_LIST 1:1) ——
    // 代表 (全部 \`void fn(Node* node)\` 单参型):
${lowerDefs.slice(0, 10).map((n) => `    // fn ${n.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}(node) /// v8: src/compiler/js-generic-lowering.cc JSGenericLowering::${n}`).join("\n")}
    // ... 其余 ${lowerDefs.length - 10} 个同型

    // —— 替换 helper ——
    /// v8: src/compiler/js-generic-lowering.cc JSGenericLowering::ReplaceWithBuiltinCall
    /// 重载 ${builtinOverloads} 个: (node, Builtin) / (node, Callable, Flags) / (node, Callable, Flags, Properties)
    fn replace_with_builtin_call(&mut self, node: &Node, builtin: Builtin /*, 重载差异 */);
    /// v8: src/compiler/js-generic-lowering.cc JSGenericLowering::ReplaceWithRuntimeCall
    /// 重载见 h; args = -1 表示按节点输入数
    fn replace_with_runtime_call(&mut self, node: &Node, f: RuntimeFunctionId, args: i32);
    /// v8: src/compiler/js-generic-lowering.cc JSGenericLowering::ReplaceUnaryOpWithBuiltinCall
    fn replace_unary_op_with_builtin_call(&mut self, node: &Node, builtin: Builtin);
    /// 双内建: builtin_without_feedback / builtin_with_feedback
    /// v8: src/compiler/js-generic-lowering.cc JSGenericLowering::ReplaceBinaryOpWithBuiltinCall
    fn replace_binary_op_with_builtin_call(&mut self, node: &Node, without_feedback: Builtin, with_feedback: Builtin);
}

// —— 对数基准 ——
// Lower* 定义 ${lowerDefs.length} / Reduce switch case ${switchCases.length}
// BuiltinCall 重载 ${builtinOverloads} / RuntimeCall 重载 ${runtimeOverloads}
`;

const p = writeSrc("v8-compiler-frontend", "js_generic_lowering.rs.skel", body);
console.log(`✓ js-generic-lowering: Lower* ${lowerDefs.length} / switch case ${switchCases.length} / 重载 ${builtinOverloads}+${runtimeOverloads} / 锚点 ${anchors.length + ccAnchors.length}`);
