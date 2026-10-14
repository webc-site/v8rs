#!/usr/bin/env bun
// tool/src/interpreter/bytecodes.h.js
// 机械提取 v8/src/interpreter/bytecodes.{h,cc} 的 Bytecode 全表与操作数元表
// → v8-interpreter/src/bytecodes.rs
//
// 对数基准 (clang -E 独立验证, 2026-10-11):
//   BYTECODE_LIST = 217 = BYTECODE_LIST_WITH_UNIQUE_HANDLERS_IMPL(200,
//   含 CALL_PROPERTY_BYTECODES 4) + SHORT_STAR_BYTECODE_LIST(16) + Illegal(1)
//   分类宏: DEBUG_BREAK 7+2 / JUMP_CONDITIONAL_{IMMEDIATE,CONSTANT} 各 11 /
//   JUMP_ALL 25 / RETURN 2 / UNCONDITIONAL_THROW 2 (完整表见 catGuards)
// 默认展开取非 TSA 配置 (V8_ENABLE_EXPERIMENTAL_TSA_BUILTINS 未定义, V_TSA == V);
// V8_TARGET_OS_ANDROID 的 PrefixBytecodeToOperandScale 分支语义等同, 注释登记不展开。
import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/interpreter/bytecodes.h");
const ccSrc = readV8("src/interpreter/bytecodes.cc");

// —— 宏体抽取: 拼续行 + 去注释 ——
function rawMacroBody(src, name) {
  const re = new RegExp(`#define\\s+${name}\\(`);
  const m = re.exec(src);
  if (!m) throw new Error(`宏缺失: ${name}`);
  const allLines = src.slice(m.index).split("\n");
  const head = allLines[0].slice(allLines[0].indexOf(")") + 1);
  const body = [];
  let start = 1;
  if (head.trim()) {
    body.push(head.trimEnd().endsWith("\\") ? head.trimEnd().slice(0, -1) : head);
    if (!head.trimEnd().endsWith("\\")) start = allLines.length;
  }
  for (let i = start; i < allLines.length; i++) {
    const line = allLines[i];
    const cont = line.endsWith("\\");
    body.push(cont ? line.slice(0, -1) : line);
    if (!cont) break;
  }
  return body.join("\n").replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/.*$/gm, "");
}

// —— 条目解析: V(Name, args...) / V_TSA(Name, args...) ——
function parseV(body) {
  const out = [];
  const re = /\bV(?:_TSA)?\s*\(([^()]*)\)/g;
  let m;
  while ((m = re.exec(body)) !== null) {
    out.push(m[1].split(",").map((s) => s.trim()).filter(Boolean));
  }
  return out;
}

// 递归展开: 宏体内对其他 X_LIST(V) 宏的引用就地拼接
const macroCache = new Map();
function expandList(name) {
  if (macroCache.has(name)) return macroCache.get(name);
  const body = rawMacroBody(hSrc, name).replace(
    /\b([A-Z][A-Z_0-9]+)\((V(?:\s*,\s*V)?)\)/g,
    (full, inner, args) => {
      if (inner === "V" || inner === "V_TSA") return full; // 条目本体, 不动
      const parts = expandList(inner).map((e) => `V(${e.join(", ")})`);
      return parts.join("\n");
    },
  );
  const entries = parseV(body);
  macroCache.set(name, entries);
  return entries;
}

function expectList(macro, want) {
  const got = expandList(macro);
  if (got.length !== want) throw new Error(`${macro} 对数不符: 期望 ${want} 实得 ${got.length}`);
  return got;
}

// —— clang -E 基准对数 ——
const shortStarList = expectList("SHORT_STAR_BYTECODE_LIST", 16);
const callPropertyList = expectList("CALL_PROPERTY_BYTECODES", 4);
const implList = expectList("BYTECODE_LIST_WITH_UNIQUE_HANDLERS_IMPL", 200);
const illegal = ["Illegal", "ImplicitRegisterUse::kNone"];
const bytecodes = [...implList, ...shortStarList, illegal];
if (bytecodes.length !== 217) throw new Error(`BYTECODE_LIST 对数不符: 期望 217 实得 ${bytecodes.length}`);

// —— 条目结构化: Name / ImplicitRegisterUse / OperandType 列表 ——
function structEntry(entry) {
  const [name, use, ...operands] = entry;
  if (!/^[A-Za-z_][A-Za-z0-9_]*$/.test(name)) throw new Error(`条目名非法: ${name}`);
  if (use !== undefined && !use.startsWith("ImplicitRegisterUse::k")) {
    throw new Error(`${name} 隐式用法非 ImplicitRegisterUse: ${use}`);
  }
  for (const op of operands) {
    if (!op.startsWith("OperandType::k")) throw new Error(`${name} 操作数类型非法: ${op}`);
  }
  return {
    name,
    rustName: name.replace(/_/g, ""),
    use: use ? use.replace("ImplicitRegisterUse::k", "") : "None",
    operands: operands.map((op) => op.replace("OperandType::k", "")),
  };
}
const table = bytecodes.map(structEntry);
if (table[0].name !== "Wide" || table[1].name !== "ExtraWide") {
  throw new Error("表头锚错位: 期望 Wide/ExtraWide 领头");
}
if (table[200].name !== "Star15" || table[215].name !== "Star0" || table[216].name !== "Illegal") {
  throw new Error(`ShortStar/Illegal 锚错位: got ${table[200].name}/${table[215].name}/${table[216].name}`);
}

// —— 操作数类型 → 元信息 (重新解析 bytecode-operands.h, 与 operand 脚本同源) ——
const opH = readV8("src/interpreter/bytecode-operands.h");
const opSubLists = [
  "INVALID_OPERAND_TYPE_LIST",
  "UNSIGNED_FIXED_SCALAR_OPERAND_TYPE_LIST",
  "UNSIGNED_SCALABLE_SCALAR_OPERAND_TYPE_LIST",
  "SIGNED_SCALABLE_SCALAR_OPERAND_TYPE_LIST",
  "REGISTER_INPUT_OPERAND_TYPE_LIST",
  "REGISTER_OUTPUT_OPERAND_TYPE_LIST",
];
const opEntries = [];
for (const m of opSubLists) {
  opEntries.push(...parseV(rawMacroBody(opH, m)));
}
opEntries.push(["RegInOut", "OperandTypeInfo::kScalableSignedByte"]);
if (opEntries.length !== 23) throw new Error(`OPERAND_TYPE_LIST 对数不符: 期望 23 实得 ${opEntries.length}`);
const typeInfoOf = new Map(opEntries.map(([n, info]) => [n, info.replace("OperandTypeInfo::k", "")]));
const regInputNames = parseV(rawMacroBody(opH, "REGISTER_INPUT_OPERAND_TYPE_LIST")).map((e) => e[0]);
const regOutputNames = parseV(rawMacroBody(opH, "REGISTER_OUTPUT_OPERAND_TYPE_LIST")).map((e) => e[0]);
for (const b of table) {
  for (const op of b.operands) {
    if (!typeInfoOf.has(op)) throw new Error(`${b.name} 引用未知操作数类型: ${op}`);
  }
}

