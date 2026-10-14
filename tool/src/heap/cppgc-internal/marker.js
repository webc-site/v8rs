#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/marker.js — v8/src/heap/cppgc-internal/marker.h + .cc → v8-heap-cppgc/src/marker.rs.skel
// 标记器: WriteBarrierType 双型 + 七步生命周期 + 增量观察器 (256KB/步, 2ms 上限) + 150 步进间隔
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/marker.h";
const REL_CC = "src/heap/cppgc-internal/marker.cc";
const src = readV8(REL);
const srcCc = readV8(REL_CC);

let n = 0;
const need = (s, needle, label, file) => {
  if (!s.includes(needle)) throw new Error(`${file} 缺锚 ${label}: ${needle}`);
  n++;
};

// ── ① 类与嵌套 ────────────────────────────────────────────────────────
need(src, "class V8_EXPORT_PRIVATE MarkerBase {", "MarkerBase 声明");
need(src, "enum class WriteBarrierType {\n    kDijkstra,\n    kSteele,\n  };", "WriteBarrierType 枚举");
need(src, "class PauseConcurrentMarkingScope final {", "暂停并发作用域");
need(src, "const bool resume_on_exit_;", "恢复标志字段");
need(src, "void EnterAtomicPause(StackState);", "EnterAtomicPause 声明");
need(src, "void EnterProcessGlobalAtomicPause();", "进程全局停顿声明");
need(src, "void ReEnableConcurrentMarking();", "ReEnable 声明");
need(src, "bool AdvanceMarkingWithLimits(v8::base::TimeDelta max_duration,", "AdvanceMarkingWithLimits 声明");
need(src, "void LeaveAtomicPause();", "LeaveAtomicPause 声明");
need(src, "void StartMarking();", "StartMarking 声明");
need(src, "void FinishMarking(StackState);", "FinishMarking 声明");
need(src, "void ProcessCrossThreadWeaknessIfNeeded();", "跨线程弱处理");
need(src, "void ProcessWeakness();", "弱处理");
need(src, "bool JoinConcurrentMarkingIfNeeded();", "Join 声明");
need(src, "void NotifyConcurrentMarkingOfWorkIfNeeded(cppgc::TaskPriority);", "通知并发声明");
need(src, "static constexpr size_t kMinAllocatedBytesPerStep = 256 * kKB;", "步进字节 256KB");
need(src, "static constexpr v8::base::TimeDelta kMaximumIncrementalStepDuration =", "步进时长上限声明");
need(src, "v8::base::TimeDelta::FromMilliseconds(2);", "2ms");
need(src, "virtual cppgc::Visitor& visitor() = 0;", "五个纯虚 (visitor)");
need(src, "virtual ConservativeTracingVisitor& conservative_visitor() = 0;", "纯虚 conservative");
need(src, "virtual heap::base::StackVisitor& stack_visitor() = 0;", "纯虚 stack");
need(src, "virtual ConcurrentMarkerBase& concurrent_marker() = 0;", "纯虚 concurrent");
need(src, "virtual heap::base::IncrementalMarkingSchedule& schedule() = 0;", "纯虚 schedule");
need(src, "bool ProcessWorklistsWithDeadline(v8::base::TimeTicks time_deadline,", "Worklists 排空声明");
need(src, "void VisitLocalRoots(StackState);", "本地根声明");
need(src, "void VisitCrossThreadRoots();", "跨线程根声明");
need(src, "void MarkNotFullyConstructedObjects();", "构造中标记声明");
need(src, "class V8_EXPORT_PRIVATE Marker final : public MarkerBase {", "Marker 声明");
// 字段
for (const f of ["MarkingConfig config_ = MarkingConfig::Default();", "MarkingWorklists marking_worklists_;",
  "MutatorMarkingState mutator_marking_state_;", "size_t last_bytes_marked_ = 0;", "bool is_marking_{false};",
  "bool main_marking_disabled_for_testing_{false};", "bool processed_cross_thread_weakness_{false};"]) need(src, f, "MarkerBase 字段");
