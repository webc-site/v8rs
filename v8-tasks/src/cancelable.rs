//! 对应 `src/tasks/cancelable-task.{h,cc}`：CancelableTaskManager / Cancelable /
//! CancelableTask / CancelableIdleTask。
//!
//! 所有权模型与原版差异：原版管理器持 `Cancelable*` 裸指针；此处用
//! `Arc<CancelInner>` + 管理器侧 `Weak<CancelInner>` 表达同一协议，析构摘除逻辑
//! 逐条对齐 `~Cancelable`。原版 `Register(Cancelable*)` 折叠进 `Cancelable::new`。

use std::{
  cell::UnsafeCell,
  fmt::{self, Debug, Formatter},
  sync::{
    Arc, Weak,
    atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering},
  },
};

use gxhash::{HashMap, HashMapExt};
use strum::FromRepr;
use v8_base_platform::{ConditionVariable, Mutex};

use crate::{
  error::{Error, Result},
  task::{IdleTask, Task},
};

/// 对应原 `CancelableTaskManager::Id`。
pub type TaskId = u64;

/// 对应原 `CancelableTaskManager::kInvalidTaskId`。
pub const INVALID_TASK_ID: TaskId = 0;

/// 对应原匿名 `enum Status { kWaiting, kCanceled, kRunning }`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr)]
#[repr(u8)]
pub enum Status {
  /// 已调度待执行，`try_run` 会成功。
  Waiting = 0,
  /// 已取消，`try_run` 会失败。
  Canceled = 1,
  /// 正在执行，不可再取消。
  Running = 2,
}

/// 对应原 `enum class TryAbortResult`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryAbortResult {
  /// 任务已执行完毕或此前已取消（已从管理器移除）。
  TaskRemoved,
  /// 任务正在执行，无法取消。
  TaskRunning,
  /// 任务尚未执行，已成功取消并移除。
  TaskAborted,
}

/// 取消状态机内核，对应原版 `Cancelable` 内的原子状态 + id。
struct CancelInner {
  id: TaskId,
  status: AtomicU8,
  manager: Weak<ManagerShared>,
}

impl CancelInner {
  fn new(id: TaskId, manager: &Arc<ManagerShared>) -> Self {
    Self {
      id,
      status: AtomicU8::new(Status::Waiting as u8),
      manager: Arc::downgrade(manager),
    }
  }

  /// 对应原 `CompareExchangeStatus`：仅失败时 `expected` 被就地更新为实际值，
  /// 成功时保持传入值不变（`compare_exchange_strong` 语义）。
  fn compare_exchange_status(
    &self,
    mut expected: Status,
    desired: Status,
    previous: Option<&mut Status>,
  ) -> bool {
    let ok = match self.status.compare_exchange(
      expected as u8,
      desired as u8,
      Ordering::AcqRel,
      Ordering::Acquire,
    ) {
      Ok(_) => true,
      Err(actual) => {
        // CAS 失败时取回实际值；该值只可能来自本枚举。
        expected = Status::from_repr(actual).unwrap_or(expected);
        false
      }
    };
    if let Some(p) = previous {
      *p = expected;
    }
    ok
  }

  fn try_run(&self, previous: Option<&mut Status>) -> bool {
    self.compare_exchange_status(Status::Waiting, Status::Running, previous)
  }

  fn cancel(&self) -> bool {
    self.compare_exchange_status(Status::Waiting, Status::Canceled, None)
  }
}

/// 管理器共享状态，对应原版 `CancelableTaskManager` 的私有成员组。
struct ManagerShared {
  mutex: Mutex,
  cancelable_tasks_barrier: ConditionVariable,
  canceled: AtomicBool,
  task_id_counter: AtomicU64,
  /// 仅在持有 `mutex` 时访问。
  tasks: UnsafeCell<HashMap<TaskId, Weak<CancelInner>>>,
}

// SAFETY: `tasks` 只在持有 `mutex` 时读写；其余字段为原子量或线程安全原语。
unsafe impl Sync for ManagerShared {}
// SAFETY: 同 `Sync` 论证。
unsafe impl Send for ManagerShared {}

impl ManagerShared {
  /// 对应原 `RemoveFinishedTask`：仅 `Cancelable` 析构路径调用。
  fn remove_finished_task(&self, id: TaskId) {
    debug_assert_ne!(id, INVALID_TASK_ID);
    self.mutex.lock();
    // SAFETY: 持有 `mutex` 期间独占 `tasks`。
    let removed = unsafe { &mut *self.tasks.get() }.remove(&id);
    debug_assert!(removed.is_some());
    self.cancelable_tasks_barrier.notify_one();
    // SAFETY: 本函数持有 `mutex`，此处配平释放。
    unsafe {
      self.mutex.unlock();
    }
  }
}

/// 跟踪可取消任务，对应原 `class CancelableTaskManager`。
/// 任意前/后台线程均可注册与摘除。
pub struct CancelableTaskManager {
  shared: Arc<ManagerShared>,
}

