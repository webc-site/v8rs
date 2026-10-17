#!/usr/bin/env bun
// tool/src/regexp/regexp-bytecodes.h.js — v8/src/regexp/regexp-bytecodes.h (+inl.h 布局算法)
//   → v8-regexp-compiler/src/bytecodes_generated.rs
// 机械提取: 操作数类型 3 表 (BASIC/LIMITS/SPECIAL) + BytecodeFlag 位表 + 4 张 opcode 表
//   (INVALID/BASIC/SPECIAL/PEEPHOLE) + 逐条操作数名/类型/flags 元组;
// 对数断言: opcode 总数/分表数/操作数上限; JS 独立复刻 CalculateAlignedOffsets
//   (inl.h:107-136) 产出尺寸金标准 GOLDEN_SIZES;
// 同源交叉对数: v8-regexp-exec/src/regexp_interpreter.rs 的 Bytecode 枚举 (61 序数)
//   + BYTECODE_LAYOUTS 操作数类型序列 + OperandType 宽度表, 与本表逐项比对, 缺项 throw。
import { banner, readV8, writeSrc, root } from "../../lib.js";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const src = joinContinuations(readV8("src/regexp/regexp-bytecodes.h"));

// 拼接 C 宏续行, 使每个 #define 体落为单行
function joinContinuations(text) {
  return text.replace(/\\\r?\n\s*/g, " ");
}

