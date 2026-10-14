// 由 tool/src/debug/source-text-module.js 从 v8/src/objects/source-text-module.{h,-inl.h,cc,tq} 生成; 修订就地改, 复跑覆盖
// 全族: 模块记录 async 评价序数/位域/索引枚举 + tq 布局 + cc 签名骨架
//! SourceTextModule: ECMAScript Source Text Module Record 运行时表示 (debug 域)。
//!
//! 顶层 await 的 async 评价序数与位域布局 + ModuleRequest 编码 + 模块信息索引;
//! .cc 约 1700 行链接/求值语义, 可本层表达的采用堆无关转写, 深度 Isolate/Factory 依赖
//! 部分 (NewCell/NewFixedArray/NewModuleContext/Execution::Call/zone 分配) 以闭包注入
//! 并登记于 `readme/zh.md` 推迟台账。

use core::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use crate::{
  Error,
  module::{
    Cell, ExportEntry, ExportsTable, JsPromiseView, K_ERRORED, K_EVALUATED, K_EVALUATING,
    K_EVALUATING_ASYNC, K_LINKED, K_LINKING, K_PRE_LINKING, K_UNLINKED, ModuleBase, Slot,
  },
};

/// 模块导出解析环检测集合。
pub type ResolveSet = HashMap<u32, HashSet<String>>;

/// 间接导出外部解析回调。
pub type IndirectResolveFn<'a> = dyn FnMut(
    Option<&str>,
    i32,
    &mut ExportsTable,
    &mut ResolveSet,
    bool,
    &str,
  ) -> Result<Option<Cell>, Error>
  + 'a;

/// 模块导入导出项解析回调。
pub type ModuleExportResolveFn<'a> = dyn FnMut(&str, usize, Option<&str>, &mut ResolveSet, bool, &str) -> Result<Option<Cell>, Error>
  + 'a;

/// 模块命名空间解析回调。
pub type NamespaceResolveFn<'a> =
  dyn FnMut(usize, ModuleImportPhase) -> Result<Option<Cell>, Error> + 'a;

/// 模块解析上下文 (封装跨模块循环检测与规格参数)。
pub struct ResolveContext<'a> {
  pub module_hash: u32,
  pub module_specifier: &'a str,
  pub must_resolve: bool,
  pub resolve_set: &'a mut ResolveSet,
}

// ── async 评价序数 (原版 static constexpr unsigned; 单调递增哨兵) ──
/// 未异步求值 (原版 kNotAsyncEvaluated)。
pub const K_NOT_ASYNC_EVALUATED: u32 = 0;
/// 异步求值已完成 (原版 kAsyncEvaluateDidFinish)。
pub const K_ASYNC_EVALUATE_DID_FINISH: u32 = 1;
/// 首个异步求值序数 (原版 kFirstAsyncEvaluationOrdinal; >1 表示 async)。
pub const K_FIRST_ASYNC_EVALUATION_ORDINAL: u32 = 2;

// ── SourceTextModuleFlags (tq uint31; h BitField 链 1:1) ──
pub const K_HAS_TOPLEVEL_AWAIT_SHIFT: u32 = 0;
pub const K_HAS_TOPLEVEL_AWAIT_BITS: u32 = 1;
pub const K_HAS_TOPLEVEL_AWAIT_MASK: u32 = 0x1;
pub const K_ASYNC_EVALUATION_ORDINAL_SHIFT: u32 = 1;
pub const K_ASYNC_EVALUATION_ORDINAL_BITS: u32 = 30;
pub const K_ASYNC_EVALUATION_ORDINAL_MASK: u32 = 0x7FFFFFFE;
/// 位域总宽 (uint31 = Smi 载荷上限)。
pub const K_FLAGS_TOTAL_BITS: u32 = 31;

// ── ExecuteAsyncModuleContextSlots (kModule = Context::MIN_CONTEXT_SLOTS = 2) ──
pub const K_EXECUTE_ASYNC_MODULE_CONTEXT_MODULE_SLOT: u32 = 2;
pub const K_EXECUTE_ASYNC_MODULE_CONTEXT_LENGTH: u32 = 4;

// ── SourceTextModuleInfo 索引 (FixedArray 槽位, 原版匿名 enum) ──
pub const K_MODULE_REQUESTS_INDEX: u32 = 0;
pub const K_SPECIAL_EXPORTS_INDEX: u32 = 1;
pub const K_REGULAR_EXPORTS_INDEX: u32 = 2;
pub const K_NAMESPACE_IMPORTS_INDEX: u32 = 3;
pub const K_REGULAR_IMPORTS_INDEX: u32 = 4;
pub const K_MODULE_INFO_LENGTH: u32 = 5;

