#!/usr/bin/env bun
// tool/src/wasm/wasm-module.h.js
// 机械提取 v8/src/wasm/wasm-module.h 中的枚举与布局常量
// (AddressType / BoundsCheckStrategy / WasmElemSegment::Status,ElementType /
//  AdaptiveMap / TypeDefinition::Kind / WasmDebugSymbols::Type / tier-up 哨兵)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-module.h");

function must(re, what) {
  const m = hSrc.match(re);
  if (!m) throw new Error(`未找到 ${what}`);
  return m;
}

// 1. AddressType
const addrMatch = must(/enum class AddressType : uint8_t \{([^}]*)\};/, "AddressType");
const addrTypes = addrMatch[1]
  .split(",")
  .map((s) => s.replace(/\/\/.*$/, "").trim())
  .filter(Boolean);
if (addrTypes.join(",") !== "kI32,kI64") {
  throw new Error(`AddressType 内容不符: ${addrTypes}`);
}

// 2. BoundsCheckStrategy
const bcsMatch = must(/enum BoundsCheckStrategy : int8_t \{([\s\S]*?)\n\};/, "BoundsCheckStrategy");
const bcsComments = [];
const bcs = [];
for (const rawLine of bcsMatch[1].split("\n")) {
  const line = rawLine.trim();
  if (!line) continue;
  const cmt = line.match(/^\/\/\s?(.*)$/);
  if (cmt) {
    bcsComments.push(cmt[1]);
    continue;
  }
  const m = line.match(/^(k[A-Za-z0-9]+),?$/);
  if (!m) throw new Error(`无法解析 BoundsCheckStrategy 行: "${line}"`);
  bcs.push({ name: m[1], doc: bcsComments.join(" ") });
  bcsComments.length = 0;
}
if (bcs.length !== 3 || bcs[0].name !== "kTrapHandler" || bcs[2].name !== "kNoBoundsChecks") {
  throw new Error(`BoundsCheckStrategy 条数/内容不符: ${bcs.map((b) => b.name)}`);
}

// 3. WasmElemSegment::Status / ElementType
const statusMatch = must(/enum Status \{([^}]*)\};/, "WasmElemSegment::Status");
const statuses = statusMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (statuses.length !== 3) throw new Error(`Status 条数不符: ${statuses.length}`);

const elemMatch = must(/enum ElementType \{([^}]*)\};/, "WasmElemSegment::ElementType");
const elemTypes = elemMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (elemTypes.length !== 2) throw new Error(`ElementType 条数不符: ${elemTypes.length}`);

