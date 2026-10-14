#!/usr/bin/env bun
// tool/src/base/immediate-crash.h.js — 族: src/base/immediate-crash.h
// 覆盖: TRAP_SEQUENCE1_/2_ 各架构展开 + TRAP_SEQUENCE_ + WRAPPED_ + IMMEDIATE_CRASH
// → v8-base/src/immediate_crash.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/immediate-crash.h");

// —— 锚点断言 ——
const seq1 = [...src.matchAll(/^#define TRAP_SEQUENCE1_\(\) (.+)$/gm)].map((m) => m[1]);
const seq2 = [...src.matchAll(/^#define TRAP_SEQUENCE2_\(\) (.+)$/gm)].map((m) => m[1]);
if (seq1.length < 8) throw new Error(`TRAP_SEQUENCE1_ 分支 ${seq1.length} < 8`);
if (seq2.length < 8) throw new Error(`TRAP_SEQUENCE2_ 分支 ${seq2.length} < 8`);
for (const a of [
  'asm volatile("int3")', 'asm volatile("ud2")',
  'asm volatile("bkpt #0")', 'asm volatile("udf #0")',
  'asm volatile("brk #0")', 'asm volatile("hlt #0")',
  "__debugbreak()", "__builtin_trap()",
]) if (!src.includes(a)) throw new Error(`缺陷阱指令 ${a}`);
if (!src.includes("#define TRAP_SEQUENCE_()")) throw new Error("缺组合宏");
if (!src.includes("[] { TRAP_SEQUENCE_(); }();")) throw new Error("缺 GNU lambda 包装 (constexpr 兼容)");
if (!src.includes("__builtin_unreachable();")) throw new Error("缺 noreturn 提示");
if (!src.includes("#error No supported trap sequence!")) throw new Error("缺兜底 #error");
console.log(`immediate-crash: TRAP_SEQUENCE1_ 分支 ${seq1.length}, TRAP_SEQUENCE2_ 分支 ${seq2.length}, 宏层 4 级`);

let rust = banner(
  "src/base/immediate-crash.h.js",
  "src/base/immediate-crash.h",
  "最快崩溃路径 (无日志, 信号须异于访存错, 禁折叠)",
);

rust += `
//! v8: src/base/immediate-crash.h
//! Rust 对应: std::process::abort / core::arch 陷阱指令; 语义约束登记如下

/// v8: src/base/immediate-crash.h IMMEDIATE_CRASH
/// 约束: 必致死 (不可被调试器 continue 跳过); 各实例不折叠 (asm volatile 保唯一);
///       信号区别于非法访存; noreturn (unreachable 提示)
pub fn immediate_crash() -> !;

// v8: TRAP_SEQUENCE_ = TRAP_SEQUENCE1_ + TRAP_SEQUENCE2_, 各架构指令表:
// | 架构 | SEQ1 | SEQ2 |
// | --- | --- | --- |
// | x64/ia32 (GNU) | int3 | ud2 (darwin 为空, unreachable 已出 ud2) |
// | arm | bkpt #0 | udf #0 |
// | arm64 | brk #0 | hlt #0 |
// | ppc64 | .4byte 0x7D821008 (AIX: .vbyte 4,...) | 空 |
// | s390x | .2byte 0x0001 | 空 |
// | zOS / 兜底 | __builtin_trap() | 空 |
// | MSVC 非 clang | __debugbreak() | 空 |
// v8: WRAPPED_TRAP_SEQUENCE_ — GNU 下包进立即 lambda 绕 constexpr 禁 asm 限制
//     (代价: 栈顶指向匿名 lambda, 而非 CHECK 调用点)
// Rust: Developer 选 std::intrinsics::abort / platform-specific trap!, 登记跳过
`;

writeSrc("v8-base", "immediate_crash.rs.skel", rust);
console.log("written v8-base/src/immediate_crash.rs.skel");
