// v8-objects-rest/src/feedback_cell.rs
// 对应原版 v8/src/objects/feedback-cell.h 与 feedback-cell-inl.h

use std::{
  mem::{offset_of, size_of},
  sync::atomic::{AtomicI32, Ordering},
};

use v8_objects_core::{
  heap_object::HeapObject,
  map::Map,
  tagged::{Object, Tagged},
};

/// 闭包数量状态迁移 (对应 C++ FeedbackCell::ClosureCountTransition)
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ClosureCountTransition {
  /// 0 到 1 个闭包
  NoneToOne,
  /// 1 到多个闭包
  OneToMany,
  /// 多个闭包状态保持
  Many,
}

/// 反馈单元格 (FeedbackCell)
///
/// 维护闭包与其 FeedbackVector 之间的链接，并通过原子中断预算计数器 (interrupt_budget)
/// 追踪函数执行热度。
#[repr(C)]
#[derive(Debug)]
pub struct FeedbackCell {
  /// 继承自 HeapObject (原版继承 Struct)
  pub header: HeapObject,
  /// 单元格存储的值 (Undefined, FeedbackVector, 或 ClosureFeedbackCellArray)
  pub value: Tagged<Object>,
  /// 调度句柄 (JSDispatchHandleMember)
  pub dispatch_handle: u32,
  /// 中断与优化触发预算 (原子更新)
  pub interrupt_budget: AtomicI32,
}

// 静态断言保证 64 位未压缩 tagged 布局精确匹配原版
const _: () = assert!(size_of::<FeedbackCell>() == 24);
const _: () = assert!(offset_of!(FeedbackCell, value) == 8);
const _: () = assert!(offset_of!(FeedbackCell, dispatch_handle) == 16);
const _: () = assert!(offset_of!(FeedbackCell, interrupt_budget) == 20);

impl FeedbackCell {
  /// 空调度句柄哨兵值 (原版 kNullJSDispatchHandle)
  pub const NULL_DISPATCH_HANDLE: u32 = 0;

  /// 构造新的 FeedbackCell
  #[inline]
  pub fn new(map: Tagged<Map>, value: Tagged<Object>, interrupt_budget: i32) -> Self {
    Self {
      header: HeapObject::new(map),
      value,
      dispatch_handle: Self::NULL_DISPATCH_HANDLE,
      interrupt_budget: AtomicI32::new(interrupt_budget),
    }
  }

  /// 获取存储的值 (Relaxed 序)
  #[inline]
  pub const fn value(&self) -> Tagged<Object> {
    self.value
  }

  /// 设置存储的值 (Relaxed 序)
  #[inline]
  pub fn set_value(&mut self, value: Tagged<Object>) {
    self.value = value;
  }

  /// 获取中断预算 (Relaxed 序)
  #[inline]
  pub fn interrupt_budget(&self) -> i32 {
    self.interrupt_budget.load(Ordering::Relaxed)
  }

  /// 获取中断预算 (Acquire 序)
  #[inline]
  pub fn interrupt_budget_acquire(&self) -> i32 {
    self.interrupt_budget.load(Ordering::Acquire)
  }

  /// 设置中断预算 (Relaxed 序)
  #[inline]
  pub fn set_interrupt_budget(&self, value: i32) {
    self.interrupt_budget.store(value, Ordering::Relaxed);
  }

  /// 设置中断预算 (Release 序)
  #[inline]
  pub fn set_interrupt_budget_release(&self, value: i32) {
    self.interrupt_budget.store(value, Ordering::Release);
  }

  /// 清空中断预算为 0
  #[inline]
  pub fn clear_interrupt_budget(&self) {
    self.set_interrupt_budget(0);
  }

  /// 减少中断预算并返回扣减后的剩余值 (原子 fetch_sub)
  #[inline]
  pub fn decrement_interrupt_budget(&self, delta: i32) -> i32 {
    self.interrupt_budget.fetch_sub(delta, Ordering::Relaxed) - delta
  }

  /// 增加中断预算并返回增加后的总值 (原子 fetch_add)
  #[inline]
  pub fn increment_interrupt_budget(&self, delta: i32) -> i32 {
    self.interrupt_budget.fetch_add(delta, Ordering::Relaxed) + delta
  }

  /// 获取调度句柄
  #[inline]
  pub const fn dispatch_handle(&self) -> u32 {
    self.dispatch_handle
  }

  /// 设置调度句柄
  #[inline]
  pub fn set_dispatch_handle(&mut self, handle: u32) {
    self.dispatch_handle = handle;
  }

  /// 重置反馈向量 (原版 reset_feedback_vector)
  #[inline]
  pub fn reset_feedback_vector(&mut self, fallback_value: Tagged<Object>) {
    self.clear_interrupt_budget();
    self.set_value(fallback_value);
  }
}
