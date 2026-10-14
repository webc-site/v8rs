// 由 tool/src/torque/declarable.js 从 v8/src/torque/declarable.h 生成; 修订就地改, 复跑覆盖
// Declarable 体系全量 (12 kind + 载荷 + Scope 存储)

use std::{cell::Cell, collections::BTreeMap, fmt};

use crate::{
  ast::{Ast, AstNode, GenericParameters, NodeId},
  context::{
    CurrentScopeGuard, SourcePositionScope, current_scope, current_source_position, report_error,
    with_type_oracle_mut,
  },
  error::{Error, Result},
  global_context::GlobalContext,
  source_positions::SourcePosition,
  type_inference::TypeArgumentInference,
  type_oracle::TypeOracle,
  type_visitor::TypeVisitor,
  types::{ParameterTypes, Signature, TypeData, TypeId, TypeVector, VisitResult, is_subtype_of},
  utils::K_TEST_NAMESPACE_NAME,
};

// 继承树映射: Declarable(kind) → DeclarableData 十二变体; Scope 存储抽为 ScopeData
// (Namespace 与 Callable 共有); C++ 模板 GenericDeclarable<S,D> 去模板, 两个具体类
// 各自持特化表; 特化键 TypeVector 用 BTreeMap 保确定性序。
/// Declarable::Kind 1:1 (12 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DeclarableKind {
  Namespace,
  TorqueMacro,
  ExternMacro,
  Method,
  Builtin,
  RuntimeFunction,
  Intrinsic,
  GenericCallable,
  GenericType,
  TypeAlias,
  ExternConstant,
  NamespaceConstant,
}

/// 声明 arena 索引 (对应 C++ Declarable*)
pub type DeclarableId = u32;

/// 原版 QualifiedName 1:1
#[derive(Debug, Clone, Default)]
pub struct QualifiedName {
  pub namespace_qualification: Vec<String>,
  pub name: String,
}

impl QualifiedName {
  /// 原版 explicit QualifiedName(name) — 无限定名
  #[must_use]
  pub fn plain(name: String) -> Self {
    Self {
      namespace_qualification: Vec::new(),
      name,
    }
  }

  /// 原版 Parse (declarable.cc:19-29): 按 "::" 逐段切分, 余下为 name
  #[must_use]
  pub fn parse(qualified_name: &str) -> Self {
    // v8: src/torque/declarable.cc QualifiedName::Parse
    let mut qualifications = Vec::new();
    let mut rest = qualified_name;
    while let Some(index) = rest.find("::") {
      qualifications.push(rest[..index].to_owned());
      rest = &rest[index + 2..];
    }
    Self {
      namespace_qualification: qualifications,
      name: rest.to_owned(),
    }
  }

  #[must_use]
  pub fn has_namespace_qualification(&self) -> bool {
    !self.namespace_qualification.is_empty()
  }

  /// 原版 DropFirstNamespaceQualification
  #[must_use]
  pub fn drop_first_namespace_qualification(&self) -> Self {
    Self {
      namespace_qualification: self
        .namespace_qualification
        .iter()
        .skip(1)
        .cloned()
        .collect(),
      name: self.name.clone(),
    }
  }
}

/// 原版 operator<<(QualifiedName&) (declarable.cc:31-36): "q1::q2::name"
impl fmt::Display for QualifiedName {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // v8: src/torque/declarable.cc operator<<(const QualifiedName&)
    for qualifier in &self.namespace_qualification {
      write!(f, "{qualifier}::")?;
    }
    write!(f, "{}", self.name)
  }
}

/// 原版 SpecializationRequester 1:1 (特化成因, 供错误报告)
#[derive(Debug, Clone, Default)]
pub struct SpecializationRequester {
  pub position: SourcePosition,
  pub scope: Option<DeclarableId>,
  pub name: String,
}

impl SpecializationRequester {
  /// 原版 None()
  #[must_use]
  pub fn none() -> Self {
    Self {
      position: SourcePosition::invalid(),
      scope: None,
      name: String::new(),
    }
  }

  #[must_use]
  pub fn is_none(&self) -> bool {
    self.position == SourcePosition::invalid() && self.scope.is_none() && self.name.is_empty()
  }
}

/// Builtin::Kind 1:1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum BuiltinKind {
  #[default]
  Stub,
  FixedArgsJavaScript,
  VarArgsJavaScript,
}

/// Builtin::Flag 位标志 1:1 (base::Flags → u32)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum BuiltinFlag {
  None = 0,
  CustomInterfaceDescriptor = 1 << 0,
  SupportsTsa = 1 << 1,
}

/// 原版 using Builtin::Flags = base::Flags<Builtin::Flag>
pub type BuiltinFlags = u32;

/// 原版 OutputType 1:1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum OutputType {
  #[default]
  Csa,
  CcDebug,
}

/// 原版 Scope 私有存储 (declarations_ + requester_), Namespace/Callable 共有
#[derive(Debug, Clone, Default)]
pub struct ScopeData {
  pub declarations: BTreeMap<String, Vec<DeclarableId>>,
  pub requester: SpecializationRequester,
}

/// 原版 Callable 私有字段 (external_name/readable_name/signature/returns/body)
#[derive(Debug, Clone, Default)]
pub struct CallableCommon {
  pub external_name: String,
  pub readable_name: String,
  pub signature: Signature,
  pub returns: usize,
  pub body: Option<NodeId>,
}

/// 原版 Macro 私有字段 (used/supports_tsa)
#[derive(Debug, Clone, Default)]
pub struct MacroCommon {
  pub callable: CallableCommon,
  pub used: bool,
  pub supports_tsa: bool,
}

/// 原版 Value 私有字段 (type/name/value)
#[derive(Debug, Clone, Default)]
pub struct ValueCommon {
  pub type_: Option<TypeId>,
  pub name: NodeId,
  pub value: Option<VisitResult>,
}

