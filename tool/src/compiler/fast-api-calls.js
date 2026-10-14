#!/usr/bin/env bun
// tool/src/compiler/fast-api-calls.js
// 从 v8/src/compiler/fast-api-calls.h + fast-api-calls.cc
// 提取 CTypeInfo::Type → ElementsKind 映射表、OverloadsResolutionResult 与快速调用构建签名
// 生成 v8-compiler-frontend/src/fast_api_calls.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/fast-api-calls.h");
const cc = readV8("src/compiler/fast-api-calls.cc");

// —— 1. 锚点断言 ——
const anchors = [
  ["struct OverloadsResolutionResult {", "OverloadsResolutionResult 结构"],
  ["return OverloadsResolutionResult(-1, CTypeInfo::Type::kVoid);", "Invalid() = (-1, kVoid)"],
  ["bool is_valid() const { return distinguishable_arg_index >= 0; }", "is_valid 判据"],
  ["using GetParameter = std::function<Node*(int, GraphAssemblerLabel<0>*)>;", "GetParameter 回调"],
  ["using ConvertReturnValue = std::function<Node*(const CFunctionInfo*, Node*)>;", "ConvertReturnValue 回调"],
  ["using InitializeOptions = std::function<void(Node*)>;", "InitializeOptions 回调"],
  ["using GenerateSlowApiCall = std::function<Node*()>;", "GenerateSlowApiCall 回调"],
  ["Node* BuildFastApiCall(", "BuildFastApiCall 入口"],
  ["FastApiCallFunction GetFastApiCallTarget(", "GetFastApiCallTarget 入口"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit)) throw new Error(`fast-api-calls.h 锚点缺失: ${name}`);
}
const ccAnchors = [
  ["ElementsKind GetTypedArrayElementsKind(CTypeInfo::Type type) {", "映射函数"],
  ["bool CanOptimizeFastSignature(const CFunctionInfo* c_signature) {", "签名可优化判定"],
  ["class FastApiCallBuilder {", "Builder 类"],
  ["Node* FastApiCallBuilder::WrapFastCall(", "WrapFastCall"],
  ["void FastApiCallBuilder::PropagateException() {", "PropagateException"],
  ["Node* FastApiCallBuilder::Build(", "Build 主流程"],
  ["static constexpr int kReceiver = 1;", "kReceiver=1"],
  ["static_assert(kSize == sizeof(uintptr_t) * 2);", "FastApiCallbackOptions 尺寸 2×uintptr"],
];
for (const [lit, name] of ccAnchors) {
  if (!cc.includes(lit)) throw new Error(`fast-api-calls.cc 锚点缺失: ${name}`);
}

// —— 2. 真解析: CTypeInfo::Type → ElementsKind 映射 ——
const mapSrc = cc.slice(
  cc.indexOf("ElementsKind GetTypedArrayElementsKind"),
  cc.indexOf("bool CanOptimizeFastSignature"),
);
const pairs = [...mapSrc.matchAll(/case CTypeInfo::Type::k(\w+):\s*\n\s*return (\w+)_ELEMENTS;/g)]
  .map((m) => [m[1], m[2]]);
if (pairs.length !== 7) throw new Error(`映射应 7 项, 得 ${pairs.length}`);
const unreachable = [...mapSrc.matchAll(/case CTypeInfo::Type::k(\w+):/g)].map((m) => m[1])
  .filter((k) => !pairs.some(([a]) => a === k));
const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

// —— 3. 可优化规则 (平台宏分派) ——
const ruleCount = (cc.match(/return CpuFeatures::IsSupported\(SSE4_2\);|return false;/g) || []).length;

