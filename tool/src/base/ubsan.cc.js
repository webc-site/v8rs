#!/usr/bin/env bun
// tool/src/base/ubsan.cc.js — 覆盖 v8/src/base/ubsan.cc + v8/src/base/sanitizer/ubsan.h
// 解析 __mulodi4 32 位分块乘法 (compiler_rt 替身) 与 DISABLE_UBSAN 宏开关
// → v8-base/src/ubsan.rs.skel

import { readV8, writeSrc, banner, lines } from "../../lib.js";

const cc = readV8("src/base/ubsan.cc");
const h = readV8("src/base/sanitizer/ubsan.h");

for (const anchor of [
  [cc, '__mulodi4'],
  [cc, "This file is only needed for 32-bit UBSan builds."],
  [cc, "uint64_t a_low = a & 0xFFFFFFFFu;"],
  [h, "V8_BASE_SANITIZER_UBSAN_H_"],
  [h, "#define DISABLE_UBSAN"],
]) {
  if (!anchor[0].includes(anchor[1])) throw new Error(`anchor missing: ${anchor[1]}`);
}

// 真解析: 32 位分块中间量
const chunks = [...cc.matchAll(/^  uint64_t (\w+) = a_low \* b_low;|^  uint64_t (r_\w+) = /gm)].map((m) => m[1] || m[2]).filter(Boolean);
const mulodi4 = lines(cc, 17, 62);

const body = `${banner("src/base/ubsan.cc.js", "src/base/ubsan.cc + src/base/sanitizer/ubsan.h", "32 位 UBSan 构建的 compiler_rt 补位乘法")}
//! v8 侧: extern "C" 函数 1 (__mulodi4); 宏开关 1 (DISABLE_UBSAN)
//! 前提: 仅 32 位 UBSan 构建需要 (Rust: i64::checked_mul 原生, 本件为对账空转)

/// v8: src/base/sanitizer/ubsan.h DISABLE_UBSAN
/// UNDEFINED_SANITIZER 时 no_sanitize("undefined"), 否则空
/// Rust: 无 per-fn UBSan 属性需求 (debug_assertions/overflow-checks 由 cargo profile 控制)
pub const DISABLE_UBSAN: bool = false;

/// v8: src/base/ubsan.cc __mulodi4(a, b, *overflow)
/// 64 位乘法按 32 位分块: [AH AL]*[BH BL] = r_low/r_mid1/r_mid2/r_high 四积 + 列间进位
/// 溢出判定: r_high > 0 或 结果符号 != 操作数符号异或
/// Rust 等价: a.checked_mul(b) → (result, overflow); 保留原文证据供差分
#[no_mangle]
pub extern "C" fn __mulodi4(a: i64, b: i64, overflow: &mut i32) -> i64 {
    match a.checked_mul(b) {
        Some(r) => { *overflow = 0; r }
        None => { *overflow = 1; a.wrapping_mul(b) }
    }
}

// ---- v8 原文 (__mulodi4, Developer 对照) ----
${mulodi4.map((l) => `// ${l}`).join("\n")}
`;

writeSrc("v8-base", "ubsan.rs.skel", body);
console.log(`ubsan.cc.js: externC=1 macros=1 chunks=${chunks.length}`);
