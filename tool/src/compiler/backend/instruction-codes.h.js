#!/usr/bin/env bun
// tool/src/compiler/backend/instruction-codes.h.js
// 机械提取 v8/src/compiler/backend/instruction-codes.{h} + x64/arm64 变体
// → v8-compiler-backend/src/instruction_codes.rs + instruction_codes_x64.rs
//   + instruction_codes_arm64.rs
//
// 提取面: ArchOpcode (COMMON + TARGET 按 x64/arm64 分别展开)、AddressingMode
// (None + TARGET)、FlagsMode/FlagsCondition、RecordWriteMode 及
// WriteBarrierKindToRecordWriteMode 映射、MemoryAccessMode/AtomicWidth/LaneSize/
// VectorLength、InstructionCode 位域布局常量、memory-access opcode 集。
//
// 目标配置: V8_ENABLE_WEBASSEMBLY=1, V8_ENABLE_SANDBOX=1,
// V8_ENABLE_SANDBOX_HARDWARE_SUPPORT=0 (与 operations.h.js 对齐)
import { readV8, writeSrc, banner } from "../../../lib.js";

const COMMON_H = readV8("src/compiler/backend/instruction-codes.h");
const X64_H = readV8("src/compiler/backend/x64/instruction-codes-x64.h");
const ARM64_H = readV8("src/compiler/backend/arm64/instruction-codes-arm64.h");

const IF_FLAGS = {
  IF_WASM: true,
  IF_SANDBOX: true,
  IF_HARDWARE_SANDBOX: false,
  IF_SIMD128: true,
};

// ---------------------------------------------------------------------------
// 通用宏展开器
// ---------------------------------------------------------------------------
const CONT_LINE = "(?:[^\\n]*\\\\\\n)*[^\\n]*";
function macroBody(src, name) {
  const re = new RegExp(`#define ${name}\\(V\\)${CONT_LINE}`, "g");
  const defs = [...src.matchAll(re)].map((d) => d[0]);
  if (defs.length === 0) throw new Error(`未找到宏 ${name}`);
  const nonEmpty = defs.filter((d) => /V\(\w+\)/.test(d));
  return nonEmpty[0] ?? defs[0];
}
function expandList(src, name) {
  const names = [];
  const body = macroBody(src, name).replace(/^#define [^\n]*/, "");
  for (const m of body.matchAll(/V\((\w+)\)/g)) names.push(m[1]);
  for (const m of body.matchAll(/IF_(\w+)\(V,\s*(\w+)\)/g)) {
    const flag = IF_FLAGS[`IF_${m[1]}`];
    if (flag === undefined) throw new Error(`未登记 IF 宏: IF_${m[1]}`);
    if (flag) names.push(m[2]);
  }
  for (const m of body.matchAll(/(\w+_LIST)\(V\)/g)) {
    if (m[1] === name) continue;
    names.push(...expandList(src, m[1]));
  }
  return names;
}
function enumOf(src, re, label, expect) {
  const m = src.match(re);
  if (!m) throw new Error(`未找到 ${label}`);
  const names = [...m[1].replace(/\/\/[^\n]*/g, "").matchAll(/\bk(\w+)\b/g)].map((x) => x[1]);
  if (expect && names.length !== expect) throw new Error(`${label} ${names.length} != ${expect}`);
  return names;
}

// ---------------------------------------------------------------------------
// 1. 公共枚举
// ---------------------------------------------------------------------------
const recordWriteModes = enumOf(COMMON_H, /enum class RecordWriteMode\s*\{([^}]+)\};/, "RecordWriteMode", 5);
const flagsModes = enumOf(COMMON_H, /enum FlagsMode\s*\{([^}]+)\};/, "FlagsMode", 8);
const flagsConditions = enumOf(COMMON_H, /enum FlagsCondition : uint8_t\s*\{([^}]+)\};/, "FlagsCondition", 26);
const laneSizes = enumOf(COMMON_H, /enum class LaneSize : uint8_t\s*\{([^}]+)\};/, "LaneSize", 4);
const vectorLengths = enumOf(COMMON_H, /enum class VectorLength : uint8_t\s*\{([^}]+)\};/, "VectorLength", 4);