impl Default for CancelableTaskManager {
  fn default() -> Self {
    Self::new()
  }
}

impl Drop for CancelableTaskManager {
  fn drop(&mut self) {
    // 原版为 CHECK(canceled_)：产品路径禁 panic，降级为 debug_assert，偏差登记 readme。
    debug_assert!(self.shared.canceled.load(Ordering::Relaxed));
  }
}

impl CancelableTaskManager {
  pub fn new() -> Self {
    Self {
      shared: Arc::new(ManagerShared {
        mutex: Mutex::new(),
        cancelable_tasks_barrier: ConditionVariable::new(),
        canceled: AtomicBool::new(false),
        task_id_counter: AtomicU64::new(INVALID_TASK_ID),
        // SAFETY: 初始空表，无并发访问。
        tasks: UnsafeCell::new(HashMap::new()),
      }),
    }
  }

  /// 对应原 `canceled()`。
  pub fn canceled(&self) -> bool {
    self.shared.canceled.load(Ordering::Relaxed)
  }

  /// 对应原 `TryAbort(Id)`：id 无效时返回错误（原版 `CHECK_NE`）。
  pub fn try_abort(&self, id: TaskId) -> Result<TryAbortResult> {
    if id == INVALID_TASK_ID {
      return Err(Error::InvalidTaskId);
    }
    let shared = &self.shared;
    shared.mutex.lock();
    let outcome = {
      // SAFETY: 持有 `mutex`，独占访问 `tasks`。
      let tasks = unsafe { &mut *shared.tasks.get() };
      match tasks.get(&id).and_then(Weak::upgrade) {
        Some(inner) => {
          if inner.cancel() {
            // 不能在此调用 RemoveFinishedTask（递归加锁），原位删除并通知。
            tasks.remove(&id);
            shared.cancelable_tasks_barrier.notify_one();
            TryAbortResult::TaskAborted
          } else {
            TryAbortResult::TaskRunning
          }
        }
        // 已登记但内核销毁属协议外状态（见 readme），按已移除处理。
        None => TryAbortResult::TaskRemoved,
      }
    };
    // SAFETY: 本函数持有 `mutex`。
    unsafe {
      shared.mutex.unlock();
    }
    Ok(outcome)
  }

  /// 对应原 `TryAbortAll()`：返回是否仍有无法取消的任务。
  pub fn try_abort_all(&self) -> TryAbortResult {
    let shared = &self.shared;
    shared.mutex.lock();
    let outcome = {
      // SAFETY: 持有 `mutex`。
      let tasks = unsafe { &mut *shared.tasks.get() };
      if tasks.is_empty() {
        TryAbortResult::TaskRemoved
      } else {
        tasks.retain(|_, inner| match inner.upgrade() {
          Some(inner) => !inner.cancel(),
          None => false,
        });
        if tasks.is_empty() {
          TryAbortResult::TaskAborted
        } else {
          TryAbortResult::TaskRunning
        }
      }
    };
    // SAFETY: 本函数持有 `mutex`。
    unsafe {
      shared.mutex.unlock();
    }
    outcome
  }

  /// 对应原 `CancelAndWait()`：取消全部可取消任务并等待执行中任务收尾，
  /// 之后禁止再注册。
  pub fn cancel_and_wait(&self) {
    let shared = &self.shared;
    shared.mutex.lock();
    shared.canceled.store(true, Ordering::Release);

    // 任务可能正在执行或并发注册，需要循环；每轮取消后等待已启动任务收尾。
    loop {
      // SAFETY: 持有 `mutex`。
      let tasks = unsafe { &mut *shared.tasks.get() };
      if tasks.is_empty() {
        break;
      }
      tasks.retain(|_, inner| match inner.upgrade() {
        Some(inner) => !inner.cancel(),
        None => false,
      });
      // 复用循环开头的 `tasks` 独占引用，避免二次裸解引用。
      let empty = tasks.is_empty();
      if !empty {
        // 对应原 `Wait`：原子释放 mutex 挂起，唤醒后重新持有（容忍 spurious wakeup，
        // 唤醒条件由下一轮循环复查）。
        shared.cancelable_tasks_barrier.wait(&shared.mutex);
      }
    }

    // SAFETY: 本函数持有 `mutex`。
    unsafe {
      shared.mutex.unlock();
    }
  }

