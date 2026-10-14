//! AST 作用域模型 (v8/src/ast/scopes.h/.cc)
//!
//! 原版为指针树 (Scope* outer_scope_ / inner_scope_ / sibling_) + zone 分配;
//! Rust 以 arena 句柄 (ScopeId/VariableId) 等价表达, 生命周期归 ScopeTree。
//! 本阶段覆盖解析期核心路径: 声明 (Declare 族) / 查找 (Lookup 族, 含
//! with/sloppy-eval 动态回退与全局回退) / 变量分配 (Allocate 族, 栈槽与
//! context 槽) / 未消解引用 (unresolved + Resolve 族)。
//! ScopeInfo 反序列化 (LookupInScopeInfo/DeserializeScopeChain) 与
//! preparse 数据路径不在本阶段。

use gxhash::HashMap;
use v8_common::{
  InitializationFlag, IsStaticFlag, LanguageMode, MaybeAssignedFlag, ScopeType, VariableKind,
  VariableLocation, VariableMode,
};

use crate::{
  error::{Error, Result},
  value::AstRawString,
  variables::Variable,
};

/// Context 最小槽数 (原版 Context::MIN_CONTEXT_SLOTS = EXTENSION_INDEX = 2)
pub const MIN_CONTEXT_SLOTS: i32 = 2;
/// 含扩展槽的 Context 最小槽数 (原版 Context::MIN_CONTEXT_EXTENDED_SLOTS = 3)
pub const MIN_CONTEXT_EXTENDED_SLOTS: i32 = 3;
/// catch 作用域首个本地槽 (原版 Context::THROWN_OBJECT_INDEX = MIN_CONTEXT_SLOTS)
pub const THROWN_OBJECT_INDEX: i32 = MIN_CONTEXT_SLOTS;

/// 作用域句柄 (原版 Scope* 指针身份 → arena 索引)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(pub(super) u32);

impl ScopeId {
  /// 从原始索引构造 (跨树混用属调用方责任)
  pub const fn from_raw(raw: u32) -> Self {
    Self(raw)
  }

  /// 原始索引
  pub const fn to_raw(self) -> u32 {
    self.0
  }
}

/// 变量句柄 (原版 Variable* 指针身份 → arena 索引)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VariableId(pub(super) u32);

impl VariableId {
  /// 从原始索引构造 (跨树混用属调用方责任)
  pub const fn from_raw(raw: u32) -> Self {
    Self(raw)
  }

  /// 原始索引
  pub const fn to_raw(self) -> u32 {
    self.0
  }
}

/// 未消解变量引用快照 (原版 unresolved_list_ 中的 VariableProxy*)
#[derive(Debug, Clone, PartialEq)]
pub struct UnresolvedRef {
  pub name: AstRawString,
  pub position: i32,
  pub is_assigned: bool,
}

/// 单个作用域数据 (原版 class Scope + DeclarationScope 字段并集)
#[derive(Debug, Clone)]
pub(super) struct ScopeData {
  scope_type: ScopeType,
  outer: Option<ScopeId>,
  children: Vec<ScopeId>,
  start_position: i32,
  end_position: i32,
  language_mode: LanguageMode,
  /// 变量表 (原版 VariableMap variables_)
  variables: HashMap<AstRawString, VariableId>,
  /// 本地变量声明序 (原版 locals_)
  locals: Vec<VariableId>,
  /// 未消解引用 (原版 unresolved_list_)
  unresolved: Vec<UnresolvedRef>,
  /// 内层作用域出现 eval 调用 (原版 inner_scope_calls_eval)
  inner_scope_calls_eval: bool,
  /// sloppy eval 可扩展 var (原版 sloppy_eval_can_extend_vars)
  sloppy_eval_can_extend_vars: bool,
  num_stack_slots: i32,
  num_heap_slots: i32,
  /// catch 绑定变量 (原版 catch_variable_)
  catch_variable: Option<VariableId>,
  // ── DeclarationScope 专属 (非声明作用域恒为空) ──
  /// 形参表 (原版 params_)
  params: Vec<VariableId>,
  /// this 声明 (原版 receiver_)
  receiver: Option<VariableId>,
  /// 函数名变量 (原版 function_)
  function_var: Option<VariableId>,
  /// new.target 变量 (原版 new_target_)
  new_target_var: Option<VariableId>,
  /// 生成器对象变量 (原版 generator_object_var_)
  generator_object_var: Option<VariableId>,
}

