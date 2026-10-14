//! 原版 v8/test/unittests/torque/ls-message-unittest.cc 1:1 (8 TEST)
//!
//! 断言规约清单 (先规约后断言, 强度守恒):
//! - language_server_message_initialize_request: initialize 请求 → 响应 id 回带 5,
//!   capabilities.definitionProvider/documentSymbolProvider 同真, writer 恰被调一次
//!   (ls-message-unittest.cc:17-36)
//! - language_server_message_register_dynamic_capabilities_after_initialized_notification:
//!   "initialized" 通知 → 回发注册请求: method == "client/registerCapability",
//!   registrations_size == 1, registration.method == "workspace/didChangeWatchedFiles",
//!   watchers_size == 1 (ls-message-unittest.cc:38-62)
//! - language_server_message_goto_definition_unkown_file: 未登记 URI → 响应 id 42 +
//!   result 为 null (ls-message-unittest.cc:64-81)
//! - language_server_message_goto_definition: 已登记 definition 表 →
//!   未命中位置 (2,0) result 为 null (id 42); 命中位置 (1,5) → result Location:
//!   uri "file://base.tq", range (4,1)-(4,5) (id 43) (ls-message-unittest.cc:83-137)
//! - language_server_message_compilation_error_sends_diagnostics: 带 Error 消息的编译
//!   结果 → publishDiagnostics 通知: uri "<unknown>", diagnostics_size > 0, 首条
//!   severity == kError 且 message 逐字 (ls-message-unittest.cc:139-166)
//! - language_server_message_lint_error_sends_diagnostics: 两条 Lint → 单通知:
//!   uri "file://test.tq", diagnostics_size == 2, 依次 kWarning + 逐字消息
//!   (ls-message-unittest.cc:168-209)
//! - language_server_message_clean_compile_sends_no_diagnostics: 无消息编译结果 →
//!   writer 绝不被调 (FAIL() 路径) (ls-message-unittest.cc:211-221)
//! - language_server_message_no_symbols_sends_empty_response: documentSymbol 请求于
//!   空符号表 → 响应 id 42 + result_size == 0 (ls-message-unittest.cc:223-241)
//!
//! 延期登记 (todo 12.6, 轮4 收口): 依赖 v8-torque LSP 消息面 (LanguageServerMessage/
//! handle_message/writer 协议族) — 未实现 (parser 链已通, implementation-visitor/generator
//! 梯队待填, 看板 v8-torque 行)。LSP 面就位后删除下一行 cfg 恢复本 8 TEST。
#![cfg(any())]

