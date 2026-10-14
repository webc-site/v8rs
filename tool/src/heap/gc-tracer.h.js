#!/usr/bin/env bun
// tool/src/heap/gc-tracer.h.js
// 机械提取 v8/src/heap/gc-tracer.{h,cc}:
// GCTracer::ScopeId (TRACER_SCOPES + TRACER_BACKGROUND_SCOPES 全展开, 宏源 src/init/heap-symbols.h),
// YoungGenerationSpeedMode / Event::Type / Event::State / Mode / MarkingType。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/gc-tracer.h");
const symbolsSrc = readV8("src/init/heap-symbols.h");

// —— 宏体提取 (合并续行, 保留 F(...) 序) ——
function macroBody(src, name) {
  const lines = src.split("\n");
  for (let i = 0; i < lines.length; i++) {
    const m = lines[i].match(new RegExp(`^#define\\s+${name}\\(([^)]*)\\)\\s*(.*)$`));
    if (!m) continue;
    let body = m[2];
    while (body.endsWith("\\")) body = body.slice(0, -1) + " " + (lines[++i] ?? "").trim();
    return body.replace(/\/\*[\s\S]*?\*\//g, " ");
  }
  throw new Error(`未找到宏 ${name}`);
}

// 把组合宏展开为 F 条目序列 (叶子宏引用 + 直接 F 条目)
function expandScopes(compositeName, depth = 0) {
  if (depth > 6) throw new Error(`宏嵌套过深: ${compositeName}`);
  const body = macroBody(symbolsSrc, compositeName);
  const out = [];
  const re = /([A-Z_0-9]+)\s*\(\s*F\s*\)|F\(\s*(\w+)\s*\)/g;
  let m;
  let found = false;
  while ((m = re.exec(body)) !== null) {
    found = true;
    if (m[2] !== undefined) {
      out.push(m[2]);
    } else {
      out.push(...expandScopes(m[1], depth + 1));
    }
  }
  if (!found) throw new Error(`${compositeName} 无 F 条目`);
  return out;
}

// ScopeId 顺序: TRACER_SCOPES(DEFINE_SCOPE) + TRACER_BACKGROUND_SCOPES(DEFINE_SCOPE)
const scopes = [...expandScopes("TRACER_SCOPES"), ...expandScopes("TRACER_BACKGROUND_SCOPES")];
{
  const s = new Set(scopes);
  if (s.size !== scopes.length) throw new Error("ScopeId 重名");
  if (scopes[0] !== "MC_INCREMENTAL") {
    throw new Error(`ScopeId 首条漂移: ${scopes[0]}`);
  }
  const bgStart = scopes.indexOf("BACKGROUND_YOUNG_ARRAY_BUFFER_SWEEP");
  const bgEnd = scopes.indexOf("SCAVENGER_BACKGROUND_TRACED_HANDLES_COMPUTE_WEAKNESS_PARALLEL");
  if (bgStart < 0 || bgEnd < 0 || bgEnd !== scopes.length - 1) {
    throw new Error("背景段锚点漂移 (LAST_BACKGROUND_SCOPE 应为末条)");
  }
  const incFirst = scopes.indexOf("MC_INCREMENTAL");
  if (incFirst < 0) throw new Error("FIRST_INCREMENTAL_SCOPE=MC_INCREMENTAL 缺失");
  const mcClear = scopes.indexOf("MC_CLEAR");
  const mcSweep = scopes.indexOf("MC_SWEEP");
  if (mcClear < 0 || mcSweep < 0) throw new Error("TOP_MC 段锚点缺失");
  console.log(
    `ScopeId ${scopes.length} 条 (背景段 ${bgStart}..${bgEnd}, MC_INCREMENTAL@${incFirst}, MC_CLEAR@${mcClear}, MC_SWEEP@${mcSweep})`,
  );
}

// YoungGenerationSpeedMode
const ygs = hSrc.match(/enum YoungGenerationSpeedMode \{([^}]*)\};/);
if (!ygs) throw new Error("未找到 YoungGenerationSpeedMode");
const ygsList = ygs[1].split(",").map((s) => s.trim()).filter(Boolean);
if (ygsList.length !== 2) throw new Error("YoungGenerationSpeedMode 漂移");

// Event::Type
const evType = hSrc.match(/enum class Type \{([^}]*)\};/);
if (!evType) throw new Error("未找到 Event::Type");
const evTypeList = evType[1].split(",").map((s) => s.trim()).filter(Boolean);
if (evTypeList.length !== 6 || evTypeList[0] !== "SCAVENGER = 0" || evTypeList[5] !== "START = 5") {
  throw new Error(`Event::Type 漂移: ${evTypeList}`);
}

// Event::State
const evState = hSrc.match(/enum class State \{([^}]*)\};/);
if (!evState) throw new Error("未找到 Event::State");
const evStateList = evState[1].split(",").map((s) => s.trim()).filter(Boolean);
if (evStateList.join() !== "NOT_RUNNING,MARKING,ATOMIC,SWEEPING") throw new Error("Event::State 漂移");