impl ScopeData {
  fn new(scope_type: ScopeType, outer: Option<ScopeId>) -> Self {
    Self {
      scope_type,
      outer,
      children: Vec::new(),
      start_position: 0,
      end_position: 0,
      language_mode: LanguageMode::Sloppy,
      variables: HashMap::default(),
      locals: Vec::new(),
      unresolved: Vec::new(),
      inner_scope_calls_eval: false,
      sloppy_eval_can_extend_vars: false,
      num_stack_slots: 0,
      num_heap_slots: 0,
      catch_variable: None,
      params: Vec::new(),
      receiver: None,
      function_var: None,
      new_target_var: None,
      generator_object_var: None,
    }
  }
}

/// 作用域树容器 (原版解析期所有 Scope/Variable 的 zone 驻留集)
#[derive(Debug, Clone, Default)]
pub struct ScopeTree {
  scopes: Vec<ScopeData>,
  variables: Vec<Variable>,
}

impl ScopeTree {
  pub fn new() -> Self {
    Self::default()
  }

  fn scope(&self, id: ScopeId) -> &ScopeData {
    // 句柄仅由本树分配, 索引恒有效 (跨树混用属调用方逻辑错误)
    debug_assert!((id.0 as usize) < self.scopes.len());
    &self.scopes[id.0 as usize]
  }

  fn scope_mut(&mut self, id: ScopeId) -> &mut ScopeData {
    debug_assert!((id.0 as usize) < self.scopes.len());
    &mut self.scopes[id.0 as usize]
  }

  fn var(&self, id: VariableId) -> &Variable {
    debug_assert!((id.0 as usize) < self.variables.len());
    &self.variables[id.0 as usize]
  }

  fn var_mut(&mut self, id: VariableId) -> &mut Variable {
    debug_assert!((id.0 as usize) < self.variables.len());
    &mut self.variables[id.0 as usize]
  }

  fn alloc_var(&mut self, scope: ScopeId, var: Variable) -> VariableId {
    let id = VariableId(self.variables.len() as u32);
    self.variables.push(var);
    // 记录归属作用域 (原版 Variable::scope_)
    self.variables[id.0 as usize].set_scope(scope);
    id
  }

  // ── 构建期 ──

  /// 新建作用域 (原版 Scope 构造 + 父链挂接 inner_scope_/sibling_)
  pub fn new_scope(&mut self, scope_type: ScopeType, outer: Option<ScopeId>) -> ScopeId {
    let id = ScopeId(self.scopes.len() as u32);
    self.scopes.push(ScopeData::new(scope_type, outer));
    if let Some(parent) = outer {
      self.scope_mut(parent).children.push(id);
    }
    id
  }

  /// 作用域数量
  pub fn len(&self) -> usize {
    self.scopes.len()
  }

  /// 是否为空树
  pub fn is_empty(&self) -> bool {
    self.scopes.is_empty()
  }

  pub fn scope_type(&self, id: ScopeId) -> ScopeType {
    self.scope(id).scope_type
  }

  pub fn outer_scope(&self, id: ScopeId) -> Option<ScopeId> {
    self.scope(id).outer
  }

  /// 直接子作用域 (原版 inner_scope_/sibling_ 链展开)
  pub fn inner_scopes(&self, id: ScopeId) -> &[ScopeId] {
    &self.scope(id).children
  }

  pub fn start_position(&self, id: ScopeId) -> i32 {
    self.scope(id).start_position
  }

  pub fn end_position(&self, id: ScopeId) -> i32 {
    self.scope(id).end_position
  }

  pub fn set_start_position(&mut self, id: ScopeId, pos: i32) {
    self.scope_mut(id).start_position = pos;
  }

  pub fn set_end_position(&mut self, id: ScopeId, pos: i32) {
    self.scope_mut(id).end_position = pos;
  }

  /// 设置语言模式 (原版 SetLanguageMode, 仅函数/脚本/eval/repl 作用域)
  pub fn set_language_mode(&mut self, id: ScopeId, mode: LanguageMode) {
    self.scope_mut(id).language_mode = mode;
  }