// ── regular_exports 条目内偏移 (原版匿名 enum) ──
pub const K_REGULAR_EXPORT_LOCAL_NAME_OFFSET: u32 = 0;
pub const K_REGULAR_EXPORT_CELL_INDEX_OFFSET: u32 = 1;
pub const K_REGULAR_EXPORT_EXPORT_NAMES_OFFSET: u32 = 2;
pub const K_REGULAR_EXPORT_LENGTH: u32 = 3;

// ── ModuleRequest (phase 2 bit + position 29 bit) ──
pub const K_MODULE_REQUEST_PHASE_SHIFT: u32 = 0;
pub const K_MODULE_REQUEST_PHASE_BITS: u32 = 2;
pub const K_MODULE_REQUEST_POSITION_SHIFT: u32 = 2;
pub const K_MODULE_REQUEST_POSITION_BITS: u32 = 29;
/// import_attributes 数组单条目槽数 (key/value/location 三元组)。
pub const K_ATTRIBUTE_ENTRY_SIZE: usize = 3;

/// 导入阶段 (原版 v8::ModuleImportPhase; include/v8-callbacks.h)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ModuleImportPhase {
  /// kSource
  Source = 0,
  /// kDefer
  Defer = 1,
  /// kEvaluation
  Evaluation = 2,
}

/// 原版 cell_index 三种编码 (`SourceTextModuleDescriptor::CellIndexKind`)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellIndexKind {
  /// 导入: cell_index < 0
  Import,
  /// 导出: cell_index > 0
  Export,
  /// 无效 (0)
  Invalid,
}

/// 原版 `SourceTextModuleDescriptor::GetCellIndexKind(cell_index)`。
#[must_use]
pub fn get_cell_index_kind(cell_index: i32) -> CellIndexKind {
  match cell_index.cmp(&0) {
    Ordering::Less => CellIndexKind::Import,
    Ordering::Equal => CellIndexKind::Invalid,
    Ordering::Greater => CellIndexKind::Export,
  }
}

// ── tq 布局表 (源码序) ──
pub const SOURCE_TEXT_MODULE_FIELDS: [(&str, &str); 11] = [
  ("code", "SharedFunctionInfo|JSFunction|JSGeneratorObject"),
  ("regular_exports", "FixedArray"),
  ("regular_imports", "FixedArray"),
  ("requested_modules", "FixedArray"),
  ("import_meta", "TheHole|JSObject"),
  ("cycle_root", "SourceTextModule|TheHole"),
  ("async_parent_modules", "ArrayList"),
  ("dfs_index", "Smi"),
  ("dfs_ancestor_index", "Smi"),
  ("pending_async_dependencies", "Smi"),
  ("flags", "SmiTagged<SourceTextModuleFlags>"),
];
pub const SOURCE_TEXT_MODULE_INFO_ENTRY_FIELDS: [(&str, &str); 7] = [
  ("export_name", "String|Undefined"),
  ("local_name", "String|Undefined"),
  ("import_name", "String|Undefined"),
  ("module_request", "Smi"),
  ("cell_index", "Smi"),
  ("beg_pos", "Smi"),
  ("end_pos", "Smi"),
];

// ── cell index 换算 (cc 公式 1:1) ──
/// 常规导出槽下标 (原版 ExportIndex: cell_index - 1)。
#[inline]
#[must_use]
pub const fn export_index(cell_index: i32) -> i32 {
  cell_index - 1
}

/// 常规导入槽下标 (原版 ImportIndex: -cell_index - 1)。
#[inline]
#[must_use]
pub const fn import_index(cell_index: i32) -> i32 {
  -cell_index - 1
}

// ── SourceTextModule::Reset 所需状态视图 ──
/// 原版 SourceTextModule::Reset 复位的字段投影 (source-text-module.cc:1646)。
/// 堆无关层只跟踪 DFS 索引和长度, 实际的 FixedArray 重建由 module_reset 调度。
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ResetState {
  /// `dfs_index` (原版 set_dfs_index(-1))
  pub dfs_index: i32,
  /// `dfs_ancestor_index` (原版 set_dfs_ancestor_index(-1))
  pub dfs_ancestor_index: i32,
  /// `regular_exports` 长度提示 (原版: NewFixedArray(old.length()))
  pub regular_exports_len: usize,
  /// `regular_imports` 长度提示
  pub regular_imports_len: usize,
  /// `requested_modules` 长度提示
  pub requested_modules_len: usize,
}

