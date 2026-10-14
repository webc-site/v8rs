#!/usr/bin/env bun
// tool/src/wasm/interpreter/wasm-interpreter-runtime.h.js
// 机械提取 v8/src/wasm/interpreter/wasm-interpreter-runtime.{h,cc,-inl.h} 的运行时契约

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/wasm/interpreter/wasm-interpreter-runtime.h");
const ccSrc = readV8("src/wasm/interpreter/wasm-interpreter-runtime.cc");
const inlSrc = readV8("src/wasm/interpreter/wasm-interpreter-runtime-inl.h");

// 1. h: InstanceScope RAII 契约
for (const a of [
  "class V8_NODISCARD InstanceScope final {",
  "IndirectHandle<WasmTrustedInstanceData> const previous_trusted_data_;",
  "Nested scopes save/restore the previous trusted data",
]) {
  if (!hSrc.includes(a)) throw new Error(`InstanceScope 契约缺失: ${a}`);
}

// 2. h: 沙箱安全说明 (off-cage 句柄)
if (!hSrc.includes("// off-cage, validated value instead of re-loading the in-cage")) {
  throw new Error("未找到 off-cage 句柄说明");
}
if (!hSrc.includes("IndirectHandle<WasmTrustedInstanceData> current_trusted_data_;")) {
  throw new Error("未找到 current_trusted_data_ 成员");
}

// 3. h: 间接调用三态
const iccMatch = hSrc.match(/enum class IndirectCallCheck \{ kValid, kInvalid, kNull \};/);
if (!iccMatch) throw new Error("未找到 IndirectCallCheck");

// 4. h: IndirectCallValue::Mode 双态 + 无效索引哨兵
const modeMatch = hSrc.match(/enum class Mode \{([\s\S]*?)\};/);
if (!modeMatch) throw new Error("未找到 IndirectCallValue::Mode");
const modeValues = [...modeMatch[1].replace(/\/\/[^\n]*/g, "").matchAll(/\b(k\w+)/g)].map((m) => m[1]);
if (modeValues.join(",") !== "kInvalid,kInternalCall,kExternalCall") {
  throw new Error(`Mode 值异常: ${modeValues.join(",")}`);
}
if (!hSrc.includes("static const uint32_t kInvalidFunctionIndex = UINT_MAX;")) {
  throw new Error("未找到 kInvalidFunctionIndex");
}
if (!hSrc.includes("sig_index({CanonicalTypeIndex::Invalid()})")) {
  throw new Error("未找到 sig 无效初值");
}

// 5. h: 跟踪栈类型枚举
if (!hSrc.includes("enum class TracePopType : bool {")) throw new Error("未找到 TracePopType");
if (!hSrc.includes("enum class TracePushType : bool {")) throw new Error("未找到 TracePushType");

// 6. h: 分配结果结构 (write barrier 决策)
for (const s of [
  "struct StructNewResult {",
  "struct ArrayNewResult {",
  "bool needs_write_barrier;",
]) {
  if (!hSrc.includes(s)) throw new Error(`分配结果缺失: ${s}`);
}
if (!hSrc.includes("For young-space allocations the")) {
  throw new Error("未找到 young-space 屏障豁免注释");
}

// 7. h: 状态字段面
for (const f of [
  "uint32_t start_function_index_;",
  "int trap_function_index_;",
  "pc_t trap_pc_;",
  "WasmInterpreterThread* current_thread_;",
  "uint8_t* memory_start_;",
  "std::vector<bool> is_memory64_;",
  "PWasmOp* const* instruction_table_;",
]) {
  if (!hSrc.includes(f)) throw new Error(`状态字段缺失: ${f}`);
}

// 8. cc: Torque 槽宽约定 + 数值类型分派
if (!ccSrc.includes("size_t kTqStackSlotSize = sizeof(intptr_t);")) {
  throw new Error("未找到 kTqStackSlotSize");
}
const caseArgCount = (ccSrc.match(/CASE_ARG_TYPE\(kWasm(I32|I64|F32|F64)\.kind\(\), \w+\)/g) || []).length;
if (caseArgCount !== 4) throw new Error(`CASE_ARG_TYPE ${caseArgCount} != 4`);
const caseRetCount = (ccSrc.match(/CASE_RET_TYPE\(kWasm(I32|I64|F32|F64)\.kind\(\), \w+\)/g) || []).length;
if (caseRetCount !== 4) throw new Error(`CASE_RET_TYPE ${caseRetCount} != 4`);
// ref 参数从 GC 可见 FixedArray 读, 不从 arg_buffer 读
if (!ccSrc.includes("// Read reference parameters from the GC-visible FixedArray rather than")) {
  throw new Error("未找到 ref 参数安全通道注释");
}
if (!ccSrc.includes("SBXCHECK(IsFixedArray(*ref_param_array_obj));")) {
  throw new Error("未找到 SBXCHECK");
}

