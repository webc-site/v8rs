#!/usr/bin/env bun
// tool/src/wasm/baseline/liftoff-bailout-reasons.h.js
// 机械提取 v8/src/wasm/baseline/liftoff-bailout-reasons.h:
// LiftoffBailoutReason 枚举 (显式值) 与 ToString 名称表。

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/wasm/baseline/liftoff-bailout-reasons.h");

const em = hSrc.match(/enum LiftoffBailoutReason : int8_t \{([\s\S]*?)\n\};/);
if (!em) throw new Error("未找到 LiftoffBailoutReason");
const raw = em[1]
  .replace(/\/\/[^\n]*/g, "")
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);

const entries = [];
for (const item of raw) {
  const m = item.match(/k(\w+)\s*=\s*(\d+)/);
  if (m) entries.push({ name: m[1], value: Number(m[2]) });
  else entries.push({ name: item.replace(/^k/, ""), value: null }); // kNumBailoutReasons 哨兵
}
// 显式值填充
let next = 0;
for (const e of entries) {
  if (e.value === null) e.value = next;
  else next = e.value;
  next = e.value + 1;
}
const named = entries.filter((e) => e.name !== "NumBailoutReasons");
if (named.length !== 17) throw new Error(`条目 ${named.length}, 期望 17`);
if (named[0].name !== "NoReason" || named[0].value !== 0) throw new Error("kNoReason 锚点漂移");
if (named.find((e) => e.name === "OtherReason")?.value !== 20) throw new Error("kOtherReason=20 漂移");
const numSentinel = entries.find((e) => e.name === "NumBailoutReasons");
if (!numSentinel || numSentinel.value !== 21) throw new Error(`kNumBailoutReasons 漂移: ${numSentinel}`);

// ToString 表
const strTable = [...hSrc.matchAll(/case k(\w+):\s*\n\s*return "([^"]+)";/g)].map((m) => ({
  name: m[1],
  str: m[2],
}));
if (strTable.length !== entries.length) throw new Error(`ToString ${strTable.length} != 枚举 ${entries.length}`);

console.log(`LiftoffBailoutReason ${named.length} + 哨兵(=${numSentinel.value})`);

let rust = banner(
  "src/wasm/baseline/liftoff-bailout-reasons.h.js",
  "src/wasm/baseline/liftoff-bailout-reasons.h",
  "Liftoff 退出原因枚举与名称表 (直方图 V8.LiftoffBailoutReasons 对齐, 序号敏感)",
);

rust += `
/// Liftoff 放弃编译原因 (对应 enum LiftoffBailoutReason : int8_t)
/// 序号与 chromium 直方图 "V8.LiftoffBailoutReasons" 对齐, 禁重排;
/// kOtherReason=20 前留有前向兼容空档。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum LiftoffBailoutReason {
`;

for (const e of entries) {
  rust += `  /// k${e.name}${e.name === "NumBailoutReasons" ? " (哨兵)" : ""}\n  ${e.name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase()} = ${e.value},\n`;
}

rust += `}

impl LiftoffBailoutReason {
  /// 对应 LiftoffBailoutReasonToString
  #[inline]
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
`;

for (const t of strTable) {
  rust += `      Self::${t.name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase()} => "${t.str}",\n`;
}

rust += `    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(LiftoffBailoutReason::NO_REASON as i8 == 0);
  assert!(LiftoffBailoutReason::DECODE_ERROR as i8 == 1);
  assert!(LiftoffBailoutReason::GC as i8 == 13);
  assert!(LiftoffBailoutReason::WASMFX as i8 == 15);
  assert!(LiftoffBailoutReason::OTHER_REASON as i8 == 20);
  assert!(LiftoffBailoutReason::NUM_BAILOUT_REASONS as i8 == 21);
  assert!(LiftoffBailoutReason::NO_REASON.to_str() == "no bailout");
  assert!(LiftoffBailoutReason::OTHER_REASON.to_str() == "other reason");
};
`;
;

const p = writeSrc("v8-wasm-liftoff", "liftoff_bailout_reasons.rs", rust);
console.log(`written ${p}`);
