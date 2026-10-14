#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/garbage-collector.js — v8/src/heap/cppgc-internal/garbage-collector.h → v8-heap-cppgc/src/garbage_collector.rs.skel
// GC 抽象接口 (供 GCInvoker 包装/mock 测试): trait 方法 7 + 条件项 1
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/garbage-collector.h";
const src = readV8(REL);

let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`garbage-collector.h 缺锚 ${label}: ${needle}`);
  n++;
};

need("class GarbageCollector {", "GarbageCollector 声明");
const METHODS = [
  "virtual void CollectGarbage(GCConfig) = 0;",
  "virtual void StartIncrementalGarbageCollection(GCConfig) = 0;",
  "virtual bool RetryAllocate(v8::base::FunctionRef<bool()> allocate) = 0;",
  "virtual size_t epoch() const = 0;",
  "virtual std::optional<EmbedderStackState> overridden_stack_state() const = 0;",
  "virtual void set_override_stack_state(EmbedderStackState state) = 0;",
  "virtual void clear_overridden_stack_state() = 0;",
];
for (const m of METHODS) need(m, "接口方法");
need("#ifdef V8_ENABLE_ALLOCATION_TIMEOUT", "超时条件块");
need("virtual std::optional<int> UpdateAllocationTimeout() = 0;", "UpdateAllocationTimeout");

const body = `${banner("src/heap/cppgc-internal/garbage-collector.js", REL, "GC 抽象接口; HeapBase 与之共享同签名虚方法")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/garbage-collector.h GarbageCollector
pub trait GarbageCollector {
    /// v8: GarbageCollector::CollectGarbage — 执行 config 指定的 GC
    fn collect_garbage(&mut self, config: GcConfig);

    /// v8: GarbageCollector::StartIncrementalGarbageCollection
    fn start_incremental_garbage_collection(&mut self, config: GcConfig);

    /// v8: GarbageCollector::RetryAllocate — GC 后重试 allocate, 最多两轮 (见 gc-invoker.cc)
    fn retry_allocate(&mut self, allocate: &mut dyn FnMut() -> bool) -> bool;

    /// v8: GarbageCollector::epoch — GC 纪元, 每次 GC 递增
    fn epoch(&self) -> usize;

    /// v8: GarbageCollector::overridden_stack_state
    fn overridden_stack_state(&self) -> Option<StackState>;

    /// v8: GarbageCollector::set_override_stack_state
    fn set_override_stack_state(&mut self, state: StackState);

    /// v8: GarbageCollector::clear_overridden_stack_state
    fn clear_overridden_stack_state(&mut self);

    /// v8: src/heap/cppgc-internal/garbage-collector.h GarbageCollector::UpdateAllocationTimeout
    /// (C++ 侧在 V8_ENABLE_ALLOCATION_TIMEOUT 下; 语义面留锚)
    fn update_allocation_timeout(&mut self) -> Option<i32> {
        None
    }
}
`;

writeSrc("v8-heap-cppgc", "garbage_collector.rs.skel", body);
console.log(`garbage-collector.js: 对数 ${n} 项 (声明 1 + 方法 7 + 条件项 2) → v8-heap-cppgc/src/garbage_collector.rs.skel`);