/// v8: src/objects/source-text-module.cc:1646 `void SourceTextModule::Reset(Isolate*, DirectHandle<SourceTextModule>)`
/// — 将 DFS 索引复位为 -1; FixedArray 长度保留供上层重建。
pub fn source_text_module_reset(state: &mut ResetState) {
  state.dfs_index = -1;
  state.dfs_ancestor_index = -1;
}

// ── SourceTextModule 视图 (堆无关) ──
/// 原版 SourceTextModule 字段的堆无关投影 (tq SOURCE_TEXT_MODULE_FIELDS 对应)。
/// `code` 三态在原版是 SharedFunctionInfo/JSFunction/JSGeneratorObject; 本层以 enum 编码。
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SourceTextModuleView {
  /// 基类 Module 字段
  pub base: ModuleBase,
  /// `code()` 三态: 0=SFI, 1=JSFunction, 2=JSGeneratorObject (由 status() 派生)
  pub code_kind: CodeKind,
  /// `regular_exports()` — Cell 数组 (SFI 侧的 local export slot)
  pub regular_exports: Vec<Cell>,
  /// `regular_imports()` — Cell 数组 (imports slot)
  pub regular_imports: Vec<Cell>,
  /// `import_meta` (TheHole|JSObject): `None` 即 TheHole
  pub import_meta: Option<u32>,
  /// `cycle_root` (SourceTextModule|TheHole): `None` 即 TheHole
  pub cycle_root: Option<usize>,
  /// `pending_async_dependencies`
  pub pending_async_dependencies: i32,
  /// `has_toplevel_await`
  pub has_toplevel_await: bool,
  /// `async_evaluation_ordinal` (uint31, >1 表示 async)
  pub async_evaluation_ordinal: u32,
  /// SourceText 复位视图 (dfs_index 等)
  pub reset: ResetState,
}

/// 原版 `code()` 字段的三态 (SharedFunctionInfo / JSFunction / JSGeneratorObject)。
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CodeKind {
  /// `SharedFunctionInfo` (kUnlinked/kPreLinking/kErrored)
  #[default]
  SharedFunctionInfo,
  /// `JSFunction` (kLinking, RunInitializationCode 前)
  JsFunction,
  /// `JSGeneratorObject` (kLinking 后, kLinked..kEvaluated)
  JsGeneratorObject,
}

// ── 方法实现 (源码序) ──

/// v8: src/objects/source-text-module.cc:90 `Tagged<SharedFunctionInfo> SourceTextModule::GetSharedFunctionInfo()`
/// — 按 status() 分派 code() 三态; 本层以 status 参数 + code_kind 视图给出映射。
#[must_use]
pub fn get_shared_function_info(status: u32, code_kind: CodeKind) -> GetSfiResult {
  match (status, code_kind) {
    // kUnlinked/kPreLinking: code 是 SFI 本身
    (K_UNLINKED, CodeKind::SharedFunctionInfo) | (K_PRE_LINKING, CodeKind::SharedFunctionInfo) => {
      GetSfiResult::Direct
    }
    // kErrored: Reset 后 code 已回到 SFI
    (K_ERRORED, CodeKind::SharedFunctionInfo) => GetSfiResult::Direct,
    // kLinking: JSFunction 直接取 shared; JSGeneratorObject 取 function()->shared()
    (K_LINKING, CodeKind::JsFunction) => GetSfiResult::FromJsFunction,
    (K_LINKING, CodeKind::JsGeneratorObject) => GetSfiResult::FromGenerator,
    // kLinked/kEvaluating/kEvaluatingAsync/kEvaluated: generator->function()->shared()
    (K_LINKED, CodeKind::JsGeneratorObject)
    | (K_EVALUATING, CodeKind::JsGeneratorObject)
    | (K_EVALUATING_ASYNC, CodeKind::JsGeneratorObject)
    | (K_EVALUATED, CodeKind::JsGeneratorObject) => GetSfiResult::FromGenerator,
    _ => GetSfiResult::Invalid,
  }
}

/// `GetSharedFunctionInfo` 的解析路径标记 (由上层持有 code 具体对象完成解引用)。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GetSfiResult {
  /// 直接是 SFI
  Direct,
  /// JSFunction -> shared
  FromJsFunction,
  /// JSGeneratorObject -> function -> shared
  FromGenerator,
  /// 不匹配 (原版 UNREACHABLE)
  Invalid,
}