  pub fn language_mode(&self, id: ScopeId) -> LanguageMode {
    self.scope(id).language_mode
  }

  /// 记录内层 eval 调用向上传播 (原版 RecordInnerScopeEvalCall + RecordEvalCall)
  pub fn record_inner_scope_eval_call(&mut self, id: ScopeId) {
    let mut cur = Some(id);
    while let Some(sid) = cur {
      let s = self.scope_mut(sid);
      s.inner_scope_calls_eval = true;
      cur = s.outer;
    }
  }

  pub fn inner_scope_calls_eval(&self, id: ScopeId) -> bool {
    self.scope(id).inner_scope_calls_eval
  }

  pub fn set_sloppy_eval_can_extend_vars(&mut self, id: ScopeId, value: bool) {
    self.scope_mut(id).sloppy_eval_can_extend_vars = value;
  }

  // ── 类别判定 (原版 Scope::is_* 内联族) ──

  pub fn is_declaration_scope(&self, id: ScopeId) -> bool {
    matches!(
      self.scope(id).scope_type,
      ScopeType::ScriptScope
        | ScopeType::ReplModeScope
        | ScopeType::EvalScope
        | ScopeType::ModuleScope
        | ScopeType::FunctionScope
        | ScopeType::ShadowRealmScope
    )
  }

  pub fn is_script_scope(&self, id: ScopeId) -> bool {
    matches!(self.scope(id).scope_type, ScopeType::ScriptScope)
  }

  pub fn is_eval_scope(&self, id: ScopeId) -> bool {
    matches!(self.scope(id).scope_type, ScopeType::EvalScope)
  }

  pub fn is_module_scope(&self, id: ScopeId) -> bool {
    matches!(self.scope(id).scope_type, ScopeType::ModuleScope)
  }

  pub fn is_function_scope(&self, id: ScopeId) -> bool {
    matches!(self.scope(id).scope_type, ScopeType::FunctionScope)
  }

  pub fn is_catch_scope(&self, id: ScopeId) -> bool {
    matches!(self.scope(id).scope_type, ScopeType::CatchScope)
  }

  pub fn is_block_scope(&self, id: ScopeId) -> bool {
    matches!(
      self.scope(id).scope_type,
      ScopeType::BlockScope | ScopeType::CatchScope | ScopeType::WithScope
    )
  }

  pub fn is_with_scope(&self, id: ScopeId) -> bool {
    matches!(self.scope(id).scope_type, ScopeType::WithScope)
  }

  pub fn is_class_scope(&self, id: ScopeId) -> bool {
    matches!(self.scope(id).scope_type, ScopeType::ClassScope)
  }

  /// 闭包作用域: 函数/脚本/eval/repl/module (原版 is_closure_scope)
  pub fn is_closure_scope(&self, id: ScopeId) -> bool {
    self.is_declaration_scope(id) && !matches!(self.scope(id).scope_type, ScopeType::ClassScope)
  }

  /// var 声明提升目标 (原版 GetDeclarationScope)
  pub fn get_declaration_scope(&self, mut id: ScopeId) -> ScopeId {
    while !self.is_declaration_scope(id) {
      match self.scope(id).outer {
        Some(outer) => id = outer,
        None => return id,
      }
    }
    id
  }

  /// 最近的持有 context 的外层 (原版 GetOuterScopeWithContext)
  pub fn get_outer_scope_with_context(&self, id: ScopeId) -> Option<ScopeId> {
    let mut cur = self.scope(id).outer;
    while let Some(sid) = cur {
      if self.needs_context(sid) {
        return Some(sid);
      }
      cur = self.scope(sid).outer;
    }
    None
  }

  /// 是否为 other 的外层 (原版 IsOuterScopeOf)
  pub fn is_outer_scope_of(&self, outer: ScopeId, inner: ScopeId) -> bool {
    let mut cur = self.scope(inner).outer;
    while let Some(sid) = cur {
      if sid == outer {
        return true;
      }
      cur = self.scope(sid).outer;
    }
    false
  }

  /// 本作用域到目标作用域的 context 链长 (原版 ContextChainLength)
  pub fn context_chain_length(&self, id: ScopeId, until: ScopeId) -> usize {
    let mut n = 0usize;
    let mut cur = Some(id);
    while let Some(sid) = cur {
      if sid == until {
        return n;
      }
      if self.needs_context(sid) {
        n += 1;
      }
      cur = self.scope(sid).outer;
    }
    n
  }

