#!/usr/bin/env bun
// tool/src/wasm/module-instantiate.h.js
// 机械提取 v8/src/wasm/module-instantiate.{h,cc} 的导入调用分派枚举与实例化阶段

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/module-instantiate.h");
const ccSrc = readV8("src/wasm/module-instantiate.cc");

// 1. h: ImportCallKind 枚举 (7 值按声明序)
const ickMatch = hSrc.match(/enum class ImportCallKind : uint8_t \{([\s\S]*?)\};/);
if (!ickMatch) throw new Error("未找到 ImportCallKind");
const ickEntries = [...ickMatch[1].replace(/\/\/[^\n]*/g, "").matchAll(/\b(k\w+)/g)].map((m) => m[1]);
if (ickEntries.join(",") !== "kLinkError,kRuntimeTypeError,kWasmToCapi,kWasmToJSFastApi,kWasmToWasm,kJSFunction,kUseCallBuiltin") {
  throw new Error(`ImportCallKind 异常: ${ickEntries.join(",")}`);
}

// 2. h: Suspend / Promise / PrecreateExternal
if (!hSrc.includes("enum Suspend : int { kSuspend, kNoSuspend };")) {
  throw new Error("未找到 Suspend");
}
if (!hSrc.includes("enum Promise : int { kPromise, kNoPromise, kStressSwitch };")) {
  throw new Error("未找到 Promise");
}
if (!hSrc.includes("enum PrecreateExternal : bool {\n  kOnlyInternalFunction = false,\n  kPrecreateExternal = true,\n}")) {
  throw new Error("未找到 PrecreateExternal");
}

// 3. h: 默认 kind
if (!hSrc.includes("constexpr ImportCallKind kDefaultImportCallKind = ImportCallKind::kJSFunction;")) {
  throw new Error("未找到 kDefaultImportCallKind");
}

// 4. h: WrapperCompilationInfo 字段
if (!hSrc.includes("struct WrapperCompilationInfo {")) throw new Error("未找到 WrapperCompilationInfo");
for (const f of ["CodeKind code_kind;", "int expected_arity = 0;", "wasm::Suspend suspend = kNoSuspend;"]) {
  if (!hSrc.includes(f)) throw new Error(`WrapperCompilationInfo 字段缺失: ${f}`);
}

// 5. cc: 实例化阶段 (两段式构建)
for (const phase of [
  "Maybe<bool> InstanceBuilder::Build_Phase1(",
  "void InstanceBuilder::Build_Phase1_Infallible()",
  "Maybe<bool> InstanceBuilder::Build_Phase2()",
  "bool InstanceBuilder::ExecuteStartFunction()",
]) {
  if (!ccSrc.includes(phase)) throw new Error(`阶段缺失: ${phase}`);
}

