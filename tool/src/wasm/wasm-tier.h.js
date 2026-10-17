#!/usr/bin/env bun
// tool/src/wasm/wasm-tier.h.js
// 机械提取 v8/src/wasm/wasm-tier.h 的执行层枚举

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-tier.h");

// ExecutionTier: kNone, [kInterpreter (DRUMBRAKE)], kLiftoff, kTurbofan
// 按默认目标配置展开 (V8_ENABLE_DRUMBRAKE 关闭); DRUMBRAKE 变体登记于注释
const drumbrake = hSrc.includes("V8_ENABLE_DRUMBRAKE");
if (!drumbrake) throw new Error("未找到 V8_ENABLE_DRUMBRAKE 分支标记");
const tierMatch = hSrc.match(/enum class ExecutionTier : int8_t\s*\{([^}]*)\};/);
if (!tierMatch) throw new Error("未找到 ExecutionTier 枚举");
const tierRaw = tierMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter((s) => s && !s.startsWith("//") && !s.includes("#"));
// 展开: kNone, kInterpreter(条件), kLiftoff, kTurbofan
const tiers = ["None", "Liftoff", "Turbofan"];
if (!tierMatch[1].includes("kInterpreter")) throw new Error("kInterpreter 分支缺失(应在 DRUMBRAKE 内)");

const strMatch = hSrc.match(
  /inline const char\* ExecutionTierToString\(ExecutionTier tier\)\s*\{([\s\S]*?)\n\}/,
);
if (!strMatch) throw new Error("未找到 ExecutionTierToString");
const strPairs = [...strMatch[1].matchAll(/ExecutionTier::k(\w+):\s*\n?\s*return "(\w+)";/g)].map(
  (m) => ({ name: m[1], str: m[2] }),
);
if (strPairs.length !== 4) throw new Error(`ExecutionTierToString 提取 ${strPairs.length} 条, 期望 4 (含 DRUMBRAKE 分支)`);

const dbgMatch = hSrc.match(/enum ForDebugging : int8_t\s*\{([^}]*)\};/);
if (!dbgMatch) throw new Error("未找到 ForDebugging 枚举");
const debugs = dbgMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean)
  .map((s) => {
    const m = s.match(/k(\w+)\s*=\s*(-?\d+)/);
    return m ? { name: m[1], repr: Number(m[2]) } : { name: s.replace(/^k/, ""), repr: null };
  });
// kNotForDebugging=0 显式, 其余顺序递增
if (debugs.length !== 4) throw new Error(`ForDebugging ${debugs.length} 条, 期望 4`);
let next = 0;
for (const d of debugs) {
  if (d.repr === null) d.repr = next;
  next = d.repr + 1;
}
if (debugs[0].name !== "NotForDebugging" || debugs[0].repr !== 0) throw new Error("ForDebugging 锚点漂移");

const stateMatch = hSrc.match(/enum DebugState : bool\s*\{([^}]*)\};/);
if (!stateMatch) throw new Error("未找到 DebugState 枚举");
const states = stateMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean)
  .map((s) => {
    const m = s.match(/k(\w+)\s*=\s*(\w+)/);
    if (!m) throw new Error(`DebugState 条目异常: ${s}`);
    return { name: m[1], repr: m[2] === "true" ? 1 : 0 };
  });
if (states.length !== 2 || states[0].name !== "NotDebugging") throw new Error("DebugState 锚点漂移");

console.log(`ExecutionTier ${tiers.length}(默认配置)+1(DRUMBRAKE), ForDebugging 4, DebugState 2`);

let rust = banner(
  "src/wasm/wasm-tier.h.js",
  "src/wasm/wasm-tier.h",
  "Wasm 执行层枚举 (DRUMBRAKE 按默认关闭展开, 变体登记注释)",
);

rust += `
/// Wasm 执行层 (对应 enum class ExecutionTier : int8_t)。
///
/// 原版在 V8_ENABLE_DRUMBRAKE 下于 None 与 Liftoff 之间多一个 kInterpreter;
/// 本产物按默认目标配置 (DRUMBRAKE 关闭) 展开, 序号随之下移。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i8)]
pub enum ExecutionTier {
  /// kNone
  None = 0,
  // kInterpreter = 1, // 仅 V8_ENABLE_DRUMBRAKE
  /// kLiftoff (基线层)
  Liftoff = 1,
  /// kTurbofan (优化层)
  Turbofan = 2,
}

impl ExecutionTier {
  /// 名称串 (对应 ExecutionTierToString)
  #[inline]
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
      Self::None => "none",
      Self::Liftoff => "liftoff",
      Self::Turbofan => "turbofan",
    }
  }
}

/// 调试用编译档位 (对应 enum ForDebugging : int8_t)
/// {kForDebugging} 默认降档代码; {kWithBreakpoints} 含断点; {kForStepping} 全量断点
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i8)]
pub enum ForDebugging {
`;

for (const d of debugs) {
  rust += `  /// k${d.name}\n  ${d.name} = ${d.repr},\n`;
}

rust += `}

/// 调试状态 (对应 enum DebugState : bool)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(bool)]
pub enum DebugState {
`;

for (const s of states) {
  rust += `  /// k${s.name}\n  ${s.name} = ${s.repr},\n`;
}

rust += `}

// —— 提取不变式断言 ——
const _: () = {
  assert!(ExecutionTier::None as i8 == 0);
  assert!(ExecutionTier::Liftoff as i8 == 1);
  assert!(ExecutionTier::Turbofan as i8 == 2);
  assert!(ForDebugging::NotForDebugging as i8 == 0);
  assert!(ForDebugging::ForDebugging as i8 == 1);
  assert!(ForDebugging::WithBreakpoints as i8 == 2);
  assert!(ForDebugging::ForStepping as i8 == 3);
  assert!(ExecutionTier::Liftoff.to_str() == "liftoff");
};
`;

const p = writeSrc("v8-wasm-core", "wasm_tier.rs", rust);
console.log(`written ${p}`);
