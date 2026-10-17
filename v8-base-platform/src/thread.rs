//! Thread：对应原 `src/base/platform/platform.h` 中的 `Thread` 类与
//! `platform-posix.cc` 中的 TLS / start/join / priority 逻辑。
//!
//! 偏差：现代 V8 `Thread` 是抽象基类，用户继承并重写 `Run()`。Rust 无虚函数，
//! 这里以 `Box<dyn FnOnce() + Send>` 作为闭包形式的 `Run`；其他 1:1 语义保留。
//! - `Start / StartSynchronously / Join` 与 V8 一致；`join` 前禁止 drop，否则记为泄漏（本实现
//!   提供 `JoinHandle` 弱保证，drop 不 join 时按 detached 语义处理，登记在 readme）。
//! - TLS 使用 `pthread_key_create/delete/getspecific/setspecific`。
//!   `LocalStorageKey` 直接用 `libc::pthread_key_t`，与原 `int32_t` 语义等价但更宽。
//! - Priority 在闭包内设置：darwin 走 `pthread_set_qos_class_self_np`；
//!   linux 走 `setpriority(PRIO_PROCESS, 0, nice)`。

use std::{
  ffi::c_void,
  fmt::{self, Debug, Formatter},
  sync::Arc,
  thread::{Builder, JoinHandle},
};

use crate::{
  error::{Error, Result},
  semaphore::Semaphore,
};

/// 线程优先级；对应 V8 `Thread::Priority`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
  BestEffort,
  UserVisible,
  UserBlocking,
  Default,
}

/// 线程名最大长度；对应 V8 `kMaxThreadNameLength = 16`。
pub const K_MAX_THREAD_NAME_LENGTH: usize = 16;

/// 线程选项；对应 V8 `Thread::Options`。
#[derive(Debug, Clone)]
pub struct Options {
  pub name: String,
  pub stack_size: i32,
  pub priority: Priority,
}

impl Default for Options {
  fn default() -> Self {
    Self {
      name: "v8:<unknown>".to_string(),
      stack_size: 0,
      priority: Priority::Default,
    }
  }
}

impl Options {
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      stack_size: 0,
      priority: Priority::Default,
    }
  }
  pub fn with_stack(mut self, stack_size: i32) -> Self {
    self.stack_size = stack_size;
    self
  }
  pub fn with_priority(mut self, priority: Priority) -> Self {
    self.priority = priority;
    self
  }
}

/// 与 V8 `Thread` 对应。
pub struct Thread {
  options: Options,
  closure: Option<Box<dyn FnOnce() + Send + 'static>>,
  join_handle: Option<JoinHandle<()>>,
}

impl Debug for Thread {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("Thread")
      .field("name", &self.options.name)
      .field("started", &self.join_handle.is_some())
      .finish()
  }
}

impl Thread {
  /// 构造线程但不启动。
  pub fn new(options: Options, closure: impl FnOnce() + Send + 'static) -> Self {
    Self {
      options,
      closure: Some(Box::new(closure)),
      join_handle: None,
    }
  }

  pub fn name(&self) -> &str {
    &self.options.name
  }

  pub fn priority(&self) -> Priority {
    self.options.priority
  }

  /// 对应 V8 `bool Start()`：非阻塞启动。
  pub fn start(&mut self) -> Result<()> {
    let closure = self.closure.take().ok_or(Error::ThreadStartFailed)?;
    let opts = self.options.clone();
    let mut b = Builder::new();
    // std 的 name 会写入 OS 线程名（macOS 通过 `_pthread_setname_np`，
    // Linux 通过 `prctl(PR_SET_NAME)`），语义与 V8 `set_name` 一致；
    // 名字超过 15 字节会被截断（与 V8 `kMaxThreadNameLength` 相同）。
    if !opts.name.is_empty() {
      b = b.name(opts.name.clone());
    }
    let stack = effective_stack_size(opts.stack_size);
    if stack > 0 {
      b = b.stack_size(stack);
    }
    let priority = opts.priority;
    let handle = b
      .spawn(move || {
        apply_priority(priority);
        closure();
      })
      .map_err(|_| Error::ThreadStartFailed)?;
    self.join_handle = Some(handle);
    Ok(())
  }