// WriteBarrierForObject 语义
need(src, "case MarkerBase::WriteBarrierType::kDijkstra:\n      if (!header.TryMarkAtomic()) {", "Dijkstra 未标即标");
need(src, "case MarkerBase::WriteBarrierType::kSteele:\n      if (!header.IsMarked<AccessMode::kAtomic>()) {", "Steele 已标才追");
need(src, "header.Unmark<AccessMode::kAtomic>();", "构造中回退标记");
need(src, "mutator_marking_state_.write_barrier_worklist().Push(&header);", "Dijkstra 入队");
need(src, "mutator_marking_state_.retrace_marked_objects_worklist().Push(&header);", "Steele 重trace入队");
// ── ② .cc 常量与生命周期 ──────────────────────────────────────────────
need(srcCc, "static constexpr size_t kDefaultDeadlineCheckInterval = 150u;", "步进间隔 150");
need(srcCc, "WriteBarrier::FlagUpdater::Enter();", "屏障位进入");
need(srcCc, "heap.set_incremental_marking_in_progress(true);", "进度旗置位");
need(srcCc, "bool ExitIncrementalMarkingIfNeeded(MarkingConfig config, HeapBase& heap) {", "退出判定");
need(srcCc, "class MarkerBase::IncrementalMarkingTask final : public cppgc::Task {", "增量任务类");
need(srcCc, "void MarkerBase::StartMarking() {", "StartMarking 实体");
need(srcCc, "void MarkerBase::FinishMarking(StackState stack_state) {", "FinishMarking 实体");
need(srcCc, "void MarkerBase::EnterAtomicPause(StackState stack_state) {", "EnterAtomicPause 实体");
need(srcCc, "void MarkerBase::LeaveAtomicPause() {", "LeaveAtomicPause 实体");
need(srcCc, "void MarkerBase::EnterProcessGlobalAtomicPause() { VisitCrossThreadRoots(); }", "进程停顿内联");
need(srcCc, "void MarkerBase::ProcessWeakness() {", "ProcessWeakness 实体");
need(srcCc, "config.marking_type == MarkingConfig::MarkingType::kIncremental ||", "增量型判定");
// 方法实体计数
const defs = (srcCc.match(/^void MarkerBase::|^bool MarkerBase::|^size_t MarkerBase::|^void MarkerBase::IncrementalMarkingTask::|^void MarkerBase::IncrementalMarkingAllocationObserver::|^std::unique_ptr<Visitor> ConcurrentMarker::|^MarkerBase::IncrementalMarkingTask|^void ConcurrentMarker::/gm) || []).length;
if (defs !== 31) throw new Error(`marker.cc 实体计数失配: ${defs} want 31`);
n += defs;

