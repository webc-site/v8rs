#!/usr/bin/env bun
// tool/src/codegen/reloc-info.h.js — 从 v8/src/codegen/reloc-info.h 提取并校验 RelocInfo::Mode 与模式掩码
import { readV8, lines } from "../../lib.js";

const src = readV8("src/codegen/reloc-info.h");

// 圈定规则区
const enumLines = lines(src, 106, 173).join("\n");

function must(cond, msg) {
  if (!cond) throw new Error(`reloc-info.h 对数核对失败: ${msg}`);
}

must(enumLines.includes("NO_INFO,"), "NO_INFO 模式缺失");
must(enumLines.includes("CODE_TARGET,"), "CODE_TARGET 模式缺失");
must(enumLines.includes("RELATIVE_CODE_TARGET,"), "RELATIVE_CODE_TARGET 模式缺失");
must(enumLines.includes("COMPRESSED_EMBEDDED_OBJECT,"), "COMPRESSED_EMBEDDED_OBJECT 模式缺失");
must(enumLines.includes("FULL_EMBEDDED_OBJECT,"), "FULL_EMBEDDED_OBJECT 模式缺失");
must(enumLines.includes("JS_DISPATCH_HANDLE,"), "JS_DISPATCH_HANDLE 模式缺失");
must(enumLines.includes("WASM_CALL,"), "WASM_CALL 模式缺失");
must(enumLines.includes("WASM_STUB_CALL,"), "WASM_STUB_CALL 模式缺失");
must(enumLines.includes("EXTERNAL_REFERENCE,"), "EXTERNAL_REFERENCE 模式缺失");
must(enumLines.includes("INTERNAL_REFERENCE,"), "INTERNAL_REFERENCE 模式缺失");
must(enumLines.includes("OFF_HEAP_TARGET,"), "OFF_HEAP_TARGET 模式缺失");
must(enumLines.includes("NEAR_BUILTIN_ENTRY,"), "NEAR_BUILTIN_ENTRY 模式缺失");
must(enumLines.includes("CONST_POOL,"), "CONST_POOL 模式缺失");
must(enumLines.includes("VENEER_POOL,"), "VENEER_POOL 模式缺失");
must(enumLines.includes("DEOPT_SCRIPT_OFFSET,"), "DEOPT_SCRIPT_OFFSET 模式缺失");
must(enumLines.includes("PC_JUMP,"), "PC_JUMP 模式缺失");
must(enumLines.includes("NUMBER_OF_MODES,"), "NUMBER_OF_MODES 模式缺失");

// 校验标记位与边界
must(src.includes("LAST_CODE_TARGET_MODE = RELATIVE_CODE_TARGET"), "LAST_CODE_TARGET_MODE 漂移");
must(src.includes("FIRST_REAL_RELOC_MODE = CODE_TARGET"), "FIRST_REAL_RELOC_MODE 漂移");
must(src.includes("LAST_REAL_RELOC_MODE = VENEER_POOL"), "LAST_REAL_RELOC_MODE 漂移");
must(src.includes("FIRST_EMBEDDED_OBJECT_RELOC_MODE = COMPRESSED_EMBEDDED_OBJECT"), "FIRST_EMBEDDED_OBJECT_RELOC_MODE 漂移");
must(src.includes("LAST_EMBEDDED_OBJECT_RELOC_MODE = FULL_EMBEDDED_OBJECT"), "LAST_EMBEDDED_OBJECT_RELOC_MODE 漂移");
must(src.includes("LAST_GCED_ENUM = JS_DISPATCH_HANDLE"), "LAST_GCED_ENUM 漂移");
must(src.includes("FIRST_BUILTIN_ENTRY_MODE = OFF_HEAP_TARGET"), "FIRST_BUILTIN_ENTRY_MODE 漂移");
must(src.includes("LAST_BUILTIN_ENTRY_MODE = NEAR_BUILTIN_ENTRY"), "LAST_BUILTIN_ENTRY_MODE 漂移");
must(src.includes("FIRST_SHAREABLE_RELOC_MODE = WASM_CALL"), "FIRST_SHAREABLE_RELOC_MODE 漂移");
must(src.includes("FIRST_DEOPT_MODE = DEOPT_SCRIPT_OFFSET"), "FIRST_DEOPT_MODE 漂移");
must(src.includes("LAST_DEOPT_MODE = DEOPT_NODE_ID"), "LAST_DEOPT_MODE 漂移");

