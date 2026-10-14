#!/usr/bin/env bun
// tool/src/heap/sweeper.cc.js
// 机械提取 v8/src/heap/sweeper.{h,cc}: ZappingMode 规则 / 并发任务常量 /
// 槽位空间计数 (AllocationSpace 推导) + 方法签名骨架
// 枚举面 (SweepingMode/FreeSpaceTreatmentMode) 已在 sweeper_enums.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/sweeper.h");
const ccSrc = readV8("src/heap/sweeper.cc");
const gSrc = readV8("src/common/globals.h");

// —— ZappingMode (cc:630) ——
const zm = ccSrc.match(/enum class ZappingMode \{([^}]*)\};/);
if (!zm) throw new Error("ZappingMode 缺失");
const zappingModes = zm[1].split(",").map((s) => s.trim()).filter(Boolean);
if (zappingModes.join() !== "kNone,kCreateFillers,kCreateFillersAndZap") {
  throw new Error(`ZappingMode 序漂移: ${zappingModes.join()}`);
}

// ShouldZapDeadObjectsOnPage 规则锚 (cc:631-643)
const szdoStart = ccSrc.indexOf("ZappingMode ShouldZapDeadObjectsOnPage()");
const szdo = ccSrc.slice(szdoStart, ccSrc.indexOf("\n}", szdoStart));
if (!szdo.includes("heap::ShouldZapGarbage() || v8_flags.track_gc_object_stats")) {
  throw new Error("ShouldZapDeadObjectsOnPage 首支漂移");
}
if (!szdo.includes("kCreateFillersAndZap")) throw new Error("kCreateFillersAndZap 支缺失");
if (!szdo.includes("v8_flags.conservative_stack_scanning ? ZappingMode::kCreateFillers")) {
  throw new Error("conservative_stack_scanning 支缺失");
}
if (!szdo.includes(": ZappingMode::kNone")) throw new Error("kNone 支缺失");

// —— AtomicZapBlock 的 kZapTagged (cc:54, Tagged_t = 压缩后 uint32) ——
const zapTagged = ccSrc.match(/static constexpr Tagged_t kZapTagged = static_cast<Tagged_t>\(kZapValue\);/);
if (!zapTagged) throw new Error("kZapTagged 定义缺失");
const zap64 = gSrc.match(/constexpr uint64_t kZapValue = uint64_t\{0x([0-9a-fA-F]+)\};/);
if (!zap64) throw new Error("64 位 kZapValue 缺失");
const ZAP64_HI = BigInt("0x" + zap64[1]);
const ZAP_TAGGED = Number(ZAP64_HI & 0xffffffffn);
if (ZAP_TAGGED !== 0xbeadbeef) throw new Error(`kZapTagged 截断漂移: 0x${ZAP_TAGGED.toString(16)}`);
// Tagged_t 为 uint32 的前提 (globals.h:667 static_assert 链) — 指针压缩目标配置
if (!gSrc.includes("constexpr int kTaggedSize = kInt32Size;")) throw new Error("kTaggedSize=4 支缺失 (指针压缩)");

// —— AllocationSpace 计数推导 (globals.h) ——
const asEnum = gSrc.match(/enum AllocationSpace \{([\s\S]*?)\n\};/);
if (!asEnum) throw new Error("AllocationSpace 缺失");
const spaceLines = asEnum[1].split("\n").map((l) => l.trim().replace(/\/\/.*$/, "").trim()).filter(Boolean);
const spaces = spaceLines.filter((l) => /^[A-Z_]+,$/.test(l)).map((l) => l.replace(",", ""));
if (spaces.length !== 13) throw new Error(`AllocationSpace 成员 ${spaces.length}, 期望 13`);
const aliases = {};
for (const l of spaceLines) {
  const m = l.match(/^([A-Z_]+) = ([A-Z_]+),?$/);
  if (m) aliases[m[1]] = m[2];
}
if (aliases.FIRST_SWEEPABLE_SPACE !== "NEW_SPACE" || aliases.LAST_SWEEPABLE_SPACE !== "SHARED_TRUSTED_SPACE") {
  throw new Error(`sweepable 别名漂移: ${JSON.stringify(aliases)}`);
}
const NEW_SPACE = spaces.indexOf("NEW_SPACE");
const OLD_SPACE = spaces.indexOf("OLD_SPACE");
const SHARED_TRUSTED = spaces.indexOf("SHARED_TRUSTED_SPACE");
if (NEW_SPACE !== 1 || OLD_SPACE !== 2 || SHARED_TRUSTED !== 6) {
  throw new Error(`空间判别值漂移: NEW=${NEW_SPACE} OLD=${OLD_SPACE} SHARED_TRUSTED=${SHARED_TRUSTED}`);
}
const N_SWEEPING = SHARED_TRUSTED - NEW_SPACE + 1;
const N_MAJOR_SWEEPING = N_SWEEPING - 1;
if (N_SWEEPING !== 6 || N_MAJOR_SWEEPING !== 5) throw new Error(`清扫空间计数漂移: ${N_SWEEPING}/${N_MAJOR_SWEEPING}`);

