// 原版 base::ContextualClass 体系 (src/base/contextual.h) 的 thread_local 直译:
// CurrentAst / CurrentSourcePosition / CurrentSourceFile / TorqueMessages /
// SourceFileMap::Scope / TargetArchitecture::Scope / BuildFlags /
// CurrentScope / TypeOracle 静态单例 / LanguageServerData。
// 编译器为单线程管线, thread_local + RefCell 对齐原版 Scope 栈语义。
//
// 借用纪律 (红线, 防 RefCell 重入 panic):
//   with_ast 的闭包内严禁再调 with_ast; 辅助函数一律显式收 &mut Ast 参数。

use std::{
  cell::{Cell, RefCell},
  mem::{replace, take},
};

use crate::{
  ast::Ast,
  declarable::DeclarableId,
  error::{Error, Result},
  global_context::TargetArchitecture,
  server_data::LanguageServerData,
  source_positions::{SourceFileMap, SourceId, SourcePosition},
  type_oracle::TypeOracle,
  utils::{MessageKind, TorqueMessage},
};

// ── CurrentAst (ast.h: CurrentAst::Scope/Get) ──

thread_local! {
  static CURRENT_AST: RefCell<Ast> = RefCell::new(Ast::new());
}

/// 原版 CurrentAst::Scope: 整体换入/换出 (parse 结束后取走 AST)
pub fn take_ast() -> Ast {
  CURRENT_AST.with_borrow_mut(take)
}

/// 原版 CurrentAst::Scope(ast): 换入新 AST (保留旧值供 swap 语义)
pub fn replace_ast(ast: Ast) -> Ast {
  CURRENT_AST.with_borrow_mut(|slot| replace(slot, ast))
}

/// 原版 CurrentAst::Get(): 借用可变 AST; 闭包内禁重入
pub fn with_ast<R>(f: impl FnOnce(&mut Ast) -> R) -> R {
  CURRENT_AST.with_borrow_mut(f)
}

// ── CurrentSourcePosition / CurrentSourceFile (source-positions.h) ──

thread_local! {
  static CURRENT_SOURCE_POSITION: Cell<SourcePosition> =
    const { Cell::new(SourcePosition::invalid()) };
  static CURRENT_SOURCE_FILE: Cell<SourceId> = const { Cell::new(SourceId::invalid()) };
}

/// 原版 CurrentSourcePosition::Get()
#[must_use]
pub fn current_source_position() -> SourcePosition {
  CURRENT_SOURCE_POSITION.with(Cell::get)
}

/// 原版 CurrentSourcePosition::Scope: Drop 恢复旧值
pub struct SourcePositionScope {
  previous: SourcePosition,
}

impl SourcePositionScope {
  #[must_use]
  pub fn new(pos: SourcePosition) -> Self {
    let previous = current_source_position();
    CURRENT_SOURCE_POSITION.with(|slot| slot.set(pos));
    Self { previous }
  }
}

impl Drop for SourcePositionScope {
  fn drop(&mut self) {
    CURRENT_SOURCE_POSITION.with(|slot| slot.set(self.previous));
  }
}

/// 原版 CurrentSourceFile::Get()
#[must_use]
pub fn current_source_file() -> SourceId {
  CURRENT_SOURCE_FILE.with(Cell::get)
}

/// 原版 CurrentSourceFile::Scope: Drop 恢复旧值
pub struct SourceFileScope {
  previous: SourceId,
}

impl SourceFileScope {
  #[must_use]
  pub fn new(source: SourceId) -> Self {
    let previous = current_source_file();
    CURRENT_SOURCE_FILE.with(|slot| slot.set(source));
    Self { previous }
  }
}

impl Drop for SourceFileScope {
  fn drop(&mut self) {
    CURRENT_SOURCE_FILE.with(|slot| slot.set(self.previous));
  }
}

// ── SourceFileMap::Scope (source-positions.h: 上下文单例) ──

thread_local! {
  static SOURCE_FILE_MAP: RefCell<SourceFileMap> = RefCell::new(SourceFileMap::new(String::new()));
}

/// 原版 SourceFileMap::Scope(v8_root): 换入映射 (旧值由换出返回, 结果阶段取回用 Get)
pub fn replace_source_file_map(map: SourceFileMap) -> SourceFileMap {
  SOURCE_FILE_MAP.with_borrow_mut(|slot| replace(slot, map))
}

/// 原版 SourceFileMap::Get(): 克隆取走
#[must_use]
pub fn get_source_file_map() -> SourceFileMap {
  SOURCE_FILE_MAP.with_borrow(Clone::clone)
}

