#!/usr/bin/env bun
// tool/src/debug/module.js — 覆盖 v8/src/objects/module.{-inl.h,cc,tq}
// 组→文件映射 (本脚本管辖): 席位表 3 文件 (.h Status 枚举归既有 tool/src/objects/module.h.js)
// 产物: v8-debug/src/module.rs
// 机械提取: tq Module 7 字段 + JSModuleNamespace/JSDeferredModuleNamespace/ScriptOrModule 布局
//   + -inl.h 访问器面 + .cc 方法签名骨架 (todo! 就地修订)

import { readV8, writeSrc, banner } from "../../lib.js";

const tq = readV8("src/objects/module.tq");
const inl = readV8("src/objects/module-inl.h");
const cc = readV8("src/objects/module.cc");
const h = readV8("src/objects/module.h");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[module.js] ${m}`); };

// ── 1. tq 布局 ───────────────────────────────────────────────────────────
function tqFields(block, count, label) {
  const fields = [...block.matchAll(/^  (\w+): ([^;]+);$/gm)].map((m) => [m[1], m[2].trim()]);
  must(fields.length === count, `${label} 字段数 ${fields.length} != ${count}`);
  return fields;
}
const modBlock = tq.slice(tq.indexOf("extern class Module extends HeapObject {"), tq.indexOf("extern class JSModuleNamespace"));
const modFields = tqFields(modBlock, 7, "tq Module");
must(modFields.map((f) => f[0]).join(",") === "exports,hash,status,module_namespace,deferred_module_namespace,exception,top_level_capability", "tq Module 布局漂移");
must(modFields[0][1] === "ObjectHashTable", "tq Module.exports 类型漂移");

const nsBlock = tq.slice(tq.indexOf("extern class JSModuleNamespace extends JSSpecialObject {"), tq.indexOf("extern class JSDeferredModuleNamespace"));
const nsFields = tqFields(nsBlock, 1, "tq JSModuleNamespace");
must(nsFields[0][0] === "module" && nsFields[0][1] === "Module", "tq JSModuleNamespace 布局漂移");

must(tq.includes("extern class JSDeferredModuleNamespace extends JSModuleNamespace {}"), "tq 缺 JSDeferredModuleNamespace (零字段)");

const somBlock = tq.slice(tq.indexOf("extern class ScriptOrModule extends Struct {"));
const somFields = tqFields(somBlock, 2, "tq ScriptOrModule");
must(somFields.map((f) => f[0]).join(",") === "resource_name,host_defined_options", "tq ScriptOrModule 布局漂移");
must(tq.includes("@abstract"), "tq 缺 @abstract (Module 抽象)");

// ── 2. .h Status 枚举交叉核对 (8 态, 既有 module.h.js 已产, 此处只锚定) ──
const status = h.match(/enum Status\s*\{([\s\S]*?)\n  \};/);
must(status, "module.h 缺 Status 枚举");
const statusNames = status[1].split(",").map((s) => s.replace(/\/\/.*$/gm, "").trim()).filter(Boolean);
must(statusNames.length === 8, `Status 条数 ${statusNames.length} != 8`);
must(statusNames[0] === "kUnlinked" && statusNames[7] === "kErrored", "Status 首尾态漂移");
// .h 布局交叉: Module 类体内 TaggedMember 7 字段 (排除 JSModuleNamespace/ScriptOrModule)
const modCls = h.slice(h.indexOf("V8_OBJECT class Module : public HeapObject {"), h.indexOf("} V8_OBJECT_END;"));
const hMembers = [...modCls.matchAll(/^  TaggedMember<[^;]+> (\w+);$/gm)].map((m) => m[1]);
must(hMembers.length === 7, `module.h Module TaggedMember 数 ${hMembers.length} != 7`);
must(hMembers.join(",") === "exports_,hash_,status_,module_namespace_,deferred_module_namespace_,exception_,top_level_capability_", "Module 成员序漂移");

// ── 3. -inl.h 访问器面锚 ─────────────────────────────────────────────────
for (const s of [
  "ScriptOrModule::resource_name() const", "ScriptOrModule::host_defined_options() const",
  "Module::exports() const", "Module::hash() const", "Module::status() const",
  "Module::module_namespace() const", "Module::deferred_module_namespace() const",
  "Module::exception() const", "Module::top_level_capability() const",
  "JSModuleNamespace::module() const",
  "struct Module::Hash", "struct ModuleHandleHash", "struct ModuleHandleEqual", "class UnorderedModuleSet",
]) must(inl.includes(s), `module-inl.h 缺 ${s}`);

// ── 4. .cc 方法签名骨架 (源码序提取) ──────────────────────────────────────
const ccSigs = [...cc.matchAll(/^(?:const char\*|Handle<\w+>|MaybeHandle<\w+>|MaybeDirectHandle<\w+>|Maybe<[^>]+>|DirectHandle<\w+>|Tagged<\w+>|bool|void|int)\s+(\w+)::(\w+)\(/gm)]
  .map((m) => `${m[1]}::${m[2]}`);
must(ccSigs.length >= 20, `.cc 方法数 ${ccSigs.length} < 20`);
for (const s of ["Module::StatusString", "Module::SetStatus", "Module::RecordError", "Module::Instantiate", "Module::Evaluate",
  "Module::GetModuleNamespaceCell", "JSModuleNamespace::GetExport", "JSDeferredModuleNamespace::EvaluateModuleSync",
  "JSModuleNamespace::DefineOwnProperty", "Module::IsGraphAsync"]) {
  must(ccSigs.includes(s), `.cc 缺方法 ${s}`);
}
must(new Set(ccSigs).size === ccSigs.length, ".cc 方法签名重复");

const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

let rust = banner("src/debug/module.js", "src/objects/module.{-inl.h,cc,tq}",
  "席位表差集: tq 布局 + inl 访问器 + cc 签名骨架 (Status 枚举数值归既有 module.h.js)");
rust += `//! Module 族差集 (debug 域): 抽象模块 + 命名空间 + ScriptOrModule。
//!
//! Module::Status 8 态与 JSModuleNamespace 索引归既有 module.h.js 产物;
//! 本文件登记 tq 布局与 .cc 方法骨架。

