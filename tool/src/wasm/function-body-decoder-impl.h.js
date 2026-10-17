#!/usr/bin/env bun
// tool/src/wasm/function-body-decoder-impl.h.js
// 机械提取 v8/src/wasm/function-body-decoder-impl.h 的规则区 (>2k 行, 只取枚举/常量表):
// load/store 类型硬编码表、ATOMIC_OP_LIST、ControlKind/Reachability、DecodingMode。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/function-body-decoder-impl.h");

// 1. GetLoadType 硬编码表
const loadFn = hSrc.match(/inline constexpr LoadType GetLoadType\(WasmOpcode opcode\)\s*\{([\s\S]*?)\n\}/);
if (!loadFn) throw new Error("未找到 GetLoadType");
const loadTypes = [...loadFn[1].matchAll(/LoadType::k(\w+)/g)].map((m) => m[1]);
const loadMin = loadFn[1].match(/kMinOpcode = kExpr(\w+);/);
const loadMax = loadFn[1].match(/kMaxOpcode = kExpr(\w+);/);
if (!loadMin || !loadMax) throw new Error("load 区间锚点缺失");
if (loadTypes.length !== 14) throw new Error(`kLoadTypes ${loadTypes.length} 条, 期望 14`);
if (loadMin[1] !== "I32LoadMem" || loadMax[1] !== "I64LoadMem32U") throw new Error("load 区间漂移");

// 2. GetStoreType 硬编码表
const storeFn = hSrc.match(/inline constexpr StoreType GetStoreType\(WasmOpcode opcode\)\s*\{([\s\S]*?)\n\}/);
if (!storeFn) throw new Error("未找到 GetStoreType");
const storeTypes = [...storeFn[1].matchAll(/StoreType::k(\w+)/g)].map((m) => m[1]);
const storeMin = storeFn[1].match(/kMinOpcode = kExpr(\w+);/);
const storeMax = storeFn[1].match(/kMaxOpcode = kExpr(\w+);/);
if (!storeMin || !storeMax) throw new Error("store 区间锚点缺失");
if (storeTypes.length !== 9) throw new Error(`kStoreTypes ${storeTypes.length} 条, 期望 9`);
if (storeMin[1] !== "I32StoreMem" || storeMax[1] !== "I64StoreMem32") throw new Error("store 区间漂移");

// 3. DecodingMode
const dmMatch = hSrc.match(/enum DecodingMode \{([^}]*)\};/);
if (!dmMatch) throw new Error("未找到 DecodingMode");
const decodingModes = dmMatch[1].split(",").map((s) => s.trim()).filter(Boolean);
if (decodingModes.join() !== "kFunctionBody,kConstantExpression") throw new Error("DecodingMode 漂移");

// 4. ControlKind
const ckMatch = hSrc.match(/enum ControlKind : uint8_t \{([^}]*)\};/);
if (!ckMatch) throw new Error("未找到 ControlKind");
const controlKinds = ckMatch[1].split(",").map((s) => s.trim()).filter(Boolean);
if (controlKinds.length !== 8) throw new Error(`ControlKind ${controlKinds.length}, 期望 8`);
if (controlKinds[0] !== "kControlIf" || controlKinds[controlKinds.length - 1] !== "kControlTryCatchAll") {
  throw new Error("ControlKind 锚点漂移");
}

// 5. Reachability
const reMatch = hSrc.match(/enum Reachability : uint8_t \{([^}]*)\};/);
if (!reMatch) throw new Error("未找到 Reachability");
const reachability = reMatch[1]
  .replace(/\/\/[^\n]*/g, "")
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (reachability.length !== 3) throw new Error(`Reachability ${reachability.length}, 期望 3`);

