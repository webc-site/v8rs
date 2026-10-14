#!/usr/bin/env bun
// tool/src/wasm/wasm-js.cc.js (族合一: wasm-js.cc + wasm-js.h)
// 大 .cc 规则面: WebAssembly JS API 安装表 (命名空间方法/各 proto 方法/getter/错误构造器)、
// 对象名常量、JS Tag 签名、resolver 全局句柄; 另出 API 函数签名骨架。
// 头侧骨架在 v8-wasm-js/src/wasm_js.rs.skel (wasm-js.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/wasm/wasm-js.cc");

// ---------- 1. 对象名常量 (5 个) ----------
const names = [...cc.matchAll(/constexpr const char\* (kName_\w+) = "([^"]+)";/g)].map((m) => [m[1], m[2]]);
if (names.length !== 5) throw new Error(`对象名常量 ${names.length} != 5`);
if (names[0].join("=") !== "kName_WasmGlobalObject=WebAssembly.Global") throw new Error("对象名序异常");

// ---------- 2. JS Tag 签名: 0 返回 + 1 个 externref 参数 ----------
if (!cc.includes("constexpr wasm::ValueType kWasmExceptionTagParams[] = {\n    wasm::kWasmExternRef,\n};")) {
  throw new Error("JS Tag 参数表缺失");
}
if (!cc.includes("constexpr wasm::FunctionSig kWasmExceptionTagSignature{\n    0, arraysize(kWasmExceptionTagParams), kWasmExceptionTagParams};")) {
  throw new Error("JS Tag 签名缺失");
}
const initialIdx = cc.match(/static constexpr wasm::CanonicalTypeIndex kInitialCanonicalTypeIndex\{(\d+)\}/);
if (!initialIdx || initialIdx[1] !== "0") throw new Error("JS Tag 初始 canonical index != 0");

// ---------- 3. WebAssembly 命名空间方法 (3 个) ----------
for (const [name, func, len] of [
  ["compile", "WebAssemblyCompile", "1"],
  ["validate", "WebAssemblyValidate", "1"],
  ["instantiate", "WebAssemblyInstantiate", "1"],
]) {
  if (!new RegExp(`InstallFunc\\(isolate, webassembly, "${name}",\\s*wasm::${func}, ${len}\\);`).test(cc)) {
    throw new Error(`命名空间方法 ${name} 缺失`);
  }
}

