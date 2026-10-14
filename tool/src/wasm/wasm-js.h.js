#!/usr/bin/env bun
// tool/src/wasm/wasm-js.h.js
// 机械提取 v8/src/wasm/wasm-js.{h,cc} 的 JS API 函数清单与类型名字符串

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-js.h");
const ccSrc = readV8("src/wasm/wasm-js.cc");

// 1. h: X-macro 展开的外部引用函数表
const listMatch = hSrc.match(
  /#define WASM_JS_EXTERNAL_REFERENCE_LIST\(V\) \\\n([\s\S]*?)\n\n#define DECL_WASM_JS_EXTERNAL_REFERENCE/,
);
if (!listMatch) throw new Error("未找到 WASM_JS_EXTERNAL_REFERENCE_LIST");
const fnList = [...listMatch[1].matchAll(/V\((\w+)\)/g)].map((m) => m[1]);
if (fnList.length !== 32) throw new Error(`外部引用函数 ${fnList.length} != 32`);

// 2. h: WasmJs 静态安装面
for (const api of [
  "static void PrepareForSnapshot(Isolate* isolate);",
  "static void Install(Isolate* isolate);",
  "static void InstallConditionalFeatures(",
  "static void InstallModule(",
  "static void InstallMemoryControl(",
  "static bool InstallJSPromiseIntegration(",
]) {
  if (!hSrc.includes(api)) throw new Error(`WasmJs API 缺失: ${api}`);
}

// 3. cc: 顶层 JS 属性名
if (!ccSrc.includes('"WebAssembly"')) throw new Error("未找到 WebAssembly 对象名");
for (const ctor of ['"Module"', '"Instance"', '"Memory"', '"Table"', '"Global"', '"Tag"', '"Exception"']) {
  if (!ccSrc.includes(ctor)) throw new Error(`构造器名缺失: ${ctor}`);
}

// 4. cc: 值类型字符串 (WebAssembly.Module 的类型名)
const typeNames = ["anyfunc", "anyref", "arrayref", "eqref", "exnref", "externref", "stringref", "structref"];
for (const t of typeNames) {
  if (!ccSrc.includes(`"${t}"`)) throw new Error(`类型名缺失: ${t}`);
}

// 5. cc: 方法名 (属性族)
for (const m of ['"compile"', '"compileStreaming"', '"instantiate"', '"instantiateStreaming"', '"validate"', '"imports"', '"exports"', '"customSections"']) {
  if (!ccSrc.includes(m)) throw new Error(`方法名缺失: ${m}`);
}

// 6. cc: JSPI / 栈切换面
for (const j of ['"Suspending"', '"promising"', '"Suspender"', '"traceStack"']) {
  if (!ccSrc.includes(j)) throw new Error(`JSPI 名缺失: ${j}`);
}

// 7. cc: 流式编译入口
if (!ccSrc.includes("wasm::StartStreamingForTesting") && !hSrc.includes("StartStreamingForTesting")) {
  throw new Error("未找到流式测试入口");
}

console.log(`wasm-js: 外部引用函数=${fnList.length} 类型名=${typeNames.length}`);

// 8. 生成 Rust
let rust = banner(
  "src/wasm/wasm-js.h.js",
  "src/wasm/wasm-js.{h,cc}",
  "WebAssembly JS API 函数表与安装面",
);

rust += `
//! JS 侧 WebAssembly 对象的构建与回调表。
/// v8: src/wasm/wasm-js.h WasmJs

/// 全部 JS 回调 (原版 X-macro WASM_JS_EXTERNAL_REFERENCE_LIST 展开, 32 项)
/// v8: src/wasm/wasm-js.h WASM_JS_EXTERNAL_REFERENCE_LIST
pub const WASM_JS_EXTERNAL_REFERENCES: &[&str] = &[
${fnList.map((f) => `    "${f}",`).join("\n")}
];

/// 构造器对象名 (挂在 WebAssembly 上)
/// v8: src/wasm/wasm-js.cc Install*
pub const CONSTRUCTOR_NAMES: &[&str] = &[
    "Module", "Instance", "Memory", "Table", "Global", "Tag", "Exception",
    "MemoryMapDescriptor", "Suspending", "Suspender", "SuspendError",
];

/// 静态方法名
/// v8: src/wasm/wasm-js.cc
pub const STATIC_METHOD_NAMES: &[&str] = &[
    "compile", "compileStreaming", "instantiate", "instantiateStreaming", "validate",
];

/// 实例访问器名 (getter/setter 共用键)
/// v8: src/wasm/wasm-js.cc
pub const ACCESSOR_NAMES: &[&str] = &[
    "imports", "exports", "customSections", "buffer", "length", "value", "valueOf",
    "get", "set", "grow", "getArg", "is", "stack", "map", "unmap", "toFixedLengthBuffer",
    "toResizableBuffer",
];

/// 值类型名字符串 (toString 输出/parse 输入)
/// v8: src/wasm/wasm-js.cc 类型名表
pub const VALUE_TYPE_NAMES: &[&str] = &[
${typeNames.map((t) => `    "${t}",`).join("\n")}
];

/// 安装阶段
/// v8: src/wasm/wasm-js.h WasmJs::PrepareForSnapshot / Install / InstallConditionalFeatures
pub const INSTALL_PHASES: &[&str] = &["PrepareForSnapshot", "Install", "InstallConditionalFeatures"];

/// 条件安装 (flag 门控): Module/MemoryControl/JSPromiseIntegration
/// v8: src/wasm/wasm-js.h WasmJs 私有 Install*
pub const CONDITIONAL_INSTALLERS: &[&str] = &["InstallModule", "InstallMemoryControl", "InstallJSPromiseIntegration"];

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(WASM_JS_EXTERNAL_REFERENCES.len(), 32);
    assert_eq!(VALUE_TYPE_NAMES.len(), 8);
    assert_eq!(INSTALL_PHASES.len(), 3);
    assert_eq!(CONDITIONAL_INSTALLERS.len(), 3);
};
`;

const p = writeSrc("v8-wasm-js", "wasm_js.rs.skel", rust);
console.log(`written ${p}`);
