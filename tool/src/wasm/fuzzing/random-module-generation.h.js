#!/usr/bin/env bun
// tool/src/wasm/fuzzing/random-module-generation.h.js
// 机械提取 v8/src/wasm/fuzzing/random-module-generation.h 的生成选项枚举

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/wasm/fuzzing/random-module-generation.h");

const optMatch = hSrc.match(/enum WasmModuleGenerationOption \{([^}]*)\};/);
if (!optMatch) throw new Error("未找到 WasmModuleGenerationOption");
const opts = optMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (opts.join(",") !== "kGenerateSIMD,kGenerateWasmGC") {
  throw new Error(`生成选项不符: ${opts}`);
}

// 预设组合存在性
for (const preset of ["MVP", "Simd", "WasmGC", "All"]) {
  if (!hSrc.includes(`WasmModuleGenerationOptions ${preset}()`)) {
    throw new Error(`未找到预设 ${preset}`);
  }
}

console.log(`random-module-generation.h: options=${opts.length}`);

let rust = banner(
  "src/wasm/fuzzing/random-module-generation.h.js",
  "src/wasm/fuzzing/random-module-generation.h",
  "随机模块生成选项 (fuzzing 面)",
);

rust += `
/// 随机模块生成选项位 (对应原版 enum WasmModuleGenerationOption)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum WasmModuleGenerationOption {
  /// kGenerateSIMD
  GenerateSimd = 0,
  /// kGenerateWasmGC
  GenerateWasmGc = 1,
}

/// 选项集合 (原版 WasmModuleGenerationOptions, base::EnumSet)
pub type WasmModuleGenerationOptions = u8;

/// 原版 MVP() — 空集合
pub const GEN_MVP: WasmModuleGenerationOptions = 0;
/// 原版 Simd() — 仅 kGenerateSIMD
pub const GEN_SIMD: WasmModuleGenerationOptions = 1 << WasmModuleGenerationOption::GenerateSimd as u8;
/// 原版 WasmGC() — 仅 kGenerateWasmGC
pub const GEN_WASM_GC: WasmModuleGenerationOptions = 1 << WasmModuleGenerationOption::GenerateWasmGc as u8;
/// 原版 All() — 全部
pub const GEN_ALL: WasmModuleGenerationOptions = GEN_SIMD | GEN_WASM_GC;

// —— 提取不变式断言 ——
const _: () = {
  assert!(GEN_MVP == 0);
  assert!(GEN_ALL == 0b11);
};
`;

const p = writeSrc("v8-wasm-core", "random_module_generation_consts.rs", rust);
console.log(`written ${p}`);