/// 按变体载荷 (C++ 派生类私有字段 1:1)
#[derive(Debug, Clone)]
pub enum DeclarableData {
  Namespace(NamespaceData),
  TorqueMacro(TorqueMacroData),
  ExternMacro(ExternMacroData),
  Method(MethodData),
  Builtin(BuiltinData),
  RuntimeFunction(RuntimeFunctionData),
  Intrinsic(IntrinsicData),
  GenericCallable(GenericCallableData),
  GenericType(GenericTypeData),
  TypeAlias(TypeAliasData),
  ExternConstant(ExternConstantData),
  NamespaceConstant(NamespaceConstantData),
}

/// 原版 Namespace 1:1
#[derive(Debug, Clone, Default)]
pub struct NamespaceData {
  pub name: String,
}

/// 原版 TorqueMacro 1:1
#[derive(Debug, Clone, Default)]
pub struct TorqueMacroData {
  pub macro_: MacroCommon,
  pub exported_to_csa: bool,
}

/// 原版 ExternMacro 1:1
#[derive(Debug, Clone, Default)]
pub struct ExternMacroData {
  pub macro_: MacroCommon,
  pub external_assembler_name: String,
}

/// 原版 Method 1:1 (aggregate_type 指针 → TypeId)
#[derive(Debug, Clone, Default)]
pub struct MethodData {
  pub macro_: MacroCommon,
  pub aggregate_type: Option<TypeId>,
}

/// 原版 Builtin 1:1
#[derive(Debug, Clone, Default)]
pub struct BuiltinData {
  pub callable: CallableCommon,
  pub kind: BuiltinKind,
  pub flags: BuiltinFlags,
  pub use_counter_name: Option<String>,
}

/// 原版 RuntimeFunction 1:1
#[derive(Debug, Clone, Default)]
pub struct RuntimeFunctionData {
  pub callable: CallableCommon,
}

/// 原版 Intrinsic 1:1
#[derive(Debug, Clone, Default)]
pub struct IntrinsicData {
  pub callable: CallableCommon,
}

/// 原版 GenericCallableDeclarable (模板 GenericDeclarable<Callable*, ...> 去模板)
#[derive(Debug, Clone, Default)]
pub struct GenericCallableData {
  pub name: String,
  pub declaration: NodeId,
  pub specializations: BTreeMap<TypeVector, DeclarableId>,
  pub constraints: Option<Vec<TypeConstraint>>,
}

/// 原版 GenericTypeDeclarable (模板 GenericDeclarable<const Type*, ...> 去模板)
#[derive(Debug, Clone, Default)]
pub struct GenericTypeData {
  pub name: String,
  pub declaration: NodeId,
  pub specializations: BTreeMap<TypeVector, TypeId>,
  pub constraints: Option<Vec<TypeConstraint>>,
}

/// 原版 TypeAlias 1:1
/// 登记偏差: 原版 `mutable` 成员 (being_resolved_/delayed_/type_) → Cell,
/// 使 Resolve 可在 &GlobalContext 下惰性回填 (对齐 C++ const 方法语义)。
#[derive(Debug, Clone, Default)]
pub struct TypeAliasData {
  pub being_resolved: Cell<bool>,
  pub delayed: Cell<Option<NodeId>>,
  pub type_: Cell<Option<TypeId>>,
  pub redeclaration: bool,
  pub declaration_position: SourcePosition,
}

/// 原版 ExternConstant 1:1
#[derive(Debug, Clone, Default)]
pub struct ExternConstantData {
  pub value: ValueCommon,
}

/// 原版 NamespaceConstant 1:1
#[derive(Debug, Clone, Default)]
pub struct NamespaceConstantData {
  pub value: ValueCommon,
  pub external_name: String,
  pub body: NodeId,
}

/// Declarable 公共头 (kind + 作用域链 + 源位置 + 数据载荷) 1:1
#[derive(Debug, Clone)]
pub struct Declarable {
  pub kind: DeclarableKind,
  /// 原版 parent_scope_ (构造时取 CurrentScope::Get() → 由创建方显式传入)
  pub parent_scope: Option<DeclarableId>,
  pub position: SourcePosition,
  pub identifier_position: SourcePosition,
  pub is_user_defined: bool,
  /// IsScope() (Namespace 或 Callable) 为真时持有 Scope 存储
  pub scope: Option<ScopeData>,
  pub data: DeclarableData,
}

impl Default for Declarable {
  fn default() -> Self {
    Self {
      kind: DeclarableKind::Namespace,
      parent_scope: None,
      position: SourcePosition::invalid(),
      identifier_position: SourcePosition::invalid(),
      is_user_defined: true,
      scope: None,
      data: DeclarableData::Namespace(NamespaceData::default()),
    }
  }
}

impl Declarable {
  // 原 IsXxx 谓词 1:1 (全部由 kind 判别)
  #[must_use]
  pub fn is_namespace(&self) -> bool {
    self.kind == DeclarableKind::Namespace
  }

  /// 原版 IsMacro: TorqueMacro 或 ExternMacro
  #[must_use]
  pub fn is_macro(&self) -> bool {
    self.is_torque_macro() || self.is_extern_macro()
  }

  /// 原版 IsTorqueMacro: kTorqueMacro 或 kMethod
  #[must_use]
  pub fn is_torque_macro(&self) -> bool {
    matches!(
      self.kind,
      DeclarableKind::TorqueMacro | DeclarableKind::Method
    )
  }

  #[must_use]
  pub fn is_method(&self) -> bool {
    self.kind == DeclarableKind::Method
  }

  #[must_use]
  pub fn is_extern_macro(&self) -> bool {
    self.kind == DeclarableKind::ExternMacro
  }

  #[must_use]
  pub fn is_intrinsic(&self) -> bool {
    self.kind == DeclarableKind::Intrinsic
  }

  #[must_use]
  pub fn is_builtin(&self) -> bool {
    self.kind == DeclarableKind::Builtin
  }

