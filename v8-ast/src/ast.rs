//! 核心 AST 节点结构体体系与 enum_dispatch 派发 (v8/src/ast/ast.h/.cc)
//!
//! 第二阶段: 节点字段按上游 ast.h 逐类补全 (FunctionLiteral/ClassLiteral/
//! VariableProxy/Assignment/Block/TryCatch 等), 工厂构造器覆盖全部 60 类节点,
//! 并接入 v8-zone 分配路径 (节点驻留 + ZoneList/ZoneVector 集合物化;
//! 避开 Snapshot/reset 族)。

use enum_dispatch::enum_dispatch;
use strum::{Display, FromRepr};
use v8_common::{LookupHoistingMode, VariableMode};
use v8_zone::{Zone, ZoneList, ZoneVector};

pub use crate::ast_generated::*;
use crate::{
  error::{Error, Result},
  modules_generated::ModuleImportPhase,
  scopes::{ScopeId, VariableId},
  value::{AstBigInt, AstConsString, AstRawString, AstValue},
};

/// AST 节点统一特征接口 (原版 class AstNode 抽象接口)
#[enum_dispatch]
pub trait AstNodeTrait {
  /// 节点类型枚举
  fn node_type(&self) -> AstNodeType;
  /// 源码起始位置
  fn position(&self) -> i32;
  /// 节点基类属性引用
  fn base(&self) -> &AstNodeBase;
}

macro_rules! impl_ast_node {
  ($ty:ident) => {
    impl AstNodeTrait for $ty {
      #[inline]
      fn node_type(&self) -> AstNodeType {
        AstNodeType::$ty
      }
      #[inline]
      fn position(&self) -> i32 {
        self.base.position
      }
      #[inline]
      fn base(&self) -> &AstNodeBase {
        &self.base
      }
    }
  };
}

// ── 第二阶段新增辅助枚举 (修订层; 上游定义于 ast.h / function-syntax-kind.h / handler-table.h) ──

/// 函数语法类别 (原版 enum class FunctionSyntaxKind, objects/function-syntax-kind.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
#[repr(u8)]
pub enum FunctionSyntaxKind {
  AnonymousExpression = 0,
  NamedExpression = 1,
  Declaration = 2,
  AccessorOrMethod = 3,
  Wrapped = 4,
}

impl FunctionSyntaxKind {
  /// 变体数 (kLastFunctionSyntaxKind + 1)
  pub const COUNT: usize = 5;

  /// 是否为匿名表达式 (原版 is_anonymous_expression)
  pub const fn is_anonymous_expression(self) -> bool {
    matches!(self, Self::AnonymousExpression)
  }
}

/// catch 预测 (原版 HandlerTable::CatchPrediction, codegen/handler-table.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
#[repr(u8)]
pub enum CatchPrediction {
  /// 处理器将 (很可能) 重抛异常
  Uncaught = 0,
  /// 异常将被处理器捕获
  Caught = 1,
  /// 异常将被捕获并导致 promise 拒绝
  Promise = 2,
  /// async 函数 desugar 中的 promise 拒绝 (调试器特殊处理)
  AsyncAwait = 3,
  /// async REPL 脚本的 promise 拒绝 (消息对象需保活)
  UncaughtAsyncAwait = 4,
}

/// 赋值操作符 (原版 Assignment 的 TokenField; 纯 '=' + 复合赋值 15 种)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
#[repr(u8)]
pub enum AstAssignOp {
  Assign = 0,
  AssignNullish,
  AssignOr,
  AssignAnd,
  AssignBitOr,
  AssignBitXor,
  AssignBitAnd,
  AssignShl,
  AssignSar,
  AssignShr,
  AssignMul,
  AssignDiv,
  AssignMod,
  AssignExp,
  AssignAdd,
  AssignSub,
}

impl AstAssignOp {
  /// 变体数
  pub const COUNT: usize = 16;

  /// 字符表示
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::Assign => "=",
      Self::AssignNullish => "??=",
      Self::AssignOr => "||=",
      Self::AssignAnd => "&&=",
      Self::AssignBitOr => "|=",
      Self::AssignBitXor => "^=",
      Self::AssignBitAnd => "&=",
      Self::AssignShl => "<<=",
      Self::AssignSar => ">>=",
      Self::AssignShr => ">>>=",
      Self::AssignMul => "*=",
      Self::AssignDiv => "/=",
      Self::AssignMod => "%=",
      Self::AssignExp => "**=",
      Self::AssignAdd => "+=",
      Self::AssignSub => "-=",
    }
  }

  /// 对应二元运算类别 (纯赋值无对应)
  pub const fn binary_op(&self) -> Option<AstBinaryOp> {
    let compound = match self {
      Self::Assign => return None,
      Self::AssignNullish => AstCompoundAssignOp::AssignNullish,
      Self::AssignOr => AstCompoundAssignOp::AssignOr,
      Self::AssignAnd => AstCompoundAssignOp::AssignAnd,
      Self::AssignBitOr => AstCompoundAssignOp::AssignBitOr,
      Self::AssignBitXor => AstCompoundAssignOp::AssignBitXor,
      Self::AssignBitAnd => AstCompoundAssignOp::AssignBitAnd,
      Self::AssignShl => AstCompoundAssignOp::AssignShl,
      Self::AssignSar => AstCompoundAssignOp::AssignSar,
      Self::AssignShr => AstCompoundAssignOp::AssignShr,
      Self::AssignMul => AstCompoundAssignOp::AssignMul,
      Self::AssignDiv => AstCompoundAssignOp::AssignDiv,
      Self::AssignMod => AstCompoundAssignOp::AssignMod,
      Self::AssignExp => AstCompoundAssignOp::AssignExp,
      Self::AssignAdd => AstCompoundAssignOp::AssignAdd,
      Self::AssignSub => AstCompoundAssignOp::AssignSub,
    };
    Some(compound.binary_op())
  }

  /// 从复合赋值操作符构造
  pub const fn from_compound(op: AstCompoundAssignOp) -> Self {
    match op {
      AstCompoundAssignOp::AssignNullish => Self::AssignNullish,
      AstCompoundAssignOp::AssignOr => Self::AssignOr,
      AstCompoundAssignOp::AssignAnd => Self::AssignAnd,
      AstCompoundAssignOp::AssignBitOr => Self::AssignBitOr,
      AstCompoundAssignOp::AssignBitXor => Self::AssignBitXor,
      AstCompoundAssignOp::AssignBitAnd => Self::AssignBitAnd,
      AstCompoundAssignOp::AssignShl => Self::AssignShl,
      AstCompoundAssignOp::AssignSar => Self::AssignSar,
      AstCompoundAssignOp::AssignShr => Self::AssignShr,
      AstCompoundAssignOp::AssignMul => Self::AssignMul,
      AstCompoundAssignOp::AssignDiv => Self::AssignDiv,
      AstCompoundAssignOp::AssignMod => Self::AssignMod,
      AstCompoundAssignOp::AssignExp => Self::AssignExp,
      AstCompoundAssignOp::AssignAdd => Self::AssignAdd,
      AstCompoundAssignOp::AssignSub => Self::AssignSub,
    }
  }
}

/// 预解析数据 (原版 ProducedPreparseData 的字节数组 + 子数据树形态)
///
/// 上游持有者在 v8-parsing (ConsumedPreparseData 对接); 此处落地字节与
/// 子函数数据的最小真实结构, 随解析器接入扩展。
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PreparseData {
  /// 字节化作用域数据 (原版 ByteData)
  pub byte_data: Vec<u8>,
  /// 子函数预解析数据 (原版 children_)
  pub children: Vec<PreparseData>,
}

impl PreparseData {
  pub fn new() -> Self {
    Self::default()
  }

  /// 是否含数据
  pub fn is_empty(&self) -> bool {
    self.byte_data.is_empty() && self.children.is_empty()
  }
}