// ── tq extern class 布局表 (源码序) ──
`;
for (const [name, fields] of [["MODULE", modFields], ["JS_MODULE_NAMESPACE", nsFields], ["SCRIPT_OR_MODULE", somFields]]) {
  rust += `pub const ${name}_FIELDS: [(&str, &str); ${fields.length}] = [\n`;
  for (const f of fields) rust += `  ("${f[0]}", "${f[1]}"),\n`;
  rust += `];\n`;
}
rust += `/// JSDeferredModuleNamespace: 零新字段, 布局同 JSModuleNamespace (tq 空体)。
pub const JS_DEFERRED_MODULE_NAMESPACE_NEW_FIELDS: usize = 0;

// ── .h TaggedMember 布局交叉 (${hMembers.length} 字段, 源码序; 与 tq 1:1) ──
pub const MODULE_MEMBERS_H: [&str; ${hMembers.length}] = [
`;
for (const m of hMembers) rust += `  "${m}",\n`;
rust += `];

// ── .cc 方法签名骨架 (${ccSigs.length} 个, 源码序; 语义就地修订) ──
`;
for (const s of ccSigs) {
  const [cls, meth] = s.split("::");
  rust += `/// 原版 ${s} (module.cc)。
pub fn ${snake(cls)}_${snake(meth)}() {
  todo!("就地修订: module.cc ${s}")
}

`;
}
rust += `/// 编译期字符串相等 (str 的 const PartialEq 尚未稳定, 手写逐字节比较)
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(MODULE_FIELDS.len() == 7);
  assert!(str_eq(MODULE_FIELDS[2].0, "status"));
  assert!(str_eq(MODULE_FIELDS[6].0, "top_level_capability"));
  assert!(JS_MODULE_NAMESPACE_FIELDS.len() == 1);
  assert!(SCRIPT_OR_MODULE_FIELDS.len() == 2);
  assert!(JS_DEFERRED_MODULE_NAMESPACE_NEW_FIELDS == 0);
  assert!(MODULE_MEMBERS_H.len() == 7);
};
`;

const p = writeSrc("v8-debug", "module.rs", rust);
console.log(`module.js: ${p} — 布局 Module ${modFields.length} + NS ${nsFields.length} + SOM ${somFields.length} + Deferred 0、.h 交叉 ${hMembers.length}、.cc 方法 ${ccSigs.length}, 断言 ${checks} 项全过`);
