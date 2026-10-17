// 由 tool/src/torque/ast.js 从 v8/src/torque/ast.h 生成; 修订就地改, 复跑覆盖
// AST 节点体系全量 (60 kind 1:1)

use std::collections::{BTreeMap, BTreeSet};

use crate::{
  constants::{AbstractTypeFlags, ClassFlags, FieldSynchronization, StructFlags},
  source_positions::{SourceId, SourcePosition},
};

// 层级映射: C++ 继承树 → NodeId 引用 + AstNode enum。
// AstNode Kind 标签 → AstNodeKind; Expression/Statement/Declaration/TypeExpression
// 四个中间层以 is_* 谓词保留 (对应 AstNodeClassCheck::IsInstanceOf)。
// 原版 IntegerLiteral (src/numbers/integer-literal.h) 在此最小组件化。
/// 原版 IntegerLiteral 最小建模 (原版 src/numbers/integer-literal.h: 符号 + 十进制数字串)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerLiteral {
  pub negative: bool,
  pub digits: String,
}

/// AstNode::Kind 1:1 (60 变体; 变体名 = 节点类名)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AstNodeKind {
  CallExpression,
  CallMethodExpression,
  IntrinsicCallExpression,
  StructExpression,
  LogicalOrExpression,
  LogicalAndExpression,
  SpreadExpression,
  ConditionalExpression,
  IdentifierExpression,
  StringLiteralExpression,
  IntegerLiteralExpression,
  FloatingPointLiteralExpression,
  FieldAccessExpression,
  ElementAccessExpression,
  DereferenceExpression,
  AssignmentExpression,
  IncrementDecrementExpression,
  NewExpression,
  AssumeTypeImpossibleExpression,
  StatementExpression,
  TryLabelExpression,
  BasicTypeExpression,
  FunctionTypeExpression,
  PrecomputedTypeExpression,
  UnionTypeExpression,
  BlockStatement,
  ExpressionStatement,
  IfStatement,
  WhileStatement,
  TypeswitchStatement,
  ForLoopStatement,
  BreakStatement,
  ContinueStatement,
  ReturnStatement,
  DebugStatement,
  AssertStatement,
  TailCallStatement,
  VarDeclarationStatement,
  GotoStatement,
  AbstractTypeDeclaration,
  TypeAliasDeclaration,
  BitFieldStructDeclaration,
  ClassDeclaration,
  StructDeclaration,
  GenericCallableDeclaration,
  GenericTypeDeclaration,
  SpecializationDeclaration,
  ExternConstDeclaration,
  NamespaceDeclaration,
  ConstDeclaration,
  CppIncludeDeclaration,
  TorqueMacroDeclaration,
  TorqueBuiltinDeclaration,
  ExternalMacroDeclaration,
  ExternalBuiltinDeclaration,
  ExternalRuntimeDeclaration,
  IntrinsicDeclaration,
  Identifier,
  TryHandler,
  ClassBody,
}

impl AstNodeKind {
  /// 对应 AST_EXPRESSION_NODE_KIND_LIST
  #[must_use]
  pub fn is_expression(self) -> bool {
    matches!(
      self,
      Self::CallExpression
        | Self::CallMethodExpression
        | Self::IntrinsicCallExpression
        | Self::StructExpression
        | Self::LogicalOrExpression
        | Self::LogicalAndExpression
        | Self::SpreadExpression
        | Self::ConditionalExpression
        | Self::IdentifierExpression
        | Self::StringLiteralExpression
        | Self::IntegerLiteralExpression
        | Self::FloatingPointLiteralExpression
        | Self::FieldAccessExpression
        | Self::ElementAccessExpression
        | Self::DereferenceExpression
        | Self::AssignmentExpression
        | Self::IncrementDecrementExpression
        | Self::NewExpression
        | Self::AssumeTypeImpossibleExpression
        | Self::StatementExpression
        | Self::TryLabelExpression
    )
  }

  /// 对应 AST_TYPE_EXPRESSION_NODE_KIND_LIST
  #[must_use]
  pub fn is_type_expression(self) -> bool {
    matches!(
      self,
      Self::BasicTypeExpression
        | Self::FunctionTypeExpression
        | Self::PrecomputedTypeExpression
        | Self::UnionTypeExpression
    )
  }

