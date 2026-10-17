#!/usr/bin/env bun
// tool/src/bigint/div-barrett.cc.js
// 机械提取 v8/src/bigint/ 除法四族: div-barrett.cc + div-burnikel.cc + div-schoolbook.cc
// + div-helpers-inl.h — 阈值分派、Burnikel 参数规则、内联骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const barrett = readV8("src/bigint/div-barrett.cc");
const burnikel = readV8("src/bigint/div-burnikel.cc");
const schoolbook = readV8("src/bigint/div-schoolbook.cc");
const helpersInl = readV8("src/bigint/div-helpers-inl.h");
const inlH = readV8("src/bigint/bigint-inl.h");

// 1. config 阈值 (burnikel/newton 平台无关单一值; barrett 32/64 成对)
const all = [...inlH.matchAll(/constexpr uint32_t kBurnikelThreshold = (\d+);/g)].map((x) => x[1]);
if (all.length !== 1 || all[0] !== "57") throw new Error(`kBurnikelThreshold 漂移: ${all.join()}`);
const newtonAll = [...inlH.matchAll(/constexpr uint32_t kNewtonInversionThreshold = (\d+);/g)].map((x) => x[1]);
if (newtonAll.length !== 1 || newtonAll[0] !== "25") throw Error("kNewtonInversionThreshold 漂移");
const barrettAll = [...inlH.matchAll(/constexpr uint32_t kBarrettThreshold = (\d+);/g)].map((x) => x[1]);
if (barrettAll.length !== 2 || barrettAll[1] !== "13000") throw Error("kBarrettThreshold 64 位漂移");

// 2. Burnikel 参数规则: m = 2^k 使 2^k * kBurnikelThreshold > s (原版注释 "1. Set m = ...")
if (!burnikel.includes("uint32_t m = 1 << BitLength(s / config::kBurnikelThreshold);")) {
  throw new Error("Burnikel m=2^k 规则漂移");
}
if ((burnikel.match(/config::kBurnikelThreshold/g) || []).length < 3) throw new Error("burnikel 阈值引用不足");
// Barrett 回落 Burnikel
if (!barrett.includes("n < config::kBurnikelThreshold")) throw new Error("barrett 回落分支漂移");

// 3. schoolbook 算法存在性
if (!schoolbook.includes("DivideSchoolbook") && !/^(void|int|uint)/m.test(schoolbook)) {
  throw new Error("schoolbook 定义缺失");
}

// 4. div-helpers-inl 内联面
const helperFns = [...helpersInl.matchAll(/^inline (?:void|digit_t|uint32_t|bool) (\w+)\(/gm)].map((m) => m[1]);
if (helperFns.join() !== "Copy,LeftShift,RightShift,PutAt") {
  throw new Error(`div-helpers-inl 清单漂移: ${helperFns.join()}`);
}

let rust = banner(
  "src/bigint/div-barrett.cc.js",
  "src/bigint/div-{barrett,burnikel,schoolbook}.cc + div-helpers-inl.h",
  "除法四族阈值与骨架",
);

rust += `
/// 除法算法分派阈值 (原版 bigint-inl.h config; 64 位为当前目标)
pub mod div_config {
  /// burnikel 起步界 (平台无关)
  pub const BURNIKEL_THRESHOLD: u32 = ${all[0]};
  /// newton 求逆起步界 (供 barrett 倒数预计算)
  pub const NEWTON_INVERSION_THRESHOLD: u32 = ${newtonAll[0]};
  /// burnikel → barrett 交接 (9K-13K 互有胜负, 13K 上 barrett 稳赢)
  pub const BARRETT_THRESHOLD_32: u32 = ${barrettAll[0]};
  pub const BARRETT_THRESHOLD_64: u32 = ${barrettAll[1]};
}

/// Burnikel 参数规则 (原版 div-burnikel.cc):
/// m = min { 2^k | 2^k * BURNIKEL_THRESHOLD > s }, 以 2^m 切分被除数
pub mod burnikel_skel {
  /// n 为奇数或 n < 阈值时回落 schoolbook (原版分支)
  pub fn fallback_to_schoolbook(_n: u32) -> bool {
    unimplemented!("骨架")
  }
  /// Barrett 除法主入口: n < BURNIKEL_THRESHOLD 先走 Burnikel (原版分支)
  pub fn divide_barrett() {
    unimplemented!("骨架")
  }
}

/// div-helpers-inl 内联骨架 (原版语义, 实现见手写 div_helpers.rs)
pub mod div_helpers_inl_skel {
  /// 拷贝并清零补齐 (原版 Copy)
  pub fn copy(_z: &mut [u64], _x: &[u64]) {
    unimplemented!("骨架")
  }
  /// 左移按位拼接 (原版 LeftShift)
  pub fn left_shift(_z: &mut [u64], _x: &[u64], _shift: i32) {
    unimplemented!("骨架")
  }
  /// 右移丢弃低位 (原版 RightShift)
  pub fn right_shift(_z: &mut [u64], _x: &[u64], _shift: i32) {
    unimplemented!("骨架")
  }
  /// 块放置: A 重复 count 份写入 Z (原版 PutAt)
  pub fn put_at(_z: &mut [u64], _a: &[u64], _count: u32) {
    unimplemented!("骨架")
  }
}

// div-barrett.cc 定义头清单:
${collectDefs(barrett).map((s) => "// " + s).join("\n")}

// div-burnikel.cc 定义头清单:
${collectDefs(burnikel).map((s) => "// " + s).join("\n")}

// div-schoolbook.cc 定义头清单:
${collectDefs(schoolbook).map((s) => "// " + s).join("\n")}
`;

function collectDefs(src) {
  const out = [];
  const ls = src.split("\n");
  for (let i = 0; i < ls.length; i++) {
    const l = ls[i];
    if (!/^(?:void|int|uint32_t|uint64_t|bool|static|inline)\s/.test(l) || !/\(/.test(l)) continue;
    let sig = l;
    let j = i;
    while (!/\{\s*$|;\s*$/.test(sig) && j + 1 < ls.length) sig += " " + ls[++j].trim();
    sig = sig.replace(/\{\s*$/, "").trim();
    if (sig.endsWith(";")) continue;
    out.push(sig);
  }
  return out;
}

const p = writeSrc("v8-bigint", "div_generated.rs", rust);
console.log(`written ${p}; burnikel=${all[0]}, barrett=${barrettAll[1]}, 内联=${helperFns.length}`);
