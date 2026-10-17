//! allocation：对应 `src/base/platform/memory.h`。
//! 现代 V8 已删除 `allocation.{cc,h}` 与 `GlobalAllocator` 钩子，因此以 `memory.h` 为准；
//! 相关差异登记在 readme。

use std::{ffi::c_void, mem::align_of, ptr};

/// 对应 V8 `malloc(size)`；size=0 返回 null。
pub fn malloc(size: usize) -> *mut c_void {
  if size == 0 {
    return ptr::null_mut();
  }
  // SAFETY: 分配 `size` 字节；调用方负责 `free`。
  unsafe { libc::malloc(size) }
}

/// 对应 V8 `realloc(memory, size)`；size=0 视为非法并返回原指针。
///
/// # Safety
/// `memory` 必须是此前 `malloc/realloc/calloc` 返回且尚未释放的指针，或为 null。
pub unsafe fn realloc(memory: *mut c_void, size: usize) -> *mut c_void {
  if size == 0 {
    // 原 V8 `CHECK_NE(0, size)`；这里返回原指针避免 panic/abort。
    return memory;
  }
  // SAFETY: 由调用方保证 memory 与 size 匹配；realloc 允许 null。
  unsafe { libc::realloc(memory, size) }
}

/// 对应 V8 `free(memory)`。
///
/// # Safety
/// `memory` 必须是此前由 `malloc/realloc/calloc` 返回且尚未释放的指针，或为 null。
pub unsafe fn free(memory: *mut c_void) {
  // SAFETY: 由调用方保证 `memory` 来源。
  unsafe {
    libc::free(memory);
  }
}

/// 对应 V8 `calloc(count, size)`；size=0 允许（返回 null 或空对象指针均可，此处按 calloc 语义）。
pub fn calloc(count: usize, size: usize) -> *mut c_void {
  // SAFETY: calloc 语义由 libc 提供。
  unsafe { libc::calloc(count, size) }
}

/// 对应 V8 `aligned_alloc(size, alignment)`；alignment 必须为 2 的幂且不小于 `alignof(void*)`。
/// 参数不合法返回 null。
pub fn aligned_alloc(size: usize, alignment: usize) -> *mut c_void {
  if alignment == 0 || !alignment.is_power_of_two() {
    return ptr::null_mut();
  }
  let min_align = align_of::<*mut c_void>();
  if alignment < min_align {
    return ptr::null_mut();
  }
  if size == 0 {
    // posix_memalign(0) 行为不确定；返回 null。
    return ptr::null_mut();
  }
  #[cfg(unix)]
  {
    let mut p: *mut c_void = ptr::null_mut();
    // SAFETY: `posix_memalign` 是 POSIX 定义调用，`&mut p` 有效。
    let rc = unsafe { libc::posix_memalign(&mut p, alignment, size) };
    if rc != 0 { ptr::null_mut() } else { p }
  }
  #[cfg(windows)]
  {
    unsafe extern "C" {
      fn _aligned_malloc(size: usize, alignment: usize) -> *mut c_void;
    }
    unsafe { _aligned_malloc(size, alignment) }
  }
  #[cfg(not(any(unix, windows)))]
  {
    let _ = (size, alignment);
    ptr::null_mut()
  }
}

/// 对应 V8 `aligned_free(ptr)`；POSIX 下即 `free`，Windows 下为 `_aligned_free`。
///
/// # Safety
/// `ptr` 必须由 `aligned_alloc` 分配。
pub unsafe fn aligned_free(ptr: *mut c_void) {
  #[cfg(unix)]
  unsafe {
    libc::free(ptr);
  }
  #[cfg(windows)]
  {
    unsafe extern "C" {
      fn _aligned_free(ptr: *mut c_void);
    }
    unsafe {
      _aligned_free(ptr);
    }
  }
  #[cfg(not(any(unix, windows)))]
  {
    let _ = ptr;
  }
}

/// 对应 V8 `malloc_usable_size(ptr)`；darwin 走 `malloc_size`，linux 走 `malloc_usable_size`，windows 走 `_msize`。
///
/// # Safety
/// `ptr` 必须为 null 或此前由 malloc/realloc/calloc/memalign 分配且尚未释放的指针。
pub unsafe fn malloc_usable_size(ptr: *mut c_void) -> usize {
  if ptr.is_null() {
    return 0;
  }
  #[cfg(target_os = "macos")]
  {
    // SAFETY: `malloc_size` 接受任意此前 malloc/realloc/calloc/memalign 分配的指针。
    unsafe { libc::malloc_size(ptr) }
  }
  #[cfg(target_os = "linux")]
  {
    // SAFETY: 同上。
    unsafe { libc::malloc_usable_size(ptr) as usize }
  }
  #[cfg(target_os = "windows")]
  {
    unsafe extern "C" {
      fn _msize(ptr: *mut c_void) -> usize;
    }
    unsafe { _msize(ptr) }
  }
  #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
  {
    let _ = ptr;
    0
  }
}

/// 对应 V8 `AllocationResult<T*>`。
#[derive(Debug, Clone, Copy)]
pub struct AllocationResult<P> {
  pub ptr: P,
  pub count: usize,
}

/// 对应 V8 `allocate_at_least<T>(n)`。这里以 u8 元素为默认；调用方自行 cast。
/// 分配失败返回 `{null, 0}`（原版 memory.h:144-163：usable_size 为 0 时 count=0，
/// 调用方以 ptr==null 判失败）。
pub fn allocate_at_least(n: usize) -> AllocationResult<*mut c_void> {
  let memory = malloc(n);
  // SAFETY: `memory` 来自刚分配的 `malloc`（或失败 null）。
  let usable = unsafe { malloc_usable_size(memory) };
  AllocationResult {
    ptr: memory,
    count: usable,
  }
}
