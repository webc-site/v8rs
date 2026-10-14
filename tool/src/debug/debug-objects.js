#!/usr/bin/env bun
// tool/src/debug/debug-objects.js — 覆盖 v8/src/objects/debug-objects.{-inl.h,cc,tq}
// 组→文件映射 (本脚本管辖): 席位表 3 文件 (.h 位链数值归既有 tool/src/objects/debug-objects.h.js)
// 产物: v8-debug/src/debug_objects.rs
// 机械提取: tq 8 个 extern class 布局 + 3 个 bitfield struct + CoverageInfoSlot
//   + -inl.h 访问器面 + .cc (debug-objects.cc) 方法签名骨架 (todo! 就地修订)

import { readV8, writeSrc, banner } from "../../lib.js";

const tq = readV8("src/objects/debug-objects.tq");
const inl = readV8("src/objects/debug-objects-inl.h");
const cc = readV8("src/objects/debug-objects.cc");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[debug-objects.js] ${m}`); };

// ── 1. tq extern class 布局 (8 个) ────────────────────────────────────────
function tqClass(name, base) {
  const m = tq.match(new RegExp(`extern class ${name} extends (\\w+)\\s*\\{([\\s\\S]*?)\\n\\}`));
  must(m, `tq 缺 extern class ${name}`);
  must(m[1] === base, `${name} 基类 ${m[1]} != ${base}`);
  // 字段行: 过滤注释, 允许 0..n 个 @cpp 注解与 const 前缀, 只留 "name: type;"
  const fields = [...m[2].matchAll(/^  ((?:@cpp\w+ )*)(?:const )?(\w+)(\[\w+\])?: ([^;]+);$/gm)]
    .map((x) => [x[2] + (x[3] || ""), x[4].trim()]);
  return fields;
}
const layouts = {
  BreakPoint: tqClass("BreakPoint", "Struct"),
  BreakPointInfo: tqClass("BreakPointInfo", "Struct"),
  DebugInfo: tqClass("DebugInfo", "ExposedTrustedObject"),
  CoverageInfo: tqClass("CoverageInfo", "HeapObject"),
  StackFrameInfo: tqClass("StackFrameInfo", "Struct"),
  StackTraceInfo: tqClass("StackTraceInfo", "Struct"),
  ErrorStackData: tqClass("ErrorStackData", "Struct"),
  DebugScriptScopeInfo: tqClass("DebugScriptScopeInfo", "Struct"),
};
const expectCounts = { BreakPoint: 2, BreakPointInfo: 2, DebugInfo: 7, CoverageInfo: 2, StackFrameInfo: 4, StackTraceInfo: 2, ErrorStackData: 2, DebugScriptScopeInfo: 2 };
for (const [name, fields] of Object.entries(layouts)) {
  must(fields.length === expectCounts[name], `tq ${name} 字段数 ${fields.length} != ${expectCounts[name]}`);
}
must(layouts.BreakPoint.map((f) => f[0]).join(",") === "id,condition", "BreakPoint 布局漂移");
must(layouts.BreakPointInfo.map((f) => f[0]).join(",") === "source_position,break_points", "BreakPointInfo 布局漂移");
must(layouts.DebugInfo.map((f) => f[0]).join(",") === "shared,debugger_hints,break_points,flags,coverage_info,original_bytecode_array,debug_bytecode_array", "DebugInfo 布局漂移");
must(layouts.DebugInfo[3][1] === "SmiTagged<DebugInfoFlags>", "DebugInfo flags 类型漂移");
must(layouts.CoverageInfo.map((f) => f[0]).join(",") === "slot_count,slots[slot_count]", "CoverageInfo 布局漂移");
must(layouts.CoverageInfo[1][1] === "CoverageInfoSlot", "CoverageInfo 槽类型漂移");
must(layouts.StackFrameInfo.map((f) => f[0]).join(",") === "shared_or_script,function_name,flags,bytecode_offset_or_source_position", "StackFrameInfo 布局漂移");
must(layouts.ErrorStackData.map((f) => f[0]).join(",") === "raw_data_for_call_site_infos_or_formatted_stack,stack_trace", "ErrorStackData 布局漂移");
must(layouts.DebugScriptScopeInfo.map((f) => f[0]).join(",") === "numeric_data,string_table", "DebugScriptScopeInfo 布局漂移");

// ── 2. tq bitfield struct (3 个) ─────────────────────────────────────────
function tqBits(name, expectTotal) {
  const m = tq.match(new RegExp(`bitfield struct ${name} extends uint31 \\{([\\s\\S]*?)\\}`));
  must(m, `tq 缺 bitfield struct ${name}`);
  const fields = [...m[1].matchAll(/^  (\w+): (\w+): (\d+) bit;/gm)].map((x) => [x[1], x[2], +x[3]]);
  const total = fields.reduce((a, f) => a + f[2], 0);
  must(total === expectTotal, `${name} 位域总宽 ${total} != ${expectTotal}`);
  return fields;
}
const df = tqBits("DebugInfoFlags", 6);
must(df.map((f) => f[0]).join(",") === "has_break_info,prepared_for_debug_execution,has_coverage_info,break_at_entry,can_break_at_entry,debug_execution_mode", "DebugInfoFlags 序漂移");
must(df.every((f) => f[1] === "bool" && f[2] === 1), "DebugInfoFlags 应全为 bool 1 bit");
const dh = tqBits("DebuggerHints", 24);
must(dh.map((f) => f[0]).join(",") === "side_effect_state,debug_is_blackboxed,computed_debug_is_blackboxed,debugging_id", "DebuggerHints 序漂移");
must(dh[0][2] === 2 && dh[1][2] === 1 && dh[2][2] === 1 && dh[3][2] === 20, "DebuggerHints 位宽漂移 (2+1+1+20)");
const sf = tqBits("StackFrameInfoFlags", 1);
must(sf[0][0] === "is_constructor" && sf[0][2] === 1, "StackFrameInfoFlags 漂移");

// ── 3. tq CoverageInfoSlot struct (4 字段) ───────────────────────────────
const slot = tq.match(/struct CoverageInfoSlot \{([\s\S]*?)\}/);
must(slot, "tq 缺 CoverageInfoSlot");
const slotFields = [...slot[1].matchAll(/^  (\w+): (\w+);\s*(?:\/\/.*)?$/gm)].map((m) => [m[1], m[2]]);
must(slotFields.length === 4, `CoverageInfoSlot 字段数 ${slotFields.length} != 4`);
must(slotFields.map((f) => f[0]).join(",") === "start_source_position,end_source_position,block_count,padding", "CoverageInfoSlot 布局漂移");
must(slotFields.every((f) => f[1] === "int32"), "CoverageInfoSlot 应全为 int32");

// ── 4. -inl.h 访问器面锚 ─────────────────────────────────────────────────
for (const s of [
  "BreakPointInfo::break_points() const", "BreakPoint::id() const", "BreakPoint::condition() const",
  "DebugInfo::shared() const", "DebugInfo::flags(RelaxedLoadTag) const", "DebugInfo::coverage_info() const",
  "DebugInfo::original_bytecode_array() const", "DebugInfo::debug_bytecode_array() const",
  "StackFrameInfo::shared_or_script()", "StackTraceInfo::length() const", "StackTraceInfo::get(int index) const",
  "ErrorStackData::HasFormattedStack() const", "ErrorStackData::HasRawDataForCallSiteInfos() const",
  "DebugScriptScopeInfo::numeric_data() const", "DebugScriptScopeInfo::string_table() const",
  "CoverageInfo::slot_count() const", "CoverageInfo::AllocatedSize() const",
]) must(inl.includes(s), `-inl.h 缺 ${s}`);
const bitAccessors = [...inl.matchAll(/BIT_FIELD_ACCESSORS\(DebugInfo, debugger_hints, (\w+),/g)].map((m) => m[1]);
must(bitAccessors.join(",") === "side_effect_state,debug_is_blackboxed,computed_debug_is_blackboxed,debugging_id", "BIT_FIELD_ACCESSORS 序漂移");

// ── 5. .cc 方法签名骨架 (源码序提取) ──────────────────────────────────────
const ccSigs = [...cc.matchAll(/^(?:bool|void|int|Tagged<\w+>|MaybeDirectHandle<\w+>)\s+(\w+)::(\w+)\(/gm)]
  .map((m) => `${m[1]}::${m[2]}`);
must(ccSigs.length >= 18, `.cc 方法数 ${ccSigs.length} < 18`);
for (const s of ["DebugInfo::IsEmpty", "DebugInfo::HasBreakPoint", "DebugInfo::SetBreakPoint", "DebugInfo::ClearBreakPoint",
  "BreakPointInfo::SetBreakPoint", "BreakPointInfo::GetBreakPointById", "CoverageInfo::ResetBlockCount", "StackFrameInfo::GetSourcePosition"]) {
  must(ccSigs.includes(s), `.cc 缺方法 ${s}`);
}
must(new Set(ccSigs).size === ccSigs.length, ".cc 方法签名重复");

let rust = banner("src/debug/debug-objects.js", "src/objects/debug-objects.{-inl.h,cc,tq}",
  "席位表差集: tq 布局/bitfield + inl 访问器 + cc 签名骨架 (位链数值归既有 debug-objects.h.js)");
rust += `//! 调试对象族: BreakPoint/BreakPointInfo/DebugInfo/CoverageInfo/StackFrameInfo/
//! StackTraceInfo/ErrorStackData/DebugScriptScopeInfo (debug 域)。
//!
//! 位链数值面 (掩码/偏移) 归既有 debug-objects.h.js 产物; 本文件登记 tq 布局与 .cc 方法骨架。