  /// 对应 AST_STATEMENT_NODE_KIND_LIST
  #[must_use]
  pub fn is_statement(self) -> bool {
    matches!(
      self,
      Self::BlockStatement
        | Self::ExpressionStatement
        | Self::IfStatement
        | Self::WhileStatement
        | Self::TypeswitchStatement
        | Self::ForLoopStatement
        | Self::BreakStatement
        | Self::ContinueStatement
        | Self::ReturnStatement
        | Self::DebugStatement
        | Self::AssertStatement
        | Self::TailCallStatement
        | Self::VarDeclarationStatement
        | Self::GotoStatement
    )
  }

  /// 对应 AST_DECLARATION_NODE_KIND_LIST (首行展开 AST_TYPE_DECLARATION_NODE_KIND_LIST, 18 kind)
  #[must_use]
  pub fn is_declaration(self) -> bool {
    matches!(
      self,
      Self::AbstractTypeDeclaration
        | Self::TypeAliasDeclaration
        | Self::BitFieldStructDeclaration
        | Self::ClassDeclaration
        | Self::StructDeclaration
        | Self::GenericCallableDeclaration
        | Self::GenericTypeDeclaration
        | Self::SpecializationDeclaration
        | Self::ExternConstDeclaration
        | Self::NamespaceDeclaration
        | Self::ConstDeclaration
        | Self::CppIncludeDeclaration
        | Self::TorqueMacroDeclaration
        | Self::TorqueBuiltinDeclaration
        | Self::ExternalMacroDeclaration
        | Self::ExternalBuiltinDeclaration
        | Self::ExternalRuntimeDeclaration
        | Self::IntrinsicDeclaration
    )
  }

  /// CallableDeclaration 子树 (含宏/内建/运行时/内联函数/泛型/特化声明)
  #[must_use]
  pub fn is_callable(self) -> bool {
    matches!(
      self,
      Self::TorqueMacroDeclaration
        | Self::TorqueBuiltinDeclaration
        | Self::ExternalMacroDeclaration
        | Self::ExternalBuiltinDeclaration
        | Self::ExternalRuntimeDeclaration
        | Self::IntrinsicDeclaration
        | Self::GenericCallableDeclaration
        | Self::SpecializationDeclaration
    )
  }
}

/// IncrementDecrementOperator 1:1
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum IncrementDecrementOperator {
  Increment,
  Decrement,
}

/// ImplicitKind 1:1
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum ImplicitKind {
  #[default]
  NoImplicit,
  JsImplicit,
  Implicit,
}

/// IncludeSelector 1:1
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum IncludeSelector {
  Any,
  Csa,
  Tsa,
}

/// ConditionalAnnotationType 1:1
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum ConditionalAnnotationType {
  #[default]
  Positive,
  Negative,
}

/// NameAndExpression 1:1
#[derive(Debug, Clone, Default)]
pub struct NameAndExpression {
  pub name: NodeId,
  pub expression: NodeId,
}

/// TypeswitchCase 1:1
#[derive(Debug, Clone, Default)]
pub struct TypeswitchCase {
  pub pos: SourcePosition,
  pub name: Option<NodeId>,
  pub r#type: NodeId,
  pub block: NodeId,
}

/// ParameterList 1:1 (Default 语义 1:1, 对应原版 ParameterList::Empty)
#[derive(Debug, Clone, Default)]
pub struct ParameterList {
  pub names: Vec<NodeId>,
  pub types: Vec<NodeId>,
  pub implicit_kind: ImplicitKind,
  pub implicit_kind_pos: SourcePosition,
  pub implicit_count: usize,
  pub has_varargs: bool,
  pub arguments_variable: String,
}

/// NameAndTypeExpression 1:1
#[derive(Debug, Clone, Default)]
pub struct NameAndTypeExpression {
  pub name: NodeId,
  pub r#type: NodeId,
}

/// ImplicitParameters 1:1
#[derive(Debug, Clone, Default)]
pub struct ImplicitParameters {
  pub kind: NodeId,
  pub parameters: Vec<NameAndTypeExpression>,
}

/// StructFieldExpression 1:1
#[derive(Debug, Clone, Default)]
pub struct StructFieldExpression {
  pub name_and_type: NameAndTypeExpression,
  pub const_qualified: bool,
}

