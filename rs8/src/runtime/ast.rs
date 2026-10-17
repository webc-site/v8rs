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

/// 对象字面量属性种类
///
/// 原版靠值位置 `FunctionLiteral` 的 kind 区分 get/set/普通方法, 移植侧的函数值不携带这个信息,
/// 故把种类提到属性节点上
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropKind {
  Data,
  Get,
  Set,
}

/// 对象字面量的一项属性
#[derive(Debug, Clone)]
pub struct ObjectProp {
  pub key: ObjectKey,
  pub value: Expr,
  pub kind: PropKind,
}

/// 解构模式 (12.38): parser 构造期的纯中间形态 —— `parse_program`/`parse_block` 出口的
/// desugar 单点把它展开成普通 VarDecl/赋值语句序列后才进任何执行链,
/// AST 解释器与字节码桥都看不到这些节点
#[derive(Debug, Clone)]
pub enum Pattern {
  Binding(String),
  /// 数组模式: 空位 (hole) = None; `...rest` 只能出现在末位
  Array {
    elements: Vec<Option<Pattern>>,
    rest: Option<Box<Pattern>>,
  },
  /// 对象模式: (键名, 子模式); 短语法 `{x}` 的键与 Binding 名同
  Object {
    props: Vec<(String, Pattern)>,
    rest: Option<Box<Pattern>>,
  },
  /// `pat = 默认值`: 实取值为 undefined 时触发 (node 口径, 非缺省判定)
  Default {
    pat: Box<Pattern>,
    fallback: Box<Expr>,
  },
}

/// for-of 头部目标 (12.41 暂态, 展开点见 parser `expand_forof`): 同为两链不可见形态
#[derive(Debug, Clone)]
pub enum ForOfHead {
  /// 标识符绑定 (声明式 `for (const x of …)` 与非声明式 `for (x of …)` 同型, var 语义)
  Binding(String),
  /// 解构模式 `for (const [a,b] of …)` / `for ([a] of …)`
  Pattern(Pattern),
  /// 成员/下标赋值目标 `for (o.p of …)` / `for (o[k] of …)`
  Target(Expr),
}

/// 表达式 AST 节点
#[derive(Debug, Clone)]
pub enum Expr {
  Literal(JsValue),
  /// 正则字面量 (战役 C-④a): 解析期只存 pattern/flags, 每次求值新建实例
  /// (node/规范口径"per-eval 新对象"); 编译体走 `RegExpCore` 进程级缓存, 不付重编译税
  RegExp {
    pattern: String,
    flags: String,
  },
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
  /// 语句位解构赋值 `[a,b] = ...` (12.38): 与 VarPattern 同为暂态, desugar 就地展开;
  /// 括号形表达式位解构 (`([a]=b)` 作子表达式) 本轮不支持, 记账十五 B
  AssignPattern {
    pat: Pattern,
    value: Box<Expr>,
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
  Object(Vec<ObjectProp>),
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
  /// 解构声明 `var/let/const [a] = ...` / `{x} = ...` (12.38, 暂态节点见 Pattern)
  VarPattern {
    pat: Pattern,
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
  /// for-of 迭代语句 (17 轮): 暂态节点, desugar 单点摊成 `$src/$idx` 三段式 For 索引循环
  /// (node 数组迭代器本就是活长度索引循环, 逐位口径天然对齐), 两链执行器零感知
  ForOf {
    head: ForOfHead,
    src: Expr,
    body: Box<Stmt>,
  },
  FunctionDecl(Arc<SharedFunctionInfo>),
  Return(Option<Expr>),
  /// `break` / `break L` (Some = 目标标签; 合法性由 parser 解析期校验, 同原版 target 指针)
  Break(Option<String>),
  /// `continue` / `continue L` (L 必指迭代语句, 同校验)
  Continue(Option<String>),
  /// 标记语句 `L1: L2: stmt` (连续标签收进一条节点, 同原版 labels 头链)
  Labeled {
    labels: Vec<String>,
    body: Box<Stmt>,
  },
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