  /// 是否含 context 扩展槽 (原版 HasContextExtensionSlot)
  pub fn has_context_extension_slot(&self, id: ScopeId) -> bool {
    match self.scope(id).scope_type {
      ScopeType::ModuleScope | ScopeType::WithScope => true,
      _ => self.scope(id).sloppy_eval_can_extend_vars,
    }
  }

  /// context 头长度 (原版 ContextHeaderLength)
  pub fn context_header_length(&self, id: ScopeId) -> i32 {
    if self.has_context_extension_slot(id) {
      MIN_CONTEXT_EXTENDED_SLOTS
    } else {
      MIN_CONTEXT_SLOTS
    }
  }

  /// 是否需要 context (原版 NeedsContext)
  pub fn needs_context(&self, id: ScopeId) -> bool {
    self.scope(id).num_heap_slots > 0
  }

  /// 已分配 context 槽总数 (原版 num_heap_slots)
  pub fn num_heap_slots(&self, id: ScopeId) -> i32 {
    self.scope(id).num_heap_slots
  }

  /// 已分配栈槽总数 (原版 num_stack_slots)
  pub fn num_stack_slots(&self, id: ScopeId) -> i32 {
    self.scope(id).num_stack_slots
  }

  // ── 声明族 (原版 Declare 族) ──

  /// 在作用域变量表内建变量 (原版 VariableMap::Declare)
  fn declare(
    &mut self,
    scope: ScopeId,
    name: AstRawString,
    mode: VariableMode,
    kind: VariableKind,
    initialization_flag: InitializationFlag,
    maybe_assigned_flag: MaybeAssignedFlag,
  ) -> (VariableId, bool) {
    match self.scope(scope).variables.get(&name).copied() {
      Some(id) => (id, false),
      None => {
        let var = Variable::new(
          name.clone(),
          mode,
          kind,
          initialization_flag,
          maybe_assigned_flag,
          IsStaticFlag::NotStatic,
        );
        let id = self.alloc_var(scope, var);
        let s = self.scope_mut(scope);
        s.variables.insert(name, id);
        s.locals.push(id);
        (id, true)
      }
    }
  }

  /// 本地声明 (原版 Scope::DeclareLocal; 仅声明类模式)
  pub fn declare_local(
    &mut self,
    scope: ScopeId,
    name: AstRawString,
    mode: VariableMode,
    kind: VariableKind,
    initialization_flag: InitializationFlag,
  ) -> (VariableId, bool) {
    debug_assert!(mode.is_declared());
    self.declare(
      scope,
      name,
      mode,
      kind,
      initialization_flag,
      MaybeAssignedFlag::NotAssigned,
    )
  }

  /// 声明变量名, 含重声明冲突检测 (原版 Scope::DeclareVariableName 预解析期路径)
  ///
  /// var 声明在非声明作用域时提升至声明作用域 (原版 hoisting)。
  /// 词法重复声明返回 Err (原版返回 nullptr)。
  pub fn declare_variable_name(
    &mut self,
    scope: ScopeId,
    name: AstRawString,
    mode: VariableMode,
    kind: VariableKind,
  ) -> Result<VariableId> {
    let target = if mode == VariableMode::Var && !self.is_declaration_scope(scope) {
      self.get_declaration_scope(scope)
    } else {
      scope
    };
    let (id, was_added) = self.declare_local(
      target,
      name.clone(),
      mode,
      kind,
      InitializationFlag::NeedsInitialization,
    );
    if was_added {
      self.var_mut(id).set_is_used();
      return Ok(id);
    }
    let existing_mode = self.var(id).mode();
    let existing_kind = self.var(id).kind();
    // 词法冲突判定; 唯一豁免 = 已有与新增均为 sloppy 块函数 (上游 web 兼容, bug 4693)
    if mode.is_lexical() || existing_mode.is_lexical() {
      let sloppy_pair = existing_kind == VariableKind::SloppyBlockFunctionVariable
        && kind == VariableKind::SloppyBlockFunctionVariable;
      if !sloppy_pair {
        return Err(Error::Redeclaration {
          name: name.to_string_lossy(),
          mode,
          existing_mode,
        });
      }
    }
    let var = self.var_mut(id);
    var.set_maybe_assigned();
    var.set_is_used();
    Ok(id)
  }