// WriteBarrierKindToRecordWriteMode 映射
const wbMap = new Map();
const wbFn = COMMON_H.match(/inline RecordWriteMode WriteBarrierKindToRecordWriteMode\([\s\S]*?\n\}/);
if (!wbFn) throw new Error("未找到 WriteBarrierKindToRecordWriteMode");
for (const m of wbFn[0].matchAll(/case (k\w+):\s*return RecordWriteMode::k(\w+);/g)) {
  wbMap.set(m[1], m[2]);
}
if (wbMap.size !== 5) throw new Error(`WriteBarrier 映射 ${wbMap.size} != 5`);

// 位域布局 (from using XField = ...BitField<T, offset, width>)
const fields = [];
for (const m of COMMON_H.matchAll(/using (\w+Field) = (?:base::BitField<\w+, (\d+), (\d+)>|(\w+)::Next<[^,]+, (\d+)>)/g)) {
  fields.push({ name: m[1], prev: m[4] ?? null, offset: m[2] ? Number(m[2]) : null, width: Number(m[5] ?? m[3]) });
}
if (fields.length < 12) throw new Error(`位域 ${fields.length} < 12`);
// 计算偏移链
const fieldByName = new Map(fields.map((f) => [f.name, f]));
const resolved = new Map();
function offsetOf(f) {
  if (resolved.has(f.name)) return resolved.get(f.name);
  const off = f.prev ? (offsetOf(fieldByName.get(f.prev)) + fieldByName.get(f.prev).width) : f.offset;
  if (Number.isNaN(off)) throw new Error(`位域偏移解析失败: ${f.name}`);
  resolved.set(f.name, off);
  return off;
}
for (const f of fields) offsetOf(f);
// 布局哨兵 (注释文档: ArchOpcode|10, AddressingMode|5, FlagsMode|3, FlagsCondition|5, Misc|9)
for (const [n, o] of [["ArchOpcodeField", 0], ["AddressingModeField", 10], ["FlagsModeField", 15], ["FlagsConditionField", 18], ["MiscField", 23]]) {
  if (resolved.get(n) !== o) throw new Error(`位域 ${n} 偏移 ${resolved.get(n)} != ${o}`);
}

// 常量
const stackCondM = COMMON_H.match(/static constexpr FlagsCondition kStackPointerGreaterThanCondition =\s*k(\w+);/);
if (!stackCondM) throw new Error("未找到 kStackPointerGreaterThanCondition");
const lazyDeoptM = COMMON_H.match(/static constexpr int kLazyDeoptOnThrowSentinel = (-?\d+);/);
if (!lazyDeoptM || Number(lazyDeoptM[1]) !== -1) throw new Error("kLazyDeoptOnThrowSentinel 异常");

// ---------------------------------------------------------------------------
// 2. 架构侧
// ---------------------------------------------------------------------------
function archData(src, archLabel, prefix) {
  const targetOpcodes = expandList(src, "TARGET_ARCH_OPCODE_LIST");
  const commonMemory = expandList(COMMON_H, "COMMON_ARCH_OPCODE_WITH_MEMORY_ACCESS_MODE_LIST");
  const common = expandList(COMMON_H, "COMMON_ARCH_OPCODE_LIST");
  const archMem = expandList(src, "TARGET_ARCH_OPCODE_WITH_MEMORY_ACCESS_MODE_LIST");
  const archOpcodes = [...common, ...targetOpcodes];
  const addressing = ["None", ...expandList(src, "TARGET_ADDRESSING_MODE_LIST")];
  // 去重
  const seen = new Set();
  for (const n of archOpcodes) {
    if (seen.has(n)) throw new Error(`${archLabel} ArchOpcode 重复: ${n}`);
    seen.add(n);
  }
  const memSet = new Set([...commonMemory, ...archMem]);
  for (const n of memSet) {
    if (!seen.has(n)) throw new Error(`${archLabel} memory-access opcode 不在 ArchOpcode 表: ${n}`);
  }
  return { archOpcodes, addressing, memSet };
}
const x64 = archData(X64_H, "x64");
const arm64 = archData(ARM64_H, "arm64");
// 对数: 尾哨兵 (x64 表尾 / arm64 表尾从原文核对首尾项)
if (!x64.archOpcodes[9].startsWith("Arch") && !x64.archOpcodes[9].startsWith("Ieee")) {
  throw new Error(`x64 公共段尾异常: ${x64.archOpcodes[9]}`);
}
if (!arm64.archOpcodes[arm64.archOpcodes.length - 1].startsWith("Arm64")) {
  throw new Error(`arm64 表尾异常: ${arm64.archOpcodes[arm64.archOpcodes.length - 1]}`);
}
if (x64.addressing.length < 20) throw new Error(`x64 addressing ${x64.addressing.length} < 20`);
if (arm64.addressing.length < 10) throw new Error(`arm64 addressing ${arm64.addressing.length} < 10`);

