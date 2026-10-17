// 由 tool/src/ast/ast-source-ranges.h.js 从 v8/src/ast/ast-source-ranges.h + source-range-ast-visitor.{h,cc} + prettyprinter.{h,cc} + ast-traversal-visitor.h + ast-function-literal-id-reindexer.{h,cc} 生成; 修订就地改, 复跑覆盖
// AST 范围/打印/遍历/重排骨架

use std::collections::BTreeSet;

use crate::ast::AstNode;
/// 源码范围类别 (原版 enum class SourceRangeKind, 7 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SourceRangeKind {
  /// `kBody`
  Body = 0,
  /// `kCatch`
  Catch = 1,
  /// `kContinuation`
  Continuation = 2,
  /// `kElse`
  Else = 3,
  /// `kFinally`
  Finally = 4,
  /// `kRight`
  Right = 5,
  /// `kThen`
  Then = 6,
}

impl SourceRangeKind {
  pub const COUNT: usize = 7;
}

/// 节点范围表类族 (原版 ast-source-ranges.h, 11 类, 各挂若干 SourceRangeKind)
pub const RANGE_TABLE_CLASSES: [&str; 11] = [
  "BinaryOperationSourceRanges",
  "CaseClauseSourceRanges",
  "ConditionalChainSourceRanges",
  "ConditionalSourceRanges",
  "FunctionLiteralSourceRanges",
  "IfStatementSourceRanges",
  "IterationStatementSourceRanges",
  "NaryOperationSourceRanges",
  "ExpressionSourceRanges",
  "TryCatchStatementSourceRanges",
  "TryFinallyStatementSourceRanges",
];

use std::fmt::Write;

use crate::{ast::*, visitor::AstVisitor};

/// 遍历访问器 Trait (原版 v8/src/ast/ast-traversal-visitor.h AstTraversalVisitor)
pub trait AstTraversalVisitor: AstVisitor {
  /// 是否继续遍历该节点
  fn visit_node(&mut self, _node: &AstNode) -> bool {
    true
  }

  /// 是否继续遍历该表达式节点
  fn visit_expression(&mut self, _node: &AstNode) -> bool {
    true
  }

  /// 批量访问语句节点 (原版 VisitStatements)
  fn visit_statements(&mut self, statements: &[AstNode]) {
    for stmt in statements {
      self.visit(stmt);
    }
  }

  /// 批量访问表达式节点 (原版 VisitExpressions)
  fn visit_expressions(&mut self, expressions: &[AstNode]) {
    for expr in expressions {
      self.visit(expr);
    }
  }

  /// 批量访问声明节点 (原版 VisitDeclarations)
  fn visit_declarations(&mut self, declarations: &[AstNode]) {
    for decl in declarations {
      self.visit(decl);
    }
  }
}

/// 默认 AST 全遍历访问器
#[derive(Debug, Default, Clone)]
pub struct DefaultAstTraversalVisitor {
  pub visited_count: usize,
}

impl AstVisitor for DefaultAstTraversalVisitor {
  fn enter_node(&mut self, _node: &AstNode) -> bool {
    self.visited_count += 1;
    true
  }
}

impl AstTraversalVisitor for DefaultAstTraversalVisitor {}

/// 简单缩进 AST 打印器 (原版 AstPrinter)
#[derive(Debug, Default)]
pub struct SimpleAstPrinter {
  output: String,
  indent: usize,
}

impl SimpleAstPrinter {
  pub fn new() -> Self {
    Self::default()
  }

  fn write_indent(&mut self) {
    for _ in 0..self.indent {
      self.output.push_str("  ");
    }
  }

  pub fn finish(self) -> String {
    self.output
  }
}

impl AstVisitor for SimpleAstPrinter {
  fn enter_node(&mut self, node: &AstNode) -> bool {
    self.write_indent();
    match node {
      AstNode::VariableDeclaration(v) => {
        let _ = writeln!(self.output, "var {:?};", v.base.position);
      }
      AstNode::FunctionDeclaration(f) => {
        let _ = writeln!(self.output, "function {:?}();", f.base.position);
      }
      AstNode::Block(_) => {
        self.output.push_str("{\n");
        self.indent += 1;
      }
      AstNode::ReturnStatement(_) => {
        self.output.push_str("return;\n");
      }
      AstNode::IfStatement(_) => {
        self.output.push_str("if (...)\n");
      }
      AstNode::Literal(lit) => {
        let _ = writeln!(self.output, "Literal({:?})", lit.base.position);
      }
      AstNode::VariableProxy(_) => {
        self.output.push_str("VariableProxy\n");
      }
      _ => {
        let _ = writeln!(self.output, "Node({:?})", node.node_type());
      }
    }
    true
  }

