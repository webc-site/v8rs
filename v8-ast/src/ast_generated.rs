// 由 tool/src/ast/ast.h.js 从 v8/src/ast/ast.h + src/parsing/token.h 生成; 修订就地改, 复跑覆盖
// AST 节点枚举、操作符、基类属性、位置字段与辅助枚举定义

use strum::{Display, FromRepr};

/// 特殊位置常量: 表示无源码位置 (原版 kNoSourcePosition = -1)
pub const NO_SOURCE_POSITION: i32 = -1;

/// 函数字面量返回位置指示常量 (原版 kFunctionLiteralReturnPosition = -2)
pub const FUNCTION_LITERAL_RETURN_POSITION: i32 = -2;

/// 所有 AST 节点类别枚举 (原版 enum NodeType : uint8_t, 共 60 种)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
#[repr(u8)]
pub enum AstNodeType {
  VariableDeclaration = 0,
  FunctionDeclaration = 1,
  DoWhileStatement = 2,
  WhileStatement = 3,
  ForStatement = 4,
  ForInStatement = 5,
  ForOfStatement = 6,
  Block = 7,
  SwitchStatement = 8,
  ExpressionStatement = 9,
  EmptyStatement = 10,
  SloppyBlockFunctionStatement = 11,
  IfStatement = 12,
  ContinueStatement = 13,
  BreakStatement = 14,
  ReturnStatement = 15,
  WithStatement = 16,
  TryCatchStatement = 17,
  TryFinallyStatement = 18,
  DebuggerStatement = 19,
  InitializeClassMembersStatement = 20,
  InitializeClassStaticElementsStatement = 21,
  AutoAccessorGetterBody = 22,
  AutoAccessorSetterBody = 23,
  RegExpLiteral = 24,
  ObjectLiteral = 25,
  ArrayLiteral = 26,
  Assignment = 27,
  Await = 28,
  BinaryOperation = 29,
  NaryOperation = 30,
  Call = 31,
  SuperCallForwardArgs = 32,
  CallNew = 33,
  CallRuntime = 34,
  ClassLiteral = 35,
  CompareOperation = 36,
  CompoundAssignment = 37,
  ConditionalChain = 38,
  Conditional = 39,
  CountOperation = 40,
  EmptyParentheses = 41,
  FunctionLiteral = 42,
  GetTemplateObject = 43,
  ImportCallExpression = 44,
  Literal = 45,
  NativeFunctionLiteral = 46,
  OptionalChain = 47,
  Property = 48,
  Spread = 49,
  SuperCallReference = 50,
  SuperPropertyReference = 51,
  TemplateLiteral = 52,
  ThisExpression = 53,
  Throw = 54,
  UnaryOperation = 55,
  VariableProxy = 56,
  Yield = 57,
  YieldStar = 58,
  FailureExpression = 59,
}

impl AstNodeType {
  /// 节点类型总数 (含 FailureExpression)
  pub const COUNT: usize = 60;
  /// 有效 AST 节点总数 (不含 FailureExpression)
  pub const AST_NODE_COUNT: usize = 59;
  /// 声明节点数
  pub const DECLARATION_COUNT: usize = 2;
  /// 语句节点数
  pub const STATEMENT_COUNT: usize = 22;
  /// 表达式节点数
  pub const EXPRESSION_COUNT: usize = 35;

  /// 判断是否为声明节点
  pub const fn is_declaration(&self) -> bool {
    matches!(self, Self::VariableDeclaration | Self::FunctionDeclaration)
  }

  /// 判断是否为语句节点
  pub const fn is_statement(&self) -> bool {
    matches!(
      self,
      Self::DoWhileStatement
        | Self::WhileStatement
        | Self::ForStatement
        | Self::ForInStatement
        | Self::ForOfStatement
        | Self::Block
        | Self::SwitchStatement
        | Self::ExpressionStatement
        | Self::EmptyStatement
        | Self::SloppyBlockFunctionStatement
        | Self::IfStatement
        | Self::ContinueStatement
        | Self::BreakStatement
        | Self::ReturnStatement
        | Self::WithStatement
        | Self::TryCatchStatement
        | Self::TryFinallyStatement
        | Self::DebuggerStatement
        | Self::InitializeClassMembersStatement
        | Self::InitializeClassStaticElementsStatement
        | Self::AutoAccessorGetterBody
        | Self::AutoAccessorSetterBody
    )
  }

