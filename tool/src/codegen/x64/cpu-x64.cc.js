#!/usr/bin/env bun
// tool/src/codegen/x64/cpu-x64.cc.js
// 机械提取 v8/src/codegen/x64/cpu-x64.cc: x64 平台 CpuFeatures 实现 (FlushICache 空操作)
// → v8-codegen-x64/src/cpu.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/codegen/x64/cpu-x64.cc");

// 对数: 原版仅 FlushICache 一个实现 (Intel 无需刷新指令缓存)
const methods = [...src.matchAll(/void CpuFeatures::(\w+)\(/g)].map((x) => x[1]);
if (!methods.includes("FlushICache")) throw new Error("缺 CpuFeatures::FlushICache");
if (!src.includes("No need to flush the instruction cache on Intel"))
  throw new Error("缺 Intel 空操作注释锚点");

let rust = banner(
  "src/codegen/x64/cpu-x64.cc.js",
  "src/codegen/x64/cpu-x64.cc",
  "x64 平台 CpuFeatures 实现 (FlushICache 空操作)",
);

rust += `
/// x64 平台 FlushICache (原版 CpuFeatures::FlushICache):
/// Intel 无需显式刷新 — 单线程代码补丁场景下核心指令缓存自动一致;
/// 若确需刷新由 Windows FlushInstructionCache API 承担。
/// VALGRIND_DISCARD_TRANSLATIONS 分支: 非 valgrind 构建, 登记跳过。
#[inline]
pub fn flush_i_cache(_start: *mut u8, _size: usize) {
  // x64: 空操作 (原版语义)
}
`;

const p = writeSrc("v8-codegen-x64", "cpu.rs", rust);
console.log(`cpu-x64.cc: 方法 ${methods.length} (FlushICache 空操作) → ${p}`);