// ── 辅助结构体 ──

/// Switch 分支子句
#[derive(Debug, Clone, PartialEq)]
pub struct CaseClause {
  pub label: Option<Box<AstNode>>,
  pub statements: Vec<AstNode>,
}

impl CaseClause {
  pub fn new(label: Option<AstNode>, statements: Vec<AstNode>) -> Self {
    Self {
      label: label.map(Box::new),
      statements,
    }
  }

  pub fn is_default(&self) -> bool {
    self.label.is_none()
  }
}

/// 对象字面量属性项 (原版 ObjectLiteralProperty)
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectLiteralProperty {
  pub kind: ObjectPropertyKind,
  pub key: Box<AstNode>,
  pub value: Box<AstNode>,
  pub is_computed: bool,
  /// 是否为该键的首次出现 (原版 is_first_instance_of_key_)
  pub is_first_instance_of_key: bool,
  /// 键末次出现索引 (原版 value_index_; 仅首次实例有效)
  pub last_instance_index: i32,
}

impl ObjectLiteralProperty {
  pub fn new(kind: ObjectPropertyKind, key: AstNode, value: AstNode, is_computed: bool) -> Self {
    Self {
      kind,
      key: Box::new(key),
      value: Box::new(value),
      is_computed,
      is_first_instance_of_key: true,
      last_instance_index: -1,
    }
  }

  /// 是否为 __proto__ 属性 (原版 IsPrototype)
  pub fn is_prototype(&self) -> bool {
    matches!(self.kind, ObjectPropertyKind::Prototype)
  }
}

/// 类字面量属性项 (原版 ClassLiteralProperty)
#[derive(Debug, Clone, PartialEq)]
pub struct ClassLiteralProperty {
  pub kind: ClassPropertyKind,
  pub key: Box<AstNode>,
  pub value: Box<AstNode>,
  pub is_static: bool,
  pub is_computed: bool,
  pub is_private: bool,
}

impl ClassLiteralProperty {
  pub fn new(
    kind: ClassPropertyKind,
    key: AstNode,
    value: AstNode,
    is_static: bool,
    is_computed: bool,
    is_private: bool,
  ) -> Self {
    Self {
      kind,
      key: Box::new(key),
      value: Box::new(value),
      is_static,
      is_computed,
      is_private,
    }
  }
}

/// 类静态元素项
#[derive(Debug, Clone, PartialEq)]
pub struct ClassLiteralStaticElement {
  pub kind: ClassStaticElementKind,
  pub property: Option<ClassLiteralProperty>,
  pub block: Option<Box<Block>>,
}

impl ClassLiteralStaticElement {
  pub fn property(prop: ClassLiteralProperty) -> Self {
    Self {
      kind: ClassStaticElementKind::Property,
      property: Some(prop),
      block: None,
    }
  }

  pub fn static_block(block: Block) -> Self {
    Self {
      kind: ClassStaticElementKind::StaticBlock,
      property: None,
      block: Some(Box::new(block)),
    }
  }
}

// ── 60 个核心节点定义 ──

/// 顶层函数字面量 id (原版 kFunctionLiteralIdTopLevel, v8/src/common/globals.h)
pub const FUNCTION_LITERAL_ID_TOP_LEVEL: i32 = 0;

/// 未分配的函数字面量 id (本 crate 约定哨兵, 分配前占位)
pub const FUNCTION_LITERAL_ID_UNASSIGNED: i32 = -1;

// 1. 声明节点
#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
  pub base: AstNodeBase,
  pub var: Option<AstRawString>,
  pub mode: VariableMode,
}
impl_ast_node!(VariableDeclaration);

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
  pub base: AstNodeBase,
  pub fun: Option<Box<FunctionLiteral>>,
}
impl_ast_node!(FunctionDeclaration);

// 2. 迭代语句
#[derive(Debug, Clone, PartialEq)]
pub struct DoWhileStatement {
  pub base: AstNodeBase,
  pub body: Box<AstNode>,
  pub cond: Box<AstNode>,
}
impl_ast_node!(DoWhileStatement);

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
  pub base: AstNodeBase,
  pub cond: Box<AstNode>,
  pub body: Box<AstNode>,
}
impl_ast_node!(WhileStatement);

#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
  pub base: AstNodeBase,
  pub init: Option<Box<AstNode>>,
  pub cond: Option<Box<AstNode>>,
  pub next: Option<Box<AstNode>>,
  pub body: Box<AstNode>,
}
impl_ast_node!(ForStatement);

#[derive(Debug, Clone, PartialEq)]
pub struct ForInStatement {
  pub base: AstNodeBase,
  pub each: Box<AstNode>,
  pub subject: Box<AstNode>,
  pub body: Box<AstNode>,
  /// subject 表达式的隐藏块作用域 (原版 subject_scope_; 调试器暂停语义)
  pub subject_scope: Option<ScopeId>,
}
impl_ast_node!(ForInStatement);

#[derive(Debug, Clone, PartialEq)]
pub struct ForOfStatement {
  pub base: AstNodeBase,
  pub each: Box<AstNode>,
  pub subject: Box<AstNode>,
  pub body: Box<AstNode>,
  pub iter_type: IteratorType,
  /// subject 表达式的隐藏块作用域 (原版 subject_scope_)
  pub subject_scope: Option<ScopeId>,
}
impl_ast_node!(ForOfStatement);

// 3. 可跳出语句
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
  pub base: AstNodeBase,
  pub statements: Vec<AstNode>,
  pub is_breakable: bool,
  /// 完成值是否被忽略 (原版 ignore_completion_value_)
  pub ignore_completion: bool,
  /// 参数初始化块 (原版 is_initialization_block_for_parameters_)
  pub is_initialization_block_for_parameters: bool,
  /// 块作用域 (原版 scope_; 解析器填充)
  pub scope: Option<ScopeId>,
}
impl_ast_node!(Block);

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchStatement {
  pub base: AstNodeBase,
  pub tag: Box<AstNode>,
  pub cases: Vec<CaseClause>,
}
impl_ast_node!(SwitchStatement);

// 4. 其他语句
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
  pub base: AstNodeBase,
  pub expression: Box<AstNode>,
}
impl_ast_node!(ExpressionStatement);

#[derive(Debug, Clone, PartialEq)]
pub struct EmptyStatement {
  pub base: AstNodeBase,
}
impl_ast_node!(EmptyStatement);

#[derive(Debug, Clone, PartialEq)]
pub struct SloppyBlockFunctionStatement {
  pub base: AstNodeBase,
  pub statement: Box<AstNode>,
}
impl_ast_node!(SloppyBlockFunctionStatement);

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
  pub base: AstNodeBase,
  pub condition: Box<AstNode>,
  pub then_statement: Box<AstNode>,
  pub else_statement: Option<Box<AstNode>>,
}
impl_ast_node!(IfStatement);

#[derive(Debug, Clone, PartialEq)]
pub struct ContinueStatement {
  pub base: AstNodeBase,
  pub target_pos: i32,
}
impl_ast_node!(ContinueStatement);

#[derive(Debug, Clone, PartialEq)]
pub struct BreakStatement {
  pub base: AstNodeBase,
  pub target_pos: i32,
}
impl_ast_node!(BreakStatement);

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
  pub base: AstNodeBase,
  pub expression: Option<Box<AstNode>>,
  pub return_type: ReturnType,
}
impl_ast_node!(ReturnStatement);

#[derive(Debug, Clone, PartialEq)]
pub struct WithStatement {
  pub base: AstNodeBase,
  pub object: Box<AstNode>,
  pub statement: Box<AstNode>,
}
impl_ast_node!(WithStatement);

