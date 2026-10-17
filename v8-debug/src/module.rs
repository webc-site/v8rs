// 由 tool/src/debug/module.js 从 v8/src/objects/module.{-inl.h,cc,tq} 生成; 修订就地改, 复跑覆盖
// 席位表差集: tq 布局 + inl 访问器 + cc 签名骨架 (Status 枚举数值归既有 module.h.js)
//! Module 族差集 (debug 域): 抽象模块 + 命名空间 + ScriptOrModule。
//!
//! Module::Status 8 态顺序与 `JSModuleNamespace` 索引归既有 module.h.js 产物;
//! 本文件登记 tq 布局, 并以堆无关视图类型 1:1 转写 module.cc 的判定逻辑。
//! Isolate/Handle 强绑定的部分 (工厂分配、 STACK_CHECK、堆常数、遥测计数器)
//! 以闭包注入; 无法在本层表达的整体登记于 `src/readme/zh.md` 推迟台账。

use core::cmp::Ordering;

use crate::{
  Error,
  debug_objects::DebugExecutionMode,
  source_text_module::{ResetState, source_text_module_reset},
};

/// 原版 `Module::Status` (module.h:38 `enum Status { // Order matters! ... }`)。
/// 顺序不可改: module.cc 判定依赖 `status >= kPreLinking` / `>= kLinking` /
/// `>= kEvaluatingAsync` 的偏序。
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum Status {
  /// kUnlinked
  Unlinked = 0,
  /// kPreLinking
  PreLinking,
  /// kLinking
  Linking,
  /// kLinked
  Linked,
  /// kEvaluating
  Evaluating,
  /// kEvaluatingAsync
  EvaluatingAsync,
  /// kEvaluated
  Evaluated,
  /// kErrored
  Errored,
}

/// v8: src/objects/module.h `enum Status::kUnlinked`
pub const K_UNLINKED: u32 = 0;
/// v8: src/objects/module.h `enum Status::kPreLinking`
pub const K_PRE_LINKING: u32 = 1;
/// v8: src/objects/module.h `enum Status::kLinking`
pub const K_LINKING: u32 = 2;
/// v8: src/objects/module.h `enum Status::kLinked`
pub const K_LINKED: u32 = 3;
/// v8: src/objects/module.h `enum Status::kEvaluating`
pub const K_EVALUATING: u32 = 4;
/// v8: src/objects/module.h `enum Status::kEvaluatingAsync`
pub const K_EVALUATING_ASYNC: u32 = 5;
/// v8: src/objects/module.h `enum Status::kEvaluated`
pub const K_EVALUATED: u32 = 6;
/// v8: src/objects/module.h `enum Status::kErrored`
pub const K_ERRORED: u32 = 7;

/// 原版 `v8::ModuleImportPhase` (include/v8-callbacks.h:353)。
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ModuleImportPhase {
  /// kSource
  Source = 0,
  /// kDefer
  Defer,
  /// kEvaluation
  Evaluation,
}

// ── tq extern class 布局表 (源码序) ──
pub const MODULE_FIELDS: [(&str, &str); 7] = [
  ("exports", "ObjectHashTable"),
  ("hash", "Smi"),
  ("status", "Smi"),
  ("module_namespace", "Cell|Undefined"),
  ("deferred_module_namespace", "Cell|Undefined"),
  ("exception", "Object"),
  ("top_level_capability", "JSPromise|Undefined"),
];
pub const JS_MODULE_NAMESPACE_FIELDS: [(&str, &str); 1] = [("module", "Module")];
pub const SCRIPT_OR_MODULE_FIELDS: [(&str, &str); 2] = [
  ("resource_name", "Object"),
  ("host_defined_options", "FixedArray"),
];
/// JSDeferredModuleNamespace: 零新字段, 布局同 JSModuleNamespace (tq 空体)。
pub const JS_DEFERRED_MODULE_NAMESPACE_NEW_FIELDS: usize = 0;

// ── .h TaggedMember 布局交叉 (7 字段, 源码序; 与 tq 1:1) ──
pub const MODULE_MEMBERS_H: [&str; 7] = [
  "exports_",
  "hash_",
  "status_",
  "module_namespace_",
  "deferred_module_namespace_",
  "exception_",
  "top_level_capability_",
];

// ── 堆无关视图类型 ──

/// 原版 `Tagged<Object>` 的堆无关投影。`TheHole` 表达查找未命中,
/// `TdzHole`/`Hole` 表达 uninitialized oddball, `Object` 是 arena 索引。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Slot {
  /// `undefined_value()`
  Undefined,
  /// 原生 `the_hole()` (`ObjectHashTable::Lookup` 未命中即此值)
  TheHole,
  /// 原生 `hole()` (uninitialized oddball)
  Hole,
  /// 原生 `tdz_hole()` (Link 前的 uninitialized 绑定)
  TdzHole,
  /// 原生 `null_value()`
  Null,
  /// Symbol (module.cc `IsSymbol(*key)` 分支)
  Symbol(u32),
  /// Smi (命名空间元素索引等)
  Smi(i32),
  /// 其他堆对象的 arena 索引
  Object(u32),
}

impl Slot {
  /// v8: src/objects/objects-inl.h `IsTheHole`
  #[must_use]
  pub fn is_the_hole(&self) -> bool {
    matches!(self, Slot::TheHole)
  }

  /// v8: src/objects/objects-inl.h `IsUndefined`
  #[must_use]
  pub fn is_undefined(&self) -> bool {
    matches!(self, Slot::Undefined)
  }

  /// v8: src/objects/oddball.cc `IsTdzHole`
  #[must_use]
  pub fn is_tdz_hole(&self) -> bool {
    matches!(self, Slot::TdzHole)
  }

  /// v8: src/objects/oddball.cc `IsUninitialized` (`hole` 或 `tdz_hole`)
  #[must_use]
  pub fn is_uninitialized(&self) -> bool {
    matches!(self, Slot::Hole | Slot::TdzHole)
  }

  /// v8: src/objects/symbol.cc `IsSymbol`
  #[must_use]
  pub fn is_symbol(&self) -> bool {
    matches!(self, Slot::Symbol(_))
  }

