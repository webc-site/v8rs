#!/usr/bin/env bun
// tool/src/heap/mark-compact.cc.js
// 机械提取 v8/src/heap/mark-compact.{cc,inl.h}: 疏散模式枚举与启发式常量 +
// EvacuationMode 纯规则 + 全量方法签名骨架 (todo 就地修订)
// 内部枚举 (StartCompactionMode 等) 已在 mark_compact_enums.rs (mark-compact.h.js)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/mark-compact.h");
const ccSrc = readV8("src/heap/mark-compact.cc");
const inlSrc = readV8("src/heap/mark-compact-inl.h");

// —— EvacuationMode (cc:4700-4704) ——
const em = ccSrc.match(/enum EvacuationMode \{([\s\S]*?)\};/);
if (!em) throw new Error("EvacuationMode 缺失");
const evacModes = em[1].replace(/\/\/[^\n]*/g, "").split(",").map((s) => s.trim()).filter(Boolean);
if (evacModes.join() !== "kObjectsNewToOld,kPageNewToOld,kObjectsOldToOld") {
  throw new Error(`EvacuationMode 序漂移: ${evacModes.join()}`);
}

// EvacuationModeName (cc:4706-4715): case → 字符串
const emnStart = ccSrc.indexOf("static const char* EvacuationModeName(EvacuationMode mode)");
const emn = ccSrc.slice(emnStart, ccSrc.indexOf("UNREACHABLE", emnStart));
const namePairs = [...emn.matchAll(/case (k\w+):\s*return "([^"]+)";/g)];
if (namePairs.length !== 3) throw new Error(`EvacuationModeName 映射数 ${namePairs.length}, 期望 3`);
const modeNames = namePairs.map((m) => m[1]);
if (modeNames.join() !== evacModes.join()) throw new Error(`EvacuationModeName 序漂移: ${modeNames.join()}`);
if (namePairs.map((m) => m[2]).join("|") !== "objects-new-to-old|page-new-to-old|objects-old-to-old") {
  throw new Error(`EvacuationModeName 值漂移: ${namePairs.map((m) => m[2]).join("|")}`);
}

// ComputeEvacuationMode 判定序锚 (cc:4718-4728)
const cemStart = ccSrc.indexOf("static inline EvacuationMode ComputeEvacuationMode(");
const cem = ccSrc.slice(cemStart, ccSrc.indexOf("UNREACHABLE", cemStart) < 0 ? cemStart + 600 : ccSrc.indexOf("}", ccSrc.indexOf("kObjectsOldToOld", cemStart)));
if (!cem.includes("metadata->will_be_promoted()")) throw new Error("ComputeEvacuationMode 首判锚缺失");
if (cem.indexOf("will_be_promoted") > cem.indexOf("InYoungGeneration")) {
  throw new Error("ComputeEvacuationMode 判定序漂移 (原注释要求 will_be_promoted 先判)");
}

// —— 启发式常量 (逐个锚定) ——
function mustConst(src, re, expect, what) {
  const m = src.match(re);
  if (!m) throw new Error(`${what} 缺失`);
  if (m[1].replace(/\s+/g, "") !== expect.replace(/\s+/g, "")) throw new Error(`${what} 漂移: ${m[1]}`);
  return m[1].replace(/\s+/g, " ");
}
mustConst(ccSrc, /const int kTargetFragmentationPercent = ([^;]+);/, "70", "kTargetFragmentationPercent");
mustConst(ccSrc, /const float kTargetMsPerArea = ([^;]+);/, ".5", "kTargetMsPerArea");
const fractions = [...ccSrc.matchAll(/const double kFraction = ([^;]+);/g)].map((m) => m[1].trim());
if (fractions.length !== 2 || fractions[0] !== "0.05" || fractions[1] !== "0.05") {
  throw new Error(`kFraction 出现 ${fractions.length} 处: ${fractions.join(",")}`);
}
mustConst(ccSrc, /constexpr size_t kDeadlineCheckInterval = ([^;]+);/, "128u", "kDeadlineCheckInterval");
mustConst(ccSrc, /constexpr int kMaxStringTableChunks = ([^;]+);/, "8", "kMaxStringTableChunks");
mustConst(ccSrc, /constexpr int kMinStringTableChunkSize = ([^;]+);/, "1024", "kMinStringTableChunkSize");
mustConst(ccSrc, /const size_t kMaxPointerUpdateTasks = ([^;]+);/, "8", "kMaxPointerUpdateTasks");

// flag 驱动的疏散启发式 (cc:601-607, 值属 flags 族不落值)
for (const f of [
  "compaction_target_fragmentation_percent_for_reduce_memory",
  "compaction_max_evacuated_bytes_mb_for_reduce_memory",
  "compaction_target_fragmentation_percent_for_optimize_memory",
  "compaction_max_evacuated_bytes_mb_for_optimize_memory",
  "compaction_max_evacuated_bytes_mb",
]) {
  if (!ccSrc.includes(f)) throw new Error(`疏散启发式 flag 缺失: ${f}`);
}

