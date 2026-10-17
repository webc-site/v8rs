// 由 tool/src/codegen/tick-counter.h.js 从 v8/src/codegen/tick-counter.{h,cc} 生成; 修订就地改, 复跑覆盖
// TickCounter 常量 + 签名骨架

use std::ptr::null_mut;

/// 编译 tick 计数上限哨兵 (原版 kMaxTicks; 性能退化/编译器发散探测,
/// 取值约为实际需求的 10 倍)
pub const K_MAX_TICKS: usize = 100000000;

/// 原版 TickCounter: 以确定性 tick 近似时间, 驱动 safepoint。
#[derive(Debug, Clone, Copy, Default)]
pub struct TickCounter {
  /// 原版 ticks_
  ticks_: usize,
  /// 原版 local_heap_: LocalHeap* (占位, 待堆侧接线)
  local_heap_: *mut (),
}

impl TickCounter {
  /// 原版 TickAndMaybeEnterSafepoint: ++ticks_; 需要 local heap 则进 safepoint
  pub fn tick_and_maybe_enter_safepoint(&mut self) {
    self.ticks_ += 1;
    debug_assert!(self.ticks_ < K_MAX_TICKS, "tick 溢出哨兵");
    // local_heap_->Safepoint() 待堆侧接线
  }

  /// 原版 TickCounter::AttachLocalHeap: DCHECK_NULL 后绑定 LocalHeap。
  /// v8: src/codegen/tick-counter.cc:15-19
  pub fn attach_local_heap(&mut self, local_heap: *mut ()) {
    debug_assert!(self.local_heap_.is_null(), "原版 DCHECK_NULL(local_heap_)");
    self.local_heap_ = local_heap;
    debug_assert!(
      !self.local_heap_.is_null(),
      "原版 DCHECK_NOT_NULL(local_heap_)"
    );
  }

  /// 原版 TickCounter::DetachLocalHeap: local_heap_ = nullptr。
  /// v8: src/codegen/tick-counter.cc:21
  pub fn detach_local_heap(&mut self) {
    self.local_heap_ = null_mut();
  }

  /// 原版 CurrentTicks
  #[inline]
  #[must_use]
  pub const fn current_ticks(&self) -> usize {
    self.ticks_
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_MAX_TICKS == 100000000);
};
