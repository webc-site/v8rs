// 由 tool/src/parsing/parse-info.cc.js 从 v8/src/parsing/{parse-info,parsing,token,literal-buffer,func-name-inferrer,import-attributes,pending-compilation-error-handler,rewriter}.{h,cc} 生成; 修订就地改, 复跑覆盖
// 解析杂项族骨架

/// 杂项族结构锚点 (14 文件登记)
pub const PARSING_MISC_ANCHORS: [&str; 14] = [
  "parse-info.h",
  "parse-info.cc",
  "parsing.h",
  "parsing.cc",
  "token.cc",
  "literal-buffer.cc",
  "func-name-inferrer.h",
  "func-name-inferrer.cc",
  "import-attributes.h",
  "import-attributes.cc",
  "pending-compilation-error-handler.h",
  "pending-compilation-error-handler.cc",
  "rewriter.h",
  "rewriter.cc",
];

use crate::scanner_generated::Location;

/// 解析参数包 (原版 v8/src/parsing/parse-info.h ParseInfo)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseInfo {
  pub is_toplevel: bool,
  pub is_module: bool,
  pub is_eval: bool,
  pub is_eager: bool,
  pub start_position: usize,
  pub end_position: usize,
  pub function_literal_id: i32,
}

impl Default for ParseInfo {
  fn default() -> Self {
    Self {
      is_toplevel: true,
      is_module: false,
      is_eval: false,
      is_eager: true,
      start_position: 0,
      end_position: 0,
      function_literal_id: 0,
    }
  }
}

/// 函数名推断器 (原版 v8/src/parsing/func-name-inferrer.h FuncNameInferrer)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FuncNameInferrer {
  names_stack: Vec<String>,
  scope_depth: usize,
}

impl FuncNameInferrer {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn push_enclosing_name(&mut self, name: String) {
    self.names_stack.push(name);
  }

  pub fn push_literal_name(&mut self, name: String) {
    self.names_stack.push(name);
  }

  pub fn enter(&mut self) {
    self.scope_depth += 1;
  }

  pub fn leave(&mut self) {
    self.scope_depth = self.scope_depth.saturating_sub(1);
  }

  pub fn infer(&self) -> Option<String> {
    if self.names_stack.is_empty() {
      None
    } else {
      Some(self.names_stack.join("."))
    }
  }

  pub const fn is_open(&self) -> bool {
    self.scope_depth > 0
  }
}

/// import 属性表 (原版 v8/src/parsing/import-attributes.h ImportAttributes)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ImportAttributes {
  pub attributes: Vec<(String, String, Location)>,
}

impl ImportAttributes {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn insert(&mut self, key: String, value: String, loc: Location) {
    self.attributes.push((key, value, loc));
  }

  pub fn get(&self, key: &str) -> Option<&(String, String, Location)> {
    self.attributes.iter().find(|(k, ..)| k == key)
  }

  pub fn has(&self, key: &str) -> bool {
    self.get(key).is_some()
  }
}

/// 延迟编译错误处理器 (原版 v8/src/parsing/pending-compilation-error-handler.h PendingCompilationErrorHandler)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PendingCompilationErrorHandler {
  pub has_pending_error: bool,
  pub stack_overflow: bool,
  pub error_message: Option<(usize, usize, String)>,
  pub warnings: Vec<(usize, usize, String)>,
}

impl PendingCompilationErrorHandler {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn report_error_at(&mut self, start: usize, end: usize, msg: String) {
    self.has_pending_error = true;
    self.error_message = Some((start, end, msg));
  }

  pub fn report_warning_at(&mut self, start: usize, end: usize, msg: String) {
    self.warnings.push((start, end, msg));
  }

  pub fn set_stack_overflow(&mut self) {
    self.has_pending_error = true;
    self.stack_overflow = true;
  }
}

/// AST 重写器 (原版 v8/src/parsing/rewriter.h Rewriter)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Rewriter {
  pub result_assigned: bool,
}

impl Rewriter {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn rewrite(&mut self) -> bool {
    self.result_assigned = true;
    true
  }
}

pub mod parsing_misc_skel {
  use super::{
    FuncNameInferrer, ImportAttributes, ParseInfo, PendingCompilationErrorHandler, Rewriter,
  };
  use crate::{
    character_stream::{OneByteCharacterStream, ScannerStream},
    literal_buffer::LiteralBuffer,
    scanner::Scanner,
    token_generated::Token,
  };

  /// 解析入口 (原版 parsing.cc ParseProgram)
  pub fn parse_program() {
    let stream = OneByteCharacterStream::new(b"");
    let mut scanner = Scanner::new(stream);
    scanner.initialize();
    let _ = scanner.next_token();
  }

  /// 扫描并解析源文本所有 tokens (支持完整字符集与容量预分配)
  pub fn parse_source(source: &str) -> Vec<Token> {
    let stream = ScannerStream::for_testing(source);
    let mut scanner = Scanner::new(stream);
    scanner.initialize();
    let mut tokens = Vec::with_capacity((source.len() / 4).max(8));
    loop {
      let t = scanner.next_token();
      tokens.push(t);
      if t == Token::Eos {
        break;
      }
    }
    tokens
  }

  /// 解析参数包 (原版 ParseInfo: 入口/标志/结果聚合)
  pub fn parse_info_skel() -> ParseInfo {
    ParseInfo::default()
  }

  /// token 名表 (原版 token.cc name_/string_/string_length_/precedence_ 表)
  pub const TOKEN_TABLES: [&str; 4] = ["name_", "string_", "string_length_", "precedence_"];

  /// 字面缓冲 (原版 LiteralBuffer: 增长式字符缓冲, ASCII/UTF-16 双态)
  pub fn literal_buffer_skel() -> LiteralBuffer {
    LiteralBuffer::new()
  }

  /// 函数名推断 (原版 FuncNameInferrer: 匿名函数按赋值/属性上下文命名)
  pub fn func_name_inferrer_skel() -> FuncNameInferrer {
    FuncNameInferrer::new()
  }

  /// import 属性 (原版 ImportAttributes: with 选项表)
  pub fn import_attributes_skel() -> ImportAttributes {
    ImportAttributes::new()
  }

  /// 延迟编译错误 (原版 PendingCompilationErrorHandler: 惰性函数体错误转发)
  pub fn pending_error_handler_skel() -> PendingCompilationErrorHandler {
    PendingCompilationErrorHandler::new()
  }

  /// 重写器 (原版 Rewriter: 顶层表达式结果丢弃补 undefined)
  pub fn rewriter_skel() -> Rewriter {
    Rewriter::new()
  }
}
