#!/usr/bin/env bun
// tool/src/compiler/turboshaft/wasm-turboshaft-compiler.h.js
// 机械提取 v8/src/compiler/turboshaft/wasm-turboshaft-compiler.{h,cc}
// → v8-turboshaft/src/wasm_turboshaft_compiler.rs.skel
//
// 提取面: ExecuteTurboshaftWasmCompilation 签名与后置断言;
// TurboshaftCompilationJob。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/wasm-turboshaft-compiler.h");
const cc = readV8("src/compiler/turboshaft/wasm-turboshaft-compiler.cc");

// 1. 哨兵
for (const [re, name] of [
  [/wasm::WasmCompilationResult ExecuteTurboshaftWasmCompilation\(/, "入口签名"],
  [/wasm::CompilationEnv\*, WasmCompilationData&, wasm::WasmDetectedFeatures\*,\s*\n\s*DelayedCounterUpdates\*/, "参数面"],
  [/class TurboshaftCompilationJob : public OptimizedCompilationJob/, "Job 类"],
  [/OptimizedCompilationJob\("Turboshaft", initial_state\)/, "Job 名"],
  [/OptimizedCompilationInfo\* compilation_info\(\) const/, "info 访问器"],
  [/#error This header should only be included if WebAssembly is enabled\./, "wasm 门"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. cc 流程断言
for (const [re, name] of [
  [/Pipeline::GenerateWasmCode\(env, data, detected, counter_updates\);/, "委托 Pipeline"],
  [/DCHECK\(result\.succeeded\(\)\);/, "成功断言"],
  [/DCHECK_EQ\(wasm::ExecutionTier::kTurbofan, result\.result_tier\);/, "tier 断言"],
  [/DCHECK_NULL\(result\.assumptions\);/, "assumptions 初始空断言"],
  [/result\.assumptions = std::move\(data\.assumptions\);/, "assumptions 移交"],
  [/DCHECK_IMPLIES\(result\.assumptions, !result\.assumptions->empty\(\)\);/, "assumptions 非空断言"],
]) {
  if (!re.test(cc)) throw new Error(`cc 哨兵缺失: ${name}`);
}

let rust = banner(
  "src/compiler/turboshaft/wasm-turboshaft-compiler.h.js",
  "src/compiler/turboshaft/wasm-turboshaft-compiler.{h,cc}",
  "wasm 编译入口: Turboshaft tier 的 job 与执行",
);

rust += `
/// 原版 ExecuteTurboshaftWasmCompilation — 委托 Pipeline::GenerateWasmCode,
/// 后置断言: succeeded / result_tier == kTurbofan / assumptions 移交非空。
/// v8: src/compiler/turboshaft/wasm-turboshaft-compiler.{h,cc}
pub fn execute_turboshaft_wasm_compilation(
  env: &CompilationEnv, data: &mut WasmCompilationData,
  detected: &mut WasmDetectedFeatures,
  counter_updates: &mut DelayedCounterUpdates,
) -> WasmCompilationResult { todo!() }

/// 原版 TurboshaftCompilationJob — OptimizedCompilationJob 子类,
/// 名字固定 "Turboshaft", 持 compilation_info_。
/// v8: wasm-turboshaft-compiler.h TurboshaftCompilationJob
pub struct TurboshaftCompilationJob {
  pub compilation_info: *mut OptimizedCompilationInfo,
}
`;

const p = writeSrc("v8-turboshaft", "wasm_turboshaft_compiler.rs.skel", rust);
console.log(`written ${p}`);
console.log(`入口=1 Job=1 cc断言=6`);
