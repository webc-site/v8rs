#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/marking-worklists.js — v8/src/heap/cppgc-internal/marking-worklists.h + .cc → v8-heap-cppgc/src/marking_worklists.rs.skel
// 标记工作列表族: 8 类 worklist (段大小 512/16/64) + 4 item 结构 + 外部集合型 worklist
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/marking-worklists.h";
const REL_CC = "src/heap/cppgc-internal/marking-worklists.cc";
const src = readV8(REL);
const srcCc = readV8(REL_CC);

let n = 0;
const need = (s, needle, label, file) => {
  if (!s.includes(needle)) throw new Error(`${file} 缺锚 ${label}: ${needle}`);
  n++;
};

need(src, "class MarkingWorklists {", "MarkingWorklists 声明");
need(src, "static constexpr int kMutatorThreadId = 0;", "kMutatorThreadId 0");
// item 结构 4
need(src, "struct WeakCallbackItem {\n    cppgc::WeakCallback callback;\n    const void* parameter;\n  };", "WeakCallbackItem");
need(src, "struct ConcurrentMarkingBailoutItem {", "ConcurrentMarkingBailoutItem");
need(src, "size_t bailedout_size;", "bailedout_size");
need(src, "struct EphemeronPairItem {\n    const void* key;\n    const void* value;\n    TraceDescriptor value_desc;\n  };", "EphemeronPairItem");
// worklist 别名 + 段大小
const SEGS = [
  ["MarkingWorklist", "heap::base::Worklist<MarkingItem, 512 /* local entries */>;"],
  ["PreviouslyNotFullyConstructedWorklist", "heap::base::Worklist<HeapObjectHeader*, 16 /* local entries */>;"],
  ["WeakCallbackWorklist", "heap::base::Worklist<WeakCallbackItem, 64 /* local entries */>;"],
  ["WeakCustomCallbackWorklist", "heap::base::Worklist<WeakCallbackItem, 16 /* local entries */>;"],
  ["WriteBarrierWorklist", "heap::base::Worklist<HeapObjectHeader*, 64 /*local entries */>;"],
  ["ConcurrentMarkingBailoutWorklist", "heap::base::Worklist<ConcurrentMarkingBailoutItem,\n                           64 /* local entries */>;"],
  ["EphemeronPairsWorklist", "heap::base::Worklist<EphemeronPairItem, 64 /* local entries */>;"],
  ["RetraceMarkedObjectsWorklist", "heap::base::Worklist<HeapObjectHeader*, 16 /* local entries */>;"],
];
for (const [name] of SEGS) {
  if (!src.includes(`using ${name} =`)) throw new Error(`marking-worklists.h 缺锚 别名 ${name}`);
  n++;
}
for (const [, def] of SEGS) {
  if (!src.includes(def)) throw new Error(`marking-worklists.h 缺锚 段定义 ${def.slice(0, 50)}`);
  n++;
}
need(src, "using NotFullyConstructedWorklist = ExternalMarkingWorklist;", "NotFullyConstructed 别名");
need(src, "using WeakContainersWorklist = ExternalMarkingWorklist;", "WeakContainers 别名");
// ExternalMarkingWorklist 方法 5 + 守卫
need(src, "class V8_EXPORT_PRIVATE ExternalMarkingWorklist {", "ExternalMarkingWorklist");
for (const m of ["void Push(HeapObjectHeader*);", "bool Contains(HeapObjectHeader*);",
  "absl::flat_hash_set<HeapObjectHeader*> Extract();", "void Clear();", "bool IsEmpty();"]) need(src, m, "External 方法");
