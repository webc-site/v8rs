//! Mutex / RecursiveMutex / LockGuard / MutexGuard / MutexGuardIf。
//! 对应原 `src/base/platform/mutex.{h,cc}`。
//!
//! 偏差：现代 V8 使用 `absl::Mutex` 作为底层。这里选择 `parking_lot::RawMutex`
//! （公开且与 V8 API 语义对齐：`Lock/Unlock/TryLock`），不再引入 `absl`。
//! 由于 Rust 无内建 thread-id，`RecursiveMutex` 使用 `libc::pthread_self()`。

use std::{
  fmt::{self, Debug, Formatter},
  marker::PhantomData,
  sync::atomic::{AtomicU32, AtomicU64, Ordering},
};

use parking_lot::{RawMutex, lock_api::RawMutex as _};
use v8_base::lazy_instance::LazyInstance;

use crate::thread_id::current_thread_id;

/// V8 `Mutex` 的对等实现；`Lock/Unlock/TryLock` 语义 1:1。
///
/// V8 版本在 `Mutex` 上直接暴露 `Lock/Unlock/TryLock`，无 RAII guard。Rust 中若
/// 直接把锁嵌进结构，用户仍需要手动 `unlock`，容易遗漏；本实现同时提供
/// `lock_guard()` 便捷方法返回 `LockGuard<Mutex>`。
pub struct Mutex {
  inner: RawMutex,
  _not_send_sync_marker: PhantomData<()>,
}

// SAFETY: `parking_lot::RawMutex` 内部保证跨线程安全；本类型未附加额外约束。
unsafe impl Send for Mutex {}
// SAFETY: 同上。
unsafe impl Sync for Mutex {}

impl Debug for Mutex {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("Mutex")
      .field("locked", &self.inner.is_locked())
      .finish()
  }
}

impl Default for Mutex {
  fn default() -> Self {
    Self::new()
  }
}

impl Mutex {
  #[inline]
  pub fn new() -> Self {
    Self {
      inner: RawMutex::INIT,
      _not_send_sync_marker: PhantomData,
    }
  }

  /// 对应 V8 `Mutex::Lock()`。
  #[inline]
  pub fn lock(&self) {
    self.inner.lock();
  }

  /// 对应 V8 `Mutex::Unlock()`。调用方必须持有锁。
  ///
  /// # Safety
  /// 调用方必须已持有本锁；否则行为未定义。
  #[inline]
  pub unsafe fn unlock(&self) {
    // SAFETY: 由调用方保证前置条件。
    unsafe {
      self.inner.unlock();
    }
  }

  /// 对应 V8 `Mutex::TryLock()`；返回是否成功获取。
  #[inline]
  pub fn try_lock(&self) -> bool {
    self.inner.try_lock()
  }

  /// 对应 V8 `Mutex::AssertHeld()`（mutex.h:58-62）：持锁状态自检。
  /// `RawMutex` 无所有者计数，以 `is_locked` 承接（`Debug::fmt` 同款探测）。
  #[inline]
  pub fn assert_held(&self) {
    debug_assert!(self.inner.is_locked(), "mutex must be held");
  }

  /// 便捷 RAII：获取并返回 `LockGuard<Mutex>`。
  #[inline]
  pub fn lock_guard(&self) -> LockGuard<'_, Mutex> {
    LockGuard::new(self)
  }
}

/// V8 `RecursiveMutex` 的对等实现；线程内可重入，`level_` 记录深度。
pub struct RecursiveMutex {
  inner: RawMutex,
  owner: AtomicU64,
  level: AtomicU32,
  _marker: PhantomData<()>,
}

// SAFETY: `RawMutex` 与原子变量都是 `Send + Sync`。
unsafe impl Send for RecursiveMutex {}
// SAFETY: 同上。
unsafe impl Sync for RecursiveMutex {}

impl Debug for RecursiveMutex {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("RecursiveMutex")
      .field("owner", &self.owner.load(Ordering::Relaxed))
      .field("level", &self.level.load(Ordering::Relaxed))
      .finish()
  }
}

impl Default for RecursiveMutex {
  fn default() -> Self {
    Self::new()
  }
}

impl RecursiveMutex {
  #[inline]
  pub fn new() -> Self {
    Self {
      inner: RawMutex::INIT,
      owner: AtomicU64::new(0),
      level: AtomicU32::new(0),
      _marker: PhantomData,
    }
  }