  /// 判断是否为迭代语句
  pub const fn is_iteration_statement(&self) -> bool {
    matches!(
      self,
      Self::DoWhileStatement
        | Self::WhileStatement
        | Self::ForStatement
        | Self::ForInStatement
        | Self::ForOfStatement
    )
  }

  /// 判断是否为可跳出语句
  pub const fn is_breakable_statement(&self) -> bool {
    matches!(self, Self::Block | Self::SwitchStatement)
  }

  /// 判断是否为表达式节点
  pub const fn is_expression(&self) -> bool {
    matches!(
      self,
      Self::RegExpLiteral
        | Self::ObjectLiteral
        | Self::ArrayLiteral
        | Self::Assignment
        | Self::Await
        | Self::BinaryOperation
        | Self::NaryOperation
        | Self::Call
        | Self::SuperCallForwardArgs
        | Self::CallNew
        | Self::CallRuntime
        | Self::ClassLiteral
        | Self::CompareOperation
        | Self::CompoundAssignment
        | Self::ConditionalChain
        | Self::Conditional
        | Self::CountOperation
        | Self::EmptyParentheses
        | Self::FunctionLiteral
        | Self::GetTemplateObject
        | Self::ImportCallExpression
        | Self::Literal
        | Self::NativeFunctionLiteral
        | Self::OptionalChain
        | Self::Property
        | Self::Spread
        | Self::SuperCallReference
        | Self::SuperPropertyReference
        | Self::TemplateLiteral
        | Self::ThisExpression
        | Self::Throw
        | Self::UnaryOperation
        | Self::VariableProxy
        | Self::Yield
        | Self::YieldStar
    )
  }

  /// 判断是否为字面量表达式
  pub const fn is_literal(&self) -> bool {
    matches!(
      self,
      Self::Literal | Self::RegExpLiteral | Self::ObjectLiteral | Self::ArrayLiteral
    )
  }
}

/// 二元运算符 (原版 BINARY_OP_TOKEN_LIST)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
pub enum AstBinaryOp {
  /// `??` (优先级 3)
  Nullish,
  /// `||` (优先级 4)
  Or,
  /// `&&` (优先级 5)
  And,
  /// `|` (优先级 6)
  BitOr,
  /// `^` (优先级 7)
  BitXor,
  /// `&` (优先级 8)
  BitAnd,
  /// `<<` (优先级 11)
  Shl,
  /// `>>` (优先级 11)
  Sar,
  /// `>>>` (优先级 11)
  Shr,
  /// `*` (优先级 13)
  Mul,
  /// `/` (优先级 13)
  Div,
  /// `%` (优先级 13)
  Mod,
  /// `**` (优先级 14)
  Exp,
  /// `+` (优先级 12)
  Add,
  /// `-` (优先级 12)
  Sub,
  /// `,` (优先级 1)
  Comma,
}

impl AstBinaryOp {
  /// 运算优先级 (越高越先计算)
  pub const fn precedence(&self) -> u8 {
    match self {
      Self::Comma => 1,
      Self::Nullish => 3,
      Self::Or => 4,
      Self::And => 5,
      Self::BitOr => 6,
      Self::BitXor => 7,
      Self::BitAnd => 8,
      Self::Shl => 11,
      Self::Sar => 11,
      Self::Shr => 11,
      Self::Mul => 13,
      Self::Div => 13,
      Self::Mod => 13,
      Self::Exp => 14,
      Self::Add => 12,
      Self::Sub => 12,
    }
  }
  /// 字符表示
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::Comma => ",",
      Self::Nullish => "??",
      Self::Or => "||",
      Self::And => "&&",
      Self::BitOr => "|",
      Self::BitXor => "^",
      Self::BitAnd => "&",
      Self::Shl => "<<",
      Self::Sar => ">>",
      Self::Shr => ">>>",
      Self::Mul => "*",
      Self::Div => "/",
      Self::Mod => "%",
      Self::Exp => "**",
      Self::Add => "+",
      Self::Sub => "-",
    }
  }
}

