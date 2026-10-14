#!/usr/bin/env bun
// tool/src/base/qnx-math.h.js — 覆盖 v8/src/base/qnx-math.h
// 解析 QNX 平台宏污染清洗表 (undef + using std::lrint)
// → v8-base/src/qnx_math.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/base/qnx-math.h");

for (const anchor of ["V8_BASE_QNX_MATH_H_", "#undef fpclassify", "using std::lrint;"]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: #undef 清单与 using 清单
const undefs = [...h.matchAll(/^#undef (\w+)$/gm)].map((m) => m[1]);
const usings = [...h.matchAll(/^using std::(\w+);$/gm)].map((m) => m[1]);

const body = `${banner("src/base/qnx-math.h.js", "src/base/qnx-math.h", "QNX 数学宏污染清洗 (Rust 无此问题, 登记语义对账)")}
//! v8 侧: #undef ${undefs.length} 项; using std:: ${usings.length} 项
//! QNX 的 math.h 把这些函数做成宏污染全局; Rust 侧方法/自由函数无宏, 本文件为空壳对账

/// v8: src/base/qnx-math.h #undef 清单
pub const UNDEF_MACROS: &[&str] = &[${undefs.map((u) => `"${u}"`).join(", ")}];

/// v8: src/base/qnx-math.h using std::lrint — 就近整数 (ties 偶) 
/// Rust: f64::round_ties_even (Rust 1.77+) 或手动; 语义面就地修订
pub fn lrint(x: f64) -> i64 { /* 语义: round half to even → i64 */ todo!() }
`;

writeSrc("v8-base", "qnx_math.rs.skel", body);
console.log(`qnx-math.h.js: undefs=${undefs.length} usings=${usings.length}`);