  #[must_use]
  pub fn is_runtime_function(&self) -> bool {
    self.kind == DeclarableKind::RuntimeFunction
  }

  #[must_use]
  pub fn is_generic_callable(&self) -> bool {
    self.kind == DeclarableKind::GenericCallable
  }

  #[must_use]
  pub fn is_generic_type(&self) -> bool {
    self.kind == DeclarableKind::GenericType
  }

  #[must_use]
  pub fn is_type_alias(&self) -> bool {
    self.kind == DeclarableKind::TypeAlias
  }

  #[must_use]
  pub fn is_extern_constant(&self) -> bool {
    self.kind == DeclarableKind::ExternConstant
  }

  #[must_use]
  pub fn is_namespace_constant(&self) -> bool {
    self.kind == DeclarableKind::NamespaceConstant
  }

  /// 原版 IsValue: ExternConstant 或 NamespaceConstant
  #[must_use]
  pub fn is_value(&self) -> bool {
    self.is_extern_constant() || self.is_namespace_constant()
  }

  /// 原版 IsCallable: 全部可调用类别
  #[must_use]
  pub fn is_callable(&self) -> bool {
    matches!(
      self.kind,
      DeclarableKind::TorqueMacro
        | DeclarableKind::ExternMacro
        | DeclarableKind::Method
        | DeclarableKind::Builtin
        | DeclarableKind::RuntimeFunction
        | DeclarableKind::Intrinsic
    )
  }

  /// 原版 IsScope: Namespace 或 Callable (scope 存储是否在位)
  #[must_use]
  pub fn is_scope(&self) -> bool {
    self.is_namespace() || self.is_callable()
  }

  /// 原版 Position
  #[must_use]
  pub fn position(&self) -> SourcePosition {
    self.position
  }

  /// 原版 SetPosition
  pub fn set_position(&mut self, position: SourcePosition) {
    self.position = position;
  }

  /// 原版 IdentifierPosition: 无标识符位置时回退整体位置
  #[must_use]
  pub fn identifier_position(&self) -> SourcePosition {
    if self.identifier_position.source.is_valid() {
      self.identifier_position
    } else {
      self.position
    }
  }

  /// 原版 SetIdentifierPosition
  pub fn set_identifier_position(&mut self, position: SourcePosition) {
    self.identifier_position = position;
  }

  /// 原版 PrefixNameForCCOutput
  #[must_use]
  pub fn prefix_name_for_cc_output(name: &str) -> String {
    format!("TqRuntime{name}")
  }

  /// 原版 PrefixNameForCCDebugOutput
  #[must_use]
  pub fn prefix_name_for_cc_debug_output(name: &str) -> String {
    format!("TqDebug{name}")
  }
}

/// 原版 Scope::LookupShallow (declarable.h:178-193, 限定名浅查)。
/// 填体定形: arena 上下文显式传 cx + scope id。
pub fn lookup_shallow(
  cx: &GlobalContext,
  scope: DeclarableId,
  name: &QualifiedName,
) -> Result<Vec<DeclarableId>> {
  // v8: src/torque/declarable.h Scope::LookupShallow
  if !name.has_namespace_qualification() {
    // 原版 declarations_[name.name] 缺键默认空表, 语义等价返回空
    let d = cx.declarable(scope);
    let scope_data = d
      .scope
      .as_ref()
      .ok_or_else(|| Error::Message("internal: non-scope declarable lookup".into()))?;
    return Ok(
      scope_data
        .declarations
        .get(&name.name)
        .cloned()
        .unwrap_or_default(),
    );
  }
  let qualifier = name.namespace_qualification[0].clone();
  let mut child: Option<DeclarableId> = None;
  let list = {
    let d = cx.declarable(scope);
    d.scope
      .as_ref()
      .and_then(|s| s.declarations.get(&qualifier))
      .cloned()
      .unwrap_or_default()
  };
  for declarable in list {
    if cx.declarable(declarable).is_scope() {
      if child.is_some() {
        return Err(report_error(format!(
          "ambiguous reference to scope {qualifier}"
        )));
      }
      child = Some(declarable);
    }
  }
  let Some(child) = child else {
    return Ok(Vec::new());
  };
  lookup_shallow(cx, child, &name.drop_first_namespace_qualification())
}

/// 原版 Scope::Lookup (declarable.cc:91-105, 深查: 先递归父链再浅查本级)。
/// 填体定形: arena 上下文显式传 cx + scope id。
pub fn scope_lookup(
  cx: &GlobalContext,
  scope: DeclarableId,
  name: &QualifiedName,
) -> Result<Vec<DeclarableId>> {
  // v8: src/torque/declarable.cc Scope::Lookup
  if name
    .namespace_qualification
    .first()
    .is_some_and(String::is_empty)
  {
    // 原版: 首限定符为空串 → 从默认命名空间起查并丢弃首限定符
    let default_namespace = cx
      .default_namespace()
      .ok_or_else(|| Error::Message("internal: default namespace missing".into()))?;
    return scope_lookup(
      cx,
      default_namespace,
      &name.drop_first_namespace_qualification(),
    );
  }
  let parent = cx.declarable(scope).parent_scope;
  let mut result = match parent {
    Some(parent) => scope_lookup(cx, parent, name)?,
    None => Vec::new(),
  };
  result.extend(lookup_shallow(cx, scope, name)?);
  Ok(result)
}

/// 原版 CurrentNamespace (declarable.h:231-239, 沿父链找 Namespace) —
/// 上下文 → CurrentScope thread_local + 显式传 cx
pub fn current_namespace(cx: &GlobalContext) -> Result<DeclarableId> {
  // v8: src/torque/declarable.h CurrentNamespace
  let mut scope = current_scope()
    .ok_or_else(|| Error::Message("internal: CurrentScope::Get() is null".into()))?;
  loop {
    let declarable = cx.declarable(scope);
    if declarable.is_namespace() {
      return Ok(scope);
    }
    scope = declarable
      .parent_scope
      .ok_or_else(|| Error::Message("internal: scope chain has no Namespace root".into()))?;
  }
}

