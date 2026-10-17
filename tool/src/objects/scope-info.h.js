#!/usr/bin/env bun
// tool/src/objects/scope-info.h.js — 覆盖 v8/src/objects/scope-info.{h,-inl.h,cc,tq}
// → v8-objects-rest/src/scope_info.rs
// 机械提取: Fields 平铺索引枚举 + BootstrappingType + kMaxVariablePositionDistance
//   + tq 全套枚举 (ScopeType/VariableAllocationInfo/VariableMode/InitializationFlag/
//   IsStaticFlag/MaybeAssignedFlag) + ScopeFlags/VariableProperties 位段表。
// >2k 行语义面 (scope-info.cc 变量解析逻辑) 登记跳过。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/scope-info.h");
const tqSrc = readV8("src/objects/scope-info.tq");

// ── h 侧 ────────────────────────────────────────────────────────────────────
// Fields 枚举 (固定头 5 字段 + 变长区起点)
const fieldsRe = hSrc.match(/enum Fields\s*\{([\s\S]*?)\n\s*\};/);
if (!fieldsRe) throw new Error("未找到 ScopeInfo::Fields 枚举");
const fields = fieldsRe[1].split(",").map((s) => s.trim()).filter(Boolean);
if (fields.length !== 6) throw new Error(`Fields 条数 ${fields.length} != 6`);
if (fields[5] !== "kVariablePartStart") throw new Error("Fields 尾条应为 kVariablePartStart");

// BootstrappingType
const boot = hSrc.match(/enum class BootstrappingType\s*\{([^}]+)\}/);
if (!boot) throw new Error("未找到 BootstrappingType");
const bootEntries = boot[1].split(",").map((s) => s.trim()).filter(Boolean);
if (bootEntries.join(",") !== "kScript,kFunction,kNative,kShadowRealm") {
  throw new Error(`BootstrappingType 条目失配: ${bootEntries.join(",")}`);
}

// kMaxVariablePositionDistance
const maxDist = hSrc.match(/kMaxVariablePositionDistance = (0x[0-9A-Fa-f]+|\d+);/);
if (!maxDist || maxDist[1] !== "0xFFFF") throw new Error("kMaxVariablePositionDistance 失配");

// ── tq 侧: extern enum 系列 (顺序值) ────────────────────────────────────────
function tqEnum(name, expect) {
  const re = new RegExp(`extern enum ${name} extends (\\w+)\\s*\\{([^}]+)\\}`);
  const m = tqSrc.match(re);
  if (!m) throw new Error(`未找到 tq enum ${name}`);
  const entries = m[2].replace(/\/\/.*$/gm, "").split(",").map((s) => s.trim()).filter(Boolean);
  if (expect && entries.join(",") !== expect) throw new Error(`tq enum ${name} 条目失配: ${entries.join(",")}`);
  return { repr: m[1], entries };
}

const scopeType = tqEnum("ScopeType", "SCRIPT_SCOPE,REPL_MODE_SCOPE,EVAL_SCOPE,MODULE_SCOPE,CLASS_SCOPE,FUNCTION_SCOPE,CATCH_SCOPE,BLOCK_SCOPE,WITH_SCOPE,SHADOW_REALM_SCOPE");
if (scopeType.repr !== "uint32") throw new Error("ScopeType 表示类型失配");
if (scopeType.entries.length !== 10) throw new Error(`ScopeType 条数 ${scopeType.entries.length} != 10`);

const vai = tqEnum("VariableAllocationInfo", "NONE,STACK,CONTEXT,UNUSED");
const varMode = tqEnum("VariableMode", "kLet,kConst,kUsing,kAwaitUsing,kVar,kTemporary,kDynamic,kDynamicGlobal,kDynamicLocal,kPrivateMethod,kPrivateSetterOnly,kPrivateGetterOnly,kPrivateGetterAndSetter");
if (varMode.repr !== "int32") throw new Error("VariableMode 表示类型失配");
const initFlag = tqEnum("InitializationFlag", "kNeedsInitialization,kCreatedInitialized");
const isStatic = tqEnum("IsStaticFlag", "kNotStatic,kStatic");
const maybeAssigned = tqEnum("MaybeAssignedFlag", "kNotAssigned,kMaybeAssigned");