// —— 方法清单 (cc + inl) ——
function collectDefs(src, re) {
  const out = [];
  const lines = src.split("\n");
  for (let i = 0; i < lines.length; i++) {
    const m = lines[i].match(re);
    if (!m) continue;
    let full = lines[i].trim();
    const tpl = /^template\s*</.test(lines[i - 1] ?? "") ? lines[i - 1].trim().replace(/\s+/g, " ") + " " : "";
    let j = i;
    while (!/\)\s*(?:const\s*)?[;{=]?\s*$/.test(full) && j + 1 < lines.length) {
      full += " " + lines[++j].trim();
    }
    out.push({ line: i + 1, cls: m[1], name: m[2], sig: (tpl + full).replace(/\s+/g, " ").replace(/\s*\{\s*$/, "") });
  }
  return out;
}
const CLS_RE = /^(?:[A-Za-z_][\w:<>,&*\[\] ]*[\s>])?(MarkCompactCollector|Evacuator|LiveObjectVisitor|RootMarkingVisitor)::([A-Za-z_~]\w*)\s*\(/;
const ccDefs = collectDefs(ccSrc, CLS_RE);
const inlDefs = collectDefs(inlSrc, CLS_RE);
const byCls = {};
for (const d of ccDefs) byCls[d.cls] = (byCls[d.cls] || 0) + 1;
if (byCls.MarkCompactCollector !== 80) throw new Error(`MCC cc 定义数 ${byCls.MarkCompactCollector}, 期望 80`);
if (byCls.Evacuator !== 3) throw new Error(`Evacuator 定义数 ${byCls.Evacuator}, 期望 3`);
if (byCls.LiveObjectVisitor !== 2) throw new Error(`LiveObjectVisitor 定义数 ${byCls.LiveObjectVisitor}, 期望 2`);
if (byCls.RootMarkingVisitor !== 3) throw new Error(`RootMarkingVisitor 定义数 ${byCls.RootMarkingVisitor}, 期望 3`);
const inlByCls = {};
for (const d of inlDefs) inlByCls[d.cls] = (inlByCls[d.cls] || 0) + 1;
if (inlByCls.MarkCompactCollector !== 6) throw new Error(`MCC inl 定义数 ${inlByCls.MarkCompactCollector}, 期望 6`);
if (inlByCls.RootMarkingVisitor !== 3) throw new Error(`RMV inl 定义数 ${inlByCls.RootMarkingVisitor}, 期望 3`);

// inl 关键语义锚: RecordSlot 的记忆集四分派
for (const anchor of [
  "RememberedSet<OLD_TO_NEW_BACKGROUND>::Insert",
  "RememberedSet<TRUSTED_TO_CODE>::Insert",
  "RememberedSet<TRUSTED_TO_TRUSTED>::Insert",
  "RememberedSet<OLD_TO_OLD>::Insert",
]) {
  if (!inlSrc.includes(anchor)) throw new Error(`RecordSlot 分派锚缺失: ${anchor}`);
}

console.log(`mark-compact: EvacuationMode ${evacModes.length} + 名 ${namePairs.length}, 常量 7, 定义 cc ${ccDefs.length} (80/3/2/3) + inl ${inlDefs.length} (6/3)`);

let rust = banner(
  "src/heap/mark-compact.cc.js",
  "src/heap/mark-compact.{cc,inl.h} + mark-compact.h (锚)",
  "疏散模式/压实启发式常量 + 全量方法骨架; 内部枚举在 mark_compact_enums.rs",
);

const snake = (s) => s.replace(/^~/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

rust += `
//! MarkCompactCollector 的机械规则面 (cc + inl)。
//!
//! - EvacuationMode: 疏散执行三态 (对象级新生→老生 / 页级新生→老生 / 老生→老生)
//! - 压实启发式: 常规模式目标碎片率 70%, 单区目标时长 0.5ms;
//!   reduce-memory / optimize-memory 模式由 5 个 compaction_* flag 驱动 (值属 flags 族)
//! - 大函数体 (标记/清理/疏散/指针更新) 留签名骨架, 语义实现由 Developer 就地修订

/// 疏散模式 (Evacuator::EvacuationMode, cc:4700)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EvacuationMode {
  /// kObjectsNewToOld
  ObjectsNewToOld = 0,
  /// kPageNewToOld
  PageNewToOld = 1,
  /// kObjectsOldToOld
  ObjectsOldToOld = 2,
}

impl EvacuationMode {
  /// 追踪名 — EvacuationModeName 的字符串面
  /// v8: src/heap/mark-compact.cc:4706
  #[must_use]
  pub const fn trace_name(self) -> &'static str {
    match self {
      Self::ObjectsNewToOld => "objects-new-to-old",
      Self::PageNewToOld => "page-new-to-old",
      Self::ObjectsOldToOld => "objects-old-to-old",
    }
  }
}

/// 疏散模式选择 — ComputeEvacuationMode 的 1:1 转写
/// (原注释: 判定序重要, will_be_promoted 必须先判)
/// v8: src/heap/mark-compact.cc:4718 Evacuator::ComputeEvacuationMode
#[must_use]
pub const fn compute_evacuation_mode(will_be_promoted: bool, in_young_generation: bool) -> EvacuationMode {
  if will_be_promoted {
    EvacuationMode::PageNewToOld
  } else if in_young_generation {
    EvacuationMode::ObjectsNewToOld
  } else {
    EvacuationMode::ObjectsOldToOld
  }
}

/// 常规模式目标碎片率 % (kTargetFragmentationPercent; cc:613 ComputeEvacuationHeuristics)
pub const K_TARGET_FRAGMENTATION_PERCENT: i32 = 70;
/// 单区 (页载荷) 目标压实时长 ms (kTargetMsPerArea, 原版 float; cc:618)
pub const K_TARGET_MS_PER_AREA: f32 = 0.5;
/// 压测中止抽查比例 (kFraction = 0.05; cc:1566 SetUpAbortEvacuationAtAddress 与
/// cc:5202 EvacuatePagesInParallel 两处同值, 均仅 stress_compaction 路径)
pub const K_STRESS_ABORT_FRACTION: f64 = 0.05;
/// 标记工作表处理的截止检查间隔对象数 (kDeadlineCheckInterval; cc:2295)
pub const K_DEADLINE_CHECK_INTERVAL: usize = 128;
/// 字符串表分块上限 (kMaxStringTableChunks; cc:3136 ClearNonLiveReferences)
pub const K_MAX_STRING_TABLE_CHUNKS: i32 = 8;
/// 字符串表分块最小尺寸 (kMinStringTableChunkSize; cc:3137)
pub const K_MIN_STRING_TABLE_CHUNK_SIZE: i32 = 1024;
/// 指针更新并行任务上限 (kMaxPointerUpdateTasks; cc:5406)
pub const K_MAX_POINTER_UPDATE_TASKS: usize = 8;
`;

// —— 方法签名骨架 (按类分组) ——
rust += `
// —— 方法签名骨架 (语义实现由 Developer 就地修订) ——
`;
const mods = {
  MarkCompactCollector: "mark_compact_collector",
  RootMarkingVisitor: "root_marking_visitor",
  Evacuator: "evacuator",
  LiveObjectVisitor: "live_object_visitor",
};
const groups = {};
for (const d of [...ccDefs.map((d) => ({ ...d, src: "cc" })), ...inlDefs.map((d) => ({ ...d, src: "inl.h" }))]) {
  (groups[d.cls] ??= []).push(d);
}
for (const [cls, mod] of Object.entries(mods)) {
  rust += `\n/// ${cls} 方法骨架 (v8: src/heap/mark-compact.cc + mark-compact-inl.h)
pub mod ${mod} {
`;
  const seen = new Map();
  for (const d of groups[cls]) {
    const base = snake(d.name);
    const n = (seen.get(base) ?? 0) + 1;
    seen.set(base, n);
    const nm = n === 1 ? base : `${base}_v${n}`;
    rust += `  /// 原版签名: ${d.sig}
  /// v8: src/heap/mark-compact.${d.src}:${d.line}
  pub fn ${nm}() {
    todo!("就地修订")
  }
`;
  }
  rust += `}\n`;
}

rust += `
// —— 提取不变式断言 ——
const _: () = {
  assert!(K_TARGET_FRAGMENTATION_PERCENT == 70);
  assert!(K_TARGET_MS_PER_AREA == 0.5);
  assert!(K_STRESS_ABORT_FRACTION == 0.05);
  assert!(K_DEADLINE_CHECK_INTERVAL == 128);
  assert!(K_MAX_STRING_TABLE_CHUNKS == 8);
  assert!(K_MIN_STRING_TABLE_CHUNK_SIZE == 1024);
  assert!(K_MAX_POINTER_UPDATE_TASKS == 8);
  assert!(matches!(compute_evacuation_mode(true, true), EvacuationMode::PageNewToOld));
  assert!(matches!(compute_evacuation_mode(true, false), EvacuationMode::PageNewToOld));
  assert!(matches!(compute_evacuation_mode(false, true), EvacuationMode::ObjectsNewToOld));
  assert!(matches!(compute_evacuation_mode(false, false), EvacuationMode::ObjectsOldToOld));
};

// —— 字符串面对数 (str 相等非 const, 挂接后随 crate 测试跑; 暂以标准 #[test] 占位,
//    挂接时由 Developer 改写为仓库规范测试样式) ——
#[cfg(test)]
mod generated_checks {
  use super::EvacuationMode;

  #[test]
  fn evacuation_mode_names() {
    assert_eq!(EvacuationMode::ObjectsNewToOld.trace_name(), "objects-new-to-old");
    assert_eq!(EvacuationMode::PageNewToOld.trace_name(), "page-new-to-old");
    assert_eq!(EvacuationMode::ObjectsOldToOld.trace_name(), "objects-old-to-old");
  }
}
`;

const p = writeSrc("v8-heap-gc", "mark_compact_cc.rs.skel", rust);
console.log(`written ${p}`);
