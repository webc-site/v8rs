#!/usr/bin/env bun
// tool/src/wasm/function-compiler.cc.js
// 机械提取 v8/src/wasm/function-compiler.cc 的差集:
// WasmCompilationUnit::ExecuteCompilation 的 ExecutionTier 分派 + Liftoff 回落 Turbofan 规则,
// 计数器/直方图名表, 巨型函数阈值, 以及方法定义台账。
// 头侧在 v8-wasm-core/src/function_compiler_consts.rs (function-compiler.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/function-compiler.cc");

// ---------- 锚断言 ----------
const anchors = [
  ["include 自头", src.includes('#include "src/wasm/function-compiler.h"')],
  ["ExecuteCompilation", src.includes("WasmCompilationResult WasmCompilationUnit::ExecuteCompilation(")],
  ["CompileWasmFunction", src.includes("void WasmCompilationUnit::CompileWasmFunction(NativeModule* native_module,")],
  ["Liftoff 分支", src.includes("case ExecutionTier::kLiftoff: {")],
  ["Turbofan 分支", src.includes("case ExecutionTier::kTurbofan: {")],
];
for (const [n, ok] of anchors) {
  if (!ok) throw new Error(`锚点缺失: ${n}`);
}

// ---------- tier 分派清单 ----------
const execBody = src.slice(
  src.indexOf("WasmCompilationResult WasmCompilationUnit::ExecuteCompilation("),
  src.indexOf("// static\nvoid WasmCompilationUnit::CompileWasmFunction"),
);
const tiers = [...execBody.matchAll(/case ExecutionTier::k(\w+):/g)].map((m) => m[1]);
if (!tiers.includes("None") || !tiers.includes("Liftoff") || !tiers.includes("Turbofan")) {
  throw new Error(`tier 分派异常: ${tiers}`);
}
if (!execBody.includes("case ExecutionTier::kInterpreter:")) throw new Error("kInterpreter (Drumbrake) 条件分支缺失");
if (!execBody.includes("result = ExecuteLiftoffCompilation(env, func_body, options);")) {
  throw new Error("Liftoff 执行调用缺失");
}
if (!execBody.includes("ExecuteTurboshaftWasmCompilation")) throw new Error("Turbofan 执行调用缺失");
if (!execBody.includes("[[fallthrough]];")) throw new Error("Liftoff 失败回落 Turbofan 缺失");
if (!execBody.includes("if (v8_flags.liftoff_only) break;")) throw new Error("liftoff_only 不回落缺失");

// ---------- 计数器/直方图名表 ----------
const counters = [...new Set([...src.matchAll(/Counters::(wasm_\w+)/g)].map((m) => m[1]))];
if (counters.length < 6) throw new Error(`计数器 ${counters.length} 条, 期望 >= 6`);
for (const need of ["wasm_generated_code_size", "wasm_reloc_size", "wasm_deopt_data_size"]) {
  if (!counters.includes(need)) throw new Error(`计数器缺失: ${need}`);
}

// ---------- 巨型函数阈值 ----------
const huge = execBody.match(/func_body\.end - func_body\.start >= (\d+) \* KB/);
if (!huge || huge[1] !== "100") throw new Error("巨型函数阈值 100*KB 缺失");

