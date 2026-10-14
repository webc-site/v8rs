#!/usr/bin/env bun
// tool/src/compiler/turbofan-disabled.cc.js
// v8/src/compiler/turbofan-disabled.cc
// → v8-compiler-passes/src/turbofan_disabled.rs.skel
// TF 关闭桩: 解析两个 FATAL stub 的函数面与文案。
import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/compiler/turbofan-disabled.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`turbofan-disabled: ${msg}`);
};

for (const [s, name] of [
  ["std::unique_ptr<TurbofanCompilationJob> NewCompilationJob(", "NewCompilationJob 桩"],
  ["std::unique_ptr<OptimizedCompilationJob> NewJSToWasmCompilationJob(", "NewJSToWasmCompilationJob 桩"],
  ["FATAL(", "FATAL 调用"],
  ["v8_enable_turbofan = false", "构建开关文案"],
]) need(cc.includes(s), `哨兵缺失 ${name}`);

// FATAL 文案计数 (真解析)
const fatals = [...cc.matchAll(/FATAL\(\s*"([^"]+)"/g)].map((m) => m[1]);
need(fatals.length === 2, `FATAL 桩 ${fatals.length}`);
for (const f of fatals) need(f.includes("must not be called"), `文案异常 ${f}`);

const body = `${banner("src/compiler/turbofan-disabled.cc.js", "src/compiler/turbofan-disabled.cc", "v8_enable_turbofan=false 时的 API 桩 (FATAL)")}
//! turbofan-disabled — TF 关闭构建的实现面
//!
//! 两个入口全部 FATAL (调用即错, 不产代码):
//! - NewCompilationJob — "compiler::NewCompilationJob must not be called when Turbofan is disabled"
//! - NewJSToWasmCompilationJob — 同类文案。

/// 原版 NewCompilationJob 桩 — TF 关闭时调用即 FATAL
/// v8: src/compiler/turbofan-disabled.cc NewCompilationJob
#[cfg(not(feature = "turbofan"))]
pub fn new_compilation_job(/* isolate, function, has_script, osr_offset */) -> ! {
  panic!("compiler::NewCompilationJob must not be called when Turbofan is disabled (v8_enable_turbofan = false)")
}

/// 原版 NewJSToWasmCompilationJob 桩
/// v8: src/compiler/turbofan-disabled.cc NewJSToWasmCompilationJob
#[cfg(not(feature = "turbofan"))]
pub fn new_js_to_wasm_compilation_job(/* isolate, sig */) -> ! {
  panic!("compiler::NewJSToWasmCompilationJob must not be called when Turbofan is disabled (v8_enable_turbofan = false)")
}
`;

const p = writeSrc("v8-compiler-passes", "turbofan_disabled.rs.skel", body);
console.log(`written ${p}\nfatal_stubs=${fatals.length}`);