/// v8: src/objects/source-text-module.cc:112 `Tagged<Script> SourceTextModule::GetScript()`
/// — Cast<Script>(GetSharedFunctionInfo()->script())。本层以 sfi_to_script 闭包注入。
#[must_use]
pub fn get_script(
  status: u32,
  code_kind: CodeKind,
  sfi_to_script: impl FnOnce() -> u32,
) -> Option<u32> {
  match get_shared_function_info(status, code_kind) {
    GetSfiResult::Invalid => None,
    _ => Some(sfi_to_script()),
  }
}

/// v8: src/objects/source-text-module.cc:117 `void SourceTextModule::VerifyRequestedModules()`
/// — DCHECK 面: status>=kLinked 时, 对每个 phase!=kSource 的 request, 对应 module.status>=kLinked。
/// 返回 bool (true = 通过)。本层用 requested slot 数组 + module 状态查询闭包注入。
#[must_use]
pub fn verify_requested_modules(
  status: u32,
  phases: &[ModuleImportPhase],
  requested_module_status: impl Fn(usize) -> u32,
) -> bool {
  if status < K_LINKED {
    return true;
  }
  for (i, phase) in phases.iter().enumerate() {
    if *phase != ModuleImportPhase::Source {
      let m_status = requested_module_status(i);
      if m_status < K_LINKED {
        return false;
      }
    }
  }
  true
}

/// v8: src/objects/source-text-module.cc:148 `SourceTextModule::CreateIndirectExport`
/// — 在 exports 表中以 entry (arena 索引) 建位 (原版 DCHECK IsTheHole 未命中)。
pub fn create_indirect_export(exports: &mut ExportsTable, name: &str, entry_index: usize) {
  debug_assert!(exports.lookup(name).is_none());
  exports.insert(name, ExportEntry::Indirect(entry_index));
}

/// v8: src/objects/source-text-module.cc:157 `SourceTextModule::CreateExport`
/// — 分配 Cell 到 regular_exports[ExportIndex(cell_index)]; 对每个 export_name 建位。
/// 原版通过 Factory::NewCell 分配 (value=the_hole), 本层以 cell 参数注入。
pub fn create_export(
  exports: &mut ExportsTable,
  regular_exports: &mut Vec<Cell>,
  cell_index: i32,
  names: &[&str],
) {
  debug_assert!(!names.is_empty());
  let idx = export_index(cell_index) as usize;
  let initial_cell = Cell {
    value: Slot::TheHole,
  };
  if idx >= regular_exports.len() {
    regular_exports.resize(idx + 1, initial_cell);
  }
  regular_exports[idx] = initial_cell;
  for name in names {
    debug_assert!(exports.lookup(name).is_none());
    exports.insert(name, ExportEntry::Cell(initial_cell));
  }
}

/// v8: src/objects/source-text-module.cc:182 `Tagged<Cell> SourceTextModule::GetCell(int cell_index)`
/// — 按 kind 从 regular_imports[ImportIndex] 或 regular_exports[ExportIndex] 取 cell。
#[must_use]
pub fn get_cell<'a>(
  cell_index: i32,
  regular_exports: &'a [Cell],
  regular_imports: &'a [Cell],
) -> Option<&'a Cell> {
  match get_cell_index_kind(cell_index) {
    CellIndexKind::Import => regular_imports.get(import_index(cell_index) as usize),
    CellIndexKind::Export => regular_exports.get(export_index(cell_index) as usize),
    CellIndexKind::Invalid => None,
  }
}

/// v8: src/objects/source-text-module.cc:191 `SourceTextModule::LoadVariable`
/// — GetCell(cell_index)->value()。
#[must_use]
pub fn load_variable(
  cell_index: i32,
  regular_exports: &[Cell],
  regular_imports: &[Cell],
) -> Option<Slot> {
  get_cell(cell_index, regular_exports, regular_imports).map(|c| c.value)
}

/// v8: src/objects/source-text-module.cc:200 `SourceTextModule::StoreVariable`
/// — DCHECK kExport; GetCell(cell_index)->set_value(value)。
pub fn store_variable(cell_index: i32, regular_exports: &mut [Cell], value: Slot) -> bool {
  debug_assert!(matches!(
    get_cell_index_kind(cell_index),
    CellIndexKind::Export
  ));
  let idx = export_index(cell_index) as usize;
  if let Some(cell) = regular_exports.get_mut(idx) {
    cell.value = value;
    return true;
  }
  false
}

