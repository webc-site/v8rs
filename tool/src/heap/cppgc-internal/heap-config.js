#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/heap-config.js — v8/src/heap/cppgc-internal/heap-config.h → v8-heap-cppgc/src/heap_config.rs.skel
// GC 配置数据面: CollectionType/FreeMemoryHandling/IsForcedGC/CompactableSpaceHandling 枚举 + MarkingConfig/SweepingConfig/GCConfig 三结构 + 8 个预置 GCConfig 工厂
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/heap-config.h";
const src = readV8(REL);

let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`heap-config.h 缺锚 ${label}: ${needle}`);
  n++;
};

// ── ① 枚举 ─────────────────────────────────────────────────────────────
need("enum class CollectionType : uint8_t {\n  kMinor,\n  kMajor,\n};", "CollectionType");
need("enum class FreeMemoryHandling : uint8_t {", "FreeMemoryHandling");
need("kRetainMemory,", "kRetainMemory");
need("kReleaseMemory\n};", "kReleaseMemory");
need("enum class IsForcedGC : uint8_t {\n    kNotForced,\n    kForced,\n  };", "IsForcedGC");
need("enum class CompactableSpaceHandling { kSweep, kIgnore };", "CompactableSpaceHandling");

// ── ② 三结构字段 ────────────────────────────────────────────────────────
need("struct MarkingConfig {", "MarkingConfig");
for (const f of ["const CollectionType collection_type = CollectionType::kMajor;",
  "StackState stack_state = StackState::kMayContainHeapPointers;",
  "MarkingType marking_type = MarkingType::kIncremental;",
  "IsForcedGC is_forced_gc = IsForcedGC::kNotForced;"]) need(f, "MarkingConfig 字段");
need("struct SweepingConfig {", "SweepingConfig");
for (const f of ["SweepingType sweeping_type = SweepingType::kIncrementalAndConcurrent;",
  "CompactableSpaceHandling compactable_space_handling =",
  "FreeMemoryHandling free_memory_handling = FreeMemoryHandling::kRetainMemory;"]) need(f, "SweepingConfig 字段");
need("struct GCConfig {", "GCConfig");

// ── ③ GCConfig 预置工厂 8 + MarkingConfig::Default ─────────────────────
const FACTORIES = ["ConservativeAtomicConfig", "PreciseAtomicConfig", "ConservativeIncrementalConfig",
  "PreciseIncrementalConfig", "PreciseIncrementalMarkingConcurrentSweepingConfig",
  "PreciseConcurrentConfig", "MinorPreciseAtomicConfig", "MinorConservativeAtomicConfig"];
for (const f of FACTORIES) {
  if (!src.includes(`static constexpr GCConfig\n  ${f}()`) && !src.includes(`static constexpr GCConfig ${f}()`)) {
    throw new Error(`heap-config.h 缺锚 工厂 ${f}`);
  }
  n++;
}
need("static constexpr MarkingConfig Default()", "MarkingConfig::Default");

// 工厂字面量核对: Minor 工厂用 kMinor, 其余 kMajor; Precise* 用 kNoHeapPointers
const minorCount = (src.match(/CollectionType::kMinor, StackState/g) || []).length;
const preciseCount = (src.match(/StackState::kNoHeapPointers,/g) || []).length;
if (minorCount !== 2) throw new Error(`Minor 工厂计数失配: ${minorCount} want 2`);
if (preciseCount !== 5) throw new Error(`Precise 工厂计数失配: ${preciseCount} want 5`);
n += minorCount + preciseCount;

