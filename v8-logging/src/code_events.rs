//! 1:1 转写自 `v8/src/logging/code-events.h`(可移植部分:事件/标签枚举与名称表)。
//!
//! 【推迟登记】`LogEventListener` 纯虚接口、`Logger` 监听器注册表(RecursiveMutex
//! + add/remove/find/dedup)、`wasm` 相关事件——它们耦合 Heap/Handle/Isolate,
//!   待 v8-heap/v8-objects 可用后回收。

/// 对应 `LogEventListener::Event`(`LOG_EVENT_LIST(X-macro)`,原版即
/// `enum class Event : uint8_t`,含 `kLength` 哨兵)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Event {
  CodeCreation = 0,
  CodeDisableOpt = 1,
  CodeMove = 2,
  CodeDeopt = 3,
  CodeDelete = 4,
  CodeMovingGC = 5,
  SharedFuncMove = 6,
  SnapshotCodeName = 7,
  Tick = 8,
  /// 对应 `kLength`。
  Length = 9,
}

/// 旧版移植命名(V8 早期该类名为 `CodeEvent`);与 lib.rs 摊平导出面兼容,
/// 名字以原版 `Event` 为准,此处保留别名。
pub type CodeEvent = Event;

/// `LOG_EVENT_LIST` 的机械展开(顺序与 C++ 一致)。
pub const LOG_EVENT_NAMES: [&str; 9] = [
  "code-creation",
  "code-disable-optimization",
  "code-move",
  "code-deopt",
  "code-delete",
  "code-moving-gc",
  "sfi-move",
  "snapshot-code-name",
  "tick",
];

/// 对应 `LogEventListener::CodeTag`(`CODE_TYPE_LIST(X-macro)`)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CodeTag {
  Builtin = 0,
  Callback = 1,
  Eval = 2,
  Function = 3,
  Handler = 4,
  BytecodeHandler = 5,
  RegExp = 6,
  Script = 7,
  Stub = 8,
  NativeFunction = 9,
  NativeScript = 10,
  /// 对应 `kLength`。
  Length = 11,
}

/// `CODE_TYPE_LIST` 的机械展开(注意 Native 变体映射回原始标签名)。
pub const CODE_TAG_NAMES: [&str; 11] = [
  "Builtin",
  "Callback",
  "Eval",
  "JS",
  "Handler",
  "BytecodeHandler",
  "RegExp",
  "Script",
  "Stub",
  "JS",
  "Script",
];

/// 事件名(对应 C++ `kLogEventsNames` / `operator<<(ostream&, Event)`)。
#[inline]
pub fn event_name(event: CodeEvent) -> &'static str {
  let idx = event as usize;
  if idx < LOG_EVENT_NAMES.len() {
    LOG_EVENT_NAMES[idx]
  } else {
    // kLength:与 C++ 越界读不同,此处返回空串(禁 panic)。
    ""
  }
}

/// 标签名(对应 C++ `kCodeTagNames` / `operator<<(ostream&, CodeTag)`)。
#[inline]
pub fn code_tag_name(tag: CodeTag) -> &'static str {
  let idx = tag as usize;
  if idx < CODE_TAG_NAMES.len() {
    CODE_TAG_NAMES[idx]
  } else {
    ""
  }
}

/// 对应 `v8::CodeEventType`(`include/v8-profiler.h`,API 暴露枚举)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CodeEventType {
  Unknown = 0,
  BuiltinType = 1,
  CallbackType = 2,
  EvalType = 3,
  FunctionType = 4,
  InterpretedFunctionType = 5,
  HandlerType = 6,
  BytecodeHandlerType = 7,
  LazyCompileType = 8,
  RegExpType = 9,
  ScriptType = 10,
  StubType = 11,
  RelocationType = 12,
}

/// 1:1 对应 `GetCodeEventTypeForTag(LogEventListener::CodeTag)`(log.cc:94-125)。
/// C++ 中 `kLength` 与 `kBuiltin` 同映射 `kBuiltinType`,其余分支全对齐;
/// C++ 末尾 `UNREACHABLE()` 对应本枚举无未覆盖变体,无需 fallback。
#[inline]
pub fn get_code_event_type_for_tag(tag: CodeTag) -> CodeEventType {
  match tag {
    // 手工维护的分支,因 v8::CodeEventType 是 API 暴露枚举,不能随意修改。
    CodeTag::Length | CodeTag::Builtin => CodeEventType::BuiltinType,
    CodeTag::Callback => CodeEventType::CallbackType,
    CodeTag::Eval => CodeEventType::EvalType,
    CodeTag::NativeFunction | CodeTag::Function => CodeEventType::FunctionType,
    CodeTag::Handler => CodeEventType::HandlerType,
    CodeTag::BytecodeHandler => CodeEventType::BytecodeHandlerType,
    CodeTag::RegExp => CodeEventType::RegExpType,
    CodeTag::NativeScript | CodeTag::Script => CodeEventType::ScriptType,
    CodeTag::Stub => CodeEventType::StubType,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tag_names_match_cpp() {
    assert_eq!(code_tag_name(CodeTag::Function), "JS");
    assert_eq!(code_tag_name(CodeTag::NativeFunction), "JS");
    assert_eq!(code_tag_name(CodeTag::NativeScript), "Script");
    assert_eq!(event_name(CodeEvent::CodeCreation), "code-creation");
    assert_eq!(event_name(CodeEvent::Tick), "tick");
  }
}