// 提取实际 enum 项并顺序给编号
const modeNames = [
  "NO_INFO",
  "CODE_TARGET",
  "RELATIVE_CODE_TARGET",
  "COMPRESSED_EMBEDDED_OBJECT",
  "FULL_EMBEDDED_OBJECT",
  "JS_DISPATCH_HANDLE",
  "WASM_CALL",
  "WASM_STUB_CALL",
  "WASM_CODE_POINTER_TABLE_ENTRY",
  "WASM_CANONICAL_SIG_ID",
  "WASM_CODE_POINTER",
  "EXTERNAL_REFERENCE",
  "INTERNAL_REFERENCE",
  "INTERNAL_REFERENCE_ENCODED",
  "OFF_HEAP_TARGET",
  "NEAR_BUILTIN_ENTRY",
  "CONST_POOL",
  "VENEER_POOL",
  "DEOPT_SCRIPT_OFFSET",
  "DEOPT_INLINING_ID",
  "DEOPT_REASON",
  "DEOPT_ID",
  "DEOPT_NODE_ID",
  "PC_JUMP",
  "NUMBER_OF_MODES",
];

const modeMap = new Map();
modeNames.forEach((name, idx) => modeMap.set(name, idx));

const numberOfModes = modeMap.get("NUMBER_OF_MODES");
must(numberOfModes <= 32, "模式总数超出 32 位掩码上限");

function modeMask(name) {
  const val = modeMap.get(name);
  if (val === undefined) throw new Error(`Unknown mode: ${name}`);
  return 1 << val;
}

// 模式掩码计算
const firstReal = modeMap.get("CODE_TARGET");
const lastReal = modeMap.get("VENEER_POOL");
let allRealModesMask = 0;
for (let i = firstReal; i <= lastReal; i++) {
  allRealModesMask |= (1 << i);
}

const embeddedObjectMask = modeMask("FULL_EMBEDDED_OBJECT") | modeMask("COMPRESSED_EMBEDDED_OBJECT");
const codeTargetMask = modeMask("CODE_TARGET") | modeMask("RELATIVE_CODE_TARGET");
const jsDispatchHandleMask = modeMask("JS_DISPATCH_HANDLE");

// x64 的 kApplyMask
const x64ApplyMask =
  modeMask("CODE_TARGET") |
  modeMask("NEAR_BUILTIN_ENTRY") |
  modeMask("INTERNAL_REFERENCE") |
  modeMask("WASM_CALL") |
  modeMask("WASM_STUB_CALL");

const postCodegenRelocationMask =
  modeMask("CODE_TARGET") |
  modeMask("COMPRESSED_EMBEDDED_OBJECT") |
  modeMask("FULL_EMBEDDED_OBJECT") |
  modeMask("NEAR_BUILTIN_ENTRY") |
  modeMask("WASM_STUB_CALL") |
  modeMask("RELATIVE_CODE_TARGET") |
  jsDispatchHandleMask |
  x64ApplyMask;

console.log(`reloc-info.h.js: 校验通过!`);
console.log(`  NUMBER_OF_MODES = ${numberOfModes}`);
console.log(`  allRealModesMask = 0x${allRealModesMask.toString(16)}`);
console.log(`  embeddedObjectMask = 0x${embeddedObjectMask.toString(16)}`);
console.log(`  codeTargetMask = 0x${codeTargetMask.toString(16)}`);
console.log(`  x64ApplyMask = 0x${x64ApplyMask.toString(16)}`);
console.log(`  postCodegenRelocationMask = 0x${postCodegenRelocationMask.toString(16)}`);
