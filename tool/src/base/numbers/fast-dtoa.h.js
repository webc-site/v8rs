#!/usr/bin/env bun
// tool/src/base/numbers/fast-dtoa.h.js — 族: src/base/numbers/fast-dtoa.h + .cc
// 覆盖: FastDtoaMode 枚举 (2) + Grisu3 双版本 + RoundWeed 双版本 + DigitGen 双版本
//        + DivMagic 快除表 (8 项) + kTen4..kTen9 常量 + 指数窗常量
// → v8-base/src/numbers/fast_dtoa.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/numbers/fast-dtoa.h");
const ccSrc = readV8("src/base/numbers/fast-dtoa.cc");

// —— 枚举真解析 ——
const enumBody = hSrc.match(/enum FastDtoaMode \{([\s\S]*?)\};/)[1];
const modes = [...enumBody.matchAll(/^\s{2}(FAST_DTOA_\w+),?$/gm)].map((m) => m[1]);
if (modes.join(",") !== "FAST_DTOA_SHORTEST,FAST_DTOA_PRECISION") throw new Error(`模式枚举 ${modes}`);

// —— 锚点断言 ——
if (!hSrc.includes("const int kFastDtoaMaximalLength = 17;")) throw new Error("缺最大长度常量");
if (!hSrc.includes("V8_BASE_EXPORT bool FastDtoa(double d, FastDtoaMode mode, int requested_digits,")) throw new Error("缺入口声明");
if (!ccSrc.includes("static const int kMinimalTargetExponent = -60;")) throw new Error("缺指数窗下界");
if (!ccSrc.includes("static const int kMaximalTargetExponent = -32;")) throw new Error("缺指数窗上界");
if (!ccSrc.includes("static bool RoundWeed(char* last_digit, uint64_t distance_too_high_w,")) throw new Error("缺 RoundWeed");
if (!ccSrc.includes("static bool RoundWeedCounted(Vector<char> buffer, int length, uint64_t rest,")) throw new Error("缺 RoundWeedCounted");
if (!ccSrc.includes("static bool DigitGen(DiyFp low, DiyFp w, DiyFp high, char** outptr,")) throw new Error("缺 DigitGen");
if (!ccSrc.includes("static bool DigitGenCounted(DiyFp w, int requested_digits, Vector<char> buffer,")) throw new Error("缺 DigitGenCounted");
if (!ccSrc.includes("static bool Grisu3(double v, char** outptr, int* decimal_exponent)")) throw new Error("缺 Grisu3");
if (!ccSrc.includes("static bool Grisu3Counted(double v, int requested_digits, Vector<char> buffer,")) throw new Error("缺 Grisu3Counted");
if (!ccSrc.includes("static inline uint32_t fast_divmod(uint32_t* val, uint32_t divisor,")) throw new Error("缺快除");
if (!ccSrc.includes("static inline void BiggestPowerTen(uint32_t number, int number_bits,")) throw new Error("缺最大十幂");

// —— 真解析: kTen 常量与 DivMagic 表 ——
const tens = [...ccSrc.matchAll(/^static const uint32_t (kTen\d) = (\d+);/gm)].map((m) => `${m[1]}=${m[2]}`);
if (tens.length !== 6) throw new Error(`kTen 常量 ${tens.length} != 6`);
const divMagic = [...ccSrc.matchAll(/\{(0x[\da-fA-F]+), (\d+)\},?\s*(?:\/\/ (\d+))?/g)].map((m) => ({ mul: m[1], shift: m[2], divisor: m[3] }));
if (divMagic.length !== 8 || divMagic[0].divisor !== "10" || divMagic[7].divisor !== "100000000") {
  throw new Error(`DivMagic 十六进制项 ${divMagic.length} 异常`);
}
if (!ccSrc.includes("{0, 0},            // Not used")) throw new Error("缺 {0,0} 占位项");
console.log(`fast-dtoa: 模式 2, Grisu 内核 2, RoundWeed 2, DigitGen 2, kTen 常量 ${tens.length}, DivMagic ${divMagic.length} 项`);

let rust = banner(
  "src/base/numbers/fast-dtoa.h.js",
  "src/base/numbers/fast-dtoa.h + src/base/numbers/fast-dtoa.cc",
  "Grisu3 快速 double→ASCII (失败须回退 bignum)",
);

