#!/usr/bin/env bun
// tool/src/wasm/wasm-module.cc.js
// 机械提取 v8/src/wasm/wasm-module.cc 的差集:
// 方法/自由函数定义台账 (NameMap 族 / LazilyGeneratedNames / WasmModule / 反射 API),
// UpdateComputedInformation 内存约束规则, NameMap 显式模板实例化。
// 头侧在 v8-wasm-core/src/wasm_module_consts.rs (wasm-module.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/wasm-module.cc");

// ---------- 锚断言 ----------
const anchors = [
  ["include 自头", src.includes('#include "src/wasm/wasm-module.h"')],
  ["UpdateComputedInformation", src.includes("void UpdateComputedInformation(WasmMemory* memory) {")],
  ["FinishInitialization", src.includes("void AdaptiveMap<Value>::FinishInitialization() {")],
  ["GetImports", src.includes("DirectHandle<JSArray> GetImports(Isolate* isolate,")],
  ["GetExports", src.includes("DirectHandle<JSArray> GetExports(Isolate* isolate,")],
  ["operator<<", src.includes("std::ostream& operator<<(std::ostream& os, const WasmFunctionName& name) {")],
];
for (const [n, ok] of anchors) {
  if (!ok) throw new Error(`锚点缺失: ${n}`);
}

// ---------- UpdateComputedInformation 真实规则锚 ----------
if (!/platform_max_pages =\s*memory->is_memory64\(\) \? wasm::max_mem64_pages\(\) : wasm::max_mem32_pages\(\);/.test(src)) {
  throw new Error("platform_max_pages 三目分派缺失");
}
if (!src.includes("memory->min_memory_size = static_cast<uintptr_t>(std::min<uint64_t>(")) {
  throw new Error("min_memory_size 公式缺失");
}
if (!src.includes("memory->max_memory_size = static_cast<uintptr_t>(std::min<uint64_t>(")) {
  throw new Error("max_memory_size 公式缺失");
}
if (!src.includes("kWasmPageSize;")) throw new Error("页转字节乘子缺失");
// bounds_checks 五路分派锚
if (!src.includes("memory->bounds_checks = kNoBoundsChecks;")) throw new Error("kNoBoundsChecks 分支缺失");
if (!src.includes("memory->bounds_checks = kExplicitBoundsChecks;")) throw new Error("kExplicitBoundsChecks 分支缺失");
if (!src.includes("memory->bounds_checks = kTrapHandler;")) throw new Error("kTrapHandler 分支缺失");
if (!src.includes("if (trap_handler::IsTrapHandlerEnabled()) {")) throw new Error("trap handler 启用分支缺失");
if (!src.includes("memory->is_memory64() && !v8_flags.wasm_memory64_trap_handling")) {
  throw new Error("memory64 无 trap handling 分支缺失");
}
if (!src.includes("if constexpr (kSystemPointerSize == 4) UNREACHABLE();")) {
  throw new Error("32 位 trap handler UNREACHABLE 缺失");
}