/// 原版 TypeConstraint 1:1
#[derive(Debug, Clone, Default)]
pub struct TypeConstraint {
  pub upper_bound: Option<TypeId>,
}

impl TypeConstraint {
  /// 原版 Unconstrained
  #[must_use]
  pub fn unconstrained() -> Self {
    Self { upper_bound: None }
  }

  /// 原版 SubtypeConstraint
  #[must_use]
  pub fn subtype_constraint(upper_bound: TypeId) -> Self {
    Self {
      upper_bound: Some(upper_bound),
    }
  }

  /// 原版 IsViolated (declarable.cc:107-117)。填体定形: 类型显示需 cx + oracle
  pub fn is_violated(
    &self,
    cx: &GlobalContext,
    oracle: &TypeOracle,
    type_: TypeId,
  ) -> Result<Option<String>> {
    // v8: src/torque/declarable.cc TypeConstraint::IsViolated
    let Some(upper_bound) = self.upper_bound else {
      return Ok(None);
    };
    if is_subtype_of(oracle, type_, upper_bound) {
      return Ok(None);
    }
    let t = &oracle.types[type_ as usize];
    if t.is_top_type() {
      // 原版 TopType::cast(type)->reason()
      match &t.data {
        TypeData::Top(top) => Ok(Some(top.reason.clone())),
        _ => Err(Error::Message("internal: top type payload mismatch".into())),
      }
    } else {
      Ok(Some(format!(
        "expected {} to be a subtype of {}",
        t.to_display_string(cx, oracle),
        oracle.types[upper_bound as usize].to_display_string(cx, oracle)
      )))
    }
  }
}

/// 原版 FindConstraintViolation (declarable.cc:119-129)。填体定形: 需 cx + oracle 显示类型
pub fn find_constraint_violation(
  cx: &GlobalContext,
  oracle: &TypeOracle,
  types: &TypeVector,
  constraints: &[TypeConstraint],
) -> Result<Option<String>> {
  // v8: src/torque/declarable.cc FindConstraintViolation
  debug_assert_eq!(constraints.len(), types.len());
  for (i, type_) in types.iter().enumerate() {
    if let Some(violation) = constraints[i].is_violated(cx, oracle, *type_)? {
      return Ok(Some(format!("Could not instantiate generic, {violation}.")));
    }
  }
  Ok(None)
}

/// 原版 ComputeConstraints (declarable.cc:131-144, scope 上下文 → CurrentScope 桥)。
/// 填体定形: oracle 显式可变传入 (TypeVisitor::ComputeType 需 interning)
pub fn compute_constraints(
  oracle: &mut TypeOracle,
  scope: Option<DeclarableId>,
  parameters: &GenericParameters,
) -> Result<Vec<TypeConstraint>> {
  // v8: src/torque/declarable.cc ComputeConstraints
  let _scope_guard = CurrentScopeGuard::new(scope);
  let mut result = Vec::new();
  for parameter in parameters {
    if let Some(constraint) = parameter.constraint {
      result.push(TypeConstraint::subtype_constraint(
        TypeVisitor::compute_type(oracle, constraint)?,
      ));
    } else {
      result.push(TypeConstraint::unconstrained());
    }
  }
  Ok(result)
}

// ── 构造/登记辅助 (原版构造函数字段初始化 + Scope::AddDeclarable) ──

/// 原版 Declarable::IsScope 的 kind 级判别 (供 make_declarable 决定是否建 Scope 存储)
#[must_use]
pub const fn kind_is_scope(kind: DeclarableKind) -> bool {
  // v8: src/torque/declarable.h Declarable::IsScope
  matches!(
    kind,
    DeclarableKind::Namespace
      | DeclarableKind::TorqueMacro
      | DeclarableKind::ExternMacro
      | DeclarableKind::Method
      | DeclarableKind::Builtin
      | DeclarableKind::RuntimeFunction
      | DeclarableKind::Intrinsic
  )
}

/// 原版 Declarable 构造的隐式上下文注入: parent_scope_ = CurrentScope::Get(),
/// position_ = CurrentSourcePosition::Get(); IsScope kind 附带 Scope 存储。
#[must_use]
pub fn make_declarable(
  kind: DeclarableKind,
  data: DeclarableData,
  is_user_defined: bool,
) -> Declarable {
  // v8: src/torque/declarable.h Declarable (protected ctor)
  Declarable {
    kind,
    parent_scope: current_scope(),
    position: current_source_position(),
    identifier_position: SourcePosition::invalid(),
    is_user_defined,
    scope: kind_is_scope(kind).then(ScopeData::default),
    data,
  }
}

/// 原版 Scope::AddDeclarable (declarable.h:197-200): 名字表追加
pub fn add_declarable(
  cx: &mut GlobalContext,
  scope: DeclarableId,
  name: &str,
  declarable: DeclarableId,
) -> Result<()> {
  // v8: src/torque/declarable.h Scope::AddDeclarable
  cx.declarable_mut(scope)
    .scope
    .as_mut()
    .ok_or_else(|| Error::Message("internal: AddDeclarable on non-scope".into()))?
    .declarations
    .entry(name.to_owned())
    .or_default()
    .push(declarable);
  Ok(())
}

/// 原版 SpecializationRequester 构造 (declarable.cc:82-89):
/// 跳过与特化无关的 scope, 沿父链找首个持有 requester 的 Scope
#[must_use]
pub fn make_specialization_requester(
  cx: &GlobalContext,
  position: SourcePosition,
  scope: Option<DeclarableId>,
  name: String,
) -> SpecializationRequester {
  // v8: src/torque/declarable.cc SpecializationRequester::SpecializationRequester
  let mut current = scope;
  while let Some(id) = current {
    let none = cx
      .declarable(id)
      .scope
      .as_ref()
      .is_none_or(|s| s.requester.is_none());
    if !none {
      break;
    }
    current = cx.declarable(id).parent_scope;
  }
  SpecializationRequester {
    position,
    scope: current,
    name,
  }
}