  /// 对应 V8 `RecursiveMutex::Lock()`。
  pub fn lock(&self) {
    let tid = current_thread_id();
    if self.owner.load(Ordering::Acquire) == tid {
      self.level.fetch_add(1, Ordering::Relaxed);
      return;
    }
    self.inner.lock();
    self.owner.store(tid, Ordering::Release);
    self.level.store(1, Ordering::Relaxed);
  }

  /// 对应 V8 `RecursiveMutex::Unlock()`。
  ///
  /// # Safety
  /// 调用方必须持有本锁；否则行为未定义。
  pub unsafe fn unlock(&self) {
    debug_assert_eq!(
      self.owner.load(Ordering::Relaxed),
      current_thread_id(),
      "RecursiveMutex::unlock called by thread that does not own it"
    );
    let cur = self.level.load(Ordering::Relaxed);
    debug_assert!(cur > 0, "RecursiveMutex::unlock called without ownership");
    let new_level = cur.saturating_sub(1);
    self.level.store(new_level, Ordering::Relaxed);
    if new_level == 0 {
      self.owner.store(0, Ordering::Release);
      // SAFETY: 由调用方保证持有。
      unsafe {
        self.inner.unlock();
      }
    }
  }

  /// 对应 V8 `RecursiveMutex::TryLock()`。同线程重入返回 true。
  pub fn try_lock(&self) -> bool {
    let tid = current_thread_id();
    if self.owner.load(Ordering::Acquire) == tid {
      self.level.fetch_add(1, Ordering::Relaxed);
      return true;
    }
    if !self.inner.try_lock() {
      return false;
    }
    self.owner.store(tid, Ordering::Release);
    self.level.store(1, Ordering::Relaxed);
    true
  }

  /// 对应 V8 `RecursiveMutex::AssertHeld()`（mutex.h:153-157）：
  /// `DCHECK_LT(0, level_)` → level > 0 且属当前线程。
  #[inline]
  pub fn assert_held(&self) {
    debug_assert!(
      self.level.load(Ordering::Relaxed) > 0
        && self.owner.load(Ordering::Acquire) == current_thread_id(),
      "recursive mutex must be held by current thread"
    );
  }
}

/// 原 `LazyMutex`（mutex.h:90-103）：`LazyStaticInstance<Mutex, DefaultConstructTrait,
/// ThreadSafeInitOnceTrait>`——静态存储 POD，首次 `Pointer()` 时线程安全地默认构造。
/// 已回收 → `v8-base::lazy_instance::LazyInstance`（与原版 platform→lazy-instance 的
/// include 方向一致）；配合 `static` 绑定即为原版的全局单例用法。
pub struct LazyMutex {
  inner: LazyInstance<Mutex>,
}

/// 原 `LAZY_MUTEX_INITIALIZER` 宏（mutex.h:105）：const 构造入口 [`LazyMutex::new`]，
/// 无运行时分配、不可能 panic；`Mutex` 的默认构造推迟到首次 `pointer()`。
/// 原版宏展开为 POD `{0}` 初始化器，逐项复用时各自惰性初始化，语义一致
/// （`LazyInstance` 含内部可变单元格，静态单例承接形态为
/// `static M: LazyMutex = LazyMutex::new();`，const 具名值会触发
/// clippy::declare_interior_mutable_const，故以 const fn 直用）。
impl LazyMutex {
  /// const 构造入口；等价原版初始化器。
  #[inline]
  pub const fn new() -> Self {
    Self {
      inner: LazyInstance::new(),
    }
  }

  /// 对应原 `Pointer()`：首次调用构造并返回内部 `Mutex`。
  #[inline]
  pub fn pointer(&self) -> &Mutex {
    self.inner.get()
  }
}

impl Default for LazyMutex {
  fn default() -> Self {
    Self::new()
  }
}

/// 原 `LazyRecursiveMutex`（mutex.h:167-177）：同 `LazyMutex` 的递归版。
pub struct LazyRecursiveMutex {
  inner: LazyInstance<RecursiveMutex>,
}

/// 原 `LAZY_RECURSIVE_MUTEX_INITIALIZER` 宏（mutex.h:177）：同 [`LazyMutex`] 的
/// const fn 直用形态（见上方 interior mutability 说明）。
impl LazyRecursiveMutex {
  #[inline]
  pub const fn new() -> Self {
    Self {
      inner: LazyInstance::new(),
    }
  }

