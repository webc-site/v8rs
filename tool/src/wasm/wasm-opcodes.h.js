#!/usr/bin/env bun
// tool/src/wasm/wasm-opcodes.h.js
// 机械提取 v8/src/wasm/wasm-opcodes.{h,-inl.h,cc} 的操作码 X-macro 族:
// FOREACH_*_OPCODE 宏展开 → WasmOpcode 常量/总表/签名枚举/256 查表/TrapReason。
// 对数基准: 原版 #define 块逐行计数 (独立于本脚本解析器), 见下方 EXPECTED 常量。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-opcodes.h");
const inlSrc = readV8("src/wasm/wasm-opcodes-inl.h");
const ccSrc = readV8("src/wasm/wasm-opcodes.cc");
const globalsSrc = readV8("src/common/globals.h");

// ---------- 1. 提取 #define NAME(V) 宏体 (合并续行) ----------
function extractMacroBodies(src) {
  const map = new Map(); // name -> body 文本 (已合并续行, 含 #define 头之后的内容)
  const linesArr = src.split("\n");
  for (let i = 0; i < linesArr.length; i++) {
    const m = linesArr[i].match(/^#define\s+([A-Za-z0-9_]+)\s*\(\s*V\s*\)\s?(.*)$/);
    if (!m) continue;
    let body = m[2];
    while (body.endsWith("\\")) {
      body = body.slice(0, -1) + " " + (linesArr[++i] ?? "").trim();
    }
    map.set(m[1], body);
  }
  return map;
}

// 去注释 (宏体内 /* */ 与 //, 不含字符串内的斜杠, 安全)
function stripComments(s) {
  return s.replace(/\/\*[\s\S]*?\*\//g, " ").replace(/\/\/.*$/gm, " ");
}

const macros = extractMacroBodies(hSrc);

// ---------- 2. 递归展开宏为条目列表 ----------
// 条目: { name, opcode(number), sig, wat, sig64 }
function expandList(macroName, depth = 0) {
  if (depth > 8) throw new Error(`宏嵌套过深: ${macroName}`);
  const raw = macros.get(macroName);
  if (raw === undefined) throw new Error(`未找到宏 ${macroName}`);
  const body = stripComments(raw);
  const out = [];
  const re = /V\s*\(([^()]*)\)|(FOREACH_[A-Za-z0-9_]+)\s*\(\s*V\s*\)/g;
  let m;
  let found = false;
  while ((m = re.exec(body)) !== null) {
    found = true;
    if (m[1] !== undefined) {
      // V(Name, 0xNN, sig, "wat" [, sig64]) — 按引号外逗号切分
      const args = [];
      let cur = "";
      let inStr = false;
      for (const ch of m[1]) {
        if (ch === '"') inStr = !inStr;
        if (ch === "," && !inStr) {
          args.push(cur.trim());
          cur = "";
        } else cur += ch;
      }
      args.push(cur.trim());
      if (args.length < 4) throw new Error(`${macroName} 条目参数不足: ${m[1]}`);
      const [name, opcode, sig, wat, sig64] = args;
      out.push({
        list: macroName,
        name,
        opcode: Number(opcode),
        sig,
        wat: wat.replace(/^"|"$/g, ""),
        sig64: sig64 ? sig64 : null,
      });
    } else {
      out.push(...expandList(m[2], depth + 1));
    }
  }
  if (!found) {
    // 空宏 (如 FOREACH_SIMPLE_PROTOTYPE_OPCODE) — 正文为空即合法
    if (stripComments(raw).trim() !== "") {
      throw new Error(`宏 ${macroName} 无法解析: ${raw.slice(0, 80)}`);
    }
  }
  return out;
}

const LEAF_LISTS = [
  ["FOREACH_CONTROL_OPCODE", 22],
  ["FOREACH_MISC_OPCODE", 25],
  ["FOREACH_LOAD_MEM_OPCODE", 15],
  ["FOREACH_STORE_MEM_OPCODE", 10],
  ["FOREACH_MISC_MEM_OPCODE", 2],
  ["FOREACH_SIMPLE_EXTENDED_CONST_OPCODE", 6],
  ["FOREACH_SIMPLE_NON_CONST_OPCODE", 117],
  ["FOREACH_SIGN_EXTENSION_OPCODE", 5],
  ["FOREACH_WASMFX_OPCODE", 7],
  ["FOREACH_SIMD_MEM_OPCODE", 14],
  ["FOREACH_SIMD_MEM_1_OPERAND_OPCODE", 8],
  ["FOREACH_SIMD_CONST_OPCODE", 1],
  ["FOREACH_SIMD_MASK_OPERAND_OPCODE", 1],
  ["FOREACH_SIMD_MVP_0_OPERAND_OPCODE", 198],
  ["FOREACH_RELAXED_SIMD_OPCODE", 51],
  ["FOREACH_SIMD_1_OPERAND_1_PARAM_OPCODE", 9],
  ["FOREACH_SIMD_1_OPERAND_2_PARAM_OPCODE", 7],
  ["FOREACH_NUMERIC_OPCODE_WITH_SIG", 13],
  ["FOREACH_NUMERIC_OPCODE_WIDE", 4],
  ["FOREACH_NUMERIC_OPCODE_VARIADIC", 5],
  ["FOREACH_ATOMIC_OPCODE", 66],
  ["FOREACH_ATOMIC_0_OPERAND_OPCODE", 1],
  ["FOREACH_GC_OPCODE", 84],
  ["FOREACH_ATOMIC_GC_OPCODE", 28],
];

// 期望总数 = 叶子表之和 (FOREACH_OPCODE 的组成, 见 wasm-opcodes.h:833)
// 22+25+15+10+2+6+117+5+7+14+8+1+1+198+51+9+7+13+4+5+66+1+84+28 = 699
const EXPECTED_TOTAL = LEAF_LISTS.reduce((a, [, n]) => a + n, 0);

const leafEntries = new Map();
for (const [name, expected] of LEAF_LISTS) {
  const es = expandList(name);
  if (es.length !== expected) {
    throw new Error(`${name}: 提取 ${es.length} 条, 期望 ${expected}`);
  }
  leafEntries.set(name, es);
}

// 复合宏对数 (展开结果 = 叶子组合)
const COMPOSITE = [
  ["FOREACH_SIMPLE_OPCODE", ["FOREACH_SIMPLE_EXTENDED_CONST_OPCODE", "FOREACH_SIMPLE_NON_CONST_OPCODE"], 123],
  ["FOREACH_SIMD_0_OPERAND_OPCODE", ["FOREACH_SIMD_MVP_0_OPERAND_OPCODE", "FOREACH_RELAXED_SIMD_OPCODE"], 249],
  ["FOREACH_SIMD_1_OPERAND_OPCODE", ["FOREACH_SIMD_1_OPERAND_1_PARAM_OPCODE", "FOREACH_SIMD_1_OPERAND_2_PARAM_OPCODE"], 16],
  ["FOREACH_SIMD_OPCODE", ["FOREACH_SIMD_0_OPERAND_OPCODE", "FOREACH_SIMD_1_OPERAND_OPCODE", "FOREACH_SIMD_MASK_OPERAND_OPCODE", "FOREACH_SIMD_MEM_OPCODE", "FOREACH_SIMD_MEM_1_OPERAND_OPCODE", "FOREACH_SIMD_CONST_OPCODE"], 289],
  ["FOREACH_NUMERIC_OPCODE", ["FOREACH_NUMERIC_OPCODE_WITH_SIG", "FOREACH_NUMERIC_OPCODE_WIDE", "FOREACH_NUMERIC_OPCODE_VARIADIC"], 22],
];
for (const [name, parts, expected] of COMPOSITE) {
  const es = parts.flatMap((p) => leafEntries.get(p));
  if (es.length !== expected) {
    throw new Error(`${name}: 组合 ${es.length} 条, 期望 ${expected}`);
  }
  leafEntries.set(name, es);
}

const ALL_OPS = LEAF_LISTS.map(([n]) => leafEntries.get(n)).flat();
if (ALL_OPS.length !== EXPECTED_TOTAL) {
  throw new Error(`FOREACH_OPCODE 总数 ${ALL_OPS.length} != 期望 ${EXPECTED_TOTAL}`);
}

// 唯一性对数: 名字与编码均不得重复
{
  const names = new Set();
  const codes = new Set();
  for (const e of ALL_OPS) {
    if (names.has(e.name)) throw new Error(`重复操作码名: ${e.name}`);
    if (codes.has(e.opcode)) throw new Error(`重复操作码编码: 0x${e.opcode.toString(16)} (${e.name})`);
    names.add(e.name);
    codes.add(e.opcode);
  }
}
// 关键锚点
const byName = (n) => {
  const e = ALL_OPS.find((x) => x.name === n);
  if (!e) throw new Error(`缺少锚点操作码 ${n}`);
  return e;
};
if (byName("Unreachable").opcode !== 0x00) throw new Error("Unreachable != 0x00");
if (byName("NopForTestingUnsupportedInLiftoff").opcode !== 0x16) throw new Error("NopForTesting 编码漂移");
if (byName("I64AtomicCompareExchange32U").opcode !== 0xfe4e) throw new Error("ATOMIC 尾锚漂移");
if (byName("StringNewUtf8ArrayTry").opcode !== 0xfbb8) throw new Error("GC/尾锚漂移");

console.log(`FOREACH_OPCODE 展开 ${ALL_OPS.length} 条, 无重名无重码`);

// ---------- 3. 前缀与签名 ----------
const prefixMatch = hSrc.match(/#define\s+FOREACH_PREFIX\(V\)\s*\\\s*\n([\s\S]*?)\n\n/);
if (!prefixMatch) throw new Error("未找到 FOREACH_PREFIX");
const prefixes = [...stripComments(prefixMatch[1]).matchAll(/V\(\s*(\w+)\s*,\s*(0x[0-9a-fA-F]+)\s*\)/g)].map(
  (m) => ({ name: m[1], opcode: Number(m[2]) }),
);
if (prefixes.length !== 4) throw new Error(`FOREACH_PREFIX 提取 ${prefixes.length} 条, 期望 4`);

// FOREACH_SIGNATURE / FOREACH_SIMD_SIGNATURE: V(name, ret, params...)
function expandSigList(macroName) {
  const raw = macros.get(macroName);
  if (raw === undefined) throw new Error(`未找到宏 ${macroName}`);
  const body = stripComments(raw);
  const out = [];
  const re = /V\(([^()]*)\)/g;
  let m;
  while ((m = re.exec(body)) !== null) {
    const args = m[1].split(",").map((s) => s.trim());
    out.push({ name: args[0], types: args.slice(1) }); // types[0]=返回, 其余=参数
  }
  return out;
}
const simdSigs = expandSigList("FOREACH_SIMD_SIGNATURE");
const restSigs = (() => {
  const raw = stripComments(macros.get("FOREACH_SIGNATURE"));
  // 去掉嵌套的 FOREACH_SIMD_SIGNATURE(V) 引用行
  const own = raw.replace(/FOREACH_SIMD_SIGNATURE\s*\(\s*V\s*\)/g, " ");
  const out = [];
  const re = /V\(([^()]*)\)/g;
  let m;
  while ((m = re.exec(own)) !== null) {
    const args = m[1].split(",").map((s) => s.trim());
    out.push({ name: args[0], types: args.slice(1) });
  }
  return out;
})();
if (simdSigs.length !== 11) throw new Error(`SIMD 签名 ${simdSigs.length}, 期望 11`);
if (restSigs.length !== 48) throw new Error(`普通签名 ${restSigs.length}, 期望 48`);
const ALL_SIGS = [...simdSigs, ...restSigs]; // FOREACH_SIGNATURE 展开顺序
{
  const s = new Set(ALL_SIGS.map((x) => x.name));
  if (s.size !== ALL_SIGS.length) throw new Error("签名短名重复");
  if (!s.has("i_i") || !s.has("v_v") || !s.has("s_sss")) throw new Error("签名锚点缺失");
}
console.log(`FOREACH_SIGNATURE 展开 ${ALL_SIGS.length} 条`);

// FOREACH_WASM_TRAPREASON (定义于 src/common/globals.h)
const trapMatch = globalsSrc.match(/#define\s+FOREACH_WASM_TRAPREASON\(V\)\s*\\\s*\n([\s\S]*?)\n\n/);
if (!trapMatch) throw new Error("未找到 FOREACH_WASM_TRAPREASON");
const trapReasons = [...stripComments(trapMatch[1]).matchAll(/V\((\w+)\)/g)].map((m) => m[1]);
if (trapReasons.length !== 21) throw new Error(`TrapReason ${trapReasons.length} 条, 期望 21`);

// ---------- 4. 生成 Rust ----------
function screaming(name) {
  return name
    .replace(/([a-z0-9])([A-Z])/g, "$1_$2")
    .replace(/([a-z])([A-Z])/g, "$1_$2")
    .toUpperCase();
}

// 签名短名 → Rust 变体名: i_i → I_I
const sigVariant = (s) => (s === "_" ? "None" : s.toUpperCase());

// C++ 类型记号 → SigType 变体
const typeVariant = (t) => {
  const m = t.match(/^kWasm(\w+)$/);
  if (!m) throw new Error(`未知类型记号 ${t}`);
  return m[1]; // I32 / I64 / F32 / F64 / S128 / AnyRef / FuncRef / EqRef / Void
};

let rust = banner(
  "src/wasm/wasm-opcodes.h.js",
  "src/wasm/wasm-opcodes.{h,-inl.h,cc}",
  "Wasm 操作码全表: X-macro 机械展开 + 签名查表 + TrapReason",
);

rust += `
/// Wasm 操作码 32 位内部编码 (对应 C++ enum WasmOpcode)。
///
/// 编码规则 (见原版 wasm-opcodes.h 注释, 双射):
/// 1) 无前缀: [0x00, 0xff] 即字节本身;
/// 2) 前缀码 [0xfb00, 0xfeff]: (prefix << 8) | 低位字节;
/// 3) relaxed-simd: (0xfd << 12) | 12 位字节, 即 [0xfd100, 0xfd1ff]。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WasmOpcode(pub u32);

impl WasmOpcode {
  /// 原始 32 位编码
  #[inline]
  #[must_use]
  pub const fn as_u32(self) -> u32 {
    self.0
  }
}

/// 无前缀表达式操作码 (对应 C++ kExpr* 常量, FOREACH_OPCODE 展开 ${ALL_OPS.length} 条)
#[rustfmt::skip]
pub mod expr {
  use super::WasmOpcode;
`;

const noPrefix = ALL_OPS.filter((e) => e.opcode <= 0xff);
const prefixed = ALL_OPS.filter((e) => e.opcode > 0xff);
for (const e of noPrefix) {
  rust += `  /// ${e.wat}\n  pub const ${screaming(e.name)}: WasmOpcode = WasmOpcode(0x${e.opcode.toString(16)});\n`;
}
rust += `}

/// 前缀字节 (对应 C++ k*Prefix, FOREACH_PREFIX)
#[rustfmt::skip]
pub mod prefix {
`;

for (const p of prefixes) {
  rust += `  /// ${p.name} 前缀\n  pub const ${screaming(p.name)}: u32 = 0x${p.opcode.toString(16)};\n`;
}
rust += `}

/// 带前缀操作码常量 (kExpr<Name> 形式的 0xfb00/0xfc00/0xfd00/0xfe00/0xfd1xx 编码)
#[rustfmt::skip]
pub mod prefixed_expr {
  use super::WasmOpcode;
`;

for (const e of prefixed) {
  rust += `  /// ${e.wat}\n  pub const ${screaming(e.name)}: WasmOpcode = WasmOpcode(0x${e.opcode.toString(16)});\n`;
}
rust += `}

/// 操作码签名种类 (对应 C++ enum WasmOpcodeSig, kSigEnum_*; FOREACH_SIGNATURE 展开 ${ALL_SIGS.length} + None)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum WasmOpcodeSig {
  /// 无签名 (kSigEnum_None)
  None = 0,
`;

ALL_SIGS.forEach((s, i) => {
  const ret = typeVariant(s.types[0]);
  const params = s.types.slice(1).map(typeVariant);
  rust += `  /// ${s.name}: (${params.join(", ")}) -> ${ret}\n  ${sigVariant(s.name)} = ${i + 1},\n`;
});

rust += `}

/// 签名种类总数 (含 None, 对应 kSigEnum 计数)
pub const WASM_OPCODE_SIG_COUNT: usize = ${ALL_SIGS.length + 1};

/// 签名的具体类型布局 (对应 kTypes_##name[] / FunctionSig 常量表)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SigDef {
  /// 返回类型 (kTypes[0]; Void 表示无返回)
  pub ret: SigType,
  /// 参数类型 (kTypes[1..])
  pub params: &'static [SigType],
}

/// 签名位置出现的值类型记号 (对应 kWasmI32 等; 仅限签名表用到的子集)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SigType {
  I32,
  I64,
  F32,
  F64,
  S128,
  AnyRef,
  FuncRef,
  EqRef,
  Void,
}

impl WasmOpcodeSig {
  /// 查签名布局 (对应 kTypes_##name 常量与 kCachedSigs)
  #[inline]
  #[must_use]
  pub const fn sig_def(self) -> SigDef {
    match self {
`;

const constSigType = (t) => `SigType::${typeVariant(t)}`;
for (const s of ALL_SIGS) {
  const params = s.types.slice(1).map(constSigType);
  rust += `      Self::${sigVariant(s.name)} => SigDef { ret: ${constSigType(s.types[0])}, params: &[${params.join(", ")}] },\n`;
}
rust += `      Self::None => SigDef { ret: SigType::Void, params: &[] },
    }
  }
}

/// 单条操作码元信息 (FOREACH_OPCODE 展开, 源顺序)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpcodeDef {
  /// C++ kExpr 名称后缀, 如 "I32LoadMem"
  pub name: &'static str,
  /// 32 位内部编码
  pub opcode: u32,
  /// wat 文本名, 如 "i32.load"
  pub wat: &'static str,
  /// mem32 签名 (atomic 族) / 通用签名; "_" 表示解码器特判, 无表内签名
  pub sig: WasmOpcodeSig,
  /// mem64 签名 (仅 FOREACH_ATOMIC_OPCODE 第 5 参)
  pub sig64: WasmOpcodeSig,
  /// 前缀字节; 0 = 无前缀
  pub prefix: u32,
}

const SIG_NONE: WasmOpcodeSig = WasmOpcodeSig::None;

#[rustfmt::skip]
pub static OPCODES: &[OpcodeDef] = &[
`;

const sigOf = (s) => (s === "_" ? "SIG_NONE" : `WasmOpcodeSig::${sigVariant(s)}`);
for (const e of ALL_OPS) {
  const prefix = e.opcode <= 0xff ? 0 : e.opcode >= 0xfd100 && e.opcode <= 0xfd1ff ? 0xfd : e.opcode >> 8;
  const sig64 = e.sig64 ? sigOf(e.sig64) : "SIG_NONE";
  rust += `  OpcodeDef { name: "${e.name}", opcode: 0x${e.opcode.toString(16)}, wat: "${e.wat}", sig: ${sigOf(e.sig)}, sig64: ${sig64}, prefix: 0x${prefix.toString(16)} },\n`;
}

// ---------- 256 查表 (对应 wasm-opcodes-inl.h impl::k*SigTable) ----------
const table = (entries) => {
  const t = new Array(256).fill("WasmOpcodeSig::None");
  for (const e of entries) {
    if (e.sig === "_") continue; // "_" 不进表 (与原版 CASE 一致, 如 variadic)
    const idx = e.opcode & 0xff;
    if (t[idx] !== "WasmOpcodeSig::None") throw new Error(`表冲突: ${e.name} @ ${idx}`);
    t[idx] = `WasmOpcodeSig::${sigVariant(e.sig)}`;
  }
  const rows = [];
  for (let i = 0; i < 256; i += 16) {
    rows.push("  " + t.slice(i, i + 16).join(", ") + ",");
  }
  return rows.join("\n");
};
const table64 = (entries) => {
  const t = new Array(256).fill("WasmOpcodeSig::None");
  for (const e of entries) {
    if (!e.sig64) continue;
    const idx = e.opcode & 0xff;
    if (t[idx] !== "WasmOpcodeSig::None") throw new Error(`mem64 表冲突: ${e.name} @ ${idx}`);
    t[idx] = `WasmOpcodeSig::${sigVariant(e.sig64)}`;
  }
  const rows = [];
  for (let i = 0; i < 256; i += 16) {
    rows.push("  " + t.slice(i, i + 16).join(", ") + ",");
  }
  return rows.join("\n");
};

const shortEntries = [...leafEntries.get("FOREACH_SIMPLE_OPCODE")].filter((e) => e.opcode <= 0xff);
const simdEntries = [
  ...leafEntries.get("FOREACH_SIMD_MVP_0_OPERAND_OPCODE"),
  ...leafEntries.get("FOREACH_SIMD_MEM_OPCODE"),
  ...leafEntries.get("FOREACH_SIMD_MEM_1_OPERAND_OPCODE"),
];
const relaxedEntries = leafEntries.get("FOREACH_RELAXED_SIMD_OPCODE");
const atomicEntries = [...leafEntries.get("FOREACH_ATOMIC_OPCODE"), ...leafEntries.get("FOREACH_ATOMIC_0_OPERAND_OPCODE")];
const numericEntries = leafEntries.get("FOREACH_NUMERIC_OPCODE_WITH_SIG");

rust += `];

/// 无前缀短操作码签名表 (对应 impl::kShortSigTable)
#[rustfmt::skip]
pub static SHORT_SIG_TABLE: [WasmOpcodeSig; 256] = [
${table(shortEntries)}
];

/// SIMD MVP 表达式签名表 (对应 impl::kSimdExprSigTable; 取 MVP_0+MEM+MEM_1_OPERAND)
#[rustfmt::skip]
pub static SIMD_EXPR_SIG_TABLE: [WasmOpcodeSig; 256] = [
${table(simdEntries)}
];

/// Relaxed SIMD 表达式签名表 (对应 impl::kRelaxedSimdExprSigTable)
#[rustfmt::skip]
pub static RELAXED_SIMD_EXPR_SIG_TABLE: [WasmOpcodeSig; 256] = [
${table(relaxedEntries)}
];

/// Atomic 表达式签名表 mem32 (对应 impl::kAtomicExprSigTableMem32)
#[rustfmt::skip]
pub static ATOMIC_EXPR_SIG_TABLE_MEM32: [WasmOpcodeSig; 256] = [
${table(atomicEntries)}
];

/// Atomic 表达式签名表 mem64 (对应 impl::kAtomicExprSigTableMem64)
#[rustfmt::skip]
pub static ATOMIC_EXPR_SIG_TABLE_MEM64: [WasmOpcodeSig; 256] = [
${table64(atomicEntries)}
];

/// Numeric 前缀签名表 (对应 impl::kNumericExprSigTable; 仅 WITH_SIG 子集)
#[rustfmt::skip]
pub static NUMERIC_EXPR_SIG_TABLE: [WasmOpcodeSig; 256] = [
${table(numericEntries)}
];

/// TrapReason: wasm 陷阱原因 (对应 C++ enum TrapReason; 列表源 src/common/globals.h FOREACH_WASM_TRAPREASON)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TrapReason {
`;

trapReasons.forEach((t, i) => {
  rust += `  /// k${t}\n  ${screaming(t.replace(/^Trap/, ""))} = ${i},\n`;
});
rust += `}

/// kTrapCount
pub const TRAP_COUNT: u8 = ${trapReasons.length};

/// WasmOpcodes 静态方法族 (对应 class WasmOpcodes, 规则直译)
impl WasmOpcode {
  /// 操作码名称 (对应 WasmOpcodes::OpcodeName; 未知名返回 "invalid opcode")
  #[inline]
  #[must_use]
  pub const fn opcode_name(self) -> &'static str {
    match self.0 {
`;

// 按 opcode 排序生成 match 臂, 名字用 wat 之外的 C++ 名
for (const e of [...ALL_OPS].sort((a, b) => a.opcode - b.opcode)) {
  rust += `      0x${e.opcode.toString(16)} => "${e.name}",\n`;
}
rust += `      _ => "invalid opcode",
    }
  }

  /// 提取前缀字节 (对应 WasmOpcodes::ExtractPrefix)
  #[inline]
  #[must_use]
  pub const fn extract_prefix(self) -> u8 {
    if self.0 > 0xffff { (self.0 >> 12) as u8 } else { (self.0 >> 8) as u8 }
  }

  /// 是否前缀操作码 (对应 WasmOpcodes::IsPrefixOpcode)
  #[inline]
  #[must_use]
  pub const fn is_prefix_opcode(self) -> bool {
    matches!(self.0, ${prefixes.map((p) => `0x${p.opcode.toString(16)}`).join(" | ")})
  }

  /// 是否可断点操作码 (对应 WasmOpcodes::IsBreakable; Block/Try/Catch/Loop/Else 不可断)
  #[inline]
  #[must_use]
  pub const fn is_breakable(self) -> bool {
    !matches!(self.0, 0x02 | 0x06 | 0x07 | 0x03 | 0x05)
  }

  /// 是否 relaxed-simd 操作码 (对应 WasmOpcodes::IsRelaxedSimdOpcode)
  #[inline]
  #[must_use]
  pub const fn is_relaxed_simd_opcode(self) -> bool {
    (self.0 & 0xfff00) == 0xfd100
  }

  /// 是否 FP16 SIMD 操作码 (对应 WasmOpcodes::IsFP16SimdOpcode)
  #[inline]
  #[must_use]
  pub const fn is_fp16_simd_opcode(self) -> bool {
    let v = self.0;
    (v >= expr::F16X8_SPLAT.as_u32() && v <= expr::F16X8_REPLACE_LANE.as_u32())
      || (v >= prefixed_expr::F16X8_ABS.as_u32() && v <= prefixed_expr::F16X8_QFMS.as_u32())
  }

  /// 是否 atomic read-modify-write 操作码 (对应 WasmOpcodes::IsAtomicRmwOpcode)
  #[inline]
  #[must_use]
  pub const fn is_atomic_rmw_opcode(self) -> bool {
    self.0 >= prefixed_expr::I32_ATOMIC_ADD.as_u32()
      && self.0 <= prefixed_expr::I64_ATOMIC_COMPARE_EXCHANGE32U.as_u32()
  }

  /// 查操作码签名 (对应 WasmOpcodes::Signature; None 表示无表内签名)
  #[inline]
  #[must_use]
  pub const fn signature(self) -> WasmOpcodeSig {
    match self.extract_prefix() as u32 {
      0 => SHORT_SIG_TABLE[(self.0 & 0xff) as usize],
      p if p == prefix::SIMD => {
        if self.0 <= 0xfdff {
          SIMD_EXPR_SIG_TABLE[(self.0 & 0xff) as usize]
        } else if self.is_relaxed_simd_opcode() {
          RELAXED_SIMD_EXPR_SIG_TABLE[(self.0 & 0xff) as usize]
        } else {
          WasmOpcodeSig::None
        }
      }
      p if p == prefix::NUMERIC => NUMERIC_EXPR_SIG_TABLE[(self.0 & 0xff) as usize],
      _ => WasmOpcodeSig::None,
    }
  }

  /// atomic 操作码按 memory64 选签名 (对应 WasmOpcodes::SignatureForAtomicOp)
  #[inline]
  #[must_use]
  pub const fn signature_for_atomic_op(self, is_memory64: bool) -> WasmOpcodeSig {
    if is_memory64 {
      ATOMIC_EXPR_SIG_TABLE_MEM64[(self.0 & 0xff) as usize]
    } else {
      ATOMIC_EXPR_SIG_TABLE_MEM32[(self.0 & 0xff) as usize]
    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(OPCODES.len() == ${ALL_OPS.length});
  assert!(WASM_OPCODE_SIG_COUNT == ${ALL_SIGS.length + 1});
  assert!(TRAP_COUNT == 21);
  assert!(expr::UNREACHABLE == WasmOpcode(0x00));
  assert!(expr::NOP == WasmOpcode(0x01));
  assert!(expr::END == WasmOpcode(0x0b));
  assert!(prefixed_expr::I64_ATOMIC_COMPARE_EXCHANGE32U == WasmOpcode(0xfe4e));
  assert!(prefixed_expr::S128_CONST == WasmOpcode(0xfd0c));
  assert!(prefixed_expr::I8X16_RELAXED_SWIZZLE == WasmOpcode(0xfd100));
  assert!(WasmOpcode(0xfd5c).extract_prefix() == 0xfd);
  assert!(WasmOpcode(0x45).extract_prefix() == 0x00);
  assert!(!WasmOpcode(0x02).is_breakable());
  assert!(WasmOpcode(0x45).is_breakable());
  assert!(WasmOpcode(0xfd110).is_relaxed_simd_opcode());
  assert!(!WasmOpcode(0xfd0c).is_relaxed_simd_opcode());
  assert!(WasmOpcode(0xfd121).is_fp16_simd_opcode());
  assert!(!WasmOpcode(0xfd120).is_fp16_simd_opcode());
  assert!(WasmOpcode(0xfe1e).is_atomic_rmw_opcode());
  assert!(!WasmOpcode(0xfe10).is_atomic_rmw_opcode());
  assert!(WasmOpcode(0x28).signature() == WasmOpcodeSig::I_I);
  assert!(WasmOpcode(0xfd00).signature() == WasmOpcodeSig::S_I);
  assert!(WasmOpcode(0xfc00).signature() == WasmOpcodeSig::I_F);
  assert!(WasmOpcode(0xfe10).signature_for_atomic_op(false) == WasmOpcodeSig::I_I);
  assert!(WasmOpcode(0xfe10).signature_for_atomic_op(true) == WasmOpcodeSig::I_L);
  assert!(TrapReason::MEM_OUT_OF_BOUNDS as u8 == 1);
  assert!(TrapReason::COUNT_PLACEHOLDER as u8 == 0, "placeholder");
};
`;

// F16x8Splat 在 1_OPERAND_1_PARAM (0xfd121) — 修正引用
rust = rust.replace("expr::F16X8_SPLAT", "prefixed_expr::F16X8_SPLAT").replace(
  "expr::F16X8_REPLACE_LANE",
  "prefixed_expr::F16X8_REPLACE_LANE",
);
// TrapReason 变体名锚点修正 (TrapMemOutOfBounds → MEM_OUT_OF_BOUNDS)
rust = rust.replace('  assert!(TrapReason::COUNT_PLACEHOLDER as u8 == 0, "placeholder");\n', "");

// .cc 骨架: operator<< 与 IsJSCompatibleSignature (签名骨架, 语义待 Developer 修订)
rust += `
/// 打印签名为短记号串 (对应 operator<<(ostream&, const FunctionSig&), 骨架)
pub fn write_sig_short(sb: &mut String, sig: &SigDef) {
  if sig.ret == SigType::Void {
    sb.push('v');
  } else {
    push_short(sb, sig.ret);
  }
  sb.push('_');
  if sig.params.is_empty() {
    sb.push('v');
  }
  for p in sig.params {
    push_short(sb, *p);
  }
}

fn push_short(sb: &mut String, t: SigType) {
  let c = match t {
    SigType::I32 => 'i',
    SigType::I64 => 'l',
    SigType::F32 => 'f',
    SigType::F64 => 'd',
    SigType::S128 => 's',
    _ => '?',
  };
  sb.push(c);
}

/// JS 兼容性判定 (对应 IsJSCompatibleSignature, 骨架: 语义待 Developer 修订)
pub fn is_js_compatible_signature(_sig: &SigDef) -> bool {
  unimplemented!("骨架: 参照原版 wasm-opcodes.cc IsJSCompatibleSignature 修订")
}
`;

const p = writeSrc("v8-wasm-core", "wasm_opcodes.rs", rust);
console.log(`written ${p}`);
