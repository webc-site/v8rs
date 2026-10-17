#!/usr/bin/env bun
// tool/src/heap/cppgc-js/unified-heap-marking-state.h.js — v8/src/heap/cppgc-js/{unified-heap-marking-state.h,unified-heap-marking-state.cc,unified-heap-marking-state-inl.h} → v8-heap-cppgc/src/unified_heap_marking_state.rs.skel
// 职责: UnifiedHeapMarkingState — TracedReferenceBase 标记推入 (统一堆附加/分离双模式) + MarkMode 双态
// 规则:
//   ① 构造: heap 三态 (null 时 marking_state 亦 null, DCHECK 蕴含); MarkMode = minor→kOnlyYoung / major→kAll
//   ② Update: 换 worklist 指针 + DCHECK heap 非空
//   ③ MarkAndPush (inl): GetSlotThreadSafe → null 早退 (ephemeron 无早退口) → TracedHandles::Mark → 非堆对象退 (Smi 数字) → ShouldMarkObject 后 TryMarkAndPush
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL_H = "src/heap/cppgc-js/unified-heap-marking-state.h";
const REL_CC = "src/heap/cppgc-js/unified-heap-marking-state.cc";
const REL_INL = "src/heap/cppgc-js/unified-heap-marking-state-inl.h";
const src = readV8(REL_H);
const cc = readV8(REL_CC);
const inl = readV8(REL_INL);
let n = 0;
const need = (s, needle, label) => {
  if (!s.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need(src, "class UnifiedHeapMarkingState final {", "类声明");
need(src, "UnifiedHeapMarkingState(Heap*, MarkingWorklists::Local*,", "构造声明");
need(src, "cppgc::internal::CollectionType);", "收集类型参数");
need(src, "void Update(MarkingWorklists::Local*);", "Update 声明");
need(src, "V8_INLINE void MarkAndPush(const TracedReferenceBase&);", "MarkAndPush 声明");
need(src, "Heap* heap() const { return heap_; }", "heap 取用内联");
need(src, "MarkingState* const marking_state_;", "标记状态成员");
need(src, "MarkingWorklists::Local* local_marking_worklist_ = nullptr;", "本地工作表成员");
need(src, "const TracedHandles::MarkMode mark_mode_;", "标记模式成员");
need(cc, "marking_state_(heap_ ? heap_->marking_state() : nullptr),", "cc 状态三态式");
need(cc, "collection_type == cppgc::internal::CollectionType::kMinor", "cc minor 判定");
need(cc, "? TracedHandles::MarkMode::kOnlyYoung", "cc 仅年轻臂");
need(cc, ": TracedHandles::MarkMode::kAll) {", "cc 全标臂");
need(cc, "DCHECK_IMPLIES(heap_, marking_state_);", "cc 蕴含断言");
need(cc, "DCHECK_NOT_NULL(heap_);", "cc Update 断言");
need(inl, "class BasicTracedReferenceExtractor final {", "提取器类");
need(inl, "return const_cast<Address*>(ref.GetSlotThreadSafe());", "线程安全取槽");
need(inl, "if (!traced_handle_location) {", "inl 空槽早退");
need(inl, "Tagged<Object> object =\n      TracedHandles::Mark(traced_handle_location, mark_mode_);", "Mark 调用");
need(inl, "if (!IsHeapObject(object)) {", "非堆对象退");
need(inl, "MarkingHelper::ShouldMarkObject(heap_, heap_object);", "应标判定");
need(inl, "MarkingHelper::TryMarkAndPush(heap_, local_marking_worklist_,", "试标推入");

const body = `${banner("src/heap/cppgc-js/unified-heap-marking-state.h.js", REL_H, "UnifiedHeapMarkingState — TracedReference 标记 (附加/分离双模式, 分离期期望无非空引用) (cc+inl 一并落地)")}
// ══ BasicTracedReferenceExtractor (原版 unified-heap-marking-state-inl.h:26-31) ══

pub struct BasicTracedReferenceExtractor;
impl BasicTracedReferenceExtractor {
    /// GetSlotThreadSafe — 线程安全槽位 (标记入口)
    pub fn get_object_slot_for_marking(ref: &TracedReferenceBase) -> *mut *mut u8 {
        ref.get_slot_thread_safe() as *mut *mut u8
    }
}

// ══ UnifiedHeapMarkingState (原版 .h:19-38, 构造 .cc:14-24) ══

pub struct UnifiedHeapMarkingState {
    heap: *mut Heap,
    /// heap 空 → null (分离模式)
    marking_state: *mut MarkingState,
    local_marking_worklist: *mut MarkingWorklistsLocal, // = nullptr
    /// minor → kOnlyYoung; major → kAll
    mark_mode: TracedHandlesMarkMode,
}

impl UnifiedHeapMarkingState {
    /// 原版 cc:14-24 — 三态: heap 有 → 其 marking_state; 无 → null (DCHECK_IMPLIES)
    pub fn new(heap: *mut Heap, local_marking_worklist: *mut MarkingWorklistsLocal, collection_type: CollectionType) -> Self {
        let marking_state = if heap.is_null() { std::ptr::null_mut() } else { unsafe { &mut *heap }.marking_state() };
        debug_assert!(heap.is_null() || !marking_state.is_null());
        let mark_mode = if collection_type == CollectionType::Minor {
            TracedHandlesMarkMode::OnlyYoung
        } else {
            TracedHandlesMarkMode::All
        };
        Self { heap, marking_state, local_marking_worklist, mark_mode }
    }

    /// 原版 cc:26-30 — 换工作表 (DCHECK heap 非空)
    pub fn update(&mut self, local_marking_worklist: *mut MarkingWorklistsLocal) {
        self.local_marking_worklist = local_marking_worklist;
        debug_assert!(!self.heap.is_null());
    }

    pub fn heap(&self) -> *mut Heap {
        self.heap // 原版 h:31
    }

    /// 原版 inl:33-60 — 4 段: 空槽退 (ephemeron 无早退) → Mark → 非堆对象退 (Smi) → 应标才试标推入
    pub fn mark_and_push(&mut self, reference: &TracedReferenceBase) {
        // 分离模式下遇非空 TracedReferenceBase 会空指针解引崩溃 (原版注释)
        let traced_handle_location = BasicTracedReferenceExtractor::get_object_slot_for_marking(reference);
        // ephemeron 追踪可达此处, 无早退口 — 槽可空
        if traced_handle_location.is_null() {
            return;
        }
        let object = TracedHandles::mark(traced_handle_location, self.mark_mode);
        if !is_heap_object(object) {
            // 数字可物化为堆对象或仅作 Smi 传递, 嵌入层无感知
            return;
        }
        let heap_object = cast_heap_object(object);
        if let Some(worklist_target) = MarkingHelper::should_mark_object(self.heap, heap_object) {
            MarkingHelper::try_mark_and_push(self.heap, self.local_marking_worklist, self.marking_state, worklist_target, heap_object);
        }
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "unified_heap_marking_state.rs.skel", body);
console.log(`OK unified-heap-marking-state.h.js: 锚 ${n} 条 (h8+cc6+inl7); 4 段推入链; 产物 ${out}`);