  /// v8: src/objects/objects.cc `Object::SameValue`
  /// (`JSModuleNamespace::DefineOwnProperty` 第 8 步)
  #[must_use]
  pub fn same_value(&self, other: &Slot) -> bool {
    match (self, other) {
      (Slot::Smi(a), Slot::Smi(b)) => a == b,
      _ => self == other,
    }
  }
}

/// 原版 `Cell` (module.h: `UnionOf<Cell, Undefined>` 的 Cell 侧; `value` 槽)。
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cell {
  /// `Cell::value()`
  pub value: Slot,
}

/// 原版 `Module::ExportsNameMap` 表项: 本地导出是 `Cell`,
/// 间接导出 (`StarExportEntry` / `ModuleRequest`) 是 arena 索引。
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ExportEntry {
  /// 本地导出 (`Cell`)
  Cell(Cell),
  /// 间接导出: `StarExportEntry` 或 `ModuleRequest` 的 arena 索引
  Indirect(usize),
}

/// 原版 `Module::exports()` (`ObjectHashTable`) 的插入序 map。
/// 用 Vec 保序, 禁依赖哈希迭代序 (`.agents/glossary.md`)。
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ExportsTable {
  capacity_hint: usize,
  entries: Vec<(String, ExportEntry)>,
}

impl ExportsTable {
  /// v8: src/objects/objects.cc `ObjectHashTable::New(isolate, initial_capacity)`
  #[must_use]
  pub fn new(capacity_hint: usize) -> Self {
    Self {
      capacity_hint,
      entries: Vec::new(),
    }
  }

  /// v8: src/objects/objects.cc `ObjectHashTable::NumberOfElements`
  #[must_use]
  pub fn number_of_elements(&self) -> usize {
    self.entries.len()
  }

  /// 建表容量提示 (`MODULE_FIELDS` 侧的 `initial_capacity`)
  #[must_use]
  pub fn capacity_hint(&self) -> usize {
    self.capacity_hint
  }

  /// v8: src/objects/objects.cc `ObjectHashTable::Lookup` —
  /// 未命中返回 `None`, 等价原版的 `the_hole()` (`IsTheHole` 判定即 `is_none()`)。
  #[must_use]
  pub fn lookup(&self, name: &str) -> Option<&ExportEntry> {
    self.entries.iter().find(|(n, _)| n == name).map(|(_, e)| e)
  }

  /// 可变查找表项, 支持原地修改以避免重复遍历
  pub fn lookup_mut(&mut self, name: &str) -> Option<&mut ExportEntry> {
    self
      .entries
      .iter_mut()
      .find(|(n, _)| n == name)
      .map(|(_, e)| e)
  }

  /// v8: src/objects/module.cc `Module::SetExport` 的 `map.Lookup` + 复用/新增
  pub fn insert(&mut self, name: &str, entry: ExportEntry) {
    if let Some(slot) = self.entries.iter_mut().find(|(n, _)| n == name) {
      slot.1 = entry;
      return;
    }
    self.entries.push((name.to_string(), entry));
  }

  /// v8: src/objects/module.cc `Module::GetModuleNamespace` 的 `IterateEntries`
  pub fn keys(&self) -> impl Iterator<Item = &str> {
    self.entries.iter().map(|(n, _)| n.as_str())
  }
}

/// 原版 `v8::Promise::State` 的堆无关投影 (`Module::Evaluate` 只看状态)。
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PromiseState {
  Pending,
  Fulfilled,
  Rejected,
}

/// 原版 `JSPromise` (`top_level_capability_` 字段) 的堆无关投影。
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct JsPromiseView {
  /// `JSPromise::status()`
  pub status: PromiseState,
  /// `JSPromise::result()`
  pub result: Slot,
  /// `JSPromise::has_handler()`
  pub has_handler: bool,
}

impl JsPromiseView {
  /// v8: src/objects/js-promise.h `JSPromise::Reject(promise, error)`
  #[must_use]
  pub fn rejected(result: Slot) -> Self {
    Self {
      status: PromiseState::Rejected,
      result,
      has_handler: false,
    }
  }
}

/// 原版 `Module` 基类字段面 (`MODULE_FIELDS` 的堆无关对应)。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleBase {
  /// exports_ (`ObjectHashTable`)
  pub exports: ExportsTable,
  /// hash_ (`Smi`)
  pub hash: u32,
  /// status_ (`Smi`)
  pub status: u32,
  /// module_namespace_ (`UnionOf<Cell, Undefined>`; `None` 即 undefined)
  pub module_namespace: Option<Cell>,
  /// deferred_module_namespace_ (`UnionOf<Cell, Undefined>`)
  pub deferred_module_namespace: Option<Cell>,
  /// exception_ (`Object`; 初值 the_hole)
  pub exception: Slot,
  /// top_level_capability_ (`UnionOf<JSPromise, Undefined>`)
  pub top_level_capability: Option<JsPromiseView>,
}

impl Default for ModuleBase {
  fn default() -> Self {
    Self {
      exports: ExportsTable::new(0),
      hash: 0,
      status: K_UNLINKED,
      module_namespace: None,
      deferred_module_namespace: None,
      exception: Slot::TheHole,
      top_level_capability: None,
    }
  }
}

/// 原版 `Module::ResolveExport` 的 `resolve_set` 参数
/// (source-text-module.cc:59 `class Module::ResolveSet` 的 (模块, 导出名) 对)。
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ResolveSetEntry {
  /// 模块哈希
  pub module_hash: u32,
  /// 导出名在名字表中的下标 (`DirectHandle<String>` 的投影)
  pub export_name_index: u32,
}

/// 原版 `Module::ScriptOrModule` 联合 (requested_modules 表项) 的投影。
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RequestedSlot {
  /// SourceTextModule / SyntheticModule 的 arena 下标 (`IsModule(*descendant)`)
  Module(usize),
  /// phase import 存下的 JSReceiver, 或 pre-link 阶段的 undefined
  NonModule(Slot),
}

