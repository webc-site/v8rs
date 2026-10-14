#!/usr/bin/env bun
// tool/src/compiler/js-typed-lowering.js
// 从 v8/src/compiler/js-typed-lowering.h + js-typed-lowering.cc
// 提取 Signedness 枚举、Reduce 方法面 (49 个) 与 JSBinopReduction 协作类
// 生成 v8-compiler-frontend/src/js_typed_lowering.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/js-typed-lowering.h");
const cc = readV8("src/compiler/js-typed-lowering.cc");

// —— 1. 锚点断言 ——
const anchors = [
  ["enum Signedness { kSigned, kUnsigned };", "Signedness 枚举"],
  ["class V8_EXPORT_PRIVATE JSTypedLowering final", "JSTypedLowering 类"],
  ['return "JSTypedLowering";', "reducer_name"],
  ["friend class JSBinopReduction;", "JSBinopReduction 友元"],
  ["Reduction ReduceJSAdd(Node* node);", "ReduceJSAdd"],
  ["Reduction ReduceUI32Shift(Node* node, Signedness signedness);", "ReduceUI32Shift 带枚举参"],
  ["Reduction ReduceJSToNumberInput(Node* input);", "ReduceJSToNumberInput (输入参)"],
  ["Node* BuildGetModuleCell(Node* node);", "BuildGetModuleCell"],
  ["bool should_create_cons_string);", "GenerateStringAddition 参数"],
  ["Type empty_string_type_;", "两个缓存类型成员"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit)) throw new Error(`js-typed-lowering.h 锚点缺失: ${name}`);
}
const ccAnchors = [
  ["Reduction JSTypedLowering::Reduce(Node* node) {", "Reduce 分派"],
  ["class JSBinopReduction final", "JSBinopReduction 类"],
  ["Reduction JSTypedLowering::ReduceJSAdd(Node* node) {", "ReduceJSAdd 实现"],
  ["Reduction JSTypedLowering::GenerateStringAddition(", "GenerateStringAddition 实现"],
];
for (const [lit, name] of ccAnchors) {
  if (!cc.includes(lit)) throw new Error(`js-typed-lowering.cc 锚点缺失: ${name}`);
}

// —— 2. 真解析: Reduce* 方法面 (含 input 变体) ——
const reduceFns = [...new Set([...h.matchAll(/Reduction (Reduce\w+)\((?:Node\* node|Node\* input)/g)].map((m) => m[1]))];
if (reduceFns.length < 45) throw new Error(`Reduce 面不足: ${reduceFns.length}`);

// —— 3. cc 实现面 ——
const impls = new Set([...cc.matchAll(/^Reduction JSTypedLowering::(Reduce\w+)\(/gm)].map((m) => m[1]));

// —— 4. JSBinopReduction 方法面 (cc 协作类) ——
const binopSrc = cc.slice(cc.indexOf("class JSBinopReduction final"), cc.indexOf("Reduction JSTypedLowering::Reduce("));
const binopMethods = [...new Set([...binopSrc.matchAll(/^\s+(?:bool|void|Reduction|Node\*|Type) (\w+)\(/gm)].map((m) => m[1]))]
  .filter((n) => !["JSTypedLowering"].includes(n));

const body = `${banner("src/compiler/js-typed-lowering.js", "src/compiler/js-typed-lowering.h + js-typed-lowering.cc", "类型驱动降级: JS 算子 → simplified 算子")}

/// 移位符号性
/// v8: src/compiler/js-typed-lowering.h Signedness
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signedness {
    Signed,
    Unsigned,
}

/// JSTypedLowering — 按类型把 JS 级算子降为 simplified 级
/// v8: src/compiler/js-typed-lowering.h JSTypedLowering
pub struct JSTypedLowering {
    jsgraph: JSGraphRef,
    broker: JSHeapBrokerRef,
    /// 两个缓存类型: empty_string_type_ / pointer_comparable_type_
    type_cache: TypeCacheRef,
}

impl JSTypedLowering {
    /// 总分派 Reduce (按 IrOpcode)
    /// v8: src/compiler/js-typed-lowering.cc JSTypedLowering::Reduce
    fn reduce(&mut self, node: &Node) -> Reduction;

    // —— Reduce 面 ${reduceFns.length} 个 (cc 实现 ${impls.size}) ——
${reduceFns.slice(0, 16).map((n) => `    /// v8: src/compiler/js-typed-lowering.h JSTypedLowering::${n}\n    fn ${n.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}(&mut self, node: &Node) -> Reduction;`).join("\n")}
    // ... 其余 ${reduceFns.length - 16} 个 (${reduceFns.slice(16, 30).join(",").replace(/Reduce/g, "")} ...)
${reduceFns.slice(30).map((n) => `    /// v8: src/compiler/js-typed-lowering.h JSTypedLowering::${n}\n    fn ${n.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}(&mut self, node: &Node) -> Reduction;`).join("\n")}

    /// ReduceJSAdd 字符串加法子图 (should_create_cons_string 分叉)
    /// v8: src/compiler/js-typed-lowering.cc JSTypedLowering::GenerateStringAddition
    fn generate_string_addition(&mut self, node: &Node, left: NodeRef, right: NodeRef, context: NodeRef, frame_state: FrameState, effect: &mut NodeRef, control: &mut NodeRef, should_create_cons_string: bool) -> Reduction;
    /// 字符串包装解包
    /// v8: src/compiler/js-typed-lowering.cc JSTypedLowering::UnwrapStringWrapper
    fn unwrap_string_wrapper(&mut self, string_or_wrapper: NodeRef, effect: &mut NodeRef, control: &mut NodeRef) -> NodeRef;
    /// 模块 cell 读取 (Load/StoreModule 共用)
    /// v8: src/compiler/js-typed-lowering.cc JSTypedLowering::BuildGetModuleCell
    fn build_get_module_cell(&mut self, node: &Node) -> NodeRef;
}

// —— JSBinopReduction — 二元算子降级协作类 (${binopMethods.length} 方法) ——
/// v8: src/compiler/js-typed-lowering.cc JSBinopReduction
// 方法面: ${binopMethods.join(", ")}
// (输入类型判定 → 输入改写 → simplified 算子替换三段式)

// —— 对数基准 ——
// Signedness 2 值 / Reduce ${reduceFns.length}(实现 ${impls.size}) / Binop 协作方法 ${binopMethods.length}
`;

const p = writeSrc("v8-compiler-frontend", "js_typed_lowering.rs.skel", body);
console.log(`✓ js-typed-lowering: Reduce ${reduceFns.length}(实现 ${impls.size}) / Binop 协作 ${binopMethods.length} / 锚点 ${anchors.length + ccAnchors.length}`);
