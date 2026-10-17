// LS 子模块装配: crate 内转发 (crate AGENTS.md: 禁 pub mod 对外)
pub(crate) mod globals;
pub use globals::{Logger, TorqueFileList};
pub(crate) mod json;
pub use json::{JsonArray, JsonMember, JsonObject, JsonValue, serialize_to_string};
pub(crate) mod json_parser;
pub use json_parser::{JsonParserResult, parse_json};
pub(crate) mod message;
pub use message::{
  Diagnostic, DiagnosticSeverity, DidChangeWatchedFilesParams,
  DidChangeWatchedFilesRegistrationOptions, FileEvent, FileListParams, InitializeParams,
  InitializeResult, JsonPosition, Location, Message, PublishDiagnosticsParams, Range, Registration,
  RegistrationParams, ResponseError, SaveOptions, ServerCapabilities, SymbolKind,
  TextDocumentIdentifier, TextDocumentPositionParams, TextDocumentSyncOptions, WatchKind,
};
pub(crate) mod message_handler;
pub use message_handler::{CompilerHandler, MessageHandler};
pub(crate) mod message_macros;
pub(crate) mod message_pipe;
pub use message_pipe::{read_message, write_message};
pub(crate) mod torque_language_server;
pub use torque_language_server::run_language_server;
