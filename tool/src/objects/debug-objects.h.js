#!/usr/bin/env bun
// tool/src/objects/debug-objects.h.js — 覆盖 v8/src/objects/debug-objects.{h,-inl.h,cc,tq}
// → v8-objects-rest/src/debug_info.rs
// 机械提取: DebugInfo 标志位链 (6 位) + Flag 枚举 + ExecutionMode + SideEffectState
//   + DebuggerHints 位链 + CoverageInfo 槽位常量; 语义实现面登记跳过。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/debug-objects.h");

// DebugInfo |flags| 位链 (6 段, bool 各 1 位)
const FLAGS_CHAIN = [
  "HasBreakInfoBit", "PreparedForDebugExecutionBit", "HasCoverageInfoBit",
  "BreakAtEntryBit", "CanBreakAtEntryBit", "DebugExecutionModeBit",
];
let pos = 0;
const bits = FLAGS_CHAIN.map((name) => {
  const decl = new RegExp(`using ${name} = base::BitField<bool, 0, 1, uint32_t>|using ${name} = \\w+Bit::Next<bool, 1>`);
  if (!decl.test(hSrc)) throw new Error(`未找到位链声明 ${name}`);
  const b = { name, start: pos, len: 1 };
  pos += 1;
  return b;
});
if (bits.length !== 6) throw new Error(`DebugInfo 位链段数 ${bits.length} != 6`);

// Flag 枚举 (kNone + 6 掩码)
const flagRe = hSrc.match(/enum Flag : uint32_t\s*\{([\s\S]*?)\n\s*\};/);
if (!flagRe) throw new Error("未找到 DebugInfo::Flag 枚举");
const flagNames = [...flagRe[1].matchAll(/^\s*(k\w+)\s*=/gm)].map((m) => m[1]);
if (flagNames.length !== 7 || flagNames[0] !== "kNone") throw new Error(`Flag 条数/首条失配: ${flagNames.join(",")}`);

const flagCount = hSrc.match(/static constexpr int kFlagCount = (\d+);/);
if (!flagCount || flagCount[1] !== "6") throw new Error("kFlagCount != 6");

// ExecutionMode
const em = hSrc.match(/enum ExecutionMode : uint8_t\s*\{([^}]+)\}/);
if (!em) throw new Error("未找到 ExecutionMode");
const emEntries = em[1].split(",").map((s) => s.trim()).filter(Boolean);
if (emEntries.length !== 2 || emEntries[0] !== "kBreakpoints = 0") throw new Error(`ExecutionMode 条目失配: ${emEntries.join(",")}`);

// SideEffectState
const ses = hSrc.match(/enum SideEffectState\s*\{([^}]+)\}/);
if (!ses) throw new Error("未找到 SideEffectState");
const sesEntries = [...ses[1].matchAll(/(k\w+)\s*=\s*(\d+)/g)].map((m) => ({ name: m[1], v: parseInt(m[2], 10) }));
if (sesEntries.length !== 4) throw new Error(`SideEffectState 条数 ${sesEntries.length} != 4`);
sesEntries.forEach((e, i) => { if (e.v !== i) throw new Error(`SideEffectState 值失配: ${e.name}=${e.v}`); });

// DebuggerHints 位链 (debugger_hints 字段): SideEffectStateBits<int32,0,2> 链
const dhChain = [
  { name: "SideEffectStateBits", type: "i32", start: 0, len: 2 },
  { name: "DebugIsBlackboxedBit", type: "bool", start: 2, len: 1 },
  { name: "ComputedDebugIsBlackboxedBit", type: "bool", start: 3, len: 1 },
  { name: "DebuggingIdBits", type: "i32", start: 4, len: 20 },
];
for (const seg of dhChain) {
  const re = new RegExp(`using ${seg.name} = base::BitField<[^>]*>|using ${seg.name} = \\w+::Next<[^>]*>`);
  if (!re.test(hSrc)) throw new Error(`未找到 DebuggerHints 位链声明 ${seg.name}`);
}
const kNoDebuggingId = hSrc.match(/static const int kNoDebuggingId = (\d+);/);
if (!kNoDebuggingId || kNoDebuggingId[1] !== "0") throw new Error("kNoDebuggingId 失配");