  /// catch 绑定变量声明 (原版 Scope::DeclareCatchVariableName)
  pub fn declare_catch_variable_name(&mut self, scope: ScopeId, name: AstRawString) -> VariableId {
    debug_assert!(self.is_catch_scope(scope));
    let (id, was_added) = self.declare_local(
      scope,
      name,
      VariableMode::Var,
      VariableKind::NormalVariable,
      InitializationFlag::CreatedInitialized,
    );
    debug_assert!(was_added);
    self.var_mut(id).set_is_used();
    self.scope_mut(scope).catch_variable = Some(id);
    id
  }

  /// catch 绑定变量 (原版 catch_variable())
  pub fn catch_variable(&self, scope: ScopeId) -> Option<VariableId> {
    self.scope(scope).catch_variable
  }

  /// 非局部动态变量 (原版 Scope::NonLocal; 直接分配 LOOKUP 槽)
  pub fn non_local(
    &mut self,
    scope: ScopeId,
    name: AstRawString,
    mode: VariableMode,
  ) -> VariableId {
    debug_assert!(mode.is_dynamic());
    let (id, _) = self.declare(
      scope,
      name,
      mode,
      VariableKind::NormalVariable,
      InitializationFlag::CreatedInitialized,
      MaybeAssignedFlag::NotAssigned,
    );
    self.var_mut(id).allocate_to(VariableLocation::Lookup, -1);
    id
  }

  /// 动态全局声明 (原版 DeclarationScope::DeclareDynamicGlobal)
  fn declare_dynamic_global(&mut self, scope: ScopeId, name: AstRawString) -> VariableId {
    let (id, _) = self.declare(
      scope,
      name,
      VariableMode::DynamicGlobal,
      VariableKind::NormalVariable,
      InitializationFlag::CreatedInitialized,
      MaybeAssignedFlag::NotAssigned,
    );
    id
  }

  /// 形参声明 (原版 DeclarationScope::DeclareParameter)
  pub fn declare_parameter(
    &mut self,
    scope: ScopeId,
    name: AstRawString,
    mode: VariableMode,
    kind: VariableKind,
  ) -> VariableId {
    let (id, _) = self.declare_local(
      scope,
      name,
      mode,
      kind,
      InitializationFlag::CreatedInitialized,
    );
    self.scope_mut(scope).params.push(id);
    id
  }

  /// 形参表 (原版 params())
  pub fn params(&self, scope: ScopeId) -> &[VariableId] {
    &self.scope(scope).params
  }

  /// 函数名变量声明 (原版 DeclarationScope::DeclareFunctionVar)
  pub fn declare_function_var(&mut self, scope: ScopeId, name: AstRawString) -> Option<VariableId> {
    let (id, _) = self.declare(
      scope,
      name,
      VariableMode::Var,
      VariableKind::NormalVariable,
      InitializationFlag::NeedsInitialization,
      MaybeAssignedFlag::NotAssigned,
    );
    self.scope_mut(scope).function_var = Some(id);
    Some(id)
  }

  /// 生成器对象变量声明 (原版 DeclarationScope::DeclareGeneratorObjectVar)
  pub fn declare_generator_object_var(&mut self, scope: ScopeId, name: AstRawString) -> VariableId {
    let (id, _) = self.declare(
      scope,
      name,
      VariableMode::Var,
      VariableKind::NormalVariable,
      InitializationFlag::CreatedInitialized,
      MaybeAssignedFlag::NotAssigned,
    );
    self.scope_mut(scope).generator_object_var = Some(id);
    id
  }

  /// new.target 变量声明 (原版 DeclarationScope::DeclareNewTargetVar)
  pub fn declare_new_target_var(&mut self, scope: ScopeId, name: AstRawString) -> VariableId {
    let (id, _) = self.declare(
      scope,
      name,
      VariableMode::Const,
      VariableKind::NormalVariable,
      InitializationFlag::CreatedInitialized,
      MaybeAssignedFlag::NotAssigned,
    );
    self.scope_mut(scope).new_target_var = Some(id);
    id
  }

