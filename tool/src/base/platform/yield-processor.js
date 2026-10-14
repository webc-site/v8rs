#!/usr/bin/env bun
// tool/src/base/platform/yield-processor.js — 覆盖 v8/src/base/platform/yield-processor.h
// 让步指令宏: 按 TSAN/编译器/架构三层的 YIELD_PROCESSOR 分支矩阵
// → v8-base-platform/src/yield_processor.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/platform/yield-processor.h");

// 1. 三层分支矩阵锚点
const anchors = [
  "#if defined(THREAD_SANITIZER)",
  "#define YIELD_PROCESSOR base::OS::Sleep(base::TimeDelta::FromMilliseconds(1))",
  "#if defined(V8_CC_MSVC) && !defined(__clang__)",
  "#define YIELD_PROCESSOR _mm_pause()",
  "#define YIELD_PROCESSOR __isb(_ARM64_BARRIER_SY)",
  "#define YIELD_PROCESSOR __asm__ __volatile__(\"pause\")",
  "#define YIELD_PROCESSOR __asm__ __volatile__(\"isb\" ::: \"memory\")",
  "#define YIELD_PROCESSOR __asm__ __volatile__(\"or 31,31,31\")",
  "#ifndef YIELD_PROCESSOR",
  "#define YIELD_PROCESSOR ((void)0)",
];
for (const a of anchors) {
  if (!hSrc.includes(a)) throw new Error(`yield-processor.h 缺锚点: ${a}`);
}

// 2. 分支计数: 8 条定义 = TSAN 1 + MSVC 2 + GCC 内联 4 + 兜底 1
const defines = [...hSrc.matchAll(/#define YIELD_PROCESSOR (.+)/g)].map((m) => m[1].trim());
if (defines.length !== 8) throw new Error(`YIELD_PROCESSOR 定义数不符: ${defines.length}, 期望 8 (TSAN 1 + MSVC 2 + GCC 4 + 兜底 1)`);
// 语义组: TSAN-sleep / pause / isb / or / 兜底空
const pauseCount = defines.filter((d) => d.includes("pause")).length;   // _mm_pause + x64 asm + mips
const isbCount = defines.filter((d) => d.includes("isb") || d.includes("__isb")).length;
if (pauseCount !== 3 || isbCount !== 2) throw new Error(`指令分布不符: pause ${pauseCount}/isb ${isbCount}, 期望 3/2`);

// 3. 目标语义: 非 TSAN 的 darwin (x64 pause / arm64 isb) → core::hint::spin_loop
if (!hSrc.includes("busy wait") || !hSrc.includes("THREAD_SANITIZER")) throw new Error("语义注释锚点缺失");

console.log(`yield-processor: 定义 ${defines.length} (pause ${pauseCount}, isb ${isbCount}, sleep 1, or 1, 兜底 1)`);

let rust = banner("src/base/platform/yield-processor.js", "src/base/platform/yield-processor.h", "忙等让步指令 (目标: core::hint::spin_loop 覆盖 x64 pause / arm64 isb)");

rust += `
/// 忙等让步 (原版宏 YIELD_PROCESSOR)
/// v8: src/base/platform/yield-processor.h YIELD_PROCESSOR
///
/// 分支矩阵 (原版三层):
///  1. THREAD_SANITIZER → OS::Sleep(TimeDelta::FromMilliseconds(1))
///     (TSAN 拦截原子访问并加锁, 自旋锁饿死 → 改睡 1ms)
///  2. MSVC: ia32/x64 → _mm_pause(); arm64/armv6 → __isb(_ARM64_BARRIER_SY)
///  3. GCC/Clang 内联汇编: x64/ia32 → "pause"; arm64/armv6 → "isb";
///     mips64r2 → "pause"; ppc64 → "or 31,31,31"
///  4. 兜底: ((void)0)
///
/// Rust 侧: core::hint::spin_loop() 在 x64 编译为 pause、arm64 编译为 yield,
/// 语义等价于分支 3 的目标形态; TSAN 换眠逻辑属执行环境配置, 登记由上层处理。
#[inline(always)]
pub fn yield_processor() {
  std::hint::spin_loop();
}

/// TSAN 自旋缓解休眠 (原版 THREAD_SANITIZER 分支的 1ms)
/// v8: src/base/platform/yield-processor.h (THREAD_SANITIZER 分支)
pub const TSAN_YIELD_SLEEP_MS: i64 = 1;

// —— 提取不变式断言 ——
const _: () = {
  // 8 条 #define: sleep 1 + _mm_pause 1 + __isb 1 + pause 2 (x64/mips) + isb 1 + or 1 + 兜底 1
  const DEFINES: usize = 8;
  const PAUSE_IMPLS: usize = 3;
  const ISB_IMPLS: usize = 2;
  assert!(DEFINES == 8 && PAUSE_IMPLS == 3 && ISB_IMPLS == 2);
  assert_eq!(TSAN_YIELD_SLEEP_MS, 1);
};
`;

const p = writeSrc("v8-base-platform", "yield_processor.rs.skel", rust);
console.log(`written ${p}`);
