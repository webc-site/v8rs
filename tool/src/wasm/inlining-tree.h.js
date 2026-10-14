#!/usr/bin/env bun
// tool/src/wasm/inlining-tree.h.js
// 机械提取 v8/src/wasm/inlining-tree.h 的内联决策常量

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/inlining-tree.h");

function mustConst(re, what) {
  const m = hSrc.match(re);
  if (!m) throw new Error(`未找到 ${what}`);
  return m[1];
}

const modeMatch = hSrc.match(/enum class Mode \{ ([^}]*) \};/);
if (!modeMatch) throw new Error("未找到 Mode");
const modes = modeMatch[1].split(",").map((s) => s.trim());
if (modes.join(",") !== "kUninitialized,kVector,kMap") throw new Error(`Mode 不符: ${modes}`);

const maxInlinedCount = parseInt(mustConst(/static constexpr int kMaxInlinedCount = (\d+);/, "kMaxInlinedCount"), 10);
const maxNestingDepth = parseInt(mustConst(/static constexpr uint32_t kMaxInliningNestingDepth = (\d+);/, "kMaxInliningNestingDepth"), 10);
const turboshaftAdjustment = parseInt(mustConst(/constexpr int kTurboshaftAdjustment = (\d+);/, "kTurboshaftAdjustment"), 10);
const lowestUsefulValue = parseInt(mustConst(/constexpr int kLowestUsefulValue = (\d+);/, "kLowestUsefulValue"), 10);
const correctionFactor = parseFloat(mustConst(/constexpr double kTurboshaftCorrectionFactor = ([0-9.]+);/, "kTurboshaftCorrectionFactor"));
const oneLessCall = parseInt(mustConst(/constexpr int kOneLessCall = (\d+);/, "kOneLessCall"), 10);

if (maxInlinedCount !== 60) throw new Error(`kMaxInlinedCount 不符: ${maxInlinedCount}`);
if (maxNestingDepth !== 7) throw new Error(`kMaxInliningNestingDepth 不符: ${maxNestingDepth}`);
if (correctionFactor !== 1.2) throw new Error(`kTurboshaftCorrectionFactor 不符: ${correctionFactor}`);
if (oneLessCall !== 6) throw new Error(`kOneLessCall 不符: ${oneLessCall}`);

console.log(`inlining-tree.h: mode=${modes.length} maxInlined=${maxInlinedCount} depth=${maxNestingDepth}`);

let rust = banner(
  "src/wasm/inlining-tree.h.js",
  "src/wasm/inlining-tree.h",
  "Wasm 内联决策树常量",
);

rust += `
/// 内联树的存储形态 (对应原版 InliningTree::Mode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InliningMode {
  /// kUninitialized
  Uninitialized = 0,
  /// kVector
  Vector = 1,
  /// kMap
  Map = 2,
}

/// 内联函数数量上限 (原版 kMaxInlinedCount)
pub const MAX_INLINED_COUNT: i32 = ${maxInlinedCount};

/// 内联嵌套深度上限 (原版 kMaxInliningNestingDepth)
pub const MAX_INLINING_NESTING_DEPTH: u32 = ${maxNestingDepth};

/// budget 调整: Turboshaft 每调用点扣减 (原版 kTurboshaftAdjustment)
pub const TURBOSHAFT_ADJUSTMENT: i32 = ${turboshaftAdjustment};

/// budget 低于此值不再内联 (原版 kLowestUsefulValue)
pub const LOWEST_USEFUL_VALUE: i32 = ${lowestUsefulValue};

/// budget 折算系数 (原版 kTurboshaftCorrectionFactor)
pub const TURBOSHAFT_CORRECTION_FACTOR: f64 = ${correctionFactor};

/// 一次调用的预估节省 wire-bytes (原版 kOneLessCall, 估算值)
pub const ONE_LESS_CALL: i32 = ${oneLessCall};

// —— 提取不变式断言 ——
const _: () = {
  assert!(MAX_INLINED_COUNT == 60);
  assert!(MAX_INLINING_NESTING_DEPTH == 7);
  assert!(TURBOSHAFT_CORRECTION_FACTOR > 1.0);
  assert!(LOWEST_USEFUL_VALUE < TURBOSHAFT_ADJUSTMENT);
};
`;

const p = writeSrc("v8-wasm-core", "inlining_tree_consts.rs", rust);
console.log(`written ${p}`);