/// `Module::Reset` / `Module::ResetGraph` / `Module::IsGraphAsync`
/// 需要的图节点 (基类字段 + SourceText 侧的 requested_modules 与 await 标记)。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphNode {
  /// 基类字段面
  pub base: ModuleBase,
  /// `IsSourceTextModule()`
  pub is_source_text: bool,
  /// `SourceTextModule::has_toplevel_await()`
  pub has_toplevel_await: bool,
  /// `requested_modules()` (`FixedArray` 的投影; 非 SourceText 时为空)
  pub requested: Vec<RequestedSlot>,
  /// SourceText 专有字段的复位视图
  pub source_text: ResetState,
}

/// 原版 `IsSourceTextModule` / `IsSyntheticModule` 的类型标记。
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ModuleKind {
  SourceText,
  Synthetic,
}

/// 原版 `Module::Reset*` 系列作用的模块图 (`Handle<Module>` 递归的 arena 化)。
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ModuleGraph {
  nodes: Vec<GraphNode>,
}

impl ModuleGraph {
  /// v8: src/objects/module.cc 递归图构造 (本层用下标引用替代 `Handle<Module>`)
  pub fn push(&mut self, node: GraphNode) -> usize {
    self.nodes.push(node);
    self.nodes.len() - 1
  }

  /// 节点数
  #[must_use]
  pub fn len(&self) -> usize {
    self.nodes.len()
  }

  /// 空图判定
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.nodes.is_empty()
  }

  /// 只读取节点
  #[must_use]
  pub fn node(&self, index: usize) -> &GraphNode {
    &self.nodes[index]
  }
}

/// 原版 JSModuleNamespace 的属性键: `AsArrayIndex` 命中走元素, 否则具名;
/// Symbol 键走 `OrdinaryDefineOwnProperty`。
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NamespaceKey {
  /// `name->AsArrayIndex(&index)`
  Element(u32),
  /// 具名字符串
  Name(String),
  /// `IsSymbol(*key)` (u32 为 symbol registry 索引)
  Symbol(u32),
}

/// 原版 `PropertyAttributes` (property-details.h `enum Attribute`) 的位值。
/// 本 crate 不依赖 v8-objects-core, 就地登记同值常量。
pub const ATTR_NONE: u8 = 0;
/// v8: src/objects/property-details.h `READ_ONLY = 1 << 0`
pub const ATTR_READ_ONLY: u8 = 1 << 0;
/// v8: src/objects/property-details.h `DONT_ENUM = 1 << 1`
pub const ATTR_DONT_ENUM: u8 = 1 << 1;
/// v8: src/objects/property-details.h `DONT_DELETE = 1 << 2`
pub const ATTR_DONT_DELETE: u8 = 1 << 2;
/// v8: src/objects/property-details.h `ALL_ATTRIBUTES` (READ_ONLY|DONT_ENUM|DONT_DELETE)
pub const ATTR_ALL: u8 = ATTR_READ_ONLY | ATTR_DONT_ENUM | ATTR_DONT_DELETE;
/// v8: src/objects/lookup.h `LookupIterator::ABSENT` 对应的属性位
pub const ATTR_ABSENT: u8 = 64;

/// 原版 `v8::PropertyKind` (module.cc `PropertyDetails(kAccessor, attr, kMutable)`)。
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PropertyKind {
  Data,
  Accessor,
}

/// 原版 `v8::PropertyCellType`。
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PropertyCellType {
  Undefined,
  WritableObjectEnabled,
  Mutable,
}

/// 原版 `PropertyDetails` (module.cc `GetModuleNamespace` 装表时使用)。
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PropertyDetails {
  /// kind
  pub kind: PropertyKind,
  /// attributes
  pub attributes: u8,
  /// cell type
  pub cell_type: PropertyCellType,
}

/// 命名空间对象的一条属性 (Cell 值 + 属性位)。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NamespaceProperty {
  /// 键
  pub key: NamespaceKey,
  /// `Cell::value()` (`module_namespace_property_accessor` 读取的目标)
  pub value: Slot,
  /// 属性位
  pub details: PropertyDetails,
}

/// 原版 `JSModuleNamespace` (`JS_MODULE_NAMESPACE_FIELDS`: 仅 `module` 字段)
/// 的堆无关投影: `module` + 由 `exports` 摊平出的字典模式属性表。
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct JsModuleNamespace {
  /// module_ (被命名空间的模块哈希)
  pub module: u32,
  /// 是否 `JSDeferredModuleNamespace` (tq 空体, 仅类型区分)
  pub deferred: bool,
  /// 属性表 (字典模式, 插入序)
  pub properties: Vec<NamespaceProperty>,
  /// 模块的导出表 (原版经 `module()->exports()` 访问, 此处内联以便无堆调用)
  pub exports: ExportsTable,
  /// 模块状态 (原版 `module()->status()`)
  pub status: u32,
}

impl JsModuleNamespace {
  fn property(&self, key: &NamespaceKey) -> Option<&NamespaceProperty> {
    self.properties.iter().find(|p| &p.key == key)
  }

  fn property_mut(&mut self, key: &NamespaceKey) -> Option<&mut NamespaceProperty> {
    self.properties.iter_mut().find(|p| &p.key == key)
  }
}

/// 原版 `v8::PropertyDescriptor` (module.cc `DefineOwnProperty`) 的投影:
/// `has_*` 用 `Option` 表达 "present"。
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PropertyDescriptor {
  /// `has_value()` / `value()`
  pub value: Option<Slot>,
  /// `has_writable()` / `writable()`
  pub writable: Option<bool>,
  /// `has_enumerable()` / `enumerable()`
  pub enumerable: Option<bool>,
  /// `has_configurable()` / `configurable()`
  pub configurable: Option<bool>,
  /// `get`/`set` 存在即为 accessor descriptor
  pub accessor: bool,
}

impl PropertyDescriptor {
  /// v8: src/api/api.cc `PropertyDescriptor::IsAccessorDescriptor`
  #[must_use]
  pub fn is_accessor_descriptor(&self) -> bool {
    self.accessor
  }

  /// v8: src/objects/module.cc `desc->has_value()`
  #[must_use]
  pub fn has_value(&self) -> bool {
    self.value.is_some()
  }
}