// —— 元信息 → 尺寸/缩放计算 (bytecode-traits.h OperandScaler 公式) ——
const unscaledSize = { None: 0, ScalableSignedByte: 1, ScalableUnsignedByte: 1, FixedUnsignedByte: 1, FixedUnsignedShort: 2 };
const isScalable = (info) => info === "ScalableSignedByte" || info === "ScalableUnsignedByte";
const scales = [1, 2, 4]; // Single/Double/Quadruple
const sized = table.map((b) => ({
  ...b,
  sizes: scales.map((sc) =>
    b.operands.map((op) => {
      const info = typeInfoOf.get(op);
      return unscaledSize[info] * (isScalable(info) ? sc : 1);
    }),
  ),
  offsets: scales.map((sc) => {
    const offs = [];
    let cur = 1;
    for (const op of b.operands) {
      offs.push(cur);
      const info = typeInfoOf.get(op);
      cur += unscaledSize[info] * (isScalable(info) ? sc : 1);
    }
    return offs;
  }),
  total: scales.map((sc) =>
    1 + b.operands.reduce((acc, op) => {
      const info = typeInfoOf.get(op);
      return acc + unscaledSize[info] * (isScalable(info) ? sc : 1);
    }, 0),
  ),
}));

// —— 分类宏展开 (clang 实测条数守卫) ——
const catGuards = {
  DEBUG_BREAK_PLAIN_BYTECODE_LIST: 7,
  DEBUG_BREAK_PREFIX_BYTECODE_LIST: 2,
  DEBUG_BREAK_BYTECODE_LIST: 9,
  JUMP_UNCONDITIONAL_IMMEDIATE_BYTECODE_LIST: 2,
  JUMP_UNCONDITIONAL_CONSTANT_BYTECODE_LIST: 1,
  JUMP_TOBOOLEAN_CONDITIONAL_IMMEDIATE_BYTECODE_LIST: 2,
  JUMP_TOBOOLEAN_CONDITIONAL_CONSTANT_BYTECODE_LIST: 2,
  JUMP_CONDITIONAL_IMMEDIATE_BYTECODE_LIST: 11,
  JUMP_CONDITIONAL_CONSTANT_BYTECODE_LIST: 11,
  JUMP_CONSTANT_BYTECODE_LIST: 12,
  JUMP_IMMEDIATE_BYTECODE_LIST: 13,
  JUMP_TO_BOOLEAN_BYTECODE_LIST: 4,
  JUMP_UNCONDITIONAL_BYTECODE_LIST: 3,
  JUMP_CONDITIONAL_BYTECODE_LIST: 22,
  JUMP_FORWARD_BYTECODE_LIST: 24,
  JUMP_BYTECODE_LIST: 25,
  RETURN_BYTECODE_LIST: 2,
  UNCONDITIONAL_THROW_BYTECODE_LIST: 2,
};
const cat = {};
for (const [m, want] of Object.entries(catGuards)) {
  const entries = expandList(m);
  if (entries.length !== want) throw new Error(`${m} 对数不符: 期望 ${want} 实得 ${entries.length}`);
  const names = entries.map((e) => {
    if (e.length !== 1) throw new Error(`${m} 条目非单参: ${e.join(",")}`);
    return e[0];
  });
  cat[m] = names;
}
const nameSet = new Set(table.map((b) => b.name));
for (const names of Object.values(cat)) {
  for (const n of names) {
    if (!nameSet.has(n)) throw new Error(`分类宏成员不在全表: ${n}`);
  }
}

// 索引查找
const idxOf = (n) => {
  const i = table.findIndex((b) => b.name === n);
  if (i < 0) throw new Error(`查无字节码: ${n}`);
  return i;
};

// GetDebugBreak (bytecodes.cc): Wide/ExtraWide 特判, 其余按单刻度尺寸首个匹配 DEBUG_BREAK_PLAIN
function debugBreakOf(i) {
  const n = table[i].name;
  if (n === "Wide") return "DebugBreakWide";
  if (n === "ExtraWide") return "DebugBreakExtraWide";
  const size = sized[i].total[0];
  for (const dn of cat.DEBUG_BREAK_PLAIN_BYTECODE_LIST) {
    if (sized[idxOf(dn)].total[0] === size) return dn;
  }
  throw new Error(`GetDebugBreak 无匹配: ${n}`);
}
const debugBreakMap = table.map((_, i) => debugBreakOf(i));

// cc 显式 case 表机械抓取
function ccCaseList(fnName, want) {
  const m = ccSrc.match(new RegExp(`(?:bool|Bytecode) Bytecodes::${fnName}\\([\\s\\S]*?\\{([\\s\\S]*?)\\n\\}`));
  if (!m) throw new Error(`cc 函数缺失: ${fnName}`);
  const names = [...m[1].matchAll(/case Bytecode::k(\w+):/g)].map((c) => c[1]);
  if (names.length !== want) throw new Error(`${fnName} 对数不符: 期望 ${want} 实得 ${names.length}`);
  for (const n of names) if (!nameSet.has(n)) throw new Error(`${fnName} 未知成员: ${n}`);
  return names;
}
const starLookahead = ccCaseList("IsStarLookahead", 38);
const makesCallCases = ccCaseList("MakesCallAlongCriticalPath", 6);