/// BitFieldDeclaration 1:1
#[derive(Debug, Clone, Default)]
pub struct BitFieldDeclaration {
  pub name_and_type: NameAndTypeExpression,
  pub num_bits: i32,
}

/// ConditionalAnnotation 1:1
#[derive(Debug, Clone, Default)]
pub struct ConditionalAnnotation {
  pub condition: String,
  pub r#type: ConditionalAnnotationType,
}

/// AnnotationParameter 1:1
#[derive(Debug, Clone, Default)]
pub struct AnnotationParameter {
  pub string_value: String,
  pub int_value: i32,
  pub is_int: bool,
}

/// Annotation 1:1
#[derive(Debug, Clone, Default)]
pub struct Annotation {
  pub name: NodeId,
  pub param: Option<AnnotationParameter>,
}

/// ClassFieldIndexInfo 1:1
#[derive(Debug, Clone, Default)]
pub struct ClassFieldIndexInfo {
  pub expr: NodeId,
  pub optional: bool,
}

/// ClassFieldExpression 1:1
#[derive(Debug, Clone, Default)]
pub struct ClassFieldExpression {
  pub name_and_type: NameAndTypeExpression,
  pub index: Option<ClassFieldIndexInfo>,
  pub conditions: Vec<ConditionalAnnotation>,
  pub custom_weak_marking: bool,
  pub const_qualified: bool,
  pub synchronization: FieldSynchronization,
}

/// LabelAndTypes 1:1
#[derive(Debug, Clone, Default)]
pub struct LabelAndTypes {
  pub name: NodeId,
  pub types: Vec<NodeId>,
}

/// GenericParameter 1:1
#[derive(Debug, Clone, Default)]
pub struct GenericParameter {
  pub name: NodeId,
  pub constraint: Option<NodeId>,
}

/// InstanceTypeConstraints 1:1 (原版默认 value=-1, num_flags_bits=-1)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstanceTypeConstraints {
  pub value: i32,
  pub num_flags_bits: i32,
}

impl Default for InstanceTypeConstraints {
  fn default() -> Self {
    Self {
      value: -1,
      num_flags_bits: -1,
    }
  }
}

/// 原版 EnumDescription::Entry 1:1
#[derive(Debug, Clone, Default)]
pub struct EnumEntryDesc {
  pub name: String,
  pub alias_entry: String,
}

/// EnumDescription 1:1 (嵌套 Entry 抽出为 EnumEntryDesc, 登记偏差)
#[derive(Debug, Clone, Default)]
pub struct EnumDescription {
  pub pos: SourcePosition,
  pub name: String,
  pub constexpr_generates: String,
  pub is_open: bool,
  pub entries: Vec<EnumEntryDesc>,
}

/// DebugKind (原版类内 enum) 1:1
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum DebugKind {
  Unreachable,
  Debug,
}

/// AssertKind (原版类内 enum) 1:1
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum AssertKind {
  Dcheck,
  Check,
  SbxCheck,
  StaticAssert,
}

/// HandlerKind (原版类内 enum) 1:1
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum HandlerKind {
  Catch,
  Label,
}

/// 每个节点共用头: kind 标签 + 源位置 (对应 C++ AstNode 基类)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeHeader {
  pub kind: AstNodeKind,
  pub pos: SourcePosition,
}

/// CallExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct CallExpression {
  pub callee: NodeId,
  pub arguments: Vec<NodeId>,
  pub labels: Vec<NodeId>,
}

/// CallMethodExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct CallMethodExpression {
  pub target: NodeId,
  pub method: NodeId,
  pub arguments: Vec<NodeId>,
  pub labels: Vec<NodeId>,
}

/// IntrinsicCallExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct IntrinsicCallExpression {
  pub name: NodeId,
  pub generic_arguments: Vec<NodeId>,
  pub arguments: Vec<NodeId>,
}

/// StructExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct StructExpression {
  pub r#type: NodeId,
  pub initializers: Vec<NameAndExpression>,
}

/// LogicalOrExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct LogicalOrExpression {
  pub left: NodeId,
  pub right: NodeId,
}

/// LogicalAndExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct LogicalAndExpression {
  pub left: NodeId,
  pub right: NodeId,
}

/// SpreadExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct SpreadExpression {
  pub spreadee: NodeId,
}

/// ConditionalExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct ConditionalExpression {
  pub condition: NodeId,
  pub if_true: NodeId,
  pub if_false: NodeId,
}

/// IdentifierExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct IdentifierExpression {
  pub namespace_qualification: Vec<String>,
  pub name: NodeId,
  pub generic_arguments: Vec<NodeId>,
}

/// StringLiteralExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct StringLiteralExpression {
  pub literal: String,
}

/// IntegerLiteralExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct IntegerLiteralExpression {
  pub value: IntegerLiteral,
}

/// FloatingPointLiteralExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct FloatingPointLiteralExpression {
  pub value: f64,
}

/// FieldAccessExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct FieldAccessExpression {
  pub object: NodeId,
  pub field: NodeId,
}

/// ElementAccessExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct ElementAccessExpression {
  pub array: NodeId,
  pub index: NodeId,
}

/// DereferenceExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct DereferenceExpression {
  pub reference: NodeId,
}

/// AssignmentExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct AssignmentExpression {
  pub location: NodeId,
  pub op: Option<String>,
  pub value: NodeId,
}

/// IncrementDecrementExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct IncrementDecrementExpression {
  pub location: NodeId,
  pub op: IncrementDecrementOperator,
  pub postfix: bool,
}

/// NewExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct NewExpression {
  pub r#type: NodeId,
  pub initializers: Vec<NameAndExpression>,
  pub pretenured: bool,
  pub clear_padding: bool,
}

/// AssumeTypeImpossibleExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct AssumeTypeImpossibleExpression {
  pub excluded_type: NodeId,
  pub expression: NodeId,
}

/// StatementExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct StatementExpression {
  pub statement: NodeId,
}

/// TryLabelExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct TryLabelExpression {
  pub try_expression: NodeId,
  pub label_block: NodeId,
}

/// BasicTypeExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct BasicTypeExpression {
  pub namespace_qualification: Vec<String>,
  pub is_constexpr: bool,
  pub name: NodeId,
  pub generic_arguments: Vec<NodeId>,
}

/// FunctionTypeExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct FunctionTypeExpression {
  pub parameters: Vec<NodeId>,
  pub return_type: NodeId,
}

/// PrecomputedTypeExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct PrecomputedTypeExpression {
  pub r#type: NodeId,
}

/// UnionTypeExpression 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct UnionTypeExpression {
  pub a: NodeId,
  pub b: NodeId,
}

/// BlockStatement 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct BlockStatement {
  pub deferred: bool,
  pub statements: Vec<NodeId>,
}

/// ExpressionStatement 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct ExpressionStatement {
  pub expression: NodeId,
}

/// IfStatement 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct IfStatement {
  pub condition: NodeId,
  pub is_constexpr: bool,
  pub if_true: NodeId,
  pub if_false: Option<NodeId>,
}

/// WhileStatement 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct WhileStatement {
  pub condition: NodeId,
  pub body: NodeId,
}

/// TypeswitchStatement 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct TypeswitchStatement {
  pub expr: NodeId,
  pub cases: Vec<TypeswitchCase>,
}

/// ForLoopStatement 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct ForLoopStatement {
  pub var_declaration: Option<NodeId>,
  pub test: Option<NodeId>,
  pub action: Option<NodeId>,
  pub body: NodeId,
}

/// BreakStatement 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct BreakStatement {}

/// ContinueStatement 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct ContinueStatement {}

/// ReturnStatement 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct ReturnStatement {
  pub value: Option<NodeId>,
}

/// DebugStatement 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct DebugStatement {
  pub kind: DebugKind,
}

/// AssertStatement 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct AssertStatement {
  pub kind: AssertKind,
  pub expression: NodeId,
  pub source: String,
}

/// TailCallStatement 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct TailCallStatement {
  pub call: NodeId,
}

/// VarDeclarationStatement 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct VarDeclarationStatement {
  pub const_qualified: bool,
  pub name: NodeId,
  pub r#type: Option<NodeId>,
  pub initializer: Option<NodeId>,
}

