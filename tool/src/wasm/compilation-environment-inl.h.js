#!/usr/bin/env bun
// tool/src/wasm/compilation-environment-inl.h.js
// 机械提取 v8/src/wasm/compilation-environment-inl.h 的 CompilationEnv::ForModule 字段表。
// 结构常量在 v8-wasm-core/src/compilation_environment_consts.rs (compilation-environment.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/compilation-environment-inl.h");

// ---------- 1. ForModule 四字段转发 ----------
if (!src.includes("return CompilationEnv(")) throw new Error("ForModule 构造缺失");
const fields = [...src.matchAll(/native_module->(\w+\(\))/g)].map((m) => m[1]);
if (fields.join(",") !== "module(),enabled_features(),fast_api_data(),coverage_data()") {
  throw new Error(`ForModule 字段序异常: ${fields.join(",")}`);
}

// ---------- 2. inline 语义 ----------
if (!/^inline CompilationEnv CompilationEnv::ForModule/m.test(src)) {
  throw new Error("inline 声明缺失");
}

// ---------- 3. 头包含顺序约束 ----------
if (!src.includes("// Include the non-inl header before the rest of the headers.")) {
  throw new Error("包含顺序注释缺失");
}

console.log(`compilation-environment-inl: 字段=${fields.length}`);

// ---------- 生成 Rust ----------
const rust = `${banner(
  "src/wasm/compilation-environment-inl.h.js",
  "src/wasm/compilation-environment-inl.h",
  "CompilationEnv 从 NativeModule 的构造字段表",
)}
//! 编译环境构造规则。

/// ForModule 从 NativeModule 取的四字段 (按构造序)
/// v8: src/wasm/compilation-environment-inl.h CompilationEnv::ForModule
#[rustfmt::skip]
pub static FOR_MODULE_FIELDS: &[&str] = &[
    "module()",
    "enabled_features()",
    "fast_api_data()",
    "coverage_data()",
];

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(FOR_MODULE_FIELDS.len(), 4);
    assert_eq!(FOR_MODULE_FIELDS[0], "module()");
    assert_eq!(FOR_MODULE_FIELDS[3], "coverage_data()");
};
`;

const p = writeSrc("v8-wasm-core", "compilation_environment_inl.rs", rust);
console.log(`written ${p}`);