  /// 对应 V8 `bool StartSynchronously()`：启动并等待进入 `Run`。
  pub fn start_synchronously(&mut self) -> Result<()> {
    let closure = self.closure.take().ok_or(Error::ThreadStartFailed)?;
    let opts = self.options.clone();
    let sem = Arc::new(Semaphore::new(0));
    let sem_inner = sem.clone();
    let priority = opts.priority;
    let mut b = Builder::new();
    if !opts.name.is_empty() {
      b = b.name(opts.name.clone());
    }
    // 与 start 同源：钳 PTHREAD_STACK_MIN + darwin 默认 1MB（原版构造一次、两路生效）。
    let stack = effective_stack_size(opts.stack_size);
    if stack > 0 {
      b = b.stack_size(stack);
    }
    let handle = b
      .spawn(move || {
        apply_priority(priority);
        // 对应 `NotifyStartedAndDispatch()`。
        sem_inner.signal();
        closure();
      })
      .map_err(|_| Error::ThreadStartFailed)?;
    self.join_handle = Some(handle);
    sem.wait();
    Ok(())
  }

  /// 对应 V8 `void Join()`。等待线程结束。
  pub fn join(&mut self) -> Result<()> {
    let handle = self.join_handle.take().ok_or(Error::InvalidArgument)?;
    // std 提供 panic 传播；此处只映射 JoinError 为错误，绝不 re-panic。
    handle.join().map_err(|_| Error::JoinFailed)?;
    Ok(())
  }

  /// 已启动的句柄；用于测试或外部 join。
  pub fn is_started(&self) -> bool {
    self.join_handle.is_some()
  }

  /// 放弃等待（等价于 detach）。
  pub fn detach(&mut self) {
    self.join_handle.take();
  }
}

/// 计算传给 Builder 的栈字节数；返回 0 表示交给 std 默认。
/// 对应原 Thread 构造钳 `PTHREAD_STACK_MIN`（platform-posix.cc:1232-1233，
/// 对 Start / StartSynchronously 一并生效）与 darwin 默认 1MB
/// （platform-posix.cc:1325-1329，pthread 默认 512kB 偏小故 V8 提升）。
fn effective_stack_size(stack_size: i32) -> usize {
  // darwin: PTHREAD_STACK_MIN 16384(arm64)/8192(x86_64)（SDK limits.h:113-118）；
  // linux: 16384。取 16384 覆盖 arm64/linux 精确值；darwin x86_64 偏高（安全方向）。
  const PTHREAD_STACK_MIN: i32 = 16 * 1024;
  const DARWIN_DEFAULT_STACK: i32 = 1024 * 1024;
  let s = if stack_size > 0 {
    stack_size.max(PTHREAD_STACK_MIN)
  } else if cfg!(target_os = "macos") {
    DARWIN_DEFAULT_STACK
  } else {
    return 0;
  };
  s as usize
}

