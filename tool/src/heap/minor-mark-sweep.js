#!/usr/bin/env bun
// tool/src/heap/minor-mark-sweep.js
// 机械提取 v8/src/heap/minor-mark-sweep{.h,.cc,-inl.h} 族:
// MinorMarkSweepCollector / 记忆集标记 worklist / SlotsType 枚举

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/minor-mark-sweep.h");
const ccSrc = readV8("src/heap/minor-mark-sweep.cc");
const inlSrc = readV8("src/heap/minor-mark-sweep-inl.h");

const anchors = [
  [hSrc, "class YoungGenerationRememberedSetsMarkingWorklist {"],
  [hSrc, "enum class SlotsType { kRegularSlots, kTypedSlots };"],
  [hSrc, "class YoungGenerationRootMarkingVisitor final : public RootVisitor {"],
  [hSrc, "class MinorMarkSweepCollector final {"],
  [hSrc, "static constexpr size_t kMaxParallelTasks = 8;"],
  [hSrc, "GarbageCollector collector() const override {"],
  [hSrc, "void StartMarking(bool force_use_background_threads);"],
  [hSrc, "void RequestGC();"],
  [hSrc, "using ResizeNewSpaceMode = Heap::ResizeNewSpaceMode;"],
  [ccSrc, "constexpr size_t MinorMarkSweepCollector::kMaxParallelTasks;"],
  [inlSrc, "VisitRootPointer"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

// 常量真提取
const m = hSrc.match(/kMaxParallelTasks = (\d+);/);
if (!m || m[1] !== "8") throw new Error("kMaxParallelTasks 漂移");
// SlotsType 枚举提取
const sm = hSrc.match(/enum class SlotsType \{([^}]*)\};/);
if (!sm) throw new Error("SlotsType 未找到");
const slotsType = sm[1].split(",").map((s) => s.trim()).filter(Boolean);
if (slotsType.join() !== "kRegularSlots,kTypedSlots") throw new Error(`SlotsType 漂移: ${slotsType}`);

// cc 关键函数对数
const ccFns = ["MinorMarkSweepCollector::MinorMarkSweepCollector", "MinorMarkSweepCollector::CollectGarbage", "MinorMarkSweepCollector::StartMarking", "MinorMarkSweepCollector::MarkLiveObjects", "MinorMarkSweepCollector::Sweep", "MinorMarkSweepCollector::TearDown", "CollectItems", "MinorMarkSweepCollector::RequestGC"];
for (const f of ccFns) if (!ccSrc.includes(f)) throw new Error(`cc 缺 ${f}`);
console.log(`minor-mark-sweep: kMaxParallelTasks=8, SlotsType ${slotsType.length}, cc 关键函数 ${ccFns.length}`);

const snake = (s) => s.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

let rust = banner(
  "src/heap/minor-mark-sweep.js",
  "src/heap/minor-mark-sweep.h, src/heap/minor-mark-sweep.cc, src/heap/minor-mark-sweep-inl.h",
  "新生代 mark-sweep 收集器 (minor-ms)",
);

rust += `/// v8: src/heap/minor-mark-sweep.h MarkingItem::SlotsType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SlotsType {
  /// ${slotsType[0]}
  ${snake(slotsType[0])} = 0,
  /// ${slotsType[1]}
  ${snake(slotsType[1])} = 1,
}

/// v8: src/heap/minor-mark-sweep.h YoungGenerationRememberedSetsMarkingWorklist
/// MarkingItem = (MutablePage*, SlotsType, SlotSet*/TypedSlotSet* 联合, background_slot_set)
pub struct YoungGenerationRememberedSetsMarkingWorklist {
  /// v8: remembered_sets_marking_items_
  pub items: Vec<MarkingItemEntry>,
  /// v8: remaining_remembered_sets_marking_items_ (atomic_size_t, relaxed)
  pub remaining: AtomicUsize,
}

/// v8: YoungGenerationRememberedSetsMarkingWorklist::MarkingItem 的字段面
pub struct MarkingItemEntry {
  pub chunk: *mut MutablePage,
  pub slots_type: SlotsType,
  /// v8: union { SlotSet* slot_set_; TypedSlotSet* typed_slot_set_; }
  pub slot_set: *mut SlotSet,
  /// v8: background_slot_set_ = nullptr
  pub background_slot_set: *mut SlotSet,
}

impl YoungGenerationRememberedSetsMarkingWorklist {
  /// v8: CollectItems(Heap*) (static) — 扫全部页收集记忆集项
  /// v8: ProcessNextItem(visitor, index) — IndexGenerator 认领
  /// v8: RemainingRememberedSetsMarkingIteams (原文拼写) — relaxed load
  /// v8: TearDown — DeleteSetsOnTearDown
}

/// v8: src/heap/minor-mark-sweep.h MinorMarkSweepCollector
pub struct MinorMarkSweepCollector {
  /// v8: marking_worklists_ / main_marking_visitor_ /
  ///     remembered_sets_marking_handler_ / ephemeron_table_list_ /
  ///     sweeper_ / is_in_atomic_pause_ / gc_finalization_requested_ /
  ///     use_background_threads_in_cycle_ / resize_new_space_mode_
  pub fields_todo: (),
}

impl MinorMarkSweepCollector {
  /// v8: MinorMarkSweepCollector::kMaxParallelTasks = 8
  pub const K_MAX_PARALLEL_TASKS: usize = 8;

  /// v8: MinorMarkSweepCollector::CollectGarbage
  pub fn collect_garbage(&mut self) { todo!() }
  /// v8: MinorMarkSweepCollector::StartMarking(force_use_background_threads)
  pub fn start_marking(&mut self, force_use_background_threads: bool) { todo!() }
  /// v8: MinorMarkSweepCollector::RequestGC
  pub fn request_gc(&mut self) { todo!() }
  /// v8: MinorMarkSweepCollector::TearDown
  pub fn tear_down(&mut self) { todo!() }
  /// v8: is_in_atomic_pause / gc_finalization_requsted (原文拼写) / UseBackgroundThreadsInCycle
}

/// v8: src/heap/minor-mark-sweep.h YoungGenerationRootMarkingVisitor
/// collector() = GarbageCollector::MINOR_MARK_SWEEPER;
/// VisitRootPointer / VisitRootPointers (inl 转发 VisitPointersImpl)
/// v8: src/heap/minor-mark-sweep-inl.h YoungGenerationRootMarkingVisitor::VisitRootPointer
/// v8: ResizeNewSpaceMode = Heap::ResizeNewSpaceMode { kShrink, kGrow, kNone }

// —— 提取不变式断言 ——
const _: () = {
  assert!(SlotsType::TYPED_SLOTS == 1);
  assert!(MinorMarkSweepCollector::K_MAX_PARALLEL_TASKS == 8);
};
`;

const p = writeSrc("v8-heap-gc", "minor_mark_sweep.rs.skel", rust);
console.log(`written ${p}`);
