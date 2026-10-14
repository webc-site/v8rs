//! 1:1 对位 `v8/src/logging/metrics.h/.cc`(签名骨架, 7.1 阶段一; 推迟登记见下)。
//!
//! **校正登记**: 任务简报所指 `MetricsReader`(macOS os_log / Linux tracepoint)在现存
//! 源码树**不存在**(该设施早已自 V8 移除, 全树 grep 无此符号); 现存 `metrics.h/.cc`
//! 实为 embedder 指标转发器 `Recorder`(metrics.h:22-84)。本模块按真实原版建骨架,
//! 不虚构 MetricsReader。
//!
//! 【推迟登记】(依赖 L7 API/init, 待 v8-api / v8-init / v8-tasks 回收):
//! - `SetEmbedderRecorder(Isolate*, ...)` 中 `V8::GetCurrentPlatform()
//!   ->GetForegroundTaskRunner(isolate)`(metrics.cc:36-41)与
//!   `PostDelayedTask(Task, 1.0)`(metrics.cc:56-58), 本层仅保留可移植的状态语义;
//! - 泛型事件模板 `AddMainThreadEvent<T>` / `DelayMainThreadEvent<T>` /
//!   `AddThreadSafeEvent<T>` 的 `T` = `include/v8-metrics.h` 事件类族,
//!   `v8::metrics::Recorder::ContextId` 亦为该头文件类型, 此处仅做不透明占位;
//! - `DelayedEvent<T>::Run` 闭环(`enable_shared_from_this` 回收时以
//!   `Weak<Recorder>` 表达)。

use std::{collections::VecDeque, mem::take, sync::Arc};

use parking_lot::Mutex;

/// 对应 `v8::metrics::Recorder::ContextId`(`include/v8-metrics.h`, L7 API)。
/// 不透明占位: 回收时按其真实构造(isolate/计数)扩展。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContextId {
  /// 占位标识(原版构造参数在 API 头, 本层不解释)。
  pub opaque: u64,
}

/// embedder 侧记录器的最小可对象安全边界。
///
/// 对应 `v8::metrics::Recorder`(include/v8-metrics.h)中本层可引用的非模板成员:
/// `NotifyIsolateDisposal()`。泛型 `AddMainThreadEvent<T>` 族**不进本 trait**
/// (泛型方法破坏 dyn 对象安全), 待 v8-api 以具体事件类型表面对接。
pub trait MetricsRecorder: Send + Sync {
  /// 对应 `v8::metrics::Recorder::NotifyIsolateDisposal()`。
  fn notify_isolate_disposal(&self);
}

/// 对应 `Recorder::DelayedEventBase`(metrics.h:53-58)。
pub trait DelayedEventBase: Send {
  /// 对应 `DelayedEventBase::Run(const std::shared_ptr<Recorder>&)`。
  fn run(self: Box<Self>, recorder: &Recorder);
}

/// 对应 `class Recorder`(metrics.h:22)。
///
/// 线程模型 1:1: `base::Mutex lock_` → [`parking_lot::Mutex`], 守卫
/// `embedder_recorder_` 与 `delayed_events_` 两成员(metrics.cc:21-22, 51-58)。
pub struct Recorder {
  state: Mutex<RecorderState>,
}

struct RecorderState {
  /// 对应 `embedder_recorder_`(std::shared_ptr<v8::metrics::Recorder>)。
  embedder_recorder: Option<Arc<dyn MetricsRecorder>>,
  /// 对应 `delayed_events_`(std::queue<std::unique_ptr<DelayedEventBase>>)。
  delayed_events: VecDeque<Box<dyn DelayedEventBase>>,
}

impl Recorder {
  /// 对应 `Recorder` 构造(C++ 成员零初始化)。
  /// C++ 使用点为 `shared_ptr<Recorder>`(enable_shared_from_this),
  /// Rust 侧由调用方 `Arc::new(recorder)` 包裹。
  pub fn new() -> Self {
    Self {
      state: Mutex::new(RecorderState {
        embedder_recorder: None,
        delayed_events: VecDeque::new(),
      }),
    }
  }

  /// 对应 `Recorder::HasEmbedderRecorder()`(metrics.cc:43)。
  pub fn has_embedder_recorder(&self) -> bool {
    self.state.lock().embedder_recorder.is_some()
  }

  /// 对应 `Recorder::SetEmbedderRecorder`(metrics.cc:34-41)的可移植子集:
  /// 原版 `CHECK_NULL(embedder_recorder_)` 禁止重复设置——此处以返回值表达
  /// (禁 panic), 返回 false 表示已设置被拒; `Isolate`/foreground task runner
  /// 接驳推迟至 v8-init 回收。
  pub fn set_embedder_recorder(&self, recorder: Arc<dyn MetricsRecorder>) -> bool {
    let mut state = self.state.lock();
    if state.embedder_recorder.is_some() {
      // CHECK_NULL(embedder_recorder_) 语义: 已设置则拒绝。
      return false;
    }
    state.embedder_recorder = Some(recorder);
    true
  }

  /// 对应 `Recorder::NotifyIsolateDisposal()`(metrics.cc:45-49)。
  pub fn notify_isolate_disposal(&self) {
    // 锁外回调, 避免嵌入方反调 Recorder 发生重入死锁 (对齐 metrics.cc:45-49)
    let recorder = self.state.lock().embedder_recorder.clone();
    if let Some(recorder) = recorder {
      recorder.notify_isolate_disposal();
    }
  }

  /// 对应 `Recorder::Delay`(metrics.cc:51-59)的队列入队部分;
  /// `was_empty` 时原版向 task runner 投递 `Task`(1.0s 延迟)——L7 推迟,
  /// 此处返回 `was_empty` 供未来接驳方判定。
  pub fn delay(&self, event: Box<dyn DelayedEventBase>) -> bool {
    let mut state = self.state.lock();
    let was_empty = state.delayed_events.is_empty();
    state.delayed_events.push_back(event);
    was_empty
  }

  /// 对应 `Recorder::Task::Run`(metrics.cc:18-28): 锁内 swap 队列, 锁外逐条执行。
  /// 队列 swap → Rust `std::mem::take`; `Run(recorder_)` 借用本 `Recorder`。
  pub fn drain_delayed_events(&self) {
    let drained = {
      let mut state = self.state.lock();
      take(&mut state.delayed_events)
    };
    // while (!delayed_events.empty()) { front()->Run(recorder_); pop; }
    for event in drained {
      event.run(self);
    }
  }
}

impl Default for Recorder {
  fn default() -> Self {
    // 直接构造形态(C++ 就地 new Recorder 亦存在); shared_from_this 场景用 `new()`。
    Self {
      state: Mutex::new(RecorderState {
        embedder_recorder: None,
        delayed_events: VecDeque::new(),
      }),
    }
  }
}