/// GotoStatement 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct GotoStatement {
  pub label: NodeId,
  pub arguments: Vec<NodeId>,
}

/// AbstractTypeDeclaration 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct AbstractTypeDeclaration {
  pub name: NodeId,
  pub flags: AbstractTypeFlags,
  pub extends: Option<NodeId>,
  pub generates: Option<String>,
}

/// TypeAliasDeclaration 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct TypeAliasDeclaration {
  pub name: NodeId,
  pub r#type: NodeId,
}

/// BitFieldStructDeclaration 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct BitFieldStructDeclaration {
  pub name: NodeId,
  pub parent: NodeId,
  pub fields: Vec<BitFieldDeclaration>,
  pub cpp_scope: Option<String>,
}

/// ClassDeclaration 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct ClassDeclaration {
  pub name: NodeId,
  pub flags: ClassFlags,
  pub super_class: NodeId,
  pub generates: Option<String>,
  pub methods: Vec<NodeId>,
  pub fields: Vec<ClassFieldExpression>,
  pub instance_type_constraints: InstanceTypeConstraints,
}

/// StructDeclaration 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct StructDeclaration {
  pub name: NodeId,
  pub flags: StructFlags,
  pub methods: Vec<NodeId>,
  pub fields: Vec<StructFieldExpression>,
}

/// GenericCallableDeclaration 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct GenericCallableDeclaration {
  pub generic_parameters: GenericParameters,
  pub declaration: NodeId,
}

/// GenericTypeDeclaration 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct GenericTypeDeclaration {
  pub generic_parameters: GenericParameters,
  pub declaration: NodeId,
}

/// SpecializationDeclaration 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct SpecializationDeclaration {
  pub transitioning: bool,
  pub name: NodeId,
  pub parameters: ParameterList,
  pub return_type: NodeId,
  pub labels: LabelAndTypesVector,
  pub generic_parameters: Vec<NodeId>,
  pub body: NodeId,
}

/// ExternConstDeclaration 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct ExternConstDeclaration {
  pub name: NodeId,
  pub r#type: NodeId,
  pub literal: String,
}

/// NamespaceDeclaration 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct NamespaceDeclaration {
  pub declarations: Vec<NodeId>,
  pub name: String,
}

/// ConstDeclaration 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct ConstDeclaration {
  pub name: NodeId,
  pub r#type: NodeId,
  pub expression: NodeId,
}

/// CppIncludeDeclaration 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct CppIncludeDeclaration {
  pub include_path: String,
  pub include_selector: IncludeSelector,
}

/// TorqueMacroDeclaration 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct TorqueMacroDeclaration {
  pub transitioning: bool,
  pub name: NodeId,
  pub parameters: ParameterList,
  pub return_type: NodeId,
  pub labels: LabelAndTypesVector,
  pub op: Option<String>,
  pub export_to_csa: bool,
  pub supports_tsa: bool,
  pub body: Option<NodeId>,
}

/// TorqueBuiltinDeclaration 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct TorqueBuiltinDeclaration {
  pub transitioning: bool,
  pub name: NodeId,
  pub parameters: ParameterList,
  pub return_type: NodeId,
  pub labels: LabelAndTypesVector,
  pub javascript_linkage: bool,
  pub has_custom_interface_descriptor: bool,
  pub supports_tsa: bool,
  pub use_counter_name: Option<String>,
  pub body: Option<NodeId>,
}

/// ExternalMacroDeclaration 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct ExternalMacroDeclaration {
  pub transitioning: bool,
  pub name: NodeId,
  pub parameters: ParameterList,
  pub return_type: NodeId,
  pub labels: LabelAndTypesVector,
  pub op: Option<String>,
  pub external_assembler_name: String,
}

/// ExternalBuiltinDeclaration 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct ExternalBuiltinDeclaration {
  pub transitioning: bool,
  pub name: NodeId,
  pub parameters: ParameterList,
  pub return_type: NodeId,
  pub labels: LabelAndTypesVector,
  pub javascript_linkage: bool,
}

/// ExternalRuntimeDeclaration 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct ExternalRuntimeDeclaration {
  pub transitioning: bool,
  pub name: NodeId,
  pub parameters: ParameterList,
  pub return_type: NodeId,
  pub labels: LabelAndTypesVector,
}