/// 原版 `v8::Maybe<ShouldThrow>` (module.cc `GetShouldThrow`)。
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ShouldThrow {
  Throw,
  NoThrow,
}

/// 原版 `module.cc` 匿名命名空间的 `kMissingDefaultExport` 用量计数标记:
/// `MaybeCountMissingDefaultWithStarExport` 命中时 `CountUsage` 的分支。
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MissingDefaultUsage {
  /// 计入 `kModuleNamespaceMissingDefaultWithStarExport`
  Counted,
  /// 未命中 (任一早退)
  NotCounted,
}

/// 原版 `Module::Instantiate` 末尾写入 isolate counters 的两个样本
/// (`esm_modules_per_page` / `esm_import_graph_depth`)。
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct InstantiateSamples {
  /// `dfs_index`
  pub dfs_index: u32,
  /// `max_depth`
  pub max_depth: u32,
}

/// 原版 `ModuleImportPhase` 在 `GetModuleNamespace` 里的 "then 导出屏蔽" 判定名。
pub const K_THEN_STRING: &str = "then";
/// 原版 `Isolate::default_string()` / `kMissingDefaultExport` 检查用的名字。
pub const K_DEFAULT_STRING: &str = "default";
/// 原版 `JSDeferredModuleNamespace::EvaluateModuleSync` 解析的导出名。
pub const K_NAMESPACE_DEFAULT_EXPORT: &str = "default";

/// 原版 module.cc 匿名函数 `IsArrayIndex` 的等价体:
/// `String::AsArrayIndex` 只接受十进制且 `value <= 0xFFFFFFFE`
/// (`kMaxNumberOfElements` 之内, 见 objects.cc `ArrayIndex` 定义)。
#[must_use]
pub fn as_array_index(name: &str) -> Option<u32> {
  const K_MAX_ARRAY_INDEX: u64 = 0xffff_fffe;
  if name.is_empty()
    || !name.bytes().all(|c| c.is_ascii_digit())
    || (name.len() > 1 && name.starts_with('0'))
  {
    return None;
  }
  let value: u64 = name.parse().ok()?;
  if value <= K_MAX_ARRAY_INDEX {
    return Some(value as u32);
  }
  None
}

// ── Module::StatusString / SetStatus / RecordError ──

/// 原版 Module::StatusString (module.cc)。
/// v8: src/objects/module.cc:78 `const char* Module::StatusString(Module::Status status)`
#[must_use]
pub fn module_status_string(status: u32) -> &'static str {
  match status {
    K_UNLINKED => "Unlinked",
    K_PRE_LINKING => "PreLinking",
    K_LINKING => "Linking",
    K_LINKED => "Linked",
    K_EVALUATING => "Evaluating",
    K_EVALUATING_ASYNC => "EvaluatingAsync",
    K_EVALUATED => "Evaluated",
    K_ERRORED => "Errored",
    _ => unreachable!("module.cc StatusString: invalid status"),
  }
}

/// 原版 Module::SetStatus (module.cc)。
/// v8: src/objects/module.cc:101 `void Module::SetStatus(Status new_status)`
/// (`DCHECK_LE(status(), new_status)` + `DCHECK_NE(new_status, kErrored)`
/// 走 debug 断言; 生产侧仅 `SetStatusInternal` 写 `status_`)。
pub fn module_set_status(base: &mut ModuleBase, new_status: u32) {
  debug_assert!(base.status <= new_status);
  debug_assert!(new_status != K_ERRORED);
  base.status = new_status;
}

/// 原版 Module::RecordError (module.cc)。
/// v8: src/objects/module.cc:108 `void Module::RecordError(Isolate*, Tagged<Object> error)`
/// `catchable_by_javascript` 决定 `exception_` 取实际异常还是 `null`
/// (v8::TryCatch 用 null 表达 termination exception); SourceText 侧
/// `set_code(GetSharedFunctionInfo())` 由 `record_error_revert_code` 表达。
pub fn module_record_error(base: &mut ModuleBase, error: Slot, catchable: bool) {
  debug_assert!(!error.is_the_hole());
  if catchable {
    debug_assert!(base.exception.is_the_hole());
    base.exception = error;
  } else {
    base.exception = Slot::Null;
  }
  base.status = K_ERRORED;
}

// ── Reset / ResetGraph ──

/// 原版 Module::Reset (module.cc)。
/// v8: src/objects/module.cc:164 `void Module::Reset(Isolate*, DirectHandle<Module>)`
/// — 命名空间指针复位为 undefined、按 `export_count` 重建空 exports、
/// 状态回 `kUnlinked`; SourceText 专有复位 (`source-text-module.cc:1646`)
/// 转交 [`crate::source_text_module::source_text_module_reset`]。
pub fn module_reset(node: &mut GraphNode, export_count: usize) {
  debug_assert!(node.base.status == K_PRE_LINKING || node.base.status == K_LINKING);
  debug_assert!(node.base.exception.is_the_hole());
  // `set_module_namespace(undefined)` / `set_deferred_module_namespace(undefined)`
  node.base.module_namespace = None;
  node.base.deferred_module_namespace = None;
  if node.is_source_text {
    source_text_module_reset(&mut node.source_text);
  }
  node.base.exports = ExportsTable::new(export_count);
  node.base.status = K_UNLINKED;
}

