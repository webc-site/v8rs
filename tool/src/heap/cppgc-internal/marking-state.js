#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/marking-state.js — v8/src/heap/cppgc-internal/marking-state.h + .cc → v8-heap-cppgc/src/marking_state.rs.skel
// 标记状态四层: Base(MarkAndPush/PushMarked) → Basic(9 worklist 本地端 + 字节记账) → Mutator(重trace缓存8) / Concurrent(延迟字节)
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/marking-state.h";
const REL_CC = "src/heap/cppgc-internal/marking-state.cc";
const src = readV8(REL);
const srcCc = readV8(REL_CC);

let n = 0;
const need = (s, needle, label, file) => {
  if (!s.includes(needle)) throw new Error(`${file} 缺锚 ${label}: ${needle}`);
  n++;
};

// ── ① MarkingStateBase ────────────────────────────────────────────────
need(src, "class MarkingStateBase {", "MarkingStateBase 声明");
need(src, "inline void MarkAndPush(const void*, TraceDescriptor);", "MarkAndPush 声明");
need(src, "inline void PushMarked(HeapObjectHeader&, TraceDescriptor desc);", "PushMarked 声明");
need(src, "V8_EXPORT_PRIVATE virtual void Publish();", "Publish 声明");
need(src, "if (header.IsInConstruction<AccessMode::kAtomic>()) {\n    not_fully_constructed_worklist_.Push<AccessMode::kAtomic>(&header);", "构造中分流");
need(src, "DCHECK(!header.IsFree<AccessMode::kAtomic>());", "自由项禁标");
need(src, "return header.TryMarkAtomic();", "TryMarkAtomic 委托");
need(src, "GlobalGCInfoTable::GCInfoFromIndex(header.GetGCInfoIndex()).trace", "trace 查表");
// ── ② BasicMarkingState ───────────────────────────────────────────────
need(src, "class BasicMarkingState : public MarkingStateBase {", "BasicMarkingState 声明");
need(src, "inline void RegisterWeakReferenceIfNeeded(const void*, TraceDescriptor,", "RegisterWeakReferenceIfNeeded");
need(src, "inline void ProcessWeakContainer(const void*, TraceDescriptor, WeakCallback,", "ProcessWeakContainer 声明");
need(src, "inline void ProcessEphemeron(const void*, const void*, TraceDescriptor,", "ProcessEphemeron 声明");
need(src, "inline void AccountMarkedBytes(const HeapObjectHeader&);", "AccountMarkedBytes(header)");
need(src, "size_t RecentlyMarkedBytes() {", "RecentlyMarkedBytes");
need(src, "return marked_bytes_ - std::exchange(last_marked_bytes_, marked_bytes_);", "差值消费");
need(src, "if (V8_UNLIKELY(heap::base::Stack::IsOnStack(slot))) return;", "栈槽排除");
need(src, "size_t marked_bytes_ = 0;", "marked_bytes_ 字段");
need(src, "bool discovered_new_ephemeron_pairs_ = false;", "ephemeron 发现标志");
need(src, "heap::base::CachedUnorderedMap<BasePage*, int64_t, v8::base::hash<BasePage*>>", "页字节映射");
need(src, "key_considered_as_live", "ephemeron 键存活判定");
need(src, "discovered_ephemeron_pairs_worklist_.Push({key, value, value_desc});", "ephemeron 入队");
need(src, "header.IsLargeObject<AccessMode::kAtomic>()", "大页字节取 PayloadSize");
need(src, "marked_bytes_map_[base_page] += static_cast<int64_t>(marked_bytes);", "页映射累计");
// ── ③ Mutator / Concurrent ────────────────────────────────────────────
need(src, "class MutatorMarkingState final : public BasicMarkingState {", "MutatorMarkingState 声明");
need(src, "static constexpr size_t kMaxCacheSize = 8;", "重trace缓存 8");
need(src, "last_used_index_ = (last_used_index_ + 1) % kMaxCacheSize;", "环形缓存推进");
need(src, "void FlushNotFullyConstructedObjects();", "FlushNotFully 声明");
need(src, "void FlushDiscoveredEphemeronPairs();", "FlushEphemeron 声明");
need(src, "class ConcurrentMarkingState final : public BasicMarkingState {", "ConcurrentMarkingState 声明");
need(src, "marked_bytes_ -= deferred_bytes;", "延迟字节扣减");
// ── ④ 自由函数模板 ────────────────────────────────────────────────────
need(src, "bool DrainWorklistWithPredicate(", "排空谓词模板");
need(src, "if (V8_UNLIKELY(--processed_callback_count == 0)) {", "间隔检查递减");
need(src, "void DynamicallyTraceMarkedObject(Visitor& visitor,", "动态 trace 模板");
// ── ⑤ .cc ────────────────────────────────────────────────────────────
need(srcCc, "void MarkingStateBase::Publish() { marking_worklist_.Publish(); }", "Base Publish");
need(srcCc, "for (const auto& entry : marked_bytes_map_.Take()) {", "发布时页字节回灌");
need(srcCc, "entry.first->IncrementMarkedBytes(static_cast<size_t>(entry.second));", "页字节累计");
need(srcCc, "not_fully_constructed_worklist_.Extract<AccessMode::kAtomic>();", "Flush 提取");
need(srcCc, "ephemeron_pairs_for_processing_worklist_.Merge(", "ephemeron 合并");
need(srcCc, "DCHECK_EQ(last_marked_bytes_, marked_bytes_);", "析构对账断言");
// 方法实体计数: Publish×3 + Flush×2 = 5 (构造/析构无返回类型前缀不计)
const defs = (srcCc.match(/^(?:void|bool|size_t) (?:MarkingStateBase|BasicMarkingState|MutatorMarkingState)::/gm) || []).length;
if (defs !== 5) throw new Error(`marking-state.cc 实体计数失配: ${defs} want 5`);
n += defs;