// ---------- 方法与自由函数台账 (行首非空白锚定, ctor/dtor 单独匹配) ----------
const methods = [];   // Class::method
let prevStatic = false;
for (const [i, line] of src.split("\n").entries()) {
  if (/^\/\/ static$/.test(line)) { prevStatic = true; continue; }
  const m = line.match(/^([A-Za-z_][\w:<>,&*]*?)[ *&]+([A-Z]\w*(?:<[^>]*>)?)::~?(\w+)\(/);
  const ctor = line.match(/^([A-Z]\w*)::~?(~?\w+)\(/);
  if (m) methods.push({ cls: m[2], name: m[3], line: i + 1, isStatic: prevStatic });
  else if (ctor && ctor[2].replace("~", "") === ctor[1]) {
    methods.push({ cls: ctor[1], name: ctor[2], line: i + 1, isStatic: prevStatic });
  }
  prevStatic = false;
}
const freeFns = [];
for (const [i, line] of src.split("\n").entries()) {
  const m = line.match(/^(?:DirectHandle<\w+>|std::ostream&|WasmName|int|void|bool|uint64_t|size_t|WireBytesRef) (\w+)\(/);
  if (m && !line.includes("::")) freeFns.push({ name: m[1], line: i + 1 });
}
if (methods.length < 15) throw new Error(`类方法台账 ${methods.length} 条, 期望 >= 15`);
if (freeFns.length < 15) throw new Error(`自由函数台账 ${freeFns.length} 条, 期望 >= 15`);

// ---------- NameMap 显式模板实例化 ----------
const instantiations = [...src.matchAll(/^template void (\w+)::(\w+)\(\);/gm)].map((m) => m[2]);
if (instantiations.length !== 2) throw new Error(`显式实例化 ${instantiations.length} 条, 期望 2`);

// ---------- 反射 API (WebAssembly.Module 反射) ----------
const reflectionApis = freeFns.map((f) => f.name).filter((n) => /^Get(Type|Imports|Exports|CustomSections)/.test(n));
if (reflectionApis.length !== 7) throw new Error(`反射 API ${reflectionApis.length} 条, 期望 7`);

console.log(
  `wasm-module.cc: 类方法=${methods.length} 自由函数=${freeFns.length} 反射API=${reflectionApis.length} 实例化=${instantiations.length}`,
);

// ---------- 生成 Rust ----------
let rust = `${banner(
  "src/wasm/wasm-module.cc.js",
  "src/wasm/wasm-module.cc",
  "方法台账 + 反射 API 台账 + 内存约束合并规则",
)}
//! 模块元数据层的方法台账与反射 API。

/// UpdateComputedInformation: 内存尺寸钳制与边界检查策略
/// v8: src/wasm/wasm-module.cc UpdateComputedInformation
pub mod computed_information {
    /// platform_max_pages = is_memory64 ? max_mem64_pages() : max_mem32_pages()
    pub const PLATFORM_MAX_PAGES_BY_MEMORY64: bool = true;
    /// min_memory_size = min(platform_max_pages, initial_pages) * kWasmPageSize
    pub const MIN_SIZE_IS_MIN_OF_PLATFORM_AND_INITIAL: bool = true;
    /// max_memory_size = min(platform_max_pages, maximum_pages) * kWasmPageSize
    pub const MAX_SIZE_IS_MIN_OF_PLATFORM_AND_MAXIMUM: bool = true;
    /// bounds_checks 五路分派:
    /// 1) !wasm_bounds_checks → kNoBoundsChecks
    /// 2) wasm_enforce_bounds_checks → kExplicitBoundsChecks
    /// 3) memory64 且未开 wasm_memory64_trap_handling → kExplicitBoundsChecks
    /// 4) trap handler 已启用 → kTrapHandler (32 位平台 UNREACHABLE)
    /// 5) 兜底 → kExplicitBoundsChecks
    pub const BOUNDS_CHECKS_DISPATCH: &[(&str, &str)] = &[
        ("!wasm_bounds_checks", "kNoBoundsChecks"),
        ("wasm_enforce_bounds_checks", "kExplicitBoundsChecks"),
        ("memory64 && !wasm_memory64_trap_handling", "kExplicitBoundsChecks"),
        ("trap_handler::IsTrapHandlerEnabled()", "kTrapHandler"),
        ("其它", "kExplicitBoundsChecks"),
    ];
}

/// NameMap / IndirectNameMap 的 FinishInitialization 显式实例化 (${instantiations.length} 条)
/// v8: src/wasm/wasm-module.cc AdaptiveMap<Value>::FinishInitialization
#[rustfmt::skip]
pub static NAMEMAP_INSTANTIATIONS: &[&str] = &[
`;
for (const inst of instantiations) {
  rust += `  "${inst}",\n`;
}
rust += `];

/// 类方法定义台账 (${methods.length} 条)
/// v8: src/wasm/wasm-module.cc
#[rustfmt::skip]
pub static METHODS: &[(&str, &str, u32)] = &[
`;
for (const m of methods) {
  rust += `  ("${m.cls}", "${m.name}", ${m.line}),\n`;
}
rust += `];

/// 自由函数台账 (${freeFns.length} 条; 含 WebAssembly.Module 反射 API ${reflectionApis.length} 条)
/// v8: src/wasm/wasm-module.cc
#[rustfmt::skip]
pub static FREE_FUNCTIONS: &[(&str, u32)] = &[
`;
for (const f of freeFns) {
  rust += `  ("${f.name}", ${f.line}),\n`;
}
rust += `];

// —— 提取不变式断言 ——
const _: () = {
    assert!(NAMEMAP_INSTANTIATIONS.len() == 2);
    assert!(METHODS.len() == ${methods.length});
    assert!(FREE_FUNCTIONS.len() == ${freeFns.length});
    assert!(computed_information::MIN_SIZE_IS_MIN_OF_PLATFORM_AND_INITIAL);
    assert!(computed_information::BOUNDS_CHECKS_DISPATCH.len() == 5);
};
`;

const p = writeSrc("v8-wasm-core", "wasm_module_cc.rs.skel", rust);
console.log(`written ${p}`);
