#!/usr/bin/env bun
// tool/src/bigint/bitwise-and.js — 台账项 [bigint] 缺新算法: BigInt::BitwiseAnd → v8-bigint
// 覆盖原版四面:
//   1) v8/src/bigint/bigint.h:271-275 BitwiseAnd_PosPos/NegNeg/PosNeg 声明 (digit 内核已在 bigint.rs 1:1 落地)
//   2) v8/src/bigint/bigint-inl.h BitwiseAnd 函数区: 循环/借位/Add(Z,1)/补码锚
//   3) v8/src/builtins/builtins-bigint.tq BigIntBitwiseAndImpl: 零短路 ×2 / 长度公式 ×2 / 分配符号 ×4 / 换参调用
//   4) v8/src/objects/bigint.cc:1723-1757 MutableBigInt_BitwiseAnd*AndCanonicalize 三壳 (heap 面, 推迟登记)
// 金标准: v8/test/mjsunit/harmony/bigint/and.js 20 组向量逐项核对进 v8-bigint/tests/bitwise_and.rs
// 产物: v8-bigint/src/bitwise.rs.skel (签名表 + 落地/推迟处置); 复跑幂等 (只写 skel, 交叉核对只读)
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const h = readV8("src/bigint/bigint.h");
const inl = readV8("src/bigint/bigint-inl.h");
const tq = readV8("src/builtins/builtins-bigint.tq");
const cc = readV8("src/objects/bigint.cc");
const andJs = readV8("test/mjsunit/harmony/bigint/and.js");
const kernel = readFileSync(join(root, "v8-bigint", "src", "bigint.rs"), "utf8");
const landed = readFileSync(join(root, "v8-bigint", "src", "bitwise.rs"), "utf8");
const testSrc = readFileSync(join(root, "v8-bigint", "tests", "bitwise_and.rs"), "utf8");

let assertCount = 0;
const fail = (label, got, want) => {
  throw new Error(`对数失配: ${label} got=${JSON.stringify(got)} want=${JSON.stringify(want)}`);
};
const count = (src, re) => (src.match(re) || []).length;
// 截取 [startRe, endRe) 区域原文 (含 startRe 命中行本身)
const region = (src, startRe, endRe) => {
  const s = src.search(startRe);
  if (s < 0) fail(`区域起点 ${startRe}`, false, true);
  const e = src.slice(s).search(endRe);
  if (e < 0) fail(`区域终点 ${endRe}`, false, true);
  return src.slice(s, s + e);
};

// 1) bigint.h 声明面: 恰好 3 个 BitwiseAnd_* 声明
const DECLS = ["BitwiseAnd_PosPos", "BitwiseAnd_NegNeg", "BitwiseAnd_PosNeg"];
for (const name of DECLS) {
  if (count(h, new RegExp(`inline void ${name}\\(RWDigits Z, Digits X, Digits Y\\);`, "g")) !== 1)
    fail(`声明 ${name}`, 0, 1);
  assertCount++;
}
if (count(h, /inline void BitwiseAnd_/g) !== 3) fail("头文件 BitwiseAnd_* 声明总数", count(h, /inline void BitwiseAnd_/g), 3);
assertCount++;

// 2) bigint-inl.h 实现区 (PosPos 起 ~ BitwiseOr_PosPos 止)
const andRegion = region(inl, /inline void BitwiseAnd_PosPos/, /inline void BitwiseOr_PosPos/);
if (count(andRegion, /^inline void BitwiseAnd_/gm) !== 3) fail("inl.h BitwiseAnd_* 定义数", count(andRegion, /^inline void BitwiseAnd_/gm), 3);
assertCount++;
if (count(andRegion, /uint32_t pairs = std::min\(X\.len\(\), Y\.len\(\)\);/g) !== 3)
  fail("pairs = min(X,Y) 锚", count(andRegion, /uint32_t pairs = std::min\(X\.len\(\), Y\.len\(\)\);/g), 3);
assertCount++;
if (count(andRegion, /for \(; i < Z\.len\(\); i\+\+\) Z\[i\] = 0;/g) !== 3)
  fail("高位清零循环锚", count(andRegion, /for \(; i < Z\.len\(\); i\+\+\) Z\[i\] = 0;/g), 3);
