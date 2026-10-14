#!/usr/bin/env bun
// tool/src/base/numbers/fixed-dtoa.h.js — 覆盖 v8/src/base/numbers/fixed-dtoa.{h,cc}
// 解析 FastFixedDtoa API、7 个 static 辅助函数签名、十进制常量表与指数分派边界
// → v8-base/src/numbers/fixed_dtoa.rs.skel

import { readV8, writeSrc, banner, lines } from "../../../lib.js";

const h = readV8("src/base/numbers/fixed-dtoa.h");
const cc = readV8("src/base/numbers/fixed-dtoa.cc");

for (const anchor of [
  [h, "V8_BASE_NUMBERS_FIXED_DTOA_H_"],
  [h, "bool FastFixedDtoa(double v, int fractional_count,"],
  [cc, "static const int kDoubleSignificandSize = 53"],
  [cc, "static void FillDigits32FixedLength"],
  [cc, "static void DtoaRoundUp"],
  [cc, "static void FillFractionals"],
  [cc, "static void TrimZeros"],
  [cc, "bool FastFixedDtoa(double v, int fractional_count, Vector<char> buffer,"],
]) {
  if (!anchor[0].includes(anchor[1])) throw new Error(`anchor missing: ${anchor[1]}`);
}

// 真解析: static 辅助函数 + 导出函数
const statik = [...cc.matchAll(/^static (?:void|bool|int|uint64_t|char) (\w+)\(/gm)].map((m) => m[1]);
// 真解析: 常量
const intConsts = [...cc.matchAll(/^static const (int|uint32_t|uint64_t) (\w+) = ([^;]+);/gm)].map((m) => ({ type: m[1], name: m[2], value: m[3].trim() }));
const localConsts = [...cc.matchAll(/^  const (uint32_t|uint64_t) (\w+) = ([^;]+);/gm)].map((m) => ({ type: m[1], name: m[2], value: m[3].trim() }));
// 真解析: 指数分派边界 (主函数 5 分支)
const branches = (cc.match(/if \(exponent > 20\) return false;|if \(fractional_count > 20\) return false;|exponent \+ kDoubleSignificandSize > 64|else if \(exponent >= 0\)|exponent > -kDoubleSignificandSize|else if \(exponent < -128\)/g) || []);
// 主函数原文行区间 (分派骨架证据)
const main = lines(cc, 227, 316);

const body = `${banner("src/base/numbers/fixed-dtoa.h.js", "src/base/numbers/fixed-dtoa.h + .cc", "定点小数打印: FastFixedDtoa 规则面")}
//! v8 侧: 导出函数 1 (FastFixedDtoa) + static 辅助 ${statik.length} 个; 分支 ${branches.length} 个
//! Rust 策略: buffer → &mut [u8], length/decimal_point → 返回元组

/// v8: src/base/numbers/fixed-dtoa.h kDoubleSignificandSize (cc L45, 含隐藏位)
pub const K_DOUBLE_SIGNIFICAND_SIZE: i32 = 53;
${intConsts.filter((c) => c.name !== "kDoubleSignificandSize").map((c) => `/// v8: src/base/numbers/fixed-dtoa.cc ${c.name}
pub const ${c.name.replace(/^k/, "K_").replace(/([a-z])([A-Z])/g, "$1_$2").toUpperCase()}: ${c.type === "int" ? "i32" : c.type === "uint32_t" ? "u32" : "u64"} = ${c.value.replace(/'/g, "")};`).join("\n")}
${localConsts.map((c) => `/// v8: src/base/numbers/fixed-dtoa.cc ${c.name} = ${c.value.replace(/'/g, "")} (函数内局部)
pub const ${c.name.replace(/^k/, "K_").replace(/([a-z])([A-Z])/g, "$1_$2").toUpperCase()}: ${c.type === "uint32_t" ? "u32" : "u64"} = ${c.value.replace(/'/g, "")};`).join("\n")}

// ---- static 辅助 (v8 侧 ${statik.length} 个) ----
${statik.map((s) => `/// v8: src/base/numbers/fixed-dtoa.cc ${s}()
/// 语义面就地修订 (数字填充/进位/小数尾部/去零)`).join("\n")}
// 对账: ${statik.join(", ")}

/// v8: src/base/numbers/fixed-dtoa.h FastFixedDtoa(v, fractional_count, buffer, length*, decimal_point*)
/// 语义: v = significand * 2^exponent; 5 分支分派 (原文见尾):
///   exponent > 20 / fractional_count > 20 → false (不处理)
///   exponent + 53 > 64 → 除以 5^17 分解 (kFive17 路径)
///   0 <= exponent <= 11 → 整数左移直接填
///   -53 < exponent < 0 → 整数/小数拆分 (integrals/fractionals)
///   exponent < -128 → 全零
/// 四舍五入向远离 0 (away from zero); buffer 需容 length+1
#[allow(clippy::too_many_arguments)]
pub fn fast_fixed_dtoa(v: f64, fractional_count: i32, buffer: &mut [u8]) -> Option<(usize, i32)> {
    // 返回 (length, decimal_point); None = 参数不支持
    /// v8: 分派分支计数 = ${branches.length}
    todo!()
}

// ---- v8 主函数原文 (分支证据, Developer 对照) ----
${main.map((l) => `// ${l}`).join("\n")}
`;

writeSrc("v8-base", "numbers/fixed_dtoa.rs.skel", body);
console.log(`fixed-dtoa.h.js: exports=1 statics=${statik.length} intConsts=${intConsts.length} localConsts=${localConsts.length} branches=${branches.length}`);
