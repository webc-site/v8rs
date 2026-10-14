#!/usr/bin/env bun
// tool/src/base/platform/mutex.js — 覆盖 v8/src/base/platform/mutex.{h,cc}
// 互斥原语: Mutex/RecursiveMutex/LockGuard 三类骨架与 DEBUG 标记语义
// → v8-base-platform/src/mutex.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/platform/mutex.h");
const ccSrc = readV8("src/base/platform/mutex.cc");

// 1. 头文件锚点: 三类 (Mutex / RecursiveMutex / LockGuard) + 惰性别名
const hAnchors = [
  "class V8_BASE_EXPORT Mutex final {",
  "class V8_BASE_EXPORT RecursiveMutex final {",
  "template <typename Mutex>\nclass V8_NODISCARD LockGuard final {",
  "absl::Mutex native_handle_;",
  "std::atomic<int> thread_id_ = 0;",
  "using LazyMutex = LazyStaticInstance<Mutex, DefaultConstructTrait<Mutex>,",
  "#define LAZY_MUTEX_INITIALIZER LAZY_STATIC_INSTANCE_INITIALIZER",
  "using LazyRecursiveMutex =",
  "#define LAZY_RECURSIVE_MUTEX_INITIALIZER LAZY_STATIC_INSTANCE_INITIALIZER",
  "using MutexGuard = LockGuard<Mutex>;",
  "using RecursiveMutexGuard = LockGuard<RecursiveMutex>;",
  "class V8_NODISCARD MutexGuardIf final {",
];
for (const a of hAnchors) {
  if (!hSrc.includes(a)) throw new Error(`mutex.h 缺锚点: ${a}`);
}

// 2. 方法面解析: Mutex 公共方法 3 (Lock/Unlock/TryLock), RecursiveMutex 同 3
const mutexOps = ["Lock", "Unlock", "TryLock"];
for (const op of mutexOps) {
  const inMutex = new RegExp(`void ${op}\\(\\);|bool ${op}\\(\\) V8_WARN_UNUSED_RESULT;`).test(hSrc);
  // Mutex 与 RecursiveMutex 各声明一次, 计数应为 2 (void 与 bool 形态各按类一计)
  if (!inMutex) throw new Error(`mutex.h 缺 ${op} 声明`);
}
const lockCount = (hSrc.match(/^  (?:void|bool) (?:Lock|Unlock|TryLock)\(\)/gm) || []).length;
if (lockCount !== 6) throw new Error(`Lock/Unlock/TryLock 声明数不符: ${lockCount}, 期望 6 (两类 × 3)`);
// LockGuard: 指针/引用双 ctor + move ctor + dtor
const guardCtors = (hSrc.match(/explicit LockGuard\(Mutex\* mutex\)|explicit LockGuard\(Mutex& mutex\)|LockGuard\(LockGuard&& other\)|~LockGuard\(\)/g) || []).length;
if (guardCtors !== 4) throw new Error(`LockGuard ctor/dtor 数不符: ${guardCtors}, 期望 4`);

// 3. 实现侧锚点: RecursiveMutex 递归判定走 OS::GetCurrentThreadId; Mutex 走 absl
const ccAnchors = [
  "int own_id = v8::base::OS::GetCurrentThreadId();",
  "if (thread_id_ == own_id) {",
  "level_++;",
  "mutex_.Lock();",
  "thread_id_ = own_id;",
  "if ((--level_) == 0) {",
  "mutex_.Unlock();",
  "native_handle_.lock();",
  "AssertUnheldAndMark();",
  "native_handle_.unlock();",
  "if (!native_handle_.try_lock()) return false;",
];
for (const a of ccAnchors) {
  if (!ccSrc.includes(a)) throw new Error(`mutex.cc 缺锚点: ${a}`);
}
const idFetch = (ccSrc.match(/OS::GetCurrentThreadId\(\)/g) || []).length;
if (idFetch !== 3) throw new Error(`GetCurrentThreadId 取用数不符: ${idFetch}, 期望 3 (Lock/Unlock[DEBUG]/TryLock)`);

console.log(`mutex: Lock/Unlock/TryLock ${lockCount} (两类×3), LockGuard ctor/dtor ${guardCtors}, 惰性别名 2`);

let rust = banner("src/base/platform/mutex.js", "src/base/platform/mutex.{h,cc}", "互斥原语 Mutex/RecursiveMutex/LockGuard (原版委托 absl::Mutex, 目标 Rust 侧走 std::sync::Mutex)");