// 4. AdaptiveMap 常量
const maxKeyMatch = must(/static constexpr uint32_t kMaxKey = ([0-9']+);/, "AdaptiveMap::kMaxKey");
const maxKey = parseInt(maxKeyMatch[1].replace(/'/g, ""), 10);
if (maxKey !== 10_000_000) throw new Error(`kMaxKey 不符: ${maxKey}`);

const loadFactorMatch = must(/static constexpr uint32_t kLoadFactor = (\d+);/, "AdaptiveMap::kLoadFactor");
const loadFactor = parseInt(loadFactorMatch[1], 10);
if (loadFactor !== 4) throw new Error(`kLoadFactor 不符: ${loadFactor}`);

// 5. TypeDefinition::Kind (值取自 RefTypeKind, 见 value_type.h/value_type.rs)
const kindMatch = must(/enum Kind : int8_t \{([\s\S]*?)\n  \};/, "TypeDefinition::Kind");
const kindRefs = [...kindMatch[1].matchAll(/k(\w+) = static_cast<int8_t>\(RefTypeKind::k(\w+)\)/g)];
if (kindRefs.length !== 4) throw new Error(`TypeDefinition::Kind 条数不符: ${kindRefs.length}`);
const refTypeKind = { Function: 1, Struct: 2, Array: 3, Cont: 4 };
for (const [, name, ref] of kindRefs) {
  if (name !== ref) throw new Error(`Kind 名称与 RefTypeKind 不对应: ${name} vs ${ref}`);
}

// 6. WasmDebugSymbols
const numTypesMatch = must(/static constexpr int kNumTypes = (\d+);/, "WasmDebugSymbols::kNumTypes");
const numTypes = parseInt(numTypesMatch[1], 10);
if (numTypes !== 3) throw new Error(`kNumTypes 不符: ${numTypes}`);

const dbgTypeMatch = must(/enum Type \{ (\w+, \w+, \w+, \w+) \};/, "WasmDebugSymbols::Type");
const dbgTypes = dbgTypeMatch[1].split(",").map((s) => s.trim());
if (dbgTypes.join(",") !== "SourceMap,EmbeddedDWARF,ExternalDWARF,None") {
  throw new Error(`WasmDebugSymbols::Type 不符: ${dbgTypes}`);
}

// 7. tier-up / 帧大小哨兵 (WasmModule 尾部)
const frameSizeMatch = must(/static constexpr uint32_t kUninitializedLiftoffFrameSize = (\d+);/, "kUninitializedLiftoffFrameSize");
const frameSize = parseInt(frameSizeMatch[1], 10);
const callRefMatch = must(/static constexpr uint32_t kCallRef = (0x[0-9A-Fa-f]+);/, "kCallRef");
const callRef = parseInt(callRefMatch[1], 16);
if (callRef !== 0xffffffff) throw new Error(`kCallRef 不符: ${callRef}`);
const callIndirect = callRef - 1;

console.log(
  `wasm-module.h: AddressType=${addrTypes.length} BoundsCheckStrategy=${bcs.length} ` +
    `Status=${statuses.length} ElementType=${elemTypes.length} Kind=${kindRefs.length} ` +
    `DebugSymbols::Type=${dbgTypes.length}`,
);

// 8. 生成 Rust
let rust = banner(
  "src/wasm/wasm-module.h.js",
  "src/wasm/wasm-module.h",
  "WasmModule 相关枚举与哨兵常量",
);

rust += `
/// 内存地址类型 (对应原版 enum class AddressType : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AddressType {
  /// kI32
  I32 = 0,
  /// kI64
  I64 = 1,
}

impl AddressType {
  /// 原版 AddressTypeToStr
  #[inline]
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
      Self::I32 => "i32",
      Self::I64 => "i64",
    }
  }
}

/// 边界检查策略 (对应原版 enum BoundsCheckStrategy : int8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum BoundsCheckStrategy {
`;

for (const [i, b] of bcs.entries()) {
  rust += `  /// ${b.name}${b.doc ? ` — ${b.doc}` : ""}\n  ${b.name.replace(/^k/, "")} = ${i},\n`;
}

rust += `}

/// 元素段状态 (对应原版 WasmElemSegment::Status)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ElemSegmentStatus {
`;

for (const [i, s] of statuses.entries()) {
  rust += `  /// ${s}\n  ${s.replace(/^kStatus/, "")} = ${i},\n`;
}

rust += `}

/// 元素段元素形态 (对应原版 WasmElemSegment::ElementType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ElementType {
`;

for (const [i, e] of elemTypes.entries()) {
  rust += `  /// ${e}\n  ${e.replace(/Elements$/, "")} = ${i},\n`;
}

rust += `}

/// 自适应映射最大键 (对应原版 AdaptiveMap::kMaxKey; index+1 不得溢出)
pub const ADAPTIVE_MAP_MAX_KEY: u32 = ${maxKey};

/// 自适应映射装填因子 (对应原版 AdaptiveMap::kLoadFactor)
pub const ADAPTIVE_MAP_LOAD_FACTOR: u32 = ${loadFactor};

/// 类型定义类别 (对应原版 TypeDefinition::Kind; 数值取自 RefTypeKind)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum TypeDefinitionKind {
  /// kFunction
  Function = ${refTypeKind.Function},
  /// kStruct
  Struct = ${refTypeKind.Struct},
  /// kArray
  Array = ${refTypeKind.Array},
  /// kCont
  Cont = ${refTypeKind.Cont},
}

/// 调试符号类别总数 (对应原版 WasmDebugSymbols::kNumTypes)
pub const DEBUG_SYMBOLS_NUM_TYPES: i32 = ${numTypes};

/// 调试符号类别 (对应原版 WasmDebugSymbols::Type)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DebugSymbolsType {
  /// SourceMap
  SourceMap = 0,
  /// EmbeddedDWARF
  EmbeddedDwarf = 1,
  /// ExternalDWARF
  ExternalDwarf = 2,
  /// None
  None = 3,
}

/// 未初始化的 Liftoff 帧大小哨兵 (原版 kUninitializedLiftoffFrameSize)
pub const UNINITIALIZED_LIFTOFF_FRAME_SIZE: u32 = ${frameSize};

/// call_ref 直接调用哨兵函数索引 (原版 kCallRef)
pub const CALL_REF_SENTINEL: u32 = ${callRef};

/// call_indirect 直接调用哨兵函数索引 (原版 kCallIndirect = kCallRef - 1)
pub const CALL_INDIRECT_SENTINEL: u32 = ${callIndirect};

// —— 提取不变式断言 ——
const _: () = {
  assert!(ADAPTIVE_MAP_MAX_KEY == 10_000_000);
  assert!(ADAPTIVE_MAP_LOAD_FACTOR == 4);
  assert!(CALL_REF_SENTINEL == 0xFFFF_FFFF);
  assert!(CALL_INDIRECT_SENTINEL == CALL_REF_SENTINEL - 1);
  assert!(UNINITIALIZED_LIFTOFF_FRAME_SIZE == 1);
  assert!(TypeDefinitionKind::Function as i8 == 1);
  assert!(TypeDefinitionKind::Cont as i8 == 4);
};
`;

const p = writeSrc("v8-wasm-core", "wasm_module_consts.rs", rust);
console.log(`written ${p}`);
