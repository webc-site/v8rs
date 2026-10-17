#!/usr/bin/env bun
// tool/src/compiler/js-inlining.js
// 从 v8/src/compiler/js-inlining.h + js-inlining.cc
// 提取 JSInliner 方法面、WasmInlineResult 与人造 FrameState 工厂协议
// 生成 v8-compiler-frontend/src/js_inlining.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/js-inlining.h");
const cc = readV8("src/compiler/js-inlining.cc");

// —— 1. 锚点断言 ——
const anchors = [
  ["class JSInliner final : public AdvancedReducer {", "JSInliner 类"],
  ['return "JSInliner";', "reducer_name"],
  ["Reduction Reduce(Node* node) final { UNREACHABLE(); }", "通用 Reduce 禁用"],
  ["Reduction ReduceJSCall(Node* node);", "ReduceJSCall 主口"],
  ["OptionalSharedFunctionInfoRef DetermineCallTarget(Node* node);", "DetermineCallTarget"],
  ["FeedbackCellRef DetermineCallContext(Node* node, Node** context_out);", "DetermineCallContext"],
  ["FrameState CreateArtificialFrameState(", "CreateArtificialFrameState"],
  ["Reduction InlineCall(Node* call, Node* new_target, Node* context,", "InlineCall"],
  ["struct WasmInlineResult {", "WasmInlineResult 结构"],
  ["bool can_inline_body = false;", "can_inline_body 默认"],
  ["DCHECK_IMPLIES(inline_wasm_fct_if_supported_,", "wasm 内联前提"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit)) throw new Error(`js-inlining.h 锚点缺失: ${name}`);
}
const ccAnchors = [
  ["Reduction JSInliner::ReduceJSCall(Node* node) {", "ReduceJSCall 实现"],
  ["JSInliner::DetermineCallTarget(Node* node) {", "DetermineCallTarget 实现"],
  ["JSInliner::CreateArtificialFrameState(", "CreateArtificialFrameState 实现"],
  ["Reduction JSInliner::InlineCall(", "InlineCall 实现"],
];
for (const [lit, name] of ccAnchors) {
  if (!cc.includes(lit)) throw new Error(`js-inlining.cc 锚点缺失: ${name}`);
}

// —— 2. 真解析: cc 定义面 ——
const impls = [...cc.matchAll(/^[A-Za-z].*JSInliner::(\w+)\(/gm)].map((m) => m[1]);
const uniqueImpls = [...new Set(impls)];

// —— 3. FrameStateType 使用面 (人造帧态三型) ——
const fsTypes = new Set([...cc.matchAll(/FrameStateType::k(\w+)/g)].map((m) => m[1]));

const body = `${banner("src/compiler/js-inlining.js", "src/compiler/js-inlining.h + js-inlining.cc", "JS 内联机制: 一图嵌入另一图的芯子 (不含启发式)")}

/// JSInliner — 图内联芯子: 只管怎么嵌, 不管嵌什么/嵌多少 (启发式在 JSInliningHeuristic)
/// v8: src/compiler/js-inlining.h JSInliner
pub struct JSInliner {
    local_zone: ZoneRef,
    jsgraph: JSGraphRef,
    broker: JSHeapBrokerRef,
    source_positions: SourcePositionTableRef,
    node_origins: NodeOriginTableRef,
    wasm_native_module: Option<WasmNativeModuleRef>,
    /// wasm 支持且函数足够小时连 wasm 函数体一起内联
    inline_wasm_fct_if_supported: bool,
}

impl JSInliner {
    /// 通用 reducer 接口禁用 (UNREACHABLE) — 只经 ReduceJSCall 供启发式直调
    /// v8: src/compiler/js-inlining.h JSInliner::Reduce
    /// v8: src/compiler/js-inlining.h JSInliner::ReduceJSCall
    fn reduce_js_call(&mut self, node: &Node) -> Reduction;

    /// 调用目标决议: JSCall 节点 → SharedFunctionInfo (无则 None)
    /// v8: src/compiler/js-inlining.cc JSInliner::DetermineCallTarget
    fn determine_call_target(&mut self, node: &Node) -> Option<SharedFunctionInfoRef>;

    /// 调用上下文决议: 反馈单元 → FeedbackCellRef, context_out 回传
    /// v8: src/compiler/js-inlining.cc JSInliner::DetermineCallContext
    fn determine_call_context(&mut self, node: &Node, context_out: &mut NodeRef) -> FeedbackCellRef;

    /// 人造 FrameState 工厂 — 三型合用 (构造参数/参数适配器/内联参数)
    /// FrameStateType 使用面: ${[...fsTypes].join(" / ")}
    /// v8: src/compiler/js-inlining.cc JSInliner::CreateArtificialFrameState
    fn create_artificial_frame_state(
        node: &Node, outer: FrameState, parameter_count: i32,
        ty: FrameStateType, shared: SharedFunctionInfoRef,
        maybe_bytecode_array: Option<BytecodeArrayRef>,
        context: Option<NodeRef>, callee: Option<NodeRef>,
    ) -> FrameState;

    /// 嵌入: 异常目标对接 + uncaught 子调用重连
    /// v8: src/compiler/js-inlining.cc JSInliner::InlineCall
    fn inline_call(
        call: &Node, new_target: NodeRef, context: NodeRef, frame_state: FrameState,
        start: StartNode, end: NodeRef, exception_target: Option<NodeRef>,
        uncaught_subcalls: &NodeVector, argument_count: i32,
    ) -> Reduction;
}

/// wasm 内联试探结果
/// v8: src/compiler/js-inlining.h JSInliner::WasmInlineResult
pub struct WasmInlineResult {
    pub can_inline_body: bool,   // = false
    pub body_start: Option<NodeRef>,  // = nullptr
    pub body_end: Option<NodeRef>,    // = nullptr
}

// —— 对数基准 ——
// cc 定义面 ${uniqueImpls.length} (${uniqueImpls.slice(0, 8).join(",")}...)
// FrameStateType ${fsTypes.size} 型
`;

const p = writeSrc("v8-compiler-frontend", "js_inlining.rs.skel", body);
console.log(`✓ js-inlining: cc 定义 ${uniqueImpls.length} / FrameStateType ${fsTypes.size} 型 / 锚点 ${anchors.length + ccAnchors.length}`);