// Mode / MarkingType
const mode = hSrc.match(/enum class Mode \{([^}]*)\};/);
if (!mode) throw new Error("未找到 Mode");
const modeList = mode[1].split(",").map((s) => s.trim()).filter(Boolean);
if (modeList.join() !== "None,Scavenger,Finalize") throw new Error("Mode 漂移");
const markingType = hSrc.match(/enum class MarkingType \{([^}]*)\};/);
if (!markingType) throw new Error("未找到 MarkingType");
const mtList = markingType[1].split(",").map((s) => s.trim()).filter(Boolean);
if (mtList.join() !== "kAtomic,kIncremental") throw new Error("MarkingType 漂移");

let rust = banner(
  "src/heap/gc-tracer.h.js",
  "src/heap/gc-tracer.{h,cc} + src/init/heap-symbols.h",
  "GC 事件追踪: ScopeId 全表 (TRACER_SCOPES+背景段) 与事件枚举",
);

const snake = (s) => s.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

rust += `
/// 追踪作用域 (对应 GCTracer::ScopeId; TRACER_SCOPES + TRACER_BACKGROUND_SCOPES 展开)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum ScopeId {
`;

scopes.forEach((s, i) => {
  rust += `  /// ${s}\n  ${snake(s).toUpperCase()} = ${i},\n`;
});

rust += `}

impl ScopeId {
  /// 对应 NUMBER_OF_SCOPES
  pub const NUMBER_OF_SCOPES: u16 = ${scopes.length};

  /// FIRST_INCREMENTAL_SCOPE = MC_INCREMENTAL
  pub const FIRST_INCREMENTAL_SCOPE: Self = Self::MC_INCREMENTAL;
  /// LAST_INCREMENTAL_SCOPE = MC_INCREMENTAL_SWEEPING
  pub const LAST_INCREMENTAL_SCOPE: Self = Self::MC_INCREMENTAL_SWEEPING;
  /// FIRST_TOP_MC_SCOPE = MC_CLEAR
  pub const FIRST_TOP_MC_SCOPE: Self = Self::MC_CLEAR;
  /// LAST_TOP_MC_SCOPE = MC_SWEEP
  pub const LAST_TOP_MC_SCOPE: Self = Self::MC_SWEEP;
  /// FIRST_BACKGROUND_SCOPE
  pub const FIRST_BACKGROUND_SCOPE: Self = Self::BACKGROUND_YOUNG_ARRAY_BUFFER_SWEEP;
  /// LAST_BACKGROUND_SCOPE
  pub const LAST_BACKGROUND_SCOPE: Self = Self::SCAVENGER_BACKGROUND_TRACED_HANDLES_COMPUTE_WEAKNESS_PARALLEL;
}

/// 年代回收速度模式 (对应 enum YoungGenerationSpeedMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum YoungGenerationSpeedMode {
`;

ygsList.forEach((k, i) => {
  rust += `  /// ${k}\n  ${snake(k).toUpperCase()} = ${i},\n`;
});

rust += `}

/// GC 事件类型 (对应 GCTracer::Event::Type)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EventType {
`;

for (const t of evTypeList) {
  const [n, v] = t.split("=").map((x) => x.trim());
  rust += `  /// ${n}\n  ${snake(n).toUpperCase()} = ${v},\n`;
}

rust += `}

/// GC 周期状态 (对应 GCTracer::Event::State; 状态图见原版注释)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EventState {
`;

evStateList.forEach((k, i) => {
  rust += `  /// ${k}\n  ${snake(k).toUpperCase()} = ${i},\n`;
});

rust += `}

/// 增量模式 (对应 GCTracer::Mode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Mode {
`;

modeList.forEach((k, i) => {
  rust += `  /// ${k}\n  ${snake(k).toUpperCase()} = ${i},\n`;
});

rust += `}

/// 标记方式 (对应 GCTracer::MarkingType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MarkingType {
`;

mtList.forEach((k, i) => {
  rust += `  /// ${k}\n  ${snake(k).toUpperCase()} = ${i},\n`;
});

rust += `}

// —— 提取不变式断言 ——
const _: () = {
  assert!(ScopeId::NUMBER_OF_SCOPES == ${scopes.length});
  assert!((ScopeId::FIRST_BACKGROUND_SCOPE as u16) == ${scopes.indexOf("BACKGROUND_YOUNG_ARRAY_BUFFER_SWEEP")});
  assert!((ScopeId::LAST_BACKGROUND_SCOPE as u16) == ScopeId::NUMBER_OF_SCOPES - 1);
  assert!(ScopeId::MC_INCREMENTAL_SWEEPING as u16 > ScopeId::MC_INCREMENTAL as u16);
  assert!(YoungGenerationSpeedMode::UP_TO_AND_INCLUDING_ATOMIC_PAUSE == 0);
  assert!(EventType::SCAVENGER == 0);
  assert!(EventType::START == 5);
  assert!(EventState::SWEEPING == 3);
  assert!(Mode::FINALIZE == 2);
  assert!(MarkingType::INCREMENTAL == 1);
};
`;
;

const p = writeSrc("v8-heap-gc", "gc_tracer.rs", rust);
console.log(`written ${p}`);
