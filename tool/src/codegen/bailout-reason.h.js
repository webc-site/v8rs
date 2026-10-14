#!/usr/bin/env bun
// tool/src/codegen/bailout-reason.h.js
// 机械展开 v8/src/codegen/bailout-reason.{h,cc} 的 X-macro 消息表,
// 产出 BailoutReason / AbortReason 枚举 + 静态消息表 → v8-codegen-core/src/bailout.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/codegen/bailout-reason.h");
const ccSrc = readV8("src/codegen/bailout-reason.cc");

// 行续接符拼接: "#define X ... \\\n ..." → 单行, 便于整段提取
const flat = hSrc.replace(/\\\r?\n/g, " ");

// 提取 #define 名字(V) 开头、到下一个行首 #define/#undef 之前的宏体
function macroBody(name) {
  const re = new RegExp(`#define ${name}\\(V\\)([\\s\\S]*?)(?=#define |#undef |\\nenum )`);
  const m = flat.match(re);
  if (!m) throw new Error(`未找到宏 ${name}`);
  return m[1];
}

// V(name, "msg") 条目; 兼容 IF_TARGET_ARCH_X64(V, name, "msg"), 单遍合并保持原文顺序
function expandV(body, ctx) {
  const out = [];
  const re = /V\((\w+),\s*"((?:[^"\\]|\\.)*)"\)|IF_TARGET_ARCH_X64\(V,\s*(\w+),\s*"((?:[^"\\]|\\.)*)"\)/g;
  let m;
  while ((m = re.exec(body)) !== null) {
    if (m[1] !== undefined) {
      out.push({ name: m[1], msg: m[2] });
    } else {
      // 平台分支: 按 x64 目标展开; arm64 目标无此条目, 此处登记
      out.push({ name: m[3], msg: m[4], note: "IF_TARGET_ARCH_X64: 仅 x64 目标展开; arm64 目标无此条" });
    }
  }
  if (out.length === 0) throw new Error(`${ctx}: 零条目, 解析失败`);
  return out;
}

const abortList = expandV(macroBody("ABORT_MESSAGES_LIST"), "ABORT_MESSAGES_LIST");
const terminalBailout = expandV(macroBody("TERMINAL_BAILOUT_MESSAGES_LIST"), "TERMINAL_BAILOUT");
const terminalMaglev = expandV(macroBody("TERMINAL_MAGLEV_BAILOUT_MESSAGES_LIST"), "TERMINAL_MAGLEV");
const terminalTurbofan = expandV(macroBody("TERMINAL_TURBOFAN_BAILOUT_MESSAGES_LIST"), "TERMINAL_TURBOFAN");

// BAILOUT_MESSAGES_LIST 是组合宏: 展开前先拼接子宏体 (保持原版拼接顺序)
const bailoutRaw = macroBody("BAILOUT_MESSAGES_LIST");
const bailoutBody = bailoutRaw.replace(
  /TERMINAL_BAILOUT_MESSAGES_LIST\(V\)|TERMINAL_MAGLEV_BAILOUT_MESSAGES_LIST\(V\)|TERMINAL_TURBOFAN_BAILOUT_MESSAGES_LIST\(V\)/g,
  (tag) => {
    const name = tag.replace(/\(V\)$/, "");
    return macroBody(name);
  },
);
const bailoutList = expandV(bailoutBody, "BAILOUT_MESSAGES_LIST");

// 对数: 与原版逐项核对 (末尾哨兵 kLastErrorMessage 在 enum 里追加, 不在列表中)
// 原版侧基准 (sed 独立计数): BAILOUT = kNoReason + TERMINAL 4/2/2 + 4 条独立 = 13;
// ABORT = 89 条 V(k...) + 1 条 IF_TARGET_ARCH_X64 = 90 (含 kNoReason 与 kLastReason)
const EXPECT_BAILOUT = 13;
const EXPECT_ABORT = 90;
if (bailoutList.length !== EXPECT_BAILOUT)
  throw new Error(`BAILOUT 条数 ${bailoutList.length} != ${EXPECT_BAILOUT}`);
if (abortList.length !== EXPECT_ABORT)
  throw new Error(`ABORT 条数 ${abortList.length} != ${EXPECT_ABORT}`);
if (terminalBailout.length !== 4) throw new Error("TERMINAL_BAILOUT 条数 != 4");
if (terminalMaglev.length !== 2) throw new Error("TERMINAL_MAGLEV 条数 != 2");
if (terminalTurbofan.length !== 2) throw new Error("TERMINAL_TURBOFAN 条数 != 2");

// kName → Rust PascalCase 变体
function variantOf(kname) {
  let n = kname.replace(/^k/, "");
  if (n === "") n = kname;
  // camel 驼峰切分, 数字独立成段
  const parts = n.replace(/([a-z0-9])([A-Z])/g, "$1 $2").replace(/([A-Z]+)([A-Z][a-z])/g, "$1 $2").split(/[\s_]+/);
  return parts.filter(Boolean).map((w) => w.charAt(0).toUpperCase() + w.slice(1)).join("");
}

function snakeUpper(kname) {
  const v = variantOf(kname);
  const s = v.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
  return /^[0-9]/.test(s) ? `K${s}` : s;
}

// 消息字面量转 Rust (转义 " 与 \)
function rustStr(s) {
  return `"${s.replace(/\\/g, "\\\\").replace(/"/g, '\\"')}"`;
}

const abortVariants = abortList.map((e) => ({ ...e, v: variantOf(e.name) }));
// ABORT 内 kNoReason/kLastReason 与消息查重: kLastReason 是哨兵 ("")
const abortHasLast = abortVariants[abortVariants.length - 1].name === "kLastReason";

const bailoutEntries = bailoutList.map((e) => ({ ...e, v: variantOf(e.name) }));
const LAST = { v: "LastErrorMessage", name: "kLastErrorMessage" };

let rust = banner(
  "src/codegen/bailout-reason.h.js",
  "src/codegen/bailout-reason.{h,cc}",
  "BailoutReason/AbortReason 枚举与静态消息表 (X-macro 机械展开)",
);

rust += `
// 展开自 ABORT_MESSAGES_LIST / BAILOUT_MESSAGES_LIST (+3 个 TERMINAL 子宏);
// IF_TARGET_ARCH_X64 条目按 x64 目标展开, arm64 目标登记跳过。

/// BailoutReason (原版 enum class BailoutReason : uint8_t, 共 ${bailoutEntries.length} + 1 哨兵)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum BailoutReason {
`;

for (const e of bailoutEntries) rust += `  /// ${e.name}: ${rustStr(e.msg)}\n  ${e.v},\n`;
rust += `  /// kLastErrorMessage (列表外哨兵, 非法值上界)\n  ${LAST.v},\n`;

rust += `}

impl BailoutReason {
  /// 合法值总数 (不含哨兵)
  pub const COUNT: usize = ${bailoutEntries.length};
  /// 哨兵 = 上界
  pub const LAST_ERROR_MESSAGE: BailoutReason = BailoutReason::${LAST.v};

  /// 静态消息文本 (原版 GetBailoutReason; 沙箱内不可信输入需先校验 < LAST)
  #[inline]
  #[must_use]
  pub const fn message(self) -> &'static str {
    match self {
`;

for (const e of bailoutEntries) rust += `      Self::${e.v} => ${rustStr(e.msg)},\n`;
rust += `      Self::${LAST.v} => "<invalid bailout reason>",\n`;

rust += `    }
  }
}

/// AbortReason (原版 enum class AbortReason : uint8_t, 共 ${abortVariants.length} 条, 末位 kLastReason)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum AbortReason {
`;

for (const e of abortVariants) {
  rust += `  /// ${e.name}${e.note ? ` (${e.note})` : ""}: ${rustStr(e.msg)}\n  ${e.v},\n`;
}
rust += `  /// kLastErrorMessage (列表外哨兵, 非法值上界)\n  ${LAST.v},\n`;

rust += `}

impl AbortReason {
  /// 合法值总数 (不含哨兵)
  pub const COUNT: usize = ${abortVariants.length};
  /// 哨兵 = 上界
  pub const LAST_ERROR_MESSAGE: AbortReason = AbortReason::${LAST.v};

  /// 静态消息文本 (原版 GetAbortReason)
  #[inline]
  #[must_use]
  pub const fn message(self) -> &'static str {
    match self {
`;
for (const e of abortVariants) rust += `      Self::${e.v} => ${rustStr(e.msg)},\n`;
rust += `      Self::${LAST.v} => "<invalid abort reason>",\n`;
rust += `    }
  }

  /// 原版 IsValidAbortReason: id 落在 [kNoReason, kLastErrorMessage)
  #[inline]
  #[must_use]
  pub const fn is_valid_id(reason_id: i32) -> bool {
    reason_id >= AbortReason::NoReason as i32 && reason_id < AbortReason::${LAST.v} as i32
  }

  /// 从数值反查 (越界返回 None)
  #[inline]
  #[must_use]
  pub const fn from_repr(val: u8) -> Option<Self> {
    if (val as usize) < Self::COUNT + 1 { Some(unsafe { std::mem::transmute::<u8, Self>(val) }) } else { None }
  }
}
`;

// 终止原因集合 → const 表 (机械对应原版 IsTerminalBailoutReason 三个 switch)
function terminalTable(list, maglev, turbofan) {
  const seen = new Set();
  const rows = [];
  for (const e of [...list, ...maglev, ...turbofan]) {
    if (seen.has(e.name)) continue;
    seen.add(e.name);
    rows.push(`  BailoutReason::${variantOf(e.name)},`);
  }
  return rows;
}

const termRows = terminalTable(terminalBailout, [], []);
const termMaglevRows = terminalTable(terminalBailout, terminalMaglev, []);
const termTurbofanRows = terminalTable(terminalBailout, terminalMaglev, terminalTurbofan);

rust += `
/// 终止型 Bailout 原因 (原版 IsTerminalBailoutReason: TERMINAL_BAILOUT_MESSAGES_LIST)
pub const TERMINAL_BAILOUT_REASONS: &[BailoutReason] = &[
${termRows.join("\n")}
];

/// Maglev 终止型 (原版 IsTerminalBailoutReasonForMaglev: TERMINAL_BAILOUT + TERMINAL_MAGLEV)
pub const TERMINAL_BAILOUT_REASONS_FOR_MAGLEV: &[BailoutReason] = &[
${termMaglevRows.join("\n")}
];

/// Turbofan 终止型 (原版 IsTerminalBailoutReasonForTurbofan: 同 Maglev 集合)
pub const TERMINAL_BAILOUT_REASONS_FOR_TURBOFAN: &[BailoutReason] = &[
${termTurbofanRows.join("\n")}
];

/// 原版 IsTerminalBailoutReason
#[inline]
#[must_use]
pub fn is_terminal_bailout_reason(reason: BailoutReason) -> bool {
  TERMINAL_BAILOUT_REASONS.contains(&reason)
}

/// 原版 IsTerminalBailoutReasonForMaglev
#[inline]
#[must_use]
pub fn is_terminal_bailout_reason_for_maglev(reason: BailoutReason) -> bool {
  TERMINAL_BAILOUT_REASONS_FOR_MAGLEV.contains(&reason)
}

/// 原版 IsTerminalBailoutReasonForTurbofan
#[inline]
#[must_use]
pub fn is_terminal_bailout_reason_for_turbofan(reason: BailoutReason) -> bool {
  TERMINAL_BAILOUT_REASONS_FOR_TURBOFAN.contains(&reason)
}

/// 原版 GetBailoutReason (自由函数形态)
#[inline]
#[must_use]
pub const fn get_bailout_reason(reason: BailoutReason) -> &'static str {
  reason.message()
}

/// 原版 GetAbortReason (自由函数形态)
#[inline]
#[must_use]
pub const fn get_abort_reason(reason: AbortReason) -> &'static str {
  reason.message()
}

// —— 提取不变式断言 (对数守卫) ——
const _: () = {
  assert!(BailoutReason::COUNT == ${bailoutEntries.length});
  assert!(AbortReason::COUNT == ${abortVariants.length});
  assert!(AbortReason::COUNT == ${EXPECT_ABORT});
  assert!(BailoutReason::NoReason as u8 == 0);
  assert!(AbortReason::NoReason as u8 == 0);
  assert!((AbortReason::${LAST.v} as u8) == AbortReason::COUNT as u8);
  assert!((BailoutReason::${LAST.v} as u8) == BailoutReason::COUNT as u8);
  assert!(AbortReason::LastReason.message().is_empty());
};
`;

// 附加对数: 首尾条目须与原版一致
if (bailoutEntries[0].name !== "kNoReason") throw new Error("BAILOUT 首条非 kNoReason");
if (abortVariants[0].name !== "kNoReason") throw new Error("ABORT 首条非 kNoReason");
if (!abortHasLast) throw new Error("ABORT 末条非 kLastReason");

const p = writeSrc("v8-codegen-core", "bailout.rs", rust);
console.log(`bailout-reason: bailout ${bailoutEntries.length} 条 / abort ${abortVariants.length} 条 / terminal ${termRows.length}/${termMaglevRows.length} 条 → ${p}`);