  fn leave_node(&mut self, node: &AstNode) {
    if matches!(node, AstNode::Block(_)) {
      self.indent = self.indent.saturating_sub(1);
      self.write_indent();
      self.output.push_str("}\n");
    }
  }
}

/// 函数字面量 id 重排器 (原版 AstFunctionLiteralIdReindexer)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionLiteralIdReindexer {
  pub delta: i32,
}

impl FunctionLiteralIdReindexer {
  #[inline]
  pub const fn new(delta: i32) -> Self {
    Self { delta }
  }

  #[inline]
  pub const fn reindex_id(&self, old_id: i32) -> i32 {
    old_id.wrapping_add(self.delta)
  }
}

/// 源码范围收集器 (原版 SourceRangeAstVisitor)
#[derive(Debug, Default, Clone)]
pub struct SourceRangeCollector {
  pub continuation_positions: BTreeSet<i32>,
  pub ranges: Vec<(SourceRangeKind, i32, i32)>,
}

impl SourceRangeCollector {
  #[inline]
  pub fn new() -> Self {
    Self::default()
  }

  pub fn record_range(&mut self, kind: SourceRangeKind, start: i32, end: i32) {
    if kind == SourceRangeKind::Continuation && !self.continuation_positions.insert(start) {
      return;
    }
    self.ranges.push((kind, start, end));
  }

  pub fn maybe_remove_continuation_range(&mut self, pos: i32) {
    self.continuation_positions.remove(&pos);
    self
      .ranges
      .retain(|(k, s, _)| !(*k == SourceRangeKind::Continuation && *s == pos));
  }
}

/// 打印器骨架 (语义见手写 visitor.rs)
pub mod ast_printer_skel {
  use super::print_ast;
  use crate::ast::AstNode;

  /// 调用打印器: 找出可能抛错的调用点 (原版 CallPrinter; spread 误用提示双态)
  pub const SPREAD_HINT_STATES: [&str; 2] = ["kErrorInArgs", "kNoErrorInArgs"];
  /// AST 打印器: 缩进式反解析 (原版 AstPrinter; IndentedScope 栈式缩进)
  pub const PRINTER_CLASSES: [&str; 2] = ["CallPrinter", "AstPrinter"];

  /// 打印入口 (原版 AstPrinter::PrintProgram)
  ///
  /// # Safety
  /// `program` 必须为有效的 `AstNode` 指针或 0
  pub unsafe fn print(program: usize) -> String {
    if program == 0 {
      return String::new();
    }
    let node = unsafe { &*(program as *const AstNode) };
    print_ast(node)
  }
}

/// 安全打印 AST 节点
pub fn print_ast(node: &AstNode) -> String {
  use crate::visitor::AstVisitor;
  let mut printer = SimpleAstPrinter::new();
  printer.visit(node);
  printer.finish()
}

/// 遍历/重排骨架
pub mod ast_visitor_skel {
  use super::{DefaultAstTraversalVisitor, FunctionLiteralIdReindexer, SourceRangeCollector};
  use crate::{ast::AstNode, visitor::AstVisitor};

  /// 通用遍历 (原版 AstTraversalVisitor: 模板 DFS, 默认不递归子表达式可短路)
  ///
  /// # Safety
  /// `node` 必须为有效的 `AstNode` 指针或 0
  pub unsafe fn traverse(node: usize) {
    if node == 0 {
      return;
    }
    let node_ref = unsafe { &*(node as *const AstNode) };
    traverse_ast(node_ref);
  }

  /// 安全遍历 AST 节点
  pub fn traverse_ast(node: &AstNode) {
    let mut visitor = DefaultAstTraversalVisitor::default();
    visitor.visit(node);
  }

  /// 函数字面量 id 重排 (原版 AstFunctionLiteralIdReindexer:
  /// 快照恢复时把 id 段平移到新起点)
  pub fn reindex_function_literal_ids(base: i32, old_id: i32) -> i32 {
    let reindexer = FunctionLiteralIdReindexer::new(base.wrapping_sub(old_id));
    reindexer.reindex_id(old_id)
  }

  /// 源码范围访问器 (原版 SourceRangeAstVisitor: try/catch/finally 与
  /// 逻辑操作续接范围收集)
  pub fn collect_source_ranges() -> SourceRangeCollector {
    SourceRangeCollector::default()
  }
}
