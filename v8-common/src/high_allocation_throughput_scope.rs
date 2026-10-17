// 由 tool/src/common.js 从 v8/src/common/high-allocation-throughput-scope.h 生成;
// 修订就地改, 复跑覆盖
//
// 修订 (Developer): Platform::GetHighAllocationThroughputObserver 依赖
// v8::Platform (include/v8-platform.h) → trait 抽象; 构造 LeaveSection /
// 析构 EnterSection 与原版 1:1 (high-allocation-throughput-scope.h:19-28)。

/// 原版 HighAllocationThroughputObserver (include/v8-platform.h)
pub trait HighAllocationThroughputObserver: Send + Sync {
  fn enter_section(&self);
  fn leave_section(&self);
}

/// 原版 HighAllocationThroughputScope (high-allocation-throughput-scope.h:17-32)
/// — 通知 embedder 进入高吞吐 malloc/free 区段; 非 Clone 非 Copy (RAII)
pub struct HighAllocationThroughputScope<'a> {
  observer: &'a dyn HighAllocationThroughputObserver,
}

impl<'a> HighAllocationThroughputScope<'a> {
  /// 原版构造: LeaveSection (high-allocation-throughput-scope.h:20-22)
  pub fn new(observer: &'a dyn HighAllocationThroughputObserver) -> Self {
    observer.leave_section();
    Self { observer }
  }
}

impl Drop for HighAllocationThroughputScope<'_> {
  /// 原版析构: EnterSection (high-allocation-throughput-scope.h:28)
  fn drop(&mut self) {
    self.observer.enter_section();
  }
}
