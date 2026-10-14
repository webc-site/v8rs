#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/heap-state.js — v8/src/heap/cppgc-internal/heap-state.cc → v8-heap-cppgc/src/heap_state.rs.skel
// HeapState 查询面: 标记/清扫/原子停顿/上次保守 GC 五查询
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/heap-state.cc";
const src = readV8(REL);

let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`heap-state.cc 缺锚 ${label}: ${needle}`);
  n++;
};

const QUERIES = [
  ["bool HeapState::IsMarking(const HeapHandle& heap_handle) {", "marker && marker->IsMarking()"],
  ["bool HeapState::IsSweeping(const HeapHandle& heap_handle) {", ".sweeper().IsSweepingInProgress()"],
  ["bool HeapState::IsSweepingOnOwningThread(const HeapHandle& heap_handle) {", ".IsSweepingOnMutatorThread()"],
  ["bool HeapState::IsInAtomicPause(const HeapHandle& heap_handle) {", ".in_atomic_pause()"],
  ["bool HeapState::PreviousGCWasConservative(const HeapHandle& heap_handle) {", "kMayContainHeapPointers"],
];
for (const [sig, delegate] of QUERIES) {
  need(sig, "查询签名");
  need(delegate, "委托实参");
}

const body = `${banner("src/heap/cppgc-internal/heap-state.js", REL, "嵌入方堆状态只读查询")}
#![allow(dead_code)]

/// v8: include/cppgc/heap-state.h HeapState — 实现于 src/heap/cppgc-internal/heap-state.cc
pub struct HeapState;

impl HeapState {
    /// v8: src/heap/cppgc-internal/heap-state.cc HeapState::IsMarking
    pub fn is_marking(heap_handle: &HeapHandle) -> bool {
        HeapBase::from(heap_handle).marker().map_or(false, |m| m.is_marking())
    }

    /// v8: src/heap/cppgc-internal/heap-state.cc HeapState::IsSweeping
    pub fn is_sweeping(heap_handle: &HeapHandle) -> bool {
        HeapBase::from(heap_handle).sweeper().is_sweeping_in_progress()
    }

    /// v8: src/heap/cppgc-internal/heap-state.cc HeapState::IsSweepingOnOwningThread
    pub fn is_sweeping_on_owning_thread(heap_handle: &HeapHandle) -> bool {
        HeapBase::from(heap_handle).sweeper().is_sweeping_on_mutator_thread()
    }

    /// v8: src/heap/cppgc-internal/heap-state.cc HeapState::IsInAtomicPause
    pub fn is_in_atomic_pause(heap_handle: &HeapHandle) -> bool {
        HeapBase::from(heap_handle).in_atomic_pause()
    }

    /// v8: src/heap/cppgc-internal/heap-state.cc HeapState::PreviousGCWasConservative
    pub fn previous_gc_was_conservative(heap_handle: &HeapHandle) -> bool {
        HeapBase::from(heap_handle).stack_state_of_prev_gc()
            == EmbedderStackState::MayContainHeapPointers
    }
}
`;

writeSrc("v8-heap-cppgc", "heap_state.rs.skel", body);
console.log(`heap-state.js: 对数 ${n} 项 (签名 5 + 委托 5) → v8-heap-cppgc/src/heap_state.rs.skel`);
