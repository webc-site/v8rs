#!/usr/bin/env bun
// tool/src/heap/sweeper.h.js + 相邻小枚举打包:
// sweeper.h (FreeSpaceTreatmentMode/SweepingMode/SweepingScope),
// incremental-marking.h (StepOrigin), safepoint.h (SafepointKind/IncludeMainThread)。
// 注意: 脚本名镜像 sweeper.h 主体, 其余两个小族在同产物中登记来源。

import { readV8, writeSrc, banner } from "../../lib.js";

const sweepSrc = readV8("src/heap/sweeper.h");
const incSrc = readV8("src/heap/incremental-marking.h");
const safeSrc = readV8("src/heap/safepoint.h");
// 族内 .cc 实读对数
const sweepCcSrc = readV8("src/heap/sweeper.cc");
if (!sweepCcSrc.includes("Sweeper::LocalSweeper::ParallelSweepSpace(")) throw new Error("sweeper.cc 锚点缺失");

function enumBody(src, name) {
  const m = src.match(new RegExp(`enum(?: class)? ${name}[^\\{]*\\{([^}]*)\\};`, "s"));
  if (!m) throw new Error(`未找到枚举 ${name}`);
  return m[1]
    .replace(/\/\/[^\n]*/g, "")
    .split(",")
    .map((s) => s.trim())
    .filter(Boolean);
}

const freeMode = enumBody(sweepSrc, "FreeSpaceTreatmentMode");
if (freeMode.join() !== "kIgnoreFreeSpace,kZapFreeSpace") throw new Error("FreeSpaceTreatmentMode 漂移");
const sweepingMode = enumBody(sweepSrc, "SweepingMode");
if (sweepingMode.join() !== "kEagerDuringGC,kLazyOrConcurrent") throw new Error("SweepingMode 漂移");
const sweepingScope = enumBody(sweepSrc, "SweepingScope");
if (sweepingScope.join() !== "kMinor,kMajor") throw new Error("SweepingScope 漂移");

const stepOrigin = enumBody(incSrc, "StepOrigin");
if (stepOrigin.join() !== "kV8,kTask") throw new Error("StepOrigin 漂移");

const includeMain = enumBody(safeSrc, "IncludeMainThread");
if (includeMain.join() !== "kYes,kNo") throw new Error("IncludeMainThread 漂移");
const safepointKind = enumBody(safeSrc, "SafepointKind");
if (safepointKind.join() !== "kIsolate,kGlobal") throw new Error("SafepointKind 漂移");

console.log("sweeper 3 + incremental-marking 1 + safepoint 2 枚举");

let rust = banner(
  "src/heap/sweeper.h.js",
  "src/heap/sweeper.h + incremental-marking.h + safepoint.h",
  "清扫/增量标记/安全点枚举",
);

const snake = (s) => s.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

function emitEnum(rustName, doc, entries, repr = "u8") {
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

rust += emitEnum("FreeSpaceTreatmentMode", "空闲块处理方式 (对应 enum class FreeSpaceTreatmentMode)", freeMode);
rust += emitEnum("SweepingMode", "清扫时机 (对应 Sweeper::SweepingMode)", sweepingMode);
rust += emitEnum("SweepingScope", "清扫范围 (对应 Sweeper::SweepingScope)", sweepingScope);
rust += emitEnum(
  "StepOrigin",
  "增量标记步进来源 (对应 enum class StepOrigin; kV8=任务排期, kTask=调用方随即完成 GC)",
  stepOrigin,
);
rust += emitEnum("IncludeMainThread", "是否含主线程 (对应 SafepointScope::IncludeMainThread)", includeMain);
rust += emitEnum("SafepointKind", "安全点种类 (对应 enum class SafepointKind)", safepointKind);

rust += `
// —— 提取不变式断言 ——
const _: () = {
  assert!(FreeSpaceTreatmentMode::ZAP_FREE_SPACE == 1);
  assert!(SweepingMode::LAZY_OR_CONCURRENT == 1);
  assert!(SweepingScope::MAJOR == 1);
  assert!(StepOrigin::TASK == 1);
  assert!(IncludeMainThread::NO == 1);
  assert!(SafepointKind::GLOBAL == 1);
};
`;
;

const p = writeSrc("v8-heap-gc", "sweeper_enums.rs", rust);
console.log(`written ${p}`);