  /// 对应原 `Register(Cancelable*)`：内部路径，由 `Cancelable::new` 调用。
  fn register(shared: &Arc<ManagerShared>) -> Result<Arc<CancelInner>> {
    // SAFETY: 调用方 `Cancelable::new` 持有 `shared.mutex`。
    if shared.canceled.load(Ordering::Relaxed) {
      // 管理器已取消：新任务立即标记取消以免执行，返回无效 id（原版行为）。
      let inner = Arc::new(CancelInner::new(INVALID_TASK_ID, shared));
      inner.cancel();
      return Ok(inner);
    }
    let current = shared.task_id_counter.load(Ordering::Relaxed);
    // 对应原 `++task_id_counter_; CHECK_NE(kInvalidTaskId, id)`：
    // 到达 u64::MAX 时直接返回溢出错误，计数器钉死在 u64::MAX（粘性错误），
    // 避免 debug 模式加 1 算术溢出 panic，同时彻底杜绝 id 复用（防 ABA）。
    if current == u64::MAX {
      return Err(Error::TaskIdOverflow);
    }
    let id = current + 1;
    shared.task_id_counter.store(id, Ordering::Relaxed);
    let inner = Arc::new(CancelInner::new(id, shared));
    // SAFETY: 调用方持有 `mutex`。
    let inserted = unsafe { &mut *shared.tasks.get() }
      .insert(id, Arc::downgrade(&inner))
      .is_none();
    debug_assert!(inserted);
    Ok(inner)
  }

  #[doc(hidden)]
  pub fn set_task_id_counter_for_testing(&self, val: u64) {
    self.shared.task_id_counter.store(val, Ordering::Relaxed);
  }
}

/// 可取消句柄，对应原 `class Cancelable`。持有任务状态机内核的共享引用。
pub struct Cancelable {
  inner: Arc<CancelInner>,
}

impl Cancelable {
  /// 对应原构造 `Cancelable(CancelableTaskManager*)`（内部即 Register）。
  pub fn new(manager: &CancelableTaskManager) -> Result<Self> {
    let shared = &manager.shared;
    shared.mutex.lock();
    let r = CancelableTaskManager::register(shared);
    // SAFETY: 本函数持有 `mutex`。
    unsafe {
      shared.mutex.unlock();
    }
    r.map(|inner| Self { inner })
  }

  /// 对应原 `id()`。注意：任务移交平台后不可再调用（原版注释）。
  pub fn id(&self) -> TaskId {
    self.inner.id
  }

  /// 对应原 protected `TryRun()`。
  pub fn try_run(&self) -> bool {
    self.inner.try_run(None)
  }

  /// 对应原 `TryRun(Status* previous)`。
  pub fn try_run_prev(&self, previous: &mut Status) -> bool {
    self.inner.try_run(Some(previous))
  }

  /// 对应原 protected `CompareExchangeStatus`（测试状态机用）。
  pub fn compare_exchange_status(&self, expected: &mut Status, desired: Status) -> bool {
    self
      .inner
      .compare_exchange_status(*expected, desired, Some(expected))
  }
}

impl Drop for Cancelable {
  fn drop(&mut self) {
    // 对应原 `~Cancelable`：先 CAS 探测状态，避免触碰已终止的管理器；
    // 曾成功执行（Running）或从未执行（TryRun 成功）才需要摘除登记。
    let mut previous = Status::Waiting;
    if !self.inner.try_run(Some(&mut previous)) && previous != Status::Running {
      return;
    }
    if let Some(shared) = self.inner.manager.upgrade() {
      shared.remove_finished_task(self.inner.id);
    }
  }
}

/// 对应原版纯虚 `RunInternal()`：用户任务体。
pub trait TaskBody {
  fn run_internal(&mut self);
}

/// 对应原版纯虚 `RunInternal(double)`。
pub trait IdleTaskBody {
  fn run_internal(&mut self, deadline_in_seconds: f64);
}

/// 对应原 `class CancelableTask : Cancelable, Task`。
pub struct CancelableTask {
  cancelable: Cancelable,
  body: Box<dyn TaskBody + Send>,
}

impl CancelableTask {
  pub fn new(
    manager: &CancelableTaskManager,
    body: impl TaskBody + Send + 'static,
  ) -> Result<Self> {
    Ok(Self {
      cancelable: Cancelable::new(manager)?,
      body: Box::new(body),
    })
  }

  pub fn id(&self) -> TaskId {
    self.cancelable.id()
  }
}

impl Task for CancelableTask {
  /// 对应原 `void Run() final`。
  fn run(&mut self) {
    if self.cancelable.try_run() {
      self.body.run_internal();
    }
  }
}

/// 对应原 `class CancelableIdleTask : Cancelable, IdleTask`。
pub struct CancelableIdleTask {
  cancelable: Cancelable,
  body: Box<dyn IdleTaskBody + Send>,
}

impl CancelableIdleTask {
  pub fn new(
    manager: &CancelableTaskManager,
    body: impl IdleTaskBody + Send + 'static,
  ) -> Result<Self> {
    Ok(Self {
      cancelable: Cancelable::new(manager)?,
      body: Box::new(body),
    })
  }

  pub fn id(&self) -> TaskId {
    self.cancelable.id()
  }
}

impl IdleTask for CancelableIdleTask {
  /// 对应原 `void Run(double) final`。
  fn run(&mut self, deadline_in_seconds: f64) {
    if self.cancelable.try_run() {
      self.body.run_internal(deadline_in_seconds);
    }
  }
}

impl Debug for CancelableTaskManager {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("CancelableTaskManager")
      .field("canceled", &self.canceled())
      .finish()
  }
}
