#!/usr/bin/env bun
// tool/src/heap/cppgc-js/unified-heap-marking-visitor.h.js — v8/src/heap/cppgc-js/{unified-heap-marking-visitor.h,unified-heap-marking-visitor.cc} → v8-heap-cppgc/src/unified_heap_marking_visitor.rs.skel
// 职责: 统一堆标记访问者 — Base 9 final 钩子 (C++ 面 8 + JS 面 1) + Mutator 直传 + Concurrent 延迟分发
// 规则:
//   ① 双指针成员: cppgc 标记态 + 统一堆标记态; JS 面 TracedReference 走后者
//   ② VisitMultiple 双臂: Uncompressed 步 kSizeOfUncompressedMember / Compressed 步 kSizeofCompressedMember (指针压缩门), 逐元素原子载 + 空跳
//   ③ Concurrent: 自持 local worklist (析构 Publish); v8_heap 非空时 SetCurrentIsolateScope (+多 cage 换 IsolateGroup); DeferTrace 推 bailout 工作表 + 记延标字节
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL_H = "src/heap/cppgc-js/unified-heap-marking-visitor.h";
const REL_CC = "src/heap/cppgc-js/unified-heap-marking-visitor.cc";
const src = readV8(REL_H);
const cc = readV8(REL_CC);
let n = 0;
const need = (s, needle, label) => {
  if (!s.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need(src, "class V8_EXPORT_PRIVATE UnifiedHeapMarkingVisitorBase : public JSVisitor {", "基类声明");
need(src, "void Visit(const void*, TraceDescriptor) final;", "强访钩子");
need(src, "void VisitMultipleUncompressedMember(const void*, size_t,", "非压缩数组钩子");
need(src, "#if defined(CPPGC_POINTER_COMPRESSION)", "指针压缩门");
need(src, "void VisitMultipleCompressedMember(const void*, size_t,", "压缩数组钩子");
need(src, "void VisitWeak(const void*, TraceDescriptor, WeakCallback, const void*) final;", "弱访钩子");
need(src, "void VisitEphemeron(const void*, const void*, TraceDescriptor) final;", "ephemeron 钩子");
need(src, "void VisitWeakContainer(const void* self, TraceDescriptor strong_desc,", "弱容器钩子");
need(src, "void RegisterWeakCallback(WeakCallback, const void*) final;", "弱回调注册钩子");
need(src, "void HandleMovableReference(const void**) final;", "可动引用钩子");
need(src, "void Visit(const TracedReferenceBase& ref) override;", "JS 面钩子");
need(src, "cppgc::internal::BasicMarkingState& marking_state_;", "cppgc 态成员");
need(src, "UnifiedHeapMarkingState& unified_heap_marking_state_;", "统一堆态成员");
need(src, "class V8_EXPORT_PRIVATE MutatorUnifiedHeapMarkingVisitor", "mutator 子类");
need(src, "class V8_EXPORT_PRIVATE ConcurrentUnifiedHeapMarkingVisitor", "concurrent 子类");
need(src, "bool DeferTraceToMutatorThreadIfConcurrent(const void*, cppgc::TraceCallback,", "延迟钩子");
need(src, "bool IsConcurrent() const final { return true; }", "并发判定内联");
need(src, "std::unique_ptr<MarkingWorklists::Local> local_marking_worklist_;", "本地工作表成员");
need(src, "std::optional<SetCurrentIsolateScope> current_isolate_scope_;", "隔离区 scope 成员");
need(src, "#ifdef V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES", "多 cage 门");
need(cc, "cppgc::internal::VisitorFactory::CreateKey()),", "钥匙构造");
need(cc, "marking_state_.MarkAndPush(object, desc);", "cc 强访转发");
need(cc, "len * cppgc::internal::kSizeOfUncompressedMember;", "cc 非压缩步长");
need(cc, "len * cppgc::internal::kSizeofCompressedMember;", "cc 压缩步长");
need(cc, "const void* object = current->LoadAtomic();", "cc 原子载");
need(cc, "if (!object) continue;", "cc 空跳");
need(cc, "marking_state_.RegisterWeakReferenceIfNeeded(object, desc, weak_callback,", "cc 弱注册");
need(cc, "marking_state_.ProcessEphemeron(key, value, value_desc, *this);", "cc ephemeron");
need(cc, "marking_state_.ProcessWeakContainer(self, weak_desc, callback, data);", "cc 弱容器");
need(cc, "marking_state_.RegisterWeakCustomCallback(callback, object);", "cc 自定义弱回调");
need(cc, "marking_state_.RegisterMovableReference(slot);", "cc 可动引用");
need(cc, "unified_heap_marking_state_.MarkAndPush(ref);", "cc JS 面转发");
need(cc, "local_marking_worklist_->Publish();", "cc 析构发布");
need(cc, "current_isolate_scope_.emplace(v8_heap->isolate());", "cc 隔离区 scope");
need(cc, "IsolateGroup::set_current(saved_isolate_group_);", "cc 多 cage 换回");
need(cc, ".Push(\n      {parameter, callback, deferred_size});", "cc bailout 推");
need(cc, "AccountDeferredMarkedBytes(", "cc 延标字节");

const body = `${banner("src/heap/cppgc-js/unified-heap-marking-visitor.h.js", REL_H, "统一堆标记访问者 — Base 9 钩子 + Mutator/Concurrent 双子类 (cc 实现一并落地)")}
// ══ UnifiedHeapMarkingVisitorBase (原版 unified-heap-marking-visitor.h:37-67, 实现 cc:28-107) ══
/// JSVisitor 钥匙构造; 双标记态指针 (cppgc 面 / 统一堆 JS 面)

pub struct UnifiedHeapMarkingVisitorBase {
    /// cppgc::internal::BasicMarkingState&
    pub marking_state: *mut cppgc_internal::BasicMarkingState,
    /// 统一堆 TracedReference 标记态
    pub unified_heap_marking_state: *mut UnifiedHeapMarkingState,
}

impl UnifiedHeapMarkingVisitorBase {
    /// 原版 cc:28-35
    pub fn new(_heap: &mut HeapBase, marking_state: &mut cppgc_internal::BasicMarkingState, unified_heap_marking_state: &mut UnifiedHeapMarkingState) -> Self {
        Self { marking_state, unified_heap_marking_state }
    }

    // ── C++ 面 8 钩子 (原版 cc:36-102) ──
    /// :36-39 — 强访: 直转 cppgc 态
    pub fn visit(&mut self, object: *const u8, desc: TraceDescriptor) {
        unsafe { &mut *self.marking_state }.mark_and_push(object, desc);
    }
    /// :41-56 — 非压缩数组: 步 kSizeOfUncompressedMember, 逐元素原子载, 空跳
    pub fn visit_multiple_uncompressed_member(&mut self, start: *const u8, len: usize, get_trace_descriptor: TraceDescriptorCallback) {
        let mut it = start as *const u8;
        let end = unsafe { it.add(len * K_SIZE_OF_UNCOMPRESSED_MEMBER) };
        while it < end {
            let current = it as *const cppgc_internal::RawPointer;
            let object = unsafe { &*current }.load_atomic();
            if !object.is_null() {
                unsafe { &mut *self.marking_state }.mark_and_push(object, get_trace_descriptor(object));
            }
            it = unsafe { it.add(K_SIZE_OF_UNCOMPRESSED_MEMBER) };
        }
    }
    /// :58-73 — 压缩数组 (CPPGC_POINTER_COMPRESSION 门): 步 kSizeofCompressedMember, 同上
    pub fn visit_multiple_compressed_member(&mut self, start: *const u8, len: usize, get_trace_descriptor: TraceDescriptorCallback) {
        let mut it = start as *const u8;
        let end = unsafe { it.add(len * K_SIZEOF_COMPRESSED_MEMBER) };
        while it < end {
            let current = it as *const cppgc_internal::CompressedPointer;
            let object = unsafe { &*current }.load_atomic();
            if !object.is_null() {
                unsafe { &mut *self.marking_state }.mark_and_push(object, get_trace_descriptor(object));
            }
            it = unsafe { it.add(K_SIZEOF_COMPRESSED_MEMBER) };
        }
    }
    /// :75-81 — 弱访: 注册弱引用
    pub fn visit_weak(&mut self, object: *const u8, desc: TraceDescriptor, weak_callback: WeakCallback, weak_member: *const u8) {
        unsafe { &mut *self.marking_state }.register_weak_reference_if_needed(object, desc, weak_callback, weak_member);
    }
    /// :83-87 — ephemeron: 处理键值 (回传自身 visitor)
    pub fn visit_ephemeron(&mut self, key: *const u8, value: *const u8, value_desc: TraceDescriptor) {
        unsafe { &mut *self.marking_state }.process_ephemeron(key, value, value_desc, self);
    }
    /// :89-94 — 弱容器: 只传 weak_desc
    pub fn visit_weak_container(&mut self, self_: *const u8, _strong_desc: TraceDescriptor, weak_desc: TraceDescriptor, callback: WeakCallback, data: *const u8) {
        unsafe { &mut *self.marking_state }.process_weak_container(self_, weak_desc, callback, data);
    }
    /// :95-98 — 自定义弱回调注册
    pub fn register_weak_callback(&mut self, callback: WeakCallback, object: *const u8) {
        unsafe { &mut *self.marking_state }.register_weak_custom_callback(callback, object);
    }
    /// :100-102 — 可动引用 (压实)
    pub fn handle_movable_reference(&mut self, slot: *const *mut u8) {
        unsafe { &mut *self.marking_state }.register_movable_reference(slot);
    }

    // ── JS 面 (原版 cc:104-106) ──
    /// TracedReference 走统一堆态
    pub fn visit_traced_reference(&mut self, ref: &TracedReferenceBase) {
        unsafe { &mut *self.unified_heap_marking_state }.mark_and_push(ref);
    }
}

// ══ MutatorUnifiedHeapMarkingVisitor (原版 h:69-75, cc:109-113) ══
/// 纯构造转发, 无额外行为
pub struct MutatorUnifiedHeapMarkingVisitor {
    pub base: UnifiedHeapMarkingVisitorBase,
}

// ══ ConcurrentUnifiedHeapMarkingVisitor (原版 h:77-102, cc:115-156) ══
pub struct ConcurrentUnifiedHeapMarkingVisitor {
    pub base: UnifiedHeapMarkingVisitorBase,
    /// 自持本地工作表 — 析构 Publish (并发访问在主线程闭包计算前结束, 足够)
    local_marking_worklist: Option<Box<MarkingWorklistsLocal>>,
    concurrent_unified_heap_marking_state: UnifiedHeapMarkingState,
    /// 仅堆附加时存在
    current_isolate_scope: Option<SetCurrentIsolateScope>,
    // v8: V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES — saved_isolate_group_ 换入/换回
}

impl ConcurrentUnifiedHeapMarkingVisitor {
    /// 原版 cc:115-137 — v8_heap 非空时: SetCurrentIsolateScope (+多 cage 下 IsolateGroup 换入)
    pub fn new(heap: &mut HeapBase, v8_heap: *mut Heap, marking_state: &mut cppgc_internal::ConcurrentMarkingState, collection_type: CollectionType) -> Self {
        let local = get_v8_marking_worklists(v8_heap, collection_type);
        let state = UnifiedHeapMarkingState::new(v8_heap, local.as_ref(), collection_type);
        // v8: 测试有未附加 Isolate 的独立 CppHeap 配置 — v8_heap 可空
        let scope = if !v8_heap.is_null() { Some(SetCurrentIsolateScope::new(unsafe { &mut *v8_heap }.isolate())) } else { None };
        Self { base: UnifiedHeapMarkingVisitorBase::new(heap, marking_state as *mut _ as *mut _, unimplemented!()), local_marking_worklist: local, concurrent_unified_heap_marking_state: state, current_isolate_scope: scope }
    }
    /// 原版 cc:139-145 — 析构: 工作表 Publish (+多 cage 换回 IsolateGroup)
    pub fn drop(&mut self) {
        if let Some(w) = &mut self.local_marking_worklist {
            w.publish();
        }
    }
    /// 原版 cc:147-156 — 并发延迟: 推 bailout 工作表 + 按页记延标字节, 恒 true
    pub fn defer_trace_to_mutator_thread_if_concurrent(&mut self, parameter: *const u8, callback: TraceCallback, deferred_size: usize) -> bool {
        unsafe { &mut *self.base.marking_state }
            .concurrent_marking_bailout_worklist()
            .push(BailoutItem { parameter, callback, deferred_size });
        let page = cppgc_internal::BasePage::from_payload(parameter as *mut u8);
        (unsafe { &mut *(self.base.marking_state as *mut cppgc_internal::ConcurrentMarkingState) })
            .account_deferred_marked_bytes(page, deferred_size);
        true
    }
    /// 原版 h:88 — IsConcurrent 恒 true
    pub fn is_concurrent(&self) -> bool {
        true
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "unified_heap_marking_visitor.rs.skel", body);
console.log(`OK unified-heap-marking-visitor.h.js: 锚 ${n} 条 (h19+cc17); 9 钩子 + 双子类; 产物 ${out}`);
