#!/usr/bin/env bun
// tool/src/ast/variables.h.js
// 机械提取 v8/src/ast/variables.h 中的空洞检测枚举/位标志/位图常量

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/ast/variables.h");

// 1. 提取 HoleCheckState 枚举
const stateMatch = src.match(/enum class HoleCheckState : uint8_t \{([\s\S]*?)\};/);
if (!stateMatch) throw new Error("未找到 HoleCheckState");
const states = [...stateMatch[1].matchAll(/k(\w+)\s*=\s*(\d+)/g)].map((m) => ({
  name: m[1],
  value: parseInt(m[2], 10),
}));

// 2. 提取 ForceHoleInitializationFlag 位标志
const flagMatch = src.match(/enum ForceHoleInitializationFlag \{([\s\S]*?)\};/);
if (!flagMatch) throw new Error("未找到 ForceHoleInitializationFlag");
// kHasHoleCheckUseInUnknownScope = A | B 由 JS 侧按位求值; 末项无尾逗号, 终止符取 , 或 }
const flagNames = new Map();
const flagItems = [];
for (const m of flagMatch[1].matchAll(/k(\w+)\s*=\s*([^,}]+)/g)) {
  const [name, expr] = [m[1], m[2].trim()];
  flagNames.set(name, expr);
  flagItems.push({ name, expr });
}
// 位表达式求值: 支持 "0"、"1 << N"、"标识符 | 标识符"
function evalFlagExpr(expr, resolved = new Set()) {
  const orParts = expr.split("|").map((s) => s.trim());
  let value = 0;
  for (const part of orParts) {
    const shift = part.match(/^1\s*<<\s*(\d+)$/);
    if (shift) {
      value |= 1 << parseInt(shift[1], 10);
      continue;
    }
    if (/^\d+$/.test(part)) {
      value |= parseInt(part, 10);
      continue;
    }
    const ref = part.match(/^k(\w+)$/);
    if (ref && !resolved.has(ref[1])) {
      resolved.add(ref[1]);
      value |= evalFlagExpr(flagNames.get(ref[1]), resolved);
      continue;
    }
    throw new Error(`无法求值位标志表达式: ${expr}`);
  }
  return value;
}
const flags = flagItems.map((f) => ({ name: f.name, value: evalFlagExpr(f.expr) }));

// 3. 提取位图类型与常量: HoleCheckBitmap 与位数
const bitmapMatch = src.match(/using HoleCheckBitmap = (\w+);/);
if (!bitmapMatch) throw new Error("未找到 HoleCheckBitmap");
const bitmapType = bitmapMatch[1]; // uint64_t
const bitmapBits = { uint64_t: 64, uint32_t: 32, uint16_t: 16, uint8_t: 8 }[bitmapType];
if (bitmapBits === undefined) throw new Error(`未知位图类型 ${bitmapType}`);

const idxMatch = src.match(/static constexpr uint8_t kUncacheableHoleCheckBitmapIndex = (\d+);/);
if (!idxMatch) throw new Error("未找到 kUncacheableHoleCheckBitmapIndex");
const uncacheableIndex = parseInt(idxMatch[1], 10);

console.log(
  `提取变量层常量: HoleCheckState=${states.map((s) => s.name).join("/")}, 位标志=${flags.map((f) => `${f.name}=${f.value}`).join(" ")}, 位图=${bitmapType}(${bitmapBits}位), 不可缓存索引=${uncacheableIndex}`,
);

// Rust 位标志常量名: k 前缀 snake_case → UPPER_SNAKE
function flagConstName(name) {
  return name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
}

// 4. 生成 Rust 代码: variables_generated.rs
const out = [
  banner("src/ast/variables.h.js", "src/ast/variables.h", "变量层空洞检测枚举、位标志与位图常量"),
  `use strum::{Display, FromRepr};`,
  ``,
  `/// 空洞检测状态 (原版 enum class Variable::HoleCheckState)`,
  `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]`,
  `#[repr(u8)]`,
  `pub enum HoleCheckState {`,
  ...states.map((s) => `  ${s.name} = ${s.value},`),
  `}`,
  ``,
  `/// 空洞检测位图 (原版 using HoleCheckBitmap)`,
  `pub type HoleCheckBitmap = u64;`,
  ``,
  `/// 空洞初始化强制位标志 (原版 enum ForceHoleInitializationFlag)`,
  `pub mod force_hole_init {`,
  ...flags.flatMap((f) => [
    `  /// 原版 k${f.name} = ${f.value}`,
    `  pub const ${flagConstName(f.name)}: u8 = ${f.value};`,
  ]),
  `}`,
  ``,
  `/// 不可缓存空洞检测位图索引 (原版 kUncacheableHoleCheckBitmapIndex)`,
  `pub const UNCACHEABLE_HOLE_CHECK_BITMAP_INDEX: u8 = ${uncacheableIndex};`,
  ``,
  `/// 位图可用位数 (原版 kHoleCheckBitmapBits = numeric_limits<HoleCheckBitmap>::digits)`,
  `pub const HOLE_CHECK_BITMAP_BITS: u8 = ${bitmapBits};`,
  ``,
  `// ── 对数不变式断言 ──`,
  `const _: () = {`,
  `  assert!(HoleCheckState::Uncached as u8 == ${states[0].value});`,
  ...flags.map((f) => `  assert!(force_hole_init::${flagConstName(f.name)} == ${f.value});`),
  `  assert!(UNCACHEABLE_HOLE_CHECK_BITMAP_INDEX == ${uncacheableIndex});`,
  `  assert!(HOLE_CHECK_BITMAP_BITS == ${bitmapBits});`,
  `  assert!(HOLE_CHECK_BITMAP_BITS == <HoleCheckBitmap>::BITS as u8);`,
  `};`,
  ``,
];

const writtenPath = writeSrc("v8-ast", "variables_generated.rs", out.join("\n"));
console.log(`生成成功: ${writtenPath}`);
