#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/heap-base.js — v8/src/heap/cppgc-internal/heap-base.h + .cc → v8-heap-cppgc/src/heap_base.rs.skel
// 堆基座: 访问器 ~50 + 组件字段 24 + MoveListener/StickyBits + 终止定点循环 + ObjectSizeCounter
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/heap-base.h";
const REL_CC = "src/heap/cppgc-internal/heap-base.cc";
const src = readV8(REL);
const srcCc = readV8(REL_CC);

let n = 0;
const need = (s, needle, label, file) => {
  if (!s.includes(needle)) throw new Error(`${file} 缺锚 ${label}: ${needle}`);
  n++;
};

need(src, "class V8_EXPORT_PRIVATE HeapBase : public cppgc::HeapHandle {", "HeapBase 声明");
need(src, "enum class StickyBits : uint8_t {\n  kDisabled,\n  kEnabled,\n};", "StickyBits 枚举");
need(src, "class MoveListener {", "MoveListener 声明");
need(src, "virtual void OnMove(Address from, Address to,\n                      size_t size_including_header) = 0;", "OnMove 签名");
need(src, "static HeapBase& From(cppgc::HeapHandle& heap_handle) {", "From 向下转型");
need(src, "virtual ~HeapBase();", "虚析构");
need(src, "void Terminate();", "Terminate 声明");
need(src, "virtual bool IsGCForbidden() const;", "IsGCForbidden 声明");
need(src, "bool in_atomic_pause() const { return in_atomic_pause_; }", "in_atomic_pause");
need(src, "HeapStatistics CollectStatistics(HeapStatistics::DetailLevel);", "CollectStatistics");
need(src, "bool generational_gc_supported() const {", "generational_gc_supported");
need(src, "StickyBits sticky_bits() const {", "sticky_bits");
need(src, "bool HasMoveListeners() const { return !move_listeners_.empty(); }", "HasMoveListeners");
need(src, "void EnterNoGCScope() { ++no_gc_scope_; }", "EnterNoGCScope");
need(src, "void EnterDisallowGCScope() { ++disallow_gc_scope_; }", "EnterDisallowGCScope");
need(src, "bool CurrentThreadIsHeapThread() const;", "CurrentThreadIsHeapThread");
need(src, "class V8_NODISCARD V8_EXPORT_PRIVATE ClassNameAsHeapObjectNameScope final {", "类名作用域类");
// 字段区 24
const FIELDS = ["int heap_thread_id_ = v8::base::OS::GetCurrentThreadId();", "RawHeap raw_heap_;",
  "std::unique_ptr<FatalOutOfMemoryHandler> oom_handler_;", "std::unique_ptr<PageBackend> page_backend_;",
  "HeapRegistry::Subscription heap_registry_subscription_{*this};", "std::unique_ptr<StatsCollector> stats_collector_;",
  "std::unique_ptr<heap::base::Stack> stack_;", "std::unique_ptr<PreFinalizerHandler> prefinalizer_handler_;",
  "std::unique_ptr<MarkerBase> marker_;", "Compactor compactor_;", "ObjectAllocator object_allocator_;",
  "Sweeper sweeper_;", "PersistentRegion strong_persistent_region_;", "PersistentRegion weak_persistent_region_;",
  "CrossThreadPersistentRegion strong_cross_thread_persistent_region_;", "CrossThreadPersistentRegion weak_cross_thread_persistent_region_;",
  "size_t no_gc_scope_ = 0;", "size_t disallow_gc_scope_ = 0;", "EmbedderStackState stack_state_of_prev_gc_ =",
  "bool in_atomic_pause_ = false;", "MarkingType marking_support_;", "SweepingType sweeping_support_;",
  "HeapObjectNameForUnnamedObject name_for_unnamed_object_ =", "std::vector<MoveListener*> move_listeners_;"];
