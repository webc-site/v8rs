// 由 tool/src/torque/ls.js 从 v8/src/torque/ls/message-handler.h 生成; 修订就地改, 复跑覆盖
// LS 消息处理器

use crate::{
  error::{Error, Result},
  ls::json::JsonValue,
};

/// 原版 MessageHandler 1:1 (纯虚 base → trait)
pub trait MessageHandler {
  /// 原版 HandleMessage
  fn handle_message(&mut self, message: &JsonValue) -> Result<()>;
}

/// 原版 CompilerHandler : MessageHandler (定义于 message-handler.cc)
#[derive(Debug, Default)]
pub struct CompilerHandler;

impl MessageHandler for CompilerHandler {
  fn handle_message(&mut self, message: &JsonValue) -> Result<()> {
    // TODO(填体): initialize / didOpenTextDocument / didChangeTextDocument /
    //   debugging request 分派 (见 .cc 清单)
    let _ = message;
    Err(Error::Unimplemented)
  }
}
// ── 原版 src/torque/ls/message-handler.cc ──
// L27: static const size_t kContentLengthSize = sizeof(kContentLength) - 1;constexpr const char* kProtocolLineEnding = "";constexpr const char* kProtocolLineEnding = "";JsonValue ReadMessage()
// L56: void WriteMessage(JsonValue message)
// L67: void ResetCompilationErrorDiagnostics(MessageWriter writer)
// L162: void SendCompilationDiagnostics(const TorqueCompilerResult& result, MessageWriter writer)
// L184: void CompilationFinished(TorqueCompilerResult result, MessageWriter writer)
// L193: void RecompileTorque(MessageWriter writer)
// L208: void RecompileTorqueWithDiagnostics(MessageWriter writer)
// L213: void HandleInitializeRequest(InitializeRequest request, MessageWriter writer)
// L230: void HandleInitializedNotification(MessageWriter writer)
// L249: void HandleTorqueFileListNotification(TorqueFileListNotification notification, MessageWriter writer)
// L268: void HandleGotoDefinitionRequest(GotoDefinitionRequest request, MessageWriter writer)
// L298: void HandleChangeWatchedFilesNotification( DidChangeWatchedFilesNotification notification, MessageWriter writer)
// L305: void HandleDocumentSymbolRequest(DocumentSymbolRequest request, MessageWriter writer)
// L353: void HandleMessage(JsonValue raw_message, MessageWriter writer)
