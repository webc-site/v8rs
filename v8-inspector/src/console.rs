// 由 tool/src/inspector/console.js 从 v8/src/inspector/v8-console.h, src/inspector/v8-console.cc, src/inspector/v8-console-agent-impl.h, src/inspector/v8-console-agent-impl.cc, src/inspector/v8-console-message.h, src/inspector/v8-console-message.cc, src/inspector/v8-stack-trace-impl.h, src/inspector/v8-stack-trace-impl.cc 生成; 修订就地改, 复跑覆盖
// Console 域: V8MessageOrigin/ConsoleAPIType 及 V8ConsoleMessage/StackFrame/Storage

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// 控制台消息来源 (v8/src/inspector/v8-console-message.h:28)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, Serialize, Deserialize)]
pub enum V8MessageOrigin {
  /// 普通控制台 API 输出
  Console,
  /// 未捕获异常
  Exception,
  /// 异常被撤销/忽略
  RevokedException,
}

impl V8MessageOrigin {
  /// 是否来自控制台
  #[inline]
  pub const fn is_console(&self) -> bool {
    matches!(self, Self::Console)
  }

  /// 是否为异常
  #[inline]
  pub const fn is_exception(&self) -> bool {
    matches!(self, Self::Exception | Self::RevokedException)
  }
}

/// Console API 方法类型 (v8/src/inspector/v8-console-message.h:30, 16 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, Serialize, Deserialize)]
pub enum ConsoleAPIType {
  /// Log
  Log,
  /// Debug
  Debug,
  /// Info
  Info,
  /// Error
  Error,
  /// Warning
  Warning,
  /// Dir
  Dir,
  /// DirXML
  DirXML,
  /// Table
  Table,
  /// Trace
  Trace,
  /// StartGroup
  StartGroup,
  /// StartGroupCollapsed
  StartGroupCollapsed,
  /// EndGroup
  EndGroup,
  /// Clear
  Clear,
  /// Assert
  Assert,
  /// TimeEnd
  TimeEnd,
  /// Count
  Count,
}

impl ConsoleAPIType {
  /// 是否为错误或警告
  #[inline]
  pub const fn is_error_or_warning(&self) -> bool {
    matches!(self, Self::Error | Self::Warning | Self::Assert)
  }

  /// 是否为分组控制
  #[inline]
  pub const fn is_group(&self) -> bool {
    matches!(
      self,
      Self::StartGroup | Self::StartGroupCollapsed | Self::EndGroup
    )
  }

  /// 是否为计时相关
  #[inline]
  pub const fn is_time(&self) -> bool {
    matches!(self, Self::TimeEnd)
  }
}

/// 默认捕获的最大调用栈深度 (v8/src/inspector/v8-stack-trace-impl.h:54)
pub const K_DEFAULT_MAX_CALL_STACK_SIZE_TO_CAPTURE: i32 = 200;

/// 堆栈帧模型 (v8/src/inspector/v8-stack-trace-impl.h:17)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StackFrame {
  pub function_name: String,
  pub script_id: i32,
  pub url: String,
  pub line_number: i32,
  pub column_number: i32,
}

impl StackFrame {
  /// 创建新堆栈帧
  pub fn new(
    function_name: impl Into<String>,
    script_id: i32,
    url: impl Into<String>,
    line_number: i32,
    column_number: i32,
  ) -> Self {
    Self {
      function_name: function_name.into(),
      script_id,
      url: url.into(),
      line_number,
      column_number,
    }
  }
}

/// 调用栈实现模型 (v8/src/inspector/v8-stack-trace-impl.h:32)
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct V8StackTraceImpl {
  pub description: Option<String>,
  pub frames: Vec<StackFrame>,
}

impl V8StackTraceImpl {
  /// 创建新调用栈
  pub fn new(description: Option<String>, frames: Vec<StackFrame>) -> Self {
    Self {
      description,
      frames,
    }
  }

  /// 顶层栈帧
  pub fn top_frame(&self) -> Option<&StackFrame> {
    self.frames.first()
  }

  /// 栈帧数
  #[inline]
  pub fn frame_count(&self) -> usize {
    self.frames.len()
  }
}

/// 控制台消息主体 (v8/src/inspector/v8-console-message.h:49)
#[derive(Debug, Clone, PartialEq)]
pub struct V8ConsoleMessage {
  pub origin: V8MessageOrigin,
  pub timestamp: f64,
  pub message: String,
  pub url: String,
  pub line_number: u32,
  pub column_number: u32,
  pub context_id: i32,
  pub message_type: ConsoleAPIType,
  pub stack_trace: Option<V8StackTraceImpl>,
  pub exception_id: u32,
}