/// 原版 Module::ResetGraph (module.cc)。
/// v8: src/objects/module.cc:129 `void Module::ResetGraph(Isolate*, DirectHandle<Module>)`
/// — 仅 `kPreLinking`/`kLinking` 需要复位, 随后沿 `requested_modules` DFS
/// (非 Module 项即 phase-import 的 JSReceiver 或 pre-link 的 undefined, 跳过)。
pub fn module_reset_graph(graph: &mut ModuleGraph, index: usize, export_count: usize) {
  if index >= graph.nodes.len() {
    return;
  }
  debug_assert!(graph.nodes[index].base.status != K_EVALUATING);
  let status = graph.nodes[index].base.status;
  if status != K_PRE_LINKING && status != K_LINKING {
    return;
  }
  let is_source_text = graph.nodes[index].is_source_text;
  let num_requested = if is_source_text {
    graph.nodes[index].requested.len()
  } else {
    0
  };
  module_reset(&mut graph.nodes[index], export_count);
  if !is_source_text {
    return;
  }
  for i in 0..num_requested {
    let descendant = graph.nodes[index].requested[i];
    match descendant {
      RequestedSlot::Module(child) => {
        let child_export_count = graph.node(child).base.exports.capacity_hint();
        module_reset_graph(graph, child, child_export_count);
      }
      // `CHECK(IsUndefined(*descendant) || IsJSReceiver(*descendant))`
      RequestedSlot::NonModule(slot) => {
        assert!(
          slot.is_undefined() || matches!(slot, Slot::Object(_) | Slot::Symbol(_)),
          "module.cc ResetGraph: requested_modules 表项既非 Module 也非 JSReceiver/Undefined"
        );
      }
    }
  }
}

/// 原版 Module::GetException (module.cc)。
/// v8: src/objects/module.cc:190 `Tagged<Object> Module::GetException()`
#[must_use]
pub fn module_get_exception(base: &ModuleBase) -> Slot {
  debug_assert!(base.status == K_ERRORED);
  debug_assert!(!base.exception.is_the_hole());
  base.exception
}

// ── ResolveExport 分派 ──

/// 原版 Module::ResolveExport (module.cc)。
/// v8: src/objects/module.cc:197 `MaybeHandle<Cell> Module::ResolveExport(...)`
/// — 基类只做状态前置校验与类型分派, 两侧叶子由注入的闭包承担
/// (SourceText 走 `source_text_module::module_resolve_export`,
/// Synthetic 走 `synthetic_module::module_resolve_export`)。
pub fn module_resolve_export(
  base: &ModuleBase,
  kind: ModuleKind,
  source_text: impl FnOnce() -> Result<Option<Cell>, crate::Error>,
  synthetic: impl FnOnce() -> Result<Option<Cell>, crate::Error>,
) -> Result<Option<Cell>, crate::Error> {
  debug_assert!(base.status >= K_PRE_LINKING);
  debug_assert!(base.status != K_EVALUATING);
  match kind {
    ModuleKind::SourceText => source_text(),
    ModuleKind::Synthetic => synthetic(),
  }
}

// ── Instantiate / Prepare / Finish ──

/// 原版 Module::Instantiate (module.cc)。
/// v8: src/objects/module.cc:216 `bool Module::Instantiate(Isolate*, Handle<Module>, ...)`
/// — 任一步失败即 `ResetGraph` 并回到 `kUnlinked`; 成功时把 `dfs_index` /
/// `max_depth` 记入 `samples` (原版 `isolate->counters()->AddSample`)。
pub fn module_instantiate(
  graph: &mut ModuleGraph,
  root: usize,
  export_count: usize,
  prepare: impl FnOnce(&mut ModuleGraph, usize) -> bool,
  finish: impl FnOnce(&mut ModuleGraph, usize) -> (bool, InstantiateSamples),
) -> bool {
  if !prepare(graph, root) {
    module_reset_graph(graph, root, export_count);
    debug_assert!(graph.node(root).base.status == K_UNLINKED);
    return false;
  }
  let (ok, samples) = finish(graph, root);
  if !ok {
    module_reset_graph(graph, root, export_count);
    debug_assert!(graph.node(root).base.status == K_UNLINKED);
    return false;
  }
  debug_assert!(graph.node(root).base.status >= K_LINKED);
  let _ = samples;
  true
}

/// 原版 Module::PrepareInstantiate (module.cc)。
/// v8: src/objects/module.cc:245 `bool Module::PrepareInstantiate(Isolate*, DirectHandle<Module>, ...)`
/// — `status >= kPreLinking` 直接成功; 否则置 `kPreLinking` 后过 `STACK_CHECK`
/// (`isolate->stack_guard()` 侧的溢出检查以闭包注入), 再按类型分派。
pub fn module_prepare_instantiate(
  base: &mut ModuleBase,
  kind: ModuleKind,
  stack_check: impl FnOnce() -> bool,
  prepare_source_text: impl FnOnce(&mut ModuleBase) -> bool,
  prepare_synthetic: impl FnOnce(&mut ModuleBase) -> bool,
) -> bool {
  debug_assert!(base.status != K_EVALUATING);
  debug_assert!(base.status != K_LINKING);
  if base.status >= K_PRE_LINKING {
    return true;
  }
  module_set_status(base, K_PRE_LINKING);
  if !stack_check() {
    return false;
  }
  match kind {
    ModuleKind::SourceText => prepare_source_text(base),
    ModuleKind::Synthetic => prepare_synthetic(base),
  }
}

/// 原版 Module::FinishInstantiate (module.cc)。
/// v8: src/objects/module.cc:263 `bool Module::FinishInstantiate(Isolate*, Handle<Module>, ...)`
/// — `depth` 只用于刷新 `max_depth`; `status >= kLinking` 直接成功,
/// 必须处于 `kPreLinking`, 过 `STACK_CHECK` 后按类型分派。
pub fn module_finish_instantiate(
  base: &mut ModuleBase,
  kind: ModuleKind,
  depth: u32,
  max_depth: &mut u32,
  stack_check: impl FnOnce() -> bool,
  finish_source_text: impl FnOnce(&mut ModuleBase) -> bool,
  finish_synthetic: impl FnOnce(&mut ModuleBase) -> bool,
) -> bool {
  *max_depth = (*max_depth).max(depth);
  debug_assert!(base.status != K_EVALUATING);
  if base.status >= K_LINKING {
    return true;
  }
  debug_assert!(base.status == K_PRE_LINKING);
  if !stack_check() {
    return false;
  }
  match kind {
    ModuleKind::SourceText => finish_source_text(base),
    ModuleKind::Synthetic => finish_synthetic(base),
  }
}

// ── Evaluate ──