/// 原版 Scope::SetSpecializationRequester (declarable.h:205-207, 含签名行; 复审 TQR5 OBS-T5-1 订正)
pub fn set_specialization_requester(
  cx: &mut GlobalContext,
  scope: DeclarableId,
  requester: &SpecializationRequester,
) -> Result<()> {
  // v8: src/torque/declarable.h Scope::SetSpecializationRequester
  cx.declarable_mut(scope)
    .scope
    .as_mut()
    .ok_or_else(|| Error::Message("internal: requester on non-scope".into()))?
    .requester = requester.clone();
  Ok(())
}

/// 原版 Namespace::IsDefaultNamespace (declarable.cc:175-177)
#[must_use]
pub fn namespace_is_default(cx: &GlobalContext, id: DeclarableId) -> bool {
  // v8: src/torque/declarable.cc Namespace::IsDefaultNamespace
  cx.default_namespace() == Some(id)
}

/// 原版 Namespace::IsTestNamespace (declarable.cc:179)
#[must_use]
pub fn namespace_is_test(data: &NamespaceData) -> bool {
  // v8: src/torque/declarable.cc Namespace::IsTestNamespace
  data.name == K_TEST_NAMESPACE_NAME
}

/// 类型声明节点 (5 kind) 的 name 标识符取值 (供报错文案, 对齐 decl->name->value)
#[must_use]
pub fn type_declaration_name(cx: &GlobalContext, decl: NodeId) -> String {
  // v8: src/torque/ast.h TypeDeclaration::name
  let name_node = match &cx.ast.node(decl).1 {
    AstNode::AbstractTypeDeclaration(d) => d.name,
    AstNode::TypeAliasDeclaration(d) => d.name,
    AstNode::BitFieldStructDeclaration(d) => d.name,
    AstNode::ClassDeclaration(d) => d.name,
    AstNode::StructDeclaration(d) => d.name,
    _ => return String::new(),
  };
  identifier_value(&cx.ast, name_node)
}

/// Identifier 节点取字符串 (原版 identifier->value)
#[must_use]
pub fn identifier_value(ast: &Ast, id: NodeId) -> String {
  match &ast.node(id).1 {
    AstNode::Identifier(identifier) => identifier.value.clone(),
    _ => String::new(),
  }
}

/// 原版 TypeAlias::type()/Resolve (declarable.h:680-683 + declarable.cc:181-195):
/// 惰性解析别名。Cell 回填对齐 C++ mutable 成员, 可在 &GlobalContext 下调用。
pub fn resolve_type_alias(cx: &GlobalContext, id: DeclarableId) -> Result<TypeId> {
  // v8: src/torque/declarable.cc TypeAlias::Resolve
  let alias = cx.declarable(id);
  let DeclarableData::TypeAlias(data) = &alias.data else {
    return Err(Error::Message("internal: Resolve on non-TypeAlias".into()));
  };
  if let Some(type_) = data.type_.get() {
    return Ok(type_);
  }
  let _scope_guard = CurrentScopeGuard::new(alias.parent_scope);
  let _position_guard = SourcePositionScope::new(alias.position);
  let Some(decl) = data.delayed.get() else {
    return Err(Error::Message(
      "internal: TypeAlias without delayed decl".into(),
    ));
  };
  if data.being_resolved.get() {
    // 登记偏差: 原版 being_resolved_ 上游从不置 true (仅读取), 保真不写位
    return Err(report_error(format!(
      "Cannot create type {} due to circular dependencies.",
      type_declaration_name(cx, decl)
    )));
  }
  let type_ =
    with_type_oracle_mut(|oracle| TypeVisitor::compute_type_decl(oracle, decl, None, None))?;
  data.type_.set(Some(type_));
  Ok(type_)
}

// ── GenericDeclarable 辅助 (declarable.h:588-642 模板去模板化) ──

/// 读 GenericCallable 载荷 (kind 判别)
fn generic_callable_data(cx: &GlobalContext, id: DeclarableId) -> Result<&GenericCallableData> {
  match &cx.declarable(id).data {
    DeclarableData::GenericCallable(g) => Ok(g),
    _ => Err(Error::Message("internal: not a GenericCallable".into())),
  }
}

/// 读 GenericType 载荷 (kind 判别)
fn generic_type_data(cx: &GlobalContext, id: DeclarableId) -> Result<&GenericTypeData> {
  match &cx.declarable(id).data {
    DeclarableData::GenericType(g) => Ok(g),
    _ => Err(Error::Message("internal: not a GenericType".into())),
  }
}

/// 原版 GenericDeclarable::generic_parameters(): 声明节点上的泛型参数表
pub fn generic_parameters(cx: &GlobalContext, generic: DeclarableId) -> Result<GenericParameters> {
  // v8: src/torque/declarable.h GenericDeclarable::generic_parameters
  if let Ok(data) = generic_callable_data(cx, generic) {
    if let AstNode::GenericCallableDeclaration(node) = &cx.ast.node(data.declaration).1 {
      return Ok(node.generic_parameters.clone());
    }
  } else {
    let data = generic_type_data(cx, generic)?;
    if let AstNode::GenericTypeDeclaration(node) = &cx.ast.node(data.declaration).1 {
      return Ok(node.generic_parameters.clone());
    }
  }
  Err(Error::Message(
    "internal: generic declaration node kind mismatch".into(),
  ))
}