// 9. cc: 方法实现规模
const methodCount = (ccSrc.match(/WasmInterpreterRuntime::/g) || []).length;
if (methodCount < 60) throw new Error(`方法实现 ${methodCount} < 60`);

// 10. inl: 内联访问器
if (!inlSrc.includes("wasm_trusted_instance_data()")) throw new Error("inl 未实现 wasm_trusted_instance_data");
if (!inlSrc.includes("GetGlobalAddress") && !hSrc.includes("GetGlobalAddress(uint32_t index)")) {
  throw new Error("未找到 GetGlobalAddress");
}

console.log(
  `wasm-interpreter-runtime: ARG/RET分派=${caseArgCount}/${caseRetCount} 方法=${methodCount}`,
);

// 11. 生成 Rust
let rust = banner(
  "src/wasm/interpreter/wasm-interpreter-runtime.h.js",
  "src/wasm/interpreter/wasm-interpreter-runtime.{h,cc,-inl.h}",
  "解释器运行时: InstanceScope 发布协议与间接调用分派",
);

rust += `
//! Drumbrake 运行时: 每次 C++ 入口以 InstanceScope 发布当前受信实例数据。
/// v8: src/wasm/interpreter/wasm-interpreter-runtime.h WasmInterpreterRuntime

/// 间接调用签名检查三态
/// v8: src/wasm/interpreter/wasm-interpreter-runtime.h enum class IndirectCallCheck
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndirectCallCheck {
    /// v8: kValid
    Valid,
    /// v8: kInvalid — 签名不匹配
    Invalid,
    /// v8: kNull — 表项为空
    Null,
}

/// 间接调用表项分派模式
/// v8: src/wasm/interpreter/wasm-interpreter-runtime.h IndirectCallValue::Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndirectCallMode {
    /// v8: kInvalid
    Invalid,
    /// v8: kInternalCall — 本实例 wasm 函数, func_index 有效, 直接 ExecuteFunction
    InternalCall,
    /// v8: kExternalCall — 跨实例/JS 边界, 调用时重新解析
    ExternalCall,
}

/// v8: kInvalidFunctionIndex = UINT_MAX
pub const INVALID_FUNCTION_INDEX: u32 = u32::MAX;

/// Torque 侧栈槽宽 (数值参数按 intptr 槽推进, 不按类型宽)
/// v8: src/wasm/interpreter/wasm-interpreter-runtime.cc kTqStackSlotSize
pub const TQ_STACK_SLOT_SIZE_IS_INTPTR: bool = true;

/// 数值参数/返回分派 (i32/i64/f32/f64 四类; ref/s128 特殊路径)
/// v8: src/wasm/interpreter/wasm-interpreter-runtime.cc CASE_ARG_TYPE / CASE_RET_TYPE
pub const NUMERIC_ARG_KINDS: usize = 4;
pub const NUMERIC_RET_KINDS: usize = 4;

/// ref 参数经 GC 可见 FixedArray 传递 (arg_buffer 对 GC 不安全), SBXCHECK 校验
/// v8: src/wasm/interpreter/wasm-interpreter-runtime.cc ref_indices 通道
pub const REF_PARAMS_VIA_FIXED_ARRAY: bool = true;

/// StructNew/ArrayNew 的屏障决策: 老年代分配需写屏障, 年轻代初始化期可豁免
/// v8: src/wasm/interpreter/wasm-interpreter-runtime.h needs_write_barrier 注释
pub const WRITE_BARRIER_BY_ALLOC_SPACE: bool = true;

/// InstanceScope: 嵌套保存/恢复, 句柄根在调用方 HandleScope
/// v8: src/wasm/interpreter/wasm-interpreter-runtime.h InstanceScope
pub const NESTED_INSTANCE_SCOPE_SAVES: bool = true;

/// 沙箱: 受信数据经 off-cage 句柄读, 不重复加载笼内可换句柄
/// v8: current_trusted_data_ 注释
pub const OFF_CAGE_TRUSTED_DATA_HANDLE: bool = true;

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(NUMERIC_ARG_KINDS, 4);
    assert_eq!(NUMERIC_RET_KINDS, 4);
    assert_eq!(INVALID_FUNCTION_INDEX, u32::MAX);
};
`;

const p = writeSrc("v8-wasm-interp", "wasm_interpreter_runtime.rs.skel", rust);
console.log(`written ${p}`);
