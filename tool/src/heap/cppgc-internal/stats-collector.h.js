#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/stats-collector.h.js — v8/src/heap/cppgc-internal/{stats-collector.h,stats-collector.cc} → v8-heap-cppgc/src/stats_collector.rs.skel
// 职责: StatsCollector — 4 X-macro 域枚举展开 + Event 双缓冲事件 + AllocationObserver 5 钩子 + 全量 Notify 生命周期
// 规则:
//   ① 4 X-macro: HISTOGRAM_SCOPES 7 + SCOPES 34 + HISTOGRAM_CONCURRENT 3 + CONCURRENT 4 → Rust enum + 计数常量
//   ② 枚举计数器: kNumHistogramScopeIds 夹在两组间; ScopeId = 直方图 7 + 普通 34 + kNumScopeIds
//   ③ Event: scope_data[kNumHistogramScopeIds] + concurrent_scope_data[...]{0} + 10 标量字段 (-1 哨兵)
//   ④ GetScopeName: Major → "CppGC.<name>" / Minor → "CppGC.<name>.Minor" 双态串
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL_H = "src/heap/cppgc-internal/stats-collector.h";
const REL_CC = "src/heap/cppgc-internal/stats-collector.cc";
const src = readV8(REL_H);
const cc = readV8(REL_CC);
let n = 0;
const need = (s, needle, label) => {
  if (!s.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

// ── X-macro 域解析 (真展开 4 组 V(...) 列表) ──
const macroList = (name) => {
  const m = src.match(new RegExp(`#define ${name}\\(V\\)([\\s\\S]*?)\\n\\n`));
  if (!m) throw new Error(`X-macro ${name} 未命中`);
  return [...m[1].matchAll(/V\((\w+)\)/g)].map((x) => x[1]);
};
const HIST = macroList("CPPGC_FOR_ALL_HISTOGRAM_SCOPES");
const SCOPES = macroList("CPPGC_FOR_ALL_SCOPES");
const HCONC = macroList("CPPGC_FOR_ALL_HISTOGRAM_CONCURRENT_SCOPES");
const CONC = macroList("CPPGC_FOR_ALL_CONCURRENT_SCOPES");
if (HIST.length !== 6) throw new Error(`HISTOGRAM_SCOPES 计数: got=${HIST.length} want=6`);
if (SCOPES.length !== 34) throw new Error(`SCOPES 计数: got=${SCOPES.length} want=34`);
if (HCONC.length !== 3) throw new Error(`HISTOGRAM_CONCURRENT 计数: got=${HCONC.length} want=3`);
if (CONC.length !== 4) throw new Error(`CONCURRENT_SCOPES 计数: got=${CONC.length} want=4`);
n += 4;
const NUM_HIST = HIST.length;               // 7
const NUM_SCOPE_IDS = HIST.length + SCOPES.length; // 41
const NUM_HCONC = HCONC.length;             // 3
const NUM_CONC_IDS = HCONC.length + CONC.length;   // 7

need(src, "enum ScopeId {", "ScopeId 枚举");
need(src, "kNumHistogramScopeIds,", "直方图计数器");
need(src, "enum ConcurrentScopeId {", "ConcurrentScopeId 枚举");
need(src, "kNumHistogramConcurrentScopeIds,", "并发直方图计数器");
need(src, "kNumConcurrentScopeIds", "并发总数");
need(src, "struct Event final {", "Event 结构");
need(src, "v8::base::TimeDelta scope_data[kNumHistogramScopeIds];", "scope 数组");
need(src, "v8::base::AtomicWord concurrent_scope_data[kNumHistogramConcurrentScopeIds]{", "并发 scope 数组");
need(src, "size_t epoch = -1;", "epoch 哨兵");
need(src, "size_t object_size_before_sweep_bytes = -1;", "对象尺寸哨兵");
need(src, "size_t memory_size_before_sweep_bytes = -1;", "内存尺寸哨兵");
need(src, "class AllocationObserver {", "观察者基类");
need(src, "virtual void AllocatedObjectSizeIncreased(size_t) {}", "对象增钩子");
need(src, "virtual void AllocatedObjectSizeDecreased(size_t) {}", "对象减钩子");
need(src, "virtual void ResetAllocatedObjectSize(size_t) {}", "对象重置钩子");
need(src, "virtual void AllocatedSizeIncreased(size_t) {}", "空间增钩子");
need(src, "virtual void AllocatedSizeDecreased(size_t) {}", "空间减钩子");
need(src, "static constexpr size_t kAllocationThresholdBytes = 1024;", "阈值常量");
need(src, "enum class GarbageCollectionState : uint8_t {", "GC 状态枚举");
need(src, "kNotRunning,", "状态未运行");
need(src, "kUnmarking,", "状态清标");
need(src, "kMarking,", "状态标记");
need(src, "kSweeping", "状态清扫");
need(src, "enum TraceCategory { kEnabled, kDisabled };", "trace 双态");
need(src, "enum ScopeContext { kMutatorThread, kConcurrentThread };", "上下文双态");
need(src, "std::vector<AllocationObserver*> allocation_observers_;", "观察者向量");
need(src, "Event current_;", "当前事件");
need(src, "Event previous_;", "前一事件");
need(src, 'return type == CollectionType::kMajor ? "CppGC." #name', "GetScopeName Major 臂");
need(src, ': "CppGC." #name ".Minor";', "GetScopeName Minor 臂");
need(cc, "StatsCollector::StatsCollector(Platform* platform)", "cc 构造");
need(cc, "void StatsCollector::NotifyUnmarkingStarted(", "cc 清标始");
need(cc, "void StatsCollector::NotifyMarkingStarted(", "cc 标记始");
need(cc, "void StatsCollector::NotifyMarkingCompleted(", "cc 标记毕");
need(cc, "void StatsCollector::NotifySweepingCompleted(", "cc 清扫毕");

const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
const enumBody = (names) => names.map((x) => `    ${snake(x)},`).join("\n");
const nameArms = (names) => names.map((x) => `            Self::${snake(x)} => if type_ == CollectionType::Major { "CppGC.${x}" } else { "CppGC.${x}.Minor" },`).join("\n");

const body = `${banner("src/heap/cppgc-internal/stats-collector.h.js", REL_H, "StatsCollector — 4 X-macro 域展开 + Event 双缓冲 + AllocationObserver + Notify 生命周期 (cc 实现一并落地)")}
// ══ ScopeId (原版 stats-collector.h:91-97, X-macro 展开: 直方图 7 + 普通 ${SCOPES.length}) ══

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ScopeId {
${enumBody(HIST)}
    /// kNumHistogramScopeIds — 直方图域计数 (夹在两组之间)
    NumHistogramScopeIds,
${enumBody(SCOPES)}
    /// kNumScopeIds
    NumScopeIds,
}
pub const K_NUM_HISTOGRAM_SCOPE_IDS: usize = ${NUM_HIST};
pub const K_NUM_SCOPE_IDS: usize = ${NUM_SCOPE_IDS};

// ══ ConcurrentScopeId (原版 :99-106, 直方图并发 ${HCONC.length} + 并发 ${CONC.length}) ══

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConcurrentScopeId {
${enumBody(HCONC)}
    /// kNumHistogramConcurrentScopeIds
    NumHistogramConcurrentScopeIds,
${enumBody(CONC)}
    /// kNumConcurrentScopeIds
    NumConcurrentScopeIds,
}
pub const K_NUM_HISTOGRAM_CONCURRENT_SCOPE_IDS: usize = ${NUM_HCONC};
pub const K_NUM_CONCURRENT_SCOPE_IDS: usize = ${NUM_CONC_IDS};

// ══ GarbageCollectionState (原版 :345-350) ══
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum GarbageCollectionState {
    #[default]
    NotRunning,
    Unmarking,
    Marking,
    Sweeping,
}

// ══ TraceCategory / ScopeContext (原版 :170-171) ══
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceCategory { Enabled, Disabled }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeContext { MutatorThread, ConcurrentThread }

// ══ Event (原版 :116-137) ══
/// 单轮 GC 事件 POD; 历史事件全量, 当前事件可部分填充。
#[derive(Clone)]
pub struct Event {
    pub scope_data: [v8_base::TimeDelta; K_NUM_HISTOGRAM_SCOPE_IDS],
    pub concurrent_scope_data: [v8_base::AtomicWord; K_NUM_HISTOGRAM_CONCURRENT_SCOPE_IDS], // {0}
    pub epoch: usize,                    // = -1 哨兵
    pub collection_type: CollectionType, // = kMajor
    pub marking_type: MarkingType,       // = kAtomic
    pub sweeping_type: SweepingType,     // = kAtomic
    pub is_forced_gc: IsForcedGc,        // = kNotForced
    pub marked_bytes: usize,             // = 0
    pub object_size_before_sweep_bytes: usize,  // = -1 哨兵
    pub memory_size_before_sweep_bytes: usize,  // = -1 哨兵
}

// ══ GetScopeName (原版 :139-167, constexpr 双态) ══
impl ScopeId {
    /// Major → "CppGC.<name>" / Minor → "CppGC.<name>.Minor"
    pub const fn get_scope_name(id: Self, type_: CollectionType) -> &'static str {
        match id {
${nameArms(HIST)}
${nameArms(SCOPES)}
            _ => "",
        }
    }
}

impl ConcurrentScopeId {
    /// 同上双态 (并发域)
    pub const fn get_scope_name(id: Self, type_: CollectionType) -> &'static str {
        match id {
${nameArms(HCONC)}
${nameArms(CONC)}
            _ => "",
        }
    }
}

// ══ AllocationObserver (原版 :243-262) ══
pub trait AllocationObserver {
    /// 可触发 GC; 增减双向报告 (观察者自选绝对值或增量视角)
    fn allocated_object_size_increased(&mut self, _bytes: usize) {}
    fn allocated_object_size_decreased(&mut self, _bytes: usize) {}
    /// 精确尺寸已知时 (标记毕 marked==allocated); 禁同步触发 GC
    fn reset_allocated_object_size(&mut self, _bytes: usize) {}
    /// 页级内存增减; 禁触发 GC
    fn allocated_size_increased(&mut self, _bytes: usize) {}
    fn allocated_size_decreased(&mut self, _bytes: usize) {}
}

// ══ StatsCollector (原版 :84-423) ══
pub struct StatsCollector {
    /// 通知阈值 — 观察者走虚调用, 低于该尺寸不值得通知
    pub allocation_observers: Vec<*mut dyn AllocationObserver>,
    allocation_observer_deleted: bool,
    gc_state: GarbageCollectionState, // = kNotRunning
    current: Event,                   // 本轮填充中
    previous: Event,                  // 上轮 (NotifySweepingCompleted 时落)
    metric_recorder: Option<Box<dyn MetricRecorder>>,
    // 原版 :371-394 私有标量区 (标记后分配/时间戳/safepoint 计数等)
    allocated_bytes_since_end_of_marking: i64,   // = 0, 可为负 (显式释放已标对象)
    time_of_last_end_of_marking: v8_base::TimeTicks,
    allocated_bytes_since_safepoint: i64,
    explicitly_freed_bytes_since_safepoint: i64,
    marked_bytes_so_far: usize,
    memory_allocated_bytes: i64,
    memory_freed_bytes_since_end_of_marking: i64,
    discarded_bytes: std::sync::atomic::AtomicUsize, // {0}
    platform: *mut Platform,
}

/// kAllocationThresholdBytes = 1024 — 观察者通知阈值
pub const K_ALLOCATION_THRESHOLD_BYTES: usize = 1024;

impl StatsCollector {
    /// 原版 h:279-280 + cc 构造 — 平台指针注入 (TRACE_EVENT_* 用)
    pub fn new(platform: *mut Platform) -> Self { unimplemented!() }

    /// 原版 h:282-283 — 注册/反注册 (仅启动/关停期)
    pub fn register_observer(&mut self, _o: *mut dyn AllocationObserver) { unimplemented!() }
    pub fn unregister_observer(&mut self, _o: *mut dyn AllocationObserver) { unimplemented!() }

    // ── Notify 生命周期 (声明 h:285-306, 实现 cc) ──
    /// 新 GC 周期 (分代 major 专用, 可选)
    pub fn notify_unmarking_started(&mut self, _t: CollectionType) { unimplemented!() }
    /// minor 周期 (或无分代时的 major)
    pub fn notify_marking_started(&mut self, _t: CollectionType, _m: MarkingType, _f: IsForcedGc) { unimplemented!() }
    /// 标记毕
    pub fn notify_marking_completed(&mut self, _marked_bytes: usize) { unimplemented!() }
    /// 清扫毕 = 周期毕
    pub fn notify_sweeping_completed(&mut self, _t: SweepingType) { unimplemented!() }
    /// 分配通知 (可能触发 GC)
    pub fn notify_allocation(&mut self, _bytes: usize) { unimplemented!() }
    /// 显式释放通知
    pub fn notify_explicit_free(&mut self, _bytes: usize) { unimplemented!() }
    /// 保守收集 safepoint (GC 可能时才可调)
    pub fn notify_safe_point_for_conservative_collection(&mut self) { unimplemented!() }
    pub fn notify_safe_point_for_testing(&mut self) { unimplemented!() }

    // ── 查询 (h:308-322) ──
    pub fn allocated_memory_size(&self) -> usize { unimplemented!() }
    pub fn allocated_object_size(&self) -> usize { unimplemented!() }
    /// 累计标记字节 (分代跨轮累计); 禁标记中调
    pub fn marked_bytes(&self) -> usize { unimplemented!() }
    /// 本轮标记字节; 仅 GC 周期内调
    pub fn marked_bytes_on_current_cycle(&self) -> usize { unimplemented!() }
    pub fn marking_time(&self) -> v8_base::TimeDelta { unimplemented!() }
    pub fn get_recent_allocation_speed_in_bytes_per_ms(&self) -> f64 { unimplemented!() }
    pub fn get_previous_event_for_testing(&self) -> &Event { &self.previous }

    // ── 内存与 discard (h:325-333) ──
    pub fn notify_allocated_memory(&mut self, _b: i64) { unimplemented!() }
    pub fn notify_freed_memory(&mut self, _b: i64) { unimplemented!() }
    pub fn increment_discarded_memory(&mut self, _b: usize) { unimplemented!() }
    pub fn decrement_discarded_memory(&mut self, _b: usize) { unimplemented!() }
    pub fn reset_discarded_memory(&mut self) { unimplemented!() }
    pub fn discarded_memory_size(&self) -> usize { unimplemented!() }
    pub fn resident_memory_size(&self) -> usize { unimplemented!() }

    /// 原版 :415-422 — 下标迭代 (允许回调内 push_back), 有删除标记则尾部压缩
    pub fn for_all_allocation_observers(&mut self, _cb: impl FnMut(&mut dyn AllocationObserver)) { unimplemented!() }
}
`;
const out = writeSrc("v8-heap-cppgc", "stats_collector.rs.skel", body);
console.log(`OK stats-collector.h.js: 锚 ${n} 条 (h30+cc5+宏计数4); X-macro ${NUM_HIST}+${SCOPES.length}/${NUM_HCONC}+${CONC.length}; ScopeId ${NUM_SCOPE_IDS} ConcurrentScopeId ${NUM_CONC_IDS}; 产物 ${out}`);