/// 关系与比较运算符 (原版 Token::Value 中的比较操作集合)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
pub enum AstCompareOp {
  /// `==` (优先级 9)
  Eq,
  /// `===` (优先级 9)
  EqStrict,
  /// `!=` (优先级 9)
  NotEq,
  /// `!==` (优先级 9)
  NotEqStrict,
  /// `<` (优先级 10)
  LessThan,
  /// `>` (优先级 10)
  GreaterThan,
  /// `<=` (优先级 10)
  LessThanEq,
  /// `>=` (优先级 10)
  GreaterThanEq,
  /// `instanceof` (优先级 10)
  InstanceOf,
  /// `in` (优先级 10)
  In,
}

impl AstCompareOp {
  /// 运算优先级
  pub const fn precedence(&self) -> u8 {
    match self {
      Self::Eq => 9,
      Self::EqStrict => 9,
      Self::NotEq => 9,
      Self::NotEqStrict => 9,
      Self::LessThan => 10,
      Self::GreaterThan => 10,
      Self::LessThanEq => 10,
      Self::GreaterThanEq => 10,
      Self::InstanceOf => 10,
      Self::In => 10,
    }
  }
  /// 字符表示
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::Eq => "==",
      Self::EqStrict => "===",
      Self::NotEq => "!=",
      Self::NotEqStrict => "!==",
      Self::LessThan => "<",
      Self::GreaterThan => ">",
      Self::LessThanEq => "<=",
      Self::GreaterThanEq => ">=",
      Self::InstanceOf => "instanceof",
      Self::In => "in",
    }
  }
}

/// 一元运算符 (原版 Token::Value 中的一元操作集合)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
pub enum AstUnaryOp {
  /// `!`
  Not,
  /// `~`
  BitNot,
  /// `delete`
  Delete,
  /// `typeof`
  TypeOf,
  /// `void`
  Void,
  /// `+`
  Add,
  /// `-`
  Sub,
}

impl AstUnaryOp {
  /// 字符表示
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::Not => "!",
      Self::BitNot => "~",
      Self::Delete => "delete",
      Self::TypeOf => "typeof",
      Self::Void => "void",
      Self::Add => "+",
      Self::Sub => "-",
    }
  }
}

/// 自增/自减运算符 (原版 CountOperation token)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
pub enum AstCountOp {
  /// `++`
  Inc,
  /// `--`
  Dec,
}

impl AstCountOp {
  /// 字符表示
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::Inc => "++",
      Self::Dec => "--",
    }
  }
}

/// 复合赋值运算符 (原版 CompoundAssignment token)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
pub enum AstCompoundAssignOp {
  /// `??=`
  AssignNullish,
  /// `||=`
  AssignOr,
  /// `&&=`
  AssignAnd,
  /// `|=`
  AssignBitOr,
  /// `^=`
  AssignBitXor,
  /// `&=`
  AssignBitAnd,
  /// `<<=`
  AssignShl,
  /// `>>=`
  AssignSar,
  /// `>>>=`
  AssignShr,
  /// `*=`
  AssignMul,
  /// `/=`
  AssignDiv,
  /// `%=`
  AssignMod,
  /// `**=`
  AssignExp,
  /// `+=`
  AssignAdd,
  /// `-=`
  AssignSub,
}