use std::mem::size_of;

// ── tq extern class 布局表 (源码序) ──
`;
for (const [name, fields] of Object.entries(layouts)) {
  rust += `pub const ${name.toUpperCase()}_FIELDS: [(&str, &str); ${fields.length}] = [\n`;
  for (const f of fields) rust += `  ("${f[0]}", "${f[1]}"),\n`;
  rust += `];\n`;
}
rust += `
// ── tq bitfield struct (位段名/类型/位宽, 源码序) ──
pub const DEBUG_INFO_FLAGS_BITS: [(&str, &str, u32); ${df.length}] = [
`;
for (const f of df) rust += `  ("${f[0]}", "${f[1]}", ${f[2]}),\n`;
rust += `];
pub const DEBUGGER_HINTS_BITS: [(&str, &str, u32); ${dh.length}] = [
`;
for (const f of dh) rust += `  ("${f[0]}", "${f[1]}", ${f[2]}),\n`;
rust += `];
pub const STACK_FRAME_INFO_FLAGS_BITS: [(&str, &str, u32); ${sf.length}] = [
`;
for (const f of sf) rust += `  ("${f[0]}", "${f[1]}", ${f[2]}),\n`;
rust += `];

// ── CoverageInfoSlot (tq @export struct, 4 × int32) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct CoverageInfoSlot {
  /// start_source_position
  pub start_source_position: i32,
  /// end_source_position
  pub end_source_position: i32,
  /// block_count
  pub block_count: i32,
  /// padding (凑齐 4 槽)
  pub padding: i32,
}

