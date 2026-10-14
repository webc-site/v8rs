//! JavaScript 轻量抽象语法树 (AST) 定义

use std::sync::Arc;

use crate::runtime::value::{JsValue, SharedFunctionInfo};

/// 二元运算符
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  BitAnd,
  BitOr,
  BitXor,
  Shl,
  Shr,
  Sar,
  Eq,
  NotEq,
  StrictEq,
  StrictNotEq,
  Lt,
  Lte,
  Gt,
  Gte,
  InstanceOf,
  In,
  LogicalAnd,
  LogicalOr,
  Comma,
}

/// 一元运算符
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
  Neg,
  Pos,
  Not,
  BitNot,
  PreInc,
  PreDec,
  PostInc,
  PostDec,
  Void,
  Delete,
}

/// 赋值运算符
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignOp {
  Assign,
  AddAssign,
  SubAssign,
  MulAssign,
  DivAssign,
  ModAssign,
  BitAndAssign,
  BitOrAssign,
  BitXorAssign,
  ShlAssign,
  ShrAssign,
  SarAssign,
}

/// 对象键类型 (标识符或计算属性)
#[derive(Debug, Clone)]
pub enum ObjectKey {
  Ident(String),
  Computed(Expr),
}

/// 表达式 AST 节点
#[derive(Debug, Clone)]
pub enum Expr {
  Literal(JsValue),
  Identifier(String),
  Unary {
    op: UnaryOp,
    expr: Box<Expr>,
  },
  Binary {
    op: BinaryOp,
    left: Box<Expr>,
    right: Box<Expr>,
  },
  Assign {
    target: Box<Expr>,
    op: AssignOp,
    value: Box<Expr>,
  },
  Call {
    callee: Box<Expr>,
    args: Vec<Expr>,
  },
  Member {
    object: Box<Expr>,
    property: String,
  },
  Index {
    object: Box<Expr>,
    index: Box<Expr>,
  },
  Array(Vec<Expr>),
  Object(Vec<(ObjectKey, Expr)>),
  FunctionExpr(Arc<SharedFunctionInfo>),
  This,
  New {
    callee: Box<Expr>,
    args: Vec<Expr>,
  },
  Typeof(Box<Expr>),
  Conditional {
    condition: Box<Expr>,
    consequent: Box<Expr>,
    alternate: Box<Expr>,
  },
}

/// Switch case 分支
#[derive(Debug, Clone)]
pub struct SwitchCase {
  pub test: Option<Expr>,
  pub body: Vec<Stmt>,
}

/// 语句 AST 节点
#[derive(Debug, Clone)]
pub enum Stmt {
  Expr(Expr),
  VarDecl {
    name: String,
    init: Option<Expr>,
  },
  VarDecls(Vec<(String, Option<Expr>)>),
  Block(Vec<Stmt>),
  If {
    cond: Expr,
    then_branch: Box<Stmt>,
    else_branch: Option<Box<Stmt>>,
  },
  While {
    cond: Expr,
    body: Box<Stmt>,
  },
  DoWhile {
    body: Box<Stmt>,
    cond: Expr,
  },
  For {
    init: Option<Box<Stmt>>,
    cond: Option<Expr>,
    step: Option<Expr>,
    body: Box<Stmt>,
  },
  ForIn {
    variable: String,
    is_var: bool,
    object: Expr,
    body: Box<Stmt>,
  },
  FunctionDecl(Arc<SharedFunctionInfo>),
  Return(Option<Expr>),
  Break,
  Continue,
  Switch {
    discriminant: Expr,
    cases: Vec<SwitchCase>,
  },
  Try {
    block: Vec<Stmt>,
    catch_param: Option<String>,
    catch_block: Option<Vec<Stmt>>,
    finally_block: Option<Vec<Stmt>>,
  },
  Throw(Expr),
  Empty,
}