  /// new.target 变量 (原版 new_target())
  pub fn new_target_var(&self, scope: ScopeId) -> Option<VariableId> {
    self.scope(scope).new_target_var
  }

  /// this 声明 (原版 DeclarationScope::DeclareReceiver)
  fn declare_receiver(&mut self, scope: ScopeId) -> VariableId {
    let (id, _) = self.declare(
      scope,
      AstRawString::from_utf8("this"),
      VariableMode::Const,
      VariableKind::ThisVariable,
      InitializationFlag::CreatedInitialized,
      MaybeAssignedFlag::NotAssigned,
    );
    self.scope_mut(scope).receiver = Some(id);
    id
  }

  /// 临时变量 (原版 Scope::NewTemporary)
  pub fn new_temporary(&mut self, scope: ScopeId, name: AstRawString) -> VariableId {
    let (id, _) = self.declare(
      scope,
      name,
      VariableMode::Temporary,
      VariableKind::NormalVariable,
      InitializationFlag::CreatedInitialized,
      MaybeAssignedFlag::MaybeAssigned,
    );
    id
  }

  // ── 查找族 (原版 Lookup 族, kParsedScope 路径) ──

  /// 本地查找 (原版 VariableMap::Lookup / Scope::LookupLocal)
  pub fn lookup_local(&self, scope: ScopeId, name: &AstRawString) -> Option<VariableId> {
    self.scope(scope).variables.get(name).copied()
  }

  /// 沿外层链查找 (原版 Scope::Lookup<kParsedScope>)
  ///
  /// - with 作用域 → 外层命中后降级为动态 LOOKUP (LookupWith)
  /// - sloppy-eval 声明作用域 → 全局对象属性降级 DynamicGlobal (LookupSloppyEval)
  /// - 抵达 script 作用域未命中 → 声明动态全局 (DeclareDynamicGlobal)
  /// - 其余未命中 → None (自由变量)
  pub fn lookup(&mut self, scope: ScopeId, name: &AstRawString) -> Option<VariableId> {
    let mut cur = scope;
    loop {
      if let Some(id) = self.lookup_local(cur, name) {
        return Some(id);
      }
      let outer = match self.scope(cur).outer {
        Some(o) => o,
        None => break,
      };
      // with 作用域: 静态解析不可靠, 外层命中后转动态 (LookupWith)
      if self.is_with_scope(cur) {
        return match self.lookup(outer, name) {
          None => None,
          Some(id) => {
            if !self.var(id).is_dynamic() && self.var(id).is_unallocated() {
              let vid = self.var_mut(id);
              vid.set_is_used();
              vid.force_context_allocation();
            }
            Some(self.non_local(cur, name.clone(), VariableMode::Dynamic))
          }
        };
      }
      // sloppy eval 可扩展 var 的声明作用域: 全局对象属性降级 (LookupSloppyEval)
      if self.is_declaration_scope(cur)
        && self.scope(cur).sloppy_eval_can_extend_vars
        && !self.is_script_scope(cur)
      {
        return match self.lookup(outer, name) {
          None => None,
          Some(id) => {
            if self.is_global_object_property(id) {
              return Some(self.non_local(cur, name.clone(), VariableMode::DynamicGlobal));
            }
            if self.var(id).is_dynamic() {
              return Some(id);
            }
            Some(self.non_local(cur, name.clone(), VariableMode::DynamicLocal))
          }
        };
      }
      cur = outer;
    }
    // 抵达根仍未命中
    if !self.is_script_scope(cur) {
      return None;
    }
    Some(self.declare_dynamic_global(cur, name.clone()))
  }

  // ── 未消解引用与消解 (原版 unresolved_list_ + Resolve 族) ──

  /// 登记未消解引用 (原版 Scope::AddUnresolved)
  pub fn add_unresolved(
    &mut self,
    scope: ScopeId,
    name: AstRawString,
    position: i32,
    is_assigned: bool,
  ) {
    self.scope_mut(scope).unresolved.push(UnresolvedRef {
      name,
      position,
      is_assigned,
    });
  }

