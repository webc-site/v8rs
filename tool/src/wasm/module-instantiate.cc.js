#!/usr/bin/env bun
// tool/src/wasm/module-instantiate.cc.js (族合一: module-instantiate.cc + module-instantiate.h)
// 大 .cc 规则面: 实例化阶段序、导入校验错误消息表、包装签名参数偏移、
// 内存分配上限防线; 另出 InstanceBuilder 方法签名骨架。
// 头侧枚举/结构在 v8-wasm-core/src/module_instantiate.rs.skel (module-instantiate.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/wasm/module-instantiate.cc");
const h = readV8("src/wasm/module-instantiate.h");

// ---------- 1. 实例化阶段序 ----------
const phases = [...cc.matchAll(/MaybeDirectHandle<WasmInstanceObject> InstanceBuilder::(Build)\(\)|Maybe<bool> InstanceBuilder::(Build_Phase1)\(|void InstanceBuilder::(Build_Phase1_Infallible)\(\)|Maybe<bool> InstanceBuilder::(Build_Phase2)\(/g)].map((m) => m.slice(1).find(Boolean));
if (phases.join(",") !== "Build,Build_Phase1,Build_Phase1_Infallible,Build_Phase2") {
  throw new Error(`阶段序异常: ${phases.join(",")}`);
}

// ---------- 2. JS 调用包装签名参数偏移 (跳过隐式第一参) ----------
if (!cc.includes("constexpr uint32_t param_offset = 1;")) {
  throw new Error("包装签名参数偏移缺失");
}

// ---------- 3. 导入错误消息表 (LinkError/TypeError 语义面) ----------
const errMsgs = [...cc.matchAll(/thrower(?:_->|->)LinkError\(\s*"([^"]+)"/g)].map((m) => m[1]);
const typeErrs = [...cc.matchAll(/thrower(?:_->|->)TypeError\(\s*"([^"]+)"/g)].map((m) => m[1]);
if (errMsgs.length !== 23) throw new Error(`LinkError 消息 ${errMsgs.length} != 23`);
if (typeErrs.length !== 2) throw new Error(`TypeError 消息 ${typeErrs.length} != 2`);
// 消息锚
for (const must of [
  "%s: import not found",
  "%s: function import requires a callable",
  "%s: table import requires a WebAssembly.Table",
  "%s: memory import must be a WebAssembly.Memory object",
]) {
  if (!errMsgs.includes(must)) throw new Error(`错误消息缺失: ${must}`);
}
// %s 前缀统一为导入名 (module.field)
if (!cc.includes("ImportName(index, module_name).c_str()")) throw new Error("导入名前缀缺失");

// ---------- 4. 内存上限防线 ----------
if (!cc.includes("static_assert(kV8MaxWasmMemories <= kMaxUInt32);")) {
  throw new Error("内存数上限防线缺失");
}

// ---------- 5. 导入种类处理器 (5 类) ----------
const processors = [...cc.matchAll(/bool InstanceBuilder::(ProcessImported\w+)\(/g)].map((m) => m[1]);
if (processors.join(",") !== "ProcessImportedFunction,ProcessImportedTable,ProcessImportedWasmGlobalObject,ProcessImportedGlobal,ProcessImportedMemories") {
  throw new Error(`导入处理器序异常: ${processors.join(",")}`);
}

// ---------- 6. 内存导入页数校验 (最小/最大/共享 三查) ----------
for (const anchor of [
  "pages which is smaller than the declared",
  "memory import has no maximum limit, expected at most",
  "memory import has a larger maximum size %u",
  "mismatch in shared state of memory, declared = %d, imported =",
]) {
  if (!cc.includes(anchor)) throw new Error(`内存校验缺失: ${anchor}`);
}

// ---------- 7. 启动函数执行 (ExecuteStartFunction) ----------
if (!cc.includes("bool InstanceBuilder::ExecuteStartFunction() {")) {
  throw new Error("启动函数缺失");
}

// ---------- 8. 方法台账 ----------
const methods = [...cc.matchAll(/^[A-Za-z][A-Za-z0-9_:<>, &*]* InstanceBuilder::(\w+)\(/gm)].map((m) => m[1]);
if (methods.length !== 22) throw new Error(`方法台账 ${methods.length} != 22`);

// ---------- 9. 头侧: 构建器声明形态 ----------
if (!h.includes("WasmInstanceObject")) {
  throw new Error("头侧 WasmInstanceObject 缺失");
}
if (!cc.includes("InstanceBuilder::Build() {")) {
  throw new Error("Build 总控缺失");
}

console.log(`module-instantiate: 阶段=${phases.length} LinkError=${errMsgs.length} 导入处理器=${processors.length} 方法=${methods.length}`);

// ---------- 生成 Rust ----------
let msgTable = "";
for (const m of errMsgs) {
  msgTable += `    "${m}",\n`;
}
let skel = "";
for (const m of methods) {
  skel += `/// v8: src/wasm/module-instantiate.cc InstanceBuilder::${m}\npub fn instance_builder_${m.toLowerCase()}() {\n    todo!("就地修订: InstanceBuilder::${m}")\n}\n\n`;
}

const rust = `${banner(
  "src/wasm/module-instantiate.cc.js",
  "src/wasm/module-instantiate.{h,cc}",
  "实例化阶段序与导入校验错误消息表 + InstanceBuilder 方法签名骨架",
)}
//! 模块实例化规则面。

/// 实例化阶段序: Build 总控 -> Phase1 (可失败) -> Phase1_Infallible -> Phase2
/// v8: src/wasm/module-instantiate.cc InstanceBuilder
#[rustfmt::skip]
pub static INSTANTIATE_PHASES: &[&str] = &[
    "Build",
    "Build_Phase1",
    "Build_Phase1_Infallible",
    "Build_Phase2",
];

/// JS 函数导入包装签名: 机器签名跳过第一个隐式参数 (param_offset = 1)
/// v8: src/wasm/module-instantiate.cc 包装签名构造
pub const JS_WRAPPER_PARAM_OFFSET: u32 = 1;

/// 导入校验 LinkError 消息表 (%s 前缀统一为导入名 "module.field")
/// v8: src/wasm/module-instantiate.cc ProcessImports 系
#[rustfmt::skip]
pub static IMPORT_LINK_ERRORS: &[&str] = &[
${msgTable}];

/// 导入种类处理器 (5 类)
/// v8: src/wasm/module-instantiate.cc InstanceBuilder
#[rustfmt::skip]
pub static IMPORT_PROCESSORS: &[&str] = &[
    "ProcessImportedFunction — 可调用 + 类型匹配",
    "ProcessImportedTable — WebAssembly.Table + 类型匹配",
    "ProcessImportedWasmGlobalObject — WebAssembly.Global (含 v128 强制)",
    "ProcessImportedGlobal — 数值/合法引用",
    "ProcessImportedMemories — 页数下限/最大上限/共享态三查",
];

/// 内存导入三查错误语义
/// v8: src/wasm/module-instantiate.cc ProcessImportedMemories
#[rustfmt::skip]
pub static MEMORY_IMPORT_CHECKS: &[&str] = &[
    "导入页数 < 声明最小页数",
    "声明有最大但导入无最大",
    "导入最大 > 声明最大",
    "共享态不一致 (declared vs imported)",
];

/// 内存条目数上限防线: kV8MaxWasmMemories <= kMaxUInt32
/// v8: src/wasm/module-instantiate.cc static_assert
pub const MEMORIES_LIMIT_GUARD: &str = "kV8MaxWasmMemories <= kMaxUInt32";

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(INSTANTIATE_PHASES.len(), 4);
    assert_eq!(JS_WRAPPER_PARAM_OFFSET, 1);
    assert_eq!(IMPORT_LINK_ERRORS.len(), 23);
    assert_eq!(IMPORT_PROCESSORS.len(), 5);
    assert_eq!(MEMORY_IMPORT_CHECKS.len(), 4);
};

// —— 方法签名骨架 (就地修订 todo) ——
${skel}`;

const p = writeSrc("v8-wasm-core", "module_instantiate_cc.rs", rust);
console.log(`written ${p}`);