/// 在 darwin/linux 上设置当前线程调度类；对应 V8 `ThreadEntry` 的 priority 逻辑。
fn apply_priority(priority: Priority) {
  match priority {
    Priority::Default => {}
    #[cfg(target_os = "macos")]
    _ => {
      // `<pthread/qos.h>`（本机 SDK sys/qos.h 实测）：
      //   QOS_CLASS_USER_INTERACTIVE = 0x21, USER_INITIATED = 0x19,
      //   UTILITY = 0x11, BACKGROUND = 0x09。
      // V8 映射（platform-posix.cc:1278-1291）:
      //   kBestEffort   -> BACKGROUND, 0
      //   kUserVisible  -> USER_INITIATED, -1
      //   kUserBlocking -> USER_INITIATED, 0
      unsafe extern "C" {
        fn pthread_set_qos_class_self_np(qos_class: i32, relative_priority: i32) -> i32;
      }
      const QOS_CLASS_USER_INITIATED: i32 = 0x19;
      const QOS_CLASS_BACKGROUND: i32 = 0x09;
      let (cls, rp) = match priority {
        Priority::BestEffort => (QOS_CLASS_BACKGROUND, 0),
        Priority::UserVisible => (QOS_CLASS_USER_INITIATED, -1),
        Priority::UserBlocking => (QOS_CLASS_USER_INITIATED, 0),
        Priority::Default => return,
      };
      // SAFETY: 纯 POSIX 调用，仅影响当前线程。
      unsafe {
        pthread_set_qos_class_self_np(cls, rp);
      }
    }
    #[cfg(target_os = "linux")]
    _ => {
      // V8 kBestEffort=10, kUserVisible=1, kUserBlocking=0, kDefault=0。
      let nice: i32 = match priority {
        Priority::BestEffort => 10,
        Priority::UserVisible => 1,
        Priority::UserBlocking => 0,
        Priority::Default => return,
      };
      // SAFETY: `setpriority` 是 POSIX 系统调用；0 表示当前进程/线程。
      unsafe {
        libc::setpriority(libc::PRIO_PROCESS as _, 0, nice as _);
      }
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    _ => {
      // 其他平台无对应实现；仅保留枚举。
    }
  }
}

// ----------------------------------------------------------------------------
// 线程本地存储 (TLS)

/// 与原 V8 `Thread::LocalStorageKey` 对齐；unix 上等价 `pthread_key_t`，windows 上等价 DWORD (`u32`)。
#[cfg(unix)]
pub type LocalStorageKey = libc::pthread_key_t;
#[cfg(windows)]
pub type LocalStorageKey = u32;

#[cfg(windows)]
mod win_tls {
  use core::ffi::c_void;
  unsafe extern "system" {
    pub fn TlsAlloc() -> u32;
    pub fn TlsFree(dwTlsIndex: u32) -> i32;
    pub fn TlsGetValue(dwTlsIndex: u32) -> *mut c_void;
    pub fn TlsSetValue(dwTlsIndex: u32, lpTlsValue: *mut c_void) -> i32;
  }
}

/// 对应 V8 `CreateThreadLocalKey()`；返回 `Result<LocalStorageKey>`。
pub fn create_thread_local_key() -> Result<LocalStorageKey> {
  #[cfg(unix)]
  {
    let mut key: libc::pthread_key_t = 0;
    // SAFETY: `pthread_key_create` 是 POSIX 定义无副作用调用；destructor 传 None 与
    // V8 默认行为一致。key 为栈局部，返回后拷贝。
    let rc = unsafe { libc::pthread_key_create(&mut key, None) };
    if rc == 0 {
      Ok(key)
    } else {
      Err(Error::TlsKeyCreateFailed)
    }
  }
  #[cfg(windows)]
  {
    let key = unsafe { win_tls::TlsAlloc() };
    if key == 0xFFFF_FFFF {
      Err(Error::TlsKeyCreateFailed)
    } else {
      Ok(key)
    }
  }
}

/// 对应 V8 `DeleteThreadLocalKey(key)`。
pub fn delete_thread_local_key(key: LocalStorageKey) {
  // SAFETY: key 必须由 create_thread_local_key 分配且此前未删除。
  #[cfg(unix)]
  unsafe {
    libc::pthread_key_delete(key);
  }
  #[cfg(windows)]
  unsafe {
    win_tls::TlsFree(key);
  }
}

/// 对应 V8 `GetThreadLocal(key)`。
pub fn get_thread_local(key: LocalStorageKey) -> *mut c_void {
  // SAFETY: `pthread_getspecific` 无副作用；key 生命周期由调用方保证。
  #[cfg(unix)]
  unsafe {
    libc::pthread_getspecific(key)
  }
  #[cfg(windows)]
  unsafe {
    win_tls::TlsGetValue(key)
  }
}

/// 对应 V8 `SetThreadLocal(key, value)`。
///
/// # Safety
/// `key` 必须由 `create_thread_local_key` 分配且未删除；`value` 生命周期由调用方管理。
pub unsafe fn set_thread_local(key: LocalStorageKey, value: *mut c_void) {
  // SAFETY: `pthread_setspecific`；由调用方保证 key 有效、value 生命周期。
  #[cfg(unix)]
  unsafe {
    libc::pthread_setspecific(key, value);
  }
  #[cfg(windows)]
  unsafe {
    win_tls::TlsSetValue(key, value);
  }
}

/// 对应 V8 `HasThreadLocal(key)`。
pub fn has_thread_local(key: LocalStorageKey) -> bool {
  !get_thread_local(key).is_null()
}

/// 与 `GetExistingThreadLocal` 语义一致；本实现与 `get_thread_local` 相同。
pub fn get_existing_thread_local(key: LocalStorageKey) -> *mut c_void {
  get_thread_local(key)
}