#[derive(Debug, Clone, PartialEq)]
pub struct TryCatchStatement {
  pub base: AstNodeBase,
  pub try_block: Box<Block>,
  pub catch_block: Box<Block>,
  pub variable: Option<AstRawString>,
  /// catch 预测 (原版 catch_prediction_)
  pub catch_prediction: CatchPrediction,
  /// catch 作用域 (原版 scope_)
  pub scope: Option<ScopeId>,
}
impl_ast_node!(TryCatchStatement);

#[derive(Debug, Clone, PartialEq)]
pub struct TryFinallyStatement {
  pub base: AstNodeBase,
  pub try_block: Box<Block>,
  pub finally_block: Box<Block>,
}
impl_ast_node!(TryFinallyStatement);

#[derive(Debug, Clone, PartialEq)]
pub struct DebuggerStatement {
  pub base: AstNodeBase,
}
impl_ast_node!(DebuggerStatement);

#[derive(Debug, Clone, PartialEq)]
pub struct InitializeClassMembersStatement {
  pub base: AstNodeBase,
  pub fields: Vec<ClassLiteralProperty>,
}
impl_ast_node!(InitializeClassMembersStatement);

#[derive(Debug, Clone, PartialEq)]
pub struct InitializeClassStaticElementsStatement {
  pub base: AstNodeBase,
  pub elements: Vec<ClassLiteralStaticElement>,
}
impl_ast_node!(InitializeClassStaticElementsStatement);

#[derive(Debug, Clone, PartialEq)]
pub struct AutoAccessorGetterBody {
  pub base: AstNodeBase,
  /// 存储属性内部名代理 (原版 name_proxy_: VariableProxy*)
  pub name_proxy: Option<Box<VariableProxy>>,
}
impl_ast_node!(AutoAccessorGetterBody);

#[derive(Debug, Clone, PartialEq)]
pub struct AutoAccessorSetterBody {
  pub base: AstNodeBase,
  /// 存储属性内部名代理 (原版 name_proxy_: VariableProxy*)
  pub name_proxy: Option<Box<VariableProxy>>,
}
impl_ast_node!(AutoAccessorSetterBody);

// 5. 字面量表达式
#[derive(Debug, Clone, PartialEq)]
pub struct RegExpLiteral {
  pub base: AstNodeBase,
  pub pattern: AstRawString,
  pub flags: u32,
}
impl_ast_node!(RegExpLiteral);

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectLiteral {
  pub base: AstNodeBase,
  pub properties: Vec<ObjectLiteralProperty>,
}
impl_ast_node!(ObjectLiteral);

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayLiteral {
  pub base: AstNodeBase,
  pub values: Vec<AstNode>,
}
impl_ast_node!(ArrayLiteral);

// 6. 其他表达式
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
  pub base: AstNodeBase,
  /// 赋值操作符 (原版 TokenField; 含纯 '=')
  pub op: AstAssignOp,
  pub target: Box<AstNode>,
  pub value: Box<AstNode>,
  /// 块级 sloppy 函数提升赋值标记 (原版 lookup_hoisting_mode_)
  pub lookup_hoisting_mode: LookupHoistingMode,
}
impl_ast_node!(Assignment);

#[derive(Debug, Clone, PartialEq)]
pub struct Await {
  pub base: AstNodeBase,
  pub expression: Box<AstNode>,
}
impl_ast_node!(Await);

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOperation {
  pub base: AstNodeBase,
  pub op: AstBinaryOp,
  pub left: Box<AstNode>,
  pub right: Box<AstNode>,
}
impl_ast_node!(BinaryOperation);

#[derive(Debug, Clone, PartialEq)]
pub struct NaryOperation {
  pub base: AstNodeBase,
  pub op: AstBinaryOp,
  pub first: Box<AstNode>,
  pub subsequent: Vec<AstNode>,
}
impl_ast_node!(NaryOperation);

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
  pub base: AstNodeBase,
  pub expression: Box<AstNode>,
  pub arguments: Vec<AstNode>,
  pub call_type: CallType,
  pub spread_pos: SpreadPosition,
}
impl_ast_node!(Call);

#[derive(Debug, Clone, PartialEq)]
pub struct SuperCallForwardArgs {
  pub base: AstNodeBase,
  pub expression: Box<AstNode>,
}
impl_ast_node!(SuperCallForwardArgs);

#[derive(Debug, Clone, PartialEq)]
pub struct CallNew {
  pub base: AstNodeBase,
  pub expression: Box<AstNode>,
  pub arguments: Vec<AstNode>,
  /// 展开参数位置 (原版 CallBase spread 位)
  pub spread_pos: SpreadPosition,
}
impl_ast_node!(CallNew);

#[derive(Debug, Clone, PartialEq)]
pub struct CallRuntime {
  pub base: AstNodeBase,
  pub name: AstRawString,
  pub arguments: Vec<AstNode>,
}
impl_ast_node!(CallRuntime);

/// 类字面量构造参数包 (字段全集对照上游 NewClassLiteral)
#[derive(Debug, Clone, PartialEq)]
pub struct ClassLiteralOptions {
  pub name: Option<AstRawString>,
  pub extends: Option<AstNode>,
  pub constructor: Option<FunctionLiteral>,
  pub public_members: Vec<ClassLiteralProperty>,
  pub private_members: Vec<ClassLiteralProperty>,
  pub static_elements: Vec<ClassLiteralStaticElement>,
  pub static_initializer: Option<FunctionLiteral>,
  pub instance_members_initializer: Option<FunctionLiteral>,
  pub end_position: i32,
  pub has_static_computed_names: bool,
  pub is_anonymous: bool,
}