  /// 移除未消解引用 (原版 Scope::DeleteUnresolved; 按位置匹配首个)
  pub fn delete_unresolved(&mut self, scope: ScopeId, position: i32) -> bool {
    let s = self.scope_mut(scope);
    match s.unresolved.iter().position(|r| r.position == position) {
      Some(i) => {
        s.unresolved.remove(i);
        true
      }
      None => false,
    }
  }

  /// 未消解引用表 (原版 unresolved_list())
  pub fn unresolved(&self, scope: ScopeId) -> &[UnresolvedRef] {
    &self.scope(scope).unresolved
  }

  /// 消解单个引用 (原版 Scope::ResolveVariable: Lookup + is_used/maybe_assigned 回写)
  ///
  /// 返回绑定变量; script 根作用域保证命中 (动态全局回退)。
  pub fn resolve_variable(
    &mut self,
    scope: ScopeId,
    name: &AstRawString,
    is_assigned: bool,
  ) -> Option<VariableId> {
    let id = self.lookup(scope, name)?;
    {
      let var = self.var_mut(id);
      var.set_is_used();
      if is_assigned {
        var.set_maybe_assigned();
      }
    }
    Some(id)
  }

  /// 递归消解全部未消解引用 (原版 Scope::ResolveVariablesRecursively, 深度优先)
  pub fn resolve_variables_recursively(&mut self, scope: ScopeId) {
    let refs: Vec<UnresolvedRef> = self.scope(scope).unresolved.clone();
    for r in &refs {
      self.resolve_variable(scope, &r.name, r.is_assigned);
    }
    self.scope_mut(scope).unresolved.clear();
    let children = self.scope(scope).children.clone();
    for child in children {
      self.resolve_variables_recursively(child);
    }
  }

  // ── 变量分配族 (原版 Allocate 族) ──

  /// 是否需要分配 (原版 Scope::MustAllocate: 全局属性与未用变量跳过)
  fn must_allocate(&self, var: VariableId) -> bool {
    !self.is_global_object_property(var) && self.var(var).is_used()
  }

  /// 全局对象属性判定 (原版 Variable::IsGlobalObjectProperty)
  ///
  /// 动态/var 模式且归属 script 作用域。
  pub fn is_global_object_property(&self, var: VariableId) -> bool {
    let v = self.var(var);
    (v.mode().is_dynamic() || v.mode() == VariableMode::Var) && self.is_script_scope(v.scope())
  }

  /// 是否需分配到 context (原版 Scope::MustAllocateInContext)
  fn must_allocate_in_context(&self, scope: ScopeId, var: VariableId) -> bool {
    let mode = self.var(var).mode();
    if mode == VariableMode::Temporary {
      return false;
    }
    if self.is_catch_scope(scope) {
      return true;
    }
    if (self.is_script_scope(scope) || self.is_eval_scope(scope)) && mode.is_lexical() {
      return true;
    }
    self.var(var).has_forced_context_allocation() || self.scope(scope).inner_scope_calls_eval
  }

  /// 栈槽分配 (原版 Scope::AllocateStackSlot; 块作用域提升至声明作用域)
  fn allocate_stack_slot(&mut self, scope: ScopeId, var: VariableId) {
    let target = if self.is_block_scope(scope) {
      let outer = self.scope(scope).outer;
      match outer {
        Some(o) => self.get_declaration_scope(o),
        None => scope,
      }
    } else {
      scope
    };
    let slots = self.scope_mut(target).num_stack_slots;
    self
      .var_mut(var)
      .allocate_to(VariableLocation::Local, slots);
    self.scope_mut(target).num_stack_slots += 1;
  }

  /// context 槽分配 (原版 Scope::AllocateHeapSlot)
  fn allocate_heap_slot(&mut self, scope: ScopeId, var: VariableId) {
    let header = self.context_header_length(scope);
    let slots = self.scope(scope).num_heap_slots;
    self
      .var_mut(var)
      .allocate_to(VariableLocation::Context, header + slots);
    self.scope_mut(scope).num_heap_slots += 1;
  }

  /// eval 可能访问时保守标记读写 (原版 Scope::MarkMaybeAssignedIfEval)
  fn mark_maybe_assigned_if_eval(&mut self, scope: ScopeId, var: VariableId) {
    if !self.var(var).name().is_empty() && self.scope(scope).inner_scope_calls_eval {
      let v = self.var_mut(var);
      v.set_is_used();
      v.set_maybe_assigned();
    }
  }