assertCount++;
for (const [re, want, label] of [
  [/Z\[i\] = X\[i\] & Y\[i\];/g, 1, "PosPos digit 与"],
  [/digit_t x_borrow = 1;/g, 1, "NegNeg x_borrow 初值"],
  [/digit_t y_borrow = 1;/g, 1, "NegNeg y_borrow 初值"],
  [/digit_sub\(X\[i\], x_borrow, &x_borrow\)/g, 2, "NegNeg X 减 1 借位链 (pairs 循环 + 尾循环)"],
  [/digit_sub\(Y\[i\], y_borrow, &y_borrow\)/g, 2, "NegNeg Y 减 1 借位链 (pairs 循环 + 尾循环)"],
  [/DCHECK\(x_borrow == 0\);/g, 1, "NegNeg 借位收敛 DCHECK"],
  [/Add\(Z, 1\);/g, 1, "NegNeg 补码取负回补 +1"],
  [/Z\[i\] = X\[i\] & ~digit_sub\(Y\[i\], borrow, &borrow\);/g, 1, "PosNeg x & ~(y-1)"],
  [/for \(; i < X\.len\(\); i\+\+\) Z\[i\] = X\[i\];/g, 1, "PosNeg 正侧高位直传"],
]) {
  if (count(andRegion, re) !== want) fail(label, count(andRegion, re), want);
  assertCount++;
}

