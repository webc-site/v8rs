//! 原版 src/zone/zone-segment.h/cc 转写
//!
//! Segment 是内存块的头部: 位于分配块起始处, next 链构成 LIFO 结构,
//! 最新段即 Zone 的 segment_head_。块由 AccountingAllocator 以 malloc 语义分配。

use std::{cell::Cell, mem, ptr};

use crate::{
  internal::bits::{ALIGNMENT, align_up},
  zone::core::Zone,
};

/// 原版 Segment 的调试填充字节
pub(crate) const ZAP_DEAD_BYTE: u8 = 0xcd;

/// 内存块 + 头部; 布局对齐原版 {zone_, next_, size_}
#[repr(C)]
pub struct Segment {
  /// 归属 zone 的类型擦除指针 (原版 set_zone/zone(), 仅记账/调试用)
  zone: Cell<*const ()>,
  /// LIFO 链后继
  next: Cell<*mut Segment>,
  /// 含头部的总大小
  size: usize,
}

impl Segment {
  /// 原版 Segment(size); 只允许 AccountingAllocator 在放置写入时构造
  pub(crate) fn new(size: usize) -> Segment {
    Segment {
      zone: Cell::new(ptr::null()),
      next: Cell::new(ptr::null_mut()),
      size,
    }
  }

  /// 原版 set_zone
  pub(crate) fn set_zone(&self, zone: &Zone<'_>) {
    self.zone.set(ptr::from_ref(zone).cast());
  }

  /// 原版 zone()
  pub fn zone_ptr(&self) -> *const () {
    self.zone.get()
  }

  /// 原版 next()
  pub(crate) fn next(&self) -> *mut Segment {
    self.next.get()
  }

  /// 原版 set_next
  pub(crate) fn set_next(&self, next: *mut Segment) {
    self.next.set(next);
  }

  /// 原版 total_size(): 含头部簿记字节的分配总大小
  pub fn total_size(&self) -> usize {
    self.size
  }

  /// 原版 capacity(): 可存储字节数
  pub fn capacity(&self) -> usize {
    self.size - mem::size_of::<Segment>()
  }

  /// 原版 start(): 可存储区起始地址 (头部之后)
  pub fn start(&self) -> usize {
    self.address(mem::size_of::<Segment>())
  }

  /// 原版 end(): 可存储区结束地址
  pub fn end(&self) -> usize {
    self.address(self.size)
  }

  /// 原版 address(n): 本段第 n 字节地址
  fn address(&self, n: usize) -> usize {
    ptr::from_ref(self) as usize + n
  }

  /// 原版 start() 处按 ALIGNMENT 上取整 (Zone 复用该结果作为 position)
  pub fn aligned_start(&self) -> usize {
    align_up(self.start(), ALIGNMENT)
  }

  /// 原版 Segment::ZapContents(): 调试构建下填充内容区。
  /// 以 raw 指针直算偏移: 不经 &Segment 共享 retag 与 int→ptr 回转。
  /// Miri SB 下 poison 写会使同段存活派生 tag 出栈 (C++ DEBUG memset 无借用
  /// 追踪对应物, 纯诊断填充), miri 构建跳过
  pub(crate) fn zap_contents(this: *mut Segment) {
    if cfg!(all(debug_assertions, not(miri))) {
      // SAFETY: this 由 AccountingAllocator 持有, [start, end) 为合法可写内存
      unsafe {
        ptr::write_bytes(
          this.cast::<u8>().add(mem::size_of::<Segment>()),
          ZAP_DEAD_BYTE,
          (*this).size - mem::size_of::<Segment>(),
        );
      }
    }
  }

  /// 原版 Segment::ZapHeader(): 调试构建下填充头部使段不可用 (miri 构建跳过,
  /// 理由同 zap_contents)
  pub(crate) fn zap_header(this: *mut Segment) {
    if cfg!(all(debug_assertions, not(miri))) {
      // SAFETY: 头部自身内存由分配器持有且此时无活跃引用
      unsafe {
        ptr::write_bytes(this.cast::<u8>(), ZAP_DEAD_BYTE, mem::size_of::<Segment>());
      }
    }
  }

  /// 调试包含判定辅助: 地址是否落于本段 [start, end)
  pub(crate) fn contains_address(&self, address: usize) -> bool {
    address >= self.start() && address < self.end()
  }
}