// 6. ATOMIC_OP_LIST: V(Name, Uint32...)
const atomicList = [...hSrc.matchAll(/^#define ATOMIC_OP_LIST\(V\)\s*\\\s*\n([\s\S]*?)\n\n/gm)];
const atomicBlock = hSrc.match(/#define ATOMIC_OP_LIST\(V\)([\s\S]*?)\n\n/);
if (!atomicBlock) throw new Error("未找到 ATOMIC_OP_LIST");
const atomicOps = [...atomicBlock[1].replace(/\\\n/g, " ").matchAll(/V\(\s*(\w+)\s*,\s*(\w+)\s*\)/g)].map(
  (m) => ({ name: m[1], type: m[2] }),
);
if (atomicOps.length < 40) throw new Error(`ATOMIC_OP_LIST ${atomicOps.length} 条, 异常偏少`);
if (atomicOps[0].name !== "AtomicNotify") throw new Error("ATOMIC_OP_LIST 首条漂移");

console.log(
  `load ${loadTypes.length}, store ${storeTypes.length}, control ${controlKinds.length}, atomic ${atomicOps.length}`,
);

let rust = banner(
  "src/wasm/function-body-decoder-impl.h.js",
  "src/wasm/function-body-decoder-impl.h",
  "函数体解码器规则区: load/store 类型表 + Control/Reachability + ATOMIC_OP_LIST",
);

const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

rust += `
/// 解码模式 (对应 enum DecodingMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecodingMode {
  /// kFunctionBody
  FunctionBody,
  /// kConstantExpression
  ConstantExpression,
}

/// 控制帧种类 (对应 enum ControlKind : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ControlKind {
`;

for (const [i, k] of controlKinds.entries()) {
  rust += `  /// ${k}\n  ${snake(k.replace(/^kControl/, ""))} = ${i},\n`;
}

rust += `}

/// 可达性 (对应 enum Reachability : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Reachability {
`;

const reachDoc = [
  "reachable code (可达代码)",
  "reachable code in unreachable block (块不可达但验证正常)",
  "code unreachable in its own block (多态验证)",
];
reachability.forEach((k, i) => {
  rust += `  /// ${k}: ${reachDoc[i]}\n  ${snake(k.replace(/^k/, ""))} = ${i},\n`;
});

rust += `}

/// load 操作码区间硬编码表 (对应 GetLoadType 内 kLoadTypes, ${loadTypes.length} 条)
/// 区间 [kExpr${loadMin[1]}, kExpr${loadMax[1]}], 索引 = opcode - 起点
#[rustfmt::skip]
pub static LOAD_TYPES: &[&str] = &[
${loadTypes.map((t) => `  "${t}",`).join("\n")}
];

/// store 操作码区间硬编码表 (对应 GetStoreType 内 kStoreTypes, ${storeTypes.length} 条)
/// 区间 [kExpr${storeMin[1]}, kExpr${storeMax[1]}]
#[rustfmt::skip]
pub static STORE_TYPES: &[&str] = &[
${storeTypes.map((t) => `  "${t}",`).join("\n")}
];

/// load 区间端点 (kExpr 编码, 引用 wasm_opcodes)
pub const LOAD_MIN_OPCODE: u32 = crate::wasm_opcodes::expr::${snake(loadMin[1])} as u32;
pub const LOAD_MAX_OPCODE: u32 = crate::wasm_opcodes::expr::${snake(loadMax[1])} as u32;
/// store 区间端点
pub const STORE_MIN_OPCODE: u32 = crate::wasm_opcodes::expr::${snake(storeMin[1])} as u32;
pub const STORE_MAX_OPCODE: u32 = crate::wasm_opcodes::expr::${snake(storeMax[1])} as u32;

/// 对应 GetLoadType(opcode): 区间查表
#[inline]
#[must_use]
pub fn get_load_type(opcode: u32) -> Option<&'static str> {
  if !(LOAD_MIN_OPCODE..=LOAD_MAX_OPCODE).contains(&opcode) {
    return None;
  }
  LOAD_TYPES.get((opcode - LOAD_MIN_OPCODE) as usize).copied()
}

/// 对应 GetStoreType(opcode): 区间查表
#[inline]
#[must_use]
pub fn get_store_type(opcode: u32) -> Option<&'static str> {
  if !(STORE_MIN_OPCODE..=STORE_MAX_OPCODE).contains(&opcode) {
    return None;
  }
  STORE_TYPES.get((opcode - STORE_MIN_OPCODE) as usize).copied()
}

/// ATOMIC_OP_LIST 展开 (V(Name, ArgType), ${atomicOps.length} 条)
#[rustfmt::skip]
pub static ATOMIC_OP_LIST: &[(&str, &str)] = &[
`;

for (const a of atomicOps) {
  rust += `  ("${a.name}", "${a.type}"),\n`;
}

rust += `];

// —— 提取不变式断言 ——
const _: () = {
  assert!(LOAD_TYPES.len() == 14);
  assert!(STORE_TYPES.len() == 9);
  assert!(LOAD_MAX_OPCODE - LOAD_MIN_OPCODE + 1 == 14);
  assert!(STORE_MAX_OPCODE - STORE_MIN_OPCODE + 1 == 9);
  assert!(ATOMIC_OP_LIST.len() == ${atomicOps.length});
  assert!(ATOMIC_OP_LIST[0].0 == "AtomicNotify");
  assert!(ControlKind::IF == 0);
  assert!(ControlKind::TRY_CATCH_ALL == 7);
  assert!(Reachability::REACHABLE == 0);
  assert!(Reachability::UNREACHABLE == 2);
  assert!(get_load_type(LOAD_MIN_OPCODE) == Some("I32Load"));
  assert!(get_load_type(LOAD_MAX_OPCODE) == Some("I64Load32U"));
  assert!(get_store_type(STORE_MIN_OPCODE) == Some("I32Store"));
};
`;
;

const p = writeSrc("v8-wasm-core", "function_body_decoder_tables.rs", rust);
console.log(`written ${p}`);