// ---------- 4. 各 proto 方法/getter 安装表 ----------
const installFuncs = [...cc.matchAll(/InstallFunc\(isolate, (\w+), "([^"]+)",\s*(?:wasm::)?(\w+)/g)].map((m) => [m[1], m[2], m[3]]);
if (installFuncs.length !== 18) throw new Error(`InstallFunc ${installFuncs.length} != 18`);
const installGetters = [...cc.matchAll(/InstallGetter\(isolate, (\w+), "([^"]+)",\s*wasm::(\w+)/g)].map((m) => [m[1], m[2], m[3]]);
if (installGetters.length !== 4) throw new Error(`InstallGetter ${installGetters.length} != 4`);
// 锚: table 四件套, memory 四件套
const tableMethods = installFuncs.filter((f) => f[0] === "table_proto").map((f) => f[1]);
if (tableMethods.join(",") !== "grow,set,get") throw new Error(`table 方法序异常: ${tableMethods.join(",")}`);
const memoryMethods = installFuncs.filter((f) => f[0] === "memory_proto").map((f) => f[1]);
if (memoryMethods.join(",") !== "grow,toFixedLengthBuffer,toResizableBuffer") {
  throw new Error(`memory 方法序异常: ${memoryMethods.join(",")}`);
}

// ---------- 5. Global value getter+setter 成对 ----------
if (!cc.includes("InstallGetterSetter(isolate, global_proto, \"value\",\n                        wasm::WebAssemblyGlobalGetValue,\n                        wasm::WebAssemblyGlobalSetValue);")) {
  throw new Error("Global value 存取对缺失");
}

// ---------- 6. 错误构造器 (3 个) ----------
const errors = [...cc.matchAll(/InstallError\(isolate, webassembly, f->(\w+)\(\),\s*Context::(\w+)\);/g)].map((m) => [m[1], m[2]]);
if (errors.length !== 3) throw new Error(`错误构造器 ${errors.length} != 3`);

// ---------- 7. 只读属性: DONT_ENUM | READ_ONLY ----------
if (!cc.includes("static_cast<PropertyAttributes>(DONT_ENUM | READ_ONLY)")) {
  throw new Error("只读属性组合缺失");
}

// ---------- 8. Exception 双私有字段 (tag/values symbol) ----------
if (!cc.includes("f->wasm_exception_tag_symbol()")) throw new Error("tag symbol 字段缺失");
if (!cc.includes("f->wasm_exception_values_symbol()")) throw new Error("values symbol 字段缺失");
if (!cc.includes("Map::EnsureDescriptorSlack(isolate, initial_map, 2);")) {
  throw new Error("描述符预留 2 缺失");
}

// ---------- 9. resolver 全局句柄常量 (5 类 resolver) ----------
const handleDecls = (cc.match(/static constexpr char kGlobal\w+Handle\[\]/g) || []).length;
if (handleDecls !== 6) throw new Error(`resolver 句柄声明 ${handleDecls} != 6`);

// ---------- 10. WebAssembly* API 函数台账 ----------
const apiFuncs = [...cc.matchAll(/(void WebAssembly\w+|WebAssembly\w+)\(\s*const v8::FunctionCallbackInfo<v8::Value>&/g)];
if (apiFuncs.length !== 35) throw new Error(`API 回调 ${apiFuncs.length} != 35`);

// ---------- 11. Module 静态方法 (js_source_phase_imports 分支外 3 个) ----------
const moduleStatics = installFuncs.filter((f) => f[0] === "module_constructor").map((f) => f[1]);
if (moduleStatics.join(",") !== "imports,exports,customSections") {
  throw new Error(`Module 静态方法序异常: ${moduleStatics.join(",")}`);
}

console.log(`wasm-js: 对象名=${names.length} InstallFunc=${installFuncs.length} InstallGetter=${installGetters.length} 错误构造器=${errors.length} resolver句柄=${handleDecls}`);

// ---------- 生成 Rust ----------
let apiTable = "";
for (const [target, prop, func] of [...installFuncs, ...installGetters]) {
  apiTable += `    ("${target}", "${prop}", "${func}"),\n`;
}

const rust = `${banner(
  "src/wasm/wasm-js.cc.js",
  "src/wasm/wasm-js.{h,cc}",
  "WebAssembly JS API 安装表与 JS Tag 签名 + API 回调签名骨架 (头侧在 wasm_js.rs.skel)",
)}
//! WebAssembly JS API 绑定规则面。

/// 内建对象名常量
/// v8: src/wasm/wasm-js.cc kName_Wasm*
#[rustfmt::skip]
pub static OBJECT_NAMES: &[(&str, &str)] = &[
${names.map(([k, v]) => `    ("${k}", "${v}"),\n`).join("")}];

/// WebAssembly 命名空间方法 (compile/validate/instantiate 恒装; streaming 对与 promising 另装)
/// v8: src/wasm/wasm-js.cc WasmJs::Install / InstallConditional
#[rustfmt::skip]
pub static NAMESPACE_METHODS: &[(&str, &str)] = &[
    ("compile", "1"),
    ("validate", "1"),
    ("instantiate", "1"),
    ("compileStreaming", "1 (流式)"),
    ("instantiateStreaming", "1 (流式)"),
    ("promising", "1 (WasmFX)"),
];

/// proto 方法/getter 安装表 (目标, 属性名, 回调)
/// v8: src/wasm/wasm-js.cc WasmJs::Install / InstallModule
#[rustfmt::skip]
pub static API_INSTALLS: &[(&str, &str, &str)] = &[
${apiTable}];

/// Global.value 为 getter+setter 成对安装
/// v8: src/wasm/wasm-js.cc WasmJs::Install
pub const GLOBAL_VALUE_ACCESSOR_PAIR: (&str, &str) = ("WebAssemblyGlobalGetValue", "WebAssemblyGlobalSetValue");

/// 错误构造器 (CompileError/LinkError/RuntimeError)
/// v8: src/wasm/wasm-js.cc WasmJs::Install
#[rustfmt::skip]
pub static ERROR_CONSTRUCTORS: &[(&str, &str)] = &[
${errors.map(([k, v]) => `    ("${k.replace("_string", "")}", "${v}"),\n`).join("")}];

/// JS Tag 签名: 0 返回 + 1 个 externref 参数; 安装时 canonical index 从 0 起
/// v8: src/wasm/wasm-js.cc kWasmExceptionTagSignature / kInitialCanonicalTypeIndex
pub const JS_TAG_SIGNATURE: &str = "() -> (externref)";
pub const JS_TAG_INITIAL_CANONICAL_INDEX: u32 = ${initialIdx[1]};

/// 只读属性组合: DONT_ENUM | READ_ONLY
/// v8: src/wasm/wasm-js.cc ro_attributes
pub const RO_ATTRIBUTES: &str = "DONT_ENUM | READ_ONLY";

/// WebAssembly.Exception 私有字段: tag/values 两个 symbol (描述符预留 2)
/// v8: src/wasm/wasm-js.cc WasmJs::Install
#[rustfmt::skip]
pub static EXCEPTION_PRIVATE_FIELDS: &[&str] = &["wasm_exception_tag_symbol", "wasm_exception_values_symbol"];

/// 异步 resolver 的全局句柄常量声明 (6 处: Promise/Module/Imports 跨 5 类 resolver)
/// v8: src/wasm/wasm-js.cc AsyncCompilationResolver 系
pub const RESOLVER_GLOBAL_HANDLES: usize = ${handleDecls};

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(OBJECT_NAMES.len(), 5);
    assert_eq!(NAMESPACE_METHODS.len(), 6);
    assert_eq!(API_INSTALLS.len(), 22);
    assert_eq!(ERROR_CONSTRUCTORS.len(), 3);
    assert_eq!(EXCEPTION_PRIVATE_FIELDS.len(), 2);
    assert_eq!(JS_TAG_INITIAL_CANONICAL_INDEX, 0);
    assert_eq!(RESOLVER_GLOBAL_HANDLES, 6);
    assert!(API_INSTALLS.contains(&("table_proto", "grow", "WebAssemblyTableGrow")));
    assert!(API_INSTALLS.contains(&("memory_proto", "buffer", "WebAssemblyMemoryGetBuffer")));
};

// —— API 回调签名骨架 (就地修订 todo) ——
/// v8: src/wasm/wasm-js.cc WebAssembly* 回调族 (${apiFuncs.length} 个 FunctionCallback)
pub fn webassembly_api_callbacks() {
    todo!("就地修订: ${apiFuncs.length} 个 WebAssembly* API 回调")
}
`;

const p = writeSrc("v8-wasm-js", "wasm_js_cc.rs", rust);
console.log(`written ${p}`);
