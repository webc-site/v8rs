#!/usr/bin/env bun
// tool/src/compiler/turbofan-enabled.cc.js
// v8/src/compiler/turbofan-enabled.cc
// → v8-compiler-passes/src/turbofan_enabled.rs.skel
// TF 开启实现: 解析到 Pipeline 的转发契约。
import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/compiler/turbofan-enabled.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`turbofan-enabled: ${msg}`);
};

for (const [s, name] of [
  ["std::unique_ptr<TurbofanCompilationJob> NewCompilationJob(", "NewCompilationJob 实现"],
  ["return Pipeline::NewCompilationJob(isolate, function, CodeKind::TURBOFAN_JS,", "Pipeline 转发"],
  ["has_script == IsScriptAvailable::kYes", "kYes 映射"],
  ["osr_offset);", "osr_offset 直传"],
]) need(cc.includes(s), `哨兵缺失 ${name}`);

// 转发参数四元组 (真解析)
const fwd = cc.match(/Pipeline::NewCompilationJob\(([^;]+)\);/);
need(fwd, "转发体解析失败");
const args = fwd[1].replace(/\s+/g, " ").trim().split(",").map((a) => a.trim());
need(args.length === 5 && args[2] === "CodeKind::TURBOFAN_JS", `转发参 ${args}`);

const body = `${banner("src/compiler/turbofan-enabled.cc.js", "src/compiler/turbofan-enabled.cc", "v8_enable_turbofan=true 时的实现 (向 Pipeline 转发)")}
//! turbofan-enabled — TF 开启构建的实现面
//!
//! 单入口: NewCompilationJob → Pipeline::NewCompilationJob(
//! ${args.join(", ")})。
//! has_script: IsScriptAvailable::kYes → bool true 直转。

use crate::pipeline::Pipeline;
use crate::turbofan::IsScriptAvailable;

/// 原版 NewCompilationJob — 转发 Pipeline (CodeKind::TURBOFAN_JS)
/// v8: src/compiler/turbofan-enabled.cc NewCompilationJob
#[cfg(feature = "turbofan")]
pub fn new_compilation_job(isolate: &Isolate, function: &JSFunction, has_script: IsScriptAvailable, osr_offset: BytecodeOffset) -> Box<dyn TurbofanCompilationJob> {
  Pipeline::new_compilation_job(isolate, function, CodeKind::TURBOFAN_JS, has_script == IsScriptAvailable::Yes, osr_offset)
}
`;

const p = writeSrc("v8-compiler-passes", "turbofan_enabled.rs.skel", body);
console.log(`written ${p}\n转发参=${args.length} (${args.join(", ")})`);
