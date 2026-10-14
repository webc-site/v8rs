#!/usr/bin/env bun
// tool/src/base/numbers/dtoa.h.js — 族: src/base/numbers/dtoa.h + .cc
// 覆盖: DtoaMode 枚举 (3) + kBase10MaximalLength + DoubleToAscii 分派
//        (fast-dtoa → fixed-dtoa → bignum-dtoa 回退)
// → v8-base/src/numbers/dtoa.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/numbers/dtoa.h");
const ccSrc = readV8("src/base/numbers/dtoa.cc");

// —— 枚举真解析 ——
const enumBody = hSrc.match(/enum DtoaMode \{([\s\S]*?)\};/)[1];
const modes = [...enumBody.matchAll(/^\s{2}(DTOA_\w+),?$/gm)].map((m) => m[1]);
if (modes.join(",") !== "DTOA_SHORTEST,DTOA_FIXED,DTOA_PRECISION") throw new Error(`模式枚举 ${modes}`);

// —— 锚点断言 ——
if (!hSrc.includes("const int kBase10MaximalLength = 17;")) throw new Error("缺最大长度常量");
if (!hSrc.includes("V8_BASE_EXPORT void DoubleToAscii(double v, DtoaMode mode, int requested_digits,")) throw new Error("缺入口声明");
if (!ccSrc.includes("static BignumDtoaMode DtoaToBignumDtoaMode(DtoaMode dtoa_mode)")) throw new Error("缺模式映射");
if (!ccSrc.includes("fast_worked = FastDtoa(v, FAST_DTOA_SHORTEST, 0, buffer, length, point);")) throw new Error("缺 SHORTEST 快路");
if (!ccSrc.includes("fast_worked = FastFixedDtoa(v, requested_digits, buffer, length, point);")) throw new Error("缺 FIXED 快路");
if (!ccSrc.includes("fast_worked = FastDtoa(v, FAST_DTOA_PRECISION, requested_digits, buffer,")) throw new Error("缺 PRECISION 快路");
if (!ccSrc.includes("// If the fast dtoa didn't succeed use the slower bignum version.")) throw new Error("缺回退注释");
if (!ccSrc.includes("DCHECK(!Double(v).IsSpecial());")) throw new Error("缺输入约束");

// —— 真解析: 模式映射表 ——
const mapping = [...ccSrc.matchAll(/case (DTOA_\w+):\s*\n\s*return (BIGNUM_DTOA_\w+);/g)].map((m) => `${m[1]}=${m[2]}`);
if (mapping.length !== 3) throw new Error(`模式映射 ${mapping.length} != 3`);
console.log(`dtoa: 模式 3, 快路 3 (FastDtoa×2 + FastFixedDtoa), bignum 回退 1, 最大长度 17`);

let rust = banner(
  "src/base/numbers/dtoa.h.js",
  "src/base/numbers/dtoa.h + src/base/numbers/dtoa.cc",
  "double→ASCII 总入口 (快路优先, bignum 兜底)",
);

rust += `
//! v8: src/base/numbers/dtoa.h/.cc

/// v8: dtoa.h DtoaMode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DtoaMode {
    /// 最短正确表示 (等距半数远离 0 进位)
    Shortest,
    /// 定点小数位 (半数向 ±Inf)
    Fixed,
    /// 定精度有效位 (半数远离 0)
    Precision,
}

/// v8: dtoa.h kBase10MaximalLength = 17 — 不含符号/小数点/指数;
/// buffer 至少 17+1 (NUL)
pub const BASE10_MAXIMAL_LENGTH: usize = 17;

/// v8: dtoa.cc DoubleToAscii(v, mode, requested_digits) → (sign, digits, point)
/// 值语义: v = sign × digits × 10^(point - digits.len());
/// 前置: v 非 NaN/Inf (DCHECK); 非 SHORTEST 时 requested_digits >= 0
/// 流程: 剥符号 → v==0 快答 ("0", point=1) → PRECISION&&0 位空串 →
///       快路分派 (SHORTEST/PRECISION→FastDtoa, FIXED→FastFixedDtoa) →
///       失败回退 BignumDtoa (模式 1:1 映射) 并补 NUL
pub fn double_to_ascii(v: f64, mode: DtoaMode, requested_digits: i32) -> (i32, Vec<u8>, i32);

// v8: dtoa.cc 模式映射 (1:1):
${mapping.map((m) => `//   ${m}`).join("\n")}
`;

writeSrc("v8-base", "numbers/dtoa.rs.skel", rust);
console.log("written v8-base/src/numbers/dtoa.rs.skel");
