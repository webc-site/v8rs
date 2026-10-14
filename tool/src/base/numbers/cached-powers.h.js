#!/usr/bin/env bun
// tool/src/base/numbers/cached-powers.h.js — 族: src/base/numbers/cached-powers.h + .cc
// 覆盖: PowersOfTenCache — 87 项十进制幂缓存表 + 3 常量 + 2 查询
// → v8-base/src/numbers/cached_powers.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/numbers/cached-powers.h");
const ccSrc = readV8("src/base/numbers/cached-powers.cc");

// —— 锚点断言 ——
if (!hSrc.includes("class PowersOfTenCache {")) throw new Error("缺 PowersOfTenCache");
if (!hSrc.includes("static void GetCachedPowerForBinaryExponentRange(")) throw new Error("缺二进制指数查询");
if (!hSrc.includes("static void GetCachedPowerForDecimalExponent(")) throw new Error("缺十进制指数查询");
if (!ccSrc.includes("struct CachedPower {")) throw new Error("缺 CachedPower 结构");
if (!ccSrc.includes("static const CachedPower kCachedPowers[] = {")) throw new Error("缺缓存表");
if (!ccSrc.includes("static const int kCachedPowersOffset = 348;")) throw new Error("缺偏移常量");

// —— 真解析: 表项逐项展开, 对数 + 值域校验 ——
const entries = [...ccSrc.matchAll(/\{0x([\dA-F']+), (-?\d+), (-?\d+)\}/g)].map((m) => ({
  significand: BigInt("0x" + m[1].replace(/'/g, "")),
  binary_exponent: Number(m[2]),
  decimal_exponent: Number(m[3]),
}));
if (entries.length !== 87) throw new Error(`缓存表项 ${entries.length} != 87`);
// 十进制指数步进 8, 首项 -348
for (let i = 0; i < entries.length; i++) {
  if (entries[i].decimal_exponent !== -348 + 8 * i) throw new Error(`第 ${i} 项 decimal_exponent 错`);
}
const consts = {
  distance: ccSrc.match(/kDecimalExponentDistance = (\d+);/)?.[1],
  min: ccSrc.match(/kMinDecimalExponent = (-?\d+);/)?.[1],
  max: ccSrc.match(/kMaxDecimalExponent = (-?\d+);/)?.[1],
};
if (consts.distance !== "8" || consts.min !== "-348" || consts.max !== "340") throw new Error(`常量 ${JSON.stringify(consts)}`);
console.log(`cached-powers: 表项 ${entries.length} (decimal -348..+340 步进 8), 常量 3, 查询 2`);

let rust = banner(
  "src/base/numbers/cached-powers.h.js",
  "src/base/numbers/cached-powers.h + src/base/numbers/cached-powers.cc",
  "十的幂缓存 (Grisu 用)",
);

rust += `
//! v8: src/base/numbers/cached-powers.h/.cc

/// v8: cached-powers.cc CachedPower
#[derive(Debug, Clone, Copy)]
pub struct CachedPower {
    pub significand: u64,
    pub binary_exponent: i16,
    pub decimal_exponent: i16,
}

// v8: cached-powers.cc 常量
/// 相邻缓存项十进制指数差
pub const DECIMAL_EXPONENT_DISTANCE: i32 = 8;
/// 最小十进制指数
pub const MIN_DECIMAL_EXPONENT: i32 = -348;
/// 最大十进制指数
pub const MAX_DECIMAL_EXPONENT: i32 = 340;
/// 首项 decimal_exponent 的负值 (索引偏移)
pub const CACHED_POWERS_OFFSET: i32 = 348;
/// 表长 = (340 - (-348)) / 8 + 1
pub const CACHED_POWERS_LENGTH: usize = ${entries.length};

/// v8: cached-powers.cc kCachedPowers — ${entries.length} 项, 每项 (significand, binary_exp, decimal_exp);
/// decimal_exponent 从 -348 起 +8 步进; Developer: 展开全表或 include 数据文件
pub static CACHED_POWERS: [CachedPower; CACHED_POWERS_LENGTH] = [ /* v8: 逐项搬运 */ ];

pub struct PowersOfTenCache;

impl PowersOfTenCache {
    /// v8: cached-powers.cc GetCachedPowerForBinaryExponentRange
    /// 求 k 使 10^k 的二进制指数落入 [min, max]: index = foo = (-61 - min) 起算,
    /// 再除以 8 对齐表步进; 输出 power 与 decimal_exponent
    pub fn get_cached_power_for_binary_exponent_range(min_exponent: i32, max_exponent: i32) -> (DiyFp, i32);

    /// v8: cached-powers.cc GetCachedPowerForDecimalExponent
    /// index = (requested + kCachedPowersOffset) / 8 (截断);
    /// 前置: kMinDecimalExponent <= requested < kMaxDecimalExponent + kDecimalExponentDistance
    pub fn get_cached_power_for_decimal_exponent(requested_exponent: i32) -> (DiyFp, i32);
}
`;

writeSrc("v8-base", "numbers/cached_powers.rs.skel", rust);
console.log("written v8-base/src/numbers/cached_powers.rs.skel");