impl AstCompoundAssignOp {
  /// 字符表示
  pub const fn as_str(&self) -> &'static str {
    match self {
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
  /// 对应二元运算类别
  pub const fn binary_op(&self) -> AstBinaryOp {
    match self {
      Self::AssignNullish => AstBinaryOp::Nullish,
      Self::AssignOr => AstBinaryOp::Or,
      Self::AssignAnd => AstBinaryOp::And,
      Self::AssignBitOr => AstBinaryOp::BitOr,
      Self::AssignBitXor => AstBinaryOp::BitXor,
      Self::AssignBitAnd => AstBinaryOp::BitAnd,
      Self::AssignShl => AstBinaryOp::Shl,
      Self::AssignSar => AstBinaryOp::Sar,
      Self::AssignShr => AstBinaryOp::Shr,
      Self::AssignMul => AstBinaryOp::Mul,
      Self::AssignDiv => AstBinaryOp::Div,
      Self::AssignMod => AstBinaryOp::Mod,
      Self::AssignExp => AstBinaryOp::Exp,
      Self::AssignAdd => AstBinaryOp::Add,
      Self::AssignSub => AstBinaryOp::Sub,
    }
  }
}

/// 迭代器类型: normal 或 async (原版 enum class IteratorType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
pub enum IteratorType {
  Normal,
  Async,
}

/// for-in / for-of 遍历模式 (原版 ForEachStatement::VisitMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
pub enum ForEachVisitMode {
  Enumerate,
  Iterate,
}

/// 返回语句类型 (原版 ReturnStatement::Type)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
pub enum ReturnType {
  Normal,
  AsyncReturn,
  SyntheticAsyncReturn,
}

/// 空洞检测模式 (原版 enum class HoleCheckMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
pub enum HoleCheckMode {
  Required,
  Elided,
}

/// 变量代理绑定模式 (原版 VariableProxy::BindingMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
pub enum VariableProxyBindingMode {
  Bound,
  Unbound,
}

/// 赋值目标类别 (原版 enum AssignType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
pub enum AssignType {
  NonAssignment,
  Variable,
  NamedProperty,
  KeyedProperty,
  NamedSuperProperty,
  KeyedSuperProperty,
  PrivateMethodOrAccessor,
  LookupResult,
  PrivateField,
}

/// 展开位置类别 (原版 Call::SpreadPosition)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
pub enum SpreadPosition {
  NoSpread,
  HasFinalSpread,
  HasNonFinalSpread,
}

/// 调用类别 (原版 Call::CallType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
pub enum CallType {
  GlobalCall,
  OrdinaryCall,
  ImplicitPrototypeCall,
  NamedPropertyCall,
  KeyedPropertyCall,
  NamedSuperCall,
  KeyedSuperCall,
  PrivateMethodCall,
  TaggedTemplateCall,
  WithCall,
}

/// 属性类别 (原版 ObjectLiteral::Property::Kind)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
pub enum ObjectPropertyKind {
  Constant,
  Computed,
  MaterializedLiteral,
  Prototype,
  Spread,
}

/// 类属性类别 (原版 ClassLiteral::Property::Kind)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
pub enum ClassPropertyKind {
  Method,
  Getter,
  Setter,
  Field,
  AutoAccessor,
}

/// 类静态元素类别 (原版 ClassLiteral::StaticElement::Kind)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
pub enum ClassStaticElementKind {
  Property,
  StaticBlock,
}

/// 函数预编译提示 (原版 FunctionLiteral::EagerCompileHint)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
pub enum EagerCompileHint {
  ShouldEagerCompile,
  ShouldLazyCompile,
}

/// 函数参数标志 (原版 FunctionLiteral::ParameterFlag)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
pub enum ParameterFlag {
  NoDuplicateParameters,
  HasDuplicateParameters,
}

/// 中断恢复模式 (原版 Yield::OnAbruptResume)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, FromRepr)]
pub enum OnAbruptResume {
  OnExceptionThrow,
  NoControl,
}

/// AST 节点基类属性 (对应原版 class AstNode 的 position_ 与 bit_field_)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstNodeBase {
  pub position: i32,
  pub bit_field: u32,
}

impl AstNodeBase {
  /// 创建指定位置与节点类型的基类
  pub const fn new(position: i32, node_type: AstNodeType) -> Self {
    Self {
      position,
      bit_field: node_type as u32,
    }
  }

  /// 节点类型
  pub fn node_type(&self) -> AstNodeType {
    AstNodeType::from_repr((self.bit_field & 0x3f) as u8).unwrap_or(AstNodeType::FailureExpression)
  }

  /// 源码位置
  pub const fn position(&self) -> i32 {
    self.position
  }
}

use crate::ast::{
  ArrayLiteral, Assignment, AstNode, AutoAccessorGetterBody, AutoAccessorSetterBody, Await,
  BinaryOperation, Block, BreakStatement, Call, CallNew, CallRuntime, ClassLiteral,
  CompareOperation, CompoundAssignment, Conditional, ConditionalChain, ContinueStatement,
  CountOperation, DebuggerStatement, DoWhileStatement, EmptyParentheses, EmptyStatement,
  ExpressionStatement, ForInStatement, ForOfStatement, ForStatement, FunctionDeclaration,
  FunctionLiteral, GetTemplateObject, IfStatement, ImportCallExpression,
  InitializeClassMembersStatement, InitializeClassStaticElementsStatement, Literal, NaryOperation,
  NativeFunctionLiteral, ObjectLiteral, OptionalChain, Property, RegExpLiteral, ReturnStatement,
  SloppyBlockFunctionStatement, Spread, SuperCallForwardArgs, SuperCallReference,
  SuperPropertyReference, SwitchStatement, TemplateLiteral, ThisExpression, Throw,
  TryCatchStatement, TryFinallyStatement, UnaryOperation, VariableDeclaration, VariableProxy,
  WhileStatement, WithStatement, Yield, YieldStar,
};

