// 由 tool/src/torque/visitor.js 从 v8/src/torque/ast-visitor.h 生成; 修订就地改, 复跑覆盖
// AST 访问器接口 (CRTP → trait, 60 节点)

use std::mem;

use crate::{
  ast::{Ast, AstNodeKind, NodeId},
  error::Result,
};

// C++ CRTP AstVisitor<Impl> → Rust trait: 每节点一个 visit_*, 由 visit_node 按
// kind 分派 (默认实现); 返回 Option<NodeId>: None = 删除节点 (对应原版返回 nullptr)。
// 实现方: TSAGenerator (原版唯一继承者, impl 见 tsa_generator.rs)。
/// 原版 AstVisitor<Derived> — visit 方法族
pub trait AstVisitor {
  /// 原版 VisitCallExpression
  fn visit_call_expression(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitCallMethodExpression
  fn visit_call_method_expression(&mut self, ast: &mut Ast, node: NodeId)
  -> Result<Option<NodeId>>;

  /// 原版 VisitIntrinsicCallExpression
  fn visit_intrinsic_call_expression(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitStructExpression
  fn visit_struct_expression(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitLogicalOrExpression
  fn visit_logical_or_expression(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitLogicalAndExpression
  fn visit_logical_and_expression(&mut self, ast: &mut Ast, node: NodeId)
  -> Result<Option<NodeId>>;

  /// 原版 VisitSpreadExpression
  fn visit_spread_expression(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitConditionalExpression
  fn visit_conditional_expression(&mut self, ast: &mut Ast, node: NodeId)
  -> Result<Option<NodeId>>;

  /// 原版 VisitIdentifierExpression
  fn visit_identifier_expression(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitStringLiteralExpression
  fn visit_string_literal_expression(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitIntegerLiteralExpression
  fn visit_integer_literal_expression(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitFloatingPointLiteralExpression
  fn visit_floating_point_literal_expression(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitFieldAccessExpression
  fn visit_field_access_expression(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitElementAccessExpression
  fn visit_element_access_expression(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitDereferenceExpression
  fn visit_dereference_expression(&mut self, ast: &mut Ast, node: NodeId)
  -> Result<Option<NodeId>>;

  /// 原版 VisitAssignmentExpression
  fn visit_assignment_expression(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitIncrementDecrementExpression
  fn visit_increment_decrement_expression(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitNewExpression
  fn visit_new_expression(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitAssumeTypeImpossibleExpression
  fn visit_assume_type_impossible_expression(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitStatementExpression
  fn visit_statement_expression(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitTryLabelExpression
  fn visit_try_label_expression(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitBasicTypeExpression
  fn visit_basic_type_expression(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitFunctionTypeExpression
  fn visit_function_type_expression(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitPrecomputedTypeExpression
  fn visit_precomputed_type_expression(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitUnionTypeExpression
  fn visit_union_type_expression(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitBlockStatement
  fn visit_block_statement(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitExpressionStatement
  fn visit_expression_statement(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitIfStatement
  fn visit_if_statement(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitWhileStatement
  fn visit_while_statement(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitTypeswitchStatement
  fn visit_typeswitch_statement(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitForLoopStatement
  fn visit_for_loop_statement(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitBreakStatement
  fn visit_break_statement(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitContinueStatement
  fn visit_continue_statement(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitReturnStatement
  fn visit_return_statement(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitDebugStatement
  fn visit_debug_statement(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitAssertStatement
  fn visit_assert_statement(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitTailCallStatement
  fn visit_tail_call_statement(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitVarDeclarationStatement
  fn visit_var_declaration_statement(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitGotoStatement
  fn visit_goto_statement(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitAbstractTypeDeclaration
  fn visit_abstract_type_declaration(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitTypeAliasDeclaration
  fn visit_type_alias_declaration(&mut self, ast: &mut Ast, node: NodeId)
  -> Result<Option<NodeId>>;

  /// 原版 VisitBitFieldStructDeclaration
  fn visit_bit_field_struct_declaration(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitClassDeclaration
  fn visit_class_declaration(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitStructDeclaration
  fn visit_struct_declaration(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitGenericCallableDeclaration
  fn visit_generic_callable_declaration(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitGenericTypeDeclaration
  fn visit_generic_type_declaration(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitSpecializationDeclaration
  fn visit_specialization_declaration(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitExternConstDeclaration
  fn visit_extern_const_declaration(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitNamespaceDeclaration
  fn visit_namespace_declaration(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitConstDeclaration
  fn visit_const_declaration(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitCppIncludeDeclaration
  fn visit_cpp_include_declaration(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitTorqueMacroDeclaration
  fn visit_torque_macro_declaration(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitTorqueBuiltinDeclaration
  fn visit_torque_builtin_declaration(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitExternalMacroDeclaration
  fn visit_external_macro_declaration(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitExternalBuiltinDeclaration
  fn visit_external_builtin_declaration(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitExternalRuntimeDeclaration
  fn visit_external_runtime_declaration(
    &mut self,
    ast: &mut Ast,
    node: NodeId,
  ) -> Result<Option<NodeId>>;

  /// 原版 VisitIntrinsicDeclaration
  fn visit_intrinsic_declaration(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitIdentifier
  fn visit_identifier(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitTryHandler
  fn visit_try_handler(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 VisitClassBody
  fn visit_class_body(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;

  /// 原版 Visit(node): kind 分派 (默认实现)
  fn visit_node(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>> {
    let kind = ast.node(node).0.kind;
    match kind {
      AstNodeKind::CallExpression => self.visit_call_expression(ast, node),
      AstNodeKind::CallMethodExpression => self.visit_call_method_expression(ast, node),
      AstNodeKind::IntrinsicCallExpression => self.visit_intrinsic_call_expression(ast, node),
      AstNodeKind::StructExpression => self.visit_struct_expression(ast, node),
      AstNodeKind::LogicalOrExpression => self.visit_logical_or_expression(ast, node),
      AstNodeKind::LogicalAndExpression => self.visit_logical_and_expression(ast, node),
      AstNodeKind::SpreadExpression => self.visit_spread_expression(ast, node),
      AstNodeKind::ConditionalExpression => self.visit_conditional_expression(ast, node),
      AstNodeKind::IdentifierExpression => self.visit_identifier_expression(ast, node),
      AstNodeKind::StringLiteralExpression => self.visit_string_literal_expression(ast, node),
      AstNodeKind::IntegerLiteralExpression => self.visit_integer_literal_expression(ast, node),
      AstNodeKind::FloatingPointLiteralExpression => {
        self.visit_floating_point_literal_expression(ast, node)
      }
      AstNodeKind::FieldAccessExpression => self.visit_field_access_expression(ast, node),
      AstNodeKind::ElementAccessExpression => self.visit_element_access_expression(ast, node),
      AstNodeKind::DereferenceExpression => self.visit_dereference_expression(ast, node),
      AstNodeKind::AssignmentExpression => self.visit_assignment_expression(ast, node),
      AstNodeKind::IncrementDecrementExpression => {
        self.visit_increment_decrement_expression(ast, node)
      }
      AstNodeKind::NewExpression => self.visit_new_expression(ast, node),
      AstNodeKind::AssumeTypeImpossibleExpression => {
        self.visit_assume_type_impossible_expression(ast, node)
      }
      AstNodeKind::StatementExpression => self.visit_statement_expression(ast, node),
      AstNodeKind::TryLabelExpression => self.visit_try_label_expression(ast, node),
      AstNodeKind::BasicTypeExpression => self.visit_basic_type_expression(ast, node),
      AstNodeKind::FunctionTypeExpression => self.visit_function_type_expression(ast, node),
      AstNodeKind::PrecomputedTypeExpression => self.visit_precomputed_type_expression(ast, node),
      AstNodeKind::UnionTypeExpression => self.visit_union_type_expression(ast, node),
      AstNodeKind::BlockStatement => self.visit_block_statement(ast, node),
      AstNodeKind::ExpressionStatement => self.visit_expression_statement(ast, node),
      AstNodeKind::IfStatement => self.visit_if_statement(ast, node),
      AstNodeKind::WhileStatement => self.visit_while_statement(ast, node),
      AstNodeKind::TypeswitchStatement => self.visit_typeswitch_statement(ast, node),
      AstNodeKind::ForLoopStatement => self.visit_for_loop_statement(ast, node),
      AstNodeKind::BreakStatement => self.visit_break_statement(ast, node),
      AstNodeKind::ContinueStatement => self.visit_continue_statement(ast, node),
      AstNodeKind::ReturnStatement => self.visit_return_statement(ast, node),
      AstNodeKind::DebugStatement => self.visit_debug_statement(ast, node),
      AstNodeKind::AssertStatement => self.visit_assert_statement(ast, node),
      AstNodeKind::TailCallStatement => self.visit_tail_call_statement(ast, node),
      AstNodeKind::VarDeclarationStatement => self.visit_var_declaration_statement(ast, node),
      AstNodeKind::GotoStatement => self.visit_goto_statement(ast, node),
      AstNodeKind::AbstractTypeDeclaration => self.visit_abstract_type_declaration(ast, node),
      AstNodeKind::TypeAliasDeclaration => self.visit_type_alias_declaration(ast, node),
      AstNodeKind::BitFieldStructDeclaration => self.visit_bit_field_struct_declaration(ast, node),
      AstNodeKind::ClassDeclaration => self.visit_class_declaration(ast, node),
      AstNodeKind::StructDeclaration => self.visit_struct_declaration(ast, node),
      AstNodeKind::GenericCallableDeclaration => self.visit_generic_callable_declaration(ast, node),
      AstNodeKind::GenericTypeDeclaration => self.visit_generic_type_declaration(ast, node),
      AstNodeKind::SpecializationDeclaration => self.visit_specialization_declaration(ast, node),
      AstNodeKind::ExternConstDeclaration => self.visit_extern_const_declaration(ast, node),
      AstNodeKind::NamespaceDeclaration => self.visit_namespace_declaration(ast, node),
      AstNodeKind::ConstDeclaration => self.visit_const_declaration(ast, node),
      AstNodeKind::CppIncludeDeclaration => self.visit_cpp_include_declaration(ast, node),
      AstNodeKind::TorqueMacroDeclaration => self.visit_torque_macro_declaration(ast, node),
      AstNodeKind::TorqueBuiltinDeclaration => self.visit_torque_builtin_declaration(ast, node),
      AstNodeKind::ExternalMacroDeclaration => self.visit_external_macro_declaration(ast, node),
      AstNodeKind::ExternalBuiltinDeclaration => self.visit_external_builtin_declaration(ast, node),
      AstNodeKind::ExternalRuntimeDeclaration => self.visit_external_runtime_declaration(ast, node),
      AstNodeKind::IntrinsicDeclaration => self.visit_intrinsic_declaration(ast, node),
      AstNodeKind::Identifier => self.visit_identifier(ast, node),
      AstNodeKind::TryHandler => self.visit_try_handler(ast, node),
      AstNodeKind::ClassBody => self.visit_class_body(ast, node),
    }
  }

  /// 原版 Run: 遍历顶层声明, 原位替换
  fn run(&mut self, ast: &mut Ast) -> Result<()> {
    let decls = mem::take(&mut ast.declarations);
    let mut out = Vec::with_capacity(decls.len());
    for decl in decls {
      if let Some(n) = self.visit_node(ast, decl)? {
        out.push(n);
      }
    }
    ast.declarations = out;
    Ok(())
  }
}