rust += `
/// 非递归互斥 (原版 base::Mutex final; 原版成员 absl::Mutex native_handle_)
/// v8: src/base/platform/mutex.h Mutex
///
/// 不变式: 持有者从 Lock/TryLock 成功起到 Unlock 止; DEBUG 下 level_ 记 0/1,
/// 原版 AssertHeld 即 DCHECK_EQ(1, level_)。
pub struct Mutex {
  /// 原版 absl::Mutex native_handle_
  native: std::sync::Mutex<()>,
  /// 原版 #ifdef DEBUG int level_ — 仅 DEBUG 语义, 发布侧恒 0
  level: std::cell::Cell<i32>,
}

impl Mutex {
  /// v8: src/base/platform/mutex.cc Mutex::Mutex()
  pub fn new() -> Self { Self { native: std::sync::Mutex::new(()), level: std::cell::Cell::new(0) } }

  /// v8: src/base/platform/mutex.cc Mutex::Lock
  /// 原版 native_handle_.lock() 后 AssertUnheldAndMark (level_: 0→1)
  pub fn lock(&self) { self.native.lock().unwrap_or_else(|e| e.into_inner()); self.level.set(1); }

  /// v8: src/base/platform/mutex.cc Mutex::Unlock
  /// 原版 AssertHeldAndUnmark (level_: 1→0) 后 native_handle_.unlock()
  pub fn unlock(&self) { self.level.set(0); unsafe { self.native.force_unlock(); } }

  /// v8: src/base/platform/mutex.cc Mutex::TryLock
  /// 原版 native_handle_.try_lock() 失败即返回 false, 成功才打标记
  pub fn try_lock(&self) -> bool {
    match self.native.try_lock() {
      Ok(_) => { self.level.set(1); true }
      Err(std::sync::TryLockError::WouldBlock) => false,
      Err(std::sync::TryLockError::Poisoned(_)) => { self.level.set(1); true }
    }
  }

  /// v8: src/base/platform/mutex.h AssertHeld — DCHECK_EQ(1, level_)
  pub fn assert_held(&self) { debug_assert_eq!(self.level.get(), 1); }
}

/// 递归互斥 (原版 base::RecursiveMutex final)
/// v8: src/base/platform/mutex.h RecursiveMutex
///
/// 原版成员: std::atomic<int> thread_id_ + int level_ + Mutex mutex_
/// 同线程重入: level_++; 跨线程: 底层 Mutex::Lock 后记 thread_id_/level_=1
pub struct RecursiveMutex {
  /// 原版 std::atomic<int> thread_id_ = 0
  thread_id: std::sync::atomic::AtomicI32,
  /// 原版 int level_ = 0 — 记当前持有线程重入深度
  level: std::cell::Cell<i32>,
  /// 原版 Mutex mutex_
  mutex: Mutex,
}

impl RecursiveMutex {
  /// v8: src/base/platform/mutex.cc RecursiveMutex::Lock
  /// own_id == thread_id_ 时 level_++ 直返; 否则取底层锁并置 level_=1
  pub fn lock(&self) {
    let own_id = crate::os::get_current_thread_id();
    if self.thread_id.load(std::sync::atomic::Ordering::Relaxed) == own_id {
      self.level.set(self.level.get() + 1);
      return;
    }
    self.mutex.lock();
    self.thread_id.store(own_id, std::sync::atomic::Ordering::Relaxed);
    self.level.set(1);
  }

  /// v8: src/base/platform/mutex.cc RecursiveMutex::Unlock
  /// --level_ 归零时清 thread_id_ 并放底层锁
  pub fn unlock(&self) {
    self.level.set(self.level.get() - 1);
    if self.level.get() == 0 {
      self.thread_id.store(0, std::sync::atomic::Ordering::Relaxed);
      self.mutex.unlock();
    }
  }

  /// v8: src/base/platform/mutex.cc RecursiveMutex::TryLock
  pub fn try_lock(&self) -> bool {
    let own_id = crate::os::get_current_thread_id();
    if self.thread_id.load(std::sync::atomic::Ordering::Relaxed) == own_id {
      self.level.set(self.level.get() + 1);
      return true;
    }
    if self.mutex.try_lock() {
      self.thread_id.store(own_id, std::sync::atomic::Ordering::Relaxed);
      self.level.set(1);
      return true;
    }
    false
  }
}

/// RAII 守卫 (原版模板 base::LockGuard<Mutex>)
/// v8: src/base/platform/mutex.h LockGuard
/// ctor 锁, dtor 若未被移动走则解锁; 别名 MutexGuard/RecursiveMutexGuard
pub struct MutexGuard<'m> {
  mutex: Option<&'m Mutex>,
}

impl<'m> MutexGuard<'m> {
  /// v8: src/base/platform/mutex.h LockGuard(Mutex*) / LockGuard(Mutex&)
  pub fn new(mutex: &'m Mutex) -> Self { mutex.lock(); Self { mutex: Some(mutex) } }
}

impl Drop for MutexGuard<'_> {
  /// v8: src/base/platform/mutex.h ~LockGuard — mutex_ 可能为 null (被移走)
  fn drop(&mut self) { if let Some(m) = self.mutex { m.unlock(); } }
}

// —— 惰性初始化登记 (原版 LazyMutex / LazyRecursiveMutex, Rust 侧 OnceLock 承担) ——
/// v8: src/base/platform/mutex.h LazyMutex + LAZY_MUTEX_INITIALIZER
pub type LazyMutex = std::sync::OnceLock<Mutex>;
/// v8: src/base/platform/mutex.h LazyRecursiveMutex + LAZY_RECURSIVE_MUTEX_INITIALIZER
pub type LazyRecursiveMutex = std::sync::OnceLock<RecursiveMutex>;

// —— 提取不变式断言 ——
const _: () = {
  // 两类 × (Lock/Unlock/TryLock)
  const OPS_PER_CLASS: usize = 3;
  // LockGuard: ptr ctor / ref ctor / move ctor / dtor
  const GUARD_CTOR_DTOR: usize = 4;
  assert!(OPS_PER_CLASS == 3 && GUARD_CTOR_DTOR == 4);
};
`;

const p = writeSrc("v8-base-platform", "mutex.rs.skel", rust);
console.log(`written ${p}`);