/// IntrinsicDeclaration 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct IntrinsicDeclaration {
  pub transitioning: bool,
  pub name: NodeId,
  pub parameters: ParameterList,
  pub return_type: NodeId,
  pub labels: LabelAndTypesVector,
}

/// Identifier 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct Identifier {
  pub value: String,
}

/// TryHandler 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct TryHandler {
  pub handler_kind: HandlerKind,
  pub label: NodeId,
  pub parameters: ParameterList,
  pub body: NodeId,
}

/// ClassBody 1:1 (指针字段 → NodeId)
#[derive(Debug, Clone)]
pub struct ClassBody {
  pub methods: Vec<NodeId>,
  pub fields: Vec<ClassFieldExpression>,
}

/// 原版 using GenericParameters = std::vector<GenericParameter>
pub type GenericParameters = Vec<GenericParameter>;

/// 原版 using LabelAndTypesVector = std::vector<LabelAndTypes>
pub type LabelAndTypesVector = Vec<LabelAndTypes>;

/// 节点索引: 对应 C++ AstNode*
pub type NodeId = u32;

/// 全部叶子节点合拢; 类别层级用 AstNodeKind 谓词查询 (Box 控制枚举尺寸)
#[derive(Debug, Clone)]
pub enum AstNode {
  CallExpression(Box<CallExpression>),
  CallMethodExpression(Box<CallMethodExpression>),
  IntrinsicCallExpression(Box<IntrinsicCallExpression>),
  StructExpression(Box<StructExpression>),
  LogicalOrExpression(Box<LogicalOrExpression>),
  LogicalAndExpression(Box<LogicalAndExpression>),
  SpreadExpression(Box<SpreadExpression>),
  ConditionalExpression(Box<ConditionalExpression>),
  IdentifierExpression(Box<IdentifierExpression>),
  StringLiteralExpression(Box<StringLiteralExpression>),
  IntegerLiteralExpression(Box<IntegerLiteralExpression>),
  FloatingPointLiteralExpression(Box<FloatingPointLiteralExpression>),
  FieldAccessExpression(Box<FieldAccessExpression>),
  ElementAccessExpression(Box<ElementAccessExpression>),
  DereferenceExpression(Box<DereferenceExpression>),
  AssignmentExpression(Box<AssignmentExpression>),
  IncrementDecrementExpression(Box<IncrementDecrementExpression>),
  NewExpression(Box<NewExpression>),
  AssumeTypeImpossibleExpression(Box<AssumeTypeImpossibleExpression>),
  StatementExpression(Box<StatementExpression>),
  TryLabelExpression(Box<TryLabelExpression>),
  BasicTypeExpression(Box<BasicTypeExpression>),
  FunctionTypeExpression(Box<FunctionTypeExpression>),
  PrecomputedTypeExpression(Box<PrecomputedTypeExpression>),
  UnionTypeExpression(Box<UnionTypeExpression>),
  BlockStatement(Box<BlockStatement>),
  ExpressionStatement(Box<ExpressionStatement>),
  IfStatement(Box<IfStatement>),
  WhileStatement(Box<WhileStatement>),
  TypeswitchStatement(Box<TypeswitchStatement>),
  ForLoopStatement(Box<ForLoopStatement>),
  BreakStatement(Box<BreakStatement>),
  ContinueStatement(Box<ContinueStatement>),
  ReturnStatement(Box<ReturnStatement>),
  DebugStatement(Box<DebugStatement>),
  AssertStatement(Box<AssertStatement>),
  TailCallStatement(Box<TailCallStatement>),
  VarDeclarationStatement(Box<VarDeclarationStatement>),
  GotoStatement(Box<GotoStatement>),
  AbstractTypeDeclaration(Box<AbstractTypeDeclaration>),
  TypeAliasDeclaration(Box<TypeAliasDeclaration>),
  BitFieldStructDeclaration(Box<BitFieldStructDeclaration>),
  ClassDeclaration(Box<ClassDeclaration>),
  StructDeclaration(Box<StructDeclaration>),
  GenericCallableDeclaration(Box<GenericCallableDeclaration>),
  GenericTypeDeclaration(Box<GenericTypeDeclaration>),
  SpecializationDeclaration(Box<SpecializationDeclaration>),
  ExternConstDeclaration(Box<ExternConstDeclaration>),
  NamespaceDeclaration(Box<NamespaceDeclaration>),
  ConstDeclaration(Box<ConstDeclaration>),
  CppIncludeDeclaration(Box<CppIncludeDeclaration>),
  TorqueMacroDeclaration(Box<TorqueMacroDeclaration>),
  TorqueBuiltinDeclaration(Box<TorqueBuiltinDeclaration>),
  ExternalMacroDeclaration(Box<ExternalMacroDeclaration>),
  ExternalBuiltinDeclaration(Box<ExternalBuiltinDeclaration>),
  ExternalRuntimeDeclaration(Box<ExternalRuntimeDeclaration>),
  IntrinsicDeclaration(Box<IntrinsicDeclaration>),
  Identifier(Box<Identifier>),
  TryHandler(Box<TryHandler>),
  ClassBody(Box<ClassBody>),
}