// header 显式成员表 (switch case 或布尔或链两种形态, 通用抓取)
function hSwitchList(fnName, want) {
  const m = hSrc.match(new RegExp(`static constexpr [\\w:<> ]+ ${fnName}\\(Bytecode bytecode\\) \\{([\\s\\S]*?)\\n  \\}`));
  if (!m) throw new Error(`header switch 缺失: ${fnName}`);
  const names = [...m[1].matchAll(/Bytecode::k(\w+)/g)].map((c) => c[1]);
  if (names.length !== want) throw new Error(`${fnName} 对数不符: 期望 ${want} 实得 ${names.length}`);
  return names;
}
const callOrConstruct = hSwitchList("IsCallOrConstruct", 14);
const callRuntime = hSwitchList("IsCallRuntime", 3);
const receiverNotNull = ["CallProperty", "CallProperty0", "CallProperty1", "CallProperty2"];
const receiverNull = ["CallUndefinedReceiver", "CallUndefinedReceiver0", "CallUndefinedReceiver1", "CallUndefinedReceiver2", "CallJSRuntime"];
const receiverAny = ["CallAnyReceiver", "Construct", "CallWithSpread", "ConstructWithSpread", "InvokeIntrinsic"];

// 结构锚 (bytecodes.h/cc 原文)
for (const [src, needle, name] of [
  [hSrc, "static const int kMaxOperands = 5;", "kMaxOperands"],
  [hSrc, "static const int kBytecodeCount = static_cast<int>(Bytecode::kLast) + 1;", "kBytecodeCount 公式"],
  [hSrc, "kFirstShortStar = kStar15,", "kFirstShortStar"],
  [hSrc, "kLastShortStar = kStar0", "kLastShortStar"],
  [hSrc, "V_TSA(BitwiseNot, ImplicitRegisterUse::kReadWriteAccumulator,", "V_TSA 条目 (默认配置等同 V)"],
  [ccSrc, "const OperandType* const Bytecodes::kOperandTypes[] = {", "kOperandTypes 表"],
  [ccSrc, "const uint8_t Bytecodes::kBytecodeSizes[3][kBytecodeCount] = {", "kBytecodeSizes 表"],
  [ccSrc, "kOperandKindSizes[3][BytecodeOperands::kOperandTypeCount] = {", "kOperandKindSizes 表"],
  [ccSrc, "#define CASE(Name, ...)   \\\n  case Bytecode::k##Name: \\\n    return #Name;", "ToString X-macro"],
]) {
  if (!src.includes(needle)) throw new Error(`结构锚缺失: ${name}`);
}

const pascal = (s) => s.replace(/_/g, "");
const useVariant = (u) => (u === "None" ? "NONE" : u.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase());
const byteList = (names) => names.map((n) => `Bytecode::${pascal(n)}`).join(", ");
const opTypeList = (names) => names.map((n) => `OperandType::${pascal(n)}`).join(" | ");
const sizeName = (v) => ({ 0: "None", 1: "Byte", 2: "Short", 4: "Quad" })[v];

let rust = banner("src/interpreter/bytecodes.h.js", "src/interpreter/bytecodes.{h,cc}", "Ignition Bytecode 全表 + 操作数元表 (clang -E 对数: 全表 217 = 200+16+1)");