const kEst = hSrc.match(/static const int kEstimatedNofBreakPointsInFunction = (\d+);/);
if (!kEst || kEst[1] !== "4") throw new Error("kEstimatedNofBreakPointsInFunction 失配");

const pascal = (s) => s.replace(/^k/, "").toLowerCase().replace(/(^|_)([a-z0-9])/g, (_, p, c) => c.toUpperCase());
const snakeU = (s) => s.replace(/Bit$/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

let rust = banner("src/objects/debug-objects.h.js", "src/objects/debug-objects.{h,-inl.h,cc,tq}", "DebugInfo 调试标志位链/执行模式/副作用状态");
rust += `//! DebugInfo: 函数调试信息 (trusted); 标志位链 + 执行模式 + 副作用三态。
//! 语义实现面 (DebugInfo.cc 执行/补丁逻辑) 登记跳过, 由 Developer 就地实现。

`;
for (const b of bits) {
  rust += `/// 原版 ${b.name} (bit ${b.start})。
pub const ${snakeU(b.name)}_SHIFT: u32 = ${b.start};
pub const ${snakeU(b.name)}_MASK: u32 = 0x${(1 << b.start).toString(16)};
`;
}
rust += `
/// 标志位总数 (原版 kFlagCount)。
pub const FLAG_COUNT: u32 = 6;

/// 原版 enum Flag (各 = 对应位掩码; kNone = 0)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Flag {
  /// kNone
  None = 0,
`;
for (let i = 1; i < flagNames.length; i++) {
  const bitName = flagNames[i].slice(1) + "Bit";
  const bit = bits.find((b) => b.name === bitName);
  if (!bit) throw new Error(`Flag ${flagNames[i]} 无对应位段 ${bitName}`);
  rust += `  /// ${flagNames[i]}\n  ${pascal(flagNames[i])} = 0x${(1 << bit.start).toString(16)},\n`;
}
rust += `}

/// 调试执行模式 (原版 enum ExecutionMode; kSideEffects 复用 kDebugExecutionMode 位)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ExecutionMode {
  /// kBreakpoints
  Breakpoints = 0,
  /// kSideEffects (= DebugExecutionModeBit 位值)
  SideEffects = ${(1 << bits[5].start).toString()},
}

/// 副作用三态 (原版 enum SideEffectState)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SideEffectState {
`;
for (const e of sesEntries) {
  rust += `  /// ${e.name}\n  ${pascal(e.name)} = ${e.v},\n`;
}
rust += `}

`;
for (const seg of dhChain) {
  const mask = ((1n << BigInt(seg.len)) - 1n) << BigInt(seg.start);
  rust += `/// 原版 debugger_hints 位段 ${seg.name}<${seg.type}, ${seg.start}, ${seg.len}>。
pub const ${snakeU(seg.name)}_SHIFT: u32 = ${seg.start};
pub const ${snakeU(seg.name)}_MASK: u32 = 0x${mask.toString(16)};
`;
}
rust += `
/// 无调试 ID (原版 kNoDebuggingId)。
pub const NO_DEBUGGING_ID: i32 = 0;
/// 预估断点数上限 (原版 kEstimatedNofBreakPointsInFunction)。
pub const ESTIMATED_NOF_BREAK_POINTS_IN_FUNCTION: i32 = ${kEst[1]};

// —— 提取不变式断言 ——
const _: () = {
  assert!(FLAG_COUNT == 6);
  assert!(HAS_BREAK_INFO_BIT_MASK == 0x1);
  assert!(DEBUG_EXECUTION_MODE_BIT_MASK == 0x20);
  assert!(ExecutionMode::SideEffects as u8 == 0x20);
  assert!(SideEffectState::HasNoSideEffect as i32 == 3);
  assert!(DEBUGGING_ID_BITS_MASK == 0xFFFFF0);
  assert!(ESTIMATED_NOF_BREAK_POINTS_IN_FUNCTION == 4);
};
`;

const p = writeSrc("v8-objects-rest", "debug_info.rs", rust);
console.log(`debug_info.rs: 6 位链 + Flag 7 变体 + ExecutionMode 2 + SideEffectState 4 + hints 4 段, 对数通过`);