impl AstNode {
  /// 节点 kind 标签 (原版 AstNode::kind(), 供 arena 头登记)
  #[must_use]
  pub fn kind(&self) -> AstNodeKind {
    match self {
      Self::CallExpression(_) => AstNodeKind::CallExpression,
      Self::CallMethodExpression(_) => AstNodeKind::CallMethodExpression,
      Self::IntrinsicCallExpression(_) => AstNodeKind::IntrinsicCallExpression,
      Self::StructExpression(_) => AstNodeKind::StructExpression,
      Self::LogicalOrExpression(_) => AstNodeKind::LogicalOrExpression,
      Self::LogicalAndExpression(_) => AstNodeKind::LogicalAndExpression,
      Self::SpreadExpression(_) => AstNodeKind::SpreadExpression,
      Self::ConditionalExpression(_) => AstNodeKind::ConditionalExpression,
      Self::IdentifierExpression(_) => AstNodeKind::IdentifierExpression,
      Self::StringLiteralExpression(_) => AstNodeKind::StringLiteralExpression,
      Self::IntegerLiteralExpression(_) => AstNodeKind::IntegerLiteralExpression,
      Self::FloatingPointLiteralExpression(_) => AstNodeKind::FloatingPointLiteralExpression,
      Self::FieldAccessExpression(_) => AstNodeKind::FieldAccessExpression,
      Self::ElementAccessExpression(_) => AstNodeKind::ElementAccessExpression,
      Self::DereferenceExpression(_) => AstNodeKind::DereferenceExpression,
      Self::AssignmentExpression(_) => AstNodeKind::AssignmentExpression,
      Self::IncrementDecrementExpression(_) => AstNodeKind::IncrementDecrementExpression,
      Self::NewExpression(_) => AstNodeKind::NewExpression,
      Self::AssumeTypeImpossibleExpression(_) => AstNodeKind::AssumeTypeImpossibleExpression,
      Self::StatementExpression(_) => AstNodeKind::StatementExpression,
      Self::TryLabelExpression(_) => AstNodeKind::TryLabelExpression,
      Self::BasicTypeExpression(_) => AstNodeKind::BasicTypeExpression,
      Self::FunctionTypeExpression(_) => AstNodeKind::FunctionTypeExpression,
      Self::PrecomputedTypeExpression(_) => AstNodeKind::PrecomputedTypeExpression,
      Self::UnionTypeExpression(_) => AstNodeKind::UnionTypeExpression,
      Self::BlockStatement(_) => AstNodeKind::BlockStatement,
      Self::ExpressionStatement(_) => AstNodeKind::ExpressionStatement,
      Self::IfStatement(_) => AstNodeKind::IfStatement,
      Self::WhileStatement(_) => AstNodeKind::WhileStatement,
      Self::TypeswitchStatement(_) => AstNodeKind::TypeswitchStatement,
      Self::ForLoopStatement(_) => AstNodeKind::ForLoopStatement,
      Self::BreakStatement(_) => AstNodeKind::BreakStatement,
      Self::ContinueStatement(_) => AstNodeKind::ContinueStatement,
      Self::ReturnStatement(_) => AstNodeKind::ReturnStatement,
      Self::DebugStatement(_) => AstNodeKind::DebugStatement,
      Self::AssertStatement(_) => AstNodeKind::AssertStatement,
      Self::TailCallStatement(_) => AstNodeKind::TailCallStatement,
      Self::VarDeclarationStatement(_) => AstNodeKind::VarDeclarationStatement,
      Self::GotoStatement(_) => AstNodeKind::GotoStatement,
      Self::AbstractTypeDeclaration(_) => AstNodeKind::AbstractTypeDeclaration,
      Self::TypeAliasDeclaration(_) => AstNodeKind::TypeAliasDeclaration,
      Self::BitFieldStructDeclaration(_) => AstNodeKind::BitFieldStructDeclaration,
      Self::ClassDeclaration(_) => AstNodeKind::ClassDeclaration,
      Self::StructDeclaration(_) => AstNodeKind::StructDeclaration,
      Self::GenericCallableDeclaration(_) => AstNodeKind::GenericCallableDeclaration,
      Self::GenericTypeDeclaration(_) => AstNodeKind::GenericTypeDeclaration,
      Self::SpecializationDeclaration(_) => AstNodeKind::SpecializationDeclaration,
      Self::ExternConstDeclaration(_) => AstNodeKind::ExternConstDeclaration,
      Self::NamespaceDeclaration(_) => AstNodeKind::NamespaceDeclaration,
      Self::ConstDeclaration(_) => AstNodeKind::ConstDeclaration,
      Self::CppIncludeDeclaration(_) => AstNodeKind::CppIncludeDeclaration,
      Self::TorqueMacroDeclaration(_) => AstNodeKind::TorqueMacroDeclaration,
      Self::TorqueBuiltinDeclaration(_) => AstNodeKind::TorqueBuiltinDeclaration,
      Self::ExternalMacroDeclaration(_) => AstNodeKind::ExternalMacroDeclaration,
      Self::ExternalBuiltinDeclaration(_) => AstNodeKind::ExternalBuiltinDeclaration,
      Self::ExternalRuntimeDeclaration(_) => AstNodeKind::ExternalRuntimeDeclaration,
      Self::IntrinsicDeclaration(_) => AstNodeKind::IntrinsicDeclaration,
      Self::Identifier(_) => AstNodeKind::Identifier,
      Self::TryHandler(_) => AstNodeKind::TryHandler,
      Self::ClassBody(_) => AstNodeKind::ClassBody,
    }
  }
}

