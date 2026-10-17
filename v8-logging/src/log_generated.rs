// 由 tool/src/logging/log.h.js 从 v8/src/logging/log.{h,cc,inl.h} + log-file.{h,cc} 生成; 修订就地改, 复跑覆盖
// 日志器: 分隔符/文件名/监听器面

/// 日志行分隔符 (原版 enum class LogSeparator)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LogSeparator {
  Separator = 0,
}

/// 日志目标常量文件名 (原版 log-file.h 声明 + log-file.cc 定义)
pub mod log_file_const {
  /// 原版 kLogToTemporaryFile
  pub const LOG_TO_TEMPORARY_FILE: &str = "+";
  /// 原版 kLogToConsole
  pub const LOG_TO_CONSOLE: &str = "-";
}

/// LogEventListener 虚面 (原版 log.h LogEventListener 派生的 wasm 监听虚函数对数)
pub const EVENT_LISTENER_METHODS: [&str; 2] = ["LogRecordedBuffer", "LogRecordedBuffer"];

/// V8FileLogger 事件写入面 (原版 log.h, 74 项 *Event 方法, 顺序登记)
pub const LOGGER_EVENT_METHODS: [&str; 74] = ["IntPtrTEvent", "NewEvent", "DeleteEvent", "FunctionEvent", "FunctionEvent", "CompilationCacheEvent", "ScriptEvent", "CodeCreateEvent", "CodeCreateEvent", "CodeCreateEvent", "CodeCreateEvent", "CodeCreateEvent", "CallbackEvent", "GetterCallbackEvent", "SetterCallbackEvent", "RegExpCodeCreateEvent", "CodeMoveEvent", "BytecodeMoveEvent", "SharedFunctionInfoMoveEvent", "NativeContextMoveEvent", "CodeMovingGCEvent", "CodeDisableOptEvent", "CodeDeoptEvent", "CodeDependencyChangeEvent", "FeedbackVectorEvent", "WeakCodeClearEvent", "ProcessDeoptEvent", "CodeLinePosInfoRecordEvent", "WasmCodeLinePosInfoRecordEvent", "CodeNameEvent", "ICEvent", "MapEvent", "MapEvent", "MapMoveEvent", "SharedLibraryEvent", "CurrentTimeEvent", "ProfilerBeginEvent", "TickEvent", "RuntimeCallTimerEvent", "UncheckedStringEvent", "LogTimerEvent", "CodeCreateEvent", "CodeCreateEvent", "CodeCreateEvent", "CodeCreateEvent", "CodeCreateEvent", "RegExpCodeCreateEvent", "CallbackEvent", "GetterCallbackEvent", "SetterCallbackEvent", "SharedFunctionInfoMoveEvent", "NativeContextMoveEvent", "CodeMovingGCEvent", "CodeDeoptEvent", "CodeDependencyChangeEvent", "WeakCodeClearEvent", "CodeCreateEvent", "CodeCreateEvent", "CodeCreateEvent", "CodeCreateEvent", "CodeCreateEvent", "RegExpCodeCreateEvent", "CallbackEvent", "GetterCallbackEvent", "SetterCallbackEvent", "SharedFunctionInfoMoveEvent", "NativeContextMoveEvent", "CodeMoveEvent", "BytecodeMoveEvent", "CodeDisableOptEvent", "CodeMovingGCEvent", "CodeDeoptEvent", "CodeDependencyChangeEvent", "WeakCodeClearEvent"];

/// 签名骨架 (语义见手写 log.rs / log_file.rs)
pub mod log_skel {
  /// 消息构造协议 (原版 LogFile::MessageBuilder): 流式 << 拼接, 析构时整行落盘并转义
  pub fn message_builder_skel() {
    unimplemented!("骨架")
  }
  /// Native 标签归一 (原版 log-inl: kFunction/kScript 落盘前转 Native 标签)
  pub fn normalize_native_tag() {
    unimplemented!("骨架")
  }
  /// 计时事件落盘 (原版 TimerEventScope::LogTimerEvent → CallEventLogger)
  pub fn log_timer_event() {
    unimplemented!("骨架")
  }
}