const body = `${banner("src/heap/cppgc-internal/marking-state.js", `${REL} + ${REL_CC}`, "标记状态: worklist 本地端聚合 + 标记字节记账")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/marking-state.h MarkingStateBase
pub struct MarkingStateBase {
    /// v8: heap_
    pub heap: *mut HeapBase,
    /// v8: marking_worklist_ (Local)
    marking_worklist: MarkingWorklistLocal,
    /// v8: not_fully_constructed_worklist_ (引用)
    not_fully_constructed_worklist: *mut ExternalMarkingWorklist,
}

impl MarkingStateBase {
    /// v8: src/heap/cppgc-internal/marking-state.h MarkingStateBase::MarkAndPush
    /// 构造中 (atomic) → not_fully_constructed; 否则 MarkNoPush 成功 → PushMarked。
    pub fn mark_and_push(&mut self, object: *const core::ffi::c_void, desc: TraceDescriptor) {
        let header = HeapObjectHeader::from_object(desc.base_object_payload);
        if header.is_in_construction() {
            unsafe { (*self.not_fully_constructed_worklist).push(header as *mut _); }
        } else if self.mark_no_push(header) {
            self.push_marked(header, desc);
        }
    }

    /// v8: MarkNoPush — TryMarkAtomic (自由项 DCHECK 禁标)
    fn mark_no_push(&self, header: &HeapObjectHeader) -> bool {
        debug_assert!(!header.is_free());
        header.try_mark_atomic()
    }

    /// v8: MarkAndPush(header) — TraceDescriptor 取 GCInfo 表 trace 回调
    pub fn mark_and_push_header(&mut self, header: &HeapObjectHeader) {
        todo!("GCInfoFromIndex(header.GetGCInfoIndex()).trace 构造 desc")
    }

    /// v8: PushMarked — push {ObjectStart, callback}
    fn push_marked(&mut self, header: &HeapObjectHeader, desc: TraceDescriptor) {
        debug_assert!(header.is_marked());
        debug_assert!(!header.is_in_construction());
        self.marking_worklist.push(desc);
    }

    /// v8: src/heap/cppgc-internal/marking-state.cc MarkingStateBase::Publish
    pub fn publish(&mut self) { self.marking_worklist.publish(); }
}

/// v8: src/heap/cppgc-internal/marking-state.h BasicMarkingState
pub struct BasicMarkingState {
    /// v8: MarkingStateBase 基类子对象
    pub base: MarkingStateBase,
    /// v8: previously_not_fully_constructed_worklist_ 等 9 个 Local/引用
    previously_not_fully_constructed_worklist: PreviouslyNotFullyConstructedLocal,
    weak_container_callback_worklist: WeakCallbackLocal,
    parallel_weak_callback_worklist: WeakCallbackLocal,
    weak_custom_callback_worklist: WeakCustomCallbackLocal,
    write_barrier_worklist: WriteBarrierLocal,
    concurrent_marking_bailout_worklist: ConcurrentMarkingBailoutLocal,
    discovered_ephemeron_pairs_worklist: EphemeronPairsLocal,
    ephemeron_pairs_for_processing_worklist: EphemeronPairsLocal,
    weak_containers_worklist: *mut ExternalMarkingWorklist,
    /// v8: movable_slots_worklist_ — 存在即压缩启用
    movable_slots_worklist: Option<MovableReferencesLocal>,
    /// v8: marked_bytes_ / last_marked_bytes_
    marked_bytes: usize,
    last_marked_bytes: usize,
    /// v8: in_ephemeron_processing_ / discovered_new_ephemeron_pairs_ / in_atomic_pause_
    in_ephemeron_processing: bool,
    discovered_new_ephemeron_pairs: bool,
    in_atomic_pause: bool,
    /// v8: marked_bytes_map_ (BasePage* → i64)
    marked_bytes_map: CachedUnorderedMap<*mut BasePage, i64>,
}

impl BasicMarkingState {
    /// v8: src/heap/cppgc-internal/marking-state.h RegisterWeakReferenceIfNeeded
    /// 已标记且非构造中 → 跳过 (WriteBarrier 保活); 否则 parallel_weak 入队。
    pub fn register_weak_reference_if_needed(&mut self, _object: *const core::ffi::c_void, desc: TraceDescriptor, weak_callback: WeakCallback, parameter: *const core::ffi::c_void) {
        let header = HeapObjectHeader::from_object(desc.base_object_payload);
        if !header.is_in_construction() && header.is_marked() { return; }
        self.parallel_weak_callback_worklist.push(WeakCallbackItem { callback: weak_callback, parameter });
    }

    /// v8: src/heap/cppgc-internal/marking-state.h ProcessWeakContainer
    /// 构造中 → not_fully; RegisterWeakContainer; MarkNoPush 失败即止;
    /// desc.callback 有 → PushMarked, 无 (弱容器免trace) → AccountMarkedBytes。
    pub fn process_weak_container(&mut self, object: *const core::ffi::c_void, desc: TraceDescriptor, callback: WeakCallback, data: *const core::ffi::c_void) {
        let _ = (object, desc, callback, data);
        todo!("弱容器四步")
    }

    /// v8: src/heap/cppgc-internal/marking-state.h ProcessEphemeron
    /// 键构造中: in_atomic_pause_ 决定活性; 键活 → 标值或直调 value_desc.callback;
    /// 否则 {key,value,value_desc} 入 discovered 队 + discovered_new_ephemeron_pairs_=true。
    pub fn process_ephemeron(&mut self, key: *const core::ffi::c_void, value: *const core::ffi::c_void, value_desc: TraceDescriptor) {
        let _ = (key, value, value_desc);
        todo!("ephemeron 键值判定")
    }

    /// v8: src/heap/cppgc-internal/marking-state.h RegisterMovableReference
    /// 无压缩工作表 → 空操作; 出笼/栈上槽 → 排除; 其余 movable 入队。
    pub fn register_movable_reference(&mut self, slot: *mut *const core::ffi::c_void) {
        if self.movable_slots_worklist.is_none() { return; }
        if Stack::is_on_stack(slot.cast()) { return; }
        todo!("movable_slots_worklist_->Push(slot)")
    }

    /// v8: AccountMarkedBytes — 大页取 PayloadSize, 常规取 AllocatedSize; 双计 (总数+页映射)
    pub fn account_marked_bytes(&mut self, header: &HeapObjectHeader) {
        let bytes = if header.is_large_object() {
            unsafe { (*LargePage::from(BasePage::from_payload(header as *const _ as *mut _))).payload_size() }
        } else {
            header.allocated_size()
        };
        self.marked_bytes += bytes;
        let page = BasePage::from_payload(header as *const _ as *mut core::ffi::c_void);
        *self.marked_bytes_map.entry(page).or_insert(0) += bytes as i64;
    }
    /// v8: RecentlyMarkedBytes — 消费式差值
    pub fn recently_marked_bytes(&mut self) -> usize {
        core::mem::replace(&mut self.last_marked_bytes, self.marked_bytes);
        self.marked_bytes - self.last_marked_bytes
    }

    /// v8: src/heap/cppgc-internal/marking-state.cc BasicMarkingState::Publish
    /// 9 队 Publish + marked_bytes_map_.Take() 逐页 IncrementMarkedBytes。
    pub fn publish(&mut self) {
        self.base.publish();
        self.previously_not_fully_constructed_worklist.publish();
        self.weak_container_callback_worklist.publish();
        self.parallel_weak_callback_worklist.publish();
        self.weak_custom_callback_worklist.publish();
        self.write_barrier_worklist.publish();
        self.concurrent_marking_bailout_worklist.publish();
        self.discovered_ephemeron_pairs_worklist.publish();
        self.ephemeron_pairs_for_processing_worklist.publish();
        if let Some(m) = &mut self.movable_slots_worklist { m.publish(); }
        for (page, bytes) in self.marked_bytes_map.take() {
            unsafe { (*page).increment_marked_bytes(bytes as usize); }
        }
    }
}

/// v8: src/heap/cppgc-internal/marking-state.h MutatorMarkingState
pub struct MutatorMarkingState {
    /// v8: BasicMarkingState 基类子对象
    pub basic: BasicMarkingState,
    /// v8: recently_retraced_weak_containers_ (kMaxCacheSize=8 环形)
    recently_retraced_weak_containers: RecentlyRetracedWeakContainers,
    /// v8: retrace_marked_objects_worklist_ (Local)
    retrace_marked_objects_worklist: RetraceMarkedObjectsLocal,
}

pub mod constants {
    /// v8: src/heap/cppgc-internal/marking-state.h MutatorMarkingState::RecentlyRetracedWeakContainers::kMaxCacheSize
    pub const K_MAX_CACHE_SIZE: usize = 8;
}

/// v8: MutatorMarkingState::RecentlyRetracedWeakContainers — 环形 8 项, Contains 线性, Insert 取模推进
pub struct RecentlyRetracedWeakContainers {
    cache: Vec<*const HeapObjectHeader>,
    last_used_index: usize,
}

impl MutatorMarkingState {
    /// v8: src/heap/cppgc-internal/marking-state.cc FlushNotFullyConstructedObjects
    /// Extract(atomic) → 逐项 MarkNoPush 成功入 previously 队。
    pub fn flush_not_fully_constructed_objects(&mut self) { todo!("Extract + 重标") }
    /// v8: src/heap/cppgc-internal/marking-state.cc FlushDiscoveredEphemeronPairs
    /// Publish discovered → 非空则 Merge 进 processing。
    pub fn flush_discovered_ephemeron_pairs(&mut self) { todo!("Publish + Merge") }
    /// v8: src/heap/cppgc-internal/marking-state.cc MutatorMarkingState::Publish — Basic::Publish + retrace Publish
    pub fn publish(&mut self) { todo!("两级发布") }
    /// v8: ReTraceMarkedWeakContainer — Insert 缓存 + retrace 入队
    pub fn re_trace_marked_weak_container(&mut self, _visitor: &mut Visitor, header: &HeapObjectHeader) { todo!("缓存+入队") }
    /// v8: DynamicallyMarkAddress — ObjectHeaderFromInnerAddress + MarkNoPush + 入队
    pub fn dynamically_mark_address(&mut self, _address: *const u8) { todo!("动态标记") }
    /// v8: InvokeWeakRootsCallbackIfNeeded — 根弱回调立即执行 (LivenessBrokerFactory::Create)
    pub fn invoke_weak_roots_callback_if_needed(&mut self, _desc: TraceDescriptor, weak_callback: WeakCallback, parameter: *const core::ffi::c_void) {
        weak_callback(LivenessBroker::default(), parameter);
    }
    /// v8: IsMarkedWeakContainer — weak_containers 含 && 未在重trace缓存
    pub fn is_marked_weak_container(&mut self, _header: &HeapObjectHeader) -> bool { todo!("双判定") }
}

/// v8: src/heap/cppgc-internal/marking-state.h ConcurrentMarkingState
pub struct ConcurrentMarkingState {
    /// v8: BasicMarkingState 基类子对象
    pub basic: BasicMarkingState,
}

impl ConcurrentMarkingState {
    /// v8: AccountDeferredMarkedBytes — 总数与页映射同步扣减 (延迟 trace 回收)
    pub fn account_deferred_marked_bytes(&mut self, base_page: *mut BasePage, deferred_bytes: usize) {
        debug_assert!(deferred_bytes <= self.basic.marked_bytes);
        self.basic.marked_bytes -= deferred_bytes;
        *self.basic.marked_bytes_map.entry(base_page).or_insert(0) -= deferred_bytes as i64;
    }
}

/// v8: src/heap/cppgc-internal/marking-state.h DrainWorklistWithPredicate
/// 模板参数: kDeadlineCheckInterval + ShouldYield + CreateStatsScope + ProcessWorklistItem;
/// 局部空且全局空 → true; 先查一次 yield; 循环 Pop 处理, 每 kInterval 复查。
/// v8: src/heap/cppgc-internal/marking-state.h DynamicallyTraceMarkedObject
/// 断言非构造中且已标记 → header.TraceImpl(visitor)。
`;

writeSrc("v8-heap-cppgc", "marking_state.rs.skel", body);
console.log(`marking-state.js: 对数 ${n} 项 (.h 27 + .cc 6 + 实体计数 5) → v8-heap-cppgc/src/marking_state.rs.skel`);