/// 原版 class Statement (STATEMENT_NODE_LIST 展开), 22 种
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
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
}

impl Statement {
  /// 种类数 (与原版 X-macro 展开 1:1)
  pub const COUNT: usize = 22;

  /// 节点类型枚举 (与 AstNodeType 对齐)
  pub const fn node_type(&self) -> AstNodeType {
    match self {
      Self::DoWhileStatement(_) => AstNodeType::DoWhileStatement,
      Self::WhileStatement(_) => AstNodeType::WhileStatement,
      Self::ForStatement(_) => AstNodeType::ForStatement,
      Self::ForInStatement(_) => AstNodeType::ForInStatement,
      Self::ForOfStatement(_) => AstNodeType::ForOfStatement,
      Self::Block(_) => AstNodeType::Block,
      Self::SwitchStatement(_) => AstNodeType::SwitchStatement,
      Self::ExpressionStatement(_) => AstNodeType::ExpressionStatement,
      Self::EmptyStatement(_) => AstNodeType::EmptyStatement,
      Self::SloppyBlockFunctionStatement(_) => AstNodeType::SloppyBlockFunctionStatement,
      Self::IfStatement(_) => AstNodeType::IfStatement,
      Self::ContinueStatement(_) => AstNodeType::ContinueStatement,
      Self::BreakStatement(_) => AstNodeType::BreakStatement,
      Self::ReturnStatement(_) => AstNodeType::ReturnStatement,
      Self::WithStatement(_) => AstNodeType::WithStatement,
      Self::TryCatchStatement(_) => AstNodeType::TryCatchStatement,
      Self::TryFinallyStatement(_) => AstNodeType::TryFinallyStatement,
      Self::DebuggerStatement(_) => AstNodeType::DebuggerStatement,
      Self::InitializeClassMembersStatement(_) => AstNodeType::InitializeClassMembersStatement,
      Self::InitializeClassStaticElementsStatement(_) => {
        AstNodeType::InitializeClassStaticElementsStatement
      }
      Self::AutoAccessorGetterBody(_) => AstNodeType::AutoAccessorGetterBody,
      Self::AutoAccessorSetterBody(_) => AstNodeType::AutoAccessorSetterBody,
    }
  }

  /// 提升回通用节点 (原版向上转型到 AstNode)
  pub fn into_node(self) -> AstNode {
    match self {
      Self::DoWhileStatement(n) => AstNode::DoWhileStatement(n),
      Self::WhileStatement(n) => AstNode::WhileStatement(n),
      Self::ForStatement(n) => AstNode::ForStatement(n),
      Self::ForInStatement(n) => AstNode::ForInStatement(n),
      Self::ForOfStatement(n) => AstNode::ForOfStatement(n),
      Self::Block(n) => AstNode::Block(n),
      Self::SwitchStatement(n) => AstNode::SwitchStatement(n),
      Self::ExpressionStatement(n) => AstNode::ExpressionStatement(n),
      Self::EmptyStatement(n) => AstNode::EmptyStatement(n),
      Self::SloppyBlockFunctionStatement(n) => AstNode::SloppyBlockFunctionStatement(n),
      Self::IfStatement(n) => AstNode::IfStatement(n),
      Self::ContinueStatement(n) => AstNode::ContinueStatement(n),
      Self::BreakStatement(n) => AstNode::BreakStatement(n),
      Self::ReturnStatement(n) => AstNode::ReturnStatement(n),
      Self::WithStatement(n) => AstNode::WithStatement(n),
      Self::TryCatchStatement(n) => AstNode::TryCatchStatement(n),
      Self::TryFinallyStatement(n) => AstNode::TryFinallyStatement(n),
      Self::DebuggerStatement(n) => AstNode::DebuggerStatement(n),
      Self::InitializeClassMembersStatement(n) => AstNode::InitializeClassMembersStatement(n),
      Self::InitializeClassStaticElementsStatement(n) => {
        AstNode::InitializeClassStaticElementsStatement(n)
      }
      Self::AutoAccessorGetterBody(n) => AstNode::AutoAccessorGetterBody(n),
      Self::AutoAccessorSetterBody(n) => AstNode::AutoAccessorSetterBody(n),
    }
  }

