//! Stack：对应 V8 `class Stack` in `platform.h / platform-posix.cc / platform.cc`。
//!
//! 偏差与平台实现：
//! - Windows 下通过 `GetCurrentThreadStackLimits` 获取当前线程栈底 (StackBase) 与保留下界；
//! - ASan fake-frame 支持 (`GetRealStackAddressForSlot`) 保留为恒等函数；
//! - V8 用 `thread_local` 缓存 stack_start/reserved_limit；本实现同样缓存。

#[cfg(target_os = "linux")]
use std::mem::zeroed;
#[cfg(target_os = "linux")]
use std::ptr::null_mut;
use std::{cell::OnceCell, ffi::c_void, hint::black_box, ptr::addr_of};

/// 与 V8 `Stack::StackSlot` 等价；使用 `usize` 承载指针或整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StackSlot(pub usize);

impl StackSlot {
  #[inline]
  pub fn from_ptr(p: *const ()) -> Self {
    Self(p as usize)
  }
  #[inline]
  pub fn from_mut_ptr(p: *mut c_void) -> Self {
    Self(p as usize)
  }
  #[inline]
  pub const fn value(self) -> usize {
    self.0
  }
  #[inline]
  pub fn as_ptr(self) -> *const () {
    self.0 as *const ()
  }
}

thread_local! {
  static STACK_START: OnceCell<StackSlot> = const { OnceCell::new() };
  static STACK_RESERVED_LIMIT: OnceCell<StackSlot> = const { OnceCell::new() };
}

/// 与 V8 `Stack` 对应。
pub struct Stack;

impl Stack {
  /// 对应 V8 `Stack::GetStackStart()`。
  pub fn get_stack_start() -> StackSlot {
    STACK_START.with(|c| *c.get_or_init(Self::obtain_current_thread_stack_start))
  }

  /// 对应 V8 `Stack::GetReservedStackLimit()`。
  pub fn get_reserved_stack_limit() -> StackSlot {
    STACK_RESERVED_LIMIT.with(|c| *c.get_or_init(Self::obtain_current_thread_stack_reserved_limit))
  }

  /// Windows 下通过 GetCurrentThreadStackLimits 获取提交栈下界，其他平台返回 0。
  #[cfg(not(windows))]
  pub fn get_committed_stack_limit() -> StackSlot {
    StackSlot(0)
  }

  #[cfg(windows)]
  pub fn get_committed_stack_limit() -> StackSlot {
    let (low, _high) = win_stack_limits();
    StackSlot(low)
  }

  /// Windows-only；本实现为 no-op。
  pub fn set_current_thread_stack_bounds(_limit: usize, _base: usize) {}

  /// 对应 V8 `Stack::GetCurrentStackPosition()`；`inline(never)` 保留原语义。
  #[inline(never)]
  pub fn get_current_stack_position() -> StackSlot {
    // 借用局部变量地址；使用 black_box 防止优化将其提升到调用者帧之上。
    let marker: u8 = 0;
    let p = addr_of!(marker) as usize;
    black_box(&marker);
    StackSlot(p)
  }

  /// 对应 V8 `Stack::GetCurrentFrameAddress()`；由于 Rust 无 `__builtin_frame_address`，
  /// 本实现与 `get_current_stack_position()` 等价。
  #[inline(never)]
  pub fn get_current_frame_address() -> StackSlot {
    Self::get_current_stack_position()
  }

  /// 对应 V8 `Stack::GetRealStackAddressForSlot()`；未启用 ASan 时即原 slot。
  pub fn get_real_stack_address_for_slot(slot: StackSlot) -> StackSlot {
    slot
  }

  #[cfg(target_os = "macos")]
  fn obtain_current_thread_stack_start() -> StackSlot {
    // SAFETY: `pthread_self` 与 `pthread_get_stackaddr_np` 均为 darwin 只读调用。
    unsafe {
      let self_pthread = libc::pthread_self();
      let addr = pthread_get_stackaddr_np(self_pthread);
      StackSlot::from_mut_ptr(addr)
    }
  }

  #[cfg(target_os = "linux")]
  fn obtain_current_thread_stack_start() -> StackSlot {
    // 起始 = stackaddr + stacksize。
    let (base, size) = linux_stack_base_and_size();
    StackSlot(base + size)
  }

  #[cfg(windows)]
  fn obtain_current_thread_stack_start() -> StackSlot {
    let (_low, high) = win_stack_limits();
    if high != 0 {
      StackSlot(high)
    } else {
      Self::get_current_stack_position()
    }
  }

  #[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
  fn obtain_current_thread_stack_start() -> StackSlot {
    // 回退：取当前栈位置。
    Self::get_current_stack_position()
  }

  #[cfg(target_os = "macos")]
  fn obtain_current_thread_stack_reserved_limit() -> StackSlot {
    // SAFETY: 见上。
    unsafe {
      let self_pthread = libc::pthread_self();
      let addr = pthread_get_stackaddr_np(self_pthread);
      let size = pthread_get_stacksize_np(self_pthread);
      StackSlot((addr as usize) - size)
    }
  }

  #[cfg(target_os = "linux")]
  fn obtain_current_thread_stack_reserved_limit() -> StackSlot {
    let (base, _size) = linux_stack_base_and_size();
    StackSlot(base)
  }

  #[cfg(windows)]
  fn obtain_current_thread_stack_reserved_limit() -> StackSlot {
    let (low, _high) = win_stack_limits();
    StackSlot(low)
  }

  #[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
  fn obtain_current_thread_stack_reserved_limit() -> StackSlot {
    StackSlot(0)
  }
}

#[cfg(windows)]
mod win_stack {
  #[link(name = "kernel32")]
  unsafe extern "system" {
    pub fn GetCurrentThreadStackLimits(low_limit: *mut usize, high_limit: *mut usize);
  }
}

#[cfg(windows)]
fn win_stack_limits() -> (usize, usize) {
  let mut low: usize = 0;
  let mut high: usize = 0;
  // SAFETY: low 与 high 为本栈帧上的有效局部变量指针，满足 GetCurrentThreadStackLimits 的双输出参数写入契约。
  unsafe {
    win_stack::GetCurrentThreadStackLimits(&mut low, &mut high);
  }
  (low, high)
}

#[cfg(target_os = "macos")]
unsafe extern "C" {
  fn pthread_get_stackaddr_np(thread: libc::pthread_t) -> *mut libc::c_void;
  fn pthread_get_stacksize_np(thread: libc::pthread_t) -> usize;
}

#[cfg(target_os = "linux")]
fn linux_stack_base_and_size() -> (usize, usize) {
  // SAFETY: `pthread_getattr_np` 是 POSIX 调用；栈局部 attr，读取后置空。
  unsafe {
    let mut attr: libc::pthread_attr_t = zeroed();
    if libc::pthread_getattr_np(libc::pthread_self(), &mut attr) != 0 {
      return (0, 0);
    }
    let mut base: *mut c_void = null_mut();
    let mut size: usize = 0;
    if libc::pthread_attr_getstack(&attr, &mut base, &mut size) != 0 {
      libc::pthread_attr_destroy(&mut attr);
      return (0, 0);
    }
    libc::pthread_attr_destroy(&mut attr);
    (base as usize, size)
  }
}