// 6. cc: 导入处理五个面
const imported = [...ccSrc.matchAll(/bool InstanceBuilder::(ProcessImported\w+)\(/g)].map((m) => m[1]);
const uniq = [...new Set(imported)];
if (uniq.join(",") !== "ProcessImportedFunction,ProcessImportedTable,ProcessImportedWasmGlobalObject,ProcessImportedGlobal,ProcessImportedMemories") {
  throw new Error(`导入处理面异常: ${uniq.join(",")}`);
}

// 7. cc: DataView fast-path 签名探测
if (!ccSrc.includes("bool IsDataViewGetterSig(const wasm::CanonicalSig* sig,")) {
  throw new Error("未找到 IsDataViewGetterSig");
}
if (!ccSrc.includes("bool IsDataViewSetterSig(const wasm::CanonicalSig* sig,")) {
  throw new Error("未找到 IsDataViewSetterSig");
}

// 8. cc: well-known import → builtin 名
if (!ccSrc.includes("std::tuple<const char*, Builtin, int> NameBuiltinLength(WellKnownImport wki)")) {
  throw new Error("未找到 NameBuiltinLength");
}

console.log(
  `module-instantiate: ImportCallKind=${ickEntries.length} 导入面=${uniq.length} 阶段=4`,
);

// 9. 生成 Rust
let rust = banner(
  "src/wasm/module-instantiate.h.js",
  "src/wasm/module-instantiate.{h,cc}",
  "实例化导入分派枚举与 InstanceBuilder 阶段面",
);

rust += `
//! 导入调用解析与实例化构建。
/// v8: src/wasm/module-instantiate.h ResolvedWasmImport / InstanceBuilder

/// 导入调用分派 (声明序)
/// v8: src/wasm/module-instantiate.h enum class ImportCallKind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ImportCallKind {
    /// v8: kLinkError — 静态 Wasm->Wasm 类型错误
    LinkError = 0,
    /// v8: kRuntimeTypeError — 运行时 Wasm->JS 类型错误
    RuntimeTypeError = 1,
    /// v8: kWasmToCapi — 快速 Wasm->C-API 调用
    WasmToCapi = 2,
    /// v8: kWasmToJSFastApi — 快速 Wasm->JS Fast API C 调用
    WasmToJSFastApi = 3,
    /// v8: kWasmToWasm — 快速 Wasm->Wasm 调用
    WasmToWasm = 4,
    /// v8: kJSFunction — 快速 Wasm->JS 调用
    JSFunction = 5,
    /// v8: kUseCallBuiltin — 其余走 call builtin
    UseCallBuiltin = 6,
}

/// v8: kDefaultImportCallKind
pub const DEFAULT_IMPORT_CALL_KIND: ImportCallKind = ImportCallKind::JSFunction;

/// 挂起语义
/// v8: src/wasm/module-instantiate.h enum Suspend
pub const SUSPEND: i32 = 0;
pub const NO_SUSPEND: i32 = 1;

/// JS Promise integration 形态 (kStressSwitch 仅测试: 换栈不带 JSPI 语义)
/// v8: src/wasm/module-instantiate.h enum Promise
pub const PROMISE_PROMISE: i32 = 0;
pub const PROMISE_NO_PROMISE: i32 = 1;
pub const PROMISE_STRESS_SWITCH: i32 = 2;

/// 元素段初始化是否预创建外部函数
/// v8: src/wasm/module-instantiate.h enum PrecreateExternal
pub const ONLY_INTERNAL_FUNCTION: bool = false;
pub const PRECREATE_EXTERNAL: bool = true;

/// 实例化阶段序 (两段式 + 不可失败段 + start 函数)
/// v8: src/wasm/module-instantiate.cc InstanceBuilder::Build_Phase1/1_Infallible/2/ExecuteStartFunction
pub const BUILD_PHASES: &[&str] = &["Build_Phase1", "Build_Phase1_Infallible", "Build_Phase2", "ExecuteStartFunction"];

/// 导入处理五个面
/// v8: src/wasm/module-instantiate.cc InstanceBuilder::ProcessImported*
pub const IMPORT_PROCESSORS: &[&str] = &[
${uniq.map((u) => `    "${u}",`).join("\n")}
];

/// wrapper 编译信息 (图构建输入)
/// v8: src/wasm/module-instantiate.h struct WrapperCompilationInfo
pub struct WrapperCompilationInfo {
    /// v8: CodeKind code_kind
    pub code_kind: CodeKind,
    /// v8: wasm::ImportCallKind import_kind (仅 wasm-js wrapper 用)
    pub import_kind: ImportCallKind,
    /// v8: int expected_arity
    pub expected_arity: i32,
    /// v8: wasm::Suspend suspend
    pub suspend: i32,
}

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(ImportCallKind::LinkError as u8, 0);
    assert_eq!(ImportCallKind::UseCallBuiltin as u8, 6);
    assert_eq!(BUILD_PHASES.len(), 4);
    assert_eq!(IMPORT_PROCESSORS.len(), 5);
    assert_eq!(DEFAULT_IMPORT_CALL_KIND, ImportCallKind::JSFunction);
};
`;

const p = writeSrc("v8-wasm-core", "module_instantiate.rs.skel", rust);
console.log(`written ${p}`);