/// Ast 容器: C++ 用 unique_ptr 存活 + 裸指针互联, Rust 改 arena (Vec + NodeId)
#[derive(Debug, Default)]
pub struct Ast {
  pub declarations: Vec<NodeId>,
  pub nodes: Vec<(NodeHeader, AstNode)>,
  pub declared_imports: BTreeMap<SourceId, BTreeSet<SourceId>>,
  pub enum_descriptions: Vec<EnumDescription>,
}

impl Ast {
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  /// 原版 Ast::AddNode: 入 arena, 返回 NodeId (kind/pos 随 header 登记)
  pub fn add_node(&mut self, header: NodeHeader, node: AstNode) -> NodeId {
    self.nodes.push((header, node));
    (self.nodes.len() - 1) as NodeId
  }

  #[must_use]
  pub fn node(&self, id: NodeId) -> &(NodeHeader, AstNode) {
    &self.nodes[id as usize]
  }

  /// 原版 DeclareImportForCurrentFile (CurrentSourcePosition 上下文 → 显式传参)
  pub fn declare_import_for_current_file(&mut self, current: SourceId, import_id: SourceId) {
    self
      .declared_imports
      .entry(current)
      .or_default()
      .insert(import_id);
  }

  /// 原版 AddEnumDescription
  pub fn add_enum_description(&mut self, description: EnumDescription) {
    debug_assert!(!description.name.is_empty());
    self.enum_descriptions.push(description);
  }

  #[must_use]
  pub fn enum_descriptions(&self) -> &[EnumDescription] {
    &self.enum_descriptions
  }
}

// 原版自由构造函数 (MakeNode/MakeCallExpression/MakeFieldAccessExpression 等) 经
// CurrentAst + CurrentSourcePosition 上下文建节点; Rust 版由 parser 动作持有
// &mut Ast + SourcePosition 显式构造, 不设独立自由函数 (登记偏差)。

// 原版 kThisParameterName
pub const K_THIS_PARAMETER_NAME: &str = "this";
