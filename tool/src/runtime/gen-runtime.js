#!/usr/bin/env bun
// tool/src/runtime/gen-runtime.js — v8-runtime 代码生成器

import { readV8, writeSrc } from "../../lib.js";
import { fnName, fmt } from "../builtins/builtins-lib.js";

const crate = "v8-runtime";

function toPascal(s) {
  return s
    .split("_")
    .map((p) => p.charAt(0).toUpperCase() + p.slice(1))
    .join("");
}

// 1. 读取 runtime.h 提取所有的 F(name, nargs, ressize) 与 I(name, nargs, ressize) 以及条件宏
const runtimeH = readV8("src/runtime/runtime.h");
const cleanH = runtimeH.replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/.*/g, "");
const intrinsicRegex =
  /\b(?:[FI]\s*\(|IF_\w+\s*\(\s*[FI]\s*,\s*)(\w+)\s*,\s*(-?\d+)\s*,\s*(\d+)/g;
const uniqueIntrinsics = [];
const seenNames = new Set();
for (const m of cleanH.matchAll(intrinsicRegex)) {
  if (!seenNames.has(m[1])) {
    seenNames.add(m[1]);
    uniqueIntrinsics.push({
      name: m[1],
      nargs: parseInt(m[2], 10),
      resultSize: parseInt(m[3], 10),
      pascal: toPascal(m[1]),
    });
  }
}

// 2. 生成 table.rs
const tableLines = [
  "// 由 tool/src/runtime/gen-runtime.js 生成; 来源 v8/src/runtime/runtime.h",
  "// Runtime 函数总表 (FunctionId / 描述符面)",
  "",
  "use strum::{Display, EnumIter, EnumString};",
  "",
  "/// Runtime 函数全局标识枚举 (来源: runtime.h:958)",
  "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]",
  "#[repr(i32)]",
  "pub enum RuntimeFunctionId {",
];
for (const item of uniqueIntrinsics) {
  const serializes = [...new Set([`k${item.name}`, item.name, item.pascal])];
  const strumAttr = serializes.map((s) => `serialize = "${s}"`).join(", ");
  tableLines.push(`  #[strum(${strumAttr})]`);
  tableLines.push(`  ${item.pascal},`);
}
tableLines.push('  #[strum(serialize = "kNumFunctions", serialize = "NumFunctions")]');
tableLines.push("  NumFunctions,");
tableLines.push("}");
tableLines.push("");
tableLines.push("impl RuntimeFunctionId {");
tableLines.push("  pub const fn name(&self) -> &'static str {");
tableLines.push("    match self {");
for (const item of uniqueIntrinsics) {
  tableLines.push(`      Self::${item.pascal} => "${item.name}",`);
}
tableLines.push('      Self::NumFunctions => "NumFunctions",');
tableLines.push("    }");
tableLines.push("  }");
tableLines.push("");
tableLines.push("  pub const fn c_enum_name(&self) -> &'static str {");
tableLines.push("    match self {");
for (const item of uniqueIntrinsics) {
  tableLines.push(`      Self::${item.pascal} => "k${item.name}",`);
}
tableLines.push('      Self::NumFunctions => "kNumFunctions",');
tableLines.push("    }");
tableLines.push("  }");
tableLines.push("");
tableLines.push("  pub fn descriptor(&self) -> Option<&'static RuntimeFunction> {");
tableLines.push("    find_runtime_function(self.name())");
tableLines.push("  }");
tableLines.push("}");
tableLines.push("");
tableLines.push("/// Runtime 函数描述符");
tableLines.push("#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
tableLines.push("pub struct RuntimeFunction {");
tableLines.push("  pub id: RuntimeFunctionId,");
tableLines.push("  pub name: &'static str,");
tableLines.push("  pub nargs: i8,");
tableLines.push("  pub result_size: i8,");
tableLines.push("}");
tableLines.push("");
tableLines.push("/// Runtime 函数全量总表");
tableLines.push(`pub const RUNTIME_FUNCTION_COUNT: usize = ${uniqueIntrinsics.length};`);
tableLines.push("pub const RUNTIME_FUNCTIONS: &[RuntimeFunction] = &[");
for (const item of uniqueIntrinsics) {
  tableLines.push("  RuntimeFunction {");
  tableLines.push(`    id: RuntimeFunctionId::${item.pascal},`);
  tableLines.push(`    name: "${item.name}",`);
  tableLines.push(`    nargs: ${item.nargs},`);
  tableLines.push(`    result_size: ${item.resultSize},`);
  tableLines.push("  },");
}
tableLines.push("];");
tableLines.push("");
tableLines.push("/// 根据名称检索运行时函数描述符");
tableLines.push("pub fn find_runtime_function(name: &str) -> Option<&'static RuntimeFunction> {");
tableLines.push("  RUNTIME_FUNCTIONS.iter().find(|f| f.name == name)");
tableLines.push("}");
tableLines.push("");
tableLines.push("/// Runtime 函数调用属性");
tableLines.push("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]");
tableLines.push("pub enum RuntimeCallProperty {");
tableLines.push('  #[strum(serialize = "kCannotTriggerGC")]');
tableLines.push("  CannotTriggerGC,");
tableLines.push("}");
tableLines.push("");
tableLines.push("/// 优化状态枚举 (来源: runtime.h)");
tableLines.push("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]");
tableLines.push("pub enum OptimizationStatus {");
tableLines.push("  IsFunction,");
tableLines.push("  NeverOptimize,");
tableLines.push("  AlwaysOptimize,");
tableLines.push("  MaybeDeopted,");
tableLines.push("  Optimized,");
tableLines.push("  Turbofanned,");
tableLines.push("  Maglevved,");
tableLines.push("  Interpreted,");
tableLines.push("}");
tableLines.push("");
tableLines.push("/// 调试打印值类型 (来源: runtime.h)");
tableLines.push("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]");
tableLines.push("#[repr(u16)]");
tableLines.push("pub enum DebugPrintValueType {");
tableLines.push("  Normal = 0,");
tableLines.push("  Smi = 1,");
tableLines.push("  HeapObject = 2,");
tableLines.push("}");
tableLines.push("");
tableLines.push("/// 双返回值调用约定结构体 (来源: runtime-utils.h ObjectPair)");
tableLines.push("#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]");
tableLines.push("pub struct ObjectPair {");
tableLines.push("  pub x: u64,");
tableLines.push("  pub y: u64,");
tableLines.push("}");
tableLines.push("");
tableLines.push("/// Runtime 隔离窄环境 trait");
tableLines.push("pub trait RuntimeEnv {");
tableLines.push("  fn isolate_address(&self) -> u64;");
tableLines.push("  fn has_pending_exception(&self) -> bool;");
tableLines.push("}");
tableLines.push("");

writeSrc(crate, "table.rs", fmt(tableLines.join("\n")));

// 3. 生成 numbers.rs
const NUMBERS_GROUPS = {
  NUMBERS: ["src/runtime/runtime-numbers.cc"],
  STRINGS: ["src/runtime/runtime-strings.cc"],
  BIGINT: ["src/runtime/runtime-bigint.cc"],
  ATOMICS: ["src/runtime/runtime-atomics.cc"],
  DATE: ["src/runtime/runtime-date.cc"],
  OPERATORS: ["src/runtime/runtime-operators.cc"],
};
const numbersFns = [];
const numbersSeen = new Set();
for (const [group, [file]] of Object.entries(NUMBERS_GROUPS)) {
  const content = readV8(file);
  for (const m of content.matchAll(/RUNTIME_FUNCTION(?:_RETURN_PAIR)?\s*\(\s*(Runtime_\w+)\s*\)/g)) {
    const raw = m[1];
    const name = raw.replace(/^Runtime_/, "");
    if (!numbersSeen.has(name)) {
      numbersSeen.add(name);
      numbersFns.push({ group, raw, name, fn: fnName(name), pascal: toPascal(name) });
    }
  }
}
const numbersLines = [
  "// 由 tool/src/runtime/gen-runtime.js 生成",
  "// 数值/字符串/BigInt/Atomics/Date/Operators 运行时函数",
  "",
  "use strum::{Display, EnumIter, EnumString};",
  "use crate::table::{find_runtime_function, RuntimeFunction, RuntimeFunctionId};",
  "",
  `/// 数值与基础操作运行时函数 (${numbersFns.length} 项)`,
  "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]",
  "pub enum NumbersRuntimeFunction {",
];
for (const item of numbersFns) {
  const serializes = [...new Set([`Runtime_${item.name}`, item.name, item.pascal])];
  const strumAttr = serializes.map((s) => `serialize = "${s}"`).join(", ");
  numbersLines.push(`  #[strum(${strumAttr})]`);
  numbersLines.push(`  ${item.pascal},`);
}
numbersLines.push("}");
numbersLines.push("");
numbersLines.push("impl NumbersRuntimeFunction {");
numbersLines.push("  pub const fn name(&self) -> &'static str {");
numbersLines.push("    match self {");
for (const item of numbersFns) {
  numbersLines.push(`      Self::${item.pascal} => "${item.name}",`);
}
numbersLines.push("    }");
numbersLines.push("  }");
numbersLines.push("");
numbersLines.push("  pub fn descriptor(&self) -> Option<&'static RuntimeFunction> {");
numbersLines.push("    find_runtime_function(self.name())");
numbersLines.push("  }");
numbersLines.push("");
numbersLines.push("  pub fn function_id(&self) -> Option<RuntimeFunctionId> {");
numbersLines.push("    self.descriptor().map(|f| f.id)");
numbersLines.push("  }");
numbersLines.push("");
for (const g of Object.keys(NUMBERS_GROUPS)) {
  const pred = `is_${g.toLowerCase()}`;
  const matched = numbersFns.filter((x) => x.group === g);
  numbersLines.push(`  pub const fn ${pred}(&self) -> bool {`);
  numbersLines.push("    matches!(");
  numbersLines.push("      self,");
  numbersLines.push(`      ${matched.map((x) => `Self::${x.pascal}`).join(" | ")}`);
  numbersLines.push("    )");
  numbersLines.push("  }");
  numbersLines.push("");
}
numbersLines.push("}");
numbersLines.push("");
for (const item of numbersFns) {
  numbersLines.push(`pub const fn ${item.fn}() -> NumbersRuntimeFunction {`);
  numbersLines.push(`  NumbersRuntimeFunction::${item.pascal}`);
  numbersLines.push("}");
  numbersLines.push("");
}

writeSrc(crate, "numbers.rs", fmt(numbersLines.join("\n")));

// 4. 生成 object.rs
const OBJECT_GROUPS = {
  OBJECT: ["src/runtime/runtime-object.cc"],
  ARRAY: ["src/runtime/runtime-array.cc"],
  COLLECTIONS: ["src/runtime/runtime-collections.cc"],
  TYPEDARRAY: ["src/runtime/runtime-typedarray.cc"],
  CLASSES: ["src/runtime/runtime-classes.cc"],
  PROXY: ["src/runtime/runtime-proxy.cc"],
};
const objectFns = [];
const objectSeen = new Set();
for (const [group, [file]] of Object.entries(OBJECT_GROUPS)) {
  const content = readV8(file);
  for (const m of content.matchAll(/RUNTIME_FUNCTION(?:_RETURN_PAIR)?\s*\(\s*(Runtime_\w+)\s*\)/g)) {
    const raw = m[1];
    const name = raw.replace(/^Runtime_/, "");
    if (!objectSeen.has(name)) {
      objectSeen.add(name);
      objectFns.push({ group, raw, name, fn: fnName(name), pascal: toPascal(name) });
    }
  }
}
const objectLines = [
  "// 由 tool/src/runtime/gen-runtime.js 生成",
  "// 对象/数组/集合/TypedArray/Classes/Proxy 运行时函数",
  "",
  "use strum::{Display, EnumIter, EnumString};",
  "use crate::table::{find_runtime_function, RuntimeFunction, RuntimeFunctionId};",
  "",
  `/// 对象与集合操作运行时函数 (${objectFns.length} 项)`,
  "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]",
  "pub enum ObjectRuntimeFunction {",
];
for (const item of objectFns) {
  const serializes = [...new Set([`Runtime_${item.name}`, item.name, item.pascal])];
  const strumAttr = serializes.map((s) => `serialize = "${s}"`).join(", ");
  objectLines.push(`  #[strum(${strumAttr})]`);
  objectLines.push(`  ${item.pascal},`);
}
objectLines.push("}");
objectLines.push("");
objectLines.push("impl ObjectRuntimeFunction {");
objectLines.push("  pub const fn name(&self) -> &'static str {");
objectLines.push("    match self {");
for (const item of objectFns) {
  objectLines.push(`      Self::${item.pascal} => "${item.name}",`);
}
objectLines.push("    }");
objectLines.push("  }");
objectLines.push("");
objectLines.push("  pub fn descriptor(&self) -> Option<&'static RuntimeFunction> {");
objectLines.push("    find_runtime_function(self.name())");
objectLines.push("  }");
objectLines.push("");
objectLines.push("  pub fn function_id(&self) -> Option<RuntimeFunctionId> {");
objectLines.push("    self.descriptor().map(|f| f.id)");
objectLines.push("  }");
objectLines.push("");
for (const g of Object.keys(OBJECT_GROUPS)) {
  const pred = `is_${g.toLowerCase()}`;
  const matched = objectFns.filter((x) => x.group === g);
  objectLines.push(`  pub const fn ${pred}(&self) -> bool {`);
  objectLines.push("    matches!(");
  objectLines.push("      self,");
  objectLines.push(`      ${matched.map((x) => `Self::${x.pascal}`).join(" | ")}`);
  objectLines.push("    )");
  objectLines.push("  }");
  objectLines.push("");
}
objectLines.push("}");
objectLines.push("");
for (const item of objectFns) {
  objectLines.push(`pub const fn ${item.fn}() -> ObjectRuntimeFunction {`);
  objectLines.push(`  ObjectRuntimeFunction::${item.pascal}`);
  objectLines.push("}");
  objectLines.push("");
}

writeSrc(crate, "object.rs", fmt(objectLines.join("\n")));

// 5. 生成 misc.rs
const MISC_FILES = [
  "src/runtime/runtime-scopes.cc",
  "src/runtime/runtime-module.cc",
  "src/runtime/runtime-debug.cc",
  "src/runtime/runtime-compiler.cc",
  "src/runtime/runtime-internal.cc",
  "src/runtime/runtime-wasm.cc",
  "src/runtime/runtime-function.cc",
  "src/runtime/runtime-generator.cc",
  "src/runtime/runtime-promise.cc",
  "src/runtime/runtime-regexp.cc",
  "src/runtime/runtime-symbol.cc",
  "src/runtime/runtime-literals.cc",
  "src/runtime/runtime-forin.cc",
  "src/runtime/runtime-futex.cc",
  "src/runtime/runtime-intl.cc",
  "src/runtime/runtime-shadow-realm.cc",
  "src/runtime/runtime-trace.cc",
  "src/runtime/runtime-weak-refs.cc",
];
const miscFns = [];
const miscSeen = new Set();
for (const file of MISC_FILES) {
  const content = readV8(file);
  const tag = file.replace("src/runtime/runtime-", "").replace(".cc", "").replace(/-/g, "_");
  for (const m of content.matchAll(/RUNTIME_FUNCTION(?:_RETURN_PAIR)?\s*\(\s*(Runtime_\w+)\s*\)/g)) {
    const raw = m[1];
    const name = raw.replace(/^Runtime_/, "");
    if (!miscSeen.has(name)) {
      miscSeen.add(name);
      miscFns.push({ tag, raw, name, fn: fnName(name), pascal: toPascal(name) });
    }
  }
}
const miscLines = [
  "// 由 tool/src/runtime/gen-runtime.js 生成",
  "// 杂项运行时函数 (Scopes, Compiler, Debug, Internal, Wasm, Promise 等)",
  "",
  "use strum::{Display, EnumIter, EnumString};",
  "use crate::table::{find_runtime_function, RuntimeFunction, RuntimeFunctionId};",
  "",
  `/// 杂项操作运行时函数 (${miscFns.length} 项)`,
  "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]",
  "pub enum MiscRuntimeFunction {",
];
for (const item of miscFns) {
  const serializes = [...new Set([`Runtime_${item.name}`, item.name, item.pascal])];
  const strumAttr = serializes.map((s) => `serialize = "${s}"`).join(", ");
  miscLines.push(`  #[strum(${strumAttr})]`);
  miscLines.push(`  ${item.pascal},`);
}
miscLines.push("}");
miscLines.push("");
miscLines.push("impl MiscRuntimeFunction {");
miscLines.push("  pub const fn name(&self) -> &'static str {");
miscLines.push("    match self {");
for (const item of miscFns) {
  miscLines.push(`      Self::${item.pascal} => "${item.name}",`);
}
miscLines.push("    }");
miscLines.push("  }");
miscLines.push("");
miscLines.push("  pub fn descriptor(&self) -> Option<&'static RuntimeFunction> {");
miscLines.push("    find_runtime_function(self.name())");
miscLines.push("  }");
miscLines.push("");
miscLines.push("  pub fn function_id(&self) -> Option<RuntimeFunctionId> {");
miscLines.push("    self.descriptor().map(|f| f.id)");
miscLines.push("  }");
miscLines.push("");

const allTags = [...new Set(miscFns.map((x) => x.tag))];
for (const tag of allTags) {
  const pred = `is_${tag}`;
  const matched = miscFns.filter((x) => x.tag === tag);
  miscLines.push(`  pub const fn ${pred}(&self) -> bool {`);
  miscLines.push("    matches!(");
  miscLines.push("      self,");
  miscLines.push(`      ${matched.map((x) => `Self::${x.pascal}`).join(" | ")}`);
  miscLines.push("    )");
  miscLines.push("  }");
  miscLines.push("");
}
miscLines.push("}");
miscLines.push("");
for (const item of miscFns) {
  miscLines.push(`pub const fn ${item.fn}() -> MiscRuntimeFunction {`);
  miscLines.push(`  MiscRuntimeFunction::${item.pascal}`);
  miscLines.push("}");
  miscLines.push("");
}

writeSrc(crate, "misc.rs", fmt(miscLines.join("\n")));

// 6. 生成 error.rs
const errorLines = `use std::result;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
  #[error("Runtime function not found: {0}")]
  FunctionNotFound(String),

  #[error("Invalid argument count for {0}: expected {1}, got {2}")]
  InvalidArgumentCount(String, i8, i8),

  #[error("Runtime execution trapped: {0}")]
  ExecutionTrapped(String),
}

pub type Result<T> = result::Result<T, Error>;
`;
writeSrc(crate, "error.rs", fmt(errorLines));

// 7. 生成 lib.rs
const libLines = [
  "// 由 tool/src/runtime/gen-runtime.js 生成",
  "// V8 运行时函数大全",
  "",
  "#![forbid(unsafe_code)]",
  "",
  "mod error;",
  "mod misc;",
  "mod numbers;",
  "mod object;",
  "mod table;",
  "",
  "pub use error::{Error, Result};",
  "pub use misc::{MiscRuntimeFunction, " + miscFns.map((x) => x.fn).join(", ") + "};",
  "pub use numbers::{NumbersRuntimeFunction, " + numbersFns.map((x) => x.fn).join(", ") + "};",
  "pub use object::{ObjectRuntimeFunction, " + objectFns.map((x) => x.fn).join(", ") + "};",
  "pub use table::{",
  "  DebugPrintValueType, ObjectPair, OptimizationStatus, RuntimeCallProperty, RuntimeEnv,",
  "  RuntimeFunction, RuntimeFunctionId, find_runtime_function, RUNTIME_FUNCTIONS,",
  "  RUNTIME_FUNCTION_COUNT,",
  "};",
];
writeSrc(crate, "lib.rs", fmt(libLines.join("\n")));

console.log(`[gen-runtime.js] 生成完毕: table ${uniqueIntrinsics.length}, numbers ${numbersFns.length}, object ${objectFns.length}, misc ${miscFns.length}`);