// ── tq 侧: bitfield struct 位段表 (机械展开为 显式 shift/width 常量) ─────────
function tqBitfield(name, repr) {
  const re = new RegExp(`bitfield struct ${name} extends ${repr}\\s*\\{([\\s\\S]*?)\\n\\}`);
  const m = tqSrc.match(re);
  if (!m) throw new Error(`未找到 tq bitfield struct ${name}`);
  const segs = [];
  let pos = 0;
  for (const raw of m[1].split("\n")) {
    const line = raw.replace(/\/\/.*$/, "").trim();
    if (!line) continue;
    const fm = line.match(/^(\w+):\s*([\w|]+):\s*(\d+) bit;$/);
    if (!fm) throw new Error(`${name} 位段行不可解析: ${line}`);
    segs.push({ name: fm[1], type: fm[2], width: parseInt(fm[3], 10), start: pos });
    pos += parseInt(fm[3], 10);
  }
  return { segs, total: pos };
}

const scopeFlags = tqBitfield("ScopeFlags", "uint32");
if (scopeFlags.segs.length !== 22) throw new Error(`ScopeFlags 段数 ${scopeFlags.segs.length} != 22`);
// uint32 载体, 31 位在用 (bit31 预留)
if (scopeFlags.total !== 31) throw new Error(`ScopeFlags 总宽 ${scopeFlags.total} != 31`);
// 位段类型引用的枚举须已提取 (ScopeType/VariableAllocationInfo/FunctionKind/LanguageMode)
for (const s of scopeFlags.segs) {
  if (!["bool", "ScopeType", "VariableAllocationInfo", "LanguageMode", "FunctionKind", "uint32"].includes(s.type)) {
    throw new Error(`ScopeFlags 引用未提取类型: ${s.type}`);
  }
}

const varProps = tqBitfield("VariableProperties", "uint31");
if (varProps.segs.length !== 6) throw new Error(`VariableProperties 段数 ${varProps.segs.length} != 6`);
if (varProps.total !== 24) throw new Error(`VariableProperties 总宽 ${varProps.total} != 24`);

const pascal = (s) => s.replace(/^k/, "").toLowerCase().replace(/(^|_)([a-z0-9])/g, (_, p, c) => c.toUpperCase());
const SCREAMING = (s) => s.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

function emitTqEnum(rustName, en) {
  let s = `/// 原版 tq \`extern enum ${rustName} extends ${en.repr}\`, 共 ${en.entries.length} 变体。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ${rustName} {
`;
  en.entries.forEach((e, i) => {
    rustName;
    s += `  /// ${e}\n  ${pascal(e)} = ${i},\n`;
  });
  s += `}

impl ${rustName} {
  /// 变体总数
  pub const COUNT: usize = ${en.entries.length};

  /// 数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(v: i32) -> Option<Self> {
    match v {
`;
  en.entries.forEach((e, i) => {
    s += `      ${i} => Some(Self::${pascal(e)}),\n`;
  });
  s += `      _ => None,
    }
  }
}

`;
  return s;
}

function emitBitfield(name, bf) {
  let s = `/// 原版 tq \`bitfield struct ${name}\` 位段表 (顺序 1:1, 共 ${bf.segs.length} 段 / ${bf.total} 位)。
pub mod ${snake(name)} {
`;
  for (const seg of bf.segs) {
    s += `  /// ${seg.name}: ${seg.type} (bit ${seg.start}..${seg.start + seg.width - 1})。
  pub const ${seg.name.toUpperCase()}_SHIFT: u32 = ${seg.start};
  pub const ${seg.name.toUpperCase()}_WIDTH: u32 = ${seg.width};
  pub const ${seg.name.toUpperCase()}_MASK: u32 = 0x${(((1n << BigInt(seg.width)) - 1n) << BigInt(seg.start)).toString(16)};
`;
  }
  s += `}

`;
  return s;
}

