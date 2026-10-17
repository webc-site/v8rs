//! 线性分配区 (LinearAllocationArea)。
//! 对应原版 v8/src/heap/linear-allocation-area.h (HC-1 落地)。
//!
//! LAB 三元组 start/top/limit; 布局进 IsolateData, 与原版逐项对齐 (tests 金标准断言)。
//! 不变量 (原版 h:17-18): `start <= top <= limit` 恒成立。

use core::mem::size_of;

use v8_common::{
  Address, K_NULL_ADDRESS, K_OBJECT_ALIGNMENT, K_OBJECT_ALIGNMENT_8GB_HEAP, K_SYSTEM_POINTER_SIZE,
  V8_COMPRESS_POINTERS_8GB,
};

/// 原版 LinearAllocationArea (linear-allocation-area.h:19-124)。
/// repr(C) 三 Address 字段: sizeof/offsetof 与原版一致 (IsolateData 内嵌前提, 原版 h:109-111/126)。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LinearAllocationArea {
  /// LAB 起点, 初值与 top 重合; [start, top) 为新对象区间 (原版 h:116-119)
  pub(crate) start_: Address,
  /// 分配顶 (原版 h:120-121)
  pub(crate) top_: Address,
  /// 分配上限 (原版 h:122-123)
  pub(crate) limit_: Address,
}

impl Default for LinearAllocationArea {
  /// 原版 h:21 `LinearAllocationArea() = default` + 字段初值 kNullAddress (h:119-123)
  fn default() -> Self {
    Self {
      start_: K_NULL_ADDRESS,
      top_: K_NULL_ADDRESS,
      limit_: K_NULL_ADDRESS,
    }
  }
}

impl LinearAllocationArea {
  /// 原版 h:22-25 双参 ctor: start = top
  pub fn new(top: Address, limit: Address) -> Self {
    let area = Self {
      start_: top,
      top_: top,
      limit_: limit,
    };
    area.verify();
    area
  }

  /// 原版 h:27-32 Reset(top, limit)
  pub fn reset(&mut self, top: Address, limit: Address) {
    self.start_ = top;
    self.top_ = top;
    self.limit_ = limit;
    self.verify();
  }

  /// 原版 h:34 ResetStart
  #[inline]
  pub fn reset_start(&mut self) {
    self.start_ = self.top_;
  }

  /// 原版 h:36-39 CanIncrementTop
  #[inline]
  pub fn can_increment_top(&self, bytes: usize) -> bool {
    self.verify();
    self.top_ + bytes <= self.limit_
  }

  /// 原版 h:41-46 IncrementTop: 返回旧 top
  #[inline]
  pub fn increment_top(&mut self, bytes: usize) -> Address {
    let old_top = self.top_;
    self.top_ += bytes;
    self.verify();
    old_top
  }

  /// 原版 h:48-59 DecrementTopIfAdjacent: new_top 与现 top 恰相邻才回退
  #[inline]
  pub fn decrement_top_if_adjacent(&mut self, new_top: Address, bytes: usize) -> bool {
    self.verify();
    if new_top + bytes == self.top_ {
      self.top_ = new_top;
      if self.start_ > self.top_ {
        self.reset_start();
      }
      self.verify();
      return true;
    }
    false
  }

  /// 原版 h:61-64 SetLimit
  #[inline]
  pub fn set_limit(&mut self, limit: Address) {
    self.limit_ = limit;
    self.verify();
  }

  /// 原版 h:66-69 start()
  #[inline]
  pub fn start(&self) -> Address {
    self.verify();
    self.start_
  }

  /// 原版 h:70-73 top()
  #[inline]
  pub fn top(&self) -> Address {
    self.verify();
    self.top_
  }

  /// 原版 h:74-77 limit()
  #[inline]
  pub fn limit(&self) -> Address {
    self.verify();
    self.limit_
  }

  /// 原版 h:78 top_address() const — IsolateData 快照裸指针面 1:1 承接
  #[inline]
  pub fn top_address(&self) -> *const Address {
    &raw const self.top_
  }

  /// 原版 h:79 top_address() 可变版
  #[inline]
  pub fn top_address_mut(&mut self) -> *mut Address {
    &raw mut self.top_
  }

  /// 原版 h:80 limit_address() const
  #[inline]
  pub fn limit_address(&self) -> *const Address {
    &raw const self.limit_
  }

  /// 原版 h:81 limit_address() 可变版
  #[inline]
  pub fn limit_address_mut(&mut self) -> *mut Address {
    &raw mut self.limit_
  }

  /// 原版 h:83-93 Verify — DEBUG-only SLOW_DCHECK; 目标 release 配置为 no-op。
  /// Rust 以 debug_assert 承接同语义 (调试构建等价 SLOW_DCHECK, release 零成本)
  #[inline]
  pub fn verify(&self) {
    debug_assert!(self.start_ <= self.top_);
    debug_assert!(self.top_ <= self.limit_);
    // 原版按 8GB 压缩堆选对齐 (h:87-91); 目标配置恒 kObjectAlignment
    let align = if V8_COMPRESS_POINTERS_8GB {
      K_OBJECT_ALIGNMENT_8GB_HEAP
    } else {
      K_OBJECT_ALIGNMENT
    };
    debug_assert!(self.top_.is_multiple_of(align));
  }

  // ── 私有 offset 访问器 (原版 h:96-114, friend IsolateData 专用) ──
  // Rust 侧 pub 导出: IsolateData 票与布局金标准测试消费, 与原版友元面等价

  /// 原版 h:99-101 StartOffset
  pub const fn start_offset() -> i32 {
    0
  }

  /// 原版 h:102-104 TopOffset
  pub const fn top_offset() -> i32 {
    K_SYSTEM_POINTER_SIZE as i32
  }

  /// 原版 h:105-107 LimitOffset
  pub const fn limit_offset() -> i32 {
    2 * K_SYSTEM_POINTER_SIZE as i32
  }

  /// 原版 h:108-114 Size (原版内嵌 sizeof==3*kSystemPointerSize 断言, Rust 编译期恒检)
  pub const fn size() -> i32 {
    const {
      assert!(
        size_of::<LinearAllocationArea>() == 3 * K_SYSTEM_POINTER_SIZE,
        "LinearAllocationArea 须紧凑 (IsolateData 内嵌), 原版 h:109-111 同断言"
      );
    }
    size_of::<LinearAllocationArea>() as i32
  }
}
