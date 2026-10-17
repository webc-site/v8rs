// 由 tool/src/torque/ls.js 从 v8/src/torque/ls/message.h 生成; 修订就地改, 复跑覆盖
// LS 消息模型 (LSP JSON 协议)

use crate::ls::json::JsonObject;

// 原版 BaseJsonAccessor/Message/NestedJsonAccessor 是 JSON 树上的强类型访问器;
// Rust 版直接以 JsonValue 树承载, 各消息类型保留字段清单 (访问器骨架见原版)。
/// 原版 Message (JSON 访问器; 字段清单见原版)
#[derive(Debug, Clone, Default)]
pub struct Message {
  pub object: JsonObject,
}

/// 原版 ResponseError (JSON 访问器; 字段清单见原版)
#[derive(Debug, Clone, Default)]
pub struct ResponseError {
  pub object: JsonObject,
}

/// 原版 InitializeParams (JSON 访问器; 字段清单见原版)
#[derive(Debug, Clone, Default)]
pub struct InitializeParams {
  pub object: JsonObject,
}

/// 原版 FileListParams (JSON 访问器; 字段清单见原版)
#[derive(Debug, Clone, Default)]
pub struct FileListParams {
  pub object: JsonObject,
}

/// 原版 DidChangeWatchedFilesRegistrationOptions (JSON 访问器; 字段清单见原版)
#[derive(Debug, Clone, Default)]
pub struct DidChangeWatchedFilesRegistrationOptions {
  pub object: JsonObject,
}

/// 原版 FileEvent (JSON 访问器; 字段清单见原版)
#[derive(Debug, Clone, Default)]
pub struct FileEvent {
  pub object: JsonObject,
}

/// 原版 DidChangeWatchedFilesParams (JSON 访问器; 字段清单见原版)
#[derive(Debug, Clone, Default)]
pub struct DidChangeWatchedFilesParams {
  pub object: JsonObject,
}

/// 原版 SaveOptions (JSON 访问器; 字段清单见原版)
#[derive(Debug, Clone, Default)]
pub struct SaveOptions {
  pub object: JsonObject,
}

/// 原版 TextDocumentSyncOptions (JSON 访问器; 字段清单见原版)
#[derive(Debug, Clone, Default)]
pub struct TextDocumentSyncOptions {
  pub object: JsonObject,
}

/// 原版 ServerCapabilities (JSON 访问器; 字段清单见原版)
#[derive(Debug, Clone, Default)]
pub struct ServerCapabilities {
  pub object: JsonObject,
}

/// 原版 InitializeResult (JSON 访问器; 字段清单见原版)
#[derive(Debug, Clone, Default)]
pub struct InitializeResult {
  pub object: JsonObject,
}

/// 原版 Registration (JSON 访问器; 字段清单见原版)
#[derive(Debug, Clone, Default)]
pub struct Registration {
  pub object: JsonObject,
}

/// 原版 RegistrationParams (JSON 访问器; 字段清单见原版)
#[derive(Debug, Clone, Default)]
pub struct RegistrationParams {
  pub object: JsonObject,
}

/// 原版 JsonPosition (JSON 访问器; 字段清单见原版)
#[derive(Debug, Clone, Default)]
pub struct JsonPosition {
  pub object: JsonObject,
}

/// 原版 Range (JSON 访问器; 字段清单见原版)
#[derive(Debug, Clone, Default)]
pub struct Range {
  pub object: JsonObject,
}

/// 原版 Location (JSON 访问器; 字段清单见原版)
#[derive(Debug, Clone, Default)]
pub struct Location {
  pub object: JsonObject,
}

/// 原版 TextDocumentIdentifier (JSON 访问器; 字段清单见原版)
#[derive(Debug, Clone, Default)]
pub struct TextDocumentIdentifier {
  pub object: JsonObject,
}

/// 原版 TextDocumentPositionParams (JSON 访问器; 字段清单见原版)
#[derive(Debug, Clone, Default)]
pub struct TextDocumentPositionParams {
  pub object: JsonObject,
}

/// 原版 Diagnostic (JSON 访问器; 字段清单见原版)
#[derive(Debug, Clone, Default)]
pub struct Diagnostic {
  pub object: JsonObject,
}

/// 原版 PublishDiagnosticsParams (JSON 访问器; 字段清单见原版)
#[derive(Debug, Clone, Default)]
pub struct PublishDiagnosticsParams {
  pub object: JsonObject,
}

/// 原版 FileSystemWatcher::WatchKind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum WatchKind {
  Create,
  Change,
  Delete,
}

/// 原版 Diagnostic::DiagnosticSeverity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DiagnosticSeverity {
  Error,
  Warning,
  Information,
  Hint,
}

/// 原版 SymbolKind (无作用域 enum)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SymbolKind {
  File,
  Module,
  Namespace,
  Package,
  Class,
  Method,
  Property,
  Field,
  Constructor,
  Enum,
  Interface,
  Function,
  Variable,
  Constant,
  String,
  Number,
  Boolean,
  Array,
  Object,
  Key,
  Null,
  EnumMember,
  Struct,
  Event,
  Operator,
  TypeParameter,
}

// 填体提示: 原版访问器方法 (set/get/ToJsonObject) 按 JsonValue 树直填;
// helper 函数 (ToJson/FromJson) 随 message_handler 填体补齐。
