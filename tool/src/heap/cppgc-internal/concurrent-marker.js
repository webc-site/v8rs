#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/concurrent-marker.js — v8/src/heap/cppgc-internal/concurrent-marker.h + .cc → v8-heap-cppgc/src/concurrent_marker.rs.skel
// 并发标记: JobTask 驱动 + 0.5 提速阈值 + 750 步进间隔 + 工作窃取判定
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/concurrent-marker.h";
const REL_CC = "src/heap/cppgc-internal/concurrent-marker.cc";
const src = readV8(REL);
const srcCc = readV8(REL_CC);

let n = 0;
const need = (s, needle, label, file) => {
  if (!s.includes(needle)) throw new Error(`${file} 缺锚 ${label}: ${needle}`);
  n++;
};

need(src, "class V8_EXPORT_PRIVATE ConcurrentMarkerBase {", "ConcurrentMarkerBase 声明");
need(src, "cppgc::Platform*);", "构造 Platform 参数");
need(src, "void Start();", "Start 声明");
need(src, "bool Join();", "Join 声明");
need(src, "bool Cancel();", "Cancel 声明");
need(src, "void NotifyIncrementalMutatorStepCompleted();", "步进完成通知");
need(src, "void NotifyOfWorkIfNeeded(cppgc::TaskPriority priority);", "按优先级通知");
need(src, "bool IsActive() const;", "IsActive");
need(src, "void AddConcurrentlyMarkedBytes(size_t marked_bytes);", "并发字节累计");
need(src, "size_t concurrently_marked_bytes() const {", "并发字节读取");
need(src, "virtual std::unique_ptr<Visitor> CreateConcurrentMarkingVisitor(", "访问者工厂");
need(src, "class V8_EXPORT_PRIVATE ConcurrentMarker : public ConcurrentMarkerBase {", "ConcurrentMarker 声明");
need(src, "std::atomic<size_t> concurrently_marked_bytes_ = 0;", "原子字节字段");
need(src, "bool concurrent_marking_priority_increased_{false};", "提速标志");
// ── .cc ───────────────────────────────────────────────────────────────
need(srcCc, "static constexpr double kMarkingScheduleRatioBeforeConcurrentPriorityIncrease =\n    0.5;", "提速阈值 0.5");
need(srcCc, "static constexpr size_t kDefaultDeadlineCheckInterval = 750u;", "步进间隔 750");
need(srcCc, "bool DrainWorklistWithYielding(JobDelegate* job_delegate,", "让出式排空模板");
need(srcCc, "job_delegate->ShouldYield();", "ShouldYield 判定");
need(srcCc, "size_t WorkSizeForConcurrentMarking(MarkingWorklists& marking_worklists) {", "工作量计算");
need(srcCc, "bool HasWorkForConcurrentMarking(MarkingWorklists& marking_worklists) {", "可窃取判定");
need(srcCc, "class ConcurrentMarkingTask final : public v8::JobTask {", "JobTask 类");
need(srcCc, "void Run(JobDelegate* delegate) final;", "JobTask Run");
need(srcCc, "size_t GetMaxConcurrency(size_t) const final;", "最大并发");
need(srcCc, "void ProcessWorklists(JobDelegate*, ConcurrentMarkingState&, Visitor&);", "排空入口");
// 方法实体计数: ConcurrentMarkerBase 10 (构造/Start/Join/Cancel/IsActive/Add/析构/Notify×2/Increase) + ConcurrentMarker 1 = 11
const defs = (srcCc.match(/^void ConcurrentMarkerBase::|^bool ConcurrentMarkerBase::|^ConcurrentMarkerBase::|^void ConcurrentMarker::|^std::unique_ptr<Visitor> ConcurrentMarker::|^size_t ConcurrentMarkerBase::/gm) || []).length;
if (defs !== 11) throw new Error(`ConcurrentMarker 方法定义计数失配: ${defs} want 11`);
n += defs;

