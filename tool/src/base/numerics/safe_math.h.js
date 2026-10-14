#!/usr/bin/env bun
// tool/src/base/numerics/safe_math.h.js — 覆盖 v8/src/base/numerics/safe_math.h + safe_math_shared_impl.h + safe_math_clang_gcc_impl.h + safe_math_arm_impl.h
// 解析 umbrella include、FastOp 存根/特化矩阵、BASE_HAS_OPTIMIZED_SAFE_MATH 开关、__builtin_*_overflow 分派
// → v8-base/src/numerics/safe_math.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/base/numerics/safe_math.h");
const shared = readV8("src/base/numerics/safe_math_shared_impl.h");
const clang = readV8("src/base/numerics/safe_math_clang_gcc_impl.h");
const arm = readV8("src/base/numerics/safe_math_arm_impl.h");

for (const anchor of [
  [h, "V8_BASE_NUMERICS_SAFE_MATH_H_"],
  [h, '#include "src/base/numerics/checked_math.h"'],
  [shared, "V8_BASE_NUMERICS_SAFE_MATH_SHARED_IMPL_H_"],
  [shared, "BASE_HAS_OPTIMIZED_SAFE_MATH"],
  [clang, "V8_BASE_NUMERICS_SAFE_MATH_CLANG_GCC_IMPL_H_"],
  [clang, "__builtin_add_overflow"],
  [arm, "V8_BASE_NUMERICS_SAFE_MATH_ARM_IMPL_H_"],
]) {
  if (!anchor[0].includes(anchor[1])) throw new Error(`anchor missing: ${anchor[1]}`);
}

// 真解析: umbrella re-export 清单
const umbrellas = [...h.matchAll(/#include "src\/base\/numerics\/(\w+)\.h"/g)].map((m) => m[1]);
// 真解析: FastOp 存根 (shared) 与特化 (clang/gcc) 结构
const fastStubs = [...shared.matchAll(/^struct (\w+FastOp) \{/gm)].map((m) => m[1]);
const fastSpecs = [...clang.matchAll(/^struct (\w+FastOp) \{/gm)].map((m) => m[1]);
const asmSpecs = [...clang.matchAll(/^struct (\w+FastAsmOp) \{/gm)].map((m) => m[1]);
const armAsm = [...arm.matchAll(/^struct (\w+FastAsmOp) \{/gm)].map((m) => m[1]);
// 真解析: clang/gcc 侧用的 __builtin 溢出原语
const builtins = [...new Set([...clang.matchAll(/__(\w+)_overflow\(/g)].map((m) => m[1]))];

const body = `${banner("src/base/numerics/safe_math.h.js", "src/base/numerics/safe_math{,_shared_impl,_clang_gcc_impl,_arm_impl}.h", "安全算术伞头 + 编译器快速路径分派")}
//! v8 侧: umbrella re-export ${umbrellas.length} 个 (${umbrellas.join(", ")})
//! FastOp 存根 ${fastStubs.length} (shared) / 特化 ${fastSpecs.length} (clang_gcc) / Asm 特化 ${asmSpecs.length} (clang) + ${armAsm.length} (arm)
//! __builtin 溢出原语: ${builtins.map((b) => `__${b}_overflow`).join(", ")}
//! Rust 策略: checked_*/saturating_* 原生即最优代码生成, 快速路径分派层整体删除

// ---- umbrella (v8 safe_math.h = 三头合一 re-export) ----
${umbrellas.map((u) => `pub use super::${u.replace(/_/g, "_")}::*; // v8: #include "src/base/numerics/${u}.h" IWYU export`).join("\n")}

// ---- BASE_HAS_OPTIMIZED_SAFE_MATH 编译开关 ----
/// v8: safe_math_shared_impl.h BASE_HAS_OPTIMIZED_SAFE_MATH (0/1, 按编译器)
/// 意义: 启用 __builtin_*_overflow / arm 内联汇编快速路径; Rust 恒为 "启用" (原生等价)
pub const BASE_HAS_OPTIMIZED_SAFE_MATH: bool = true;

// ---- FastOp 矩阵 ----
${fastStubs.map((s) => `/// v8: safe_math_shared_impl.h struct ${s} — is_supported=false 存根
/// (clang/gcc 或 arm 侧特化为 is_supported=true + __builtin/asm Do())`).join("\n")}
${[...new Set([...asmSpecs, ...armAsm])].map((s) => `/// v8: ${armAsm.includes(s) ? "safe_math_arm_impl.h" : "safe_math_clang_gcc_impl.h"} struct ${s} — 内联汇编特化`).join("\n")}
// 语义对照:
${builtins.map((b) => `//   __${b}_overflow → ${b.startsWith("add") ? "checked_add" : b.startsWith("sub") ? "checked_sub" : b.startsWith("mul") ? "checked_mul" : "checked_*"}`).join("\n")}
`;

writeSrc("v8-base", "numerics/safe_math.rs.skel", body);
console.log(`safe_math.h.js: umbrella=${umbrellas.length} stubs=${fastStubs.length} specs=${fastSpecs.length} asm=${asmSpecs.length}+${armAsm.length} builtins=${builtins.length}`);