function snake(s) {
  return s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
}

let rust = banner("src/objects/scope-info.h.js", "src/objects/scope-info.{h,-inl.h,cc,tq}", "ScopeInfo 平铺索引 + tq 枚举全套 + 位段表");
rust += `//! ScopeInfo: 作用域元数据。
//! scope-info.cc 语义面 (变量解析/创建逻辑, >2k 行) 登记跳过; 本产物仅规则区。

// ── h 侧: 平铺字段索引与引导类型 ────────────────────────────────────────────

/// 平铺字段索引 (原版 enum Fields; 0..4 固定头, >= kVariablePartStart 编码变长尾)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Fields {
`;
fields.forEach((f, i) => {
  rust += `  /// ${f}\n  ${pascal(f)} = ${i},\n`;
});
rust += `}

/// 变长区起点索引。
pub const VARIABLE_PART_START: u8 = ${fields.indexOf("kVariablePartStart")};

/// 变量初始化位置最大可存距离 (超出落 kMaxInt)。
pub const MAX_VARIABLE_POSITION_DISTANCE: u32 = 0xFFFF;

/// 引导创建类型 (原版 enum class BootstrappingType)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BootstrappingType {
`;
bootEntries.forEach((b, i) => {
  rust += `  /// ${b}\n  ${pascal(b)} = ${i},\n`;
});
rust += `}

// ── tq 侧: 枚举全套 ─────────────────────────────────────────────────────────

`;
rust += emitTqEnum("ScopeType", scopeType);
rust += emitTqEnum("VariableAllocationInfo", vai);
rust += emitTqEnum("VariableMode", varMode);
rust += emitTqEnum("InitializationFlag", initFlag);
rust += emitTqEnum("IsStaticFlag", isStatic);
rust += emitTqEnum("MaybeAssignedFlag", maybeAssigned);

rust += emitBitfield("ScopeFlags", scopeFlags);
rust += emitBitfield("VariableProperties", varProps);

const lastFlagsSeg = scopeFlags.segs[scopeFlags.segs.length - 1];
const lastPropsSeg = varProps.segs[varProps.segs.length - 1];
rust += `// —— 提取不变式断言 ——
const _: () = {
  assert!(VARIABLE_PART_START == 5);
  assert!(Fields::PositionInfoEnd as u8 == 4);
  assert!(MAX_VARIABLE_POSITION_DISTANCE == 0xFFFF);
  assert!(BootstrappingType::ShadowRealm as u8 == 3);
  assert!(ScopeType::COUNT == 10);
  assert!(ScopeType::ScriptScope as i32 == 0);
  assert!(ScopeType::ShadowRealmScope as i32 == 9);
  assert!(VariableMode::COUNT == 13);
  assert!(VariableAllocationInfo::COUNT == 4);
  assert!(ScopeFlags::SCOPE_TYPE_MASK == 0xF);
  assert!(ScopeFlags::${lastFlagsSeg.name.toUpperCase()}_SHIFT == ${lastFlagsSeg.start});
  assert!(VariableProperties::PARAMETER_NUMBER_OR_POSITION_WIDTH == 16);
  assert!(VariableProperties::${lastPropsSeg.name.toUpperCase()}_SHIFT == ${lastPropsSeg.start});
};
`;

const p = writeSrc("v8-objects-rest", "scope_info.rs", rust);
console.log(`scope_info.rs: Fields 6 + tq 枚举 6 套 (${scopeType.entries.length}+${vai.entries.length}+${varMode.entries.length}+${initFlag.entries.length}+${isStatic.entries.length}+${maybeAssigned.entries.length}) + 位段表 22+6 段, 对数通过`);