  /// 从通用节点静态转换 (原版 As##type 向下转型, 类型不符返回 None)
  pub fn from_node(node: AstNode) -> Option<Self> {
    match node {
      AstNode::DoWhileStatement(n) => Some(Self::DoWhileStatement(n)),
      AstNode::WhileStatement(n) => Some(Self::WhileStatement(n)),
      AstNode::ForStatement(n) => Some(Self::ForStatement(n)),
      AstNode::ForInStatement(n) => Some(Self::ForInStatement(n)),
      AstNode::ForOfStatement(n) => Some(Self::ForOfStatement(n)),
      AstNode::Block(n) => Some(Self::Block(n)),
      AstNode::SwitchStatement(n) => Some(Self::SwitchStatement(n)),
      AstNode::ExpressionStatement(n) => Some(Self::ExpressionStatement(n)),
      AstNode::EmptyStatement(n) => Some(Self::EmptyStatement(n)),
      AstNode::SloppyBlockFunctionStatement(n) => Some(Self::SloppyBlockFunctionStatement(n)),
      AstNode::IfStatement(n) => Some(Self::IfStatement(n)),
      AstNode::ContinueStatement(n) => Some(Self::ContinueStatement(n)),
      AstNode::BreakStatement(n) => Some(Self::BreakStatement(n)),
      AstNode::ReturnStatement(n) => Some(Self::ReturnStatement(n)),
      AstNode::WithStatement(n) => Some(Self::WithStatement(n)),
      AstNode::TryCatchStatement(n) => Some(Self::TryCatchStatement(n)),
      AstNode::TryFinallyStatement(n) => Some(Self::TryFinallyStatement(n)),
      AstNode::DebuggerStatement(n) => Some(Self::DebuggerStatement(n)),
      AstNode::InitializeClassMembersStatement(n) => Some(Self::InitializeClassMembersStatement(n)),
      AstNode::InitializeClassStaticElementsStatement(n) => {
        Some(Self::InitializeClassStaticElementsStatement(n))
      }
      AstNode::AutoAccessorGetterBody(n) => Some(Self::AutoAccessorGetterBody(n)),
      AstNode::AutoAccessorSetterBody(n) => Some(Self::AutoAccessorSetterBody(n)),
      _ => None,
    }
  }
}

/// 原版 class Expression (EXPRESSION_NODE_LIST 展开), 35 种
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
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
}

impl Expression {
  /// 种类数 (与原版 X-macro 展开 1:1)
  pub const COUNT: usize = 35;

  /// 节点类型枚举 (与 AstNodeType 对齐)
  pub const fn node_type(&self) -> AstNodeType {
    match self {
      Self::RegExpLiteral(_) => AstNodeType::RegExpLiteral,
      Self::ObjectLiteral(_) => AstNodeType::ObjectLiteral,
      Self::ArrayLiteral(_) => AstNodeType::ArrayLiteral,
      Self::Assignment(_) => AstNodeType::Assignment,
      Self::Await(_) => AstNodeType::Await,
      Self::BinaryOperation(_) => AstNodeType::BinaryOperation,
      Self::NaryOperation(_) => AstNodeType::NaryOperation,
      Self::Call(_) => AstNodeType::Call,
      Self::SuperCallForwardArgs(_) => AstNodeType::SuperCallForwardArgs,
      Self::CallNew(_) => AstNodeType::CallNew,
      Self::CallRuntime(_) => AstNodeType::CallRuntime,
      Self::ClassLiteral(_) => AstNodeType::ClassLiteral,
      Self::CompareOperation(_) => AstNodeType::CompareOperation,
      Self::CompoundAssignment(_) => AstNodeType::CompoundAssignment,
      Self::ConditionalChain(_) => AstNodeType::ConditionalChain,
      Self::Conditional(_) => AstNodeType::Conditional,
      Self::CountOperation(_) => AstNodeType::CountOperation,
      Self::EmptyParentheses(_) => AstNodeType::EmptyParentheses,
      Self::FunctionLiteral(_) => AstNodeType::FunctionLiteral,
      Self::GetTemplateObject(_) => AstNodeType::GetTemplateObject,
      Self::ImportCallExpression(_) => AstNodeType::ImportCallExpression,
      Self::Literal(_) => AstNodeType::Literal,
      Self::NativeFunctionLiteral(_) => AstNodeType::NativeFunctionLiteral,
      Self::OptionalChain(_) => AstNodeType::OptionalChain,
      Self::Property(_) => AstNodeType::Property,
      Self::Spread(_) => AstNodeType::Spread,
      Self::SuperCallReference(_) => AstNodeType::SuperCallReference,
      Self::SuperPropertyReference(_) => AstNodeType::SuperPropertyReference,
      Self::TemplateLiteral(_) => AstNodeType::TemplateLiteral,
      Self::ThisExpression(_) => AstNodeType::ThisExpression,
      Self::Throw(_) => AstNodeType::Throw,
      Self::UnaryOperation(_) => AstNodeType::UnaryOperation,
      Self::VariableProxy(_) => AstNodeType::VariableProxy,
      Self::Yield(_) => AstNodeType::Yield,
      Self::YieldStar(_) => AstNodeType::YieldStar,
    }
  }