/// 异常位置元数据
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct V8ExceptionLocation {
  pub url: String,
  pub line_number: u32,
  pub column_number: u32,
}

impl V8ExceptionLocation {
  /// 构造异常位置
  pub fn new(url: impl Into<String>, line_number: u32, column_number: u32) -> Self {
    Self {
      url: url.into(),
      line_number,
      column_number,
    }
  }
}

impl V8ConsoleMessage {
  /// 为 Console API 创建
  pub fn create_for_console_api(
    context_id: i32,
    timestamp: f64,
    message_type: ConsoleAPIType,
    message: impl Into<String>,
    stack_trace: Option<V8StackTraceImpl>,
  ) -> Self {
    Self {
      origin: V8MessageOrigin::Console,
      timestamp,
      message: message.into(),
      url: String::new(),
      line_number: 0,
      column_number: 0,
      context_id,
      message_type,
      stack_trace,
      exception_id: 0,
    }
  }

  /// 为未捕获异常创建
  pub fn create_for_exception(
    timestamp: f64,
    location: V8ExceptionLocation,
    message: impl Into<String>,
    context_id: i32,
    exception_id: u32,
    stack_trace: Option<V8StackTraceImpl>,
  ) -> Self {
    Self {
      origin: V8MessageOrigin::Exception,
      timestamp,
      message: message.into(),
      url: location.url,
      line_number: location.line_number,
      column_number: location.column_number,
      context_id,
      message_type: ConsoleAPIType::Error,
      stack_trace,
      exception_id,
    }
  }

  /// 为撤销异常创建
  pub fn create_for_revoked_exception(
    timestamp: f64,
    message: impl Into<String>,
    revoked_exception_id: u32,
  ) -> Self {
    Self {
      origin: V8MessageOrigin::RevokedException,
      timestamp,
      message: message.into(),
      url: String::new(),
      line_number: 0,
      column_number: 0,
      context_id: 0,
      message_type: ConsoleAPIType::Error,
      stack_trace: None,
      exception_id: revoked_exception_id,
    }
  }
}

/// 控制台消息与计时/计数存储仓 (v8/src/inspector/v8-console-message.h:114)
#[derive(Debug, Default)]
pub struct V8ConsoleMessageStorage {
  messages: Vec<V8ConsoleMessage>,
  counters: HashMap<(i32, String), i32>,
  timers: HashMap<(i32, String), f64>,
}

impl V8ConsoleMessageStorage {
  /// 创建新存储仓
  pub fn new() -> Self {
    Self {
      messages: Vec::new(),
      counters: HashMap::new(),
      timers: HashMap::new(),
    }
  }

  /// 添加消息
  pub fn add_message(&mut self, msg: V8ConsoleMessage) {
    self.messages.push(msg);
  }

  /// 全部消息列表
  #[inline]
  pub fn messages(&self) -> &[V8ConsoleMessage] {
    &self.messages
  }

  /// 消息计数
  #[inline]
  pub fn message_count(&self) -> usize {
    self.messages.len()
  }

  /// console.count 计数器增加并返回值
  pub fn count(&mut self, context_id: i32, label: &str) -> i32 {
    let key = (context_id, label.to_string());
    let entry = self.counters.entry(key).or_insert(0);
    *entry = entry.saturating_add(1);
    *entry
  }

  /// console.countReset 重置计数器
  pub fn count_reset(&mut self, context_id: i32, label: &str) -> bool {
    self
      .counters
      .remove(&(context_id, label.to_string()))
      .is_some()
  }

  /// console.time 开始计时
  pub fn time(&mut self, context_id: i32, label: &str, timestamp: f64) -> bool {
    let key = (context_id, label.to_string());
    if self.timers.contains_key(&key) {
      return false;
    }
    self.timers.insert(key, timestamp);
    true
  }

  /// console.timeEnd 结束计时返回耗时 (毫秒)
  pub fn time_end(&mut self, context_id: i32, label: &str, end_timestamp: f64) -> Option<f64> {
    let start = self.timers.remove(&(context_id, label.to_string()))?;
    Some((end_timestamp - start).max(0.0))
  }

  /// 清除所有存储消息与状态
  pub fn clear(&mut self) {
    self.messages.clear();
    self.counters.clear();
    self.timers.clear();
  }
}
