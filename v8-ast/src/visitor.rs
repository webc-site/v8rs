//! AST 访问者模式 (v8/src/ast/ast.h 中的 AstVisitor)

use crate::ast::*;

/// AST 访问者 Trait (原版 template <class Subclass> class AstVisitor)
pub trait AstVisitor {
  /// 统一节点入口。默认调用 `enter_node` -> `walk_node` -> `leave_node`
  fn visit(&mut self, node: &AstNode) {
    if !self.enter_node(node) {
      return;
    }
    self.walk_node(node);
    self.leave_node(node);
  }

  /// 进入节点的回调。返回 false 则跳过子节点遍历 (剪枝)。默认返回 true。
  fn enter_node(&mut self, _node: &AstNode) -> bool {
    true
  }

  /// 离开节点的回调 (回溯/后序遍历)。默认空实现。
  fn leave_node(&mut self, _node: &AstNode) {}

  /// 遍历分发函数: 分发到具体的 visit_* 方法
  fn walk_node(&mut self, node: &AstNode) {
    match node {
      AstNode::VariableDeclaration(n) => self.visit_variable_declaration(n),
      AstNode::FunctionDeclaration(n) => self.visit_function_declaration(n),
      AstNode::DoWhileStatement(n) => self.visit_do_while_statement(n),
      AstNode::WhileStatement(n) => self.visit_while_statement(n),
      AstNode::ForStatement(n) => self.visit_for_statement(n),
      AstNode::ForInStatement(n) => self.visit_for_in_statement(n),
      AstNode::ForOfStatement(n) => self.visit_for_of_statement(n),
      AstNode::Block(n) => self.visit_block(n),
      AstNode::SwitchStatement(n) => self.visit_switch_statement(n),
      AstNode::ExpressionStatement(n) => self.visit_expression_statement(n),
      AstNode::EmptyStatement(n) => self.visit_empty_statement(n),
      AstNode::SloppyBlockFunctionStatement(n) => self.visit_sloppy_block_function_statement(n),
      AstNode::IfStatement(n) => self.visit_if_statement(n),
      AstNode::ContinueStatement(n) => self.visit_continue_statement(n),
      AstNode::BreakStatement(n) => self.visit_break_statement(n),
      AstNode::ReturnStatement(n) => self.visit_return_statement(n),
      AstNode::WithStatement(n) => self.visit_with_statement(n),
      AstNode::TryCatchStatement(n) => self.visit_try_catch_statement(n),
      AstNode::TryFinallyStatement(n) => self.visit_try_finally_statement(n),
      AstNode::DebuggerStatement(n) => self.visit_debugger_statement(n),
      AstNode::InitializeClassMembersStatement(n) => {
        self.visit_initialize_class_members_statement(n)
      }
      AstNode::InitializeClassStaticElementsStatement(n) => {
        self.visit_initialize_class_static_elements_statement(n)
      }
      AstNode::AutoAccessorGetterBody(n) => self.visit_auto_accessor_getter_body(n),
      AstNode::AutoAccessorSetterBody(n) => self.visit_auto_accessor_setter_body(n),
      AstNode::RegExpLiteral(n) => self.visit_reg_exp_literal(n),
      AstNode::ObjectLiteral(n) => self.visit_object_literal(n),
      AstNode::ArrayLiteral(n) => self.visit_array_literal(n),
      AstNode::Assignment(n) => self.visit_assignment(n),
      AstNode::Await(n) => self.visit_await(n),
      AstNode::BinaryOperation(n) => self.visit_binary_operation(n),
      AstNode::NaryOperation(n) => self.visit_nary_operation(n),
      AstNode::Call(n) => self.visit_call(n),
      AstNode::SuperCallForwardArgs(n) => self.visit_super_call_forward_args(n),
      AstNode::CallNew(n) => self.visit_call_new(n),
      AstNode::CallRuntime(n) => self.visit_call_runtime(n),
      AstNode::ClassLiteral(n) => self.visit_class_literal(n),
      AstNode::CompareOperation(n) => self.visit_compare_operation(n),
      AstNode::CompoundAssignment(n) => self.visit_compound_assignment(n),
      AstNode::ConditionalChain(n) => self.visit_conditional_chain(n),
      AstNode::Conditional(n) => self.visit_conditional(n),
      AstNode::CountOperation(n) => self.visit_count_operation(n),
      AstNode::EmptyParentheses(n) => self.visit_empty_parentheses(n),
      AstNode::FunctionLiteral(n) => self.visit_function_literal(n),
      AstNode::GetTemplateObject(n) => self.visit_get_template_object(n),
      AstNode::ImportCallExpression(n) => self.visit_import_call_expression(n),
      AstNode::Literal(n) => self.visit_literal(n),
      AstNode::NativeFunctionLiteral(n) => self.visit_native_function_literal(n),
      AstNode::OptionalChain(n) => self.visit_optional_chain(n),
      AstNode::Property(n) => self.visit_property(n),
      AstNode::Spread(n) => self.visit_spread(n),
      AstNode::SuperCallReference(n) => self.visit_super_call_reference(n),
      AstNode::SuperPropertyReference(n) => self.visit_super_property_reference(n),
      AstNode::TemplateLiteral(n) => self.visit_template_literal(n),
      AstNode::ThisExpression(n) => self.visit_this_expression(n),
      AstNode::Throw(n) => self.visit_throw(n),
      AstNode::UnaryOperation(n) => self.visit_unary_operation(n),
      AstNode::VariableProxy(n) => self.visit_variable_proxy(n),
      AstNode::Yield(n) => self.visit_yield(n),
      AstNode::YieldStar(n) => self.visit_yield_star(n),
      AstNode::FailureExpression(n) => self.visit_failure_expression(n),
    }
  }