need(src, "template <AccessMode>\n    struct ConditionalMutexGuard;", "条件守卫前置");
need(src, "absl::flat_hash_set<HeapObjectHeader*> objects_;", "objects_ 成员");
// 访问器计数: 单行/折行 '*_worklist() { return &' 12 (concurrent_marking_bailout 的 const 版带 const 限定不匹配)
const accessors = (src.match(/\w+_worklist\(\) \{\n?\s*return &/g) || []).length;
if (accessors !== 12) throw new Error(`访问器计数失配: ${accessors} want 12`);
n += accessors;
// .cc
need(srcCc, "constexpr int MarkingWorklists::kMutatorThreadId;", "kMutatorThreadId 类外定义");
const clearCount = (srcCc.match(/_worklist_\(\)\.Clear\(\);|\.Clear\(\);/g) || []).length;
if (clearCount !== 11) throw new Error(`ClearForTesting 清理计数失配: ${clearCount} want 11`);
n += clearCount;

const body = `${banner("src/heap/cppgc-internal/marking-worklists.js", `${REL} + ${REL_CC}`, "标记期工作列表族 + 外部集合型")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/marking-worklists.h MarkingWorklists::kMutatorThreadId
pub const K_MUTATOR_THREAD_ID: i32 = 0;

/// v8: MarkingWorklists::MarkingItem = cppgc::TraceDescriptor
pub type MarkingItem = TraceDescriptor;

/// v8: src/heap/cppgc-internal/marking-worklists.h MarkingWorklists::WeakCallbackItem
pub struct WeakCallbackItem {
    pub callback: WeakCallback,
    pub parameter: *const core::ffi::c_void,
}

/// v8: src/heap/cppgc-internal/marking-worklists.h MarkingWorklists::ConcurrentMarkingBailoutItem
pub struct ConcurrentMarkingBailoutItem {
    pub parameter: *const core::ffi::c_void,
    pub callback: TraceCallback,
    /// v8: bailedout_size
    pub bailedout_size: usize,
}

/// v8: src/heap/cppgc-internal/marking-worklists.h MarkingWorklists::EphemeronPairItem
pub struct EphemeronPairItem {
    pub key: *const core::ffi::c_void,
    pub value: *const core::ffi::c_void,
    pub value_desc: TraceDescriptor,
}

pub mod worklists {
    /// v8: MarkingWorklist — Worklist<MarkingItem, 512>
    pub type MarkingWorklist = Worklist<MarkingItem, 512>;
    /// v8: PreviouslyNotFullyConstructedWorklist — Worklist<HeapObjectHeader*, 16>
    pub type PreviouslyNotFullyConstructedWorklist = Worklist<*mut HeapObjectHeader, 16>;
    /// v8: WeakCallbackWorklist — Worklist<WeakCallbackItem, 64>
    pub type WeakCallbackWorklist = Worklist<WeakCallbackItem, 64>;
    /// v8: WeakCustomCallbackWorklist — Worklist<WeakCallbackItem, 16>
    pub type WeakCustomCallbackWorklist = Worklist<WeakCallbackItem, 16>;
    /// v8: WriteBarrierWorklist — Worklist<HeapObjectHeader*, 64>
    pub type WriteBarrierWorklist = Worklist<*mut HeapObjectHeader, 64>;
    /// v8: ConcurrentMarkingBailoutWorklist — Worklist<ConcurrentMarkingBailoutItem, 64>
    pub type ConcurrentMarkingBailoutWorklist = Worklist<ConcurrentMarkingBailoutItem, 64>;
    /// v8: EphemeronPairsWorklist — Worklist<EphemeronPairItem, 64>
    pub type EphemeronPairsWorklist = Worklist<EphemeronPairItem, 64>;
    /// v8: RetraceMarkedObjectsWorklist — Worklist<HeapObjectHeader*, 16>
    pub type RetraceMarkedObjectsWorklist = Worklist<*mut HeapObjectHeader, 16>;
}

/// v8: src/heap/cppgc-internal/marking-worklists.h MarkingWorklists::ExternalMarkingWorklist
/// 互斥 hash_set; kNonAtomic 时 ConditionalMutexGuard 为空壳 (免锁)。
pub struct ExternalMarkingWorklist {
    /// v8: lock_
    lock: Mutex,
    /// v8: objects_
    objects: HashSet<*mut HeapObjectHeader>,
}

impl ExternalMarkingWorklist {
    /// v8: ExternalMarkingWorklist::Push
    pub fn push(&mut self, object: *mut HeapObjectHeader) { todo!("ConditionalMutexGuard + insert") }
    /// v8: ExternalMarkingWorklist::Contains
    pub fn contains(&mut self, object: *mut HeapObjectHeader) -> bool { todo!("find != end") }
    /// v8: ExternalMarkingWorklist::Extract — swap 出全集
    pub fn extract(&mut self) -> HashSet<*mut HeapObjectHeader> { todo!("swap") }
    /// v8: ExternalMarkingWorklist::Clear
    pub fn clear(&mut self) { todo!("clear") }
    /// v8: ExternalMarkingWorklist::IsEmpty
    pub fn is_empty(&mut self) -> bool { todo!("empty") }
}

/// v8: src/heap/cppgc-internal/marking-worklists.h MarkingWorklists
pub struct MarkingWorklists {
    /// v8: marking_worklist_
    pub marking_worklist: worklists::MarkingWorklist,
    /// v8: not_fully_constructed_worklist_ (External)
    pub not_fully_constructed_worklist: ExternalMarkingWorklist,
    /// v8: previously_not_fully_constructed_worklist_
    pub previously_not_fully_constructed_worklist: worklists::PreviouslyNotFullyConstructedWorklist,
    /// v8: write_barrier_worklist_
    pub write_barrier_worklist: worklists::WriteBarrierWorklist,
    /// v8: weak_container_callback_worklist_
    pub weak_container_callback_worklist: worklists::WeakCallbackWorklist,
    /// v8: parallel_weak_callback_worklist_
    pub parallel_weak_callback_worklist: worklists::WeakCallbackWorklist,
    /// v8: weak_custom_callback_worklist_
    pub weak_custom_callback_worklist: worklists::WeakCustomCallbackWorklist,
    /// v8: concurrent_marking_bailout_worklist_
    pub concurrent_marking_bailout_worklist: worklists::ConcurrentMarkingBailoutWorklist,
    /// v8: discovered_ephemeron_pairs_worklist_
    pub discovered_ephemeron_pairs_worklist: worklists::EphemeronPairsWorklist,
    /// v8: ephemeron_pairs_for_processing_worklist_
    pub ephemeron_pairs_for_processing_worklist: worklists::EphemeronPairsWorklist,
    /// v8: weak_containers_worklist_ (External)
    pub weak_containers_worklist: ExternalMarkingWorklist,
    /// v8: retrace_marked_objects_worklist_
    pub retrace_marked_objects_worklist: worklists::RetraceMarkedObjectsWorklist,
}

impl MarkingWorklists {
    /// v8: src/heap/cppgc-internal/marking-worklists.cc MarkingWorklists::ClearForTesting
    /// 11 项逐一 Clear (marking/not_fully/previously/write_barrier/weak_container/
    /// parallel_weak/weak_custom/concurrent_bailout/discovered_ephemeron/
    /// ephemeron_processing/retrace)。
    pub fn clear_for_testing(&mut self) {
        self.marking_worklist.clear();
        self.not_fully_constructed_worklist.clear();
        self.previously_not_fully_constructed_worklist.clear();
        self.write_barrier_worklist.clear();
        self.weak_container_callback_worklist.clear();
        self.parallel_weak_callback_worklist.clear();
        self.weak_custom_callback_worklist.clear();
        self.concurrent_marking_bailout_worklist.clear();
        self.discovered_ephemeron_pairs_worklist.clear();
        self.ephemeron_pairs_for_processing_worklist.clear();
        self.retrace_marked_objects_worklist.clear();
    }
}
`;

writeSrc("v8-heap-cppgc", "marking_worklists.rs.skel", body);
console.log(`marking-worklists.js: 对数 ${n} 项 (常量 1 + item 4 + 别名 10 + 方法 5 + 守卫 3 + 访问器 13 + cc 12) → v8-heap-cppgc/src/marking_worklists.rs.skel`);
