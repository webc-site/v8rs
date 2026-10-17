#!/usr/bin/env bun
// tool/src/wasm/compilation-environment-inl.js
// 机械提取 v8/src/wasm/compilation-environment-inl.h 的差集:
// CompilationEnv::ForModule 工厂从 NativeModule 取四项构造 CompilationEnv。
// 头侧结构在 v8-wasm-core/src/compilation_environment_consts.rs。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/compilation-environment-inl.h");

// ---------- 锚断言 ----------
const anchors = [
  ["include 自头", src.includes('#include "src/wasm/compilation-environment.h"')],
  ["include code-manager", src.includes('#include "src/wasm/wasm-code-manager.h"')],
  ["ForModule 定义", src.includes("inline CompilationEnv CompilationEnv::ForModule(")],
  ["形参", src.includes("const NativeModule* native_module) {")],
];
for (const [n, ok] of anchors) {
  if (!ok) throw new Error(`锚点缺失: ${n}`);
}

// ---------- 提取构造实参 (native_module 访问器清单) ----------
const bodyRegion = src.slice(
  src.indexOf("inline CompilationEnv CompilationEnv::ForModule"),
);
const ctorArgs = [...bodyRegion.matchAll(/native_module->(\w+)\(\)/g)].map((m) => m[1]);
if (ctorArgs.length !== 4) throw new Error(`构造实参 ${ctorArgs.length} 条, 期望 4`);
const expected = ["module", "enabled_features", "fast_api_data", "coverage_data"];
for (let i = 0; i < 4; i++) {
  if (ctorArgs[i] !== expected[i]) {
    throw new Error(`构造实参漂移: ${ctorArgs}, 期望 ${expected}`);
  }
}

console.log(`compilation-environment-inl: ForModule 实参=${ctorArgs.length}`);

// ---------- 生成 Rust ----------
const rust = `${banner(
  "src/wasm/compilation-environment-inl.js",
  "src/wasm/compilation-environment-inl.h",
  "CompilationEnv::ForModule 工厂实参清单",
)}
//! 编译环境工厂: 从 NativeModule 派生 CompilationEnv 的四项来源。

/// ForModule 实参台账 (${ctorArgs.length} 项, 顺序即构造序)
/// v8: src/wasm/compilation-environment-inl.h CompilationEnv::ForModule
#[rustfmt::skip]
pub static FOR_MODULE_FIELDS: &[(&str, &str)] = &[
  ("module", "native_module->module() — WasmModule 视图"),
  ("enabled_features", "native_module->enabled_features() — 启用的 wasm 特性"),
  ("fast_api_data", "native_module->fast_api_data() — fast API 调用数据"),
  ("coverage_data", "native_module->coverage_data() — 覆盖率数据"),
];

/// CompilationEnv::ForModule 骨架
/// v8: src/wasm/compilation-environment-inl.h CompilationEnv::ForModule
pub fn for_module(_native_module: &()) -> CompilationEnvView {
    CompilationEnvView {
        module: Source::NativeModuleGetter("module"),
        enabled_features: Source::NativeModuleGetter("enabled_features"),
        fast_api_data: Source::NativeModuleGetter("fast_api_data"),
        coverage_data: Source::NativeModuleGetter("coverage_data"),
    }
}

/// 工厂产物台账 (骨架; 真实实现待 Developer 修订)
pub struct CompilationEnvView {
    pub module: Source,
    pub enabled_features: Source,
    pub fast_api_data: Source,
    pub coverage_data: Source,
}

/// CompilationEnv 各字段来源
pub enum Source {
    /// NativeModule 上的同名 getter
    NativeModuleGetter(&'static str),
}

// —— 提取不变式断言 ——
const _: () = {
    assert!(FOR_MODULE_FIELDS.len() == 4);
    assert!(matches!(Source::NativeModuleGetter("module"), Source::NativeModuleGetter("module")));
};
`;

const p = writeSrc("v8-wasm-core", "compilation_environment_inl.rs.skel", rust);
console.log(`written ${p}`);
