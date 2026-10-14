#!/usr/bin/env bun
// tool/src/wasm/wasm-code-coverage.h.js
// 机械提取 v8/src/wasm/wasm-code-coverage.h 的代码区间结构

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-code-coverage.h");

const rangeMatch = hSrc.match(/struct WasmCodeRange \{([\s\S]*?)\n\};/);
if (!rangeMatch) throw new Error("未找到 WasmCodeRange");
const noPosMatch = hSrc.match(/static constexpr int kNoCodePosition = (-?\d+);/);
if (!noPosMatch) throw new Error("未找到 kNoCodePosition");
const noPos = parseInt(noPosMatch[1], 10);
if (noPos !== -1) throw new Error(`kNoCodePosition 不符: ${noPos}`);

const covDataMatch = hSrc.match(/class WasmFunctionCoverageData \{/);
if (!covDataMatch) throw new Error("未找到 WasmFunctionCoverageData");

let rust = banner(
  "src/wasm/wasm-code-coverage.h.js",
  "src/wasm/wasm-code-coverage.h",
  "Wasm 代码覆盖区间结构",
);

rust += `
/// 代码字节偏移区间 [start, end], 双端含 (对应原版 WasmCodeRange)
/// 偏移基于函数 wire bytes 起点而非模块起点
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WasmCodeRange {
  /// 区间起点 (无代码时为 NO_CODE_POSITION)
  pub start: i32,
  /// 区间终点 (无代码时为 NO_CODE_POSITION)
  pub end: i32,
}

/// 无代码位置哨兵 (原版 kNoCodePosition)
pub const NO_CODE_POSITION: i32 = ${noPos};

impl WasmCodeRange {
  /// 原版 constexpr 构造 WasmCodeRange(start, end)
  #[inline]
  #[must_use]
  pub const fn new(start: i32, end: i32) -> Self {
    Self { start, end }
  }
}

/// 函数级覆盖数据 (原版 WasmFunctionCoverageData, 语义面待 Developer 修订)
pub struct WasmFunctionCoverageData;

// —— 提取不变式断言 ——
const _: () = {
  assert!(NO_CODE_POSITION == -1);
};
`;

const p = writeSrc("v8-wasm-core", "wasm_code_coverage.rs", rust);
console.log(`written ${p}`);
