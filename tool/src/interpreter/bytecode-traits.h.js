#!/usr/bin/env bun
// tool/src/interpreter/bytecode-traits.h.js
// 机械提取 v8/src/interpreter/bytecode-traits.h 的操作数缩放表 (OperandScaler 模板 → 显式常量表)
// → v8-interpreter/src/bytecode_traits.rs
//
// 原版是 C++ 模板 (OperandTypeInfoTraits / OperandTraits / OperandScaler /
// BytecodeTraits), 无运行时表; Rust 侧落为按操作数类型展开的
// OPERAND_KIND_SIZES[3][23] (对齐 bytecodes.cc kOperandKindSizes) + 缩放公式。
// 每字节码的 traits 展开 (kOperandTypes/kOperandSizes/kOperandOffsets/...) 已在
// bytecodes.rs 表化, 本模块只持操作数类型维度。
import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/interpreter/bytecode-traits.h");
const opHSrc = readV8("src/interpreter/bytecode-operands.h");

// 结构锚: 模板定义原文
for (const [needle, name] of [
  ["template <OperandType operand_type, OperandScale operand_scale>\nstruct OperandScaler {", "OperandScaler 模板"],
  ["static constexpr OperandSize kOperandSize = static_cast<OperandSize>(kSize);", "kOperandSize 公式"],
  ["OPERAND_TYPE_LIST(DECLARE_OPERAND_TYPE_TRAITS)", "OperandTraits 全特化展开"],
  ["OPERAND_TYPE_INFO_LIST(DECLARE_OPERAND_TYPE_INFO)", "TypeInfoTraits 全特化展开"],
]) {
  if (!hSrc.includes(needle)) throw new Error(`结构锚缺失: ${name}`);
}

// 操作数类型表与元信息 (OPERAND_TYPE_LIST, 对数 23 = clang -E 基准)
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
const entries = [];
for (const m of [
  "INVALID_OPERAND_TYPE_LIST",
  "UNSIGNED_FIXED_SCALAR_OPERAND_TYPE_LIST",
  "UNSIGNED_SCALABLE_SCALAR_OPERAND_TYPE_LIST",
  "SIGNED_SCALABLE_SCALAR_OPERAND_TYPE_LIST",
  "REGISTER_INPUT_OPERAND_TYPE_LIST",
  "REGISTER_OUTPUT_OPERAND_TYPE_LIST",
]) {
  const body = rawMacroBody(opHSrc, m);
  const re = /V\s*\(([^()]*)\)/g;
  let mm;
  while ((mm = re.exec(body)) !== null) {
    entries.push(mm[1].split(",").map((s) => s.trim()).filter(Boolean));
  }
}
entries.push(["RegInOut", "OperandTypeInfo::kScalableSignedByte"]);
if (entries.length !== 23) throw new Error(`OPERAND_TYPE_LIST 对数不符: 期望 23 实得 ${entries.length}`);

// OperandScaler 公式: size = unscaled_size(info) * (scalable ? scale : 1)
const unscaledSize = { None: 0, ScalableSignedByte: 1, ScalableUnsignedByte: 1, FixedUnsignedByte: 1, FixedUnsignedShort: 2 };
const isScalable = (info) => info === "ScalableSignedByte" || info === "ScalableUnsignedByte";
const scales = [1, 2, 4];
const kindSizes = entries.map(([name, infoRaw]) => {
  const info = infoRaw.replace("OperandTypeInfo::k", "");
  return scales.map((sc) => unscaledSize[info] * (isScalable(info) ? sc : 1));
});
// 守卫: 可缩放类随刻度翻倍, 固定类不变 (抽检 Imm/Flag16)
const immIdx = entries.findIndex(([n]) => n === "Imm");
const flag16Idx = entries.findIndex(([n]) => n === "Flag16");
if (JSON.stringify(kindSizes[immIdx]) !== "[1,2,4]") throw new Error(`Imm 缩放不符: ${kindSizes[immIdx]}`);
if (JSON.stringify(kindSizes[flag16Idx]) !== "[2,2,2]") throw new Error(`Flag16 固定不符: ${kindSizes[flag16Idx]}`);

const pascal = (s) => s.replace(/_/g, "");
const sizeName = (v) => ({ 0: "None", 1: "Byte", 2: "Short", 4: "Quad" })[v];

let rust = banner("src/interpreter/bytecode-traits.h.js", "src/interpreter/bytecode-traits.h", "操作数缩放 traits 模板 → 显式常量表 (OperandScaler 展开; 对数 OPERAND_TYPE_LIST=23)");

rust += `
use crate::bytecode_operands::{OperandScale, OperandSize, OperandType, OperandTypeInfo};

/// 每操作数类型在三档刻度下的尺寸表 (原版 kOperandKindSizes[3][23]; 行 = Single/Double/Quadruple)
/// 由 OperandScaler<OperandType::kN, OperandScale::kS>::kOperandSize 全展开
#[rustfmt::skip]
pub static OPERAND_KIND_SIZES: [[OperandSize; 23]; 3] = [
`;
for (let s = 0; s < 3; s++) {
  rust += `  [` + kindSizes.map((row) => `OperandSize::${sizeName(row[s])}`).join(", ") + `],\n`;
}
rust += `];

/// 操作数类型列表 (OPERAND_TYPE_LIST 顺序, 供对照)
pub const OPERAND_TYPE_ORDER: [OperandType; 23] = [
${entries.map(([n]) => `  OperandType::${pascal(n)},`).join("\n")}
];

/// 缩放公式原样化 (原版 OperandScaler::kSize 静态计算)
/// size = 元信息基准尺寸 × (可缩放 ? 刻度值 : 1)
#[inline]
#[must_use]
pub const fn operand_scaled_size(info: OperandTypeInfo, scale: OperandScale) -> usize {
  let base = info.unscaled_size() as usize;
  if info.is_scalable() {
    base * (scale as usize)
  } else {
    base
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(OPERAND_TYPE_ORDER.len() == 23);
  assert!(operand_scaled_size(OperandTypeInfo::ScalableSignedByte, OperandScale::Quadruple) == 4);
  assert!(operand_scaled_size(OperandTypeInfo::FixedUnsignedShort, OperandScale::Quadruple) == 2);
  assert!(operand_scaled_size(OperandTypeInfo::None, OperandScale::Single) == 0);
  assert!(OPERAND_KIND_SIZES[0][OperandType::Imm as usize] as u8 == OperandSize::Byte as u8);
  assert!(OPERAND_KIND_SIZES[2][OperandType::Imm as usize] as u8 == OperandSize::Quad as u8);
  assert!(OPERAND_KIND_SIZES[1][OperandType::Flag16 as usize] as u8 == OperandSize::Short as u8);
};
`;

const p = writeSrc("v8-interpreter", "bytecode_traits.rs", rust);
console.log(`written ${p} (${entries.length} operand types × 3 scales)`);
