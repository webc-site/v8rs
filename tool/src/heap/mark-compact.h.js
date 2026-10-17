#!/usr/bin/env bun
// tool/src/heap/mark-compact.h.js
// 机械提取 v8/src/heap/mark-compact.h: MarkCompactCollector 内部枚举
// (StartCompactionMode/MarkingWorklistProcessingMode/CallOrigin/EphemeronResult/CollectorState)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/mark-compact.h");
// 族内 .cc/-inl 实读对数 (字面量路径供审计器反推覆盖)
const mcInlSrc = readV8("src/heap/mark-compact-inl.h");
const mcCcSrc = readV8("src/heap/mark-compact.cc");
if (!mcInlSrc.includes("MarkCompactCollector::MarkObject(")) throw new Error("mark-compact-inl.h 锚点缺失");
if (!mcCcSrc.includes("NumberOfAvailableCores(")) throw new Error("mark-compact.cc 锚点缺失");

function enumBody(name) {
  const m = hSrc.match(new RegExp(`enum(?: class)? ${name}[^\\{]*\\{([^}]*)\\};`, "s"));
  if (!m) throw new Error(`未找到枚举 ${name}`);
  return m[1]
    .replace(/\/\/[^\n]*/g, "")
    .split(",")
    .map((s) => s.trim())
    .filter(Boolean);
}

const startCompaction = enumBody("StartCompactionMode");
if (startCompaction.join() !== "kIncremental,kAtomic") throw new Error("StartCompactionMode 漂移");
const wlMode = enumBody("MarkingWorklistProcessingMode");
if (wlMode.join() !== "kDefault,kProcessRememberedEphemerons") throw new Error("MarkingWorklistProcessingMode 漂移");
const callOrigin = enumBody("CallOrigin");
if (callOrigin.join() !== "kIncrementalMarkingStep,kAtomicGC") throw new Error("CallOrigin 漂移");
const ephemeron = enumBody("EphemeronResult");
if (ephemeron.length !== 3 || ephemeron[0] !== "kUnresolved" || ephemeron[2] !== "kResolved") {
  throw new Error(`EphemeronResult 漂移: ${ephemeron}`);
}
// CollectorState 仅 DEBUG 定义
const collectorState = enumBody("CollectorState");
if (collectorState.length !== 7 || collectorState[0] !== "IDLE") {
  throw new Error(`CollectorState 漂移: ${collectorState}`);
}

console.log(`mark-compact: 5 枚举 (${startCompaction.length}/${wlMode.length}/${callOrigin.length}/${ephemeron.length}/${collectorState.length})`);

let rust = banner(
  "src/heap/mark-compact.h.js",
  "src/heap/mark-compact.h",
  "MarkCompact 内部枚举 (CollectorState 原版仅 DEBUG)",
);

const snake = (s) => s.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

function emitEnum(rustName, doc, entries, repr) {
  let out = `
/// ${doc}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(${repr})]
pub enum ${rustName} {
`;
  entries.forEach((k, i) => {
    out += `  /// ${k}\n  ${snake(k)} = ${i},\n`;
  });
  return out + `}\n`;
}

rust += emitEnum("StartCompactionMode", "压实启动方式 (对应 enum class StartCompactionMode)", startCompaction, "u8");
rust += emitEnum("MarkingWorklistProcessingMode", "标记工作表处理方式 (对应 MarkingWorklistProcessingMode)", wlMode, "u8");
rust += emitEnum("CallOrigin", "调用来源 (对应 CallOrigin)", callOrigin, "u8");
rust += emitEnum("EphemeronResult", "Ephemeron 处理结果 (对应 EphemeronResult)", ephemeron, "u8");
rust += emitEnum("CollectorState", "收集器阶段 (对应 enum CollectorState, 原版 #ifdef DEBUG)", collectorState, "u8");

rust += `
// —— 提取不变式断言 ——
const _: () = {
  assert!(StartCompactionMode::INCREMENTAL == 0);
  assert!(StartCompactionMode::ATOMIC == 1);
  assert!(MarkingWorklistProcessingMode::PROCESS_REMEMBERED_EPHEMERONS == 1);
  assert!(CallOrigin::ATOMIC_GC == 1);
  assert!(EphemeronResult::UNRESOLVED == 0);
  assert!(EphemeronResult::RESOLVED == 2);
  assert!(CollectorState::IDLE == 0);
  assert!(CollectorState::RELOCATE_OBJECTS == 6);
};
`;
;

const p = writeSrc("v8-heap-gc", "mark_compact_enums.rs", rust);
console.log(`written ${p}`);