/// 原版 GenericDeclarable::declaration() (declarable.h:616):
/// 存储的是外层泛型节点 (generic_declaration_), 需再取内层 declaration 字段
pub fn generic_declaration_node(cx: &GlobalContext, generic: DeclarableId) -> Result<NodeId> {
  // v8: src/torque/declarable.h GenericDeclarable::declaration
  let outer = if let Ok(data) = generic_callable_data(cx, generic) {
    data.declaration
  } else {
    generic_type_data(cx, generic)?.declaration
  };
  Ok(match &cx.ast.node(outer).1 {
    AstNode::GenericCallableDeclaration(d) => d.declaration,
    AstNode::GenericTypeDeclaration(d) => d.declaration,
    _ => {
      return Err(Error::Message(
        "internal: generic storage node is not a generic declaration".into(),
      ));
    }
  })
}

/// 原版 GenericDeclarable::name()
pub fn generic_name(cx: &GlobalContext, generic: DeclarableId) -> Result<String> {
  if let Ok(data) = generic_callable_data(cx, generic) {
    return Ok(data.name.clone());
  }
  Ok(generic_type_data(cx, generic)?.name.clone())
}

/// 原版 GenericDeclarable::Constraints (declarable.h:621-626): 惰性计算 + 缓存
pub fn generic_constraints(
  cx: &mut GlobalContext,
  oracle: &mut TypeOracle,
  generic: DeclarableId,
) -> Result<Vec<TypeConstraint>> {
  // v8: src/torque/declarable.h GenericDeclarable::Constraints
  let cached = match &cx.declarable(generic).data {
    DeclarableData::GenericCallable(g) => g.constraints.clone(),
    DeclarableData::GenericType(g) => g.constraints.clone(),
    _ => {
      return Err(Error::Message(
        "internal: Constraints on non-generic".into(),
      ));
    }
  };
  if let Some(constraints) = cached {
    return Ok(constraints);
  }
  let parent_scope = cx.declarable(generic).parent_scope;
  let parameters = generic_parameters(cx, generic)?;
  let result = compute_constraints(oracle, parent_scope, &parameters)?;
  let d = cx.declarable_mut(generic);
  match &mut d.data {
    DeclarableData::GenericCallable(g) => g.constraints = Some(result.clone()),
    DeclarableData::GenericType(g) => g.constraints = Some(result.clone()),
    _ => {}
  }
  Ok(result)
}

/// 原版 GenericDeclarable::AddSpecialization (declarable.h:595-603) — Callable 实例
pub fn add_callable_specialization(
  cx: &mut GlobalContext,
  oracle: &mut TypeOracle,
  generic: DeclarableId,
  type_arguments: &TypeVector,
  specialization: DeclarableId,
) -> Result<()> {
  // v8: src/torque/declarable.h GenericDeclarable::AddSpecialization
  let existing = generic_callable_data(cx, generic)?
    .specializations
    .contains_key(type_arguments);
  debug_assert!(!existing);
  let constraints = generic_constraints(cx, oracle, generic)?;
  if let Some(violation) = find_constraint_violation(cx, &*oracle, type_arguments, &constraints)? {
    // 原版 Error(*violation).Throw()
    return Err(report_error(violation));
  }
  generic_callable_data_mut(cx, generic)?
    .specializations
    .insert(type_arguments.clone(), specialization);
  Ok(())
}

/// 原版 GenericDeclarable::GetSpecialization (declarable.h:604-609) — Callable 实例
#[must_use]
pub fn get_callable_specialization(
  cx: &GlobalContext,
  generic: DeclarableId,
  type_arguments: &TypeVector,
) -> Option<DeclarableId> {
  // v8: src/torque/declarable.h GenericDeclarable::GetSpecialization
  generic_callable_data(cx, generic)
    .ok()?
    .specializations
    .get(type_arguments)
    .copied()
}

/// 原版 GenericDeclarable::AddSpecialization — Type 实例 (GenericType)
pub fn add_type_specialization(
  cx: &mut GlobalContext,
  oracle: &mut TypeOracle,
  generic: DeclarableId,
  type_arguments: &TypeVector,
  specialization: TypeId,
) -> Result<()> {
  // v8: src/torque/declarable.h GenericDeclarable::AddSpecialization
  let existing = generic_type_data(cx, generic)?
    .specializations
    .contains_key(type_arguments);
  debug_assert!(!existing);
  let constraints = generic_constraints(cx, oracle, generic)?;
  if let Some(violation) = find_constraint_violation(cx, &*oracle, type_arguments, &constraints)? {
    return Err(report_error(violation));
  }
  let d = cx.declarable_mut(generic);
  let DeclarableData::GenericType(g) = &mut d.data else {
    return Err(Error::Message("internal: not a GenericType".into()));
  };
  g.specializations
    .insert(type_arguments.clone(), specialization);
  Ok(())
}

/// 原版 GenericDeclarable::GetSpecialization — Type 实例 (GenericType)
#[must_use]
pub fn get_type_specialization(
  cx: &GlobalContext,
  generic: DeclarableId,
  type_arguments: &TypeVector,
) -> Option<TypeId> {
  // v8: src/torque/declarable.h GenericDeclarable::GetSpecialization
  generic_type_data(cx, generic)
    .ok()?
    .specializations
    .get(type_arguments)
    .copied()
}

fn generic_callable_data_mut(
  cx: &mut GlobalContext,
  id: DeclarableId,
) -> Result<&mut GenericCallableData> {
  match &mut cx.declarable_mut(id).data {
    DeclarableData::GenericCallable(g) => Ok(g),
    _ => Err(Error::Message("internal: not a GenericCallable".into())),
  }
}

