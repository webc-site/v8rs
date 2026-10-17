//! 1:1 转写自 `v8/src/logging/local-logger.h/.cc`。
//!
//! 真实语义校正(原版实测, local-logger.h:15-36):
//! `LocalLogger` **不是**字符串累积缓冲/`HasLogMessage` 判定器, 而是**主线程快路径代理**:
//! 构造时从 `isolate->v8_file_logger()` 缓存两个谓词(local-logger.cc:14-18),
//! 其后把 `ScriptDetails`/`ScriptEvent`/`CodeLinePosInfoRecordEvent`/`MapCreate`/
//! `MapDetails` 逐条转发给 `V8FileLogger`。
//! `V8FileLogger*` 依赖 `Isolate`(L3), 故此处缓存谓词以参数注入;
//! 纯格式化的 `ScriptEvent` 转发到本 crate 已落地的 [`crate::log::script_event`]
//! (log_file/is_logging 等由调用方持有), 其余依赖堆对象的转发【推迟登记】:
//! - `ScriptDetails(Tagged<Script>)`、`CodeLinePosInfoRecordEvent(...)`、
//!   `MapCreate/MapDetails(Tagged<Map>)` — 待 v8-objects/v8-execution 回收。

use crate::{
  log::{ScriptEventType, script_event},
  log_file::LogFile,
};

/// 对应 `class LocalLogger`(local-logger.h:15)。
///
/// 缓存谓词与 C++ 成员 `is_logging_`/`is_listening_to_code_events_` 一一对应
/// (local-logger.cc:16-18 构造时快照 `V8FileLogger` 状态)。
#[derive(Debug, Clone, Copy)]
pub struct LocalLogger {
  /// 对应 `is_logging_`。
  is_logging: bool,
  /// 对应 `is_listening_to_code_events_`。
  is_listening_to_code_events: bool,
}

impl LocalLogger {
  /// 对应 `LocalLogger::LocalLogger(Isolate*)`(local-logger.cc:14-18)。
  /// 原版读取 `isolate->v8_file_logger()->is_logging()` 与
  /// `is_listening_to_code_events()`; Isolate 层就绪前由调用方传入等价快照。
  #[inline]
  pub fn new(is_logging: bool, is_listening_to_code_events: bool) -> Self {
    Self {
      is_logging,
      is_listening_to_code_events,
    }
  }

  /// 对应 `LocalLogger::is_logging()`(local-logger.h:19)。
  #[inline]
  pub fn is_logging(&self) -> bool {
    self.is_logging
  }

  /// 对应 `LocalLogger::is_listening_to_code_events()`(local-logger.h:20-22)。
  #[inline]
  pub fn is_listening_to_code_events(&self) -> bool {
    self.is_listening_to_code_events
  }

  /// 对应 `LocalLogger::ScriptEvent`(local-logger.h:24 / .cc:23-25):
  /// 纯转发到 `V8FileLogger::ScriptEvent`; 后者已 1:1 落地于
  /// [`crate::log::script_event`], `log_function_events` 门控由其自查
  /// (对应 `v8_flags.log_function_events`, 读 flags_lite), `time` 由调用方
  /// (原 `V8FileLogger::Time()`)注入。
  #[inline]
  pub fn script_event(
    &self,
    log_file: &mut LogFile,
    event_type: ScriptEventType,
    script_id: i32,
    time: i64,
  ) {
    // C++ 无条件转发(V8FileLogger::ScriptEvent 内部自查 flag); 保持一致。
    let _ = self.is_logging;
    script_event(log_file, self.is_logging, event_type, script_id, time);
  }
}