const body = `${banner("src/compiler/fast-api-calls.js", "src/compiler/fast-api-calls.h + fast-api-calls.cc", "Fast API 调用: 类型映射 + 可优化签名判定 + 快/慢路径子图构建")}
//! fast_api_call — 把可优化的 C 签名 API 调用降为直接 C 调用子图。

/// CTypeInfo 类型 → TypedArray ElementsKind 映射 (7 项 1:1)
/// v8: src/compiler/fast-api-calls.cc GetTypedArrayElementsKind
pub fn typed_array_elements_kind(ty: CTypeInfoType) -> ElementsKind {
    match ty {
${pairs.map(([k, v]) => `        CTypeInfoType::${k} => ElementsKind::${snake(v)},`).join("\n")}
        // UNREACHABLE: ${unreachable.join(" / ")}
        _ => unreachable!(),
    }
}

/// 重载决议结果
/// v8: src/compiler/fast-api-calls.h OverloadsResolutionResult
pub struct OverloadsResolutionResult {
    /// 可区分重载参数的下标; 仅支持该参数为 JSArray vs TypedArray 的情形
    /// is_valid = distinguishable_arg_index >= 0
    /// v8: src/compiler/fast-api-calls.h OverloadsResolutionResult::is_valid
    pub distinguishable_arg_index: i32,
    /// TypedArray 参数的元素类型; Invalid() 时为 kVoid
    pub element_type: CTypeInfoType,
    /// 目标地址节点 (决议后填)
    pub target_address: Option<NodeRef>,
}

impl OverloadsResolutionResult {
    /// v8: src/compiler/fast-api-calls.h OverloadsResolutionResult::Invalid
    pub fn invalid() -> Self { Self { distinguishable_arg_index: -1, element_type: CTypeInfoType::Void, target_address: None } }
}

/// 签名可否优化为快速调用 — 平台规则 (${ruleCount} 条 return 分支):
/// - MacArm64: 参数 > 8 → false (不支持栈传参)
/// - 无 FP-in-C-linkage / 模拟器: 返回或参数含 float32/float64 → false
/// - 32 位: 返回或参数含 int64/uint64 → false
/// - x64: kClampBit 需要 SSE4_2
/// v8: src/compiler/fast-api-calls.cc CanOptimizeFastSignature
pub fn can_optimize_fast_signature(c_signature: &CFunctionInfo) -> bool;

/// 回调面 (4 个闭包参数)
/// v8: src/compiler/fast-api-calls.h GetParameter/ConvertReturnValue/InitializeOptions/GenerateSlowApiCall
pub type GetParameter<'a> = &'a dyn Fn(i32, &GraphAssemblerLabel) -> NodeRef;
pub type ConvertReturnValue<'a> = &'a dyn Fn(&CFunctionInfo, NodeRef) -> NodeRef;
pub type InitializeOptions<'a> = &'a dyn Fn(NodeRef);
pub type GenerateSlowApiCall<'a> = &'a dyn Fn() -> NodeRef;

/// 快速调用子图构建 (if_success/if_error 双出口 + 异常传播)
/// v8: src/compiler/fast-api-calls.cc FastApiCallBuilder::Build
pub fn build_fast_api_call(
    isolate: &Isolate,
    graph: &mut TFGraph,
    graph_assembler: &mut GraphAssembler,
    c_function: FastApiCallFunction,
    data_argument: NodeRef,
    get_parameter: GetParameter<'_>,
    convert_return_value: ConvertReturnValue<'_>,
    initialize_options: InitializeOptions<'_>,
    generate_slow_api_call: GenerateSlowApiCall<'_>,
) -> NodeRef;

/// 挑选与 arg_count 匹配的可优化重载
/// kReceiver = 1 (首个 C 参数是 receiver)
/// v8: src/compiler/fast-api-calls.cc GetFastApiCallTarget
pub fn get_fast_api_call_target(
    broker: &JSHeapBroker,
    function_template_info: FunctionTemplateInfoRef,
    arg_count: usize,
) -> FastApiCallFunction;

// Call 节点输入布局: [fast callee, receiver, ...C args, [Options], effect, control]
// FastApiCallbackOptions 栈槽 = 2 × uintptr (isolate + data)
// —— 对数基准 ——
// 类型映射 ${pairs.length} 项 / UNREACHABLE ${unreachable.length} 项
`;

const p = writeSrc("v8-compiler-frontend", "fast_api_calls.rs.skel", body);
console.log(`✓ fast-api-calls: 类型映射 ${pairs.length} / UNREACHABLE ${unreachable.length} / 锚点 ${anchors.length + ccAnchors.length}`);
