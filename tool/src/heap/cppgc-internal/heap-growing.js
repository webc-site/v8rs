#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/heap-growing.js — v8/src/heap/cppgc-internal/heap-growing.h + .cc → v8-heap-cppgc/src/heap_growing.rs.skel
// 定比增长策略: kGrowingFactor=1.5 + 增量限 [0.5, 0.9] 区间夹逼 + 默认 1MB 初始堆
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/heap-growing.h";
const REL_CC = "src/heap/cppgc-internal/heap-growing.cc";
const src = readV8(REL);
const srcCc = readV8(REL_CC);

let n = 0;
const need = (s, needle, label, file) => {
  if (!s.includes(needle)) throw new Error(`${file} 缺锚 ${label}: ${needle}`);
  n++;
};

need(src, "class V8_EXPORT_PRIVATE HeapGrowing final {", "HeapGrowing 声明");
need(src, "static constexpr double kGrowingFactor = 1.5;", "kGrowingFactor 1.5");
need(src, "static constexpr size_t kMinLimitIncrease =", "kMinLimitIncrease");
need(src, "kPageSize * RawHeap::kNumberOfRegularSpaces;", "页×常规空间数");
need(src, "size_t limit_for_atomic_gc() const;", "limit_for_atomic_gc");
need(src, "size_t limit_for_incremental_gc() const;", "limit_for_incremental_gc");
need(src, "void DisableForTesting();", "DisableForTesting");
need(src, "class HeapGrowingImpl;", "Pimpl");
// ── .cc ───────────────────────────────────────────────────────────────
need(srcCc, "constexpr double kMaximumLimitRatioForIncrementalGC = 0.9;", "上限比 0.9");
need(srcCc, "constexpr double kMinimumLimitRatioForIncrementalGC = 0.5;", "下限比 0.5");
need(srcCc, "size_t initial_heap_size_ = 1 * kMB;", "初始 1MB");
need(srcCc, "limit_for_atomic_gc_ = std::max(static_cast<size_t>(size * kGrowingFactor),", "原子限公式");
need(srcCc, "size + kMinLimitIncrease);", "最小增幅");
need(srcCc, "heap::base::IncrementalMarkingSchedule::kEstimatedMarkingTime", "估计标记时长");
need(srcCc, "stats_collector_->GetRecentAllocationSpeedInBytesPerMs()", "近期分配速度");
need(srcCc, "size_t maximum_limit_incremental_gc =", "增量上限");
need(srcCc, "size_t minimum_limit_incremental_gc =", "增量下限");
need(srcCc, "if (allocated_object_size > limit_for_atomic_gc_) {", "超原子限触发全 GC");
need(srcCc, "} else if (allocated_object_size > limit_for_incremental_gc_) {", "超增量限触发增量");
need(srcCc, "if (marking_support_ == cppgc::Heap::MarkingType::kAtomic) return;", "纯原子不支持增量");
need(srcCc, "if (constraints.initial_heap_size_bytes > 0) {", "约束覆盖初始堆");
need(srcCc, "stats_collector->RegisterObserver(this);", "注册观察者");

