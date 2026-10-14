// 由 tool/src/logging/code-events.h.js 从 v8/src/logging/code-events.h 生成; 修订就地改, 复跑覆盖
// 代码事件 X-macro 展开

/// 代码事件枚举 (原版 LogEventListener::Event, LOG_EVENT_LIST 展开 + kLength)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Event {
  /// "code-creation"
  CODE_CREATION = 0,
  /// "code-disable-optimization"
  CODE_DISABLE_OPT = 1,
  /// "code-move"
  CODE_MOVE = 2,
  /// "code-deopt"
  CODE_DEOPT = 3,
  /// "code-delete"
  CODE_DELETE = 4,
  /// "code-moving-gc"
  CODE_MOVING_GC = 5,
  /// "sfi-move"
  SHARED_FUNC_MOVE = 6,
  /// "snapshot-code-name"
  SNAPSHOT_CODE_NAME = 7,
  /// "tick"
  TICK = 8,
  /// 哨兵 (原版 kLength)
  Length = 9,
}

impl Event {
  /// 总数含哨兵 (原版 kLength)
  pub const COUNT: usize = 10;

  /// 日志串 (原版 V(kName, "log-string") 第二列)
  #[must_use]
  pub const fn log_str(self) -> Option<&'static str> {
    match self {
      Self::CODE_CREATION => Some("code-creation"),
      Self::CODE_DISABLE_OPT => Some("code-disable-optimization"),
      Self::CODE_MOVE => Some("code-move"),
      Self::CODE_DEOPT => Some("code-deopt"),
      Self::CODE_DELETE => Some("code-delete"),
      Self::CODE_MOVING_GC => Some("code-moving-gc"),
      Self::SHARED_FUNC_MOVE => Some("sfi-move"),
      Self::SNAPSHOT_CODE_NAME => Some("snapshot-code-name"),
      Self::TICK => Some("tick"),
      _ => None,
    }
  }
}

/// 代码类型标签 (原版 LogEventListener::CodeTag, CODE_TYPE_LIST 展开)
/// 注: Native* 两项写入日志时映射回原标签 (原版注释)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CodeTag {
  /// Builtin
  BUILTIN = 0,
  /// Callback
  CALLBACK = 1,
  /// Eval
  EVAL = 2,
  /// JS
  FUNCTION = 3,
  /// Handler
  HANDLER = 4,
  /// BytecodeHandler
  BYTECODE_HANDLER = 5,
  /// RegExp
  REG_EXP = 6,
  /// Script
  SCRIPT = 7,
  /// Stub
  STUB = 8,
  /// JS
  NATIVE_FUNCTION = 9,
  /// Script
  NATIVE_SCRIPT = 10,
  /// 哨兵 (原版 kLength)
  Length = 11,
}

impl CodeTag {
  /// 总数含哨兵
  pub const COUNT: usize = 12;

  /// 写日志用的类型名 (原版第二列; kFunction→JS, kNativeFunction→JS 等)
  #[must_use]
  pub const fn type_name(self) -> Option<&'static str> {
    match self {
      Self::BUILTIN => Some("Builtin"),
      Self::CALLBACK => Some("Callback"),
      Self::EVAL => Some("Eval"),
      Self::FUNCTION => Some("JS"),
      Self::HANDLER => Some("Handler"),
      Self::BYTECODE_HANDLER => Some("BytecodeHandler"),
      Self::REG_EXP => Some("RegExp"),
      Self::SCRIPT => Some("Script"),
      Self::STUB => Some("Stub"),
      Self::NATIVE_FUNCTION => Some("JS"),
      Self::NATIVE_SCRIPT => Some("Script"),
      _ => None,
    }
  }
}