// ---------------------------------------------------------------------------
// 3. 生成公共文件
// ---------------------------------------------------------------------------
function enumText(enumName, names, { explicitValue = false, repr = "u8", doc } = {}) {
  let s = `/// ${doc}\n#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n#[repr(${repr})]\npub enum ${enumName} {\n`;
  names.forEach((n, i) => {
    s += `  /// k${n}\n  ${n}${explicitValue ? ` = ${i}` : ""},\n`;
  });
  s += "}\n";
  return s;
}

let rust = banner(
  "src/compiler/backend/instruction-codes.h.js",
  "src/compiler/backend/instruction-codes.h (公共面)",
  "指令码公共面: FlagsMode/FlagsCondition/位域布局/RecordWriteMode",
);

rust += `
/// 原版 InstructionCode — 不透明 32 位编码 (u32)
pub type InstructionCode = u32;

/// 原版 kLazyDeoptOnThrowSentinel
pub const LAZY_DEOPT_ON_THROW_SENTINEL: i32 = ${lazyDeoptM[1]};

`;
rust += enumText("RecordWriteMode", recordWriteModes, { explicitValue: true, doc: "原版 RecordWriteMode — 写屏障存储模式" });
rust += `
impl RecordWriteMode {
  /// 原版 WriteBarrierKindToRecordWriteMode (kNoWriteBarrier 不应传入)
  #[must_use]
  pub fn from_write_barrier_kind(kind: &str) -> Option<Self> {
    match kind {
`;
for (const [from, to] of wbMap) {
  rust += `      "${from}" => Some(RecordWriteMode::${to}),\n`;
}
rust += `      _ => None,
    }
  }
}
`;
const flagsModeNames = flagsModes.map((n) => {
  if (n.startsWith("Flags_")) {
    const raw = n.slice("Flags_".length);
    return raw.split("_").map((part) => part.charAt(0).toUpperCase() + part.slice(1)).join("");
  }
  return n;
});
rust += "\n" + enumText("FlagsMode", flagsModeNames, { explicitValue: true, doc: "原版 FlagsMode — 标志位延续模式 (显式值)" });
rust += "\n" + enumText("FlagsCondition", flagsConditions, { explicitValue: true, doc: "原版 FlagsCondition — 标志位条件" });
rust += `
impl FlagsCondition {
  /// 原版 kStackPointerGreaterThanCondition
  pub const STACK_POINTER_GREATER_THAN_CONDITION: FlagsCondition = FlagsCondition::${stackCondM[1]};

  /// 从数值反查 (原版 static_cast 语义)
  #[inline]
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
${flagsConditions.map((n, i) => `    if val == ${i} { return FlagsCondition::${n}; }`).join("\n")}
    FlagsCondition::Equal
  }

  /// 原版 NegateFlagsCondition — 条件取反 (异或 1)
  #[inline]
  #[must_use]
  pub const fn negate(self) -> Self {
    Self::from_repr((self as u8) ^ 1)
  }
}
`;
rust += `
/// 原版 MemoryAccessMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MemoryAccessMode {
  /// kMemoryAccessDirect
  Direct = 0,
  /// kMemoryAccessTrapping
  Trapping = 1,
}

/// 原版 AtomicWidth (64 位架构双宽; 32 位全按 32 处理)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AtomicWidth {
  /// kWord32 (尺寸 4)
  Word32 = 0,
  /// kWord64 (尺寸 8)
  Word64 = 1,
}

impl AtomicWidth {
  /// 原版 AtomicWidthSize
  #[inline]
  #[must_use]
  pub const fn size(self) -> usize {
    match self {
      AtomicWidth::Word32 => 4,
      AtomicWidth::Word64 => 8,
    }
  }
}
`;
// LaneSize / VectorLength (显式值)
rust += `
/// 原版 LaneSize — SIMD 通道宽
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LaneSize {
`;
for (let i = 0; i < laneSizes.length; i++) {
  const n = laneSizes[i];
  const bits = n.slice(1); // L8 → 8
  rust += `  /// k${n} (位宽 ${bits})\n  ${n} = ${i},\n`;
}
rust += `}

impl LaneSize {
  /// 原版 LaneSizeBits
  #[inline]
  #[must_use]
  pub const fn bits(self) -> i32 {
    match self {
`;
for (const n of laneSizes) {
  rust += `      LaneSize::${n} => ${n.slice(1)},\n`;
}
rust += `    }
  }

  /// 原版 LaneSizeFromBits
  #[must_use]
  pub const fn from_bits(width: u8) -> Option<Self> {
    match width {
`;
for (const n of laneSizes) {
  rust += `      ${n.slice(1)} => Some(LaneSize::${n}),\n`;
}
rust += `      _ => None,
    }
  }
}

/// 原版 VectorLength
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VectorLength {
`;
for (let i = 0; i < vectorLengths.length; i++) {
  const n = vectorLengths[i];
  rust += `  /// k${n}\n  ${n} = ${i},\n`;
}
rust += `}

impl VectorLength {
  /// 原版 VectorLengthBits
  #[inline]
  #[must_use]
  pub const fn bits(self) -> i32 {
    match self {
`;
for (const n of vectorLengths) {
  rust += `      VectorLength::${n} => ${n.slice(1)},\n`;
}
rust += `    }
  }
}

/// 原版 InstructionCode 位域布局 (低 23 位五字段 + 9 位 Misc 覆盖区):
/// - ArchOpcodeField: offset ${resolved.get("ArchOpcodeField")}, 宽 ${fieldByName.get("ArchOpcodeField").width}
/// - AddressingModeField: offset ${resolved.get("AddressingModeField")}, 宽 ${fieldByName.get("AddressingModeField").width}
/// - FlagsModeField: offset ${resolved.get("FlagsModeField")}, 宽 ${fieldByName.get("FlagsModeField").width}
/// - FlagsConditionField: offset ${resolved.get("FlagsConditionField")}, 宽 ${fieldByName.get("FlagsConditionField").width}
/// - MiscField: offset ${resolved.get("MiscField")}, 宽 ${fieldByName.get("MiscField").width}
///
/// MiscField 覆盖区按指令类再分:
${fields.filter((f) => f.prev === "FlagsConditionField" || ["RecordWriteModeField", "AtomicWidthField", "AtomicMemoryOrderField", "AccessModeField", "LaneSizeField", "VectorLengthField", "SharedBaseField", "DeoptImmedArgsCountField", "DeoptFrameStateOffsetField", "StackCheckField", "BranchHintField"].includes(f.name)).map((f) => `/// - ${f.name}: offset ${resolved.get(f.name)}, 宽 ${f.width}`).join("\n")}
pub mod code_fields {
${fields.map((f) => {
  const constName = f.name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").replace(/_?Field$/, "_FIELD").toUpperCase();
  return `  /// 原版 ${f.name} — offset ${resolved.get(f.name)}, 宽 ${f.width}
  pub const ${constName}: (u32, u32) = (${resolved.get(f.name)}, ${f.width});`;
}).join("\n")}
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(FlagsMode::None as u8 == 0);
  assert!(FlagsMode::ConditionalBranch as u8 == 7);
  assert!(FlagsCondition::Equal as u8 == 0);
  assert!(FlagsCondition::IsNotNaN as u8 == 25);
  assert!(FlagsCondition::Equal.negate() as u8 == FlagsCondition::NotEqual as u8);
  assert!(FlagsCondition::Overflow.negate() as u8 == FlagsCondition::NotOverflow as u8);
  assert!(RecordWriteMode::ValueIsMap as u8 == 0);
  assert!(AtomicWidth::Word64.size() == 8);
  assert!(LaneSize::L8.bits() == 8);
  assert!(VectorLength::V512.bits() == 512);
  assert!(code_fields::ARCH_OPCODE_FIELD.0 == 0);
  assert!(code_fields::MISC_FIELD.0 == 23 && code_fields::MISC_FIELD.1 == 9);
};
`;

writeSrc("v8-compiler-backend", "instruction_codes.rs", rust);

// ---------------------------------------------------------------------------
// 4. 架构文件
// ---------------------------------------------------------------------------
function archRust(archLabel, snake, data, v8rel) {
  let s = banner(
    "src/compiler/backend/instruction-codes.h.js",
    v8rel,
    `${archLabel} 指令码表: ArchOpcode (公共+架构) 与 AddressingMode`,
  );
  s += `