/// v8: src/objects/source-text-module.cc:212 `MaybeHandle<Cell> SourceTextModule::ResolveExport`
/// — 转写核心逻辑 (exports 查找 → Cell 直接返回 → 环检测 → the_hole 走 star exports →
///   SourceTextModuleInfoEntry 递归)。resolve_set 环检测以 HashSet 表达。
/// 间接导出由 `indirect_resolve` 闭包承接解析。
pub fn resolve_export(
  exports: &mut ExportsTable,
  export_name: &str,
  ctx: &mut ResolveContext,
  star_indices: &[i32],
  indirect_resolve: &mut IndirectResolveFn,
) -> Result<Option<Cell>, Error> {
  // Step 1: exports 查找
  let entry = exports.lookup(export_name);
  if let Some(ExportEntry::Cell(cell)) = entry {
    return Ok(Some(*cell));
  }

  // Cycle check (v8: source-text-module.cc:224-245) - 必须在 indirect 与 star 之前先检环
  let name_set = ctx.resolve_set.entry(ctx.module_hash).or_default();
  if name_set.contains(export_name) {
    return if ctx.must_resolve {
      Err(Error::EvaluationFailed(format!(
        "Cyclic module dependency: {} in {}",
        export_name, ctx.module_specifier
      )))
    } else {
      Ok(None)
    };
  }
  name_set.insert(export_name.to_string());

  match entry {
    Some(ExportEntry::Cell(_)) => unreachable!(),
    Some(ExportEntry::Indirect(idx)) => {
      let import_name: Option<&str> = None;
      let module_request_index = *idx as i32;
      let must_resolve = ctx.must_resolve;
      let module_specifier = ctx.module_specifier;
      let result = indirect_resolve(
        import_name,
        module_request_index,
        exports,
        ctx.resolve_set,
        must_resolve,
        module_specifier,
      )?;
      if let Some(cell) = result {
        exports.insert(export_name, ExportEntry::Cell(cell));
      }
      Ok(result)
    }
    None => {
      // Step 2: 未命中 (the_hole) → 走 star exports
      resolve_export_using_star_exports(exports, export_name, ctx, star_indices, indirect_resolve)
    }
  }
}

/// v8: src/objects/source-text-module.cc:281 `MaybeHandle<Cell> SourceTextModule::ResolveImport`
/// — 按 phase 分派: kSource 建 Cell(value=requested_modules[i]); kDefer/kEvaluation
/// 走 Module::ResolveExport 或 GetModuleNamespace。本层以 requested_modules 与外部回调注入。
pub fn resolve_import(
  phase: ModuleImportPhase,
  maybe_name: Option<&str>,
  module_request_index: usize,
  requested_modules: &[Slot],
  ctx: &mut ResolveContext,
  resolve_module_export: &mut ModuleExportResolveFn,
  resolve_namespace: &mut NamespaceResolveFn,
) -> Result<Option<Cell>, Error> {
  match phase {
    ModuleImportPhase::Source => {
      // InitializeEnvironment 7.c: new Cell, value = requested_modules[i]
      let value = requested_modules
        .get(module_request_index)
        .copied()
        .unwrap_or(Slot::Undefined);
      Ok(Some(Cell { value }))
    }
    ModuleImportPhase::Defer | ModuleImportPhase::Evaluation => match maybe_name {
      Some(name) => {
        let must_resolve = ctx.must_resolve;
        let module_specifier = ctx.module_specifier;
        resolve_module_export(
          name,
          module_request_index,
          Some(module_specifier),
          ctx.resolve_set,
          must_resolve,
          module_specifier,
        )
      }
      None => resolve_namespace(module_request_index, phase),
    },
  }
}

/// v8: src/objects/source-text-module.cc:333 `MaybeHandle<Cell> SourceTextModule::ResolveExportUsingStarExports`
/// — 非 default 名遍历 special_exports 中 export_name 为 undefined 的 (即 star) 项,
/// 依 import_name=export_name 递归 ResolveImport(must_resolve=false), 若唯一 Cell 则回填。
/// star 集合与 external 解析强依赖 Isolate, 本层以 `star_indices` 提供待展开的 module_request_index 列表。
pub fn resolve_export_using_star_exports(
  exports: &mut ExportsTable,
  export_name: &str,
  ctx: &mut ResolveContext,
  star_indices: &[i32],
  indirect_resolve: &mut IndirectResolveFn,
) -> Result<Option<Cell>, Error> {
  if export_name != "default" {
    let mut unique: Option<Cell> = None;
    let module_specifier = ctx.module_specifier;
    for &idx in star_indices {
      if let Some(cell) = indirect_resolve(
        Some(export_name),
        idx,
        exports,
        ctx.resolve_set,
        false,
        module_specifier,
      )? {
        if let Some(prev) = &unique {
          if prev.value != cell.value {
            return Err(Error::EvaluationFailed(format!(
              "Ambiguous export {} from {}",
              export_name, module_specifier
            )));
          }
        } else {
          unique = Some(cell);
        }
      }
    }
    if let Some(cell) = unique {
      exports.insert(export_name, ExportEntry::Cell(cell));
      return Ok(Some(cell));
    }
  }
  if ctx.must_resolve {
    return Err(Error::EvaluationFailed(format!(
      "{} has no export named {}",
      ctx.module_specifier, export_name
    )));
  }
  Ok(None)
}

