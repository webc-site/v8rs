#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/testing.cc.js — v8/src/heap/cppgc-internal/testing.cc → v8-heap-cppgc/src/testing.rs.skel
// 职责: 测试支撑面 — OverrideEmbedderStackStateScope RAII + StandaloneTestingHeap 5 步增量 GC 驱动 + IsHeapObjectOld
// 规则:
//   ① Scope RAII: 构造 set_override_stack_state / 析构 clear_overridden_stack_state
//   ② StandaloneTestingHeap 5 方法 → HeapBase 转发链 (Start/MarkingStep/Finalize/Toggle/ForceCompaction)
//   ③ IsHeapObjectOld: CPPGC_YOUNG_GENERATION 开 → 标记位判定, 关 → 恒 true
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/testing.cc";
const src = readV8(REL);
let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need("OverrideEmbedderStackStateScope::OverrideEmbedderStackStateScope(", "Scope 构造定义");
need(".set_override_stack_state(state);", "构造设置覆写");
need("OverrideEmbedderStackStateScope::~OverrideEmbedderStackStateScope() {", "Scope 析构定义");
need(".clear_overridden_stack_state();", "析构清除覆写");
need("StandaloneTestingHeap::StartGarbageCollection() {", "Start 定义");
need("StartIncrementalGarbageCollectionForTesting();", "Start 转发链");
need("bool StandaloneTestingHeap::PerformMarkingStep(", "Step 定义");
need("->IncrementalMarkingStepForTesting(stack_state);", "Step 转发链");
need("StandaloneTestingHeap::FinalizeGarbageCollection(", "Finalize 定义");
need("FinalizeIncrementalGarbageCollectionForTesting(stack_state);", "Finalize 转发链");
need("StandaloneTestingHeap::ToggleMainThreadMarking(bool should_mark) {", "Toggle 定义");
need("->SetMainThreadMarkingDisabledForTesting(!should_mark);", "Toggle 取反转发");
need("StandaloneTestingHeap::ForceCompactionForNextGarbageCollection() {", "ForceCompaction 定义");
need(".EnableForNextGCForTesting();", "ForceCompaction 转发链");
need("bool IsHeapObjectOld(void* object) {", "IsHeapObjectOld 定义");
need("#if defined(CPPGC_YOUNG_GENERATION)", "分代门");
need("return internal::HeapObjectHeader::FromObject(object).IsMarked();", "分代开臂");
need("return true;", "分代关臂");

const forwards = (src.match(/internal::HeapBase::From\(heap_handle_\)/g) || []).length;
if (forwards !== 7) throw new Error(`HeapBase::From 转发计数: got=${forwards} want=7 (Scope ctor+dtor 2 + 5 方法 5)`);
n++;

const body = `${banner("src/heap/cppgc-internal/testing.cc.js", REL, "测试支撑 — 栈态覆写 RAII + 独立测试堆增量 GC 5 步驱动 (声明面在 include/cppgc/testing.h)")}
// ══ OverrideEmbedderStackStateScope (原版 testing.cc:13-21) ══

pub struct OverrideEmbedderStackStateScope<'a> {
    heap_handle: &'a mut HeapHandle,
}

impl<'a> OverrideEmbedderStackStateScope<'a> {
    /// 原版 testing.cc:13-17 — 构造: 设置栈态覆写
    pub fn new(heap_handle: &'a mut HeapHandle, state: EmbedderStackState) -> Self {
        HeapBase::from(heap_handle).set_override_stack_state(state);
        Self { heap_handle }
    }
}

impl Drop for OverrideEmbedderStackStateScope<'_> {
    /// 原版 testing.cc:19-21 — 析构: 清除覆写
    fn drop(&mut self) {
        HeapBase::from(self.heap_handle).clear_overridden_stack_state();
    }
}

// ══ StandaloneTestingHeap (原版 testing.cc:23-53) ══
// 6 处 HeapBase::From 转发, 全部 ForTesting 通道。

pub struct StandaloneTestingHeap {
    heap_handle: *mut HeapHandle,
}

impl StandaloneTestingHeap {
    /// 原版 testing.cc:23-24
    pub fn new(heap_handle: &mut HeapHandle) -> Self {
        Self { heap_handle }
    }

    /// 原版 testing.cc:26-29 — 发起增量 GC
    pub fn start_garbage_collection(&mut self) {
        HeapBase::from(unsafe { &mut *self.heap_handle }).start_incremental_garbage_collection_for_testing();
    }

    /// 原版 testing.cc:31-35 — 单步标记, 返回是否完成
    pub fn perform_marking_step(&mut self, stack_state: EmbedderStackState) -> bool {
        HeapBase::from(unsafe { &mut *self.heap_handle })
            .marker()
            .incremental_marking_step_for_testing(stack_state)
    }

    /// 原版 testing.cc:37-41 — 终结增量 GC
    pub fn finalize_garbage_collection(&mut self, stack_state: EmbedderStackState) {
        HeapBase::from(unsafe { &mut *self.heap_handle })
            .finalize_incremental_garbage_collection_for_testing(stack_state);
    }

    /// 原版 testing.cc:43-47 — should_mark 取反后落禁用开关
    pub fn toggle_main_thread_marking(&mut self, should_mark: bool) {
        HeapBase::from(unsafe { &mut *self.heap_handle })
            .marker()
            .set_main_thread_marking_disabled_for_testing(!should_mark);
    }

    /// 原版 testing.cc:49-53 — 下轮 GC 启用压实
    pub fn force_compaction_for_next_garbage_collection(&mut self) {
        HeapBase::from(unsafe { &mut *self.heap_handle })
            .compactor()
            .enable_for_next_gc_for_testing();
    }
}

// ══ IsHeapObjectOld (原版 testing.cc:55-61) ══

/// 分代开 → 标记位即老年代判定; 分代关 → 恒 true
pub fn is_heap_object_old(object: *mut u8) -> bool {
    #[cfg(feature = "cppgc-young-generation")]
    {
        HeapObjectHeader::from_object(object).is_marked()
    }
    #[cfg(not(feature = "cppgc-young-generation"))]
    {
        true
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "testing.rs.skel", body);
console.log(`OK testing.cc.js: 锚 ${n} 条 (定义14+门2+转发计数1); RAII + 5 步驱动; 产物 ${out}`);