/// 原版 SourceFileMap::AddSource
pub fn add_source(path: String) -> SourceId {
  SOURCE_FILE_MAP.with_borrow_mut(|map| map.add_source(path))
}

/// 原版 SourceFileMap::GetSourceId
#[must_use]
pub fn get_source_id(path: &str) -> Option<SourceId> {
  SOURCE_FILE_MAP.with_borrow(|map| map.get_source_id(path))
}

/// 原版 SourceFileMap::AbsolutePath
pub fn absolute_path(file: SourceId) -> Result<String> {
  SOURCE_FILE_MAP.with_borrow(|map| map.absolute_path(file))
}

/// 原版 SourceFileMap::FileRelativeToV8RootExists
#[must_use]
pub fn file_relative_to_v8_root_exists(path: &str) -> bool {
  SOURCE_FILE_MAP.with_borrow(|map| map.file_relative_to_v8_root_exists(path))
}

// ── TorqueMessages (utils.h: ContextualVariable 消息收集) ──

thread_local! {
  static TORQUE_MESSAGES: RefCell<Vec<TorqueMessage>> = const { RefCell::new(Vec::new()) };
}

/// 原版 TorqueMessages::Get(): 取走全部消息
#[must_use]
pub fn take_messages() -> Vec<TorqueMessage> {
  TORQUE_MESSAGES.with_borrow_mut(take)
}

/// 原版 MessageBuilder 析构: 消息入列, 默认位置 = CurrentSourcePosition
fn push_message(kind: MessageKind, message: String, pos: Option<SourcePosition>) {
  TORQUE_MESSAGES.with_borrow_mut(|messages| {
    messages.push(TorqueMessage {
      message,
      position: Some(pos.unwrap_or_else(current_source_position)),
      kind,
    });
  });
}

/// 原版 Lint(...): 记 kLint 消息, 编译继续
pub fn lint(message: String) {
  push_message(MessageKind::Lint, message, None);
}

/// 原版 Lint(...).Position(pos)
pub fn lint_at(message: String, pos: SourcePosition) {
  push_message(MessageKind::Lint, message, Some(pos));
}

/// 原版 Error(...) 不抛: 记 kError 消息, 调用方继续
pub fn error_continue(message: String) {
  push_message(MessageKind::Error, message, None);
}

/// 原版 Error(...).Position(pos) 不抛
pub fn error_continue_at(message: String, pos: SourcePosition) {
  push_message(MessageKind::Error, message, Some(pos));
}

/// 原版 Error(...).Throw() / ReportError(...): 记 kError 消息并中止编译
/// (TorqueAbortCompilation → Error::Abort, 由 CompileTorque 的 catch 收敛)
pub fn report_error(message: String) -> Error {
  push_message(MessageKind::Error, message, None);
  Error::Abort
}

// ── TargetArchitecture::Scope (global-context.cc:28-38) ──

thread_local! {
  static TARGET_ARCHITECTURE: Cell<TargetArchitecture> =
    const { Cell::new(TargetArchitecture::with_sizes(0, 0, 0, 0, 0, 0)) };
}

/// 原版 TargetArchitecture::Scope(force_32bit); 尺寸对齐上游默认 x64 release
/// (kTaggedSize=4 ptr-compr, kSystemPointerSize=8, sandbox 下 external/cppheap/trusted=4)
pub fn set_target_architecture(force_32bit: bool) {
  let arch = TargetArchitecture::new(force_32bit);
  TARGET_ARCHITECTURE.with(|slot| slot.set(arch));
}

/// 原版 TargetArchitecture::TaggedSize()
#[must_use]
pub fn tagged_size() -> usize {
  TARGET_ARCHITECTURE.with(Cell::get).tagged_size()
}

// ── CurrentScope (declarable.h: ContextualClass<CurrentScope>, 栈式作用域) ──

thread_local! {
  static CURRENT_SCOPE: RefCell<Vec<Option<DeclarableId>>> = const { RefCell::new(Vec::new()) };
}

/// 原版 CurrentScope::get(): 栈顶作用域 (空栈 = nullptr)
#[must_use]
pub fn current_scope() -> Option<DeclarableId> {
  CURRENT_SCOPE.with_borrow(|stack| stack.last().copied().flatten())
}

/// 原版 CurrentScope::Scope: 入栈新作用域; Drop 出栈恢复
pub struct CurrentScopeGuard;