// 取 #define NAME(V) ... 的体 (续行已拼, 块注释已剥)
function macroBody(name) {
  const m = src.match(new RegExp(`#define ${name}\\(V\\) (.*)`));
  if (!m) throw new Error(`regexp-bytecodes.h: ${name} 结构漂移`);
  return m[1].replace(/\/\*[\s\S]*?\*\//g, " ");
}

// 扫描 V(...) 条目, 括号配平, 返回顶层逗号分割的参数数组
function entries(body) {
  const out = [];
  let i = 0;
  while (i < body.length) {
    const start = body.indexOf("V(", i);
    if (start < 0) break;
    let depth = 0;
    let end = start + 1; // '(' of V(
    for (; end < body.length; end++) {
      if (body[end] === "(") depth++;
      else if (body[end] === ")") {
        depth--;
        if (depth === 0) break;
      }
    }
    const inner = body.slice(start + 2, end);
    out.push(splitTopLevel(inner));
    i = end + 1;
  }
  if (out.length === 0) throw new Error("宏体零条目");
  return out;
}

// 顶层逗号分割 (忽略括号内)
function splitTopLevel(s) {
  const parts = [];
  let depth = 0;
  let cur = "";
  for (const ch of s) {
    if (ch === "(") depth++;
    if (ch === ")") depth--;
    if (ch === "," && depth === 0) {
      parts.push(cur.trim());
      cur = "";
    } else cur += ch;
  }
  if (cur.trim()) parts.push(cur.trim());
  return parts;
}

function tupleNames(s) {
  const m = s.match(/^\((.*)\)$/);
  if (!m) throw new Error(`元组形态漂移: ${s}`);
  return splitTopLevel(m[1]).filter((x) => x.length > 0);
}

// ── 操作数类型 3 表 ─────────────────────────────────────────────────────────
const CT_SIZE = {
  "int16_t": 2,
  "int32_t": 4,
  "uint32_t": 4,
  "base::uc16": 2,
  "uint16_t": 2,
  "RegExpMacroAssembler::StackCheckFlag": 1,
  "StandardCharacterSet": 1,
};
const CT_LIMITS = {
  "int16_t": [-32768, 32767],
  "int32_t": [-2147483648, 2147483647],
  "uint32_t": [0, 4294967295],
  "base::uc16": [0, 65535],
  "uint16_t": [0, 65535],
};

// 限幅符号求值 (值源 regexp-macro-assembler.h:37-46 / regexp-ast.h:102-112)
const LIMIT_CONST = {
  "RegExpMacroAssembler::kMinCPOffset": -32767,
  "RegExpMacroAssembler::kMaxCPOffset": 32767,
  "RegExpMacroAssembler::kMaxEatsAtLeastValue": 255,
  "RegExpMacroAssembler::kMaxRegister": 65535,
  "RegExpMacroAssembler::StackCheckFlag::kNoStackLimitCheck": 0,
  "RegExpMacroAssembler::StackCheckFlag::kCheckStackLimit": 1,
  "StandardCharacterSet::kEverything": 42, // '*'
  "StandardCharacterSet::kWord": 119, // 'w'
};
function evalLimit(rawTok) {
  const tok = rawTok.trim();
  const plus = tok.match(/^(.+?)\s*\+\s*(.+)$/);
  if (plus) return evalLimit(plus[1]) + evalLimit(plus[2]);
  if (/^-?\d+$/.test(tok)) return Number(tok);
  if (!(tok in LIMIT_CONST)) throw new Error(`限幅符号未登记: ${tok}`);
  return LIMIT_CONST[tok];
}

// 与 regexp-macro-assembler.h 实测常量交叉 (防上游漂移)
const masmSrc = joinContinuations(readV8("src/regexp/regexp-macro-assembler.h"));
for (const [sym, expect] of [
  ["kMaxCPOffset = (1 << 15) - 1", 32767],
  ["kMaxRegisterCount = (1 << 16)", 65536],
  ["kMaxRegister = kMaxRegisterCount - 1", 65535],
]) {
  if (!masmSrc.includes(sym)) throw new Error(`macro-assembler.h 漂移: ${sym}`);
}
if (!/kMaxEatsAtLeastValue =\s*std::numeric_limits<uint8_t>::max\(\)/.test(masmSrc)) {
  throw new Error("macro-assembler.h 漂移: kMaxEatsAtLeastValue");
}

const operandTypes = [];
for (const [name, ctype] of entries(macroBody("BASIC_BYTECODE_OPERAND_TYPE_LIST"))) {
  operandTypes.push({
    name,
    size: CT_SIZE[ctype],
    align: CT_SIZE[ctype],
    basic: true,
    limits: CT_LIMITS[ctype],
    note: ctype,
  });
}
for (const [name, ctype, min, max] of entries(macroBody("BASIC_BYTECODE_OPERAND_TYPE_LIMITS_LIST"))) {
  operandTypes.push({
    name,
    size: CT_SIZE[ctype],
    align: CT_SIZE[ctype],
    basic: true,
    limits: [evalLimit(min), evalLimit(max)],
    note: `${ctype} 限幅`,
  });
}
for (const [name, size, align] of entries(macroBody("SPECIAL_BYTECODE_OPERAND_TYPE_LIST"))) {
  operandTypes.push({
    name,
    size: Number(size),
    align: Number(align),
    basic: false,
    limits: [0, 0], // 无 C 型限幅 (inl.h:60-69 traits 不含 min/max 使用面)
    note: `专列 ${size}/${align}`,
  });
}
const opIndex = new Map(operandTypes.map((t, i) => [t.name, i]));
if (operandTypes.length !== 11) throw new Error(`操作数类型应为 11, 实数 ${operandTypes.length}`);

// ── BytecodeFlag 位表 ──────────────────────────────────────────────────────
const flagBlock = src.match(/enum class BytecodeFlag : uint32_t \{([\s\S]*?)\};/)[1];
const bytecodeFlags = [...flagBlock.matchAll(/k(\w+) = 1 << (\d+)/g)].map((r) => ({
  name: r[1],
  bit: Number(r[2]),
}));
if (bytecodeFlags.length !== 4) throw new Error("BytecodeFlag 位表漂移");

// ── 4 张 opcode 表 ─────────────────────────────────────────────────────────
const sections = ["INVALID_BYTECODE_LIST", "BASIC_BYTECODE_LIST", "SPECIAL_BYTECODE_LIST", "PEEPHOLE_BYTECODE_LIST"];
const order = src.match(/#define REGEXP_BYTECODE_LIST\(V\)(.*)/)[1];
for (const s of sections) {
  if (!order.includes(s)) throw new Error(`REGEXP_BYTECODE_LIST 缺 ${s}`);
}
// 顺序须与展开宏一致
const sectionOrder = sections.slice().sort((a, b) => order.indexOf(a) - order.indexOf(b));

const bytecodes = [];
const sectionSpans = {};
for (const sec of sectionOrder) {
  const from = bytecodes.length;
  for (const [camel, opNames, opTypes, flagTuple] of entries(macroBody(sec))) {
    const names = tupleNames(opNames);
    const types = tupleNames(opTypes).map((t) => {
      const m = t.match(/^ReBcOpType::k(\w+)$/);
      if (!m || !opIndex.has(m[1])) {
        throw new Error(`${camel}: 未知操作数类型 ${t}`);
      }
      return m[1];
    });
    let flags = 0;
    for (const f of tupleNames(flagTuple)) {
      const m = f.match(/^ReBcFlag::k(\w+)$/);
      if (!m) throw new Error(`${camel}: 未知 flag ${f}`);
      const bit = bytecodeFlags.find((x) => x.name === m[1]);
      if (!bit) throw new Error(`${camel}: flag 位缺失 ${f}`);
      flags |= 1 << bit.bit;
    }
    if (names.length !== types.length) {
      throw new Error(`${camel}: 操作数名/类型数不齐 ${names.length} vs ${types.length}`);
    }
    bytecodes.push({ camel, names, types, flags, section: sec });
  }
  sectionSpans[sec] = { from, to: bytecodes.length };
}
if (bytecodes.length !== 61) throw new Error(`opcode 应 61, 实数 ${bytecodes.length}`);
if (bytecodes[0].camel !== "Break" || bytecodes[0].types.length !== 0) {
  throw new Error("Break 必须为 0 号且零操作数 (h:88 注释)");
}

// ── 金标准尺寸: JS 独立复刻 inl.h:107-136 CalculateAlignedOffsets ──────────
const ALIGN = 4;
const roundUp = (v, a) => Math.ceil(v / a) * a;
const goldenSizes = bytecodes.map((bc) => {
  let offset = 1; // first_offset = sizeof(Bytecode) (inl.h:116)
  for (const t of bc.types) {
    const { size, align } = operandTypes[opIndex.get(t)];
    offset = roundUp(offset, align); // inl.h:123
    if ((offset % ALIGN) + size > ALIGN) offset = roundUp(offset, ALIGN); // inl.h:127-129
    offset += size;
  }
  return roundUp(offset, ALIGN); // kSize (inl.h:149-151); 零操作数 → roundUp(1,4)=4
});
const maxOperandCount = Math.max(...bytecodes.map((b) => b.types.length));

// ── 同源交叉对数: v8-regexp-exec 解释器面既有表 ────────────────────────────
const execSrc = readFileSync(join(root, "v8-regexp-exec", "src", "regexp_interpreter.rs"), "utf8");
const execEnum = execSrc.match(/pub enum Bytecode \{([\s\S]*?)\n\}/)[1];
const execOpcodes = [...execEnum.matchAll(/^ {2}(\w+) = (\d+),$/gm)].map((r) => ({ name: r[1], value: Number(r[2]) }));
if (execOpcodes.length !== bytecodes.length) {
  throw new Error(`exec Bytecode 枚举对数不齐: exec ${execOpcodes.length} vs C++ ${bytecodes.length}`);
}
for (let i = 0; i < bytecodes.length; i++) {
  if (execOpcodes[i].name !== bytecodes[i].camel || execOpcodes[i].value !== i) {
    throw new Error(`exec Bytecode[${i}] 漂移: ${execOpcodes[i].name}=${execOpcodes[i].value}`);
  }
}
const execLayouts = execSrc.match(/const BYTECODE_LAYOUTS[^=]*= \[[\s\S]*?\n\];/)[0];
const execTypeSeqs = [...execLayouts.matchAll(/operand_layout\(&\[([\s\S]*?)\]\)/g)].map((m) =>
  [...m[1].matchAll(/OperandType::(\w+)/g)].map((r) => r[1]),
);
if (execTypeSeqs.length !== bytecodes.length) {
  throw new Error(`exec BYTECODE_LAYOUTS 对数不齐: ${execTypeSeqs.length} vs ${bytecodes.length}`);
}
for (let i = 0; i < bytecodes.length; i++) {
  const a = bytecodes[i].types.join(",");
  const b = execTypeSeqs[i].join(",");
  if (a !== b) throw new Error(`exec 布局[${i}] ${bytecodes[i].camel} 漂移: C++ [${a}] vs exec [${b}]`);
}
const execSizeFn = execSrc.match(/pub const fn size\(self\) -> u8 \{([\s\S]*?)\n {2}\}/)[1];
const execWidths = new Map();
// 逐 arm 行解析: "<Self::A | Self::B ...> => N," 左侧取全部变体名
for (const line of execSizeFn.split("\n")) {
  const m = line.match(/^(.*?) => (\d+),?/);
  if (!m) continue;
  for (const r of m[1].matchAll(/Self::(\w+)/g)) execWidths.set(r[1], Number(m[2]));
}
if (execWidths.size !== operandTypes.length) {
  throw new Error(`exec OperandType 宽度表对数不齐: ${execWidths.size} vs ${operandTypes.length}`);
}
for (const t of operandTypes) {
  if (execWidths.get(t.name) !== t.size) {
    throw new Error(`exec 宽度漂移: ${t.name} exec=${execWidths.get(t.name)} vs C++=${t.size}`);
  }
}
// flag 位非重复校验
{
  const bits = bytecodeFlags.map((f) => f.bit);
  if (new Set(bits).size !== bits.length) throw new Error("flag 位重复");
}
const sectionSummary = sectionOrder
  .map((s) => `${s.replace("_BYTECODE_LIST", "")}=${sectionSpans[s].to - sectionSpans[s].from}`)
  .join(" ");
console.log(
  `regexp-bytecodes: opcode ${bytecodes.length} (${sectionSummary}), 操作数类型 ${operandTypes.length}, ` +
    `flag 位 ${bytecodeFlags.length}, 操作数上限 ${maxOperandCount}, 尺寸合计 ${goldenSizes.reduce((a, b) => a + b, 0)} ✓`,
);
console.log(`exec 交叉对数: Bytecode ${execOpcodes.length}/${bytecodes.length} ✓ 布局 ${execTypeSeqs.length}/${bytecodes.length} ✓ 宽度 ${execWidths.size}/${operandTypes.length} ✓`);

// ── 生成 Rust ──────────────────────────────────────────────────────────────
const rust = [];
rust.push(banner(
  "src/regexp/regexp-bytecodes.h.js",
  "src/regexp/regexp-bytecodes.h",
  "四表 opcode + 操作数类型三表 + flag 位表 + GOLDEN_SIZES 布局金标准; 布局算法/编解码在 bytecodes.rs, Dump 在 dump.rs; 同源性: v8-regexp-exec/src/regexp_interpreter.rs (脚本交叉对数)",
));
rust.push(`
// ── 操作数类型 (BASIC + LIMITS + SPECIAL 展开, 序数即 BYTECODE_OPERAND_TYPE_LIST 序) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BytecodeOperandType {`);
for (let i = 0; i < operandTypes.length; i++) {
  const t = operandTypes[i];
  rust.push(`  /// ${t.note}, 宽 ${t.size} 对齐 ${t.align}`);
  rust.push(`  ${t.name} = ${i},`);
}
rust.push(`}

pub const K_OPERAND_TYPE_COUNT: usize = ${operandTypes.length};
const _: () = assert!(BytecodeOperandType::${operandTypes[operandTypes.length - 1].name} as usize + 1 == K_OPERAND_TYPE_COUNT);

pub const OPERAND_TYPE_NAMES: [&str; K_OPERAND_TYPE_COUNT] = [
${operandTypes.map((t) => `  "${t.name}",`).join("\n")}
];

/// OperandTypeTraits::kSize (inl.h:30/45/63)
pub const OPERAND_TYPE_SIZES: [u8; K_OPERAND_TYPE_COUNT] = [
${operandTypes.map((t) => `  ${t.size},`).join("\n")}
];

/// OperandTypeTraits::kAlignment (inl.h:35/54/65: basic=自身宽, BitTable=1)
pub const OPERAND_TYPE_ALIGNMENTS: [u8; K_OPERAND_TYPE_COUNT] = [
${operandTypes.map((t) => `  ${t.align},`).join("\n")}
];

/// OperandTypeTraits::kIsBasic (inl.h:32/53/64: 仅 BitTable 为 false)
pub const OPERAND_TYPE_IS_BASIC: [bool; K_OPERAND_TYPE_COUNT] = [
${operandTypes.map((t) => `  ${t.basic},`).join("\n")}
];

/// OperandTypeTraits::kMinValue / kMaxValue (inl.h:33-34/51-52; SPECIAL 无 C 型限幅, (0,0) 占位)
pub const OPERAND_TYPE_LIMITS: [(i64, i64); K_OPERAND_TYPE_COUNT] = [
${operandTypes.map((t) => `  (${t.limits[0]}, ${t.limits[1]}),`).join("\n")}
];

// ── 分析用 flag 位 (BytecodeFlag, h:59-79) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum BytecodeFlag {`);
for (const f of bytecodeFlags) {
  rust.push(`  /// 1 << ${f.bit}`);
  rust.push(`  ${f.name} = 1 << ${f.bit},`);
}
rust.push(`}

// ── opcode 全集 (INVALID+BASIC+SPECIAL+PEEPHOLE 四表展开序) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Bytecode {`);
let lastSection = null;
for (let i = 0; i < bytecodes.length; i++) {
  const bc = bytecodes[i];
  if (bc.section !== lastSection) {
    lastSection = bc.section;
    const span = sectionSpans[bc.section];
    rust.push(`  // ${bc.section} ↓${span.from}..=${span.to - 1}`);
  }
  rust.push(`  ${bc.camel} = ${i},`);
}
rust.push(`
  // 原版 kLast = -1 +1×61 (h:340-343) — 末项即最后实码
}

pub const K_BYTECODE_COUNT: u8 = ${bytecodes.length};
pub const K_BYTECODE_ALIGNMENT: usize = ${ALIGN};
/// 原版 kCount = ToByte(kLast)+1 (h:355); kLast 即末位
pub const K_LAST: Bytecode = Bytecode::${bytecodes[bytecodes.length - 1].camel};
const _: () = assert!(K_LAST as u8 + 1 == K_BYTECODE_COUNT);

pub const ALL_BYTECODES: [Bytecode; K_BYTECODE_COUNT as usize] = [
${bytecodes.map((b) => `  Bytecode::${b.camel},`).join("\n")}
];

/// detail::kBytecodeNames (inl.h:307-310, #CamelName)
pub const BYTECODE_NAMES: [&str; K_BYTECODE_COUNT as usize] = [
${bytecodes.map((b) => `  "${b.camel}",`).join("\n")}
];

/// 逐条操作数名 (BytecodeOperandNames::kNames, inl.h:157-169 SplitNames)
pub(crate) const BYTECODE_OPERAND_NAMES: [&[&str]; K_BYTECODE_COUNT as usize] = [
${bytecodes.map((b) => `  &[${b.names.map((n) => `"${n}"`).join(", ")}],`).join("\n")}
];

/// 逐条操作数类型 (BytecodeOperandsTraits::kOperandTypes, inl.h:141-142)
pub(crate) const BYTECODE_OPERAND_TYPES: [&[BytecodeOperandType]; K_BYTECODE_COUNT as usize] = [
${bytecodes
  .map((b) => `  &[${b.types.map((t) => `BytecodeOperandType::${t}`).join(", ")}], // ${b.camel}`)
  .join("\n")}
];

/// detail::kBytecodeFlags (inl.h:318-322)
pub(crate) const BYTECODE_FLAGS: [u32; K_BYTECODE_COUNT as usize] = [
${bytecodes.map((b) => `  ${b.flags}, // ${b.camel}`).join("\n")}
];

/// 全集最大操作数个数 (原版 SkipUntilOneOfMasked3 20 项)
pub const K_MAX_OPERAND_COUNT: usize = ${maxOperandCount};

/// 脚本侧独立复刻 inl.h:107-136 的 4 字节块对齐排布所得尺寸基线 —
/// bytecodes.rs 编译期算法表与其逐项 assert (双实现交叉验证)
pub(crate) const GOLDEN_SIZES: [u8; K_BYTECODE_COUNT as usize] = [
${bytecodes.map((b, i) => `  ${goldenSizes[i]}, // ${b.camel}`).join("\n")}
];
`);

writeSrc("v8-regexp-compiler", "bytecodes_generated.rs", rust.join("\n") + "\n");
console.log("written: v8-regexp-compiler/src/bytecodes_generated.rs");