// 公共面见 instruction_codes.rs; kLastArchOpcode/kLastAddressingMode = 条数-1

/// 原版 ArchOpcode (${archLabel}, 公共 ${data.archOpcodes.length - expandListLen(v8relPrefix(archLabel))} + 架构项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
#[rustfmt::skip]
pub enum ArchOpcode {
`;
  data.archOpcodes.forEach((n, i) => {
    s += `  /// k${n}\n  ${n} = ${i},\n`;
  });
  s += `}

/// 原版 kLastArchOpcode (条数 - 1)
pub const LAST_ARCH_OPCODE: u16 = ${data.archOpcodes.length - 1};

/// 原版 HasMemoryAccessMode 集合 (COMMON+TARGET memory-access 列表)
pub const HAS_MEMORY_ACCESS_MODE: &[ArchOpcode] = &[
`;
  for (const n of data.memSet) {
    s += `  ArchOpcode::${n},\n`;
  }
  s += `];

/// 原版 AddressingMode (None + TARGET_ADDRESSING_MODE_LIST)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
#[rustfmt::skip]
pub enum AddressingMode {
`;
  data.addressing.forEach((n, i) => {
    let varName = "Mode" + n;
    if (n.includes("_")) {
      const parts = n.split("_");
      varName = "Mode" + parts.map((p) => p.charAt(0).toUpperCase() + p.slice(1).toLowerCase()).join("");
    }
    s += `  /// kMode_${n}\n  ${varName} = ${i},\n`;
  });
  s += `}

/// 原版 kLastAddressingMode (条数 - 1)
pub const LAST_ADDRESSING_MODE: u16 = ${data.addressing.length - 1};

// —— 提取不变式断言 ——
const _: () = {
  assert!(LAST_ARCH_OPCODE as u32 >= 100);
  assert!(ArchOpcode::ArchJmp as u16 >= 8);
  assert!(AddressingMode::ModeNone as u16 == 0);
  assert!(!HAS_MEMORY_ACCESS_MODE.is_empty());
};
`;
  return s;
}
// 架构项计数辅助 (公共段 = COMMON_ARCH_OPCODE_LIST 展开数)
function expandListLen(prefix) {
  const src = prefix === "x64" ? X64_H : ARM64_H;
  return expandList(COMMON_H, "COMMON_ARCH_OPCODE_LIST").length;
}
function v8relPrefix(archLabel) {
  return archLabel === "x64" ? "x64" : "arm64";
}

writeSrc("v8-compiler-backend", "instruction_codes_x64.rs", archRust("x64", "x64", x64, "src/compiler/backend/x64/instruction-codes-x64.h + instruction-codes.h"));
writeSrc("v8-compiler-backend", "instruction_codes_arm64.rs", archRust("arm64", "arm64", arm64, "src/compiler/backend/arm64/instruction-codes-arm64.h + instruction-codes.h"));

console.log(`written instruction_codes.rs / instruction_codes_x64.rs / instruction_codes_arm64.rs`);
console.log(`FlagsMode=${flagsModes.length} FlagsCondition=${flagsConditions.length} RecordWriteMode=${recordWriteModes.length} 位域=${fields.length}`);
console.log(`x64: ArchOpcode=${x64.archOpcodes.length} AddrMode=${x64.addressing.length} memAccess=${x64.memSet.size}`);
console.log(`arm64: ArchOpcode=${arm64.archOpcodes.length} AddrMode=${arm64.addressing.length} memAccess=${arm64.memSet.size}`);