/// 原版 GenericCallable::InferSpecializationTypes (declarable.cc:146-162)。
/// 填体定形: cx/oracle 显式传参; term_parameters 为类型表达式 NodeId。
pub fn infer_specialization_types(
  cx: &mut GlobalContext,
  oracle: &mut TypeOracle,
  generic: DeclarableId,
  explicit_specialization_types: &TypeVector,
  arguments: &[Option<TypeId>],
) -> Result<TypeArgumentInference> {
  // v8: src/torque/declarable.cc GenericCallable::InferSpecializationTypes
  let parent_scope = cx.declarable(generic).parent_scope;
  let _scope_guard = CurrentScopeGuard::new(parent_scope);
  let declaration = generic_declaration_node(cx, generic)?;
  let term_parameters = callable_declaration_parameter_types(cx, declaration)?;
  let generic_params = generic_parameters(cx, generic)?;
  let mut inference = TypeArgumentInference::new(
    &generic_params,
    explicit_specialization_types,
    &term_parameters,
    arguments,
  );
  if !inference.has_failed() {
    // 原版 Constraints(): 惰性算 + 落缓存 (此处 cx 可变, 忠实缓存)
    let constraints = generic_constraints(cx, oracle, generic)?;
    // 登记偏差: 原版 GetResult() 未填体前以 Err 上抛 (类型段收敛)
    let result = inference.get_result()?;
    if let Some(violation) = find_constraint_violation(cx, &*oracle, &result, &constraints)? {
      inference.fail(violation);
    }
  }
  Ok(inference)
}

/// CallableDeclaration (泛型内层) 的参数类型表达式表 (原版 declaration()->parameters.types)
pub fn callable_declaration_parameter_types(
  cx: &GlobalContext,
  declaration: NodeId,
) -> Result<Vec<NodeId>> {
  // v8: src/torque/declarable.cc GenericCallable::InferSpecializationTypes (L149-150)
  let node = &cx.ast.node(declaration).1;
  let parameters = match node {
    AstNode::TorqueMacroDeclaration(d) => &d.parameters,
    AstNode::TorqueBuiltinDeclaration(d) => &d.parameters,
    AstNode::ExternalMacroDeclaration(d) => &d.parameters,
    AstNode::ExternalBuiltinDeclaration(d) => &d.parameters,
    AstNode::ExternalRuntimeDeclaration(d) => &d.parameters,
    AstNode::IntrinsicDeclaration(d) => &d.parameters,
    _ => {
      return Err(Error::Message("internal: not a CallableDeclaration".into()));
    }
  };
  Ok(parameters.types.clone())
}

/// 原版 GenericCallable::CallableBody (declarable.cc:164-173)
pub fn callable_body(cx: &GlobalContext, generic: DeclarableId) -> Result<Option<NodeId>> {
  // v8: src/torque/declarable.cc GenericCallable::CallableBody
  let declaration = generic_declaration_node(cx, generic)?;
  Ok(match &cx.ast.node(declaration).1 {
    AstNode::TorqueMacroDeclaration(d) => d.body,
    AstNode::TorqueBuiltinDeclaration(d) => d.body,
    _ => None,
  })
}

/// 原版内层声明是否为 IntrinsicDeclaration (declaration-visitor 判定用)
#[must_use]
pub fn is_intrinsic_declaration(cx: &GlobalContext, declaration: NodeId) -> bool {
  // v8: src/torque/declaration-visitor.cc IntrinsicDeclaration::DynamicCast
  matches!(cx.ast.node(declaration).1, AstNode::IntrinsicDeclaration(_))
}

/// 原版内层声明是否为 MacroDeclaration (CallableDeclaration 的 Macro 派生族)
#[must_use]
pub fn is_macro_declaration(cx: &GlobalContext, declaration: NodeId) -> bool {
  // v8: src/torque/declaration-visitor.cc MacroDeclaration::DynamicCast
  matches!(
    cx.ast.node(declaration).1,
    AstNode::TorqueMacroDeclaration(_) | AstNode::ExternalMacroDeclaration(_)
  )
}

/// 原版内层声明是否为 BuiltinDeclaration 族
#[must_use]
pub fn is_builtin_declaration(cx: &GlobalContext, declaration: NodeId) -> bool {
  // v8: src/torque/declaration-visitor.cc BuiltinDeclaration::cast
  matches!(
    cx.ast.node(declaration).1,
    AstNode::TorqueBuiltinDeclaration(_) | AstNode::ExternalBuiltinDeclaration(_)
  )
}

/// 读取 BuiltinDeclaration 族的关键标志 (declaration-visitor CreateBuiltin 分派所需,
/// 对应原版 BuiltinDeclaration 基类字段 javascript_linkage/parameters.has_varargs +
/// TorqueBuiltinDeclaration 子类 has_custom_interface_descriptor/supports_tsa/use_counter_name)
pub fn builtin_declaration_flags(
  cx: &GlobalContext,
  declaration: NodeId,
) -> Result<(bool, bool, bool, bool, Option<String>)> {
  // v8: src/torque/declaration-visitor.cc DeclarationVisitor::CreateBuiltin
  Ok(match &cx.ast.node(declaration).1 {
    AstNode::TorqueBuiltinDeclaration(d) => (
      d.javascript_linkage,
      d.parameters.has_varargs,
      d.has_custom_interface_descriptor,
      d.supports_tsa,
      d.use_counter_name.clone(),
    ),
    AstNode::ExternalBuiltinDeclaration(d) => (
      d.javascript_linkage,
      d.parameters.has_varargs,
      false,
      false,
      None,
    ),
    _ => return Err(Error::Message("internal: not a BuiltinDeclaration".into())),
  })
}

// ── 原版 operator<< 助手 (declarable.cc:31-80; 显示逻辑局部实现, 与类型段 PrintSignature 收敛) ──

/// 原版 operator<<(const TypeVector&) (types.cc:1129-1132): ", " 分隔
#[must_use]
pub fn display_type_vector(cx: &GlobalContext, oracle: &TypeOracle, types: &TypeVector) -> String {
  types
    .iter()
    .map(|t| oracle.types[*t as usize].to_display_string(cx, oracle))
    .collect::<Vec<_>>()
    .join(", ")
}

/// 原版 operator<<(const ParameterTypes&) (types.cc:1134-1141)
#[must_use]
pub fn display_parameter_types(
  cx: &GlobalContext,
  oracle: &TypeOracle,
  parameter_types: &ParameterTypes,
) -> String {
  let mut result = display_type_vector(cx, oracle, &parameter_types.types);
  if parameter_types.var_args {
    if !parameter_types.types.is_empty() {
      result.push_str(", ");
    }
    result.push_str("...");
  }
  result
}