// 3) builtins-bigint.tq BigIntBitwiseAndImpl (macro 起 ~ builtin BigIntBitwiseAndNoThrow 止)
const impl = region(tq, /macro BigIntBitwiseAndImpl/, /builtin BigIntBitwiseAndNoThrow/);
for (const [re, want, label] of [
  [/if \(xlength == 0\) \{/g, 1, "零短路 xlength==0"],
  [/if \(ylength == 0\) \{/g, 1, "零短路 ylength==0"],
  [/\(xlength < ylength\) \? xlength : ylength/g, 1, "PosPos 长度 min 公式"],
  [/\(\(xlength > ylength\) \? xlength : ylength\) \+ 1/g, 1, "NegNeg 长度 max+1 公式"],
  [/AllocateEmptyBigIntNoThrow\(kPositiveSign, resultLength\)/g, 1, "PosPos 正号分配"],
  [/AllocateEmptyBigIntNoThrow\(kNegativeSign, resultLength\)/g, 1, "NegNeg 负号分配"],
  [/AllocateEmptyBigIntNoThrow\(kPositiveSign, xlength\)/g, 1, "PosNeg 分配 (正侧 x 长度)"],
  [/AllocateEmptyBigIntNoThrow\(kPositiveSign, ylength\)/g, 1, "NegPos 分配 (正侧 y 长度)"],
  [/CppBitwiseAndPosPosAndCanonicalize\(result, x, y\);/g, 1, "PosPos 壳调用"],
  [/CppBitwiseAndNegNegAndCanonicalize\(result, x, y\);/g, 1, "NegNeg 壳调用"],
  [/CppBitwiseAndPosNegAndCanonicalize\(result, x, y\);/g, 1, "PosNeg 壳调用 (正 X)"],
  [/CppBitwiseAndPosNegAndCanonicalize\(result, y, x\);/g, 1, "NegPos 换参壳调用 (bigint.h:274 callers must swap)"],
  [/otherwise unreachable;/g, 3, "不可超预算分支 ×3 (PosPos/PosNeg/NegPos)"],
  [/otherwise BigIntTooBig;/g, 1, "可超预算分支 (仅 NegNeg)"],
]) {
  if (count(impl, re) !== want) fail(label, count(impl, re), want);
  assertCount++;
}

// 4) objects/bigint.cc 三壳: 各调对应内核一次 (heap 面, 推迟登记)
if (count(cc, /void MutableBigInt_BitwiseAnd\w+AndCanonicalize\(/g) !== 3)
  fail("objects/bigint.cc BitwiseAnd 壳定义数", count(cc, /void MutableBigInt_BitwiseAnd\w+AndCanonicalize\(/g), 3);
assertCount++;
for (const name of DECLS) {
  if (count(cc, new RegExp(`bigint::${name}\\(result->rw_digits\\(\\), x->digits\\(\\), y->digits\\(\\)\\);`, "g")) !== 1)
    fail(`内核调用 ${name}`, 0, 1);
  assertCount++;
}

// 5) 金标准向量: and.js data 数组逐项解析 (r: 0n 为十进制零, 前缀 0x 可选)
const vectors = [...andJs.matchAll(/\{\s*a: (-?)(?:0x)?([0-9a-f]+)n,\s*b: (-?)(?:0x)?([0-9a-f]+)n,\s*r: (-?)(?:0x)?([0-9a-f]+)n\s*\}/g)]
  .map((m) => ({ na: m[1] === "-", a: m[2], nb: m[3] === "-", b: m[4], nr: m[5] === "-", r: m[6] }));
if (vectors.length !== 20) fail("and.js 金标准组数", vectors.length, 20);
assertCount++;

// ── 产物: 签名表骨架 ───────────────────────────────────────────────────────
const skel = `${banner(
  "src/bigint/bitwise-and.js",
  "src/bigint/bigint.h + src/builtins/builtins-bigint.tq",
  "BigInt::BitwiseAnd 签名表与处置; 落地以 bitwise.rs / bigint.rs 就地修订为准",
)}
//! BitwiseAnd 落地处置 (台账 [bigint] 缺新算法: BigInt::BitwiseAnd):
//! - digit 内核 (bigint-inl.h:1146-1187, 已 1:1 在 bigint.rs):
//!   BitwiseAnd_PosPos → bitwise_and_pos_pos(&mut [Digit], &[Digit], &[Digit])
//!   BitwiseAnd_NegNeg → bitwise_and_neg_neg  (补码: -(((x-1)|(y-1))+1), Add(Z,1) 回补)
//!   BitwiseAnd_PosNeg → bitwise_and_pos_neg  (x & ~(y-1), 调用方负责换参)
//! - 顶层编排 (builtins-bigint.tq:454 BigIntBitwiseAndImpl → src/bitwise.rs, 本轮落地):
//!   bitwise_and_result_length(x, x_neg, y, y_neg) -> (负?, 预算 digit 数)  [min / max+1 / 正侧长度]
//!   bitwise_and(z, x, x_neg, y, y_neg) -> (负?, 规格化视图)                 [符号派发 + Canonicalize→norm]
//!   bitwise_and_owned(x, x_neg, y, y_neg) -> (负?, Vec<Digit>)              [tq 分配面由 Vec 承担]
//! - 推迟 (heap/objects 硬耦合, 本 crate 边界外, 显式不实现):
//!   MutableBigInt_BitwiseAnd*AndCanonicalize 三壳 (objects/bigint.cc:1723-1757, Tagged<BigInt>/Factory)
//!   AllocateEmptyBigIntNoThrow / BigIntTooBig RangeError 抛错面 (tq 壳层, 归 L4 builtins)
`;
writeSrc("v8-bigint", "bitwise.rs.skel", skel);

// ── 落地对数核对 ───────────────────────────────────────────────────────────
// [归属文件, 正则, 期望次数, 名称]
const LAND_TABLE = [
  // 内核面 (bigint.rs)
  ["kernel", /pub fn bitwise_and_pos_pos\(z: &mut \[Digit\], x: &\[Digit\], y: &\[Digit\]\)/g, 1, "内核 PosPos 签名"],
  ["kernel", /pub fn bitwise_and_neg_neg\(z: &mut \[Digit\], x: &\[Digit\], y: &\[Digit\]\)/g, 1, "内核 NegNeg 签名"],
  ["kernel", /pub fn bitwise_and_pos_neg\(z: &mut \[Digit\], x: &\[Digit\], y: &\[Digit\]\)/g, 1, "内核 PosNeg 签名"],
  ["kernel", /z\[i\] = x\[i\] & y\[i\];/g, 1, "PosPos digit 与落地"],
  ["kernel", /\(-x\) & \(-y\) == -\(\(\(x-1\) \| \(y-1\)\) \+ 1\)/g, 1, "NegNeg 补码公式注释落地"],
  ["kernel", /x & \(-y\) == x & ~\(y-1\)/g, 1, "PosNeg 补码公式注释落地"],
  ["kernel", /z\[i\] = x\[i\] & !v;/g, 1, "PosNeg x & ~(y-1) 落地"],
  // 编排面 (bitwise.rs)
  ["landed", /pub fn bitwise_and_result_length\(/g, 1, "预算函数签名"],
  ["landed", /pub fn bitwise_and<'a>\(/g, 1, "编排函数签名"],
  ["landed", /pub fn bitwise_and_owned\(/g, 1, "自分配版本签名"],
  ["landed", /if xlen == 0 \|\| ylen == 0 \{/g, 1, "零短路落地"],
  ["landed", /\(false, xlen\.min\(ylen\) as u32\)/g, 1, "PosPos min 公式落地"],
  ["landed", /\(true, xlen\.max\(ylen\) as u32 \+ 1\)/g, 1, "NegNeg max+1 公式落地"],
  ["landed", /\(false, xlen as u32\)/g, 1, "PosNeg 正侧长度落地"],
  ["landed", /\(false, ylen as u32\)/g, 1, "NegPos 正侧长度落地"],
  ["landed", /bitwise_and_pos_pos\(z, xn, yn\)/g, 1, "PosPos 派发落地"],
  ["landed", /bitwise_and_neg_neg\(z, xn, yn\)/g, 1, "NegNeg 派发落地"],
  ["landed", /bitwise_and_pos_neg\(z, xn, yn\)/g, 1, "PosNeg 派发落地"],
  ["landed", /bitwise_and_pos_neg\(z, yn, xn\)/g, 1, "NegPos 换参派发落地"],
  ["landed", /\(x_negative && y_negative, norm\(z\)\)/g, 1, "sign+norm 落地"],
  ["landed", /AllocateEmptyBigIntNoThrow/g, 1, "推迟登记锚 (tq 分配面)"],
  ["landed", /BigIntTooBig/g, 1, "推迟登记锚 (tq 抛错面)"],
  ["landed", /MutableBigInt_BitwiseAnd/g, 1, "推迟登记锚 (objects 三壳)"],
];
const SRC = { kernel, landed };
const missing = [];
for (const [which, re, want, label] of LAND_TABLE) {
  const got = count(SRC[which], re);
  if (got !== want) missing.push(`${label}: got=${got} want=${want}`);
  assertCount++;
}
// 金标准逐项落地: 20 组的 a/b/r 幅度 hex 必须原样出现在 tests/bitwise_and.rs 表内
if (count(testSrc, /\[\(bool, &str, bool, &str, bool, &str\); 20\]/g) !== 1)
  missing.push("金标准表容量声明 20: 未找到");
assertCount++;
for (const v of vectors) {
  for (const hex of [v.a, v.b, v.r]) {
    if (!testSrc.includes(`"${hex}"`)) missing.push(`and.js 向量缺失: 0x${hex}n`);
    assertCount++;
  }
}
if (count(testSrc, /#\[test\]\nfn /g) !== 6) {
  missing.push(`测试函数数: got=${count(testSrc, /#\[test\]/g)} want=6`);
}
assertCount++;
if (missing.length) throw new Error(`bitwise-and 落地对数缺项 (票单):\n  ${missing.join("\n  ")}`);
// 覆盖不变式: 原版头声明 3 = 内核落地 3 = 编排派发调用 4 (PosNeg 双分支换参) - 1
if (count(kernel, /pub fn bitwise_and_(pos_pos|neg_neg|pos_neg)\(/g) !== count(h, /inline void BitwiseAnd_/g))
  fail("覆盖不变式 (声明 3 = 内核落地 3)", count(kernel, /pub fn bitwise_and_\w+\(/g), 3);
assertCount++;

console.log(
  `bitwise-and.js: 对数断言 ${assertCount} 条全过; 头文件 3 声明 / inl.h 实现区 3 定义 (min 锚×3, 借位链×2+2, Add(Z,1)×1, 清零循环×3) / ` +
    `tq 编排 14 锚 (零短路×2, 长度公式×2, 分配×4, 换参×1, otherwise unreachable×3 + BigIntTooBig×1) / ` +
    `cc 壳 3 定义 + 3 内核调用 / and.js 金标准 20 组×3 值逐项落地 tests/bitwise_and.rs; ` +
    `落地 23 锚命中, 缺项 0; 产物 v8-bigint/src/bitwise.rs.skel`,
);