impl Default for ClassLiteralOptions {
  fn default() -> Self {
    Self {
      name: None,
      extends: None,
      constructor: None,
      public_members: Vec::new(),
      private_members: Vec::new(),
      static_elements: Vec::new(),
      static_initializer: None,
      instance_members_initializer: None,
      end_position: NO_SOURCE_POSITION,
      has_static_computed_names: false,
      is_anonymous: false,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassLiteral {
  pub base: AstNodeBase,
  pub name: Option<AstRawString>,
  pub extends: Option<Box<AstNode>>,
  pub constructor: Option<Box<FunctionLiteral>>,
  /// 公开成员 (原版 public_members_: ZonePtrList<Property>*)
  pub public_members: Vec<ClassLiteralProperty>,
  /// 私有成员 (原版 private_members_: ZonePtrList<Property>*)
  pub private_members: Vec<ClassLiteralProperty>,
  pub static_elements: Vec<ClassLiteralStaticElement>,
  /// 静态初始化器 (原版 static_initializer_)
  pub static_initializer: Option<Box<FunctionLiteral>>,
  /// 实例成员初始化器 (原版 instance_members_initializer_function_)
  pub instance_members_initializer: Option<Box<FunctionLiteral>>,
  /// 类作用域 (原版 scope_: ClassScope*)
  pub scope: Option<ScopeId>,
  /// home object 变量 (原版 home_object_: Variable*)
  pub home_object: Option<VariableId>,
  /// 静态 home object 变量 (原版 static_home_object_: Variable*)
  pub static_home_object: Option<VariableId>,
  pub end_position: i32,
  /// 静态计算名标志 (原版 HasStaticComputedNames 位)
  pub has_static_computed_names: bool,
  /// 匿名表达式标志 (原版 IsAnonymousExpression 位)
  pub is_anonymous: bool,
}
impl_ast_node!(ClassLiteral);

impl ClassLiteral {
  /// 是否为匿名函数定义 (原版 IsAnonymousFunctionDefinition)
  pub fn is_anonymous_expression(&self) -> bool {
    self.is_anonymous
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompareOperation {
  pub base: AstNodeBase,
  pub op: AstCompareOp,
  pub left: Box<AstNode>,
  pub right: Box<AstNode>,
}
impl_ast_node!(CompareOperation);

#[derive(Debug, Clone, PartialEq)]
pub struct CompoundAssignment {
  pub base: AstNodeBase,
  pub op: AstCompoundAssignOp,
  pub target: Box<AstNode>,
  pub value: Box<AstNode>,
  /// 复合赋值对应的二元运算 (原版 binary_operation_; 字节码生成器消费)
  pub binary_operation: Option<Box<BinaryOperation>>,
}
impl_ast_node!(CompoundAssignment);

#[derive(Debug, Clone, PartialEq)]
pub struct ConditionalChain {
  pub base: AstNodeBase,
  pub conditions: Vec<(AstNode, AstNode)>,
  pub else_expression: Box<AstNode>,
}
impl_ast_node!(ConditionalChain);

#[derive(Debug, Clone, PartialEq)]
pub struct Conditional {
  pub base: AstNodeBase,
  pub condition: Box<AstNode>,
  pub then_expression: Box<AstNode>,
  pub else_expression: Box<AstNode>,
}
impl_ast_node!(Conditional);

#[derive(Debug, Clone, PartialEq)]
pub struct CountOperation {
  pub base: AstNodeBase,
  pub op: AstCountOp,
  pub expression: Box<AstNode>,
  pub is_prefix: bool,
}
impl_ast_node!(CountOperation);

#[derive(Debug, Clone, PartialEq)]
pub struct EmptyParentheses {
  pub base: AstNodeBase,
}
impl_ast_node!(EmptyParentheses);

/// 函数字面量构造参数包 (字段全集对照上游 NewFunctionLiteral)
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionLiteralOptions {
  pub name: Option<AstRawString>,
  /// 推断名 (原版 raw_inferred_name_: AstConsString*)
  pub inferred_name: Option<AstConsString>,
  /// 函数作用域 (原版 scope_: DeclarationScope*)
  pub scope: Option<ScopeId>,
  pub params: Vec<AstRawString>,
  pub body: Vec<AstNode>,
  pub syntax_kind: FunctionSyntaxKind,
  pub compile_hint: EagerCompileHint,
  pub is_async: bool,
  pub is_generator: bool,
  pub parameter_count: i32,
  pub function_length: i32,
  pub expected_property_count: i32,
  pub has_duplicate_parameters: ParameterFlag,
  pub has_braces: bool,
  pub preparse_data: Option<Box<PreparseData>>,
}

impl Default for FunctionLiteralOptions {
  fn default() -> Self {
    Self {
      name: None,
      inferred_name: None,
      scope: None,
      params: Vec::new(),
      body: Vec::new(),
      syntax_kind: FunctionSyntaxKind::Declaration,
      compile_hint: EagerCompileHint::ShouldLazyCompile,
      is_async: false,
      is_generator: false,
      parameter_count: 0,
      function_length: 0,
      expected_property_count: 0,
      has_duplicate_parameters: ParameterFlag::NoDuplicateParameters,
      has_braces: true,
      preparse_data: None,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionLiteral {
  pub base: AstNodeBase,
  /// 共享名 (原版 raw_name_: AstConsString*; 空表示动态命名)
  pub name: Option<AstRawString>,
  /// 推断名 (原版 raw_inferred_name_)
  pub inferred_name: Option<AstConsString>,
  /// 函数作用域 (原版 scope_: DeclarationScope*)
  pub scope: Option<ScopeId>,
  /// 参数名快照 (上游由 scope_->params() 承载; 值语义树的等价快照)
  pub params: Vec<AstRawString>,
  pub body: Vec<AstNode>,
  /// 语法类别 (原版 FunctionSyntaxKindBits)
  pub syntax_kind: FunctionSyntaxKind,
  pub compile_hint: EagerCompileHint,
  pub is_async: bool,
  pub is_generator: bool,
  /// 形参计数 (原版 parameter_count_)
  pub parameter_count: i32,
  /// 函数 length (原版 function_length_)
  pub function_length: i32,
  /// 期望属性计数 (原版 expected_property_count_)
  pub expected_property_count: i32,
  /// 函数 token 位置 (原版 function_token_position_)
  pub function_token_position: i32,
  /// 挂起计数 (原版 suspend_count_; await/yield 数量)
  pub suspend_count: i32,
  /// 函数字面量 id (原版 function_literal_id_, 由解析器分配)
  pub function_literal_id: i32,
  /// 重复参数标志 (原版 HasDuplicateParameters 位)
  pub has_duplicate_parameters: bool,
  /// 函数体有无花括号 (原版 HasBracesField)
  pub has_braces: bool,
  /// pretenure 位 (原版 Pretenure)
  pub pretenure: bool,
  /// 需要实例成员初始化器 (原版 RequiresInstanceMembersInitializer 位)
  pub requires_instance_members_initializer: bool,
  /// 静态私有方法/存取器 (原版 HasStaticPrivateMethodsOrAccessors 位)
  pub has_static_private_methods_or_accessors: bool,
  /// 预解析数据 (原版 produced_preparse_data_)
  pub preparse_data: Option<Box<PreparseData>>,
}
impl_ast_node!(FunctionLiteral);

impl FunctionLiteral {
  /// 是否为顶层函数 (原版 is_toplevel)
  pub fn is_toplevel(&self) -> bool {
    self.function_literal_id == FUNCTION_LITERAL_ID_TOP_LEVEL
  }

  /// 是否为匿名表达式 (原版 is_anonymous_expression)
  pub fn is_anonymous_expression(&self) -> bool {
    self.syntax_kind.is_anonymous_expression()
  }

  /// 是否有共享名 (原版 has_shared_name)
  pub fn has_shared_name(&self) -> bool {
    self.name.is_some()
  }

  /// 是否可挂起 (原版 CanSuspend: async/generator 且有挂起点)
  pub fn can_suspend(&self) -> bool {
    self.suspend_count > 0
  }

  /// 是否需要急编译 (原版 ShouldEagerCompile 简化: 编译提示位)
  pub fn should_eager_compile(&self) -> bool {
    matches!(self.compile_hint, EagerCompileHint::ShouldEagerCompile)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetTemplateObject {
  pub base: AstNodeBase,
  pub strings: Vec<AstRawString>,
  pub raw_strings: Vec<AstRawString>,
}
impl_ast_node!(GetTemplateObject);

#[derive(Debug, Clone, PartialEq)]
pub struct ImportCallExpression {
  pub base: AstNodeBase,
  pub specifier: Box<AstNode>,
  pub phase: ModuleImportPhase,
  pub options: Option<Box<AstNode>>,
}
impl_ast_node!(ImportCallExpression);

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
  pub base: AstNodeBase,
  pub value: AstValue,
}
impl_ast_node!(Literal);

impl Literal {
  pub fn to_boolean_is_true(&self) -> bool {
    self.value.to_boolean_is_true()
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NativeFunctionLiteral {
  pub base: AstNodeBase,
  pub name: AstRawString,
}
impl_ast_node!(NativeFunctionLiteral);

#[derive(Debug, Clone, PartialEq)]
pub struct OptionalChain {
  pub base: AstNodeBase,
  pub expression: Box<AstNode>,
}
impl_ast_node!(OptionalChain);

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
  pub base: AstNodeBase,
  pub object: Box<AstNode>,
  pub key: Box<AstNode>,
  pub is_computed: bool,
  pub is_optional: bool,
}
impl_ast_node!(Property);

#[derive(Debug, Clone, PartialEq)]
pub struct Spread {
  pub base: AstNodeBase,
  pub expression: Box<AstNode>,
}
impl_ast_node!(Spread);

#[derive(Debug, Clone, PartialEq)]
pub struct SuperCallReference {
  pub base: AstNodeBase,
  pub new_target_var: Option<Box<VariableProxy>>,
  pub this_function_var: Option<Box<VariableProxy>>,
}
impl_ast_node!(SuperCallReference);

#[derive(Debug, Clone, PartialEq)]
pub struct SuperPropertyReference {
  pub base: AstNodeBase,
  /// home object 代理 (原版 home_object_: VariableProxy*)
  pub home_object: Option<Box<VariableProxy>>,
}
impl_ast_node!(SuperPropertyReference);

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateLiteral {
  pub base: AstNodeBase,
  pub string_parts: Vec<AstRawString>,
  pub substitutions: Vec<AstNode>,
}
impl_ast_node!(TemplateLiteral);

#[derive(Debug, Clone, PartialEq)]
pub struct ThisExpression {
  pub base: AstNodeBase,
}
impl_ast_node!(ThisExpression);

#[derive(Debug, Clone, PartialEq)]
pub struct Throw {
  pub base: AstNodeBase,
  pub exception: Box<AstNode>,
}
impl_ast_node!(Throw);

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOperation {
  pub base: AstNodeBase,
  pub op: AstUnaryOp,
  pub expression: Box<AstNode>,
}
impl_ast_node!(UnaryOperation);

#[derive(Debug, Clone, PartialEq)]
pub struct VariableProxy {
  pub base: AstNodeBase,
  pub name: AstRawString,
  pub binding_mode: VariableProxyBindingMode,
  pub hole_check_mode: HoleCheckMode,
  /// 是否为赋值目标 (原版 IsAssignedField)
  pub is_assigned: bool,
  /// 是否已消解 (原版 IsResolvedField)
  pub is_resolved: bool,
  /// 是否为 new.target (原版 IsNewTargetField)
  pub is_new_target: bool,
  /// 是否为 home object 引用 (原版 IsHomeObjectField)
  pub is_home_object: bool,
  /// 绑定变量 (原版 var_: Variable*; 消解后填充)
  pub var: Option<VariableId>,
}
impl_ast_node!(VariableProxy);

impl VariableProxy {
  /// 绑定到变量 (原版 BindTo: 标记消解 + is_used)
  pub fn bind_to(&mut self, var: VariableId) {
    self.var = Some(var);
    self.is_resolved = true;
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Yield {
  pub base: AstNodeBase,
  pub expression: Option<Box<AstNode>>,
  pub on_abrupt_resume: OnAbruptResume,
}
impl_ast_node!(Yield);

#[derive(Debug, Clone, PartialEq)]
pub struct YieldStar {
  pub base: AstNodeBase,
  pub expression: Box<AstNode>,
}
impl_ast_node!(YieldStar);

#[derive(Debug, Clone, PartialEq)]
pub struct FailureExpression {
  pub base: AstNodeBase,
}
impl_ast_node!(FailureExpression);

// ── 统一 AST 节点枚举 (enum_dispatch 派发) ──

#[enum_dispatch(AstNodeTrait)]
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
  VariableDeclaration(VariableDeclaration),
  FunctionDeclaration(FunctionDeclaration),
  DoWhileStatement(DoWhileStatement),
  WhileStatement(WhileStatement),
  ForStatement(ForStatement),
  ForInStatement(ForInStatement),
  ForOfStatement(ForOfStatement),
  Block(Block),
  SwitchStatement(SwitchStatement),
  ExpressionStatement(ExpressionStatement),
  EmptyStatement(EmptyStatement),
  SloppyBlockFunctionStatement(SloppyBlockFunctionStatement),
  IfStatement(IfStatement),
  ContinueStatement(ContinueStatement),
  BreakStatement(BreakStatement),
  ReturnStatement(ReturnStatement),
  WithStatement(WithStatement),
  TryCatchStatement(TryCatchStatement),
  TryFinallyStatement(TryFinallyStatement),
  DebuggerStatement(DebuggerStatement),
  InitializeClassMembersStatement(InitializeClassMembersStatement),
  InitializeClassStaticElementsStatement(InitializeClassStaticElementsStatement),
  AutoAccessorGetterBody(AutoAccessorGetterBody),
  AutoAccessorSetterBody(AutoAccessorSetterBody),
  RegExpLiteral(RegExpLiteral),
  ObjectLiteral(ObjectLiteral),
  ArrayLiteral(ArrayLiteral),
  Assignment(Assignment),
  Await(Await),
  BinaryOperation(BinaryOperation),
  NaryOperation(NaryOperation),
  Call(Call),
  SuperCallForwardArgs(SuperCallForwardArgs),
  CallNew(CallNew),
  CallRuntime(CallRuntime),
  ClassLiteral(ClassLiteral),
  CompareOperation(CompareOperation),
  CompoundAssignment(CompoundAssignment),
  ConditionalChain(ConditionalChain),
  Conditional(Conditional),
  CountOperation(CountOperation),
  EmptyParentheses(EmptyParentheses),
  FunctionLiteral(FunctionLiteral),
  GetTemplateObject(GetTemplateObject),
  ImportCallExpression(ImportCallExpression),
  Literal(Literal),
  NativeFunctionLiteral(NativeFunctionLiteral),
  OptionalChain(OptionalChain),
  Property(Property),
  Spread(Spread),
  SuperCallReference(SuperCallReference),
  SuperPropertyReference(SuperPropertyReference),
  TemplateLiteral(TemplateLiteral),
  ThisExpression(ThisExpression),
  Throw(Throw),
  UnaryOperation(UnaryOperation),
  VariableProxy(VariableProxy),
  Yield(Yield),
  YieldStar(YieldStar),
  FailureExpression(FailureExpression),
}

impl AstNode {
  /// 是否为语句类别节点
  pub fn is_statement(&self) -> bool {
    self.node_type().is_statement()
  }

  /// 是否为表达式类别节点
  pub fn is_expression(&self) -> bool {
    self.node_type().is_expression()
  }

  /// 是否为声明类别节点
  pub fn is_declaration(&self) -> bool {
    self.node_type().is_declaration()
  }

  /// 是否为字面量类别节点
  pub fn is_literal(&self) -> bool {
    self.node_type().is_literal()
  }

  /// 转换为字面量节点引用
  pub fn as_literal(&self) -> Option<&Literal> {
    if let Self::Literal(lit) = self {
      Some(lit)
    } else {
      None
    }
  }

  /// 转换为语句块节点引用
  pub fn as_block(&self) -> Option<&Block> {
    if let Self::Block(b) = self {
      Some(b)
    } else {
      None
    }
  }

  /// 转换为二元操作引用
  pub fn as_binary_operation(&self) -> Option<&BinaryOperation> {
    if let Self::BinaryOperation(b) = self {
      Some(b)
    } else {
      None
    }
  }

  /// 转换为变量代理引用
  pub fn as_variable_proxy(&self) -> Option<&VariableProxy> {
    if let Self::VariableProxy(vp) = self {
      Some(vp)
    } else {
      None
    }
  }
}

// ── 工厂构建器 AstNodeFactory ──

/// AST 节点创建工厂 (原版 AstNodeFactory)
///
/// - 无 zone 形态: 纯值构造 (节点落调用方堆/容器)
/// - zone 形态 (`with_zone`): 节点驻留 zone (`alloc_node`, 原版 NewAstNode)
///   + ZoneList/ZoneVector 集合物化; 避开 v8-zone 的 Snapshot/reset 族
pub struct AstNodeFactory<'z, 'a> {
  flags: Option<&'z v8_flags::Flags>,
  zone: Option<&'z Zone<'a>>,
  /// 下一个函数字面量 id (原版由解析器持有计数器, 此处随工厂分配)
  next_function_literal_id: i32,
}

impl Default for AstNodeFactory<'_, '_> {
  fn default() -> Self {
    Self::new()
  }
}

impl<'z, 'a> AstNodeFactory<'z, 'a> {
  pub fn new() -> Self {
    Self {
      flags: None,
      zone: None,
      next_function_literal_id: FUNCTION_LITERAL_ID_TOP_LEVEL + 1,
    }
  }

  pub fn with_flags(flags: &'z v8_flags::Flags) -> Self {
    Self {
      flags: Some(flags),
      zone: None,
      next_function_literal_id: FUNCTION_LITERAL_ID_TOP_LEVEL + 1,
    }
  }

  /// zone 后端形态 (分配路径见 alloc_node/new_zone_list/zone_vec 族)
  pub fn with_zone(zone: &'z Zone<'a>) -> Self {
    Self {
      flags: None,
      zone: Some(zone),
      next_function_literal_id: FUNCTION_LITERAL_ID_TOP_LEVEL + 1,
    }
  }

  pub fn flags(&self) -> Option<&'z v8_flags::Flags> {
    self.flags
  }

  pub fn zone(&self) -> Option<&'z Zone<'a>> {
    self.zone
  }

  /// 分配下一个函数字面量 id (原版 GetNextFunctionLiteralId)
  pub fn next_function_literal_id(&mut self) -> i32 {
    let id = self.next_function_literal_id;
    self.next_function_literal_id = self.next_function_literal_id.saturating_add(1);
    id
  }

  // ── Zone 分配路径 (原版 NewAstNode<T> + 集合 zone 构造) ──

  /// 节点/对象驻留 zone (原版 NewAstNode: new (zone->Allocate<T>()) T)
  pub fn alloc_node<T>(&self, node: T) -> Result<&'z mut T> {
    let zone = self.zone.ok_or(Error::NoZoneBackend)?;
    zone.make(node).map_err(Into::into)
  }

  /// 新建 ZoneList (原版 ZoneList<T>(capacity, zone); 指针列表承接节点/对象)
  pub fn new_zone_list<T>(&self, capacity: i32) -> Result<ZoneList<'z, 'a, T>> {
    let zone = self.zone.ok_or(Error::NoZoneBackend)?;
    ZoneList::new(capacity, zone).map_err(Into::into)
  }

  /// 切片物化为 ZoneVector (原版 ZoneVector(vector, zone))
  pub fn zone_vec_from_slice<T: Clone>(&self, src: &[T]) -> Result<ZoneVector<'z, 'a, T>> {
    let zone = self.zone.ok_or(Error::NoZoneBackend)?;
    ZoneVector::from_slice(zone, src).map_err(Into::into)
  }

  /// 空 ZoneVector (逐 push_back 移动构建; 解析器语句流形态)
  pub fn zone_vec<T>(&self) -> Result<ZoneVector<'z, 'a, T>> {
    let zone = self.zone.ok_or(Error::NoZoneBackend)?;
    Ok(ZoneVector::new(zone))
  }

  // ── 声明节点 ──

  pub fn new_variable_declaration(
    pos: i32,
    var: Option<AstRawString>,
    mode: VariableMode,
  ) -> VariableDeclaration {
    VariableDeclaration {
      base: AstNodeBase::new(pos, AstNodeType::VariableDeclaration),
      var,
      mode,
    }
  }

  pub fn new_function_declaration(pos: i32, fun: Option<FunctionLiteral>) -> FunctionDeclaration {
    FunctionDeclaration {
      base: AstNodeBase::new(pos, AstNodeType::FunctionDeclaration),
      fun: fun.map(Box::new),
    }
  }

  // ── 迭代语句 ──

  pub fn new_do_while_statement(pos: i32, body: AstNode, cond: AstNode) -> DoWhileStatement {
    DoWhileStatement {
      base: AstNodeBase::new(pos, AstNodeType::DoWhileStatement),
      body: Box::new(body),
      cond: Box::new(cond),
    }
  }

  pub fn new_while_statement(pos: i32, cond: AstNode, body: AstNode) -> WhileStatement {
    WhileStatement {
      base: AstNodeBase::new(pos, AstNodeType::WhileStatement),
      cond: Box::new(cond),
      body: Box::new(body),
    }
  }

  pub fn new_for_statement(
    pos: i32,
    init: Option<AstNode>,
    cond: Option<AstNode>,
    next: Option<AstNode>,
    body: AstNode,
  ) -> ForStatement {
    ForStatement {
      base: AstNodeBase::new(pos, AstNodeType::ForStatement),
      init: init.map(Box::new),
      cond: cond.map(Box::new),
      next: next.map(Box::new),
      body: Box::new(body),
    }
  }

  pub fn new_for_in_statement(
    pos: i32,
    each: AstNode,
    subject: AstNode,
    body: AstNode,
    subject_scope: Option<ScopeId>,
  ) -> ForInStatement {
    ForInStatement {
      base: AstNodeBase::new(pos, AstNodeType::ForInStatement),
      each: Box::new(each),
      subject: Box::new(subject),
      body: Box::new(body),
      subject_scope,
    }
  }

  pub fn new_for_of_statement(
    pos: i32,
    each: AstNode,
    subject: AstNode,
    body: AstNode,
    iter_type: IteratorType,
    subject_scope: Option<ScopeId>,
  ) -> ForOfStatement {
    ForOfStatement {
      base: AstNodeBase::new(pos, AstNodeType::ForOfStatement),
      each: Box::new(each),
      subject: Box::new(subject),
      body: Box::new(body),
      iter_type,
      subject_scope,
    }
  }

  // ── 可跳出语句 ──

  /// 语句块 (原版 NewBlock: capacity/ignore_completion/is_breakable)
  pub fn new_block(pos: i32, statements: Vec<AstNode>, is_breakable: bool) -> Block {
    Block {
      base: AstNodeBase::new(pos, AstNodeType::Block),
      statements,
      is_breakable,
      ignore_completion: false,
      is_initialization_block_for_parameters: false,
      scope: None,
    }
  }

  /// 忽略完成值的语句块 (原版 NewBlock ignore_completion_value=true 形态)
  pub fn new_block_ignoring_completion(pos: i32, statements: Vec<AstNode>) -> Block {
    Block {
      ignore_completion: true,
      ..Self::new_block(pos, statements, false)
    }
  }

  pub fn new_switch_statement(pos: i32, tag: AstNode, cases: Vec<CaseClause>) -> SwitchStatement {
    SwitchStatement {
      base: AstNodeBase::new(pos, AstNodeType::SwitchStatement),
      tag: Box::new(tag),
      cases,
    }
  }

  // ── 其他语句 ──

  pub fn new_expression_statement(pos: i32, expr: AstNode) -> ExpressionStatement {
    ExpressionStatement {
      base: AstNodeBase::new(pos, AstNodeType::ExpressionStatement),
      expression: Box::new(expr),
    }
  }

  pub fn new_empty_statement(pos: i32) -> EmptyStatement {
    EmptyStatement {
      base: AstNodeBase::new(pos, AstNodeType::EmptyStatement),
    }
  }

  pub fn new_sloppy_block_function_statement(
    pos: i32,
    statement: AstNode,
  ) -> SloppyBlockFunctionStatement {
    SloppyBlockFunctionStatement {
      base: AstNodeBase::new(pos, AstNodeType::SloppyBlockFunctionStatement),
      statement: Box::new(statement),
    }
  }

  pub fn new_if_statement(
    pos: i32,
    cond: AstNode,
    then_stmt: AstNode,
    else_stmt: Option<AstNode>,
  ) -> IfStatement {
    IfStatement {
      base: AstNodeBase::new(pos, AstNodeType::IfStatement),
      condition: Box::new(cond),
      then_statement: Box::new(then_stmt),
      else_statement: else_stmt.map(Box::new),
    }
  }

  pub fn new_continue_statement(pos: i32, target_pos: i32) -> ContinueStatement {
    ContinueStatement {
      base: AstNodeBase::new(pos, AstNodeType::ContinueStatement),
      target_pos,
    }
  }

  pub fn new_break_statement(pos: i32, target_pos: i32) -> BreakStatement {
    BreakStatement {
      base: AstNodeBase::new(pos, AstNodeType::BreakStatement),
      target_pos,
    }
  }

  pub fn new_return_statement(
    pos: i32,
    expr: Option<AstNode>,
    return_type: ReturnType,
  ) -> ReturnStatement {
    ReturnStatement {
      base: AstNodeBase::new(pos, AstNodeType::ReturnStatement),
      expression: expr.map(Box::new),
      return_type,
    }
  }

  pub fn new_with_statement(pos: i32, object: AstNode, statement: AstNode) -> WithStatement {
    WithStatement {
      base: AstNodeBase::new(pos, AstNodeType::WithStatement),
      object: Box::new(object),
      statement: Box::new(statement),
    }
  }

  pub fn new_try_catch_statement(
    pos: i32,
    try_block: Block,
    catch_block: Block,
    variable: Option<AstRawString>,
    catch_prediction: CatchPrediction,
  ) -> TryCatchStatement {
    TryCatchStatement {
      base: AstNodeBase::new(pos, AstNodeType::TryCatchStatement),
      try_block: Box::new(try_block),
      catch_block: Box::new(catch_block),
      variable,
      catch_prediction,
      scope: None,
    }
  }

  pub fn new_try_finally_statement(
    pos: i32,
    try_block: Block,
    finally_block: Block,
  ) -> TryFinallyStatement {
    TryFinallyStatement {
      base: AstNodeBase::new(pos, AstNodeType::TryFinallyStatement),
      try_block: Box::new(try_block),
      finally_block: Box::new(finally_block),
    }
  }

  pub fn new_debugger_statement(pos: i32) -> DebuggerStatement {
    DebuggerStatement {
      base: AstNodeBase::new(pos, AstNodeType::DebuggerStatement),
    }
  }

  pub fn new_initialize_class_members_statement(
    pos: i32,
    fields: Vec<ClassLiteralProperty>,
  ) -> InitializeClassMembersStatement {
    InitializeClassMembersStatement {
      base: AstNodeBase::new(pos, AstNodeType::InitializeClassMembersStatement),
      fields,
    }
  }

  pub fn new_initialize_class_static_elements_statement(
    pos: i32,
    elements: Vec<ClassLiteralStaticElement>,
  ) -> InitializeClassStaticElementsStatement {
    InitializeClassStaticElementsStatement {
      base: AstNodeBase::new(pos, AstNodeType::InitializeClassStaticElementsStatement),
      elements,
    }
  }

  pub fn new_auto_accessor_getter_body(
    pos: i32,
    name_proxy: Option<VariableProxy>,
  ) -> AutoAccessorGetterBody {
    AutoAccessorGetterBody {
      base: AstNodeBase::new(pos, AstNodeType::AutoAccessorGetterBody),
      name_proxy: name_proxy.map(Box::new),
    }
  }

  pub fn new_auto_accessor_setter_body(
    pos: i32,
    name_proxy: Option<VariableProxy>,
  ) -> AutoAccessorSetterBody {
    AutoAccessorSetterBody {
      base: AstNodeBase::new(pos, AstNodeType::AutoAccessorSetterBody),
      name_proxy: name_proxy.map(Box::new),
    }
  }

  // ── 字面量表达式 ──

  pub fn new_reg_exp_literal(pos: i32, pattern: AstRawString, flags: u32) -> RegExpLiteral {
    RegExpLiteral {
      base: AstNodeBase::new(pos, AstNodeType::RegExpLiteral),
      pattern,
      flags,
    }
  }

  pub fn new_object_literal(pos: i32, properties: Vec<ObjectLiteralProperty>) -> ObjectLiteral {
    ObjectLiteral {
      base: AstNodeBase::new(pos, AstNodeType::ObjectLiteral),
      properties,
    }
  }

  pub fn new_array_literal(pos: i32, values: Vec<AstNode>) -> ArrayLiteral {
    ArrayLiteral {
      base: AstNodeBase::new(pos, AstNodeType::ArrayLiteral),
      values,
    }
  }

  // ── 其他表达式 ──

  pub fn new_assignment(pos: i32, op: AstAssignOp, target: AstNode, value: AstNode) -> Assignment {
    Assignment {
      base: AstNodeBase::new(pos, AstNodeType::Assignment),
      op,
      target: Box::new(target),
      value: Box::new(value),
      lookup_hoisting_mode: LookupHoistingMode::Normal,
    }
  }

  pub fn new_await(pos: i32, expr: AstNode) -> Await {
    Await {
      base: AstNodeBase::new(pos, AstNodeType::Await),
      expression: Box::new(expr),
    }
  }

  pub fn new_binary_operation(
    pos: i32,
    op: AstBinaryOp,
    left: AstNode,
    right: AstNode,
  ) -> BinaryOperation {
    BinaryOperation {
      base: AstNodeBase::new(pos, AstNodeType::BinaryOperation),
      op,
      left: Box::new(left),
      right: Box::new(right),
    }
  }

  pub fn new_nary_operation(
    pos: i32,
    op: AstBinaryOp,
    first: AstNode,
    subsequent: Vec<AstNode>,
  ) -> NaryOperation {
    NaryOperation {
      base: AstNodeBase::new(pos, AstNodeType::NaryOperation),
      op,
      first: Box::new(first),
      subsequent,
    }
  }

  pub fn new_call(
    pos: i32,
    callee: AstNode,
    arguments: Vec<AstNode>,
    call_type: CallType,
    spread_pos: SpreadPosition,
  ) -> Call {
    Call {
      base: AstNodeBase::new(pos, AstNodeType::Call),
      expression: Box::new(callee),
      arguments,
      call_type,
      spread_pos,
    }
  }

  pub fn new_super_call_forward_args(pos: i32, expression: AstNode) -> SuperCallForwardArgs {
    SuperCallForwardArgs {
      base: AstNodeBase::new(pos, AstNodeType::SuperCallForwardArgs),
      expression: Box::new(expression),
    }
  }

  pub fn new_call_new(
    pos: i32,
    expression: AstNode,
    arguments: Vec<AstNode>,
    spread_pos: SpreadPosition,
  ) -> CallNew {
    CallNew {
      base: AstNodeBase::new(pos, AstNodeType::CallNew),
      expression: Box::new(expression),
      arguments,
      spread_pos,
    }
  }

  pub fn new_call_runtime(pos: i32, name: AstRawString, arguments: Vec<AstNode>) -> CallRuntime {
    CallRuntime {
      base: AstNodeBase::new(pos, AstNodeType::CallRuntime),
      name,
      arguments,
    }
  }

  /// 类字面量 (原版 NewClassLiteral 全参形态)
  pub fn new_class_literal(pos: i32, opts: ClassLiteralOptions) -> ClassLiteral {
    ClassLiteral {
      base: AstNodeBase::new(pos, AstNodeType::ClassLiteral),
      name: opts.name,
      extends: opts.extends.map(Box::new),
      constructor: opts.constructor.map(Box::new),
      public_members: opts.public_members,
      private_members: opts.private_members,
      static_elements: opts.static_elements,
      static_initializer: opts.static_initializer.map(Box::new),
      instance_members_initializer: opts.instance_members_initializer.map(Box::new),
      scope: None,
      home_object: None,
      static_home_object: None,
      end_position: opts.end_position,
      has_static_computed_names: opts.has_static_computed_names,
      is_anonymous: opts.is_anonymous,
    }
  }

  pub fn new_compare_operation(
    pos: i32,
    op: AstCompareOp,
    left: AstNode,
    right: AstNode,
  ) -> CompareOperation {
    CompareOperation {
      base: AstNodeBase::new(pos, AstNodeType::CompareOperation),
      op,
      left: Box::new(left),
      right: Box::new(right),
    }
  }

  pub fn new_compound_assignment(
    pos: i32,
    op: AstCompoundAssignOp,
    target: AstNode,
    value: AstNode,
    binary_operation: Option<BinaryOperation>,
  ) -> CompoundAssignment {
    CompoundAssignment {
      base: AstNodeBase::new(pos, AstNodeType::CompoundAssignment),
      op,
      target: Box::new(target),
      value: Box::new(value),
      binary_operation: binary_operation.map(Box::new),
    }
  }

  pub fn new_conditional_chain(
    pos: i32,
    conditions: Vec<(AstNode, AstNode)>,
    else_expression: AstNode,
  ) -> ConditionalChain {
    ConditionalChain {
      base: AstNodeBase::new(pos, AstNodeType::ConditionalChain),
      conditions,
      else_expression: Box::new(else_expression),
    }
  }

  pub fn new_conditional(
    pos: i32,
    condition: AstNode,
    then_expression: AstNode,
    else_expression: AstNode,
  ) -> Conditional {
    Conditional {
      base: AstNodeBase::new(pos, AstNodeType::Conditional),
      condition: Box::new(condition),
      then_expression: Box::new(then_expression),
      else_expression: Box::new(else_expression),
    }
  }

  pub fn new_count_operation(
    pos: i32,
    op: AstCountOp,
    expr: AstNode,
    is_prefix: bool,
  ) -> CountOperation {
    CountOperation {
      base: AstNodeBase::new(pos, AstNodeType::CountOperation),
      op,
      expression: Box::new(expr),
      is_prefix,
    }
  }

  pub fn new_empty_parentheses(pos: i32) -> EmptyParentheses {
    EmptyParentheses {
      base: AstNodeBase::new(pos, AstNodeType::EmptyParentheses),
    }
  }

  /// 创建函数字面量并分配 id (原版 NewFunctionLiteral + 解析器 id 计数)
  pub fn new_function_literal(
    &mut self,
    pos: i32,
    opts: FunctionLiteralOptions,
  ) -> FunctionLiteral {
    FunctionLiteral {
      base: AstNodeBase::new(pos, AstNodeType::FunctionLiteral),
      name: opts.name,
      inferred_name: opts.inferred_name,
      scope: opts.scope,
      params: opts.params,
      body: opts.body,
      syntax_kind: opts.syntax_kind,
      compile_hint: opts.compile_hint,
      is_async: opts.is_async,
      is_generator: opts.is_generator,
      parameter_count: opts.parameter_count,
      function_length: opts.function_length,
      expected_property_count: opts.expected_property_count,
      function_token_position: NO_SOURCE_POSITION,
      suspend_count: 0,
      function_literal_id: self.next_function_literal_id(),
      has_duplicate_parameters: matches!(
        opts.has_duplicate_parameters,
        ParameterFlag::HasDuplicateParameters
      ),
      has_braces: opts.has_braces,
      pretenure: false,
      requires_instance_members_initializer: false,
      has_static_private_methods_or_accessors: false,
      preparse_data: opts.preparse_data,
    }
  }

  pub fn new_get_template_object(
    pos: i32,
    strings: Vec<AstRawString>,
    raw_strings: Vec<AstRawString>,
  ) -> GetTemplateObject {
    GetTemplateObject {
      base: AstNodeBase::new(pos, AstNodeType::GetTemplateObject),
      strings,
      raw_strings,
    }
  }

  pub fn new_import_call_expression(
    pos: i32,
    specifier: AstNode,
    phase: ModuleImportPhase,
    options: Option<AstNode>,
  ) -> ImportCallExpression {
    ImportCallExpression {
      base: AstNodeBase::new(pos, AstNodeType::ImportCallExpression),
      specifier: Box::new(specifier),
      phase,
      options: options.map(Box::new),
    }
  }

  pub fn new_literal(pos: i32, val: AstValue) -> Literal {
    Literal {
      base: AstNodeBase::new(pos, AstNodeType::Literal),
      value: val,
    }
  }

  pub fn new_bigint_literal(pos: i32, bigint: AstBigInt) -> Literal {
    Literal {
      base: AstNodeBase::new(pos, AstNodeType::Literal),
      value: AstValue::BigInt(bigint),
    }
  }

  pub fn new_native_function_literal(pos: i32, name: AstRawString) -> NativeFunctionLiteral {
    NativeFunctionLiteral {
      base: AstNodeBase::new(pos, AstNodeType::NativeFunctionLiteral),
      name,
    }
  }

  pub fn new_optional_chain(pos: i32, expression: AstNode) -> OptionalChain {
    OptionalChain {
      base: AstNodeBase::new(pos, AstNodeType::OptionalChain),
      expression: Box::new(expression),
    }
  }

  pub fn new_property(
    pos: i32,
    object: AstNode,
    key: AstNode,
    is_computed: bool,
    is_optional: bool,
  ) -> Property {
    Property {
      base: AstNodeBase::new(pos, AstNodeType::Property),
      object: Box::new(object),
      key: Box::new(key),
      is_computed,
      is_optional,
    }
  }

  pub fn new_spread(pos: i32, expression: AstNode) -> Spread {
    Spread {
      base: AstNodeBase::new(pos, AstNodeType::Spread),
      expression: Box::new(expression),
    }
  }

  pub fn new_super_call_reference(
    pos: i32,
    new_target_var: Option<VariableProxy>,
    this_function_var: Option<VariableProxy>,
  ) -> SuperCallReference {
    SuperCallReference {
      base: AstNodeBase::new(pos, AstNodeType::SuperCallReference),
      new_target_var: new_target_var.map(Box::new),
      this_function_var: this_function_var.map(Box::new),
    }
  }

  pub fn new_super_property_reference(
    pos: i32,
    home_object: Option<VariableProxy>,
  ) -> SuperPropertyReference {
    SuperPropertyReference {
      base: AstNodeBase::new(pos, AstNodeType::SuperPropertyReference),
      home_object: home_object.map(Box::new),
    }
  }

  pub fn new_template_literal(
    pos: i32,
    string_parts: Vec<AstRawString>,
    substitutions: Vec<AstNode>,
  ) -> TemplateLiteral {
    TemplateLiteral {
      base: AstNodeBase::new(pos, AstNodeType::TemplateLiteral),
      string_parts,
      substitutions,
    }
  }

  pub fn new_this_expression(pos: i32) -> ThisExpression {
    ThisExpression {
      base: AstNodeBase::new(pos, AstNodeType::ThisExpression),
    }
  }

  pub fn new_throw(pos: i32, exception: AstNode) -> Throw {
    Throw {
      base: AstNodeBase::new(pos, AstNodeType::Throw),
      exception: Box::new(exception),
    }
  }

  pub fn new_unary_operation(pos: i32, op: AstUnaryOp, expr: AstNode) -> UnaryOperation {
    UnaryOperation {
      base: AstNodeBase::new(pos, AstNodeType::UnaryOperation),
      op,
      expression: Box::new(expr),
    }
  }

  pub fn new_variable_proxy(
    pos: i32,
    name: AstRawString,
    binding_mode: VariableProxyBindingMode,
    hole_check_mode: HoleCheckMode,
  ) -> VariableProxy {
    VariableProxy {
      base: AstNodeBase::new(pos, AstNodeType::VariableProxy),
      name,
      binding_mode,
      hole_check_mode,
      is_assigned: false,
      is_resolved: false,
      is_new_target: false,
      is_home_object: false,
      var: None,
    }
  }

  pub fn new_yield(
    pos: i32,
    expression: Option<AstNode>,
    on_abrupt_resume: OnAbruptResume,
  ) -> Yield {
    Yield {
      base: AstNodeBase::new(pos, AstNodeType::Yield),
      expression: expression.map(Box::new),
      on_abrupt_resume,
    }
  }

  pub fn new_yield_star(pos: i32, expression: AstNode) -> YieldStar {
    YieldStar {
      base: AstNodeBase::new(pos, AstNodeType::YieldStar),
      expression: Box::new(expression),
    }
  }

  pub fn new_failure_expression(pos: i32) -> FailureExpression {
    FailureExpression {
      base: AstNodeBase::new(pos, AstNodeType::FailureExpression),
    }
  }
}