const body = `${banner("src/heap/cppgc-internal/heap-config.js", REL, "GC 配置数据面")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/heap-config.h CollectionType
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CollectionType {
    Minor,
    Major,
}

/// v8: src/heap/cppgc-internal/heap-config.h FreeMemoryHandling
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FreeMemoryHandling {
    /// v8: kRetainMemory — 不 discard 不 decommit
    RetainMemory,
    /// v8: kReleaseMemory — 尽量归还系统 (discard + decommit pooled pages)
    ReleaseMemory,
}

/// v8: src/heap/cppgc-internal/heap-config.h MarkingConfig::IsForcedGC
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IsForcedGc {
    NotForced,
    Forced,
}

/// v8: src/heap/cppgc-internal/heap-config.h SweepingConfig::CompactableSpaceHandling
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CompactableSpaceHandling {
    Sweep,
    Ignore,
}

/// v8: src/heap/cppgc-internal/heap-config.h MarkingConfig
#[derive(Clone, Copy)]
pub struct MarkingConfig {
    pub collection_type: CollectionType,
    pub stack_state: StackState,
    pub marking_type: MarkingType,
    pub is_forced_gc: IsForcedGc,
}

impl MarkingConfig {
    /// v8: src/heap/cppgc-internal/heap-config.h MarkingConfig::Default
    pub const fn default_config() -> Self {
        Self {
            collection_type: CollectionType::Major,
            stack_state: StackState::MayContainHeapPointers,
            marking_type: MarkingType::Incremental,
            is_forced_gc: IsForcedGc::NotForced,
        }
    }
}

/// v8: src/heap/cppgc-internal/heap-config.h SweepingConfig
#[derive(Clone, Copy)]
pub struct SweepingConfig {
    pub sweeping_type: SweepingType,
    pub compactable_space_handling: CompactableSpaceHandling,
    pub free_memory_handling: FreeMemoryHandling,
}

/// v8: src/heap/cppgc-internal/heap-config.h GCConfig
#[derive(Clone, Copy)]
pub struct GcConfig {
    pub collection_type: CollectionType,
    pub stack_state: StackState,
    pub marking_type: MarkingType,
    pub sweeping_type: SweepingType,
    pub free_memory_handling: FreeMemoryHandling,
    pub is_forced_gc: IsForcedGc,
}

impl GcConfig {
    /// v8: src/heap/cppgc-internal/heap-config.h GCConfig::ConservativeAtomicConfig
    pub const fn conservative_atomic() -> Self {
        Self { collection_type: CollectionType::Major, stack_state: StackState::MayContainHeapPointers, marking_type: MarkingType::Atomic, sweeping_type: SweepingType::Atomic, ..Self::DEFAULT }
    }
    /// v8: src/heap/cppgc-internal/heap-config.h GCConfig::PreciseAtomicConfig
    pub const fn precise_atomic() -> Self {
        Self { collection_type: CollectionType::Major, stack_state: StackState::NoHeapPointers, marking_type: MarkingType::Atomic, sweeping_type: SweepingType::Atomic, ..Self::DEFAULT }
    }
    /// v8: src/heap/cppgc-internal/heap-config.h GCConfig::ConservativeIncrementalConfig
    pub const fn conservative_incremental() -> Self {
        Self { collection_type: CollectionType::Major, stack_state: StackState::MayContainHeapPointers, marking_type: MarkingType::Incremental, sweeping_type: SweepingType::Atomic, ..Self::DEFAULT }
    }
    /// v8: src/heap/cppgc-internal/heap-config.h GCConfig::PreciseIncrementalConfig
    pub const fn precise_incremental() -> Self {
        Self { collection_type: CollectionType::Major, stack_state: StackState::NoHeapPointers, marking_type: MarkingType::Incremental, sweeping_type: SweepingType::Atomic, ..Self::DEFAULT }
    }
    /// v8: src/heap/cppgc-internal/heap-config.h GCConfig::PreciseIncrementalMarkingConcurrentSweepingConfig
    pub const fn precise_incremental_marking_concurrent_sweeping() -> Self {
        Self { collection_type: CollectionType::Major, stack_state: StackState::NoHeapPointers, marking_type: MarkingType::Incremental, sweeping_type: SweepingType::IncrementalAndConcurrent, ..Self::DEFAULT }
    }
    /// v8: src/heap/cppgc-internal/heap-config.h GCConfig::PreciseConcurrentConfig
    pub const fn precise_concurrent() -> Self {
        Self { collection_type: CollectionType::Major, stack_state: StackState::NoHeapPointers, marking_type: MarkingType::Incremental, sweeping_type: SweepingType::IncrementalAndConcurrent, ..Self::DEFAULT }
    }
    /// v8: src/heap/cppgc-internal/heap-config.h GCConfig::MinorPreciseAtomicConfig
    pub const fn minor_precise_atomic() -> Self {
        Self { collection_type: CollectionType::Minor, stack_state: StackState::NoHeapPointers, marking_type: MarkingType::Atomic, sweeping_type: SweepingType::Atomic, ..Self::DEFAULT }
    }
    /// v8: src/heap/cppgc-internal/heap-config.h GCConfig::MinorConservativeAtomicConfig
    pub const fn minor_conservative_atomic() -> Self {
        Self { collection_type: CollectionType::Minor, stack_state: StackState::MayContainHeapPointers, marking_type: MarkingType::Atomic, sweeping_type: SweepingType::Atomic, ..Self::DEFAULT }
    }

    /// v8: src/heap/cppgc-internal/heap-config.h GCConfig 字段默认值
    const DEFAULT: Self = Self {
        collection_type: CollectionType::Major,
        stack_state: StackState::MayContainHeapPointers,
        marking_type: MarkingType::Atomic,
        sweeping_type: SweepingType::Atomic,
        free_memory_handling: FreeMemoryHandling::RetainMemory,
        is_forced_gc: IsForcedGc::NotForced,
    };
}

/// v8: src/heap/cppgc-internal/heap-config.h StackState (cppgc::Heap::StackState 别名)
pub use crate::stack_state::StackState;
/// v8: src/heap/cppgc-internal/heap-config.h MarkingType (cppgc::Heap::MarkingType)
pub use crate::marking_type::MarkingType;
/// v8: src/heap/cppgc-internal/heap-config.h SweepingType (cppgc::Heap::SweepingType)
pub use crate::sweeping_type::SweepingType;
`;

writeSrc("v8-heap-cppgc", "heap_config.rs.skel", body);
console.log(`heap-config.js: 对数 ${n} 项 (枚举 4 + 结构 3 + 字段 12 + 工厂 9 + 字面量 8) → v8-heap-cppgc/src/heap_config.rs.skel`);