impl CurrentScopeGuard {
  #[must_use]
  pub fn new(scope: Option<DeclarableId>) -> Self {
    CURRENT_SCOPE.with(|stack| stack.borrow_mut().push(scope));
    Self
  }
}

impl Drop for CurrentScopeGuard {
  fn drop(&mut self) {
    CURRENT_SCOPE.with(|stack| {
      stack.borrow_mut().pop();
    });
  }
}

// ── TypeOracle::get() 静态单例桥 (type-oracle.h: ContextualClass) ──
// 借用纪律: with_type_oracle 闭包内只允许纯读叶子调用 (is_subtype_of 等), 禁重入。

thread_local! {
  static TYPE_ORACLE: RefCell<TypeOracle> = RefCell::new(TypeOracle::default());
}

/// 原版 TypeOracle::Get() 只读借用; 闭包内禁重入
pub fn with_type_oracle<R>(f: impl FnOnce(&TypeOracle) -> R) -> R {
  TYPE_ORACLE.with_borrow(f)
}

/// 原版 TypeOracle::Get() 可变借用; 闭包内禁重入
pub fn with_type_oracle_mut<R>(f: impl FnOnce(&mut TypeOracle) -> R) -> R {
  TYPE_ORACLE.with_borrow_mut(f)
}

/// 换入/换出 TypeOracle 实例 (编译入口装载用)
pub fn replace_type_oracle(oracle: TypeOracle) -> TypeOracle {
  TYPE_ORACLE.with_borrow_mut(|slot| replace(slot, oracle))
}

// ── LanguageServerData 静态单例桥 (server-data.h: ContextualClass) ──

thread_local! {
  static LANGUAGE_SERVER_DATA: RefCell<LanguageServerData> =
    RefCell::new(LanguageServerData::default());
}

/// 原版 LanguageServerData::Get() 可变借用 (AddDefinition/AddSymbol)
pub fn with_language_server_data<R>(f: impl FnOnce(&mut LanguageServerData) -> R) -> R {
  LANGUAGE_SERVER_DATA.with_borrow_mut(f)
}

/// 换入/换出 LanguageServerData 实例
pub fn replace_language_server_data(data: LanguageServerData) -> LanguageServerData {
  LANGUAGE_SERVER_DATA.with_borrow_mut(|slot| replace(slot, data))
}

// ── BuildFlags (torque-parser.cc:39-109, @if/@ifnot 条件编译) ──
// 值对齐上游默认 gn 配置 (x64, sandbox/ptr-compr/intl/webassembly 开, tsan/drumbrake 关)。

/// 原版 BuildFlags::GetFlag(name, production); 未知 flag → ReportError
pub fn build_flag(name: &str, production: &str) -> Result<bool> {
  match name {
    "V8_EXTERNAL_CODE_SPACE" => Ok(true),
    "TAGGED_SIZE_8_BYTES" => Ok(tagged_size() == 8),
    "V8_ENABLE_UNDEFINED_DOUBLE" => Ok(false),
    "V8_ENABLE_EXPERIMENTAL_TSA_BUILTINS" => Ok(false),
    "V8_ENABLE_EXPERIMENTAL_TSA_BUILTINS_WITHOUT_TQ_TO_TSA" => Ok(false),
    "V8_INTL_SUPPORT" => Ok(true),
    "V8_ENABLE_SWISS_NAME_DICTIONARY" => Ok(true),
    "V8_ENABLE_JAVASCRIPT_PROMISE_HOOKS" => Ok(true),
    "V8_ENABLE_CONTINUATION_PRESERVED_EMBEDDER_DATA" => Ok(true),
    "TRUE_FOR_TESTING" => Ok(true),
    "FALSE_FOR_TESTING" => Ok(false),
    "V8_SCRIPTORMODULE_LEGACY_LIFETIME" => Ok(false),
    "V8_ENABLE_WEBASSEMBLY" => Ok(true),
    "WASM_CODE_POINTER_NEEDS_PADDING" => Ok(tagged_size() == 8),
    "V8_ENABLE_SANDBOX" => Ok(true),
    "DEBUG" => Ok(cfg!(debug_assertions)),
    "V8_ENABLE_DRUMBRAKE" => Ok(false),
    "V8_ENABLE_SEEDED_ARRAY_INDEX_HASH" => Ok(false),
    "V8_IS_TSAN" => Ok(false),
    _ => Err(report_error(format!(
      "Unknown flag used in {production}: {name}. Please add it to the list in BuildFlags."
    ))),
  }
}
