#!/usr/bin/env bun
// tool/src/compiler/turboshaft/wasm-wrappers.h.js
// 机械提取 v8/src/compiler/turboshaft/wasm-wrappers.{h,cc,-inl.h}
// → v8-turboshaft/src/wasm_wrappers.rs.skel
//
// 提取面: WasmInlinedFunctionData/WasmBodyInliningResult; builder 方法面;
// wrapper 分派表 (CodeKind); 组装器 reducer 栈。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/wasm-wrappers.h");
const cc = readV8("src/compiler/turboshaft/wasm-wrappers.cc");
const inl = readV8("src/compiler/turboshaft/wasm-wrappers-inl.h");

// 1. 数据结构
const ifdM = h.match(/struct WasmInlinedFunctionData \{([\s\S]*?)\n\};/);
if (!ifdM) throw new Error("未找到 WasmInlinedFunctionData");
const ifdFields = [...ifdM[1].matchAll(/^\s{2}([\w:<>. *]+?)\s+(\w+)(?: = [^;]+)?;/gm)].map((m) => `${m[1]} ${m[2]}`);
if (ifdFields.length !== 5) throw new Error(`InlinedFunctionData 字段 ${ifdFields.length} != 5`);
if (!/struct WasmBodyInliningResult \{/.test(h) || !/static WasmBodyInliningResult Failed\(\)/.test(h)) {
  throw new Error("未找到 WasmBodyInliningResult");
}

// 2. builder 类与方法面
if (!/class WasmWrapperTSGraphBuilder : public wasm::WasmGraphBuilderBase<Assembler>/.test(h)) {
  throw new Error("未找到 builder 基类");
}
const hMethods = [...h.matchAll(/^\s{2}void (Build\w+)\(/gm)].map((m) => m[1]);
const buildFamilies = [...new Set([...inl.matchAll(/WasmWrapperTSGraphBuilder<Assembler>::(Build\w+)\(/g)].map((m) => m[1]))];
if (buildFamilies.length < 6) throw new Error(`Build 族 ${buildFamilies.length} < 6`);
for (const must of ["BuildJSToWasmWrapper", "BuildWasmToJSWrapper", "BuildCapiCallWrapper", "BuildCWasmEntryWrapper", "BuildWasmStackEntryWrapper"]) {
  if (!buildFamilies.includes(must)) throw new Error(`缺 ${must}`);
}

// 3. cc 分派表 (4 直转 + WASM_TO_JS 二级分派)
const dispatch = [...cc.matchAll(/case CodeKind::(\w+):\s*\n\s*return builder\.(\w+)\(/g)].map((m) => `${m[1]} -> ${m[2]}`);
if (dispatch.length !== 4) throw new Error(`直转分派 ${dispatch.length} != 4`);
if (!/WASM_TO_JS_FUNCTION[\s\S]{0,220}?BuildJSFastApiCallWrapper\(callable\);[\s\S]{0,220}?BuildWasmToJSWrapper\(wrapper_info\.import_kind,/.test(cc)) {
  throw new Error("缺 WASM_TO_JS 二级分派");
}

// 4. cc reducer 栈与 helper
const stackM = cc.match(/using WrapperAssembler =\s*\n\s*Assembler<([\s\S]*?)>;/);
if (!stackM) throw new Error("未找到 WrapperAssembler 栈");
const stack = stackM[1].replace(/\s+/g, "").split(",").filter(Boolean);
if (!/GetBuiltinCallDescriptor\(Builtin name, Zone\* zone\)/.test(cc.replace(/\n\s*/g, " ").replace("const TSCallDescriptor* ", "")) && !cc.includes("GetBuiltinCallDescriptor")) {
  throw new Error("cc 缺 GetBuiltinCallDescriptor");
}
const inlMethods = [...new Set([...inl.matchAll(/WasmWrapperTSGraphBuilder<Assembler>::(\w+)\(/g)].map((m) => m[1]))];

let rust = banner(
  "src/compiler/turboshaft/wasm-wrappers.h.js",
  "src/compiler/turboshaft/wasm-wrappers.{h,cc,-inl.h}",
  "wasm 包装器: JS<->wasm 入口图构建",
);

rust += `
/// 原版 WasmInlinedFunctionData — wasm-in-JS 内联所需元数据
/// v8: wasm-wrappers.h WasmInlinedFunctionData
pub struct WasmInlinedFunctionData {
${ifdFields.map((f) => `  /// 原版 ${f}\n  pub ${f.split(" ").slice(-1)[0]}: ${f.split(" ")[0]},`).join("\n")}
}

/// 原版 WasmBodyInliningResult — 内联成败 + 结果值
/// v8: wasm-wrappers.h WasmBodyInliningResult
pub struct WasmBodyInliningResult {
  pub success: bool,
  pub result: OptionalV<Any>,
}

/// 原版 WasmWrapperTSGraphBuilder<Assembler> — 继承
/// wasm::WasmGraphBuilderBase<Assembler>; 头文件声明面 (${hMethods.length} 个 Build)。
/// Build 族实现面 (-inl.h, ${buildFamilies.length} 个):
${buildFamilies.map((b) => `/// - ${b}`).join("\n")}
/// BuildJSToWasmWrapper 有内联重载 (js_closure, js_context, args,
/// inlined_function_data) 与独立编译重载两版。
/// v8: wasm-wrappers.{h,-inl.h}
pub struct WasmWrapperTSGraphBuilder;

/// 原版 BuildWasmWrapper 分派 (cc):
${dispatch.map((d) => `/// - CodeKind::${d}`).join("\n")}
/// - CodeKind::WASM_TO_JS_FUNCTION: import_kind == kWasmToJSFastApi 时
///   BuildJSFastApiCallWrapper(callable), 否则 BuildWasmToJSWrapper(
///   import_kind, expected_arity, suspend); default: UNREACHABLE
/// v8: wasm-wrappers.cc BuildWasmWrapper
///
/// WrapperAssembler 栈: Assembler<${stack.join(", ")}>
/// 另有 cc GetBuiltinCallDescriptor(name, zone) — Linkage 装配 stub 描述符。
///
/// -inl.h 其余方法面 (${inlMethods.length} 个): ${inlMethods.join(", ")}
pub fn build_wasm_wrapper(data: &mut PipelineData, sig: &CanonicalSig,
                          info: WrapperCompilationInfo) { todo!() }
`;

const p = writeSrc("v8-turboshaft", "wasm_wrappers.rs.skel", rust);
console.log(`written ${p}`);
console.log(`InlinedData字段=${ifdFields.length} Build族=${buildFamilies.length} 分派=${dispatch.length} reducer栈=${stack.length} inl方法=${inlMethods.length}`);
