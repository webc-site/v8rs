//! bump 内核: 当前 chunk 的 [position, limit) 抬升指针
//!
//! 原版 Zone 的 position_/limit_ 字段封装; 快速路径 = 单次比较 + 指针抬升。
//! position/limit 以真实指针承载 (provenance 派生自段 raw 指针链), 分配写路径
//! 不经 int→ptr 回转 (Miri Stacked Borrows 兼容); 对外口径 (记账/快照) 仍以
//! 地址整数表达, 对应原版 `Address position_`。

use std::{mem, ptr};

use crate::{
  internal::bits::{ALIGNMENT, align_up, is_aligned},
  zone::segment::Segment,
};

/// 当前可 bump 的半开区间 [position, limit); 均为对齐地址
#[derive(Clone, Copy)]
pub(crate) struct Chunk {
  pub position: *mut u8,
  pub limit: *mut u8,
}

impl Chunk {
  /// 空 chunk (未分配任何段)
  pub const EMPTY: Chunk = Chunk {
    position: ptr::null_mut(),
    limit: ptr::null_mut(),
  };

  /// 段 raw 指针 → 对齐后的内容区 [position, limit) 视图。
  /// 偏移经整数计算, 指针经 raw add 保留 provenance (真实可写链)
  ///
  /// # Safety
  /// seg 须为由 AccountingAllocator 分配且头字段有效的段指针
  pub(crate) unsafe fn from_segment(seg: *mut Segment) -> Chunk {
    let base = seg as usize;
    let aligned = align_up(base + mem::size_of::<Segment>(), ALIGNMENT) - base;
    // SAFETY: [aligned, total_size) 落在段分配范围内 (调用方保证段头有效)
    let position = unsafe { seg.cast::<u8>().add(aligned) };
    let limit = unsafe { seg.cast::<u8>().add((*seg).total_size()) };
    Chunk { position, limit }
  }

  /// 原版 allocation_size 的 extra 分量: position 相对基准的整数字节数
  pub fn position_addr(self) -> usize {
    self.position as usize
  }

  #[inline]
  pub fn remaining(self) -> usize {
    self.limit as usize - self.position as usize
  }

  /// 原版 AllocateUnchecked: 空间足够则抬升 position 并返回地址
  #[inline]
  pub fn try_bump(&mut self, size: usize) -> Option<*mut u8> {
    debug_assert!(self.position_addr() <= self.limit as usize);
    debug_assert!(is_aligned(self.position_addr(), ALIGNMENT));
    if size <= self.remaining() {
      let result = self.position;
      // SAFETY: result + size <= limit, 段内合法范围
      self.position = unsafe { result.add(size) };
      Some(result)
    } else {
      None
    }
  }
}
