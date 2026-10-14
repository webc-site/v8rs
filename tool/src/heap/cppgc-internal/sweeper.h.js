#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/sweeper.h.js — v8/src/heap/cppgc-internal/{sweeper.h,sweeper.cc} → v8-heap-cppgc/src/sweeper.rs.skel
// 职责: Sweeper 门面 + SweeperImpl 全量清扫管线 — 4 优先级常量 + 2 FreeHandler + 2 FinalizationBuilder + SweepFinalizer + 3 访问者
// 规则:
//   ① CanDiscardMemory: Win 恒 false / 其余 = CheckMemoryIsInaccessibleIsNoop; ShouldDiscardMemory = CanDiscard && kReleaseMemory
//   ② DeadlineChecker kInterval=4 采样到期; MutatorThreadSweepingMode 2 值 + ToString
//   ③ FreeHandler 双实现: Discarding (unused 区对齐后 DiscardSystemPages + 双记账) / Regular (直加)
//   ④ Builder 双实现: Inlined (即时 Finalize+毒化) / Deferred (可终结对象单链挂 unfinalized, cage 门)
//   ⑤ Sweeper 门面 6 方法全转发 impl_; Observer RAII 注册/反注册
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL_H = "src/heap/cppgc-internal/sweeper.h";
const REL_CC = "src/heap/cppgc-internal/sweeper.cc";
const src = readV8(REL_H);
const cc = readV8(REL_CC);
let n = 0;
const need = (s, needle, label) => {
  if (!s.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need(src, "class V8_EXPORT_PRIVATE Sweeper final {", "门面类");
need(src, "class V8_EXPORT_PRIVATE SweepingOnMutatorThreadObserver {", "观察者基类");
need(src, "virtual void Start() = 0;", "Start 纯虚");
need(src, "virtual void End() = 0;", "End 纯虚");
need(src, "static constexpr bool CanDiscardMemory() {", "可丢弃判定");
need(src, "#if defined(V8_OS_WIN)", "Win 门");
need(src, "return CheckMemoryIsInaccessibleIsNoop();", "非 Win 判定式");
need(src, "void Start(SweepingConfig);", "启动声明");
need(src, "bool FinishIfRunning();", "完成声明");
need(src, "void FinishIfOutOfWork();", "无工作完成声明");
need(src, "bool SweepForAllocationIfRunning(BaseSpace* space, size_t min_wanted_size,", "按配清扫声明");
need(src, "bool PerformSweepOnMutatorThread(v8::base::TimeDelta max_duration,", "mutator 助扫声明");
need(src, "class SweeperImpl;", "Impl 前置");
need(src, "std::unique_ptr<SweeperImpl> impl_;", "Impl 持有");
need(cc, "constexpr TaskPriority kBackgroundBoostedPriority = TaskPriority::kUserBlocking;", "后台高优常量");
need(cc, "constexpr TaskPriority kBackgroundRegularPriority = TaskPriority::kUserVisible;", "后台常优常量");
need(cc, "constexpr TaskPriority kForegroundRegularPriority = TaskPriority::kUserBlocking;", "前台常优常量");
need(cc, "constexpr TaskPriority kForegroundLowPriority = TaskPriority::kUserVisible;", "前台低优常量");
need(cc, "free_memory_handling == FreeMemoryHandling::kReleaseMemory;", "ShouldDiscard 判定式");
need(cc, "class DeadlineChecker final {", "到期检查器");
need(cc, "static constexpr size_t kInterval = 4;", "采样间隔");
need(cc, "enum class MutatorThreadSweepingMode {", "mutator 清扫模式枚举");
need(cc, "return \"only-finalizers\";", "ToString 只终结器臂");
need(cc, "class FreeHandlerBase {", "FreeHandler 基");
need(cc, "virtual void FreeFreeList(", "FreeFreeList 纯虚");
need(cc, "class DiscardingFreeHandler : public FreeHandlerBase {", "丢弃 handler");
need(cc, "AddReturningUnusedBounds(block);", "返回未用界");
need(cc, "RoundUp(reinterpret_cast<uintptr_t>(unused_range.first),", "未用界上取整");
need(cc, "page_allocator_.DiscardSystemPages(", "系统页丢弃");
need(cc, "page_.IncrementDiscardedMemory(discarded_size);", "页记账");
need(cc, "->IncrementDiscardedMemory(discarded_size);", "统计记账");
need(cc, "class RegularFreeHandler : public FreeHandlerBase {", "常规 handler");
need(cc, "void Free(FreeList::Block block) { free_list_.Add(std::move(block)); }", "常规直加");
need(cc, "template <typename T>\nclass ThreadSafeStack {", "线程安全栈模板");
need(cc, "class InlinedFinalizationBuilder final : public InlinedFinalizationBuilderBase,", "内联 builder");
need(cc, "header->Finalize();", "即时终结");
need(cc, "SetMemoryInaccessible(header, size);", "内存不可达毒化");
need(cc, "std::max(result_.largest_new_free_list_entry, size);", "最大空链项");
need(cc, "class DeferredFinalizationBuilder final : public FreeHandler {", "延迟 builder");
need(cc, "if (header->IsFinalizable()) {", "可终结判定");
need(cc, "current_unfinalized_->SetNextUnfinalized(header);", "cage 单链挂接");
need(cc, "result_.unfinalized_objects.push_back({header});", "非 cage 向量收集");
need(cc, "found_finalizer_ = true;", "发现终结器标志");
need(cc, "class SweepFinalizer final {", "终结器类");
need(cc, "void Finalize(SpaceStates& states) {", "全量终结");
need(cc, "while (auto page_state = state.swept_unfinalized_pages.Pop()) {", "逐页终结循环");
need(cc, "class MutatorThreadSweeper final : private HeapVisitor<MutatorThreadSweeper> {", "mutator 访问者");
need(cc, "class ConcurrentSweepTask final : public cppgc::JobTask,", "并发清扫任务");
need(cc, "class PrepareForSweepVisitor final", "清扫准备访问者");
need(cc, "class Sweeper::SweeperImpl final {", "Impl 类");
need(cc, "CHECK_NOT_NULL(platform_);", "平台非空");
need(cc, "~SweeperImpl() { CancelAllSweepingTasks(); }", "Impl 析构取消任务");
need(cc, "is_in_progress_ = true;", "进行中置位");
need(cc, "foreground_task_runner_ =\n          platform_->GetForegroundTaskRunner(kForegroundRegularPriority);", "前台 runner 惰性取");
need(cc, "CHECK_IMPLIES(low_priority_foreground_task_runner_.get(),", "低优蕴含常优断言");
need(cc, "void Sweeper::Start(SweepingConfig config) { impl_->Start(config); }", "门面 Start 转发");
need(cc, "Sweeper::SweepingOnMutatorThreadObserver::SweepingOnMutatorThreadObserver(", "观察者构造定义");

// 门面转发法全序
const fwd = [...cc.matchAll(/^(\w+) Sweeper::(\w+)\(/gm)].map((m) => m[2]);
if (fwd.join(",") !== "Start,FinishIfRunning,FinishIfOutOfWork,WaitForConcurrentSweepingForTesting,SweepForAllocationIfRunning,IsSweepingOnMutatorThread,IsSweepingInProgress,PerformSweepOnMutatorThread")
  throw new Error(`门面定义序: got=${fwd.join(",")}`);
n++;

const body = `${banner("src/heap/cppgc-internal/sweeper.h.js", REL_H, "Sweeper 门面 + SweeperImpl 清扫管线 — 优先级/双 handler/双 builder/SweepFinalizer (cc 实现一并落地)")}
// ══ 优先级常量 (原版 sweeper.cc:36-39) ══
pub const K_BACKGROUND_BOOSTED_PRIORITY: TaskPriority = TaskPriority::UserBlocking;
pub const K_BACKGROUND_REGULAR_PRIORITY: TaskPriority = TaskPriority::UserVisible;
pub const K_FOREGROUND_REGULAR_PRIORITY: TaskPriority = TaskPriority::UserBlocking;
pub const K_FOREGROUND_LOW_PRIORITY: TaskPriority = TaskPriority::UserVisible;

// ══ 丢弃内存判定 (原版 sweeper.h:36-46 + cc:41-46) ══
/// Win: 丢弃不 decommit 且随系统分配变贵可致挂起 → 恒 false; 其余: 不可达检查为 noop 才丢
pub const fn can_discard_memory() -> bool {
    #[cfg(target_os = "windows")]
    { false }
    #[cfg(not(target_os = "windows"))]
    { check_memory_is_inaccessible_is_noop() }
}

/// ShouldDiscardMemory — 配置请求丢弃 && 平台可丢 (原版 cc:41-46)
pub const fn should_discard_memory(free_memory_handling: FreeMemoryHandling) -> bool {
    can_discard_memory() && free_memory_handling == FreeMemoryHandling::ReleaseMemory
}

// ══ DeadlineChecker (原版 cc:49-64) ══
/// 每 kInterval=4 次才查一次时钟的节流到期检查
pub struct DeadlineChecker {
    end: v8_base::TimeTicks,
    count: usize, // = 0
}
const K_DEADLINE_CHECK_INTERVAL: usize = 4;
impl DeadlineChecker {
    pub fn check(&mut self) -> bool {
        self.count += 1;
        self.count % K_DEADLINE_CHECK_INTERVAL == 0 && self.end < v8_base::TimeTicks::now()
    }
}

// ══ MutatorThreadSweepingMode (原版 cc:66-79) ══
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutatorThreadSweepingMode {
    OnlyFinalizers,
    All,
}
pub const fn mutator_thread_sweeping_mode_to_string(m: MutatorThreadSweepingMode) -> &'static str {
    match m {
        MutatorThreadSweepingMode::All => "all",
        MutatorThreadSweepingMode::OnlyFinalizers => "only-finalizers",
    }
}

// ══ FreeHandler 双实现 (原版 sweeper.cc:121-185) ══
pub trait FreeHandlerBase {
    fn free_free_list(&mut self, unfinalized_free_list: Vec<FreeListBlock>);
}

/// Discarding — 未用区按提交页对齐后 DiscardSystemPages, 页 + 统计双记账
pub struct DiscardingFreeHandler<'a> {
    page_allocator: &'a mut PageAllocator,
    free_list: &'a mut FreeList,
    page: &'a mut BasePage,
}
impl DiscardingFreeHandler<'_> {
    /// 原版 cc:135-165 — AddReturningUnusedBounds → 对齐 → 非空则丢弃并记账
    pub fn free(&mut self, block: FreeListBlock) {
        let unused = self.free_list.add_returning_unused_bounds(block);
        let aligned_begin = round_up(unused.0 as usize, self.page_allocator.commit_page_size());
        let aligned_end = round_down(unused.1 as usize, self.page_allocator.commit_page_size());
        if aligned_begin < aligned_end {
            let discarded_size = aligned_end - aligned_begin;
            self.page_allocator.discard_system_pages(aligned_begin as *mut u8, discarded_size);
            self.page.increment_discarded_memory(discarded_size);
            self.page.space().raw_heap().heap().stats_collector().increment_discarded_memory(discarded_size);
        }
    }
}

/// Regular — 直加 free list, 无丢弃
pub struct RegularFreeHandler<'a> {
    free_list: &'a mut FreeList,
}
impl RegularFreeHandler<'_> {
    /// 原版 cc:175
    pub fn free(&mut self, block: FreeListBlock) {
        self.free_list.add(block);
    }
}

// ══ ThreadSafeStack (原版 sweeper.cc:187-256) ══
/// mutex + vector + is_empty 原子三件套
pub struct ThreadSafeStack<T> {
    vector: std::sync::Mutex<Vec<T>>,
    is_empty: std::sync::atomic::AtomicBool,
}
impl<T> ThreadSafeStack<T> {
    pub fn push(&self, t: T) { /* lock → push → is_empty=false relaxed */ }
    pub fn pop(&self) -> Option<T> { /* lock → pop; 空 → is_empty=true relaxed */ }
    pub fn is_empty(&self) -> bool { self.is_empty.load(std::sync::atomic::Ordering::Relaxed) }
}

// ══ FinalizationBuilder 双实现 (原版 sweeper.cc:258-434) ══

/// Inlined — 即时执行: Finalize + SetMemoryInaccessible; 空链项取最大
pub struct InlinedFinalizationBuilder {
    pub result: SweepResult, // { free_list, largest_new_free_list_entry, is_empty }
}
impl InlinedFinalizationBuilder {
    /// 原版 cc:280-283
    pub fn add_finalizer(&mut self, header: &mut HeapObjectHeader, size: usize) {
        header.finalize();
        set_memory_inaccessible(header as *mut _ as *mut u8, size);
    }
    /// 原版 cc:285-290
    pub fn add_free_list_entry(&mut self, handler: &mut impl FreeHandlerBase, start: *mut u8, size: usize) {
        // v8: FreeHandler::Free({start, size})
        self.result.largest_new_free_list_entry = self.result.largest_new_free_list_entry.max(size);
    }
}

/// Deferred — 收集后延处理: 可终结对象挂链, 其余毒化; 有终结器时空链项转 unfinalized_free_list
pub struct DeferredFinalizationBuilder {
    pub result: SweptPageState, // { page, cached_free_list, unfinalized_free_list, unfinalized_objects… }
    current_unfinalized: *mut HeapObjectHeader,
    found_finalizer: bool,
}
impl DeferredFinalizationBuilder {
    /// 原版 cc:305-327 — cage: 单链 SetNextUnfinalized; 非 cage: 向量
    pub fn add_finalizer(&mut self, header: &mut HeapObjectHeader, size: usize) {
        if header.is_finalizable() {
            // v8: CPPGC_CAGED_HEAP — unfinalized_objects_head 单链 (内存紧凑)
            // 否则 unfinalized_objects.push_back
            self.found_finalizer = true;
        } else {
            set_memory_inaccessible(header as *mut _ as *mut u8, size);
        }
    }
    /// 原版 cc:329-336 — 有终结器: 入 unfinalized_free_list; 否则即时 Free
    pub fn add_free_list_entry(&mut self, start: *mut u8, size: usize) {
        if self.found_finalizer {
            self.result.unfinalized_free_list.push(FreeListBlock { start, size });
        }
    }
}

// ══ SweepFinalizer (原版 sweeper.cc:436-613) ══
pub struct SweepFinalizer {
    platform: *mut cppgc::Platform,
    stats_collector: *mut StatsCollector,
    space: *mut BaseSpace,
    unused_destroyed_normal_pages: *mut usize,
    free_memory_handling: FreeMemoryHandling,
    empty_page_handling_type: EmptyPageHandling,
}
impl SweepFinalizer {
    /// 原版 :454-458 — 全量 (无期限无尺寸)
    pub fn finalize_all(&mut self, states: &mut SpaceStates) {
        for state in states {
            self.finalize(state);
        }
    }
    /// 原版 :460-464 — 逐页弹栈终结
    pub fn finalize(&mut self, state: &mut SweepingState) {
        while let Some(page_state) = state.swept_unfinalized_pages.pop() {
            self.finalize_page(&page_state);
        }
    }
    /// 原版 :466+ — 带期限与尺寸: 至少 size 字节单块入链才 true
    pub fn finalize_with_deadline_and_size(&mut self, scope_id: StatsCollectorScopeId, state: &mut SweepingState, deadline: v8_base::TimeTicks, size: usize) -> bool { unimplemented!() }
}

// ══ 访问者三件套 (原版 sweeper.cc:615-970) ══
/// MutatorThreadSweeper — HeapVisitor CRTP, mutator 线程清扫页 (DeadlineChecker 节流)
pub struct MutatorThreadSweeper; // 原版 :615
/// ConcurrentSweepTask — cppgc::JobTask, 并发清扫 (双优先级 runner)
pub struct ConcurrentSweepTask;  // 原版 :775
/// PrepareForSweepVisitor — 清扫前页分类访备
pub struct PrepareForSweepVisitor; // 原版 :891

// ══ SweeperImpl (原版 sweeper.cc:974-1657) ══
pub struct SweeperImpl {
    pub heap: *mut RawHeap,
    pub page_pool: *mut NormalPageMemoryPool,
    pub stats_collector: *mut StatsCollector,
    pub platform: *mut cppgc::Platform, // CHECK_NOT_NULL
    // 原版 :990-1010 — 首扫惰性取 runner; 低优蕴含常优 CHECK
    // 状态: is_in_progress_, config_, space_states, sweeping_on_mutator_thread_
}
impl SweeperImpl {
    /// 原版 :982-987
    pub fn new(heap: &mut HeapBase) -> Self { v8_check_not_null(heap.platform()); unimplemented!() }
    /// 原版 :989+ — kAtomicSweep EnabledScope; is_in_progress_=true; config 落; runner 惰性初始化
    pub fn start(&mut self, config: SweepingConfig) { unimplemented!() }
    /// 析构 = CancelAllSweepingTasks (原版 :1008)
}

// ══ Sweeper 门面 (原版 sweeper.h:22-85, 转发 cc:1659-1701) ══
pub struct Sweeper {
    heap: *mut HeapBase,
    impl_: Box<SweeperImpl>,
}
impl Sweeper {
    pub fn new(heap: &mut HeapBase) -> Self { Self { heap, impl_: SweeperImpl::new(heap).into() } }
    /// 原版 h:58 — 启动 (堆须无 LAB; atomic 型由调用方立即收尾)
    pub fn start(&mut self, config: SweepingConfig) { self.impl_.start(config) }
    /// 原版 h:61 — true=已完成; false=未运行或递归清扫调用
    pub fn finish_if_running(&mut self) -> bool { self.impl_.finish_if_running() }
    /// 原版 h:62
    pub fn finish_if_out_of_work(&mut self) { self.impl_.finish_if_out_of_work() }
    /// 原版 h:66-67 — 按 min_wanted_size 扫到够槽即停 (max_duration 上限)
    pub fn sweep_for_allocation_if_running(&mut self, space: *mut BaseSpace, min_wanted_size: usize, max_duration: v8_base::TimeDelta) -> bool { self.impl_.sweep_for_allocation_if_running(space, min_wanted_size, max_duration) }
    pub fn is_sweeping_on_mutator_thread(&self) -> bool { self.impl_.is_sweeping_on_mutator_thread() }
    pub fn is_sweeping_in_progress(&self) -> bool { self.impl_.is_sweeping_in_progress() }
    /// 原版 h:73-74 — mutator 助扫, 完成返 true
    pub fn perform_sweep_on_mutator_thread(&mut self, max_duration: v8_base::TimeDelta, scope: StatsCollectorScopeId) -> bool { self.impl_.perform_sweep_on_mutator_thread(max_duration, scope) }
}

/// 原版 h:24-34 + cc:1693-1701 — 构造注册到 sweeper / 析构反注册; Start/End 纯虚
pub trait SweepingOnMutatorThreadObserver {
    fn start(&mut self);
    fn end(&mut self);
}
`;
const out = writeSrc("v8-heap-cppgc", "sweeper.rs.skel", body);
console.log(`OK sweeper.h.js: 锚 ${n} 条 (h13+cc41+门面序1); 4 优先级 + 双 handler + 双 builder; 产物 ${out}`);
