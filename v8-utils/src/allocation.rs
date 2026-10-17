// 由 tool/src/utils/consts.js 从 v8/src/utils/allocation.h;src/utils/allocation.cc 生成; 修订就地改, 复跑覆盖

//! 内存分配辅助, 移植自 v8/src/utils/allocation 的可独立子集。
//!
//! 依赖 v8::PageAllocator 平台层的部分 (VirtualMemory/VirtualMemoryCage/
//! AllocatePages 等) 归 v8-base platform, 见 README 映射表。

/// 分配重试次数 (每次失败触发 OnCriticalMemoryPressure 释放内存)
pub const ALLOCATION_TRIES: usize = 2;

/// 地址空间保留是否用于可执行页 (JitPermission)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JitPermission {
  NoJit,
  MapAsJittable,
}

use std::{
  alloc::{Layout, alloc, dealloc},
  ptr::NonNull,
};

/// `StrNDup` (allocation.h): 截取前 n 字节为合法 UTF-8 (多字节字符不撕裂,
/// 落在字符中间回退到字符边界; 上游 C 串语义的 Rust 安全收敛)。
#[must_use]
pub fn str_n_dup(s: &str, n: usize) -> String {
  if s.len() <= n {
    return s.to_owned();
  }
  // 回退到字符边界 (is_char_boundary)
  let mut end = n.min(s.len());
  while end > 0 && !s.is_char_boundary(end) {
    end -= 1;
  }
  s[..end].to_owned()
}

/// `AllocateAtLeast` 风格对齐分配 (allocation.cc AlignedAllocWithRetry):
/// size 0 / 非法对齐返回 None; 失败重试 ALLOCATION_TRIES 次。
///
/// # Safety
/// 返回块须以同 (size, alignment) 传给 `aligned_free` 释放; 期间不得越界。
pub unsafe fn aligned_alloc_with_retry(size: usize, alignment: usize) -> Option<NonNull<u8>> {
  if size == 0 {
    return None;
  }
  let Ok(layout) = Layout::from_size_align(size, alignment) else {
    return None;
  };
  for _ in 0..=ALLOCATION_TRIES {
    // SAFETY: layout 合法 (上面校验)
    if let Some(p) = NonNull::new(unsafe { alloc(layout) }) {
      return Some(p);
    }
    // 原版重试前触发 OnCriticalMemoryPressure 释放内存 (平台回调,
    // 独立版无挂点, 仅重试)
  }
  None
}

/// `AlignedFree` (allocation.cc): 释放 `aligned_alloc_with_retry` 的块。
///
/// # Safety
/// ptr 必须来自同 (size, alignment) 的 `aligned_alloc_with_retry` 且未释放。
pub unsafe fn aligned_free(ptr: NonNull<u8>, size: usize, alignment: usize) {
  if let Ok(layout) = Layout::from_size_align(size, alignment) {
    // SAFETY: 调用方契约保证指针来源与存活
    unsafe { dealloc(ptr.as_ptr(), layout) };
  }
}

/// `AllocatePageSize` (allocation.h): 平台分配页粒度。
#[must_use]
pub fn allocate_page_size() -> usize {
  #[cfg(unix)]
  {
    // SAFETY: sysconf 常量查询, 无前置条件
    let ps = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    usize::try_from(ps).unwrap_or(4096)
  }
  #[cfg(not(unix))]
  {
    4096
  }
}

/// `CommitPageSize` (allocation.h): 提交页粒度 (POSIX 与分配页一致)。
#[must_use]
pub fn commit_page_size() -> usize {
  allocate_page_size()
}
