// 由 tool/src/heap/cppgc-internal/process-heap-statistics.h.js 从 v8/src/heap/cppgc-internal/process-heap-statistics.h 生成; 修订就地改, 复跑覆盖
// 进程级堆统计 4 静态累加 + 观察者转发 (cc 侧 2 atomic ODR)
// 推迟台账: 原版 AllocationObserverImpl `: public StatsCollectorAllocationObserver` 的 trait 绑定面
// 依赖 stats_collector 批次 (StatsCollectorAllocationObserver 虚面), 本面先落同签名固有方法。

use std::sync::atomic::{AtomicUsize, Ordering};

/// v8: src/heap/cppgc-internal/process-heap-statistics.cc:9-10 ProcessHeapStatistics
/// 2 atomic ODR 累加 (total_allocated_space_, total_allocated_object_size_)。
pub struct ProcessHeapStatistics {
  /// std::atomic_size_t total_allocated_space_{0}
  pub total_allocated_space: AtomicUsize,
  /// std::atomic_size_t total_allocated_object_size_{0}
  pub total_allocated_object_size: AtomicUsize,
}

/// 进程级单例 (原版为 namespace-scope static)。
static PROCESS_HEAP_STATS: ProcessHeapStatistics = ProcessHeapStatistics {
  total_allocated_space: AtomicUsize::new(0),
  total_allocated_object_size: AtomicUsize::new(0),
};

impl ProcessHeapStatistics {
  /// v8: include/cppgc/process-heap-statistics.h TotalAllocatedSpace
  pub fn total_allocated_space() -> usize {
    PROCESS_HEAP_STATS
      .total_allocated_space
      .load(Ordering::Relaxed)
  }

  /// v8: include/cppgc/process-heap-statistics.h TotalAllocatedObjectSize
  pub fn total_allocated_object_size() -> usize {
    PROCESS_HEAP_STATS
      .total_allocated_object_size
      .load(Ordering::Relaxed)
  }

  /// 读取当前累计 (space, object_size), relaxed — 统计面无跨字段顺序要求。
  pub fn current() -> (usize, usize) {
    (
      Self::total_allocated_space(),
      Self::total_allocated_object_size(),
    )
  }
}

/// v8: src/heap/cppgc-internal/process-heap-statistics.h:52-67 ProcessHeapStatisticsUpdater
/// 4 方法均 relaxed 序。
pub struct ProcessHeapStatisticsUpdater;

impl ProcessHeapStatisticsUpdater {
  /// v8: :52-55 — fetch_add relaxed
  pub fn increase_total_allocated_object_size(delta: usize) {
    PROCESS_HEAP_STATS
      .total_allocated_object_size
      .fetch_add(delta, Ordering::Relaxed);
  }
  /// v8: :56-59 — fetch_sub relaxed
  pub fn decrease_total_allocated_object_size(delta: usize) {
    PROCESS_HEAP_STATS
      .total_allocated_object_size
      .fetch_sub(delta, Ordering::Relaxed);
  }
  /// v8: :60-63 — fetch_add relaxed
  pub fn increase_total_allocated_space(delta: usize) {
    PROCESS_HEAP_STATS
      .total_allocated_space
      .fetch_add(delta, Ordering::Relaxed);
  }
  /// v8: :64-67 — fetch_sub relaxed
  pub fn decrease_total_allocated_space(delta: usize) {
    PROCESS_HEAP_STATS
      .total_allocated_space
      .fetch_sub(delta, Ordering::Relaxed);
  }
}

/// v8: src/heap/cppgc-internal/process-heap-statistics.h:19-49 AllocationObserverImpl
/// 5 方法转发到 Updater; Reset 先按 since_last_reset 差额清算再置新值。
/// trait 绑定 (`StatsCollectorAllocationObserver`) 随 stats_collector 批次挂载, 见件首台账。
#[derive(Default)]
pub struct AllocationObserverImpl {
  /// 原版 :48 object_size_changes_since_last_reset_ = 0
  object_size_changes_since_last_reset: usize,
}

impl AllocationObserverImpl {
  /// v8: :22-25 — 增对象字节: 转发 + 累差
  pub fn allocated_object_size_increased(&mut self, bytes: usize) {
    ProcessHeapStatisticsUpdater::increase_total_allocated_object_size(bytes);
    self.object_size_changes_since_last_reset += bytes;
  }
  /// v8: :27-30 — 减对象字节: 转发 + 减差
  pub fn allocated_object_size_decreased(&mut self, bytes: usize) {
    ProcessHeapStatisticsUpdater::decrease_total_allocated_object_size(bytes);
    self.object_size_changes_since_last_reset -= bytes;
  }
  /// v8: :32-37 — 重置: 先回退累计差, 再转发新值, 差额记账归位
  pub fn reset_allocated_object_size(&mut self, bytes: usize) {
    ProcessHeapStatisticsUpdater::decrease_total_allocated_object_size(
      self.object_size_changes_since_last_reset,
    );
    ProcessHeapStatisticsUpdater::increase_total_allocated_object_size(bytes);
    self.object_size_changes_since_last_reset = bytes;
  }
  /// v8: :39-41 — 增空间
  pub fn allocated_size_increased(&mut self, bytes: usize) {
    ProcessHeapStatisticsUpdater::increase_total_allocated_space(bytes);
  }
  /// v8: :43-45 — 减空间
  pub fn allocated_size_decreased(&mut self, bytes: usize) {
    ProcessHeapStatisticsUpdater::decrease_total_allocated_space(bytes);
  }
}
