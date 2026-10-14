// 由 tool/src/tasks/cancelable-task.h.js 从 v8/src/tasks/cancelable-task.{h,cc} 生成; 修订就地改, 复跑覆盖
// 可取消任务体系: 枚举/常量/签名骨架

/// 尝试中止任务的结局 (对应原版 enum class TryAbortResult)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TryAbortResult {
  /// `kTaskRemoved`
  Taskremoved = 0,
  /// `kTaskRunning`
  Taskrunning = 1,
  /// `kTaskAborted`
  Taskaborted = 2,
}

impl TryAbortResult {
  /// 总数对数守卫
  pub const COUNT: usize = 3;
}

/// 可取消任务管理器: 任务 id 类型 (原版 using Id = uint64_t)
pub type TaskId = u64;

/// 非法任务 id 哨兵 (原版 kInvalidTaskId = 0)
pub const INVALID_TASK_ID: TaskId = 0;

/// Cancelable 任务状态机 (对应原版 enum Status, 保护成员)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Status {
  /// `kWaiting`
  Waiting = 0,
  /// `kCanceled`
  Canceled = 1,
  /// `kRunning`
  Running = 2,
}

/// CancelableTaskManager 方法签名骨架 (语义实现见 crate 内手写 cancelable.rs)
pub trait CancelableTaskManagerLike {
  /// 注册新可取消任务, 返回唯一 id
  fn register(&mut self, task_key: usize) -> TaskId;
  /// 按 id 尝试中止
  fn try_abort(&mut self, id: TaskId) -> TryAbortResult;
  /// 尝试取消全部已注册任务
  fn try_abort_all(&mut self) -> TryAbortResult;
  /// 取消全部并等待运行中的任务; 之后禁再注册
  fn cancel_and_wait(&mut self);
  /// 是否已取消
  fn canceled(&self) -> bool;
  /// 仅由 Cancelable 析构调用: 摘除已完成任务
  fn remove_finished_task(&mut self, id: TaskId);
}

/// Cancelable 方法签名骨架
pub trait CancelableLike {
  /// 任务 id (交给平台后禁再调)
  fn id(&self) -> TaskId;
  /// 状态从 Waiting → Running, 成功即获得执行权
  fn try_run(&self) -> bool;
  /// 状态从 Waiting → Canceled (仅管理器可调)
  fn cancel(&self) -> bool;
}

// cc 侧实现定义头对数清单 (原文签名, 供 Developer 修订对照):
//   CancelableTaskManager::Id CancelableTaskManager::Register(Cancelable* task)
//   void CancelableTaskManager::RemoveFinishedTask(CancelableTaskManager::Id id)
//   TryAbortResult CancelableTaskManager::TryAbort(CancelableTaskManager::Id id)
//   void CancelableTaskManager::CancelAndWait()
//   TryAbortResult CancelableTaskManager::TryAbortAll()
