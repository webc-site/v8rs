#!/usr/bin/env bun
// tool/src/compiler/js-intrinsic-lowering.js
// 从 v8/src/compiler/js-intrinsic-lowering.h + js-intrinsic-lowering.cc
// 提取 Runtime 函数 id → Reduce 分派表 (RUNTIME 5 + INLINE 20) 与 FrameStateFlag
// 生成 v8-compiler-frontend/src/js_intrinsic_lowering.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/js-intrinsic-lowering.h");
const cc = readV8("src/compiler/js-intrinsic-lowering.cc");

// —— 1. 锚点断言 ——
const anchors = [
  ["class V8_EXPORT_PRIVATE JSIntrinsicLowering final", "JSIntrinsicLowering 类"],
  ['return "JSIntrinsicLowering";', "reducer_name"],
  ["enum FrameStateFlag {", "FrameStateFlag 枚举"],
  ["kNeedsFrameState,", "kNeedsFrameState"],
  ["kDoesNotNeedFrameState,", "kDoesNotNeedFrameState"],
  ["Reduction ReduceCopyDataProperties(Node* node);", "ReduceCopyDataProperties 声明"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit)) throw new Error(`js-intrinsic-lowering.h 锚点缺失: ${name}`);
}
const ccAnchors = [
  ["if (node->opcode() != IrOpcode::kJSCallRuntime) return NoChange();", "只吃 JSCallRuntime"],
  ["if (f->intrinsic_type != Runtime::IntrinsicType::INLINE) return NoChange();", "INLINE 类型闸"],
  ["Reduction JSIntrinsicLowering::ReduceCopyDataProperties(Node* node) {", "首个 Reduce 实现"],
];
for (const [lit, name] of ccAnchors) {
  if (!cc.includes(lit)) throw new Error(`js-intrinsic-lowering.cc 锚点缺失: ${name}`);
}

// —— 2. 真解析: 分派表 case Runtime::kX → ReduceY ——
const dispatchSrc = cc.slice(cc.indexOf("switch (f->function_id) {"), cc.indexOf("Reduction JSIntrinsicLowering::ReduceCopyDataProperties"));
const pairs = [...dispatchSrc.matchAll(/case Runtime::k(\w+):\s*\n\s*return (Reduce\w+)\(node\);/g)]
  .map((m) => [m[1], m[2]]);
const inlinePairs = pairs.filter(([k]) => k.startsWith("Inline"));
const runtimePairs = pairs.filter(([k]) => !k.startsWith("Inline"));
if (inlinePairs.length !== 20 || runtimePairs.length !== 5) {
  throw new Error(`分派表提取异常: INLINE ${inlinePairs.length} / RUNTIME ${runtimePairs.length}`);
}

// —— 3. 真解析: h 声明面 vs cc 实现面 ——
const declared = [...new Set([...h.matchAll(/Reduction (Reduce\w+)\(Node\* node/g)].map((m) => m[1]))];
const implemented = new Set([...cc.matchAll(/^Reduction JSIntrinsicLowering::(Reduce\w+)\(/gm)].map((m) => m[1]));

const body = `${banner("src/compiler/js-intrinsic-lowering.js", "src/compiler/js-intrinsic-lowering.h + js-intrinsic-lowering.cc", "JS 运行时内建调用降级: JSCallRuntime → stub/simplified 节点")}

/// JSIntrinsicLowering — 只吃 IrOpcode::kJSCallRuntime, 按函数 id 分派
/// v8: src/compiler/js-intrinsic-lowering.h JSIntrinsicLowering
pub struct JSIntrinsicLowering { jsgraph: JSGraphRef, broker: JSHeapBrokerRef }

/// 帧态需求旗标 (Change 走 Callable 重载时的参数)
/// v8: src/compiler/js-intrinsic-lowering.h JSIntrinsicLowering::FrameStateFlag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameStateFlag {
    NeedsFrameState,
    DoesNotNeedFrameState,
}

impl JSIntrinsicLowering {
    /// 分派总闸: 非 INLINE intrinsic_type 直接 NoChange
    /// v8: src/compiler/js-intrinsic-lowering.cc JSIntrinsicLowering::Reduce
    fn reduce(&mut self, node: &Node) -> Reduction {
        match runtime_function_id(node) {
${runtimePairs.map(([k, r]) => `            // Runtime::${k} => ${r.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}() /// v8: src/compiler/js-intrinsic-lowering.cc case Runtime::${k}`).join("\n")}
            _ => {}
        }
        if !is_inline_intrinsic(node) { return NoChange; }
        match runtime_function_id(node) {
${inlinePairs.map(([k, r]) => `            // Runtime::${k} => ${r.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}() /// v8: src/compiler/js-intrinsic-lowering.cc case Runtime::${k}`).join("\n")}
            _ => {}
        }
        NoChange
    }

    // Change 四型: (op) / (op,a,b) / (op,a,b,c) / (op,a,b,c,d)
    /// v8: src/compiler/js-intrinsic-lowering.h JSIntrinsicLowering::Change
    fn change(&mut self, node: &Node, op: &Operator, /* vars: &[NodeRef] */);
    /// v8: src/compiler/js-intrinsic-lowering.h JSIntrinsicLowering::Change (Callable 重载)
    fn change_callable(&mut self, node: &Node, callable: Callable, stack_parameter_count: i32, frame_state_flag: FrameStateFlag);
}

// —— 分派表对数 (RUNTIME ${runtimePairs.length} + INLINE ${inlinePairs.length} = ${pairs.length}) ——
// h 声明 ${declared.length} 个 Reduce* vs cc 实现 ${implemented.size} 个
// 未实现于分派 (直接走 helper): ${declared.filter((d) => !implemented.has(d)).join(", ") || "无"}

// —— 对数基准 ——
// 分派 ${pairs.length} 路 / 声明 ${declared.length} / 实现 ${implemented.size}
`;

const p = writeSrc("v8-compiler-frontend", "js_intrinsic_lowering.rs.skel", body);
console.log(`✓ js-intrinsic-lowering: 分派 ${pairs.length} (RUNTIME ${runtimePairs.length}+INLINE ${inlinePairs.length}) / 声明 ${declared.length} / 实现 ${implemented.size} / 锚点 ${anchors.length + ccAnchors.length}`);