fn return_type_display(cx: &GlobalContext, oracle: &TypeOracle, signature: &Signature) -> String {
  signature
    .return_type
    .map(|t| oracle.types[t as usize].to_display_string(cx, oracle))
    .unwrap_or_default()
}

/// Callable 族载荷的统一读取 (原版 Callable 基类字段; 供显示/比较助手)
#[must_use]
pub fn callable_common_of(d: &Declarable) -> Option<&CallableCommon> {
  match &d.data {
    DeclarableData::TorqueMacro(m) => Some(&m.macro_.callable),
    DeclarableData::ExternMacro(m) => Some(&m.macro_.callable),
    DeclarableData::Method(m) => Some(&m.macro_.callable),
    DeclarableData::Builtin(b) => Some(&b.callable),
    DeclarableData::RuntimeFunction(r) => Some(&r.callable),
    DeclarableData::Intrinsic(i) => Some(&i.callable),
    _ => None,
  }
}

/// 原版 operator<<(const Callable&) (declarable.cc:38-57)
pub fn display_callable(cx: &GlobalContext, oracle: &TypeOracle, d: &Declarable) -> Result<String> {
  // v8: src/torque/declarable.cc operator<<(const Callable&)
  let callable = callable_common_of(d)
    .ok_or_else(|| Error::Message("internal: stream on non-Callable".into()))?;
  let signature = &callable.signature;
  let mut result = format!("callable {}(", callable.readable_name);
  if signature.implicit_count != 0 {
    result.push_str("implicit ");
    let implicit: TypeVector = signature.types()[..signature.implicit_count].to_vec();
    let explicit: TypeVector = signature.types()[signature.implicit_count..].to_vec();
    result.push_str(&display_type_vector(cx, oracle, &implicit));
    result.push_str(")(");
    result.push_str(&display_type_vector(cx, oracle, &explicit));
  } else {
    result.push_str(&display_parameter_types(
      cx,
      oracle,
      &signature.parameter_types,
    ));
  }
  result.push_str("): ");
  result.push_str(&return_type_display(cx, oracle, signature));
  Ok(result)
}

/// 原版 operator<<(const Builtin&) (declarable.cc:59-63)
pub fn display_builtin(cx: &GlobalContext, oracle: &TypeOracle, d: &Declarable) -> Result<String> {
  // v8: src/torque/declarable.cc operator<<(const Builtin&)
  let DeclarableData::Builtin(b) = &d.data else {
    return Err(Error::Message("internal: stream on non-Builtin".into()));
  };
  Ok(format!(
    "builtin {} {}{}",
    return_type_display(cx, oracle, &b.callable.signature),
    b.callable.readable_name,
    display_parameter_types(cx, oracle, &b.callable.signature.parameter_types)
  ))
}

/// 原版 operator<<(const RuntimeFunction&) (declarable.cc:65-69)
pub fn display_runtime_function(
  cx: &GlobalContext,
  oracle: &TypeOracle,
  d: &Declarable,
) -> Result<String> {
  // v8: src/torque/declarable.cc operator<<(const RuntimeFunction&)
  let DeclarableData::RuntimeFunction(r) = &d.data else {
    return Err(Error::Message(
      "internal: stream on non-RuntimeFunction".into(),
    ));
  };
  Ok(format!(
    "runtime function {} {}{}",
    return_type_display(cx, oracle, &r.callable.signature),
    r.callable.readable_name,
    display_parameter_types(cx, oracle, &r.callable.signature.parameter_types)
  ))
}

/// 原版 operator<<(const GenericCallable&) (declarable.cc:71-80):
/// "generic name<T1, T2>" — 形参名经 PrintCommaSeparatedList 投影
pub fn display_generic_callable(cx: &GlobalContext, generic: DeclarableId) -> Result<String> {
  // v8: src/torque/declarable.cc operator<<(const GenericCallable&)
  let data = generic_callable_data(cx, generic)?;
  let names = generic_parameters(cx, generic)?;
  let joined = names
    .iter()
    .map(|p| identifier_value(&cx.ast, p.name))
    .collect::<Vec<_>>()
    .join(", ");
  Ok(format!("generic {}<{joined}>", data.name))
}

// ── 原版 src/torque/declarable.cc ──
// L19: QualifiedName QualifiedName::Parse(std::string_view qualified_name)
// L31: std::ostream& operator<<(std::ostream& os, const QualifiedName& name)
// L38: std::ostream& operator<<(std::ostream& os, const Callable& m)
// L59: std::ostream& operator<<(std::ostream& os, const Builtin& b)
// L65: std::ostream& operator<<(std::ostream& os, const RuntimeFunction& b)
// L71: std::ostream& operator<<(std::ostream& os, const GenericCallable& g)
// L82: SpecializationRequester::SpecializationRequester(SourcePosition position, Scope* s, std::string name) : position(position), name(std::move(name))
// L91: std::vector<Declarable*> Scope::Lookup(const QualifiedName& name)
// L107: std::optional<std::string> TypeConstraint::IsViolated(const Type* type) const
// L119: std::optional<std::string> FindConstraintViolation( const std::vector<const Type*>& types, const std::vector<TypeConstraint>& constraints)
// L131: std::vector<TypeConstraint> ComputeConstraints( Scope* scope, const GenericParameters& parameters)
// L146: TypeArgumentInference GenericCallable::InferSpecializationTypes( const TypeVector& explicit_specialization_types, const std::vector<std::optional<const Type*>>& arguments)
// L164: std::optional<Statement*> GenericCallable::CallableBody()
// L175: bool Namespace::IsDefaultNamespace() const
// L179: bool Namespace::IsTestNamespace() const
// L181: const Type* TypeAlias::Resolve() const