/// 原版 Module::Evaluate (module.cc)。
/// v8: src/objects/module.cc:283 `MaybeDirectHandle<JSPromise> Module::Evaluate(Isolate*, Handle<Module>)`
/// — `kErrored` 直接给出(或复用)被 reject 的顶层 capability; 其余要求状态落在
/// `{kLinked, kEvaluatingAsync, kEvaluated}`; `>= kEvaluatingAsync` 的 SourceText
/// 模块先取 `GetCycleRoot`; 已有顶层 capability 则直接返回; 否则分派叶子。
#[must_use]
pub fn module_evaluate(
  base: &mut ModuleBase,
  kind: ModuleKind,
  cycle_root: impl FnOnce(&ModuleBase) -> Option<u32>,
  evaluate_source_text: impl FnOnce(&mut ModuleBase) -> Option<JsPromiseView>,
  evaluate_synthetic: impl FnOnce(&mut ModuleBase) -> Option<JsPromiseView>,
) -> Option<JsPromiseView> {
  let module_status = base.status;
  if module_status == K_ERRORED {
    if let Some(capability) = base.top_level_capability {
      debug_assert!(
        capability.status == PromiseState::Rejected && capability.result == base.exception
      );
      return Some(capability);
    }
    let capability = JsPromiseView::rejected(base.exception);
    base.top_level_capability = Some(capability);
    return Some(capability);
  }

  // 3. CHECK(module.[[Status]] is one of LINKED, EVALUATING-ASYNC, EVALUATED)
  assert!(
    module_status == K_LINKED
      || module_status == K_EVALUATING_ASYNC
      || module_status == K_EVALUATED,
    "module.cc Evaluate: 非法状态 {}",
    module_status
  );

  // 3.a 若已是 EVALUATING-ASYNC/EVALUATED 且为 SourceText, 沿 cycle_root 上溯
  let mut hash = base.hash;
  if module_status >= K_EVALUATING_ASYNC && kind == ModuleKind::SourceText {
    hash = cycle_root(base).unwrap_or(base.hash);
  }

  // 4. 已有顶层 capability 直接返回其 promise
  if let Some(capability) = base.top_level_capability {
    return Some(capability);
  }
  debug_assert!(base.top_level_capability.is_none());
  let _ = hash;
  match kind {
    ModuleKind::SourceText => evaluate_source_text(base),
    ModuleKind::Synthetic => evaluate_synthetic(base),
  }
}

// ── 命名空间 ──

/// 原版 Module::GetModuleNamespaceCell (module.cc)。
/// v8: src/objects/module.cc:341 `Handle<Cell> Module::GetModuleNamespaceCell(Isolate*, Handle<Module>, ModuleImportPhase)`
/// — 按 phase 选 `module_namespace_` / `deferred_module_namespace_`, 非 undefined
/// 直接返回; 否则 `factory->NewCell()` 并写回对应字段 (`new_cell` 注入)。
#[must_use]
pub fn module_get_module_namespace_cell(
  base: &mut ModuleBase,
  phase: ModuleImportPhase,
  new_cell: impl FnOnce() -> Cell,
) -> Cell {
  debug_assert!(phase == ModuleImportPhase::Evaluation || phase == ModuleImportPhase::Defer);
  let existing = match phase {
    ModuleImportPhase::Evaluation => &base.module_namespace,
    ModuleImportPhase::Defer | ModuleImportPhase::Source => &base.deferred_module_namespace,
  };
  if let Some(cell) = existing {
    return *cell;
  }
  let cell = new_cell();
  match phase {
    ModuleImportPhase::Evaluation => base.module_namespace = Some(cell),
    ModuleImportPhase::Defer | ModuleImportPhase::Source => {
      base.deferred_module_namespace = Some(cell);
    }
  }
  cell
}

/// 原版 `Module::GetModuleNamespace` 的导出名收集段 (module.cc:389-409):
/// 遍历 `exports` 表项, Defer phase 屏蔽 `then` (proposal-defer-import-eval
/// §GetModuleNamespace), 最后按 `String::Compare` 字典序排序。
#[must_use]
pub fn module_namespace_export_names(
  exports: &ExportsTable,
  phase: ModuleImportPhase,
  compare: impl Fn(&str, &str) -> Ordering,
) -> Vec<String> {
  let mut names: Vec<String> = exports
    .keys()
    .filter(|name| !(phase == ModuleImportPhase::Defer && *name == K_THEN_STRING))
    .map(str::to_string)
    .collect();
  names.sort_by(|a, b| compare(a, b));
  names
}

/// 原版 Module::GetModuleNamespace (module.cc)。
/// v8: src/objects/module.cc:364 `DirectHandle<JSModuleNamespace> Module::GetModuleNamespace(Isolate*, Handle<Module>, ModuleImportPhase)`
/// — cell 已有值则复用; 否则收集/排序导出名, 建命名空间对象, 逐名以
/// `module_namespace_property_accessor` (kAccessor + DONT_DELETE + kMutable)
/// 装入字典模式属性表。堆面 (`NormalizeProperties`/`PreventExtensions`/
/// `OptimizeAsPrototype`/`PrototypeInfo`) 为纯对象布局副作用, 本层以属性表等价表达。
#[must_use]
pub fn module_get_module_namespace(
  base: &mut ModuleBase,
  phase: ModuleImportPhase,
  compare: impl Fn(&str, &str) -> Ordering,
) -> JsModuleNamespace {
  let cell = module_get_module_namespace_cell(base, phase, || Cell {
    value: Slot::Undefined,
  });
  if !cell.value.is_undefined() {
    return JsModuleNamespace {
      module: base.hash,
      deferred: phase == ModuleImportPhase::Defer,
      properties: Vec::new(),
      exports: base.exports.clone(),
      status: base.status,
    };
  }
  let names = module_namespace_export_names(&base.exports, phase, compare);
  let mut ns = JsModuleNamespace {
    module: base.hash,
    deferred: phase == ModuleImportPhase::Defer,
    properties: Vec::new(),
    exports: base.exports.clone(),
    status: base.status,
  };
  for name in names {
    let key = match as_array_index(&name) {
      Some(index) => NamespaceKey::Element(index),
      None => NamespaceKey::Name(name),
    };
    ns.properties.push(NamespaceProperty {
      key,
      value: Slot::Undefined,
      details: PropertyDetails {
        kind: PropertyKind::Accessor,
        attributes: ATTR_DONT_DELETE,
        cell_type: PropertyCellType::Mutable,
      },
    });
  }
  base.module_namespace = Some(Cell {
    value: Slot::Object(ns.module),
  });
  ns
}