const body = `${banner("src/heap/cppgc-internal/heap-growing.js", `${REL} + ${REL_CC}`, "定比堆增长: 原子限/增量限双阈值")}
#![allow(dead_code)]

pub mod constants {
    /// v8: src/heap/cppgc-internal/heap-growing.h HeapGrowing::kGrowingFactor
    pub const K_GROWING_FACTOR: f64 = 1.5;
    /// v8: HeapGrowing::kMinLimitIncrease = kPageSize * kNumberOfRegularSpaces
    pub const K_MIN_LIMIT_INCREASE: usize = K_PAGE_SIZE * RawHeap::K_NUMBER_OF_REGULAR_SPACES;
    /// v8: src/heap/cppgc-internal/heap-growing.cc kMaximumLimitRatioForIncrementalGC
    pub const K_MAXIMUM_LIMIT_RATIO_FOR_INCREMENTAL_GC: f64 = 0.9;
    /// v8: src/heap/cppgc-internal/heap-growing.cc kMinimumLimitRatioForIncrementalGC
    pub const K_MINIMUM_LIMIT_RATIO_FOR_INCREMENTAL_GC: f64 = 0.5;
    /// v8: src/heap/cppgc-internal/heap-growing.cc initial_heap_size_
    pub const K_INITIAL_HEAP_SIZE: usize = 1 * K_MB;
}

/// v8: src/heap/cppgc-internal/heap-growing.cc HeapGrowing::HeapGrowingImpl
pub struct HeapGrowingImpl {
    /// v8: collector_
    collector: *mut dyn GarbageCollector,
    /// v8: stats_collector_
    stats_collector: *mut StatsCollector,
    /// v8: initial_heap_size_ (ResourceConstraints.initial_heap_size_bytes 可覆盖)
    initial_heap_size: usize,
    /// v8: limit_for_atomic_gc_
    limit_for_atomic_gc: usize,
    /// v8: limit_for_incremental_gc_
    limit_for_incremental_gc: usize,
    /// v8: disabled_for_testing_
    disabled_for_testing: bool,
    /// v8: marking_support_ / sweeping_support_
    marking_support: MarkingType,
    sweeping_support: SweepingType,
}

impl HeapGrowingImpl {
    /// v8: src/heap/cppgc-internal/heap-growing.cc HeapGrowingImpl 构造
    /// constraints.initial_heap_size_bytes > 0 覆盖默认; 首次 ConfigureLimit(0); RegisterObserver。
    pub fn new(collector: *mut dyn GarbageCollector, stats_collector: *mut StatsCollector,
               initial_heap_size: usize, marking_support: MarkingType, sweeping_support: SweepingType) -> Self {
        let mut s = Self {
            collector, stats_collector,
            initial_heap_size: if initial_heap_size > 0 { initial_heap_size } else { constants::K_INITIAL_HEAP_SIZE },
            limit_for_atomic_gc: 0, limit_for_incremental_gc: 0,
            disabled_for_testing: false, marking_support, sweeping_support,
        };
        s.configure_limit(0);
        s
    }

    /// v8: src/heap/cppgc-internal/heap-growing.cc HeapGrowingImpl::ConfigureLimit
    /// base = max(allocated, initial);
    /// atomic = max(base*1.5, base + kMinLimitIncrease);
    /// est = ceil(kEstimatedMarkingTime.ms × 分配速度);
    /// incremental = clamp(atomic - est, base + (atomic-base)*0.5, base + (atomic-base)*0.9)。
    pub fn configure_limit(&mut self, allocated_object_size: usize) {
        let size = allocated_object_size.max(self.initial_heap_size);
        self.limit_for_atomic_gc = ((size as f64 * constants::K_GROWING_FACTOR) as usize)
            .max(size + constants::K_MIN_LIMIT_INCREASE);
        let estimated = Self::estimate_bytes_during_incremental_gc(self.stats_collector);
        let based_on_rate = self.limit_for_atomic_gc.saturating_sub(estimated);
        let span = self.limit_for_atomic_gc - size;
        let maximum = size + (span as f64 * constants::K_MAXIMUM_LIMIT_RATIO_FOR_INCREMENTAL_GC) as usize;
        let minimum = size + (span as f64 * constants::K_MINIMUM_LIMIT_RATIO_FOR_INCREMENTAL_GC) as usize;
        self.limit_for_incremental_gc = based_on_rate.clamp(minimum, maximum);
    }

    /// v8: kEstimatedMarkingTime × GetRecentAllocationSpeedInBytesPerMs
    fn estimate_bytes_during_incremental_gc(_stats: *mut StatsCollector) -> usize {
        todo!("IncrementalMarkingSchedule::kEstimatedMarkingTime 依赖, 语义面留锚")
    }

    /// v8: src/heap/cppgc-internal/heap-growing.cc HeapGrowingImpl::AllocatedObjectSizeIncreased
    /// > atomic 限 → 保守原子全 GC; > incremental 限且支持增量 → 起增量 GC。
    pub fn allocated_object_size_increased(&mut self) {
        if self.disabled_for_testing { return; }
        let allocated = unsafe { (*self.stats_collector).allocated_object_size() };
        if allocated > self.limit_for_atomic_gc {
            unsafe { (*self.collector).collect_garbage(GcConfig {
                collection_type: CollectionType::Major,
                stack_state: StackState::MayContainHeapPointers,
                marking_type: MarkingType::Atomic,
                sweeping_type: self.sweeping_support,
                ..GcConfig::DEFAULT
            }); }
        } else if allocated > self.limit_for_incremental_gc {
            if self.marking_support == MarkingType::Atomic { return; }
            unsafe { (*self.collector).start_incremental_garbage_collection(GcConfig {
                collection_type: CollectionType::Major,
                stack_state: StackState::MayContainHeapPointers,
                marking_type: self.marking_support,
                sweeping_type: self.sweeping_support,
                ..GcConfig::DEFAULT
            }); }
        }
    }

    /// v8: HeapGrowingImpl::ResetAllocatedObjectSize
    pub fn reset_allocated_object_size(&mut self, allocated: usize) { self.configure_limit(allocated); }
    /// v8: HeapGrowingImpl::limit_for_atomic_gc / limit_for_incremental_gc
    pub fn limit_for_atomic_gc(&self) -> usize { self.limit_for_atomic_gc }
    pub fn limit_for_incremental_gc(&self) -> usize { self.limit_for_incremental_gc }
    /// v8: HeapGrowingImpl::DisableForTesting
    pub fn disable_for_testing(&mut self) { self.disabled_for_testing = true; }
}
`;

writeSrc("v8-heap-cppgc", "heap_growing.rs.skel", body);
console.log(`heap-growing.js: 对数 ${n} 项 (.h 8 + .cc 14) → v8-heap-cppgc/src/heap_growing.rs.skel`);