// ── .cc 方法签名骨架 (${ccSigs.length} 个, 源码序; 语义就地修订) ──
`;
for (const s of ccSigs) {
  const [cls, meth] = s.split("::");
  const fnName = `${cls.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}_${meth.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}`;
  rust += `/// 原版 ${s} (debug-objects.cc)。
pub fn ${fnName}() {
  todo!("就地修订: debug-objects.cc ${s}")
}

`;
}
rust += `/// 编译期字符串相等 (str 的 const PartialEq 尚未稳定, 手写逐字节比较)
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(BREAKPOINT_FIELDS.len() == 2);
  assert!(DEBUGINFO_FIELDS.len() == 7);
  assert!(str_eq(DEBUGINFO_FIELDS[6].0, "debug_bytecode_array"));
  assert!(str_eq(COVERAGEINFO_FIELDS[1].0, "slots[slot_count]"));
  assert!(DEBUG_INFO_FLAGS_BITS.len() == 6);
  assert!(DEBUGGER_HINTS_BITS.len() == 4);
  assert!(DEBUGGER_HINTS_BITS[3].2 == 20);
  assert!(STACK_FRAME_INFO_FLAGS_BITS.len() == 1);
  assert!(size_of::<CoverageInfoSlot>() == 16);
};
`;

const p = writeSrc("v8-debug", "debug_objects.rs", rust);
console.log(`debug-objects.js: ${p} — 布局 8 类 (${Object.values(layouts).map((f) => f.length).join("+")} 字段)、bitfield 3 (6/24/1 bit)、CoverageInfoSlot 4、.cc 方法 ${ccSigs.length}, 断言 ${checks} 项全过`);
