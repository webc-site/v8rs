#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/gc-invoker.js — v8/src/heap/cppgc-internal/gc-invoker.h + .cc → v8-heap-cppgc/src/gc_invoker.rs.skel
// GC 调度器: 按 StackSupport/StackState 分派同步 GC / 非嵌套任务延迟 GC / 拒绝增量
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/gc-invoker.h";
const REL_CC = "src/heap/cppgc-internal/gc-invoker.cc";
const src = readV8(REL);
const srcCc = readV8(REL_CC);

let n = 0;
const need = (s, needle, label, file) => {
  if (!s.includes(needle)) throw new Error(`${file} 缺锚 ${label}: ${needle}`);
  n++;
};

need(src, "class V8_EXPORT_PRIVATE GCInvoker final : public GarbageCollector {", "GCInvoker 声明");
need(src, "GCInvoker(GarbageCollector*, cppgc::Platform*, cppgc::Heap::StackSupport);", "构造签名");
need(src, "class GCInvokerImpl;", "Pimpl 前置");
need(srcCc, "class GCInvoker::GCInvokerImpl final : public GarbageCollector {", "GCInvokerImpl");
need(srcCc, "class GCTask final : public cppgc::Task {", "GCTask 内部类");
need(srcCc, "using Handle = SingleThreadedHandle;", "GCTask Handle");
need(srcCc, "size_t saved_epoch_;", "saved_epoch_ 字段");
// 分派语义锚
need(srcCc, "DCHECK_EQ(config.marking_type, cppgc::Heap::MarkingType::kAtomic);", "CollectGarbage 原子断言");
need(srcCc, "config.stack_state == StackState::kNoHeapPointers", "精确栈直发");
need(srcCc, "kSupportsConservativeStackScan)) {\n    collector_->CollectGarbage(config);", "保守栈直发");
need(srcCc, "NonNestableTasksEnabled())", "非嵌套任务可用");
need(srcCc, "config.stack_state = StackState::kNoHeapPointers;", "任务内强制精确");
need(srcCc, "collector_->epoch() != saved_epoch_", "epoch 失配丢弃");
need(srcCc, "DCHECK_NE(config.marking_type, cppgc::Heap::MarkingType::kAtomic);", "增量非原子断言");
need(srcCc, "there is no\n    // way to know how long it will remain enabled", "增量拒绝理由注释");
need(srcCc, "for (int i = 0; i < 2; i++) {", "RetryAllocate 两轮");
need(srcCc, "GCConfig::FreeMemoryHandling::kReleaseMemory}", "Retry 释放内存配置");
// 转发层计数: GCInvoker:: 纯转发 7 (排除 GCInvokerImpl:: 前缀 3)
const fwd = (srcCc.match(/^(?:void|bool|size_t|std::optional<EmbedderStackState>) GCInvoker::(?!GCInvokerImpl)/gm) || []).length;
if (fwd !== 7) throw new Error(`GCInvoker 转发计数失配: ${fwd} want 7`);
n += fwd;

const body = `${banner("src/heap/cppgc-internal/gc-invoker.js", `${REL} + ${REL_CC}`, "按栈支持度分派的 GC 入口")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/gc-invoker.h GCInvoker
pub struct GcInvoker {
    /// v8: impl_ (GCInvokerImpl)
    impl_: GcInvokerImpl,
}

/// v8: src/heap/cppgc-internal/gc-invoker.cc GCInvoker::GCInvokerImpl
pub struct GcInvokerImpl {
    /// v8: collector_
    collector: *mut dyn GarbageCollector,
    /// v8: platform_
    platform: *mut Platform,
    /// v8: stack_support_
    stack_support: StackSupport,
    /// v8: gc_task_handle_ (GCTask::Handle)
    gc_task_handle: SingleThreadedHandle,
}

/// v8: src/heap/cppgc-internal/gc-invoker.cc GCInvoker::GCInvokerImpl::GCTask
/// Run: handle 已取消或 epoch 已变 → 丢弃; 否则强制 kNoHeapPointers 执行 GC。
pub struct GcTask {
    /// v8: collector_
    collector: *mut dyn GarbageCollector,
    /// v8: config_
    config: GcConfig,
    /// v8: handle_
    handle: SingleThreadedHandle,
    /// v8: saved_epoch_
    saved_epoch: usize,
}

impl GcInvokerImpl {
    /// v8: src/heap/cppgc-internal/gc-invoker.cc GCInvokerImpl::CollectGarbage
    /// ① 精确栈或支持保守扫描 → 同步执行;
    /// ② 否则前台 runner 支持非嵌套任务 → 发 GCTask (栈态改精确);
    /// ③ 否则 no-op (允许超限)。
    pub fn collect_garbage(&mut self, config: GcConfig) {
        debug_assert_eq!(config.marking_type, MarkingType::Atomic);
        if config.stack_state == StackState::NoHeapPointers
            || self.stack_support == StackSupport::SupportsConservativeStackScan
        {
            self.collector_mut().collect_garbage(config);
        } else if self.foreground_non_nestable() && self.gc_task_handle.is_empty() {
            let mut precise = config;
            precise.stack_state = StackState::NoHeapPointers;
            self.gc_task_handle = GcTask::post(self.collector, self.foreground_runner(), precise);
        }
    }

    /// v8: src/heap/cppgc-internal/gc-invoker.cc GCInvokerImpl::StartIncrementalGarbageCollection
    /// 不支持保守扫描且无非嵌套任务 → 拒绝 (增量 GC 无法收尾); 否则直发。
    pub fn start_incremental_garbage_collection(&mut self, config: GcConfig) {
        debug_assert_ne!(config.marking_type, MarkingType::Atomic);
        if self.stack_support != StackSupport::SupportsConservativeStackScan && !self.foreground_non_nestable() {
            return;
        }
        self.collector_mut().start_incremental_garbage_collection(config);
    }

    /// v8: src/heap/cppgc-internal/gc-invoker.cc GCInvokerImpl::RetryAllocate
    /// 至多两轮: 保守原子 GC (kReleaseMemory) → 重试 allocate。
    pub fn retry_allocate(&mut self, allocate: &mut dyn FnMut() -> bool) -> bool {
        for _ in 0..2 {
            self.collect_garbage(GcConfig {
                collection_type: CollectionType::Major,
                stack_state: StackState::MayContainHeapPointers,
                marking_type: MarkingType::Atomic,
                sweeping_type: SweepingType::IncrementalAndConcurrent,
                free_memory_handling: FreeMemoryHandling::ReleaseMemory,
                ..GcConfig::DEFAULT
            });
            if allocate() { return true; }
        }
        false
    }

    /// v8: src/heap/cppgc-internal/gc-invoker.cc GCInvokerImpl 析构 — 取消未跑 GCTask
    fn foreground_non_nestable(&self) -> bool { todo!("platform_->GetForegroundTaskRunner()->NonNestableTasksEnabled()") }
    fn collector_mut(&mut self) -> &mut dyn GarbageCollector { todo!("裸指针可变重借") }
}
`;

writeSrc("v8-heap-cppgc", "gc_invoker.rs.skel", body);
console.log(`gc-invoker.js: 对数 ${n} 项 (.h 4 + .cc 12 + 转发 9) → v8-heap-cppgc/src/gc_invoker.rs.skel`);
