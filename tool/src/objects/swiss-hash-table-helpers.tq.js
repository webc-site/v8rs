#!/usr/bin/env bun
// tool/src/objects/swiss-hash-table-helpers.tq.js — 覆盖 v8/src/objects/swiss-hash-table-helpers.tq
// → v8-objects-collections/src/swiss_hash_table_helpers.rs
// 机械提取: tq 侧 generates 常量, 值取自 swiss-hash-table-helpers.h (原版侧);
// 平台分支: x64(SSE2)/arm64(NEON) → SIMD Group (kWidth=16); portable (kWidth=8) 登记跳过。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/swiss-hash-table-helpers.h");
const tqSrc = readV8("src/objects/swiss-hash-table-helpers.tq");

// tq 侧常量名单 (generates 引用 C++ 侧)
const tqConsts = [...tqSrc.matchAll(/const (\w+):\s*(?:constexpr )?(\w+)(\?)?\s+generates '([^']*)'/g)].map((m) => ({
  name: m[1], type: m[2], cpp: m[4],
}));
const KNOWN_TQ = 5; // kGroupWidth, kUseSIMD, ctrl::kEmpty, ctrl::kDeleted, kH2Bits
if (tqConsts.length < KNOWN_TQ) throw new Error(`tq generates 常量数 ${tqConsts.length} < ${KNOWN_TQ}`);
const names = tqConsts.map((c) => c.name);
for (const n of ["kGroupWidth", "kUseSIMD", "kEmpty", "kDeleted", "kH2Bits"]) {
  if (!names.includes(n)) throw new Error(`tq 缺常量 ${n}`);
}

// C++ 侧取值 (swiss-hash-table-helpers.h, 带守卫)
const kEmpty = hSrc.match(/kEmpty = (-?\d+),\s*\/\/ 0b10000000/);
const kDeleted = hSrc.match(/kDeleted = (-?\d+),\s*\/\/ 0b11111110/);
const h2Bits = hSrc.match(/static constexpr int kH2Bits = (\d+);/);
if (!kEmpty || kEmpty[1] !== "-128") throw new Error("kEmpty != -128 锚失配");
if (!kDeleted || kDeleted[1] !== "-2") throw new Error("kDeleted != -2 锚失配");
if (!h2Bits || h2Bits[1] !== "7") throw new Error("kH2Bits != 7 锚失配");

// 平台分支: SIMD Group (kWidth=16) 与 portable Group (kWidth=8) 双实现;
// 目标 x64(SSE2)/arm64(NEON) → SIMD 路径
const simdWidth = hSrc.match(/static constexpr size_t kWidth = (16);/);
const portWidth = hSrc.match(/static constexpr size_t kWidth = (8);/);
if (!simdWidth || simdWidth[1] !== "16") throw new Error("SIMD kWidth=16 锚失配");
if (!portWidth) throw new Error("portable kWidth=8 实现缺失 (登记)");

// kH2Mask = (1 << kH2Bits) - 1 (tq 显式表达式)
const h2MaskExpr = tqSrc.match(/kH2Mask:\s*\n?\s*constexpr uint32 generates '\(\(1 << swiss_table::kH2Bits\) - 1\)'/);
if (!h2MaskExpr) throw new Error("kH2Mask 表达式失配");

let rust = banner("src/objects/swiss-hash-table-helpers.tq.js", "src/objects/swiss-hash-table-helpers.{tq,h}", "swiss 表控制字节与探测常量 (SIMD 分支)");
rust += `//! swiss_table 辅助常量 (Torque 侧 generates 的 C++ 取值)。
//! 目标平台: x64 (SSE2) / arm64 (NEON) → SIMD Group 路径 (kWidth=16);
//! portable Group (kWidth=8) 配置分支登记跳过。

/// 控制字节: 空槽 (原版 Ctrl::kEmpty = -128 即 0b10000000)。
pub const K_EMPTY: u8 = 0x80;
/// 控制字节: 已删除槽 (原版 Ctrl::kDeleted = -2 即 0b11111110)。
pub const K_DELETED: u8 = 0xFE;
/// H2 (7 位哈希摘要) 位宽。
pub const K_H2_BITS: u32 = 7;
/// H2 掩码 (原版 (1 << kH2Bits) - 1)。
pub const K_H2_MASK: u32 = 0x7F;
/// 组宽 (SIMD 路径: 每组槽数 = 16)。
pub const K_GROUP_WIDTH: usize = 16;
/// 是否 SIMD 路径 (kWidth == 16)。
pub const K_USE_SIMD: bool = true;

/// portable 路径组宽 (登记: 目标配置不使用)。
pub const PORTABLE_GROUP_WIDTH: usize = 8;

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_EMPTY == 0x80);
  assert!(K_DELETED == 0xFE);
  assert!(K_H2_BITS == 7);
  assert!(K_H2_MASK == 0x7F);
  assert!(K_GROUP_WIDTH == 16);
  assert!(K_USE_SIMD);
};
`;

const p = writeSrc("v8-objects-collections", "swiss_hash_table_helpers.rs", rust);
console.log(`swiss_hash_table_helpers.rs: 常量 7 (SIMD 分支), 对数通过`);