const body = `${banner("src/heap/cppgc-internal/marker.js", `${REL} + ${REL_CC}`, "标记器: 增量/并发/原子停顿编排")}
#![allow(dead_code)]

pub mod constants {
    /// v8: src/heap/cppgc-internal/marker.h MarkerBase::IncrementalMarkingAllocationObserver::kMinAllocatedBytesPerStep
    pub const K_MIN_ALLOCATED_BYTES_PER_STEP: usize = 256 * K_KB;
    /// v8: src/heap/cppgc-internal/marker.h MarkerBase::kMaximumIncrementalStepDuration
    pub const K_MAXIMUM_INCREMENTAL_STEP_DURATION_MS: u64 = 2;
    /// v8: src/heap/cppgc-internal/marker.cc kDefaultDeadlineCheckInterval
    pub const K_DEFAULT_DEADLINE_CHECK_INTERVAL: usize = 150;
}

/// v8: src/heap/cppgc-internal/marker.h MarkerBase::WriteBarrierType
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WriteBarrierType {
    /// v8: kDijkstra — 增量 barrier: 未标则标 + 入 write_barrier 队
    Dijkstra,
    /// v8: kSteele — 删除 barrier: 已标才入 retrace 队
    Steele,
}

/// v8: src/heap/cppgc-internal/marker.h MarkerBase
pub struct MarkerBase {
    /// v8: heap_
    pub heap: *mut HeapBase,
    /// v8: config_ = MarkingConfig::Default()
    pub config: MarkingConfig,
    /// v8: platform_ / foreground_task_runner_
    platform: *mut Platform,
    /// v8: incremental_marking_handle_
    incremental_marking_handle: SingleThreadedHandle,
    /// v8: incremental_marking_allocation_observer_
    incremental_marking_allocation_observer: IncrementalMarkingAllocationObserver,
    /// v8: marking_worklists_
    pub marking_worklists: MarkingWorklists,
    /// v8: mutator_marking_state_
    pub mutator_marking_state: MutatorMarkingState,
    /// v8: last_bytes_marked_
    last_bytes_marked: usize,
    /// v8: is_marking_ / main_marking_disabled_for_testing_ / visited_cross_thread_persistents_in_atomic_pause_ / processed_cross_thread_weakness_
    is_marking: bool,
    main_marking_disabled_for_testing: bool,
    visited_cross_thread_persistents_in_atomic_pause: bool,
    processed_cross_thread_weakness: bool,
}

impl MarkerBase {
    /// v8: src/heap/cppgc-internal/marker.cc StartMarking
    /// WriteBarrier 位进入 + set_incremental_marking_in_progress(true);
    /// 返回是否增量 (kIncremental || kIncrementalAndConcurrent)。
    pub fn start_marking(&mut self) { todo!("屏障位 + 旗") }

    /// v8: src/heap/cppgc-internal/marker.cc EnterAtomicPause
    /// 暂停并发 → FlushNotFullyConstructed → 按栈态更新 config → VisitLocalRoots。
    pub fn enter_atomic_pause(&mut self, _stack_state: StackState) { todo!("四步") }
    /// v8: EnterProcessGlobalAtomicPause = VisitCrossThreadRoots()
    pub fn enter_process_global_atomic_pause(&mut self) { self.visit_cross_thread_roots(); }
    /// v8: src/heap/cppgc-internal/marker.cc LeaveAtomicPause
    /// 重置 mutator state (ephemeron/原子停顿) + 退出屏障位 + is_marking_=false。
    pub fn leave_atomic_pause(&mut self) { todo!("复位") }
    /// v8: src/heap/cppgc-internal/marker.cc FinishMarking
    /// EnterAtomicPause → EnterProcessGlobal → AdvanceMarkingWithLimits → ProcessWeakness → LeaveAtomicPause。
    pub fn finish_marking(&mut self, _stack_state: StackState) { todo!("五步组合") }

    /// v8: src/heap/cppgc-internal/marker.cc AdvanceMarkingWithLimits
    /// ProcessWorklistsWithDeadline(时刻上限, 字节上限); 回填 last_bytes_marked_。
    pub fn advance_marking_with_limits(&mut self, _max_duration: TimeDelta, _marked_bytes_limit: usize) -> bool { todo!("限界推进") }

    /// v8: src/heap/cppgc-internal/marker.cc ProcessWorklistsWithDeadline
    /// 逐 worklist DrainWorklistWithBytesAndTimeDeadline<150>; bailout 项不受字节上限约束。
    pub fn process_worklists_with_deadline(&mut self, _time_deadline: TimeTicks, _marked_bytes_deadline: usize) -> bool { todo!("八队排空") }

    /// v8: src/heap/cppgc-internal/marker.cc ProcessWeakness / ProcessCrossThreadWeaknessIfNeeded
    pub fn process_weakness(&mut self) { todo!("弱回调 + ephemeron 定点") }
    pub fn process_cross_thread_weakness_if_needed(&mut self) { todo!("跨线程 persistents") }

    /// v8: src/heap/cppgc-internal/marker.cc MarkNotFullyConstructedObjects
    /// Extract previously 队 → MarkNoPush 重标; 并发期 dynamisch 前需暂停并发。
    pub fn mark_not_fully_constructed_objects(&mut self) { todo!("重标先前构造中") }

    /// v8: src/heap/cppgc-internal/marker.cc JoinConcurrentMarkingIfNeeded / NotifyConcurrentMarkingOfWorkIfNeeded / ReEnableConcurrentMarking
    pub fn join_concurrent_marking_if_needed(&mut self) -> bool { todo!("job join") }
    pub fn notify_concurrent_marking_of_work_if_needed(&mut self, _p: TaskPriority) { todo!("有活才通知") }
    pub fn re_enable_concurrent_marking(&mut self) { todo!("本周期未启用才重启") }

    /// v8: src/heap/cppgc-internal/marker.h MarkerBase::WriteBarrierForObject
    /// Dijkstra: !TryMarkAtomic → return; Steele: 未标 → return;
    /// 构造中 → Unmark + not_fully 队; 否则 Dijkstra→write_barrier 队 / Steele→retrace 队。
    pub fn write_barrier_for_object(&mut self, ty: WriteBarrierType, header: &mut HeapObjectHeader) {
        match ty {
            WriteBarrierType::Dijkstra => { if !header.try_mark_atomic() { return; } }
            WriteBarrierType::Steele => { if !header.is_marked() { return; } }
        }
        if header.is_in_construction() {
            header.unmark();
            self.mutator_marking_state.basic.base_not_fully_push(header);
            return;
        }
        match ty {
            WriteBarrierType::Dijkstra => self.mutator_marking_state.basic.write_barrier_push(header),
            WriteBarrierType::Steele => self.mutator_marking_state.retrace_push(header),
        }
    }

    /// v8: IncrementalMarkingStep — GetNextIncrementalStepDuration 后 AdvanceMarkingWithLimits (≤2ms)
    pub fn incremental_marking_step(&mut self, _stack_state: StackState) -> bool { todo!("限 2ms 步进") }
}

/// v8: src/heap/cppgc-internal/marker.h MarkerBase::IncrementalMarkingAllocationObserver
/// current_allocated_size 累计 ≥ 256KB → ScheduleIncrementalMarkingTask。
pub struct IncrementalMarkingAllocationObserver {
    /// v8: marker_
    marker: *mut MarkerBase,
    /// v8: current_allocated_size_
    current_allocated_size: usize,
}

/// v8: src/heap/cppgc-internal/marker.h MarkerBase::PauseConcurrentMarkingScope
/// 构造暂停 (记录 resume_on_exit_ = was_running), 析构按需恢复。
/// v8: src/heap/cppgc-internal/marker.cc MarkerBase::IncrementalMarkingTask (cppgc::Task)
/// Run → IncrementalMarkingStep; 句柄防重入。
/// v8: src/heap/cppgc-internal/marker.cc EnterIncrementalMarkingIfNeeded / ExitIncrementalMarkingIfNeeded
/// 屏障 FlagUpdater Enter/Exit + set_incremental_marking_in_progress; 返回是否增量型。

/// v8: src/heap/cppgc-internal/marker.h Marker
/// 组装: MutatorMarkingVisitor + ConservativeMarkingVisitor + IncrementalMarkingSchedule + ConcurrentMarker。
pub struct Marker {
    /// v8: MarkerBase 基类子对象
    pub base: MarkerBase,
    /// v8: marking_visitor_ / conservative_marking_visitor_ / schedule_ / concurrent_marker_
    pub marking_visitor: MutatorMarkingVisitor,
    pub conservative_marking_visitor: ConservativeMarkingVisitor,
    pub schedule: Box<IncrementalMarkingSchedule>,
    pub concurrent_marker: ConcurrentMarker,
}
`;

writeSrc("v8-heap-cppgc", "marker.rs.skel", body);
console.log(`marker.js: 对数 ${n} 项 (.h 32 + .cc 15 + 实体计数 31) → v8-heap-cppgc/src/marker.rs.skel`);
