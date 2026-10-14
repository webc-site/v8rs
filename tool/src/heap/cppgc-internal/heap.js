#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/heap.js — v8/src/heap/cppgc-internal/heap.h + .cc → v8-heap-cppgc/src/heap.rs.skel
// 公开堆: Heap 三继承 (HeapBase/cppgc::Heap/GarbageCollector) + CheckConfig 级别断言 + GC 全周期
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/heap.h";
const REL_CC = "src/heap/cppgc-internal/heap.cc";
const src = readV8(REL);
const srcCc = readV8(REL_CC);

let n = 0;
const need = (s, needle, label, file) => {
  if (!s.includes(needle)) throw new Error(`${file} 缺锚 ${label}: ${needle}`);
  n++;
};

need(src, "class V8_EXPORT_PRIVATE Heap final : public HeapBase,\n                                     public cppgc::Heap,\n                                     public GarbageCollector {", "三继承声明");
need(src, "static Heap* From(cppgc::Heap* heap) { return static_cast<Heap*>(heap); }", "From 转型");
need(src, "Heap(std::shared_ptr<cppgc::Platform> platform,", "构造声明");
need(src, "void CollectGarbage(GCConfig) final;", "CollectGarbage final");
need(src, "void StartIncrementalGarbageCollection(GCConfig) final;", "StartIncremental final");
need(src, "bool RetryAllocate(v8::base::FunctionRef<bool()> allocate) final;", "RetryAllocate final");
need(src, "void FinalizeIncrementalGarbageCollectionIfRunning(GCConfig);", "IfRunning 声明");
need(src, "size_t epoch() const final { return epoch_; }", "epoch 内联");
need(src, "CHECK(!override_stack_state_);", "覆盖栈态互斥断言");
need(src, "void EnableGenerationalGC();", "EnableGenerationalGC 声明");
need(src, "void DisableHeapGrowingForTesting();", "禁增长测试声明");
need(src, "void StartGarbageCollection(GCConfig);", "StartGarbageCollection 声明");
need(src, "void FinalizeGarbageCollection(StackState);", "Finalize 声明");
need(src, "void FinalizeIncrementalGarbageCollectionIfNeeded(StackState) final;", "IfNeeded 声明");
for (const f of ["GCConfig config_;", "GCInvoker gc_invoker_;", "HeapGrowing growing_;",
  "bool generational_gc_enabled_ = false;", "size_t epoch_ = 0;",
  "std::optional<cppgc::EmbedderStackState> override_stack_state_;"]) need(src, f, "Heap 字段");
// ── .cc ───────────────────────────────────────────────────────────────
need(srcCc, "std::unique_ptr<Heap> Heap::Create(std::shared_ptr<cppgc::Platform> platform,", "工厂 Create");
need(srcCc, "void CheckConfig(GCConfig config, HeapBase::MarkingType marking_support,", "CheckConfig 签名");
need(srcCc, "CHECK_LE(static_cast<int>(config.marking_type),", "标记级别 ≤ 断言");
need(srcCc, "static_cast<int>(marking_support));", "支持级别取整");
need(srcCc, "gc_invoker_(this, platform_.get(), options.stack_support),", "invoker 接线");
need(srcCc, "CHECK_IMPLIES(options.marking_support != HeapBase::MarkingType::kAtomic,", "增量需前台 runner 断言");
need(srcCc, "FinalizeIncrementalGarbageCollectionIfRunning(", "析构收尾调用");
need(srcCc, "sweeper_.FinishIfRunning();", "清扫收尾");
need(srcCc, "subtle::NoGarbageCollectionScope no_gc(*this);", "析构 no_gc 域");
need(srcCc, "void Heap::StartGarbageCollection(GCConfig config) {", "StartGarbageCollection 实体");
need(srcCc, "void Heap::FinalizeGarbageCollectionImpl(StackState stack_state) {", "Finalize 实体");
need(srcCc, "void Heap::EnableGenerationalGC() {", "EnableGenerationalGC 实体");
need(srcCc, "growing_.DisableForTesting();", "禁增长委托");
need(srcCc, "Heap::Heap(std::shared_ptr<cppgc::Platform> platform,", "构造实体");
need(srcCc, "Heap::~Heap() {", "析构实体");
// 方法实体计数: 带返回类型前缀的 16 (构造/析构无前缀, 另锚于上方接线/收尾)
const defs = (srcCc.match(/^(?:std::unique_ptr<Heap>|void|bool|AllocationHandle&|HeapHandle&|size_t|Heap::) Heap::/gm) || []).length;
if (defs !== 16) throw new Error(`heap.cc 实体计数失配: ${defs} want 16`);
n += defs;

