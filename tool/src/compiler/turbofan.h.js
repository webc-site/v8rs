#!/usr/bin/env bun
// tool/src/compiler/turbofan.h.js
// v8/src/compiler/turbofan.h + turbofan-disabled.cc + turbofan-enabled.cc
// 的公共契约面 (本脚本产物覆盖 turbofan.h; disabled/enabled 两 cc 由
// turbofan-disabled.cc.js / turbofan-enabled.cc.js 分别产出)
// → v8-compiler-passes/src/turbofan.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/turbofan.h");

const need = (cond, msg) => {
  if (!cond) throw new Error(`turbofan: ${msg}`);
};

for (const [s, name] of [
  ["enum class IsScriptAvailable {", "IsScriptAvailable 枚举"],
  ["kNo,", "kNo"],
  ["kYes,", "kYes"],
  ["V8_EXPORT_PRIVATE std::unique_ptr<TurbofanCompilationJob> NewCompilationJob(", "NewCompilationJob 声明"],
  ["BytecodeOffset osr_offset = BytecodeOffset::None()", "OSR 缺省"],
]) need(h.includes(s), `哨兵缺失 ${name}`);

// 枚举成员 (真解析)
const enumBody = h.slice(h.indexOf("enum class IsScriptAvailable {"));
const members = [...enumBody.slice(0, enumBody.indexOf("};")).matchAll(/k(\w+),/g)].map((m) => m[1]);
need(members.length === 2 && members[0] === "No" && members[1] === "Yes", `成员 ${members}`);

// 签名真解析 (默认参)
const sig = h.match(/NewCompilationJob\(\s*Isolate\* isolate, Handle<JSFunction> function, IsScriptAvailable has_script,\s*BytecodeOffset osr_offset = ([^)]+)\)/);
need(sig, "NewCompilationJob 签名解析失败");

const body = `${banner("src/compiler/turbofan.h.js", "src/compiler/turbofan.h", "TurboFan 对外最小接口 (不含 compiler 内部头)")}
//! turbofan — 编排入口的稳定契约
//!
//! NewCompilationJob(isolate, function, has_script, osr_offset =
//! BytecodeOffset::None()); 实现按构建开关二选一:
//! turbofan-enabled.cc → Pipeline::NewCompilationJob;
//! turbofan-disabled.cc → FATAL 桩 (见 turbofan_disabled.rs.skel /
//! turbofan_enabled.rs.skel)。

/// 原版 IsScriptAvailable — JSFunction 是否带 Script
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsScriptAvailable {
  /// kNo
  No,
  /// kYes
  Yes,
}

/// 原版 NewCompilationJob — TurbofanCompilationJob 工厂
/// v8: src/compiler/turbofan-enabled.cc NewCompilationJob (TF 开)
/// v8: src/compiler/turbofan-disabled.cc NewCompilationJob (TF 关 → FATAL)
pub fn new_compilation_job(isolate: &Isolate, function: &JSFunction, has_script: IsScriptAvailable, osr_offset: BytecodeOffset) -> Box<dyn TurbofanCompilationJob> {
  todo!("v8: src/compiler/turbofan-enabled.cc NewCompilationJob (cfg(feature = \"turbofan\") 二选一)")
}
`;

const p = writeSrc("v8-compiler-passes", "turbofan.rs.skel", body);
console.log(`written ${p}\nis_script_available=${members.length} 默认参 osr_offset=${sig[1].trim()}`);