  /// 提升回通用节点 (原版向上转型到 AstNode)
  pub fn into_node(self) -> AstNode {
    match self {
      Self::RegExpLiteral(n) => AstNode::RegExpLiteral(n),
      Self::ObjectLiteral(n) => AstNode::ObjectLiteral(n),
      Self::ArrayLiteral(n) => AstNode::ArrayLiteral(n),
      Self::Assignment(n) => AstNode::Assignment(n),
      Self::Await(n) => AstNode::Await(n),
      Self::BinaryOperation(n) => AstNode::BinaryOperation(n),
      Self::NaryOperation(n) => AstNode::NaryOperation(n),
      Self::Call(n) => AstNode::Call(n),
      Self::SuperCallForwardArgs(n) => AstNode::SuperCallForwardArgs(n),
      Self::CallNew(n) => AstNode::CallNew(n),
      Self::CallRuntime(n) => AstNode::CallRuntime(n),
      Self::ClassLiteral(n) => AstNode::ClassLiteral(n),
      Self::CompareOperation(n) => AstNode::CompareOperation(n),
      Self::CompoundAssignment(n) => AstNode::CompoundAssignment(n),
      Self::ConditionalChain(n) => AstNode::ConditionalChain(n),
      Self::Conditional(n) => AstNode::Conditional(n),
      Self::CountOperation(n) => AstNode::CountOperation(n),
      Self::EmptyParentheses(n) => AstNode::EmptyParentheses(n),
      Self::FunctionLiteral(n) => AstNode::FunctionLiteral(n),
      Self::GetTemplateObject(n) => AstNode::GetTemplateObject(n),
      Self::ImportCallExpression(n) => AstNode::ImportCallExpression(n),
      Self::Literal(n) => AstNode::Literal(n),
      Self::NativeFunctionLiteral(n) => AstNode::NativeFunctionLiteral(n),
      Self::OptionalChain(n) => AstNode::OptionalChain(n),
      Self::Property(n) => AstNode::Property(n),
      Self::Spread(n) => AstNode::Spread(n),
      Self::SuperCallReference(n) => AstNode::SuperCallReference(n),
      Self::SuperPropertyReference(n) => AstNode::SuperPropertyReference(n),
      Self::TemplateLiteral(n) => AstNode::TemplateLiteral(n),
      Self::ThisExpression(n) => AstNode::ThisExpression(n),
      Self::Throw(n) => AstNode::Throw(n),
      Self::UnaryOperation(n) => AstNode::UnaryOperation(n),
      Self::VariableProxy(n) => AstNode::VariableProxy(n),
      Self::Yield(n) => AstNode::Yield(n),
      Self::YieldStar(n) => AstNode::YieldStar(n),
    }
  }