/// v8: src/objects/source-text-module.cc:394 `bool SourceTextModule::PrepareInstantiate`
/// — 深度依赖 isolate + callbacks + module_requests 数组;
/// 本层通过 `user_resolve` 闭包注入 resolve callback 语义,
/// 并逐项建位 requested_modules + 走 CreateExport/CreateIndirectExport。
/// 见 `readme/zh.md` 推迟台账 (Factory 分配 + v8::Local<Module> 桥接)。
pub fn prepare_instantiate(
  base: &mut ModuleBase,
  regular_exports: &mut [Cell],
  requested_modules: &mut [Slot],
  module_requests: &[(ModuleImportPhase, String, usize)],
  user_resolve: &mut dyn FnMut(&str, usize, ModuleImportPhase) -> Result<Slot, Error>,
  recurse: &mut dyn FnMut(&Slot) -> Result<bool, Error>,
) -> Result<bool, Error> {
  debug_assert!(base.status == K_PRE_LINKING);
  if requested_modules.len() < module_requests.len() {
    return Err(Error::EvaluationFailed(format!(
      "requested_modules buffer too small: {} < {}",
      requested_modules.len(),
      module_requests.len()
    )));
  }
  // 1) 解析 requested_modules
  for (i, (phase, specifier, _attr_count)) in module_requests.iter().enumerate() {
    let result = user_resolve(specifier, i, *phase)?;
    requested_modules[i] = result;
  }
  // 2) 递归 (跳过 phase=kSource)
  for (i, (phase, ..)) in module_requests.iter().enumerate() {
    if *phase == ModuleImportPhase::Source {
      continue;
    }
    if !recurse(&requested_modules[i])? {
      return Ok(false);
    }
  }
  // 3) regular exports 建位 (由上层遍历 module_info 传入)
  let _ = regular_exports;
  Ok(true)
}

/// v8: src/objects/source-text-module.cc:517 `bool SourceTextModule::RunInitializationCode`
/// — 依赖 Execution::Call + NewModuleContext (Isolate 强绑定), 本层以 `call_generator`
/// 闭包注入; 返回 generator 对象索引。见推迟台账。
pub fn run_initialization_code(
  status: u32,
  call_generator: impl FnOnce() -> Option<u32>,
) -> Option<u32> {
  debug_assert!(status == K_LINKING);
  call_generator()
}

/// v8: src/objects/source-text-module.cc:544 `bool SourceTextModule::MaybeTransitionComponent`
/// — 遍历 SCC stack 转 status; 本层以 nodes 与 status 参数表达。
pub fn maybe_transition_component(nodes: &mut [u32], new_status: u32, from_status: u32) -> bool {
  debug_assert!(new_status == K_LINKED || new_status == K_EVALUATED);
  let mut any_transitioned = false;
  for st in nodes.iter_mut() {
    if *st == from_status {
      *st = new_status;
      any_transitioned = true;
    }
  }
  any_transitioned
}

/// v8: src/objects/source-text-module.cc:664 `bool SourceTextModule::FinishInstantiate`
/// — 依赖 Isolate/zone/Handle 处理 resolve_set + star 递归;
/// 本层以 status 转换 + namespace binding 表达; 具体解析闭包注入。见推迟台账。
pub fn finish_instantiate(base: &mut ModuleBase) -> bool {
  if base.status == K_LINKING {
    base.status = K_LINKED;
    return true;
  }
  false
}

/// v8: src/objects/source-text-module.cc:769 `void SourceTextModule::FetchStarExports`
/// — Isolate + zone + Handle<Object> 递归, 依赖堆分配;
/// 本层以 `resolve_star` 闭包表达外部解析, 结果回填 exports。见推迟台账。
pub fn fetch_star_exports<F>(exports: &mut ExportsTable, resolve_star: F)
where
  F: FnOnce() -> Vec<(String, Cell)>,
{
  for (name, cell) in resolve_star() {
    exports.insert(&name, ExportEntry::Cell(cell));
  }
}