use aok::{OK, Void};
use log::info;
use v8_torque::{
  DiagnosticSeverity, DiagnosticsFiles, DocumentSymbolRequest, DocumentSymbolResponse,
  GotoDefinitionRequest, GotoDefinitionResponse, InitializeRequest, InitializeResponse, JsonValue,
  LanguageServerData, LineAndColumn, MessageKind, MessageWriter, PublishDiagnosticsNotification,
  RegistrationRequest, SourceFileMap, SourceId, SourcePosition, TorqueCompilerResult,
  TorqueFileList, TorqueMessage, compilation_finished, handle_message,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版各 Scope (SourceFileMap::Scope("") / LanguageServerData::Scope /
/// DiagnosticsFiles::Scope) → 测试本地上下文束 (fixture 等价物)
struct LsScope {
  file_map: SourceFileMap,
  server_data: LanguageServerData,
  diagnostics_files: DiagnosticsFiles,
  file_list: TorqueFileList,
}

impl LsScope {
  fn new() -> Self {
    Self {
      file_map: SourceFileMap::new(String::new()),
      server_data: LanguageServerData::default(),
      diagnostics_files: DiagnosticsFiles::default(),
      file_list: TorqueFileList::default(),
    }
  }

  /// 原版 HandleMessage(std::move(request.GetJsonValue()), writer)
  fn handle(&mut self, raw: JsonValue, writer: MessageWriter<'_>) {
    let Self {
      file_map,
      server_data,
      diagnostics_files,
      file_list,
    } = self;
    handle_message(
      raw,
      file_map,
      server_data,
      diagnostics_files,
      file_list,
      writer,
    )
    .expect("规约内分支不触及编译管线, 必成功");
  }
}

/// 原版 SourcePosition 聚合初始化 {source, WithUnknownOffset(l1,c1), (l2,c2)}
fn pos(source: SourceId, l1: i32, c1: i32, l2: i32, c2: i32) -> SourcePosition {
  SourcePosition {
    source,
    start: LineAndColumn::with_unknown_offset(l1, c1),
    end: LineAndColumn::with_unknown_offset(l2, c2),
  }
}

/// 原版 TEST(LanguageServerMessage, InitializeRequest) 1:1
#[test]
fn language_server_message_initialize_request() -> Void {
  info!("> language_server_message_initialize_request");
  let mut scope = LsScope::new();

  let mut request = InitializeRequest::new();
  request.set_id(5);
  request.set_method("initialize");
  // 原版 request.params(); 触碰创建
  let _ = request.params();

  let mut writer_called = false;
  {
    let mut writer = |raw_response: JsonValue| {
      let mut response = InitializeResponse::from_json_value(raw_response);
      // 响应 id 与请求配对, 且语言服务器声明支持定义/符号查询
      assert_eq!(response.id(), 5);
      let mut result = response.result();
      let capabilities = result.capabilities();
      assert!(capabilities.definition_provider());
      assert!(capabilities.document_symbol_provider());
      writer_called = true;
    };
    scope.handle(request.get_json_value(), &mut writer);
  }
  assert!(writer_called);
  OK
}

/// 原版 TEST(LanguageServerMessage, RegisterDynamicCapabilitiesAfterInitializedNotification)
/// 1:1
#[test]
fn language_server_message_register_dynamic_capabilities_after_initialized_notification() -> Void {
  info!("> language_server_message_register_dynamic_capabilities_after_initialized_notification");
  let mut scope = LsScope::new();

  let mut notification = RegistrationRequest::new();
  notification.set_method("initialized");

  let mut writer_called = false;
  {
    let mut writer = |raw_request: JsonValue| {
      let mut request = RegistrationRequest::from_json_value(raw_request);
      assert_eq!(request.method(), "client/registerCapability");
      assert_eq!(request.params().registrations_size(), 1);

      let mut params = request.params();
      let mut registration = params.registrations(0);
      assert_eq!(registration.method(), "workspace/didChangeWatchedFiles");

      let mut options = registration.register_options();
      assert_eq!(options.watchers_size(), 1);

      writer_called = true;
    };
    scope.handle(notification.get_json_value(), &mut writer);
  }
  assert!(writer_called);
  OK
}

/// 原版 TEST(LanguageServerMessage, GotoDefinitionUnkownFile) 1:1 (原版拼写如此)
#[test]
fn language_server_message_goto_definition_unkown_file() -> Void {
  info!("> language_server_message_goto_definition_unkown_file");
  let mut scope = LsScope::new();

  let mut request = GotoDefinitionRequest::new();
  request.set_id(42);
  request.set_method("textDocument/definition");
  {
    let mut params = request.params();
    params.text_document().set_uri("file:///unknown.tq");
  }

  let mut writer_called = false;
  {
    let mut writer = |raw_response: JsonValue| {
      let response = GotoDefinitionResponse::from_json_value(raw_response);
      assert_eq!(response.id(), 42);
      assert!(response.is_null("result"));
      writer_called = true;
    };
    scope.handle(request.get_json_value(), &mut writer);
  }
  assert!(writer_called);
  OK
}

/// 原版 TEST(LanguageServerMessage, GotoDefinition) 1:1 (两轮: 未命中 + 命中)
#[test]
fn language_server_message_goto_definition() -> Void {
  info!("> language_server_message_goto_definition");
  let mut scope = LsScope::new();

  let test_id = scope.file_map.add_source("file://test.tq".to_owned());
  let definition_id = scope.file_map.add_source("file://base.tq".to_owned());
  scope
    .server_data
    .add_definition(pos(test_id, 1, 0, 1, 10), pos(definition_id, 4, 1, 4, 5));

  // 第一轮: 未命中定义, result 必为 null
  let mut request = GotoDefinitionRequest::new();
  request.set_id(42);
  request.set_method("textDocument/definition");
  {
    let mut params = request.params();
    params.text_document().set_uri("file://test.tq");
    let mut position = params.position();
    position.set_line(2);
    position.set_character(0);
  }

  let mut writer_called = false;
  {
    let mut writer = |raw_response: JsonValue| {
      let response = GotoDefinitionResponse::from_json_value(raw_response);
      assert_eq!(response.id(), 42);
      assert!(response.is_null("result"));
      writer_called = true;
    };
    scope.handle(request.get_json_value(), &mut writer);
  }
  assert!(writer_called);

  // 第二轮: 命中定义
  let mut request = GotoDefinitionRequest::new();
  request.set_id(43);
  request.set_method("textDocument/definition");
  {
    let mut params = request.params();
    params.text_document().set_uri("file://test.tq");
    let mut position = params.position();
    position.set_line(1);
    position.set_character(5);
  }

  let mut writer_called = false;
  {
    let mut writer = |raw_response: JsonValue| {
      let mut response = GotoDefinitionResponse::from_json_value(raw_response);
      assert_eq!(response.id(), 43);
      assert!(!response.is_null("result"));

      let mut location = response.result();
      assert_eq!(location.uri(), "file://base.tq");
      let mut range = location.range();
      assert_eq!(range.start().line(), 4);
      assert_eq!(range.start().character(), 1);
      assert_eq!(range.end().line(), 4);
      assert_eq!(range.end().character(), 5);

      writer_called = true;
    };
    scope.handle(request.get_json_value(), &mut writer);
  }
  assert!(writer_called);
  OK
}

/// 原版 TEST(LanguageServerMessage, CompilationErrorSendsDiagnostics) 1:1
/// (原版 { Error("..."); } 消息构造器入 TorqueMessages 上下文 → 直接构造等价消息)
#[test]
fn language_server_message_compilation_error_sends_diagnostics() -> Void {
  info!("> language_server_message_compilation_error_sends_diagnostics");
  let mut scope = LsScope::new();

  let result = TorqueCompilerResult {
    messages: vec![TorqueMessage {
      message: "compilation failed somehow".to_owned(),
      position: None,
      kind: MessageKind::Error,
    }],
    source_file_map: Some(scope.file_map.clone()),
    ..TorqueCompilerResult::default()
  };

  let mut writer_called = false;
  {
    let mut writer = |raw_response: JsonValue| {
      let mut notification = PublishDiagnosticsNotification::from_json_value(raw_response);
      assert_eq!(notification.method(), "textDocument/publishDiagnostics");
      assert!(!notification.is_null("params"));
      assert_eq!(notification.params().uri(), "<unknown>");

      let mut params = notification.params();
      assert!(params.diagnostics_size() > 0);
      let diagnostic = params.diagnostics(0);
      assert_eq!(diagnostic.severity(), DiagnosticSeverity::Error as i64);
      assert_eq!(diagnostic.message(), "compilation failed somehow");

      writer_called = true;
    };
    compilation_finished(
      result,
      &mut scope.file_map,
      &mut scope.server_data,
      &mut scope.diagnostics_files,
      &mut writer,
    );
  }
  assert!(writer_called);
  OK
}

/// 原版 TEST(LanguageServerMessage, LintErrorSendsDiagnostics) 1:1
/// (无编译错误但两条 lint 警告; Lint(...).Position(pos) → kind + position 等价构造)
#[test]
fn language_server_message_lint_error_sends_diagnostics() -> Void {
  info!("> language_server_message_lint_error_sends_diagnostics");
  let mut scope = LsScope::new();

  let test_id = scope.file_map.add_source("file://test.tq".to_owned());
  let pos1 = pos(test_id, 0, 0, 0, 1);
  let pos2 = pos(test_id, 1, 0, 1, 1);

  let result = TorqueCompilerResult {
    messages: vec![
      TorqueMessage {
        message: "lint error 1".to_owned(),
        position: Some(pos1),
        kind: MessageKind::Lint,
      },
      TorqueMessage {
        message: "lint error 2".to_owned(),
        position: Some(pos2),
        kind: MessageKind::Lint,
      },
    ],
    source_file_map: Some(scope.file_map.clone()),
    ..TorqueCompilerResult::default()
  };

  let mut writer_called = false;
  {
    let mut writer = |raw_response: JsonValue| {
      let mut notification = PublishDiagnosticsNotification::from_json_value(raw_response);
      assert_eq!(notification.method(), "textDocument/publishDiagnostics");
      assert!(!notification.is_null("params"));
      assert_eq!(notification.params().uri(), "file://test.tq");

      let mut params = notification.params();
      assert_eq!(params.diagnostics_size(), 2);
      let diagnostic1 = params.diagnostics(0);
      assert_eq!(diagnostic1.severity(), DiagnosticSeverity::Warning as i64);
      assert_eq!(diagnostic1.message(), "lint error 1");

      let diagnostic2 = params.diagnostics(1);
      assert_eq!(diagnostic2.severity(), DiagnosticSeverity::Warning as i64);
      assert_eq!(diagnostic2.message(), "lint error 2");

      writer_called = true;
    };
    compilation_finished(
      result,
      &mut scope.file_map,
      &mut scope.server_data,
      &mut scope.diagnostics_files,
      &mut writer,
    );
  }
  assert!(writer_called);
  OK
}

/// 原版 TEST(LanguageServerMessage, CleanCompileSendsNoDiagnostics) 1:1
/// (无消息 → writer 绝不被调; 原版回调体 FAIL() << "Sending unexpected response!")
#[test]
fn language_server_message_clean_compile_sends_no_diagnostics() -> Void {
  info!("> language_server_message_clean_compile_sends_no_diagnostics");
  let mut scope = LsScope::new();

  let result = TorqueCompilerResult {
    source_file_map: Some(scope.file_map.clone()),
    ..TorqueCompilerResult::default()
  };

  let mut writer = |_raw_response: JsonValue| {
    panic!("Sending unexpected response!");
  };
  compilation_finished(
    result,
    &mut scope.file_map,
    &mut scope.server_data,
    &mut scope.diagnostics_files,
    &mut writer,
  );
  OK
}

/// 原版 TEST(LanguageServerMessage, NoSymbolsSendsEmptyResponse) 1:1
#[test]
fn language_server_message_no_symbols_sends_empty_response() -> Void {
  info!("> language_server_message_no_symbols_sends_empty_response");
  let mut scope = LsScope::new();

  let mut request = DocumentSymbolRequest::new();
  request.set_id(42);
  request.set_method("textDocument/documentSymbol");
  {
    let mut params = request.params();
    params.text_document().set_uri("file://test.tq");
  }

  let mut writer_called = false;
  {
    let mut writer = |raw_response: JsonValue| {
      let mut response = DocumentSymbolResponse::from_json_value(raw_response);
      assert_eq!(response.id(), 42);
      assert_eq!(response.result_size(), 0);
      writer_called = true;
    };
    scope.handle(request.get_json_value(), &mut writer);
  }
  assert!(writer_called);
  OK
}