  /// 单个非参数本地变量分配 (原版 Scope::AllocateNonParameterLocal)
  fn allocate_non_parameter_local(&mut self, scope: ScopeId, var: VariableId) {
    if !self.var(var).is_unallocated() {
      return;
    }
    self.mark_maybe_assigned_if_eval(scope, var);
    if !self.must_allocate(var) {
      return;
    }
    if self.must_allocate_in_context(scope, var) {
      self.allocate_heap_slot(scope, var);
    } else {
      self.allocate_stack_slot(scope, var);
    }
  }

  /// 形参分配 (原版 DeclarationScope::AllocateParameter)
  fn allocate_parameter(&mut self, scope: ScopeId, var: VariableId, index: i32) {
    self.mark_maybe_assigned_if_eval(scope, var);
    if self.must_allocate_in_context(scope, var) {
      if self.var(var).is_unallocated() {
        self.allocate_heap_slot(scope, var);
      }
    } else if self.var(var).is_unallocated() {
      self
        .var_mut(var)
        .allocate_to(VariableLocation::Parameter, index);
    }
  }

  /// 形参批量分配 (原版 DeclarationScope::AllocateParameterLocals; 倒序保持索引语义)
  fn allocate_parameter_locals(&mut self, scope: ScopeId) {
    let params = self.scope(scope).params.clone();
    for (i, var) in params.iter().rev().enumerate() {
      let index = (params.len() - 1 - i) as i32;
      self.allocate_parameter(scope, *var, index);
    }
  }

  /// receiver 分配 (原版 DeclarationScope::AllocateReceiver; 仅 this 声明作用域)
  fn allocate_receiver(&mut self, scope: ScopeId) {
    // 原版 has_this_declaration = (function && !arrow) || module
    if !self.is_function_scope(scope) && !self.is_module_scope(scope) {
      return;
    }
    let receiver = match self.scope(scope).receiver {
      Some(id) => id,
      None => self.declare_receiver(scope),
    };
    self.allocate_parameter(scope, receiver, -1);
  }

  /// 非参数本地批量分配 (原版 Scope::AllocateNonParameterLocalsAndDeclaredGlobals)
  fn allocate_non_parameter_locals_and_declared_globals(&mut self, scope: ScopeId) {
    let locals = self.scope(scope).locals.clone();
    for var in locals {
      self.allocate_non_parameter_local(scope, var);
    }
  }

  /// 递归变量分配 (原版 Scope::AllocateVariablesRecursively, ForEach 深度优先)
  ///
  /// 形参 → receiver → 本地; 若产生 context 槽则计入头长度。
  pub fn allocate_variables_recursively(&mut self, scope: ScopeId) {
    // 深度优先: 上游 ForEach 先本层后子树
    self.allocate_variables_scope(scope);
    let children = self.scope(scope).children.clone();
    for child in children {
      self.allocate_variables_recursively(child);
    }
  }

  /// 单作用域分配 (上游 ForEach 回调体)
  fn allocate_variables_scope(&mut self, scope: ScopeId) {
    if self.is_declaration_scope(scope) {
      self.allocate_receiver(scope);
      if self.is_function_scope(scope) {
        self.allocate_parameter_locals(scope);
      }
    }
    self.allocate_non_parameter_locals_and_declared_globals(scope);
    let heap = self.scope(scope).num_heap_slots;
    if heap > 0 || self.has_context_extension_slot(scope) {
      let header = self.context_header_length(scope);
      self.scope_mut(scope).num_heap_slots = header + heap;
    }
  }

  // ── 变量访问 (原版 Variable* 直接解引用) ──

  pub fn variable(&self, id: VariableId) -> &Variable {
    self.var(id)
  }

  pub fn variable_mut(&mut self, id: VariableId) -> &mut Variable {
    self.var_mut(id)
  }

  /// 作用域内变量名迭代 (原版 variables_ 遍历)
  pub fn variables(&self, scope: ScopeId) -> impl Iterator<Item = (&AstRawString, VariableId)> {
    self.scope(scope).variables.iter().map(|(k, v)| (k, *v))
  }

  /// 变量总数
  pub fn num_variables(&self, scope: ScopeId) -> usize {
    self.scope(scope).variables.len()
  }
}
