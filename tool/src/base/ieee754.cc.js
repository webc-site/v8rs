#!/usr/bin/env bun
// tool/src/base/ieee754.cc.js — 族: src/base/ieee754.cc (ieee754.h 归别席/既有)
// 覆盖: 24 个 ieee754 数学函数 (大部分委托 llvm-libc shared/math, 4 个本地实现)
// → v8-base/src/ieee754_cc.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/ieee754.cc");

// —— 锚点断言 ——
if (!src.includes("#include \"third_party/llvm-libc/src/shared/math.h\"")) throw new Error("缺 llvm-libc include");
if (!src.includes("namespace ieee754 {")) throw new Error("缺 ieee754 命名空间");
if (!src.includes("namespace legacy {")) throw new Error("缺 legacy 命名空间");

// —— 真解析: 全部函数定义, 对数 + 分类 ——
const defs = [...src.matchAll(/^double (\w+)\(/gm)].map((m) => m[1]);
const delegating = [...src.matchAll(/^double (\w+)\(.*\{ return LIBC_NAMESPACE::shared::(\w+)\(/gm)].map((m) => `${m[1]}→${m[2]}`);
const expected = [
  "acos", "acosh", "asin", "asinh", "atan", "atan2", "atanh", "cbrt", "cosh", "cos", "exp",
  "expm1", "log", "log1p", "log2", "log10", "pow", "sinh", "sin", "tan", "tanh",
];
for (const e of expected) if (!defs.includes(e)) throw new Error(`缺函数 ${e}`);
if (defs.length !== expected.length) throw new Error(`函数总数 ${defs.length} != ${expected.length}`);
// 本地实现 (非 libc 委托): acosh/asinh/atanh/cosh/sinh + legacy pow 委托
const local = defs.filter((f) => !delegating.some((d) => d.startsWith(`${f}→`)));
const expectedLocal = ["acosh", "asinh", "atanh", "cosh", "sinh"];
for (const l of expectedLocal) if (!local.includes(l)) throw new Error(`应属本地实现的 ${l} 被判为委托`);
console.log(`ieee754.cc: 函数 ${defs.length} (libc 委托 ${delegating.length}, 本地实现 ${local.length}), 溢出阈值常量 2`);

// —— 溢出阈值真解析 ——
const overflowConsts = [...src.matchAll(/static const double (K\w+_OVERFLOW) = ([\d.]+)[,;]/g)].map((m) => [m[1], m[2]]);
if (overflowConsts.length !== 2) throw new Error(`溢出常量 ${overflowConsts.length} != 2`);

let rust = banner(
  "src/base/ieee754.cc.js",
  "src/base/ieee754.cc",
  "ieee754 数学函数面 (llvm-libc 委托 + 5 个本地实现)",
);

rust += `
//! v8: src/base/ieee754.cc — 对应 ieee754.h 声明 (归 base-core 顶层)
//! 委托关系: 大多数直接转 LIBC_NAMESPACE::shared (llvm-libc);
//! Rust 侧对应策略: std/primal 取代, 语义差异处 Developer 补差分测试

// —— libc 委托面 (Rust 对应 std::f64 同名, 特例注明) ——
${delegating.map(([f, l]) => `/// v8: ieee754.cc ${f} → llvm-libc ${l}\npub fn ${f}(x: f64) -> f64;`).join("\n")}

// —— 本地实现面 (fdlibm 移植, 含 static 常量表) ——
/// v8: ieee754.cc acosh/asinh/atanh — fdlibm 风格分段实现, 内嵌 static const 常量表
pub fn acosh(x: f64) -> f64;
pub fn asinh(x: f64) -> f64;
pub fn atanh(x: f64) -> f64;
/// v8: ieee754.cc cosh — |x| > KCOSH_OVERFLOW 走溢出路径
pub fn cosh(x: f64) -> f64;
/// v8: ieee754.cc sinh — |x| > KSINH_OVERFLOW 走溢出路径
pub fn sinh(x: f64) -> f64;

// v8: 溢出阈值 (static const double)
pub const KCOSH_OVERFLOW: f64 = ${overflowConsts.find(([n]) => n === "KCOSH_OVERFLOW")?.[1]};
pub const KSINH_OVERFLOW: f64 = ${overflowConsts.find(([n]) => n === "KSINH_OVERFLOW")?.[1]};

// v8: namespace legacy { pow } — 双参 pow 也走 llvm-libc shared::pow
/// v8: ieee754.cc legacy::pow(x, y)
pub fn pow(x: f64, y: f64) -> f64;
`;

// 产物名 ieee754_cc: ieee754.h (别席) 产出 ieee754.rs.skel, 本 .cc 族错开防互覆
writeSrc("v8-base", "ieee754_cc.rs.skel", rust);
console.log("written v8-base/src/ieee754_cc.rs.skel");