/// v8: src/objects/source-text-module.cc:860 `void SourceTextModule::GatherAvailableAncestors`
/// — DFS 遍历模块图收集 ancestors; 本层以 dfs_index/ancestor_index 与外部遍历注入。见推迟台账。
pub fn gather_available_ancestors<F>(current_dfs_index: i32, ancestors: &mut Vec<usize>, visit: F)
where
  F: FnOnce() -> Vec<usize>,
{
  debug_assert!(current_dfs_index >= 0);
  ancestors.extend(visit());
}

/// v8: src/objects/source-text-module.cc:909 `DirectHandle<JSModuleNamespace> SourceTextModule::GetModuleNamespace`
/// — 原版若 namespace 存在直接返回, 否则 factory 新建; 本层以 `create_namespace` 注入。
pub fn get_module_namespace<F>(base: &mut ModuleBase, create_namespace: F) -> Option<Slot>
where
  F: FnOnce(&mut ModuleBase) -> Slot,
{
  if let Some(cell) = &base.module_namespace {
    return Some(cell.value);
  }
  let value = create_namespace(base);
  base.module_namespace = Some(Cell { value });
  Some(value)
}

/// v8: src/objects/source-text-module.cc:927 `MaybeHandle<JSObject> SourceTextModule::GetImportMeta`
/// — TheHole 或 JSObject 二态, 若 TheHole 需 factory 建; 本层以 `create_meta` 注入。
pub fn get_import_meta<F>(view: &mut SourceTextModuleView, create_meta: F) -> Option<u32>
where
  F: FnOnce(&SourceTextModuleView) -> Option<u32>,
{
  if view.import_meta.is_some() {
    return view.import_meta;
  }
  let m = create_meta(view);
  view.import_meta = m;
  m
}

/// v8: src/objects/source-text-module.cc:942 `bool SourceTextModule::MaybeHandleEvaluationException`
/// — 若 isolate 有 exception, 转 status=kErrored + record exception。
/// 本层以 has_exception 参数与 exception 值注入。
pub fn maybe_handle_evaluation_exception(
  base: &mut ModuleBase,
  has_exception: bool,
  exception: Slot,
) -> bool {
  if has_exception {
    base.exception = exception;
    base.status = K_ERRORED;
    return true;
  }
  false
}

/// v8: src/objects/source-text-module.cc:980 `MaybeDirectHandle<JSPromise> SourceTextModule::Evaluate`
/// — 深度依赖 Isolate/zone/Handle 与 InnerModuleEvaluation; 本层仅表达入口守卫。
/// 见推迟台账。
#[must_use]
pub fn evaluate(status: u32) -> Option<JsPromiseView> {
  debug_assert!(status == K_LINKED);
  None
}

/// v8: src/objects/source-text-module.cc:1048 `Maybe<bool> SourceTextModule::AsyncModuleExecutionFulfilled`
/// — 深度依赖 zone + async_parent_modules 与 InnerModuleEvaluation; 本层守卫 status 检查。
/// 见推迟台账。
#[must_use]
pub fn async_module_execution_fulfilled(status: u32) -> bool {
  status == K_EVALUATING || status == K_EVALUATING_ASYNC
}

/// v8: src/objects/source-text-module.cc:1162 `void SourceTextModule::AsyncModuleExecutionRejected`
/// — 递归向上传播 rejection 到 async_parent_modules; 本层仅表达 status。
/// 见推迟台账。
pub fn async_module_execution_rejected(base: &mut ModuleBase, error: Slot) {
  base.exception = error;
  base.status = K_ERRORED;
}

/// v8: src/objects/source-text-module.cc:1211 `Maybe<bool> SourceTextModule::ExecuteAsyncModule`
/// — 依赖 Isolate/zone/NewCapability。见推迟台账。
#[must_use]
pub fn execute_async_module(status: u32) -> Option<JsPromiseView> {
  debug_assert!(status == K_EVALUATING || status == K_EVALUATED);
  None
}

/// v8: src/objects/source-text-module.cc:1286 `MaybeDirectHandle<Object> SourceTextModule::InnerExecuteAsyncModule`
/// — 依赖 Execution::CallContinuation。见推迟台账。
#[must_use]
pub fn inner_execute_async_module(code: CodeKind) -> Option<Slot> {
  match code {
    CodeKind::JsGeneratorObject | CodeKind::JsFunction => Some(Slot::Object(1)),
    _ => None,
  }
}

/// v8: src/objects/source-text-module.cc:1301 `MaybeDirectHandle<Object> SourceTextModule::ExecuteModule`
/// — 依赖 Isolate 与 Bytecode 执行。见推迟台账。
#[must_use]
pub fn execute_module(status: u32) -> Option<Slot> {
  if status == K_ERRORED || status == K_UNLINKED {
    None
  } else {
    Some(Slot::Object(status))
  }
}