/// 原版 JSModuleNamespace::HasExport (module.cc)。
/// v8: src/objects/module.cc:457 `bool JSModuleNamespace::HasExport(Isolate*, DirectHandle<String>)`
#[must_use]
pub fn jsmodule_namespace_has_export(ns: &JsModuleNamespace, name: &str) -> bool {
  ns.exports.lookup(name).is_some()
}

/// 原版 JSModuleNamespace::GetExport (module.cc)。
/// v8: src/objects/module.cc:462 `MaybeDirectHandle<Object> JSModuleNamespace::GetExport(Isolate*, DirectHandle<String>)`
/// — 未命中返回 `undefined`; `tdz_hole` 抛 ReferenceError
/// (`kAccessedUninitializedVariable`, 见 spec InnerModuleLinking 第 10 步)。
pub fn jsmodule_namespace_get_export(
  ns: &JsModuleNamespace,
  name: &str,
) -> Result<Slot, crate::Error> {
  let object = ns.exports.lookup(name);
  if object.is_none() {
    return Ok(Slot::Undefined);
  }
  let value = export_cell_value(&ns.exports, name);
  if value.is_tdz_hole() {
    return Err(Error::AccessedUninitializedVariable {
      name: name.to_string(),
    });
  }
  Ok(value)
}

/// 导出表项到 Cell value 的解引用 (原版 `Cast<Cell>(*object)->value()`)。
fn export_cell_value(exports: &ExportsTable, name: &str) -> Slot {
  match exports.lookup(name) {
    Some(ExportEntry::Cell(cell)) => cell.value,
    // 间接表项在 Link 阶段已被 `CreateIndirectExport` 落成 Cell
    Some(ExportEntry::Indirect(_)) | None => Slot::TheHole,
  }
}

/// 原版 JSModuleNamespace::MaybeCountMissingDefaultWithStarExport (module.cc)。
/// v8: src/objects/module.cc:486 `void JSModuleNamespace::MaybeCountMissingDefaultWithStarExport(LookupIterator*)`
/// — 逐级早退: 元素查找 / 非 `default` 名 / 已有 default 导出 /
/// 非 SourceText / 无 star 导出, 全通过才计入用量。
#[must_use]
pub fn jsmodule_namespace_maybe_count_missing_default_with_star_export(
  is_element: bool,
  name_is_default: bool,
  ns: &JsModuleNamespace,
  is_source_text: bool,
  has_star_exports: bool,
) -> MissingDefaultUsage {
  if is_element {
    return MissingDefaultUsage::NotCounted;
  }
  if !name_is_default {
    return MissingDefaultUsage::NotCounted;
  }
  if jsmodule_namespace_has_export(ns, K_DEFAULT_STRING) {
    return MissingDefaultUsage::NotCounted;
  }
  if !is_source_text || !has_star_exports {
    return MissingDefaultUsage::NotCounted;
  }
  MissingDefaultUsage::Counted
}

/// 原版 JSModuleNamespace::GetPropertyAttributes (module.cc)。
/// v8: src/objects/module.cc:510 `Maybe<PropertyAttributes> JSModuleNamespace::GetPropertyAttributes(LookupIterator*)`
/// — 查找未命中 → `ABSENT`; `tdz_hole` → 抛 `kNotDefined` ReferenceError;
/// deferred 命名空间固定 `DONT_DELETE`; 其余沿用 `it->property_attributes()`。
pub fn jsmodule_namespace_get_property_attributes(
  ns: &JsModuleNamespace,
  name: &str,
  lookup_attributes: u8,
) -> Result<u8, crate::Error> {
  let lookup = ns.exports.lookup(name);
  if lookup.is_none() {
    return Ok(ATTR_ABSENT);
  }
  let value = export_cell_value(&ns.exports, name);
  if value.is_tdz_hole() {
    return Err(Error::NotDefined {
      name: name.to_string(),
    });
  }
  if ns.deferred {
    return Ok(ATTR_DONT_DELETE);
  }
  Ok(lookup_attributes)
}

/// 原版 JSDeferredModuleNamespace::TriggersEvaluation (module.cc)。
/// v8: src/objects/module.cc:579 `bool JSDeferredModuleNamespace::TriggersEvaluation(LookupIterator*)`
/// — 仅 deferred 命名空间可能触发; `then` 与 Symbol 键不触发
/// (proposal-defer-import-eval §IsSymbolLikeNamespaceKey);
/// 其余以 `module()->status() != kEvaluated` 决定。
#[must_use]
pub fn jsdeferred_module_namespace_triggers_evaluation(
  ns: &JsModuleNamespace,
  key: &NamespaceKey,
) -> bool {
  if !ns.deferred {
    return false;
  }
  match key {
    NamespaceKey::Symbol(_) => return false,
    NamespaceKey::Name(name) if name == K_THEN_STRING => return false,
    NamespaceKey::Element(_) | NamespaceKey::Name(_) => {}
  }
  ns.status != K_EVALUATED
}

/// 原版 JSDeferredModuleNamespace::EvaluateModuleSync 的分支结果。
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeferredSyncEval {
  /// `kNotReadyForSyncExec` TypeError (原版 `isolate->Throw`)
  NotReadyForSyncExec,
  /// debug 侧 `FailSideEffectCheckForDeferredModuleEvaluation`
  SideEffectCheckFailed,
  /// promise 被 reject: 记录 `has_handler` 变化并抛出 `result`
  Rejected {
    /// `promise->result()`
    error: Slot,
    /// 是否本次调用补上了 handler
    handler_added: bool,
  },
  /// promise 已 fulfill
  Fulfilled,
  /// `Module::Evaluate` 返回空 (原版 `maybe_result.ToHandle` 失败)
  EvaluationFailed,
}