  /// 从通用节点静态转换 (原版 As##type 向下转型, 类型不符返回 None)
  pub fn from_node(node: AstNode) -> Option<Self> {
    match node {
      AstNode::RegExpLiteral(n) => Some(Self::RegExpLiteral(n)),
      AstNode::ObjectLiteral(n) => Some(Self::ObjectLiteral(n)),
      AstNode::ArrayLiteral(n) => Some(Self::ArrayLiteral(n)),
      AstNode::Assignment(n) => Some(Self::Assignment(n)),
      AstNode::Await(n) => Some(Self::Await(n)),
      AstNode::BinaryOperation(n) => Some(Self::BinaryOperation(n)),
      AstNode::NaryOperation(n) => Some(Self::NaryOperation(n)),
      AstNode::Call(n) => Some(Self::Call(n)),
      AstNode::SuperCallForwardArgs(n) => Some(Self::SuperCallForwardArgs(n)),
      AstNode::CallNew(n) => Some(Self::CallNew(n)),
      AstNode::CallRuntime(n) => Some(Self::CallRuntime(n)),
      AstNode::ClassLiteral(n) => Some(Self::ClassLiteral(n)),
      AstNode::CompareOperation(n) => Some(Self::CompareOperation(n)),
      AstNode::CompoundAssignment(n) => Some(Self::CompoundAssignment(n)),
      AstNode::ConditionalChain(n) => Some(Self::ConditionalChain(n)),
      AstNode::Conditional(n) => Some(Self::Conditional(n)),
      AstNode::CountOperation(n) => Some(Self::CountOperation(n)),
      AstNode::EmptyParentheses(n) => Some(Self::EmptyParentheses(n)),
      AstNode::FunctionLiteral(n) => Some(Self::FunctionLiteral(n)),
      AstNode::GetTemplateObject(n) => Some(Self::GetTemplateObject(n)),
      AstNode::ImportCallExpression(n) => Some(Self::ImportCallExpression(n)),
      AstNode::Literal(n) => Some(Self::Literal(n)),
      AstNode::NativeFunctionLiteral(n) => Some(Self::NativeFunctionLiteral(n)),
      AstNode::OptionalChain(n) => Some(Self::OptionalChain(n)),
      AstNode::Property(n) => Some(Self::Property(n)),
      AstNode::Spread(n) => Some(Self::Spread(n)),
      AstNode::SuperCallReference(n) => Some(Self::SuperCallReference(n)),
      AstNode::SuperPropertyReference(n) => Some(Self::SuperPropertyReference(n)),
      AstNode::TemplateLiteral(n) => Some(Self::TemplateLiteral(n)),
      AstNode::ThisExpression(n) => Some(Self::ThisExpression(n)),
      AstNode::Throw(n) => Some(Self::Throw(n)),
      AstNode::UnaryOperation(n) => Some(Self::UnaryOperation(n)),
      AstNode::VariableProxy(n) => Some(Self::VariableProxy(n)),
      AstNode::Yield(n) => Some(Self::Yield(n)),
      AstNode::YieldStar(n) => Some(Self::YieldStar(n)),
      _ => None,
    }
  }
}

/// 原版 class Declaration (DECLARATION_NODE_LIST 展开), 2 种
#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
  VariableDeclaration(VariableDeclaration),
  FunctionDeclaration(FunctionDeclaration),
}

impl Declaration {
  /// 种类数 (与原版 X-macro 展开 1:1)
  pub const COUNT: usize = 2;

  /// 节点类型枚举 (与 AstNodeType 对齐)
  pub const fn node_type(&self) -> AstNodeType {
    match self {
      Self::VariableDeclaration(_) => AstNodeType::VariableDeclaration,
      Self::FunctionDeclaration(_) => AstNodeType::FunctionDeclaration,
    }
  }

  /// 提升回通用节点 (原版向上转型到 AstNode)
  pub fn into_node(self) -> AstNode {
    match self {
      Self::VariableDeclaration(n) => AstNode::VariableDeclaration(n),
      Self::FunctionDeclaration(n) => AstNode::FunctionDeclaration(n),
    }
  }

  /// 从通用节点静态转换 (原版 As##type 向下转型, 类型不符返回 None)
  pub fn from_node(node: AstNode) -> Option<Self> {
    match node {
      AstNode::VariableDeclaration(n) => Some(Self::VariableDeclaration(n)),
      AstNode::FunctionDeclaration(n) => Some(Self::FunctionDeclaration(n)),
      _ => None,
    }
  }
}

// ── 对数不变式断言 ──
const _: () = {
  assert!(AstNodeType::COUNT == 60);
  assert!(AstNodeType::AST_NODE_COUNT == 59);
  assert!(AstNodeType::DECLARATION_COUNT == 2);
  assert!(AstNodeType::STATEMENT_COUNT == 22);
  assert!(AstNodeType::EXPRESSION_COUNT == 35);
  assert!(NO_SOURCE_POSITION == -1);
  assert!(FUNCTION_LITERAL_RETURN_POSITION == -2);
};