  /// 批量访问节点列表
  fn visit_nodes(&mut self, nodes: &[AstNode]) {
    for node in nodes {
      self.visit(node);
    }
  }

  fn visit_variable_declaration(&mut self, _node: &VariableDeclaration) {}
  fn visit_function_declaration(&mut self, node: &FunctionDeclaration) {
    if let Some(fun) = &node.fun {
      self.visit_function_literal(fun);
    }
  }
  fn visit_do_while_statement(&mut self, node: &DoWhileStatement) {
    self.visit(&node.body);
    self.visit(&node.cond);
  }
  fn visit_while_statement(&mut self, node: &WhileStatement) {
    self.visit(&node.cond);
    self.visit(&node.body);
  }
  fn visit_for_statement(&mut self, node: &ForStatement) {
    if let Some(init) = &node.init {
      self.visit(init);
    }
    if let Some(cond) = &node.cond {
      self.visit(cond);
    }
    if let Some(next) = &node.next {
      self.visit(next);
    }
    self.visit(&node.body);
  }
  fn visit_for_in_statement(&mut self, node: &ForInStatement) {
    self.visit(&node.each);
    self.visit(&node.subject);
    self.visit(&node.body);
  }
  fn visit_for_of_statement(&mut self, node: &ForOfStatement) {
    self.visit(&node.each);
    self.visit(&node.subject);
    self.visit(&node.body);
  }
  fn visit_block(&mut self, node: &Block) {
    self.visit_nodes(&node.statements);
  }
  fn visit_switch_statement(&mut self, node: &SwitchStatement) {
    self.visit(&node.tag);
    for case in &node.cases {
      if let Some(label) = &case.label {
        self.visit(label);
      }
      self.visit_nodes(&case.statements);
    }
  }
  fn visit_expression_statement(&mut self, node: &ExpressionStatement) {
    self.visit(&node.expression);
  }
  fn visit_empty_statement(&mut self, _node: &EmptyStatement) {}
  fn visit_sloppy_block_function_statement(&mut self, node: &SloppyBlockFunctionStatement) {
    self.visit(&node.statement);
  }
  fn visit_if_statement(&mut self, node: &IfStatement) {
    self.visit(&node.condition);
    self.visit(&node.then_statement);
    if let Some(else_stmt) = &node.else_statement {
      self.visit(else_stmt);
    }
  }
  fn visit_continue_statement(&mut self, _node: &ContinueStatement) {}
  fn visit_break_statement(&mut self, _node: &BreakStatement) {}
  fn visit_return_statement(&mut self, node: &ReturnStatement) {
    if let Some(expr) = &node.expression {
      self.visit(expr);
    }
  }
  fn visit_with_statement(&mut self, node: &WithStatement) {
    self.visit(&node.object);
    self.visit(&node.statement);
  }
  fn visit_try_catch_statement(&mut self, node: &TryCatchStatement) {
    self.visit_block(&node.try_block);
    self.visit_block(&node.catch_block);
  }
  fn visit_try_finally_statement(&mut self, node: &TryFinallyStatement) {
    self.visit_block(&node.try_block);
    self.visit_block(&node.finally_block);
  }
  fn visit_debugger_statement(&mut self, _node: &DebuggerStatement) {}
  fn visit_initialize_class_members_statement(&mut self, node: &InitializeClassMembersStatement) {
    for field in &node.fields {
      self.visit(&field.key);
      self.visit(&field.value);
    }
  }
  fn visit_initialize_class_static_elements_statement(
    &mut self,
    node: &InitializeClassStaticElementsStatement,
  ) {
    for elem in &node.elements {
      if let Some(prop) = &elem.property {
        self.visit(&prop.key);
        self.visit(&prop.value);
      }
      if let Some(block) = &elem.block {
        self.visit_block(block);
      }
    }
  }
  fn visit_auto_accessor_getter_body(&mut self, node: &AutoAccessorGetterBody) {
    if let Some(proxy) = &node.name_proxy {
      self.visit_variable_proxy(proxy);
    }
  }
  fn visit_auto_accessor_setter_body(&mut self, node: &AutoAccessorSetterBody) {
    if let Some(proxy) = &node.name_proxy {
      self.visit_variable_proxy(proxy);
    }
  }
  fn visit_reg_exp_literal(&mut self, _node: &RegExpLiteral) {}
  fn visit_object_literal(&mut self, node: &ObjectLiteral) {
    for prop in &node.properties {
      self.visit(&prop.key);
      self.visit(&prop.value);
    }
  }
  fn visit_array_literal(&mut self, node: &ArrayLiteral) {
    self.visit_nodes(&node.values);
  }
  fn visit_assignment(&mut self, node: &Assignment) {
    self.visit(&node.target);
    self.visit(&node.value);
  }
  fn visit_await(&mut self, node: &Await) {
    self.visit(&node.expression);
  }
  fn visit_binary_operation(&mut self, node: &BinaryOperation) {
    self.visit(&node.left);
    self.visit(&node.right);
  }
  fn visit_nary_operation(&mut self, node: &NaryOperation) {
    self.visit(&node.first);
    self.visit_nodes(&node.subsequent);
  }
  fn visit_call(&mut self, node: &Call) {
    self.visit(&node.expression);
    self.visit_nodes(&node.arguments);
  }
  fn visit_super_call_forward_args(&mut self, node: &SuperCallForwardArgs) {
    self.visit(&node.expression);
  }
  fn visit_call_new(&mut self, node: &CallNew) {
    self.visit(&node.expression);
    self.visit_nodes(&node.arguments);
  }
  fn visit_call_runtime(&mut self, node: &CallRuntime) {
    self.visit_nodes(&node.arguments);
  }
  fn visit_class_literal(&mut self, node: &ClassLiteral) {
    if let Some(ext) = &node.extends {
      self.visit(ext);
    }
    if let Some(ctor) = &node.constructor {
      self.visit_function_literal(ctor);
    }
    for prop in node.public_members.iter().chain(&node.private_members) {
      self.visit(&prop.key);
      self.visit(&prop.value);
    }
    for elem in &node.static_elements {
      if let Some(prop) = &elem.property {
        self.visit(&prop.key);
        self.visit(&prop.value);
      }
      if let Some(block) = &elem.block {
        self.visit_block(block);
      }
    }
    if let Some(init) = &node.static_initializer {
      self.visit_function_literal(init);
    }
    if let Some(init) = &node.instance_members_initializer {
      self.visit_function_literal(init);
    }
  }
  fn visit_compare_operation(&mut self, node: &CompareOperation) {
    self.visit(&node.left);
    self.visit(&node.right);
  }
  fn visit_compound_assignment(&mut self, node: &CompoundAssignment) {
    self.visit(&node.target);
    self.visit(&node.value);
  }
  fn visit_conditional_chain(&mut self, node: &ConditionalChain) {
    for (cond, then_expr) in &node.conditions {
      self.visit(cond);
      self.visit(then_expr);
    }
    self.visit(&node.else_expression);
  }
  fn visit_conditional(&mut self, node: &Conditional) {
    self.visit(&node.condition);
    self.visit(&node.then_expression);
    self.visit(&node.else_expression);
  }
  fn visit_count_operation(&mut self, node: &CountOperation) {
    self.visit(&node.expression);
  }
  fn visit_empty_parentheses(&mut self, _node: &EmptyParentheses) {}
  fn visit_function_literal(&mut self, node: &FunctionLiteral) {
    self.visit_nodes(&node.body);
  }
  fn visit_get_template_object(&mut self, _node: &GetTemplateObject) {}
  fn visit_import_call_expression(&mut self, node: &ImportCallExpression) {
    self.visit(&node.specifier);
    if let Some(opts) = &node.options {
      self.visit(opts);
    }
  }
  fn visit_literal(&mut self, _node: &Literal) {}
  fn visit_native_function_literal(&mut self, _node: &NativeFunctionLiteral) {}
  fn visit_optional_chain(&mut self, node: &OptionalChain) {
    self.visit(&node.expression);
  }
  fn visit_property(&mut self, node: &Property) {
    self.visit(&node.object);
    self.visit(&node.key);
  }
  fn visit_spread(&mut self, node: &Spread) {
    self.visit(&node.expression);
  }
  fn visit_super_call_reference(&mut self, node: &SuperCallReference) {
    if let Some(nt) = &node.new_target_var {
      self.visit_variable_proxy(nt);
    }
    if let Some(tf) = &node.this_function_var {
      self.visit_variable_proxy(tf);
    }
  }
  fn visit_super_property_reference(&mut self, node: &SuperPropertyReference) {
    if let Some(proxy) = &node.home_object {
      self.visit_variable_proxy(proxy);
    }
  }
  fn visit_template_literal(&mut self, node: &TemplateLiteral) {
    self.visit_nodes(&node.substitutions);
  }
  fn visit_this_expression(&mut self, _node: &ThisExpression) {}
  fn visit_throw(&mut self, node: &Throw) {
    self.visit(&node.exception);
  }
  fn visit_unary_operation(&mut self, node: &UnaryOperation) {
    self.visit(&node.expression);
  }
  fn visit_variable_proxy(&mut self, _node: &VariableProxy) {}
  fn visit_yield(&mut self, node: &Yield) {
    if let Some(expr) = &node.expression {
      self.visit(expr);
    }
  }
  fn visit_yield_star(&mut self, node: &YieldStar) {
    self.visit(&node.expression);
  }
  fn visit_failure_expression(&mut self, _node: &FailureExpression) {}
}