  /// 对应原 `Pointer()`：首次调用构造并返回内部 `RecursiveMutex`。
  #[inline]
  pub fn pointer(&self) -> &RecursiveMutex {
    self.inner.get()
  }
}

impl Default for LazyRecursiveMutex {
  fn default() -> Self {
    Self::new()
  }
}

/// 通用 trait：让 `LockGuard<M>` 同时支持 `Mutex` 与 `RecursiveMutex`。
pub trait Lockable {
  fn acquire(&self);
  /// # Safety
  /// 调用方必须持有该锁。
  unsafe fn release(&self);
  fn try_acquire(&self) -> bool;
}

impl Lockable for Mutex {
  #[inline]
  fn acquire(&self) {
    Mutex::lock(self)
  }
  #[inline]
  unsafe fn release(&self) {
    // SAFETY: 由调用方保证。
    unsafe { Mutex::unlock(self) }
  }
  #[inline]
  fn try_acquire(&self) -> bool {
    Mutex::try_lock(self)
  }
}

impl Lockable for RecursiveMutex {
  #[inline]
  fn acquire(&self) {
    RecursiveMutex::lock(self)
  }
  #[inline]
  unsafe fn release(&self) {
    // SAFETY: 由调用方保证。
    unsafe { RecursiveMutex::unlock(self) }
  }
  #[inline]
  fn try_acquire(&self) -> bool {
    RecursiveMutex::try_lock(self)
  }
}

/// V8 `LockGuard<Mutex>` 的对等实现：RAII，可移动；移动后置空。
#[derive(Debug)]
pub struct LockGuard<'a, M: Lockable> {
  mutex: Option<&'a M>,
}

impl<'a, M: Lockable> LockGuard<'a, M> {
  /// 对应 V8 `explicit LockGuard(Mutex*)`。
  pub fn new(mutex: &'a M) -> Self {
    mutex.acquire();
    Self { mutex: Some(mutex) }
  }

  /// 对应 V8 `bool TryLock` 语义：若失败返回 `None`。
  pub fn try_new(mutex: &'a M) -> Option<Self> {
    if mutex.try_acquire() {
      Some(Self { mutex: Some(mutex) })
    } else {
      None
    }
  }

  /// 显式释放；返回原 `mutex` 引用（若已释放则返回 `None`）。析构时不再解锁。
  pub fn into_inner(mut self) -> Option<&'a M> {
    self.mutex.take()
  }

  /// 显式解锁，锁对象仍可继续使用。
  pub fn release(&mut self) {
    if let Some(m) = self.mutex.take() {
      // SAFETY: guard 存在即持有锁。
      unsafe {
        m.release();
      }
    }
  }
}

impl<M: Lockable> Drop for LockGuard<'_, M> {
  fn drop(&mut self) {
    if let Some(m) = self.mutex.take() {
      // SAFETY: guard 存在即持有锁。
      unsafe {
        m.release();
      }
    }
  }
}

impl<M: Lockable> LockGuard<'_, M> {
  /// 返回内部 `Mutex` 引用；若 guard 已释放，返回 `None`。
  pub fn mutex(&self) -> Option<&M> {
    self.mutex
  }
}

/// 别名：`MutexGuard = LockGuard<Mutex>`。
pub type MutexGuard<'a> = LockGuard<'a, Mutex>;
/// 别名：`RecursiveMutexGuard = LockGuard<RecursiveMutex>`。
pub type RecursiveMutexGuard<'a> = LockGuard<'a, RecursiveMutex>;

/// V8 `MutexGuardIf(mutex, enable_mutex)`：可选加锁 guard。
#[derive(Debug)]
pub struct MutexGuardIf<'a> {
  guard: Option<MutexGuard<'a>>,
}

impl<'a> MutexGuardIf<'a> {
  pub fn new(mutex: &'a Mutex, enable: bool) -> Self {
    Self {
      guard: if enable {
        Some(LockGuard::new(mutex))
      } else {
        None
      },
    }
  }

  /// 是否实际持有 mutex；对应 V8 中的 `mutex_.has_value()`。
  pub fn is_enabled(&self) -> bool {
    self.guard.is_some()
  }
}