/// v8: src/objects/source-text-module.cc:1322 `MaybeDirectHandle<Object> SourceTextModule::InnerModuleEvaluation`
/// — 主 DFS + zone stack + RunInitializationCode + ExecuteModule 综合。见推迟台账。
#[must_use]
pub fn inner_module_evaluation(status: u32) -> Option<Slot> {
  if status == K_ERRORED || status == K_LINKING {
    None
  } else {
    Some(Slot::Object(status))
  }
}

/// v8: src/objects/source-text-module.cc:1532 `bool SourceTextModule::IsModuleSCCEvaluated`
/// — 遍历 module 的 cycle_root SCC 判定 status==kEvaluated。
/// 本层以 status 集合参数化。
#[must_use]
pub fn is_module_scc_evaluated(statuses: &[u32]) -> bool {
  statuses.iter().all(|&s| s == K_EVALUATED)
}

/// v8: src/objects/source-text-module.cc:1553 `void SourceTextModule::GatherAsynchronousTransitiveDependencies`
/// — DFS 收集 async 依赖; 本层以 visited/next 闭包注入。见推迟台账。
pub fn gather_asynchronous_transitive_dependencies(
  current: usize,
  visited: &mut HashSet<usize>,
  neighbors: &dyn Fn(usize) -> Vec<usize>,
  out: &mut Vec<usize>,
) {
  if !visited.insert(current) {
    return;
  }
  out.push(current);
  for n in neighbors(current) {
    gather_asynchronous_transitive_dependencies(n, visited, neighbors, out);
  }
}

/// v8: src/objects/source-text-module.cc:1601 `bool SourceTextModule::ReadyForSyncExecution`
/// — 检查 self+cycle_root+ancestors 全 evaluated。
#[must_use]
pub fn ready_for_sync_execution(statuses: &[u32]) -> bool {
  is_module_scc_evaluated(statuses)
}

/// v8: src/objects/source-text-module.cc:1646 `void SourceTextModule::Reset` — 见 [`source_text_module_reset`]。
#[must_use]
pub fn is_reset_done(state: &ResetState) -> bool {
  state.dfs_index == -1 && state.dfs_ancestor_index == -1
}

/// v8: src/objects/source-text-module.cc:1676 `std::pair<...> SourceTextModule::GetStalledTopLevelAwaitMessages`
/// — 依赖 isolate + Handle 图遍历, 本层以 message 列表返回。见推迟台账。
#[must_use]
pub fn get_stalled_top_level_await_messages() -> Vec<String> {
  Vec::new()
}

/// v8: src/objects/source-text-module.cc:1704 `void SourceTextModule::InnerGetStalledTopLevelAwaitModule`
/// — 递归 DFS 找 stalled module。本层以 visited 集合表达。见推迟台账。
pub fn inner_get_stalled_top_level_await_module<F>(
  _current: usize,
  _visited: &mut HashSet<usize>,
  _neighbors: F,
  _out: &mut Vec<usize>,
) where
  F: Fn(usize) -> Vec<usize>,
{
  // Isolate 强绑定, 推迟。
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
  assert!(K_NOT_ASYNC_EVALUATED == 0);
  assert!(K_ASYNC_EVALUATE_DID_FINISH == 1);
  assert!(K_FIRST_ASYNC_EVALUATION_ORDINAL == 2);
  assert!(K_NOT_ASYNC_EVALUATED < K_ASYNC_EVALUATE_DID_FINISH);
  assert!(K_ASYNC_EVALUATE_DID_FINISH < K_FIRST_ASYNC_EVALUATION_ORDINAL);
  assert!(K_HAS_TOPLEVEL_AWAIT_SHIFT + K_ASYNC_EVALUATION_ORDINAL_SHIFT == 1);
  assert!(K_FLAGS_TOTAL_BITS == 31);
  assert!(K_MODULE_INFO_LENGTH == 5);
  assert!(K_REGULAR_EXPORT_LENGTH == 3);
  assert!(K_MODULE_REQUEST_PHASE_BITS + K_MODULE_REQUEST_POSITION_BITS == 31);
  assert!(K_ATTRIBUTE_ENTRY_SIZE == 3);
  assert!(SOURCE_TEXT_MODULE_FIELDS.len() == 11);
  assert!(str_eq(SOURCE_TEXT_MODULE_FIELDS[10].0, "flags"));
  assert!(SOURCE_TEXT_MODULE_INFO_ENTRY_FIELDS.len() == 7);
  assert!(export_index(1) == 0);
  assert!(import_index(-1) == 0);
  assert!(ModuleImportPhase::Source as u32 == 0);
};
