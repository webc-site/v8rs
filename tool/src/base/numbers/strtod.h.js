#!/usr/bin/env bun
// tool/src/base/numbers/strtod.h.js — 覆盖 v8/src/base/numbers/strtod.{h,cc}
// 解析 Strtod API、精确十次幂表 (10^0..10^22)、十进制常量边界与三级分派 (Double/DiyFp/Bignum)
// → v8-base/src/numbers/strtod.rs.skel

import { readV8, writeSrc, banner, lines } from "../../../lib.js";

const h = readV8("src/base/numbers/strtod.h");
const cc = readV8("src/base/numbers/strtod.cc");

for (const anchor of [
  [h, "V8_BASE_NUMBERS_STRTOD_H_"],
  [h, "double Strtod(Vector<const char> buffer, int exponent);"],
  [cc, "static const int kMaxExactDoubleIntegerDecimalDigits = 15"],
  [cc, "static const int kMaxDecimalPower = 309"],
  [cc, "static const int kMinDecimalPower = -324"],
  [cc, "static const int kMaxSignificantDecimalDigits = 780"],
  [cc, "static bool DoubleStrtod"],
  [cc, "static bool DiyFpStrtod"],
  [cc, "static double BignumStrtod"],
  [cc, "double Strtod(Vector<const char> buffer, int exponent) {"],
]) {
  if (!anchor[0].includes(anchor[1])) throw new Error(`anchor missing: ${anchor[1]}`);
}

// 真解析: 十进制常量
const consts = [...cc.matchAll(/^static const (int|uint64_t|double) (\w+)(?: = ([^;]+))?;/gm)].map((m) => ({ type: m[1], name: m[2], value: m[3] }));
// 真解析: exact_powers_of_ten 表项数
const powEntries = (cc.match(/^  10[\d.]*e?,?$/gm) || []).length;
const powEntries2 = (cc.match(/^  \d[\d.]*\.0\b/gm) || []).length;
if (powEntries2 !== 23) throw new Error(`expect 23 power entries, got ${powEntries2}`);
// 真解析: static 辅助
const statik = [...cc.matchAll(/^static (?:void|bool|int|uint64_t|Vector<const char>|DiyFp|double) (\w+)\(/gm)].map((m) => m[1]);
// 主分派原文
const main = lines(cc, 391, 417);

const body = `${banner("src/base/numbers/strtod.h.js", "src/base/numbers/strtod.h + .cc", "字符串→double: Strtod 三级分派规则面")}
//! v8 侧: 导出 1 (Strtod) + static ${statik.length} 个; 精确幂表 ${powEntries2} 项 (10^0..10^22)

// ---- 常量表 ----
${consts.filter((c) => c.name !== "kExactPowersOfTenSize").map((c) => `/// v8: src/base/numbers/strtod.cc ${c.name} = ${c.value?.trim() ?? "(array)"}
pub const ${c.name.replace(/^k/, "K_").replace(/([a-z])([A-Z])/g, "$1_$2").toUpperCase()}: ${c.type === "int" ? "i32" : c.type === "uint64_t" ? "u64" : "f64"} = ${c.type === "double" ? "todo!()" /* 表, 见下 */ : (c.value ?? "").trim()};`).join("\n")}

/// v8: src/base/numbers/strtod.cc exact_powers_of_ten[] — 10^0..10^22 精确幂 (23 项)
pub static EXACT_POWERS_OF_TEN: [f64; 23] = [
    1.0, 10.0, 100.0, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10,
    1e11, 1e12, 1e13, 1e14, 1e15, 1e16, 1e17, 1e18, 1e19, 1e20,
    1e21, 1e22, // 10^22 = 0x878678326EAC9 * 2^22 (v8 原注)
];

// ---- static 辅助 (v8 侧 ${statik.length} 个) ----
// 对账: ${statik.join(", ")}
/// TrimLeadingZeros/TrimTrailingZeros: 去首/尾 '0'
/// TrimToMaxSignificantDigits: 截到 780 位有效数字后递归
/// ReadUint64/ReadDiyFp: 数字串读入 (19 位内直接 u64)
/// DoubleStrtod: <=15 位且域内 → 直接乘精确幂 (guess 校正 round-up)
/// AdjustmentPowerOfTen: 10^k 校正因子
/// DiyFpStrtod: DiyFp 乘 cached power, 误差界判定正确舍入
/// BignumStrtod: 兜底大数路径 (bignum.cc)

/// v8: src/base/numbers/strtod.h Strtod(buffer, exponent)
/// 语义: buffer 为纯数字 [0-9] 非空不以 '0' 开头; 值 = digits * 10^exponent
/// 分派: 空→0.0; >780 位截断递归; 上溢→inf (>=10^309); 下溢→0.0 (<=10^-324);
///       DoubleStrtod → DiyFpStrtod → BignumStrtod 兜底
pub fn strtod(buffer: &[u8], exponent: i32) -> f64 {
    /// v8: 主分派原文见尾; 三级回退链
    todo!()
}

// ---- v8 主函数原文 (分派证据, Developer 对照) ----
${main.map((l) => `// ${l}`).join("\n")}
`;

writeSrc("v8-base", "numbers/strtod.rs.skel", body);
console.log(`strtod.h.js: exports=1 statics=${statik.length} consts=${consts.length} powTable=${powEntries2}`);