rust += `
//! v8: src/base/numbers/fast-dtoa.h/.cc

/// v8: fast-dtoa.h FastDtoaMode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FastDtoaMode {
    /// 最短表示: v == (double)(buffer * 10^(point-length)); 末位最接近 v
    Shortest,
    /// 定精度: 误差绝对值最小; 等距两可时返回 false (不信结果)
    Precision,
}

/// v8: fast-dtoa.h kFastDtoaMaximalLength = 17 (不含 NUL)
pub const FAST_DTOA_MAXIMAL_LENGTH: usize = 17;

// v8: fast-dtoa.cc 指数窗 (选缓存幂 10^-k 使缩放后指数落窗内)
/// 窗下界
pub const MINIMAL_TARGET_EXPONENT: i32 = -60;
/// 窗上界
pub const MAXIMAL_TARGET_EXPONENT: i32 = -32;

// v8: fast-dtoa.cc kTen4..kTen9 — ${tens.map((t) => t.split("=")[1]).join(", ")}

/// v8: fast-dtoa.cc DivMagic — libdivide 无分支 u32 快除魔数
/// mul = ceil(2^(33+shift) / divisor) 截 32 位; shift = floor(log2(divisor))
#[derive(Debug, Clone, Copy)]
pub struct DivMagic { pub mul: u32, pub shift: u32 }

/// v8: fast-dtoa.cc div[] — 索引即 log10(除数); [0] 项不用 (除数 1 走分支)
pub static DIV_MAGIC: [DivMagic; 9] = [
    DivMagic { mul: 0, shift: 0 }, // v8: 未用 (除数 1 走分支)
${divMagic.map((d) => `    DivMagic { mul: ${d.mul}, shift: ${d.shift} }, // v8: ${d.divisor}`).join("\n")}
];

/// v8: fast-dtoa.cc fast_divmod — 返回 *val / divisor, 就地 *val %= divisor;
/// libdivide 无分支算法 + 除数 1 特判
pub fn fast_divmod(val: &mut u32, divisor: u32, d: DivMagic) -> u32;

/// v8: fast-dtoa.cc BiggestPowerTen(number, number_bits) — 找 ≤ number 的最大 10 幂 (幂指数, 值)
pub fn biggest_power_ten(number: u32, number_bits: i32) -> (u32, i32);

/// v8: fast-dtoa.cc RoundWeed — 尾位修剪: unsafe_interval/margin 四参不等式判定回退或进位
pub fn round_weed(last_digit: &mut [u8], distance_too_high_w: u64, unsafe_interval: u64, rest: u64, ten_kappa: u64, unit: u64) -> bool;
/// v8: fast-dtoa.cc RoundWeedCounted — 计数版修剪
pub fn round_weed_counted(buffer: &mut [u8], length: usize, rest: u64, ten_kappa: u64, unit: u64, margin_delta: u64) -> bool;

/// v8: fast-dtoa.cc DigitGen — low < w < high 区间内逐位生成, RoundWeed 收尾
pub fn digit_gen(low: DiyFp, w: DiyFp, high: DiyFp) -> Option<(Vec<u8>, i32)>;
/// v8: fast-dtoa.cc DigitGenCounted — requested_digits 计数版
pub fn digit_gen_counted(w: DiyFp, requested_digits: i32) -> Option<(Vec<u8>, i32)>;

/// v8: fast-dtoa.cc Grisu3 — AsDiyFp 边界 × 10^-k 缩放进 [-60,-32] 窗后 DigitGen
pub fn grisu3(v: f64) -> Option<(Vec<u8>, i32)>;
/// v8: fast-dtoa.cc Grisu3Counted — 计数版 (Grisu 精度不足处理真半数, 等距返回 None)
pub fn grisu3_counted(v: f64, requested_digits: i32) -> Option<(Vec<u8>, i32)>;

/// v8: fast-dtoa.cc FastDtoa — SHORTEST→Grisu3 / PRECISION→Grisu3Counted;
/// 前置 v 严格正有限; 失败返回 None (调用方回退 BignumDtoa)
pub fn fast_dtoa(v: f64, mode: FastDtoaMode, requested_digits: i32) -> Option<(Vec<u8>, i32)>;
`;

writeSrc("v8-base", "numbers/fast_dtoa.rs.skel", rust);
console.log("written v8-base/src/numbers/fast_dtoa.rs.skel");