// —— 任务常量锚 ——
if (!/kNumberOfMajorSweepingSpaces =\s*kNumberOfSweepingSpaces - 1;/.test(ccSrc)) throw new Error("主清扫空间式缺失");
if (!/kMaxTasks = kNumberOfMajorSweepingSpaces;/.test(ccSrc)) throw new Error("主任务上限式缺失");
const maxTasksMinor = ccSrc.match(/class Sweeper::MinorSweeperJob final : public JobTask \{[\s\S]*?static constexpr int kMaxTasks = (\d+);/);
if (!maxTasksMinor || maxTasksMinor[1] !== "1") throw new Error("次任务上限漂移");
const pagePerTask = [...ccSrc.matchAll(/static constexpr int kPagePerTask = (\d+);/g)].map((m) => m[1]);
if (pagePerTask.length !== 2 || pagePerTask[0] !== "2" || pagePerTask[1] !== "2") {
  throw new Error(`kPagePerTask 漂移: ${pagePerTask.join(",")}`);
}
// kNewSpace (cc:98-99): sticky_mark_bits ? OLD_SPACE : NEW_SPACE
if (!/static constexpr auto kNewSpace =\s*v8_flags\.sticky_mark_bits\.value\(\) \? OLD_SPACE : NEW_SPACE;/.test(ccSrc)) {
  throw new Error("kNewSpace 选择式缺失");
}
// kDiscardEmptyPages 平台分叉 (cc:1065-1073): 非 WIN 真 / WIN 假
if (!ccSrc.includes("#if !defined(V8_OS_WIN)")) throw new Error("kDiscardEmptyPages 平台门缺失");
if (!/constexpr bool kDiscardEmptyPages = true;/.test(ccSrc) || !/constexpr bool kDiscardEmptyPages = false;/.test(ccSrc)) {
  throw new Error("kDiscardEmptyPages 双值缺失");
}

// —— .h 侧计数锚 (kNumberOfSweepingSpaces 推导式, sweeper.h:207-208) ——
if (!/kNumberOfSweepingSpaces =\s*LAST_SWEEPABLE_SPACE - FIRST_SWEEPABLE_SPACE \+ 1;/.test(hSrc)) {
  throw new Error("kNumberOfSweepingSpaces 推导式缺失");
}

// —— 方法清单 ——
function collectDefs(src, re) {
  const out = [];
  const lines = src.split("\n");
  for (let i = 0; i < lines.length; i++) {
    const m = lines[i].match(re);
    if (!m) continue;
    let full = lines[i].trim();
    let j = i;
    while (!/\)\s*(?:const\s*)?[;{=]?\s*$/.test(full) && j + 1 < lines.length) {
      full += " " + lines[++j].trim();
    }
    out.push({ line: i + 1, cls: m[1], name: m[2], sig: full.replace(/\s+/g, " ").replace(/\s*\{\s*$/, "") });
  }
  return out;
}
const defs = collectDefs(
  ccSrc,
  /^(?:[A-Za-z_][\w:<>,&*\[\] ]*[\s>])?(Sweeper::SweepingState<scope>|Sweeper::LocalSweeper|Sweeper)::([A-Za-z_~]\w*)\s*\(/,
);
const byCls = {};
for (const d of defs) byCls[d.cls] = (byCls[d.cls] || 0) + 1;
if (byCls["Sweeper::SweepingState<scope>"] !== 12) throw new Error(`SweepingState 定义数漂移: ${byCls["Sweeper::SweepingState<scope>"]}`);
if (byCls["Sweeper::LocalSweeper"] !== 7) throw new Error(`LocalSweeper 定义数漂移: ${byCls["Sweeper::LocalSweeper"]}`);
if (byCls.Sweeper !== 51) throw new Error(`Sweeper 定义数漂移: ${byCls.Sweeper}`);

console.log(`sweeper: ZappingMode ${zappingModes.length}, zapTagged 0x${ZAP_TAGGED.toString(16)}, 空间 ${N_SWEEPING}/${N_MAJOR_SWEEPING}, 定义 ${defs.length} (12/7/51)`);

let rust = banner(
  "src/heap/sweeper.cc.js",
  "src/heap/sweeper.{h,cc} + src/common/globals.h (空间计数)",
  "清扫并发常量/ZappingMode 规则; 枚举面在 sweeper_enums.rs",
);

const snake = (s) => s.replace(/^~/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
const upper = (s) => s.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

rust += `
//! Sweeper 的机械规则面。
//!
//! - ZappingMode: 死对象处置三态 (filler / filler+zap), 由构建与 flag 决定
//! - kZapTagged: kZapValue 截断到 Tagged_t (指针压缩 → u32) = 0x${ZAP_TAGGED.toString(16)}
//! - 并发任务: 主清扫空间数 = 可清扫空间数 - 1 (剥 NEW_SPACE); 每 2 页 1 任务
//! - kNewSpace (MinorSweeperJob 的清扫空间): sticky_mark_bits 时走 OLD_SPACE

/// 死对象处置模式 (enum class ZappingMode, cc:630)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ZappingMode {
  /// kNone — 不处置
  None = 0,
  /// kCreateFillers — 仅铺 filler (保守栈扫描需要)
  CreateFillers = 1,
  /// kCreateFillersAndZap — 铺 filler 并 zap (verify-heap / 对象统计需要)
  CreateFillersAndZap = 2,
}

/// 死对象处置模式选择 — ShouldZapDeadObjectsOnPage 的 1:1 转写
/// v8: src/heap/sweeper.cc:631
#[must_use]
pub const fn should_zap_dead_objects_on_page(
  should_zap_garbage: bool,
  track_gc_object_stats: bool,
  conservative_stack_scanning: bool,
) -> ZappingMode {
  if should_zap_garbage || track_gc_object_stats {
    // verify-heap / 对象统计要求死对象仍是合法对象
    ZappingMode::CreateFillersAndZap
  } else if conservative_stack_scanning {
    // 保守栈扫描要求全死对象铺 filler, 防栈上假引用复活死对象
    ZappingMode::CreateFillers
  } else {
    ZappingMode::None
  }
}

/// 原子 zap 的填充字 (kZapTagged = kZapValue 截断到 Tagged_t; 指针压缩 → u32)
/// v8: src/heap/sweeper.cc:54 AtomicZapBlock
pub const K_ZAP_TAGGED: u32 = 0x${ZAP_TAGGED.toString(16)};

/// NEW_SPACE 判别值 (AllocationSpace, globals.h:1481)
pub const K_NEW_SPACE: u8 = ${NEW_SPACE};
/// OLD_SPACE 判别值
pub const K_OLD_SPACE: u8 = ${OLD_SPACE};
/// FIRST_SWEEPABLE_SPACE = NEW_SPACE (globals.h:1512)
pub const K_FIRST_SWEEPABLE_SPACE: u8 = ${NEW_SPACE};
/// LAST_SWEEPABLE_SPACE = SHARED_TRUSTED_SPACE (globals.h:1513)
pub const K_LAST_SWEEPABLE_SPACE: u8 = ${SHARED_TRUSTED};
/// 可清扫空间数 (kNumberOfSweepingSpaces = LAST - FIRST + 1, sweeper.h:207)
pub const K_NUMBER_OF_SWEEPING_SPACES: u8 = ${N_SWEEPING};
/// 主清扫空间数 (kNumberOfMajorSweepingSpaces = 上者 - 1, 剥 NEW_SPACE; cc:135)
pub const K_NUMBER_OF_MAJOR_SWEEPING_SPACES: u8 = ${N_MAJOR_SWEEPING};
/// 主清扫任务上限 (MajorSweeperJob::kMaxTasks; cc:139)
pub const K_MAX_TASKS_MAJOR: u8 = ${N_MAJOR_SWEEPING};
/// 次清扫任务上限 (MinorSweeperJob::kMaxTasks = 1; cc:215)
pub const K_MAX_TASKS_MINOR: u8 = 1;
/// 每任务页数 (两 Job 的 GetMaxConcurrency 内 kPagePerTask = 2; cc:169/245)
pub const K_PAGE_PER_TASK: u8 = 2;

/// MinorSweeperJob 的清扫空间 — kNewSpace = sticky_mark_bits ? OLD_SPACE : NEW_SPACE
/// v8: src/heap/sweeper.cc:98
#[must_use]
pub const fn concurrent_minor_space(sticky_mark_bits: bool) -> u8 {
  if sticky_mark_bits { K_OLD_SPACE } else { K_NEW_SPACE }
}

/// GetMaxConcurrency 并发度式 — min(并发清扫器数, worker + (页数 + 每任务页数 - 1) / 每任务页数)
/// v8: src/heap/sweeper.cc:168 MajorSweeperJob::GetMaxConcurrency (Minor 同式)
#[must_use]
pub const fn get_max_concurrency(concurrent_sweepers: usize, worker_count: usize, sweeping_page_count: usize) -> usize {
  let per_task = worker_count + (sweeping_page_count + K_PAGE_PER_TASK as usize - 1) / K_PAGE_PER_TASK as usize;
  if concurrent_sweepers < per_task { concurrent_sweepers } else { per_task }
}
`;

// —— 方法签名骨架 ——
rust += `
// —— 方法签名骨架 (语义实现由 Developer 就地修订) ——
`;
const mods = { "Sweeper::SweepingState<scope>": "sweeping_state", "Sweeper::LocalSweeper": "local_sweeper", Sweeper: "sweeper" };
const seen = new Map();
for (const d of defs) {
  const base = `${mods[d.cls]}_${snake(d.name)}`;
  const n = (seen.get(base) ?? 0) + 1;
  seen.set(base, n);
  const nm = n === 1 ? base : `${base}_v${n}`;
  rust += `/// 原版签名: ${d.sig}
/// v8: src/heap/sweeper.cc:${d.line}
pub fn ${nm}() {
  todo!("就地修订")
}
`;
}

rust += `
// —— 提取不变式断言 ——
const _: () = {
  assert!(K_ZAP_TAGGED == 0xbead_beef);
  assert!(K_NEW_SPACE == 1);
  assert!(K_OLD_SPACE == 2);
  assert!(K_FIRST_SWEEPABLE_SPACE == 1);
  assert!(K_LAST_SWEEPABLE_SPACE == 6);
  assert!(K_NUMBER_OF_SWEEPING_SPACES == 6);
  assert!(K_NUMBER_OF_MAJOR_SWEEPING_SPACES == 5);
  assert!(K_MAX_TASKS_MAJOR == 5);
  assert!(K_MAX_TASKS_MINOR == 1);
  assert!(K_PAGE_PER_TASK == 2);
  assert!(concurrent_minor_space(false) == 1);
  assert!(concurrent_minor_space(true) == 2);
  assert!(matches!(should_zap_dead_objects_on_page(true, false, false), ZappingMode::CreateFillersAndZap));
  assert!(matches!(should_zap_dead_objects_on_page(false, true, false), ZappingMode::CreateFillersAndZap));
  assert!(matches!(should_zap_dead_objects_on_page(false, false, true), ZappingMode::CreateFillers));
  assert!(matches!(should_zap_dead_objects_on_page(false, false, false), ZappingMode::None));
  assert!(get_max_concurrency(5, 1, 0) == 1);
  assert!(get_max_concurrency(5, 0, 3) == 2); // 3 页 / 每任务 2 页 → 2
  assert!(get_max_concurrency(3, 8, 9) == 3);
  assert!(ZappingMode::None as u8 == 0);
  assert!(ZappingMode::CreateFillers as u8 == 1);
  assert!(ZappingMode::CreateFillersAndZap as u8 == 2);
};
`;

const p = writeSrc("v8-heap-gc", "sweeper_cc.rs.skel", rust);
console.log(`written ${p}`);
