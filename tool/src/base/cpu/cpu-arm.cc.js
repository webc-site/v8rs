#!/usr/bin/env bun
// tool/src/base/cpu/cpu-arm.cc.js — 覆盖 v8/src/base/cpu/cpu-arm.cc
// HWCAP 特征位 #define 全量展开 (arm64 目标取 HWCAP_ 块; 32 位 ARM HWCAP2/旧块登记)
// → v8-base/src/cpu_arm.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const ccSrc = readV8("src/base/cpu/cpu-arm.cc");

// 1. HWCAP_* 常量 (#define HWCAP_X (1 << n) / 复合)
const caps = [...ccSrc.matchAll(/#define (HWCAP\w*) \(([^)]+)\)/g)].map((m) => ({
  name: m[1], expr: m[2].replace(/\s+/g, " "),
}));
if (caps.length < 40) throw new Error(`HWCAP 常量数异常: ${caps.length}`);
const byName = Object.fromEntries(caps.map((c) => [c.name, c]));
// 锚点: 32 位块 SWP(bit0), arm64 块 FP(bit0)
if (byName.HWCAP_SWP?.expr !== "1 << 0") throw new Error("HWCAP_SWP 锚点不符");
if (byName.HWCAP_FP?.expr !== "1 << 0") throw new Error("HWCAP_FP 锚点不符");

// 2. 分块归属: 32 位 ARM 块 (SWP..HWCAP_LPAE 等) 与 arm64 块 (FP..)
// arm64 目标: 取 arm64 块 (HWCAP_FP 起); 32 位块登记不产码 (rule 9)
const arm64Start = caps.findIndex((c) => c.name === "HWCAP_FP");
if (arm64Start === -1) throw new Error("未找到 arm64 HWCAP 块起点");
const arm64Caps = caps.slice(arm64Start);
const arm32Caps = caps.slice(0, arm64Start);
if (arm64Caps.length < 20) throw new Error(`arm64 块条数异常: ${arm64Caps.length}`);

// 3. 复合宏 (HWCAP_IDIV = IDIVA|IDIVT) 展开验证
if (byName.HWCAP_IDIV && !byName.HWCAP_IDIV.expr.includes("|")) throw new Error("复合宏形态异常");

console.log(`cpu-arm HWCAP 总 ${caps.length} (arm64 ${arm64Caps.length}, 32 位 ${arm32Caps.length})`);

// expr → Rust u64 值 (支持 1 << n 与 A | B)
function evalExpr(expr, depth = 0) {
  if (depth > 4) throw new Error(`表达式过深: ${expr}`);
  const orParts = expr.split("|").map((s) => s.trim()).filter(Boolean);
  if (orParts.length > 1) return orParts.reduce((a, p) => a | evalExpr(p, depth + 1), 0);
  const sh = expr.match(/^1U?L?\s*<<\s*(\d+)$/);
  if (sh) return 1n << BigInt(sh[1]);
  if (byName[expr]) return evalExpr(byName[expr].expr, depth + 1);
  throw new Error(`未识别的 HWCAP 表达式: ${expr}`);
}

let rust = banner("src/base/cpu/cpu-arm.cc.js", "src/base/cpu/cpu-arm.cc", "ARM HWCAP 特征位 (arm64 目标块; 32 位 ARM 块登记不产码)");

rust += `
/// arm64 HWCAP 位 (原版 getauxval(AT_HWCAP) 掩码, getauxval(AT_HWCAP) 定义于 cpu-arm.cc)
pub const HWCAP_ARM64: [(&str, u64); ${arm64Caps.length}] = [
`;
for (const c of arm64Caps) {
  rust += `  ("${c.name}", 0x${evalExpr(c.expr).toString(16)}),\n`;
}
rust += `];

/// 32 位 ARM HWCAP/HWCAP2 块 (非目标架构, 登记不产码)
pub const ARM32_CAP_NAMES_REGISTERED: [&str; ${arm32Caps.length}] = [
${arm32Caps.map((c) => `  "${c.name}",`).join("\n")}
];

/// 按名查 arm64 位值
#[inline]
#[must_use]
pub fn hwcap(name: &str) -> Option<u64> {
  HWCAP_ARM64.iter().find(|(n, _)| *n == name).map(|(_, v)| *v)
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(HWCAP_ARM64.len() == ${arm64Caps.length});
  assert!(HWCAP_ARM64[0].0 == "HWCAP_FP" && HWCAP_ARM64[0].1 == 0x1);
  assert!(hwcap("HWCAP_ASIMD") == Some(0x2));
};
`;

const p = writeSrc("v8-base", "cpu_arm.rs", rust);
console.log(`written ${p}`);