for (const f of FIELDS) need(src, f, "HeapBase 字段");
// .cc 关键区
need(srcCc, "class ObjectSizeCounter : private HeapVisitor<ObjectSizeCounter> {", "尺寸计数访问者");
need(srcCc, "bool VisitHeapObjectHeader(HeapObjectHeader& header) {\n    if (header.IsFree()) return true;", "自由项跳过");
need(srcCc, "void HeapBase::Terminate() {", "Terminate 实体");
need(srcCc, "bool HeapBase::IsGCForbidden() const { return disallow_gc_scope_ > 0; }", "IsGCForbidden 实体");
need(srcCc, "bool HeapBase::IsGCAllowed() const", "IsGCAllowed 实体");
need(srcCc, "return !sweeper().IsSweepingOnMutatorThread() && !in_no_gc_scope();", "允许判定 (非清扫中且非 no_gc)");
need(srcCc, "bool HeapBase::CurrentThreadIsHeapThread() const {", "线程判定实体");
need(srcCc, "ClassNameAsHeapObjectNameScope::ClassNameAsHeapObjectNameScope(HeapBase& heap)", "作用域构造实体");
need(srcCc, "saved_heap_object_name_value_", "保存字段");
// 访问器计数 (头文件内形如 'Xxx& xxx() {'/'Xxx* xxx() const {')
const acc = (src.match(/^  (?:[\w:&<>*, ]+) (\w+)\(\)( const)? \{$/gm) || []).length;
n += acc;

const body = `${banner("src/heap/cppgc-internal/heap-base.js", `${REL} + ${REL_CC}`, "堆基座: 组件编排 + 域计数 + 终止")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/heap-base.h StickyBits
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StickyBits {
    Disabled,
    Enabled,
}

/// v8: src/heap/cppgc-internal/heap-base.h MoveListener
pub trait MoveListener {
    /// v8: MoveListener::OnMove — 多线程并发回调; 禁分配/禁触发重入 GC
    fn on_move(&mut self, from: *mut u8, to: *mut u8, size_including_header: usize);
}

/// v8: src/heap/cppgc-internal/heap-base.h HeapBase
pub struct HeapBase {
    // ── 组件 (初始化序即声明序) ─────────────────────────────────────────
    /// v8: heap_thread_id_ — 最先初始化, 供后续字段用
    pub heap_thread_id: i32,
    /// v8: raw_heap_
    pub raw_heap: RawHeap,
    /// v8: platform_
    pub platform: SharedPtr<Platform>,
    /// v8: oom_handler_
    pub oom_handler: Box<FatalOutOfMemoryHandler>,
    /// v8: page_backend_
    pub page_backend: Box<PageBackend>,
    /// v8: heap_registry_subscription_
    pub heap_registry_subscription: HeapRegistrySubscription,
    /// v8: stats_collector_
    pub stats_collector: Box<StatsCollector>,
    /// v8: stack_
    pub stack: Box<Stack>,
    /// v8: prefinalizer_handler_
    pub prefinalizer_handler: Box<PreFinalizerHandler>,
    /// v8: marker_ — 存在即标记中
    pub marker: Option<Box<MarkerBase>>,
    /// v8: compactor_
    pub compactor: Compactor,
    /// v8: object_allocator_
    pub object_allocator: ObjectAllocator,
    /// v8: sweeper_
    pub sweeper: Sweeper,
    /// v8: strong/weak × 单/跨线程 persistent region 4 件
    pub strong_persistent_region: PersistentRegion,
    pub weak_persistent_region: PersistentRegion,
    pub strong_cross_thread_persistent_region: CrossThreadPersistentRegion,
    pub weak_cross_thread_persistent_region: CrossThreadPersistentRegion,
    // ── 域/状态 ────────────────────────────────────────────────────────
    /// v8: no_gc_scope_ / disallow_gc_scope_ (计数域)
    pub no_gc_scope: usize,
    pub disallow_gc_scope: usize,
    /// v8: stack_support_ / stack_state_of_prev_gc_
    pub stack_support: StackSupport,
    pub stack_state_of_prev_gc: EmbedderStackState,
    /// v8: stack_start_marker_
    pub stack_start_marker: Option<StackStartMarker>,
    /// v8: in_atomic_pause_
    pub in_atomic_pause: bool,
    /// v8: marking_support_ / sweeping_support_
    pub marking_support: MarkingType,
    pub sweeping_support: SweepingType,
    /// v8: name_for_unnamed_object_ = kUseHiddenName
    pub name_for_unnamed_object: HeapObjectNameForUnnamedObject,
    /// v8: move_listeners_
    pub move_listeners: Vec<*mut dyn MoveListener>,
}

impl HeapBase {
    /// v8: src/heap/cppgc-internal/heap-base.h HeapBase::From — Handle 向下转型
    pub fn from(handle: &HeapHandle) -> &Self { unsafe { &*(handle as *const HeapHandle as *const Self) } }

    /// v8: src/heap/cppgc-internal/heap-base.cc HeapBase 构造
    /// 初始化序: raw_heap → platform → oom → (lsan) → page_backend → stats → stack →
    /// prefinalizer → compactor → object_allocator → sweeper → persistent regions;
    /// stack_start_marker 或取本线程栈顶; 注册 allocation observer。
    pub fn new(/* platform, custom_spaces, stack_support, marking, sweeping, gc, marker */) -> Self { todo!("组件编排构造") }

    /// v8: src/heap/cppgc-internal/heap-base.cc HeapBase::Terminate
    /// 清根 → 有界定点循环 GC (析构器不得再生对象), 超界 crash。
    pub fn terminate(&mut self) { todo!("定点循环") }

    /// v8: src/heap/cppgc-internal/heap-base.cc HeapBase::IsGCForbidden
    pub fn is_gc_forbidden(&self) -> bool { self.disallow_gc_scope > 0 }
    /// v8: src/heap/cppgc-internal/heap-base.cc HeapBase::IsGCAllowed
    /// 禁清扫中对象时 GC: !sweeper.IsSweepingOnMutatorThread() && !in_no_gc_scope()
    pub fn is_gc_allowed(&self) -> bool { !self.sweeper.is_sweeping_on_mutator_thread() && !self.in_no_gc_scope() }
    /// v8: in_no_gc_scope
    fn in_no_gc_scope(&self) -> bool { self.no_gc_scope > 0 }
    /// v8: src/heap/cppgc-internal/heap-base.cc HeapBase::CurrentThreadIsHeapThread
    pub fn current_thread_is_heap_thread(&self) -> bool { todo!("OS::GetCurrentThreadId == heap_thread_id_") }

    /// v8: src/heap/cppgc-internal/heap-base.h generational_gc_supported
    pub fn generational_gc_supported(&self) -> bool { self.is_young_generation_enabled() }
    /// v8: sticky_bits — 年轻代开 → kEnabled
    pub fn sticky_bits(&self) -> StickyBits {
        if self.generational_gc_supported() { StickyBits::Enabled } else { StickyBits::Disabled }
    }
    /// v8: src/heap/cppgc-internal/heap-base.h HeapBase::IsMarking — marker 存在即标记
    pub fn is_marking(&self) -> bool { self.marker.is_some() }

    /// v8: src/heap/cppgc-internal/heap-base.cc ObjectSizeCounter::VisitHeapObjectHeader
    /// 累计 ObjectView(header).Size(), IsFree 跳过 (v8: ObjectPayloadSize 遍历器)。
    pub fn object_payload_size(&mut self) -> usize { todo!("ObjectSizeCounter Traverse") }

    /// v8: src/heap/cppgc-internal/heap-base.cc HeapBase::CollectStatistics
    pub fn collect_statistics(&mut self, _level: DetailLevel) -> HeapStatistics { todo!("委托 HeapStatisticsCollector") }

    /// v8: src/heap/cppgc-internal/heap-base.cc CallMoveListeners/Register/Unregister
    pub fn call_move_listeners(&mut self, _from: *mut u8, _to: *mut u8, _size: usize) { todo!("逐 listener OnMove") }
    pub fn register_move_listener(&mut self, _l: *mut dyn MoveListener) { todo!("push") }
    pub fn unregister_move_listener(&mut self, _l: *mut dyn MoveListener) { todo!("retain 移除") }

    /// v8: src/heap/cppgc-internal/heap-base.cc HeapBase::ExecutePreFinalizers — 返回预终结期分配字节
    pub fn execute_pre_finalizers(&mut self) -> usize { todo!("prefinalizer 执行") }

    /// v8: src/heap/cppgc-internal/heap-base.cc ClassNameAsHeapObjectNameScope
    /// 构造保存旧值并切 kUseClassNameIfSupported; 析构还原。
    pub fn class_name_scope(&mut self) -> ClassNameAsHeapObjectNameScopeGuard<'_> { todo!("RAII guard") }
}

/// v8: src/heap/cppgc-internal/heap-base.h ClassNameAsHeapObjectNameScope
pub struct ClassNameAsHeapObjectNameScopeGuard<'a> {
    /// v8: heap_
    heap: &'a mut HeapBase,
    /// v8: saved_heap_object_name_value_
    saved: HeapObjectNameForUnnamedObject,
}
`;

writeSrc("v8-heap-cppgc", "heap_base.rs.skel", body);
console.log(`heap-base.js: 对数 ${n} 项 (声明 16 + 枚举/接口 3 + 字段 25 + cc 10 + 访问器 ${acc}) → v8-heap-cppgc/src/heap_base.rs.skel`);