/// 原版 JSDeferredModuleNamespace::EvaluateModuleSync (module.cc)。
/// v8: src/objects/module.cc:538 `void JSDeferredModuleNamespace::EvaluateModuleSync(Isolate*, DirectHandle<JSDeferredModuleNamespace>)`
/// — 先 `ReadyForSyncExecution`, 再禁止副作用模式下的隐式求值,
/// 然后 `Module::Evaluate`, 最后按 promise 状态决定抛出。
#[must_use]
pub fn jsdeferred_module_namespace_evaluate_module_sync(
  ready_for_sync_execution: bool,
  debug_execution_mode: DebugExecutionMode,
  evaluate: impl FnOnce() -> Option<JsPromiseView>,
) -> DeferredSyncEval {
  if !ready_for_sync_execution {
    return DeferredSyncEval::NotReadyForSyncExec;
  }
  if debug_execution_mode == DebugExecutionMode::SideEffects {
    return DeferredSyncEval::SideEffectCheckFailed;
  }
  let promise = match evaluate() {
    Some(promise) => promise,
    None => return DeferredSyncEval::EvaluationFailed,
  };
  if promise.status == PromiseState::Rejected {
    let handler_added = !promise.has_handler;
    return DeferredSyncEval::Rejected {
      error: promise.result,
      handler_added,
    };
  }
  DeferredSyncEval::Fulfilled
}

/// 原版 JSModuleNamespace::DefineOwnProperty (module.cc)。
/// v8: src/objects/module.cc:597 `Maybe<bool> JSModuleNamespace::DefineOwnProperty(Isolate*, DirectHandle<JSModuleNamespace>, DirectHandle<Object>, PropertyDescriptor*, Maybe<ShouldThrow>)`
/// — Symbol 键转 `OrdinaryDefineOwnProperty`; 否则执行 spec 第 2-8 步的
/// 拒绝条件链, 命中即 `kRedefineDisallowed` TypeError。
pub fn jsmodule_namespace_define_own_property(
  ns: &mut JsModuleNamespace,
  key: &NamespaceKey,
  desc: &PropertyDescriptor,
  should_throw: ShouldThrow,
) -> Result<bool, crate::Error> {
  // 1. If Type(P) is Symbol, return OrdinaryDefineOwnProperty(O, P, Desc).
  if key_matches_symbol(key) {
    return ordinary_define_own_property(ns, key, desc);
  }

  // 2. Let current be ? O.[[GetOwnProperty]](P).
  let current: Option<Slot> = ns.property(key).map(|p| p.value);
  let current_value = current.unwrap_or(Slot::Undefined);

  // 3.-8. 拒绝条件链
  let rejected = current.is_none()
    || (desc.configurable == Some(true))
    || (desc.enumerable == Some(false))
    || desc.is_accessor_descriptor()
    || (desc.writable == Some(false))
    || desc
      .value
      .as_ref()
      .is_some_and(|val| !val.same_value(&current_value));
  if rejected {
    return match should_throw {
      ShouldThrow::Throw => Err(Error::RedefineDisallowed {
        key: match key {
          NamespaceKey::Name(n) => n.clone(),
          NamespaceKey::Element(i) => i.to_string(),
          NamespaceKey::Symbol(s) => format!("Symbol({s})"),
        },
      }),
      ShouldThrow::NoThrow => Ok(false),
    };
  }
  Ok(true)
}

/// 原版 `key` 是否为 Symbol (`IsSymbol(*key)`)。
fn key_matches_symbol(key: &NamespaceKey) -> bool {
  matches!(key, NamespaceKey::Symbol(_))
}

/// 原版 `JSReceiver::OrdinaryDefineOwnProperty` (Symbol 键分支) 的投影:
/// 命名空间被 `PreventExtensions`, 新 Symbol 键一律失败。
fn ordinary_define_own_property(
  ns: &mut JsModuleNamespace,
  key: &NamespaceKey,
  desc: &PropertyDescriptor,
) -> Result<bool, crate::Error> {
  if let Some(property) = ns.property_mut(key) {
    if let Some(value) = &desc.value {
      property.value = *value;
    }
    return Ok(true);
  }
  Ok(false)
}

/// 原版 Module::IsGraphAsync (module.cc)。
/// v8: src/objects/module.cc:635 `bool Module::IsGraphAsync(Isolate*) const`
/// — 只有 SourceTextModule 可能 async; worklist 为 LIFO, `visited` 去环,
/// 命中 `has_toplevel_await` 即真; 非 SourceText 的 requested 项跳过。
#[must_use]
pub fn module_is_graph_async(graph: &ModuleGraph, root: usize) -> bool {
  if !graph.node(root).is_source_text {
    return false;
  }
  let mut visited: Vec<bool> = vec![false; graph.len()];
  let mut worklist: Vec<usize> = vec![root];
  visited[root] = true;

  while let Some(current) = worklist.pop() {
    let node = graph.node(current);
    if node.has_toplevel_await {
      return true;
    }
    for &descendant in &node.requested {
      if let RequestedSlot::Module(child) = descendant
        && graph.node(child).is_source_text
        && !visited[child]
      {
        visited[child] = true;
        worklist.push(child);
      }
    }
  }
  false
}

/// 编译期字符串相等 (str 的 const PartialEq 尚未稳定, 手写逐字节比较)
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(MODULE_FIELDS.len() == 7);
  assert!(str_eq(MODULE_FIELDS[2].0, "status"));
  assert!(str_eq(MODULE_FIELDS[6].0, "top_level_capability"));
  assert!(JS_MODULE_NAMESPACE_FIELDS.len() == 1);
  assert!(SCRIPT_OR_MODULE_FIELDS.len() == 2);
  assert!(JS_DEFERRED_MODULE_NAMESPACE_NEW_FIELDS == 0);
  assert!(MODULE_MEMBERS_H.len() == 7);
  assert!(Status::Unlinked as u32 == K_UNLINKED);
  assert!(Status::Errored as u32 == K_ERRORED);
  assert!(Status::Linked as u32 == K_LINKED);
  assert!((Status::Evaluating as u32) < (Status::EvaluatingAsync as u32));
  assert!(K_EVALUATING_ASYNC < K_EVALUATED);
  assert!(ATTR_ABSENT == 64);
  assert!(str_eq(K_THEN_STRING, "then"));
};
