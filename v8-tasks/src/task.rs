//! 对应 `v8/include/v8-platform.h` 中 Task / IdleTask / TaskPriority 接口，
//! 是 `src/tasks` cancelable 家族的语义来源（纯抽象接口，无实现）。

/// 对应原 `enum class v8::TaskPriority`（虚函数表外的纯值类型）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum TaskPriority {
  /// 尽力而为的后台任务。
  BestEffort = 0,
  /// 完成后改善性能/内存的长时后台任务（如后台编译、GC）。
  UserVisible = 1,
  /// 最高优先级、阻塞执行线程的任务（如 major GC）。
  UserBlocking = 2,
}

/// 最高优先级，对应原 `kMaxPriority = kUserBlocking`。
pub const MAX_PRIORITY: TaskPriority = TaskPriority::UserBlocking;

/// 对应原 `class v8::Task`：一个工作单元。
pub trait Task {
  fn run(&mut self);
}

/// 对应原 `class v8::IdleTask`：空闲期任务，参数为 MonotonicallyIncreasingTime
/// 口径的截止时间（秒）。
pub trait IdleTask {
  fn run(&mut self, deadline_in_seconds: f64);
}
