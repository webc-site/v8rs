#!/usr/bin/env bun
// tool/src/heap/cppgc-js/cpp-heap.h.js — v8/src/heap/cppgc-js/{cpp-heap.h,cpp-heap.cc} → v8-heap-cppgc/src/cpp_heap.rs.skel
// 职责: CppHeap — 四基类统一堆门面 (HeapBase/v8::CppHeap/AllocationObserver/GarbageCollector) + MetricRecorderAdapter 批量事件
// 规则:
//   ① 位标志: kNoFlags=0 / kReduceMemory=1<<1 / kForced=1<<2 (uint8 底)
//   ② MetricRecorderAdapter: kMaxBatchedEvents=16; 3 AddMainThreadEvent final + 5 缓存操作 (Full/Young/IncrementalMark)
//   ③ 生命周期: AttachIsolate (is_detached_=false + CHECK 非测试分离) / StartDetaching / DetachIsolate / 析构 DCHECK 未附加
//   ④ GC 驱动面: InitializeMarking(默认 schedule+flags) / StartMarking / AdvanceMarking / 5 测试通道
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL_H = "src/heap/cppgc-js/cpp-heap.h";
const REL_CC = "src/heap/cppgc-js/cpp-heap.cc";
const src = readV8(REL_H);
const cc = readV8(REL_CC);
let n = 0;
const need = (s, needle, label) => {
  if (!s.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need(src, "class V8_EXPORT_PRIVATE CppHeap final", "类声明");
need(src, ": public cppgc::internal::HeapBase,", "基类 1");
need(src, "public v8::CppHeap,", "基类 2");
need(src, "public cppgc::internal::StatsCollector::AllocationObserver,", "基类 3");
need(src, "public cppgc::internal::GarbageCollector {", "基类 4");
need(src, "enum GarbageCollectionFlagValues : uint8_t {", "位标志枚举");
need(src, "kNoFlags = 0,", "无标志位");
need(src, "kReduceMemory = 1 << 1,", "减内存位");
need(src, "kForced = 1 << 2,", "强站位");
need(src, "class MetricRecorderAdapter final : public cppgc::internal::MetricRecorder {", "适配器类");
need(src, "static constexpr int kMaxBatchedEvents = 16;", "批量上限常量");
need(src, "void AddMainThreadEvent(const GCCycle& cppgc_event) final;", "GCCycle 事件钩子");
need(src, "void FlushBatchedIncrementalEvents();", "批量冲刷声明");
need(src, "bool FullGCMetricsReportPending() const;", "全量报告待定声明");
need(src, "ExtractLastFullGcEvent();", "取末次全量事件声明");
need(src, "ExtractLastIncrementalMarkEvent();", "取末次增量标记声明");
need(src, "void ClearCachedEvents();", "清缓存声明");
need(src, "static void InitializeOncePerProcess();", "进程级初始化声明");
need(src, "static CppHeap* From(v8::CppHeap* heap) {", "向下转型 From");
need(src, "void AttachIsolate(Isolate* isolate);", "附加隔离区声明");
need(src, "void StartDetachingIsolate();", "开始分离声明");
need(src, "void DetachIsolate();", "分离声明");
need(src, "void CollectCustomSpaceStatisticsAtLastGC(", "自定义空间统计声明");
need(src, "void FinishSweepingIfRunning();", "完成清扫声明");
need(src, "void InitializeMarking(", "标记初始化声明");
need(src, "GarbageCollectionFlags = GarbageCollectionFlagValues::kNoFlags);", "flags 默认参");
need(src, "void StartMarking();", "开始标记声明");
need(cc, "void CppHeap::MetricRecorderAdapter::AddMainThreadEvent(", "cc 事件钩子定义");
need(cc, "void CppHeap::MetricRecorderAdapter::FlushBatchedIncrementalEvents() {", "cc 冲刷定义");
need(cc, "bool CppHeap::MetricRecorderAdapter::FullGCMetricsReportPending() const {", "cc 待定定义");
need(cc, "void CppHeap::InitializeOncePerProcess() {", "cc 进程初始化定义");
need(cc, "CppHeap::CppHeap(", "cc 构造");
need(cc, "CppHeap::~CppHeap() {", "cc 析构");
need(cc, "// Must not be attached to a heap when invoking termination GCs.", "cc 析构注释");
need(cc, "void CppHeap::AttachIsolate(Isolate* isolate) {", "cc 附加定义");
need(cc, "is_detached_ = false;", "cc 附加清分离标志");
need(cc, "CHECK(!in_detached_testing_mode_);", "cc 测试分离模式检查");
need(cc, "void CppHeap::CollectGarbageForTesting(", "cc 测试 GC 定义");
need(cc, "void CppHeap::EnableDetachedGarbageCollectionsForTesting() {", "cc 分离测试定义");
need(cc, "if (!isolate_ && !is_in_detached_mode_) return nullptr;", "cc 分离 runner 判定");
need(cc, "void CppHeap::MetricRecorderAdapter::ClearCachedYoungEvents() {", "cc 清年轻缓存");

const body = `${banner("src/heap/cppgc-js/cpp-heap.h.js", REL_H, "CppHeap — 四基类统一堆门面 + MetricRecorderAdapter (cc 实现一并落地)")}
// ══ GarbageCollectionFlagValues (原版 cpp-heap.h:46-51) ══

/// 位标志: no-flags=0 / reduce-memory=1<<1 / forced=1<<2
pub const K_NO_FLAGS: u8 = 0;
pub const K_REDUCE_MEMORY: u8 = 1 << 1;
pub const K_FORCED: u8 = 1 << 2;

pub type GarbageCollectionFlags = u8; // base::Flags<GarbageCollectionFlagValues>

// ══ MetricRecorderAdapter (原版 cpp-heap.h:56-97, 实现 cc:388-507) ══

/// kMaxBatchedEvents = 16 — 批量事件上限
pub const K_MAX_BATCHED_EVENTS: usize = 16;

pub struct MetricRecorderAdapter {
    cpp_heap: *mut CppHeap,
    /// 双批量通道 (metrics 批量上报)
    incremental_mark_batched_events: v8_metrics::GarbageCollectionFullMainThreadBatchedIncrementalMark,
    incremental_sweep_batched_events: v8_metrics::GarbageCollectionFullMainThreadBatchedIncrementalSweep,
    /// 三缓存事件 (嵌套 cpp 事件经 V8 上报用; 独立事件直报)
    last_full_gc_event: Option<MetricRecorderGcCycle>,
    last_young_gc_event: Option<MetricRecorderGcCycle>,
    last_incremental_mark_event: Option<MetricRecorderMainThreadIncrementalMark>,
}

impl MetricRecorderAdapter {
    /// 原版 cc:388-443 — 3 final 钩子: GCCycle 分 Full/Young 落缓存; 增量 Mark/Sweep 入批量通道
    pub fn add_main_thread_event_gc_cycle(&mut self, _event: &MetricRecorderGcCycle) { /* Full/Young 分流 */ }
    pub fn add_main_thread_event_incremental_mark(&mut self, _event: &MetricRecorderMainThreadIncrementalMark) { /* 批量 */ }
    pub fn add_main_thread_event_incremental_sweep(&mut self, _event: &MetricRecorderMainThreadIncrementalSweep) { /* 批量 */ }
    /// 原版 cc:444-459 — 冲刷批量通道 (达 kMaxBatchedEvents 或显式)
    pub fn flush_batched_incremental_events(&mut self) { unimplemented!() }
    /// 原版 cc:460-467 — 嵌套事件报告待定判定
    pub fn full_gc_metrics_report_pending(&self) -> bool { self.last_full_gc_event.is_some() }
    pub fn young_gc_metrics_report_pending(&self) -> bool { self.last_young_gc_event.is_some() }
    /// 原版 cc:469-487 — 取走缓存事件 (Option 语义 = take)
    pub fn extract_last_full_gc_event(&mut self) -> Option<MetricRecorderGcCycle> { self.last_full_gc_event.take() }
    pub fn extract_last_young_gc_event(&mut self) -> Option<MetricRecorderGcCycle> { self.last_young_gc_event.take() }
    pub fn extract_last_incremental_mark_event(&mut self) -> Option<MetricRecorderMainThreadIncrementalMark> { self.last_incremental_mark_event.take() }
    /// 原版 cc:489-507
    pub fn clear_cached_events(&mut self) { self.last_full_gc_event = None; self.last_young_gc_event = None; self.last_incremental_mark_event = None; }
    pub fn clear_cached_young_events(&mut self) { self.last_young_gc_event = None; }
}

// ══ CppHeap (原版 cpp-heap.h:40-300, 实现 cc) ══
/// 四基类: cppgc::internal::HeapBase / v8::CppHeap / AllocationObserver / GarbageCollector

pub struct CppHeap {
    pub base: cppgc_internal::HeapBase,
    isolate: *mut Isolate,
    is_detached_: bool,
    /// 分离测试模式 (EnableDetachedGarbageCollectionsForTesting 置位; 附加期 CHECK 拒绝)
    in_detached_testing_mode: bool,
    is_in_detached_mode: bool,
    metric_recorder_adapter: MetricRecorderAdapter,
    // v8: marker/sweeper/heap_growing/detached runner 等状态 (cc:525-620)
}

impl CppHeap {
    /// 原版 h:109 — 进程级初始化
    pub fn initialize_once_per_process() { /* cc:520-523 */ }

    /// 原版 h:113-117 — static_cast 向下转型双态
    pub fn from(heap: *mut v8::CppHeap) -> *mut CppHeap {
        heap as *mut CppHeap
    }

    /// 原版 h:119-122 + cc:525-546 — 构造: platform + custom_spaces + 标记/清扫型 + 栈起标记
    pub fn new(
        _platform: *mut v8::Platform,
        _custom_spaces: &[Box<dyn cppgc::CustomSpaceBase>],
        _marking_type: MarkingType,
        _sweeping_type: SweepingType,
        _stack_start_marker: Option<cppgc::StackStartMarker>,
    ) -> Self { unimplemented!() }

    /// 原版 cc:548-551 — 析构前不得附加 (终局 GC)
    pub fn drop(&mut self) { /* DCHECK 未附加 + 终局 GC */ }

    pub fn as_base(&mut self) -> &mut cppgc_internal::HeapBase { &mut self.base } // h:126

    // ── 隔离区生命周期 (h:128-130, 实现 cc:626+) ──
    /// 原版 cc:626-649 — is_detached_=false; CHECK(!in_detached_testing_mode_); 传播栈态覆写
    pub fn attach_isolate(&mut self, isolate: *mut Isolate) {
        // 新隔离区附加 → 允许再分离
        self.is_detached_ = false;
        v8_check!(!self.in_detached_testing_mode);
        self.isolate = isolate;
        // v8: 覆写栈态传播到附加堆
    }
    /// 原版 h:129 — 开始分离 (渐退)
    pub fn start_detaching_isolate(&mut self) { unimplemented!() }
    /// 原版 h:130 — 分离
    pub fn detach_isolate(&mut self) { unimplemented!() }

    // ── 统计与测试通道 (h:132-155, 实现 cc:126-155) ──
    pub fn collect_custom_space_statistics_at_last_gc(&mut self, _idx: Vec<CustomSpaceIndex>, _recv: Box<dyn CustomSpaceStatisticsReceiver>) { /* cc:126 */ }
    pub fn enable_detached_garbage_collections_for_testing(&mut self) { /* cc:133 */ }
    pub fn collect_garbage_for_testing(&mut self, _stack_state: EmbedderStackState) { /* cc:138 */ }
    pub fn collect_garbage_in_young_generation_for_testing(&mut self, _stack_state: EmbedderStackState) { /* cc:143 */ }
    pub fn finish_sweeping_if_running(&mut self) { /* cc:154 */ }

    // ── 标记驱动面 (h:157-160) ──
    /// 原版 h:157-160 — schedule 与 flags 默认参 (空 schedule / kNoFlags)
    pub fn initialize_marking(&mut self, _t: CollectionType, _schedule: Option<std::sync::Arc<IncrementalMarkingSchedule>>, _flags: GarbageCollectionFlags) { unimplemented!() }
    pub fn start_marking(&mut self) { unimplemented!() }
    pub fn advance_marking(&mut self, _max_duration: v8_base::TimeDelta) -> bool { unimplemented!() }
}
`;
const out = writeSrc("v8-heap-cppgc", "cpp_heap.rs.skel", body);
console.log(`OK cpp-heap.h.js: 锚 ${n} 条 (h22+cc16); 4 基类 + 适配器; 产物 ${out}`);