const body = `${banner("src/heap/cppgc-internal/heap.js", `${REL} + ${REL_CC}`, "公开堆: GC 周期编排 + 配置级别校验")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/heap.cc CheckConfig
/// 标记/清扫级别序: config 请求级别 ≤ 平台支持级别 (int 序比较)。
fn check_config(config: &GcConfig, marking_support: MarkingType, sweeping_support: SweepingType) {
    assert!((config.marking_type as i32) <= (marking_support as i32));
    assert!((config.sweeping_type as i32) <= (sweeping_support as i32));
}

/// v8: src/heap/cppgc-internal/heap.h Heap (HeapBase + cppgc::Heap + GarbageCollector)
pub struct Heap {
    /// v8: HeapBase 基类子对象
    pub base: HeapBase,
    /// v8: config_
    config: GcConfig,
    /// v8: gc_invoker_ — 传入 HeapBase 构造 (引用自身)
    gc_invoker: GcInvoker,
    /// v8: growing_
    growing: HeapGrowingImpl,
    /// v8: generational_gc_enabled_
    generational_gc_enabled: bool,
    /// v8: epoch_
    epoch: usize,
    /// v8: override_stack_state_
    override_stack_state: Option<EmbedderStackState>,
}

impl Heap {
    /// v8: src/heap/cppgc-internal/heap.cc Heap::Create — unique_ptr 工厂
    pub fn create(platform: SharedPtr<Platform>, options: HeapOptions) -> Box<Self> { todo!("Heap::Heap 封箱") }

    /// v8: src/heap/cppgc-internal/heap.cc Heap::Heap
    /// HeapBase(platform, custom_spaces, stack/marking/sweeping, gc_invoker_, stack_start_marker)
    /// + gc_invoker_(this, platform, stack_support) + growing_(invoker, stats, constraints, …);
    /// CHECK_IMPLIES: 非原子标记/清扫 → 前台 TaskRunner 必须存在。
    pub fn new(platform: SharedPtr<Platform>, options: HeapOptions) -> Self { todo!("接线构造") }

    /// v8: src/heap/cppgc-internal/heap.cc Heap::~Heap
    /// FinalizeIncrementalGarbageCollectionIfRunning(保守原子) + NoGCScope 内 sweeper.FinishIfRunning。
    pub fn destroy(&mut self) { todo!("优雅收尾") }

    /// v8: src/heap/cppgc-internal/heap.cc Heap::CollectGarbage
    /// StartGarbageCollection + FinalizeGarbageCollection(stack_state) + epoch_++。
    pub fn collect_garbage(&mut self, config: GcConfig) { todo!("全量周期") }

    /// v8: src/heap/cppgc-internal/heap.cc Heap::StartIncrementalGarbageCollection
    /// 非原子标记时增量起跳 (EnterIncrementalMarkingIfNeeded 语义), epoch_++。
    pub fn start_incremental_garbage_collection(&mut self, config: GcConfig) { todo!("增量起跳") }

    /// v8: src/heap/cppgc-internal/heap.cc Heap::RetryAllocate — 委托 gc_invoker_
    pub fn retry_allocate(&mut self, allocate: &mut dyn FnMut() -> bool) -> bool {
        self.gc_invoker.impl_.retry_allocate(allocate)
    }

    /// v8: src/heap/cppgc-internal/heap.cc Heap::FinalizeIncrementalGarbageCollectionIfRunning
    /// 标记中才收尾; atom pause 内三段式。
    pub fn finalize_incremental_garbage_collection_if_running(&mut self, config: GcConfig) { todo!("条件收尾") }

    /// v8: src/heap/cppgc-internal/heap.cc Heap::StartGarbageCollection
    /// CheckConfig → 存 config_ → (非原子) 建 Marker (EnterIncrementalMarkingIfNeeded) → 选 verifier。
    pub fn start_garbage_collection(&mut self, config: GcConfig) { todo!("GC 开局") }

    /// v8: src/heap/cppgc-internal/heap.cc Heap::FinalizeGarbageCollectionImpl
    /// EnterAtomicPause → (验证) → FinishMarking → CompactSpacesIfEnabled → sweeper.CompactSpace… → Sweep → 标记释放。
    pub fn finalize_garbage_collection_impl(&mut self, _stack_state: StackState) { todo!("GC 收官") }

    /// v8: src/heap/cppgc-internal/heap.cc Heap::EnableGenerationalGC — 委托 HeapBase::EnableGenerationalGC + 置旗
    pub fn enable_generational_gc(&mut self) { self.generational_gc_enabled = true; }
    /// v8: src/heap/cppgc-internal/heap.cc Heap::DisableHeapGrowingForTesting
    pub fn disable_heap_growing_for_testing(&mut self) { self.growing.disable_for_testing(); }

    /// v8: src/heap/cppgc-internal/heap.cc Heap::set_override_stack_state — CHECK 无既有覆盖
    pub fn set_override_stack_state(&mut self, state: EmbedderStackState) {
        assert!(self.override_stack_state.is_none());
        self.override_stack_state = Some(state);
    }
    /// v8: src/heap/cppgc-internal/heap.cc Heap::clear_overridden_stack_state
    pub fn clear_overridden_stack_state(&mut self) { self.override_stack_state = None; }
    /// v8: Heap::epoch / overridden_stack_state
    pub fn epoch(&self) -> usize { self.epoch }
    pub fn overridden_stack_state(&self) -> Option<EmbedderStackState> { self.override_stack_state }
}

/// v8: src/heap/cppgc-internal/heap.cc Heap::ForceGarbageCollectionSlow / GetAllocationHandle / GetHeapHandle
/// — 直发保守原子 GC + 句柄透传, 语义面留锚。
`;

writeSrc("v8-heap-cppgc", "heap.rs.skel", body);
console.log(`heap.js: 对数 ${n} 项 (.h 17 + .cc 17 + 实体计数 16) → v8-heap-cppgc/src/heap.rs.skel`);