// ---------- 方法定义台账 (返回类型 + [ *&] 分隔保证类名完整; ctor/dtor 单独匹配) ----------
const methods = [];
let prevStatic = false;
for (const [i, line] of src.split("\n").entries()) {
  if (/^\/\/ static$/.test(line)) { prevStatic = true; continue; }
  const m = line.match(/^([A-Za-z_][\w:<>,&*]*?)[ *&]+([A-Z]\w*(?:<[^>]*>)?)::~?(\w+)\(/);
  const ctor = line.match(/^([A-Z]\w*)::(~?\w+)\(/);
  if (m) methods.push({ cls: m[2], name: m[3], line: i + 1, isStatic: prevStatic });
  else if (ctor && ctor[2].replace("~", "") === ctor[1]) {
    methods.push({ cls: ctor[1], name: ctor[2], line: i + 1, isStatic: prevStatic });
  }
  prevStatic = false;
}
if (methods.length < 7) throw new Error(`方法台账 ${methods.length} 条, 期望 >= 7`);
if (!methods.some((x) => x.cls === "WasmCompilationUnit" && x.name === "ExecuteCompilation")) {
  throw new Error("台账缺 WasmCompilationUnit::ExecuteCompilation");
}
if (!methods.some((x) => x.cls === "JSToWasmWrapperCompilationUnit" && x.name.startsWith("~"))) {
  throw new Error("台账缺 JSToWasmWrapperCompilationUnit 析构");
}

console.log(
  `function-compiler.cc: tier=${tiers.length} 计数器=${counters.length} 方法=${methods.length} 巨型阈值=${huge[1]}KB`,
);

// ---------- 生成 Rust ----------
let rust = `${banner(
  "src/wasm/function-compiler.cc.js",
  "src/wasm/function-compiler.cc",
  "tier 分派 + 计数器名表 + 方法台账",
)}
//! 编译单元执行分派与度量台账。

/// ExecuteCompilation 的 tier 分派 (${tiers.length} 路含条件编译项)
/// v8: src/wasm/function-compiler.cc WasmCompilationUnit::ExecuteCompilation
#[rustfmt::skip]
pub static TIER_DISPATCH: &[(&str, &str)] = &[
  ("None", "UNREACHABLE"),
  ("Interpreter", "UNREACHABLE (仅 V8_ENABLE_DRUMBRAKE 时出现在分派表)"),
  ("Liftoff", "tier_mask_for_testing 命中且非 must_use_liftoff 时跳过; 成功即止; 失败且非 liftoff_only 回落 Turbofan"),
  ("Turbofan", "ExecuteTurboshaftWasmCompilation; 结果 for_debugging 对齐请求值"),
];

/// Liftoff 前置开关规则
/// v8: src/wasm/function-compiler.cc WasmCompilationUnit::ExecuteCompilation
pub mod liftoff_gate {
    /// must_use_liftoff = liftoff_only || for_debugging != kNotForDebugging
    pub const MUST_USE_LIFTOFF_OR_DEBUGGING: bool = true;
    /// tiering_requested = declared_index < 32 && (wasm_tier_mask_for_testing & (1 << declared_index))
    pub const TIER_MASK_BIT_INDEX_LIMIT: u32 = 32;
    /// wasm_code_coverage 且非调试编译时, 提升为 kForDebugging
    pub const CODE_COVERAGE_FORCES_DEBUGGING: bool = true;
}

/// 巨型函数阈值: 函数体 >= ${huge[1]} * KB 时额外采样尺寸与耗时直方图
/// v8: src/wasm/function-compiler.cc WasmCompilationUnit::ExecuteCompilation
pub const HUGE_FUNCTION_BODY_KB: usize = ${huge[1]};

/// 计数器/直方图名表 (${counters.length} 条, .cc 中实际引用)
/// v8: src/wasm/function-compiler.cc
#[rustfmt::skip]
pub static COUNTERS: &[&str] = &[
`;
for (const c of counters) {
  rust += `  "${c}",\n`;
}
rust += `];

/// 方法定义台账 (${methods.length} 条, .cc 侧)
/// v8: src/wasm/function-compiler.cc
#[rustfmt::skip]
pub static METHODS: &[(&str, &str, u32)] = &[
`;
for (const m of methods) {
  rust += `  ("${m.cls}", "${m.name}", ${m.line}),\n`;
}
rust += `];

// —— 提取不变式断言 ——
const _: () = {
    assert!(TIER_DISPATCH.len() == 4);
    assert!(COUNTERS.len() == ${counters.length});
    assert!(METHODS.len() == ${methods.length});
    assert!(HUGE_FUNCTION_BODY_KB == 100);
};
`;

const p = writeSrc("v8-wasm-core", "function_compiler_cc.rs.skel", rust);
console.log(`written ${p}`);