rust += `
use crate::bytecode_operands::{ImplicitRegisterUse, OperandScale, OperandSize, OperandType, OperandTypeInfo};

/// 字节码总数 (原版 Bytecodes::kBytecodeCount = kLast + 1; clang -E 展开核对 217)
pub const BYTECODE_COUNT: usize = ${table.length};
/// 单条字节码最大操作数个数 (原版 kMaxOperands)
pub const MAX_OPERANDS: usize = 5;
/// ShortStar 字节码数 (Star15..Star0)
pub const SHORT_STAR_COUNT: usize = ${216 - idxOf("Star15")};

/// 解释器字节码全表 (原版 enum class Bytecode : uint8_t; 顺序敏感 — 范围谓词依赖)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Bytecode {
`;
for (const b of table) {
  const ops = b.operands.length ? `, 操作数 ${b.operands.join("+")}` : "";
  rust += `  /// 原版 k${b.name} (${b.use}${ops})\n  ${b.rustName} = ${idxOf(b.name)},\n`;
}
rust += `}

impl Bytecode {
  /// 字节码总数 (原版 Bytecodes::kBytecodeCount)
  pub const BYTECODE_COUNT: usize = BYTECODE_COUNT;
  /// 单条字节码最大操作数个数 (原版 kMaxOperands)
  pub const MAX_OPERANDS: usize = MAX_OPERANDS;
  /// ShortStar 字节码数
  pub const SHORT_STAR_COUNT: usize = SHORT_STAR_COUNT;

  /// 原版 kLast (最后一个变体)
  pub const LAST: Bytecode = Self::${table[table.length - 1].rustName};
  /// ShortStar 区首 (原版 kFirstShortStar = kStar15)
  pub const FIRST_SHORT_STAR: Bytecode = Self::Star15;
  /// ShortStar 区尾 (原版 kLastShortStar = kStar0)
  pub const LAST_SHORT_STAR: Bytecode = Self::Star0;

  /// 原版名 (bytecodes.cc Bytecodes::ToString)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    NAMES[self as usize]
  }

  /// 数值反查 (原版 Bytecodes::FromByte; 全表 match 展开)
  #[inline]
  #[must_use]
  pub const fn from_repr(v: u8) -> Option<Self> {
    match v {
${table.map((b, i) => `      ${i} => Some(Self::${b.rustName}),`).join("\n")}
      _ => None,
    }
  }

  /// 字节值 (原版 Bytecodes::ToByte)
  #[inline]
  #[must_use]
  pub const fn to_byte(self) -> u8 {
    self as u8
  }

  /// 隐式累加器用法表查询 (原版 GetImplicitRegisterUse)
  #[inline]
  #[must_use]
  pub const fn implicit_register_use(self) -> ImplicitRegisterUse {
    IMPLICIT_REGISTER_USE[self as usize]
  }

  /// 操作数个数 (原版 NumberOfOperands)
  #[inline]
  #[must_use]
  pub const fn number_of_operands(self) -> usize {
    OPERAND_COUNT[self as usize] as usize
  }

  /// 操作数类型列表 (原版 GetOperandTypes)
  #[inline]
  #[must_use]
  pub const fn operand_types(self) -> &'static [OperandType] {
    OPERAND_TYPES[self as usize]
  }

  /// 指定刻度的操作数尺寸列表 (原版 GetOperandSizes; scale.as_index() = scale >> 1)
  #[inline]
  #[must_use]
  pub const fn operand_sizes(self, scale: OperandScale) -> &'static [OperandSize] {
    OPERAND_SIZES[scale.as_index()][self as usize]
  }

  /// 指定刻度的操作数偏移列表, 相对字节码起始, 首操作数恒为 1 (原版 GetOperandOffset)
  #[inline]
  #[must_use]
  pub const fn operand_offsets(self, scale: OperandScale) -> &'static [u8] {
    OPERAND_OFFSETS[scale.as_index()][self as usize]
  }

  /// 含操作数的字节码总尺寸 (原版 Bytecodes::Size)
  #[inline]
  #[must_use]
  pub const fn size(self, scale: OperandScale) -> usize {
    BYTECODE_SIZES[scale.as_index()][self as usize] as usize
  }

  #[inline]
  pub fn from_byte(v: u8) -> Option<Self> {
    Self::from_repr(v)
  }

  #[inline]
  pub fn get_operand_type(self, index: usize) -> OperandType {
    self.operand_types()[index]
  }

  #[inline]
  pub fn get_operand_offset(self, index: usize, scale: OperandScale) -> usize {
    self.operand_offsets(scale)[index] as usize
  }
}

/// 字节码原版名表 (ToString X-macro 展开, 顺序同枚举)
pub static NAMES: [&str; BYTECODE_COUNT] = [
`;
for (const b of table) rust += `  "${b.name}",\n`;
rust += `];

/// 每条字节码的隐式累加器用法 (bytecodes.cc kImplicitRegisterUse)
pub static IMPLICIT_REGISTER_USE: [ImplicitRegisterUse; BYTECODE_COUNT] = [
`;
for (const b of table) rust += `  ImplicitRegisterUse::${useVariant(b.use)},\n`;
rust += `];

/// 每条字节码的操作数类型 (bytecodes.cc kOperandTypes)
pub static OPERAND_TYPES: [&[OperandType]; BYTECODE_COUNT] = [
`;
for (const b of table) {
  rust += `  &[${b.operands.map((o) => `OperandType::${pascal(o)}`).join(", ")}],\n`;
}
rust += `];

/// 每条字节码的操作数元信息 (bytecodes.cc kOperandTypeInfos)
pub static OPERAND_TYPE_INFOS: [&[OperandTypeInfo]; BYTECODE_COUNT] = [
`;
for (const b of table) {
  rust += `  &[${b.operands.map((o) => `OperandTypeInfo::${typeInfoOf.get(o)}`).join(", ")}],\n`;
}
rust += `];

/// 每条字节码的操作数个数 (bytecodes.cc kOperandCount)
#[rustfmt::skip]
pub static OPERAND_COUNT: [u8; BYTECODE_COUNT] = [
`;
for (let i = 0; i < table.length; i += 20) {
  rust += `  ${table.slice(i, i + 20).map((b) => b.operands.length).join(", ")},\n`;
}
rust += `];

/// 三档刻度下的字节码总尺寸 (bytecodes.cc kBytecodeSizes[3][N]; 行 = Single/Double/Quadruple)
#[rustfmt::skip]
pub static BYTECODE_SIZES: [[u8; BYTECODE_COUNT]; 3] = [
`;
for (let s = 0; s < 3; s++) {
  rust += `  [` + sized.map((b) => b.total[s]).join(", ") + `],\n`;
}
rust += `];

/// 三档刻度下每操作数尺寸 (bytecodes.cc kOperandSizes[3][N])
pub static OPERAND_SIZES: [[&[OperandSize]; BYTECODE_COUNT]; 3] = [
`;
for (let s = 0; s < 3; s++) {
  rust += "  [\n";
  for (const b of sized) {
    rust += `    &[${b.sizes[s].map((v) => `OperandSize::${sizeName(v)}`).join(", ")}],\n`;
  }
  rust += "  ],\n";
}
rust += `];

/// 三档刻度下每操作数偏移 (bytecodes.cc kOperandOffsets[3][N]; 首操作数恒为 1, 紧跟操作码)
pub static OPERAND_OFFSETS: [[&[u8]; BYTECODE_COUNT]; 3] = [
`;
for (let s = 0; s < 3; s++) {
  rust += "  [\n";
  for (const b of sized) {
    rust += `    &[${b.offsets[s].join(", ")}],\n`;
  }
  rust += "  ],\n";
}
rust += `];

// —— 分类宏展开 (bytecodes.h X-macro → 常量数组) ——

/// DebugBreak 平铺族 (原版 DEBUG_BREAK_PLAIN_BYTECODE_LIST, ${cat.DEBUG_BREAK_PLAIN_BYTECODE_LIST.length} 条)
pub const DEBUG_BREAK_PLAIN: [Bytecode; ${cat.DEBUG_BREAK_PLAIN_BYTECODE_LIST.length}] = [${byteList(cat.DEBUG_BREAK_PLAIN_BYTECODE_LIST)}];

/// DebugBreak 前缀族 (原版 DEBUG_BREAK_PREFIX_BYTECODE_LIST)
pub const DEBUG_BREAK_PREFIX: [Bytecode; ${cat.DEBUG_BREAK_PREFIX_BYTECODE_LIST.length}] = [${byteList(cat.DEBUG_BREAK_PREFIX_BYTECODE_LIST)}];

/// DebugBreak 全族 (原版 DEBUG_BREAK_BYTECODE_LIST)
pub const DEBUG_BREAK_ALL: [Bytecode; ${cat.DEBUG_BREAK_BYTECODE_LIST.length}] = [${byteList(cat.DEBUG_BREAK_BYTECODE_LIST)}];

/// 无条件立即跳转 (原版 JUMP_UNCONDITIONAL_IMMEDIATE_BYTECODE_LIST)
pub const JUMP_UNCONDITIONAL_IMMEDIATE: [Bytecode; 2] = [${byteList(cat.JUMP_UNCONDITIONAL_IMMEDIATE_BYTECODE_LIST)}];

/// 无条件常量池跳转 (原版 JUMP_UNCONDITIONAL_CONSTANT_BYTECODE_LIST)
pub const JUMP_UNCONDITIONAL_CONSTANT: [Bytecode; 1] = [${byteList(cat.JUMP_UNCONDITIONAL_CONSTANT_BYTECODE_LIST)}];

/// ToBoolean 条件立即跳转 (原版 JUMP_TOBOOLEAN_CONDITIONAL_IMMEDIATE_BYTECODE_LIST)
pub const JUMP_TOBOOLEAN_CONDITIONAL_IMMEDIATE: [Bytecode; 2] = [${byteList(cat.JUMP_TOBOOLEAN_CONDITIONAL_IMMEDIATE_BYTECODE_LIST)}];

/// ToBoolean 条件常量跳转 (原版 JUMP_TOBOOLEAN_CONDITIONAL_CONSTANT_BYTECODE_LIST)
pub const JUMP_TOBOOLEAN_CONDITIONAL_CONSTANT: [Bytecode; 2] = [${byteList(cat.JUMP_TOBOOLEAN_CONDITIONAL_CONSTANT_BYTECODE_LIST)}];

/// 条件立即跳转 (原版 JUMP_CONDITIONAL_IMMEDIATE_BYTECODE_LIST, 含 ToBoolean 展开)
pub const JUMP_CONDITIONAL_IMMEDIATE: [Bytecode; 11] = [${byteList(cat.JUMP_CONDITIONAL_IMMEDIATE_BYTECODE_LIST)}];

/// 条件常量跳转 (原版 JUMP_CONDITIONAL_CONSTANT_BYTECODE_LIST, 含 ToBoolean 展开)
pub const JUMP_CONDITIONAL_CONSTANT: [Bytecode; 11] = [${byteList(cat.JUMP_CONDITIONAL_CONSTANT_BYTECODE_LIST)}];

/// 常量池跳转全族 (原版 JUMP_CONSTANT_BYTECODE_LIST)
pub const JUMP_CONSTANT_ALL: [Bytecode; 12] = [${byteList(cat.JUMP_CONSTANT_BYTECODE_LIST)}];

/// 立即跳转全族 (原版 JUMP_IMMEDIATE_BYTECODE_LIST)
pub const JUMP_IMMEDIATE_ALL: [Bytecode; 13] = [${byteList(cat.JUMP_IMMEDIATE_BYTECODE_LIST)}];

/// ToBoolean 跳转 (原版 JUMP_TO_BOOLEAN_BYTECODE_LIST)
pub const JUMP_TO_BOOLEAN: [Bytecode; 4] = [${byteList(cat.JUMP_TO_BOOLEAN_BYTECODE_LIST)}];

/// 无条件跳转全族 (原版 JUMP_UNCONDITIONAL_BYTECODE_LIST)
pub const JUMP_UNCONDITIONAL_ALL: [Bytecode; 3] = [${byteList(cat.JUMP_UNCONDITIONAL_BYTECODE_LIST)}];

/// 条件跳转全族 (原版 JUMP_CONDITIONAL_BYTECODE_LIST)
pub const JUMP_CONDITIONAL_ALL: [Bytecode; 22] = [${byteList(cat.JUMP_CONDITIONAL_BYTECODE_LIST)}];

/// 前向跳转 (原版 JUMP_FORWARD_BYTECODE_LIST)
pub const JUMP_FORWARD: [Bytecode; 24] = [${byteList(cat.JUMP_FORWARD_BYTECODE_LIST)}];

/// 跳转全族 (原版 JUMP_BYTECODE_LIST = 前向 + JumpLoop)
pub const JUMP_ALL: [Bytecode; 25] = [${byteList(cat.JUMP_BYTECODE_LIST)}];

/// 返回族 (原版 RETURN_BYTECODE_LIST)
pub const RETURN_ALL: [Bytecode; 2] = [${byteList(cat.RETURN_BYTECODE_LIST)}];

/// 无条件抛出族 (原版 UNCONDITIONAL_THROW_BYTECODE_LIST)
pub const UNCONDITIONAL_THROW: [Bytecode; 2] = [${byteList(cat.UNCONDITIONAL_THROW_BYTECODE_LIST)}];

/// 静态谓词集合 (原版 class Bytecodes : AllStatic)
pub struct Bytecodes;

impl Bytecodes {
  /// 刻度 -> 前缀字节码 (原版 OperandScaleToPrefixBytecode)
  #[inline]
  #[must_use]
  pub const fn operand_scale_to_prefix_bytecode(scale: OperandScale) -> Option<Bytecode> {
    match scale {
      OperandScale::Quadruple => Some(Bytecode::ExtraWide),
      OperandScale::Double => Some(Bytecode::Wide),
      // 原版其余刻度走 UNREACHABLE; 此处保守返回 None
      _ => None,
    }
  }

  /// 刻度是否需要前缀字节码 (原版 OperandScaleRequiresPrefixBytecode)
  #[inline]
  #[must_use]
  pub fn operand_scale_requires_prefix_bytecode(scale: OperandScale) -> bool {
    !matches!(scale, OperandScale::Single)
  }

  /// 前缀字节码 -> 刻度 (原版 PrefixBytecodeToOperandScale)
  /// 注: 原版有 V8_TARGET_OS_ANDROID 分支 (crbug.com/1379788 workaround), 语义与非安卓分支一致
  #[inline]
  #[must_use]
  pub fn prefix_bytecode_to_operand_scale(b: Bytecode) -> Option<OperandScale> {
    match b {
      Bytecode::ExtraWide | Bytecode::DebugBreakExtraWide => Some(OperandScale::Quadruple),
      Bytecode::Wide | Bytecode::DebugBreakWide => Some(OperandScale::Double),
      _ => None,
    }
  }

  /// 是否读累加器 (原版 ReadsAccumulator)
  #[inline]
  #[must_use]
  pub fn reads_accumulator(b: Bytecode) -> bool {
    b.implicit_register_use().contains(ImplicitRegisterUse::READ_ACCUMULATOR)
  }

  /// 是否写累加器 (原版 WritesAccumulator)
  #[inline]
  #[must_use]
  pub fn writes_accumulator(b: Bytecode) -> bool {
    b.implicit_register_use().contains(ImplicitRegisterUse::WRITE_ACCUMULATOR)
  }

  /// 是否覆写累加器 (原版 ClobbersAccumulator)
  #[inline]
  #[must_use]
  pub fn clobbers_accumulator(b: Bytecode) -> bool {
    b.implicit_register_use().contains(ImplicitRegisterUse::CLOBBER_ACCUMULATOR)
  }

  /// 是否写或覆写累加器 (原版 WritesOrClobbersAccumulator)
  #[inline]
  #[must_use]
  pub fn writes_or_clobbers_accumulator(b: Bytecode) -> bool {
    b.implicit_register_use().intersects(ImplicitRegisterUse::WRITE_ACCUMULATOR)
      || b.implicit_register_use().intersects(ImplicitRegisterUse::CLOBBER_ACCUMULATOR)
  }

  /// 是否隐式写寄存器 (原版 WritesImplicitRegister)
  #[inline]
  #[must_use]
  pub fn writes_implicit_register(b: Bytecode) -> bool {
    b.implicit_register_use().contains(ImplicitRegisterUse::WRITE_SHORT_STAR)
  }

  /// 无副作用累加器加载 (原版 IsAccumulatorLoadWithoutEffects; 依赖 kLdar..kLdaImmutableCurrentContextSlot 次序)
  #[inline]
  #[must_use]
  pub fn is_accumulator_load_without_effects(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::Ldar as u8 && (b as u8) <= Bytecode::LdaImmutableCurrentContextSlot as u8
  }

  /// 无副作用比较 (原版 IsCompareWithoutEffects; 依赖 kTestReferenceEqual..kTestTypeOf 次序)
  #[inline]
  #[must_use]
  pub fn is_compare_without_effects(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::TestReferenceEqual as u8 && (b as u8) <= Bytecode::TestTypeOf as u8
  }

  /// 是否 ShortStar (原版 IsShortStar)
  #[inline]
  #[must_use]
  pub fn is_short_star(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::FIRST_SHORT_STAR as u8 && (b as u8) <= Bytecode::LAST_SHORT_STAR as u8
  }

  /// 是否任意 Star (原版 IsAnyStar)
  #[inline]
  #[must_use]
  pub fn is_any_star(b: Bytecode) -> bool {
    b == Bytecode::Star || Self::is_short_star(b)
  }

  /// 无副作用寄存器加载 (原版 IsRegisterLoadWithoutEffects; kStar..=kPopContext)
  #[inline]
  #[must_use]
  pub fn is_register_load_without_effects(b: Bytecode) -> bool {
    Self::is_short_star(b) || ((b as u8) >= Bytecode::Star as u8 && (b as u8) <= Bytecode::PopContext as u8)
  }

  /// 条件立即跳转 (原版 IsConditionalJumpImmediate)
  #[inline]
  #[must_use]
  pub fn is_conditional_jump_immediate(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::JumpIfToBooleanTrue as u8 && (b as u8) <= Bytecode::JumpIfForInDone as u8
  }

  /// 条件常量跳转 (原版 IsConditionalJumpConstant)
  #[inline]
  #[must_use]
  pub fn is_conditional_jump_constant(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::JumpIfNullConstant as u8 && (b as u8) <= Bytecode::JumpIfToBooleanFalseConstant as u8
  }

  /// 条件跳转 (原版 IsConditionalJump)
  #[inline]
  #[must_use]
  pub fn is_conditional_jump(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::JumpIfNullConstant as u8 && (b as u8) <= Bytecode::JumpIfForInDone as u8
  }

  /// 无条件跳转 (原版 IsUnconditionalJump; kJumpLoop..=kJumpConstant)
  #[inline]
  #[must_use]
  pub fn is_unconditional_jump(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::JumpLoop as u8 && (b as u8) <= Bytecode::JumpConstant as u8
  }

  /// 立即跳转 (原版 IsJumpImmediate)
  #[inline]
  #[must_use]
  pub fn is_jump_immediate(b: Bytecode) -> bool {
    b == Bytecode::Jump || b == Bytecode::JumpLoop || Self::is_conditional_jump_immediate(b)
  }

  /// 常量跳转 (原版 IsJumpConstant)
  #[inline]
  #[must_use]
  pub fn is_jump_constant(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::JumpConstant as u8 && (b as u8) <= Bytecode::JumpIfToBooleanFalseConstant as u8
  }

  /// 内部做 ToBoolean 强转的跳转 (原版 IsJumpIfToBoolean)
  #[inline]
  #[must_use]
  pub fn is_jump_if_to_boolean(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::JumpIfToBooleanTrueConstant as u8 && (b as u8) <= Bytecode::JumpIfToBooleanFalse as u8
  }

  /// 是否跳转 (原版 IsJump; kJumpLoop..=kJumpIfForInDone)
  #[inline]
  #[must_use]
  pub fn is_jump(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::JumpLoop as u8 && (b as u8) <= Bytecode::JumpIfForInDone as u8
  }

  /// 前向跳转 (原版 IsForwardJump; kJump..=kJumpIfForInDone)
  #[inline]
  #[must_use]
  pub fn is_forward_jump(b: Bytecode) -> bool {
    (b as u8) >= Bytecode::Jump as u8 && (b as u8) <= Bytecode::JumpIfForInDone as u8
  }

  /// 无副作用跳转 (原版 IsJumpWithoutEffects; JumpLoop 隐含 StackCheck 除外)
  #[inline]
  #[must_use]
  pub fn is_jump_without_effects(b: Bytecode) -> bool {
    Self::is_jump(b) && b != Bytecode::JumpLoop
  }

  /// 是否 switch (原版 IsSwitch)
  #[inline]
  #[must_use]
  pub fn is_switch(b: Bytecode) -> bool {
    b == Bytecode::SwitchOnSmiNoFeedback || b == Bytecode::SwitchOnGeneratorState
  }

  /// 无外部副作用 (原版 IsWithoutExternalSideEffects)
  #[inline]
  #[must_use]
  pub fn is_without_external_side_effects(b: Bytecode) -> bool {
    Self::is_accumulator_load_without_effects(b)
      || Self::is_register_load_without_effects(b)
      || Self::is_compare_without_effects(b)
      || Self::is_jump_without_effects(b)
      || Self::is_switch(b)
      || b == Bytecode::Return
  }

  /// 是否 Ldar 或 Star (原版 IsLdarOrStar)
  #[inline]
  #[must_use]
  pub fn is_ldar_or_star(b: Bytecode) -> bool {
    b == Bytecode::Ldar || Self::is_any_star(b)
  }

  /// 是否调用/构造 (原版 IsCallOrConstruct)
  #[inline]
  #[must_use]
  pub fn is_call_or_construct(b: Bytecode) -> bool {
    matches!(b, ${byteList(callOrConstruct).replaceAll(", Bytecode::", " | Bytecode::")})
  }

  /// 是否运行时调用 (原版 IsCallRuntime)
  #[inline]
  #[must_use]
  pub fn is_call_runtime(b: Bytecode) -> bool {
    matches!(b, ${byteList(callRuntime).replaceAll(", Bytecode::", " | Bytecode::")})
  }

  /// 是否缩放前缀字节码 (原版 IsPrefixScalingBytecode)
  #[inline]
  #[must_use]
  pub fn is_prefix_scaling_bytecode(b: Bytecode) -> bool {
    matches!(b, Bytecode::ExtraWide | Bytecode::Wide | Bytecode::DebugBreakExtraWide | Bytecode::DebugBreakWide)
  }

  /// 比较类内嵌 feedback (原版 IsCompareWithEmbeddedFeedback)
  #[inline]
  #[must_use]
  pub fn is_compare_with_embedded_feedback(b: Bytecode) -> bool {
    matches!(b, Bytecode::TestEqualStrict | Bytecode::TestEqual | Bytecode::TestLessThan | Bytecode::TestGreaterThan | Bytecode::TestLessThanOrEqual | Bytecode::TestGreaterThanOrEqual)
  }

  /// 二元运算内嵌 feedback (原版 IsBinaryOpWithEmbeddedFeedback)
  #[inline]
  #[must_use]
  pub fn is_binary_op_with_embedded_feedback(b: Bytecode) -> bool {
    matches!(b, Bytecode::Add | Bytecode::Sub | Bytecode::Mul | Bytecode::Div | Bytecode::Mod | Bytecode::Exp | Bytecode::BitwiseOr | Bytecode::BitwiseXor | Bytecode::BitwiseAnd | Bytecode::ShiftLeft | Bytecode::ShiftRight | Bytecode::ShiftRightLogical | Bytecode::AddSmi | Bytecode::SubSmi | Bytecode::MulSmi | Bytecode::DivSmi | Bytecode::ModSmi | Bytecode::ExpSmi | Bytecode::BitwiseOrSmi | Bytecode::BitwiseXorSmi | Bytecode::BitwiseAndSmi | Bytecode::ShiftLeftSmi | Bytecode::ShiftRightSmi | Bytecode::ShiftRightLogicalSmi)
  }

  /// 一元运算内嵌 feedback (原版 IsUnaryOpWithEmbeddedFeedback)
  #[inline]
  #[must_use]
  pub fn is_unary_op_with_embedded_feedback(b: Bytecode) -> bool {
    matches!(b, Bytecode::Inc | Bytecode::Dec | Bytecode::Negate | Bytecode::BitwiseNot)
  }

  /// 是否内嵌 feedback 字节码 (原版 IsEmbeddedFeedbackBytecode)
  #[inline]
  #[must_use]
  pub fn is_embedded_feedback_bytecode(b: Bytecode) -> bool {
    Self::is_compare_with_embedded_feedback(b)
      || Self::is_binary_op_with_embedded_feedback(b)
      || Self::is_unary_op_with_embedded_feedback(b)
  }

  /// 是否返回 (原版 Returns; RETURN_BYTECODE_LIST 展开)
  #[inline]
  #[must_use]
  pub fn returns(b: Bytecode) -> bool {
    matches!(b, Bytecode::Return | Bytecode::SuspendGenerator)
  }

  /// 是否无条件抛出 (原版 UnconditionallyThrows)
  #[inline]
  #[must_use]
  pub fn unconditionally_throws(b: Bytecode) -> bool {
    matches!(b, Bytecode::Throw | Bytecode::ReThrow)
  }

  /// 操作数是否可缩放有符号 (原版 OperandIsScalableSignedByte)
  #[inline]
  #[must_use]
  pub fn operand_is_scalable_signed(b: Bytecode, i: usize) -> bool {
    b.operand_types()[i].type_info() == OperandTypeInfo::ScalableSignedByte
  }

  /// 操作数是否可缩放无符号 (原版 OperandIsScalableUnsignedByte)
  #[inline]
  #[must_use]
  pub fn operand_is_scalable_unsigned(b: Bytecode, i: usize) -> bool {
    b.operand_types()[i].type_info() == OperandTypeInfo::ScalableUnsignedByte
  }

  /// 操作数是否可缩放 (原版 OperandIsScalable)
  #[inline]
  #[must_use]
  pub fn operand_is_scalable(b: Bytecode, i: usize) -> bool {
    Self::operand_is_scalable_signed(b, i) || Self::operand_is_scalable_unsigned(b, i)
  }

  /// 是否存在可缩放操作数 (原版 IsBytecodeWithScalableOperands)
  #[inline]
  #[must_use]
  pub fn is_bytecode_with_scalable_operands(b: Bytecode) -> bool {
    let mut i = 0;
    while i < b.number_of_operands() {
      if Self::operand_is_scalable(b, i) {
        return true;
      }
      i += 1;
    }
    false
  }

  /// 是否有该刻度的 handler (原版 BytecodeHasHandler)
  #[inline]
  #[must_use]
  pub fn bytecode_has_handler(b: Bytecode, scale: OperandScale) -> bool {
    (scale == OperandScale::Single && (!Self::is_short_star(b) || b == Bytecode::Star0))
      || Self::is_bytecode_with_scalable_operands(b)
  }

  /// 有符号立即数的最低适配刻度 (原版 ScaleForSignedOperand; kMin/MaxInt8/16 = -128/127/-32768/32767)
  #[inline]
  #[must_use]
  pub fn scale_for_signed_operand(v: i32) -> OperandScale {
    if (-128..=127).contains(&v) {
      OperandScale::Single
    } else if (-32768..=32767).contains(&v) {
      OperandScale::Double
    } else {
      OperandScale::Quadruple
    }
  }

  /// 无符号立即数的最低适配刻度 (原版 ScaleForUnsignedOperand; kMaxUInt8/16 = 255/65535)
  #[inline]
  #[must_use]
  pub fn scale_for_unsigned_operand(v: u32) -> OperandScale {
    if v <= 255 {
      OperandScale::Single
    } else if v <= 65535 {
      OperandScale::Double
    } else {
      OperandScale::Quadruple
    }
  }

  /// 无符号立即数的最低适配尺寸 (原版 SizeForUnsignedOperand)
  #[inline]
  #[must_use]
  pub fn size_for_unsigned_operand(v: u32) -> OperandSize {
    if v <= 255 {
      OperandSize::Byte
    } else if v <= 65535 {
      OperandSize::Short
    } else {
      OperandSize::Quad
    }
  }

  /// 调用类字节码的 receiver 模式 (原版 GetReceiverMode; 类型就地落位, 原版 src/common/globals.h ConvertReceiverMode)
  #[inline]
  #[must_use]
  pub fn get_receiver_mode(b: Bytecode) -> Option<crate::bytecode_operands::ReceiverMode> {
    use crate::bytecode_operands::ReceiverMode;
    match b {
`;
for (const n of receiverNotNull) rust += `      Bytecode::${pascal(n)} => Some(ReceiverMode::NotNullOrUndefined),\n`;
for (const n of receiverNull) rust += `      Bytecode::${pascal(n)} => Some(ReceiverMode::NullOrUndefined),\n`;
for (const n of receiverAny) rust += `      Bytecode::${pascal(n)} => Some(ReceiverMode::Any),\n`;
rust += `      _ => None,
    }
  }

  /// 替换为对应 DebugBreak 字节码 (原版 GetDebugBreak; Wide/ExtraWide 特判 + 按单刻度尺寸首个匹配 DEBUG_BREAK_PLAIN)
  #[inline]
  #[must_use]
  pub fn get_debug_break(b: Bytecode) -> Bytecode {
    match b {
`;
for (let i = 0; i < table.length; i++) {
  rust += `      Bytecode::${pascal(table[i].name)} => Bytecode::${pascal(debugBreakMap[i])},\n`;
}
rust += `    }
  }

  /// 是否 DebugBreak (原版 IsDebugBreak)
  #[inline]
  #[must_use]
  pub fn is_debug_break(b: Bytecode) -> bool {
    matches!(b, ${byteList(cat.DEBUG_BREAK_BYTECODE_LIST).replaceAll(", Bytecode::", " | Bytecode::")})
  }

  /// Star 前瞻内联白名单 (原版 IsStarLookahead; 仅 Single 刻度, kDebugBreak0 为正确性必需)
  #[inline]
  #[must_use]
  pub fn is_star_lookahead(b: Bytecode, scale: OperandScale) -> bool {
    scale == OperandScale::Single
      && matches!(b, ${byteList(starLookahead).replaceAll(", Bytecode::", " | Bytecode::")})
  }

  /// handler 关键路径是否有调用 (原版 MakesCallAlongCriticalPath)
  #[inline]
  #[must_use]
  pub fn makes_call_along_critical_path(b: Bytecode) -> bool {
    Self::is_call_or_construct(b)
      || Self::is_call_runtime(b)
      || matches!(b, ${byteList(makesCallCases).replaceAll(", Bytecode::", " | Bytecode::")})
  }

  /// 寄存器操作数代表的寄存器数 (原版 GetNumberOfRegistersRepresentedBy)
  #[inline]
  #[must_use]
  pub fn number_of_registers_represented_by(t: OperandType) -> Option<u32> {
    match t {
      OperandType::Reg | OperandType::RegOut | OperandType::RegInOut => Some(1),
      OperandType::RegPair | OperandType::RegOutPair => Some(2),
      OperandType::RegOutTriple => Some(3),
      // RegList/RegOutList 数目由后续 RegCount 操作数决定, 原版 UNREACHABLE
      _ => None,
    }
  }

  /// 操作数类型的适配尺寸 (原版 SizeOfOperand; 表见 bytecode_traits::OPERAND_KIND_SIZES)
  #[inline]
  #[must_use]
  pub fn size_of_operand(t: OperandType, scale: OperandScale) -> OperandSize {
    crate::bytecode_traits::OPERAND_KIND_SIZES[scale.as_index()][t as usize]
  }

  /// 是否寄存器操作数 (原版 IsRegisterOperandType)
  #[inline]
  #[must_use]
  pub fn is_register_operand_type(t: OperandType) -> bool {
    matches!(t, ${opTypeList([...regInputNames, ...regOutputNames, "RegInOut"])})
  }

  /// 是否寄存器输入操作数 (原版 IsRegisterInputOperandType; RegInOut 双向)
  #[inline]
  #[must_use]
  pub fn is_register_input_operand_type(t: OperandType) -> bool {
    matches!(t, ${opTypeList(regInputNames)} | OperandType::RegInOut)
  }

  /// 是否寄存器输出操作数 (原版 IsRegisterOutputOperandType; RegInOut 双向)
  #[inline]
  #[must_use]
  pub fn is_register_output_operand_type(t: OperandType) -> bool {
    matches!(t, ${opTypeList(regOutputNames)} | OperandType::RegInOut)
  }

  /// 是否寄存器列表操作数 (原版 IsRegisterListOperandType)
  #[inline]
  #[must_use]
  pub fn is_register_list_operand_type(t: OperandType) -> bool {
    matches!(t, OperandType::RegList | OperandType::RegOutList)
  }

  /// 是否运行时 id 操作数 (原版 IsRuntimeIdOperandType)
  #[inline]
  #[must_use]
  pub fn is_runtime_id_operand_type(t: OperandType) -> bool {
    t == OperandType::RuntimeId
  }

  /// 操作数类型是否无符号 (原版 IsUnsignedOperandType)
  #[inline]
  #[must_use]
  pub fn is_unsigned_operand_type(t: OperandType) -> bool {
    t.type_info().is_unsigned()
  }
}

// —— 提取不变式断言 (clang -E 对数) ——
const _: () = {
  assert!(BYTECODE_COUNT == 217);
  assert!(SHORT_STAR_COUNT == 16);
  assert!(Bytecode::Wide as u8 == 0);
  assert!(Bytecode::ExtraWide as u8 == 1);
  assert!(Bytecode::Star15 as u8 == 200);
  assert!(Bytecode::Star0 as u8 == 215);
  assert!(Bytecode::Illegal as u8 == 216);
  assert!(Bytecode::LAST as u8 == Bytecode::Illegal as u8);
  // header 内次序 static_assert (范围谓词前提)
  const _: () = assert!((Bytecode::Ldar as u8) < (Bytecode::LdaImmutableCurrentContextSlot as u8));
  const _: () = assert!((Bytecode::TestReferenceEqual as u8) < (Bytecode::TestTypeOf as u8));
};
`;

const p = writeSrc("v8-interpreter", "bytecodes.rs", rust);
const distinctDb = new Set(debugBreakMap).size;
console.log(`written ${p} (${table.length} bytecodes, debug-break 映射 ${distinctDb} 个目标, 分类宏 ${Object.keys(cat).length} 组)`);