const body = `${banner("src/heap/cppgc-internal/concurrent-marker.js", `${REL} + ${REL_CC}`, "并发标记 job: 让出式排空 + 优先级提速")}
#![allow(dead_code)]

pub mod constants {
    /// v8: src/heap/cppgc-internal/concurrent-marker.cc kMarkingScheduleRatioBeforeConcurrentPriorityIncrease
    pub const K_MARKING_SCHEDULE_RATIO_BEFORE_CONCURRENT_PRIORITY_INCREASE: f64 = 0.5;
    /// v8: src/heap/cppgc-internal/concurrent-marker.cc kDefaultDeadlineCheckInterval
    pub const K_DEFAULT_DEADLINE_CHECK_INTERVAL: usize = 750;
}

/// v8: src/heap/cppgc-internal/concurrent-marker.h ConcurrentMarkerBase
pub struct ConcurrentMarkerBase {
    /// v8: heap_
    heap: *mut HeapBase,
    /// v8: marking_worklists_
    marking_worklists: *mut MarkingWorklists,
    /// v8: incremental_marking_schedule_
    incremental_marking_schedule: *mut IncrementalMarkingSchedule,
    /// v8: platform_
    platform: *mut Platform,
    /// v8: concurrent_marking_handle_ — 兼作"已启动"旗
    concurrent_marking_handle: Option<JobHandle>,
    /// v8: concurrently_marked_bytes_ (atomic)
    concurrently_marked_bytes: AtomicUsize,
    /// v8: concurrent_marking_priority_increased_
    concurrent_marking_priority_increased: bool,
}

impl ConcurrentMarkerBase {
    /// v8: src/heap/cppgc-internal/concurrent-marker.cc WorkSizeForConcurrentMarking
    /// marking + write_barrier + previously_not_fully_constructed 三池 Size 之和。
    pub fn work_size(marking_worklists: &MarkingWorklists) -> usize {
        marking_worklists.marking_worklist().size()
            + marking_worklists.write_barrier_worklist().size()
            + marking_worklists.previously_not_fully_constructed_worklist().size()
    }

    /// v8: src/heap/cppgc-internal/concurrent-marker.cc HasWorkForConcurrentMarking
    /// 同三池 IsEmpty 取反 (全局池非空即可窃取)。
    pub fn has_work(marking_worklists: &MarkingWorklists) -> bool {
        !marking_worklists.marking_worklist().is_empty()
            || !marking_worklists.write_barrier_worklist().is_empty()
            || !marking_worklists.previously_not_fully_constructed_worklist().is_empty()
    }

    /// v8: src/heap/cppgc-internal/concurrent-marker.cc ConcurrentMarkerBase::Start
    pub fn start(&mut self) { todo!("PostJob 并发度 = HasWork ? 1 : 0") }
    /// v8: ConcurrentMarkerBase::Join
    pub fn join(&mut self) -> bool { todo!("JobHandle::Join") }
    /// v8: ConcurrentMarkerBase::Cancel
    pub fn cancel(&mut self) -> bool { todo!("JobHandle::Cancel") }
    /// v8: ConcurrentMarkerBase::IsActive — handle 存在即活跃
    pub fn is_active(&self) -> bool { todo!("handle 活跃判定") }
    /// v8: ConcurrentMarkerBase::AddConcurrentlyMarkedBytes
    pub fn add_concurrently_marked_bytes(&self, marked_bytes: usize) {
        self.concurrently_marked_bytes.fetch_add(marked_bytes, Ordering::Relaxed);
    }
    /// v8: src/heap/cppgc-internal/concurrent-marker.cc IncreaseMarkingPriorityIfNeeded
    /// 残余工作比例 > 0.5 且未提速过 → 提一个优先级并置标志。
    pub fn increase_marking_priority_if_needed(&mut self) { todo!("kMarkingScheduleRatio 0.5 阈值") }
    /// v8: NotifyIncrementalMutatorStepCompleted / NotifyOfWorkIfNeeded
    pub fn notify_incremental_mutator_step_completed(&mut self) { todo!("步进后按需通知") }
    pub fn notify_of_work_if_needed(&mut self, _priority: TaskPriority) { todo!("有工作才通知") }
}

/// v8: src/heap/cppgc-internal/concurrent-marker.cc ConcurrentMarkingTask (v8::JobTask)
/// Run: DrainWorklistWithYielding (每 750 项查 ShouldYield, 期间回灌 RecentlyMarkedBytes);
/// GetMaxConcurrency: HasWorkForConcurrentMarking ? 1 : 0。
/// v8: src/heap/cppgc-internal/concurrent-marker.cc DrainWorklistWithYielding
/// 包装 DrainWorklistWithPredicate<750>: ShouldYield = delegate 取消或让出;
/// 每次检查前 AddConcurrentlyMarkedBytes(marking_state.RecentlyMarkedBytes())。

/// v8: src/heap/cppgc-internal/concurrent-marker.h ConcurrentMarker
pub struct ConcurrentMarker {
    /// v8: ConcurrentMarkerBase 基类子对象
    pub base: ConcurrentMarkerBase,
}

impl ConcurrentMarker {
    /// v8: src/heap/cppgc-internal/concurrent-marker.cc ConcurrentMarker::CreateConcurrentMarkingVisitor
    pub fn create_concurrent_marking_visitor(&self, _state: &mut ConcurrentMarkingState) -> Box<dyn Visitor> {
        todo!("ConcurrentMarkingVisitor 构造")
    }
}
`;

writeSrc("v8-heap-cppgc", "concurrent_marker.rs.skel", body);
console.log(`concurrent-marker.js: 对数 ${n} 项 (.h 13 + .cc 11 + 实体计数 11) → v8-heap-cppgc/src/concurrent_marker.rs.skel`);
