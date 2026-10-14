#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/process-heap-statistics.h.js — v8/src/heap/cppgc-internal/{process-heap-statistics.h,process-heap-statistics.cc} → v8-heap-cppgc/src/process_heap_statistics.rs.skel
// 职责: ProcessHeapStatisticsUpdater 4 静态原子累加 + AllocationObserverImpl 5 观察者转发 + 进程级 2 atomic ODR 零初始化
// 规则:
//   ① 4 static 方法: Increase/Decrease × ObjectSize/Space, 均 fetch_add/fetch_sub relaxed
//   ② AllocationObserverImpl 5 个 final 方法: 2 增/1 重置/2 空间增减; Reset 带 since_last_reset 差额清算
//   ③ cc 侧 2 个 std::atomic_size_t ODR 定义 {0} → Rust AtomicUsize::new(0) 静态
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL_H = "src/heap/cppgc-internal/process-heap-statistics.h";
const REL_CC = "src/heap/cppgc-internal/process-heap-statistics.cc";
const src = readV8(REL_H);
const cc = readV8(REL_CC);
let n = 0;
const need = (s, needle, label) => {
  if (!s.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need(src, "class ProcessHeapStatisticsUpdater {", "更新器声明");
need(src, "class AllocationObserverImpl final", "观察者声明");
need(src, ": public StatsCollector::AllocationObserver {", "观察者基类");
need(cc, "std::atomic_size_t ProcessHeapStatistics::total_allocated_space_{0};", "space ODR 定义");
need(cc, "std::atomic_size_t ProcessHeapStatistics::total_allocated_object_size_{0};", "object_size ODR 定义");

// 5 个 final 观察方法签名
const finals = [...src.matchAll(/void (\w+)\(size_t bytes\) final \{/g)].map((m) => m[1]);
if (finals.join(",") !== "AllocatedObjectSizeIncreased,AllocatedObjectSizeDecreased,ResetAllocatedObjectSize,AllocatedSizeIncreased,AllocatedSizeDecreased")
  throw new Error(`final 方法序: got=${finals.join(",")}`);
n++;
if ((src.match(/std::memory_order_relaxed/g) || []).length !== 4)
  throw new Error(`relaxed 计数: got=${(src.match(/std::memory_order_relaxed/g) || []).length} want=4`);
n++;

const body = `${banner("src/heap/cppgc-internal/process-heap-statistics.h.js", REL_H, "进程级堆统计 4 静态累加 + 观察者转发 (cc 侧 2 atomic ODR)")}
// ══ ProcessHeapStatistics atomic ODR (原版 process-heap-statistics.cc:9-10, 声明在 include/cppgc/process-heap-statistics.h) ══

pub struct ProcessHeapStatistics {
    /// std::atomic_size_t total_allocated_space_{0}
    pub total_allocated_space: std::sync::atomic::AtomicUsize,
    /// std::atomic_size_t total_allocated_object_size_{0}
    pub total_allocated_object_size: std::sync::atomic::AtomicUsize,
}

impl Default for ProcessHeapStatistics {
    fn default() -> Self {
        Self {
            total_allocated_space: std::sync::atomic::AtomicUsize::new(0),
            total_allocated_object_size: std::sync::atomic::AtomicUsize::new(0),
        }
    }
}

// ══ ProcessHeapStatisticsUpdater (原版 process-heap-statistics.h:52-67) ══
// 4 方法均 relaxed 序 — 统计面无跨字段顺序要求。
pub struct ProcessHeapStatisticsUpdater;

impl ProcessHeapStatisticsUpdater {
    /// 原版 :52-55 — fetch_add relaxed
    pub fn increase_total_allocated_object_size(delta: usize) {
        PROCESS_HEAP_STATS.total_allocated_object_size.fetch_add(delta, std::sync::atomic::Ordering::Relaxed);
    }
    /// 原版 :56-59 — fetch_sub relaxed
    pub fn decrease_total_allocated_object_size(delta: usize) {
        PROCESS_HEAP_STATS.total_allocated_object_size.fetch_sub(delta, std::sync::atomic::Ordering::Relaxed);
    }
    /// 原版 :60-63 — fetch_add relaxed
    pub fn increase_total_allocated_space(delta: usize) {
        PROCESS_HEAP_STATS.total_allocated_space.fetch_add(delta, std::sync::atomic::Ordering::Relaxed);
    }
    /// 原版 :64-67 — fetch_sub relaxed
    pub fn decrease_total_allocated_space(delta: usize) {
        PROCESS_HEAP_STATS.total_allocated_space.fetch_sub(delta, std::sync::atomic::Ordering::Relaxed);
    }
}

// ══ AllocationObserverImpl (原版 process-heap-statistics.h:19-49) ══
// 5 final 方法转发到 Updater; Reset 先按 since_last_reset 差额清算再置新值。
pub struct AllocationObserverImpl {
    object_size_changes_since_last_reset: usize, // 原版 :48 = 0
}

impl StatsCollectorAllocationObserver for AllocationObserverImpl {
    /// 原版 :22-25 — 增对象字节: 转发 + 累差
    fn allocated_object_size_increased(&mut self, bytes: usize) {
        ProcessHeapStatisticsUpdater::increase_total_allocated_object_size(bytes);
        self.object_size_changes_since_last_reset += bytes;
    }
    /// 原版 :27-30 — 减对象字节: 转发 + 减差
    fn allocated_object_size_decreased(&mut self, bytes: usize) {
        ProcessHeapStatisticsUpdater::decrease_total_allocated_object_size(bytes);
        self.object_size_changes_since_last_reset -= bytes;
    }
    /// 原版 :32-37 — 重置: 先回退累计差, 再转发新值, 差额记账归位
    fn reset_allocated_object_size(&mut self, bytes: usize) {
        ProcessHeapStatisticsUpdater::decrease_total_allocated_object_size(self.object_size_changes_since_last_reset);
        ProcessHeapStatisticsUpdater::increase_total_allocated_object_size(bytes);
        self.object_size_changes_since_last_reset = bytes;
    }
    /// 原版 :39-41 — 增空间
    fn allocated_size_increased(&mut self, bytes: usize) {
        ProcessHeapStatisticsUpdater::increase_total_allocated_space(bytes);
    }
    /// 原版 :43-45 — 减空间
    fn allocated_size_decreased(&mut self, bytes: usize) {
        ProcessHeapStatisticsUpdater::decrease_total_allocated_space(bytes);
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "process_heap_statistics.rs.skel", body);
console.log(`OK process-heap-statistics.h.js: 锚 ${n} 条 (声明2+ODR2+final 序1+relaxed1); 4 累加 + 5 观察转发; 产物 ${out}`);
