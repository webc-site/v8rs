// 由 tool/src/torque/generators.js 从 v8/src/torque/tsa-generator.h 生成; 修订就地改, 复跑覆盖
// TSA 生成器 (TSAGenerator 类在 .cc; 原版唯一 AstVisitor 继承者)

use crate::{
  ast::{Ast, NodeId},
  ast_visitor::AstVisitor,
  error::{Error, Result},
};

// 原版 tsa-generator.h 仅声明 GenerateTSA; TSAGenerator 类 (继承 AstVisitor) 定义于
// tsa-generator.cc, 此处上提为公开结构体并给出 trait impl 骨架 (60 节点逐一 Err 占位)。
/// 原版 GenerateTSA
pub fn generate_tsa(ast: &mut Ast, output_directory: &str) -> Result<()> {
  // TODO(填体): TSAGenerator 构造 + Run
  let _ = (ast, output_directory);
  Err(Error::Unimplemented)
}

/// 原版 TSAGenerator (tsa-generator.cc 内定义, 继承 AstVisitor<TSAGenerator>)
#[derive(Debug, Default)]
pub struct TsaGenerator {
  pub out: String,
  pub out_decls: String,
}

impl AstVisitor for TsaGenerator {
  fn visit_call_expression(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitCallExpression
    Err(Error::Unimplemented)
  }

  fn visit_call_method_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitCallMethodExpression
    Err(Error::Unimplemented)
  }

  fn visit_intrinsic_call_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitIntrinsicCallExpression
    Err(Error::Unimplemented)
  }

  fn visit_struct_expression(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitStructExpression
    Err(Error::Unimplemented)
  }

  fn visit_logical_or_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitLogicalOrExpression
    Err(Error::Unimplemented)
  }

  fn visit_logical_and_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitLogicalAndExpression
    Err(Error::Unimplemented)
  }

  fn visit_spread_expression(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitSpreadExpression
    Err(Error::Unimplemented)
  }

  fn visit_conditional_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitConditionalExpression
    Err(Error::Unimplemented)
  }

  fn visit_identifier_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitIdentifierExpression
    Err(Error::Unimplemented)
  }

  fn visit_string_literal_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitStringLiteralExpression
    Err(Error::Unimplemented)
  }

  fn visit_integer_literal_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitIntegerLiteralExpression
    Err(Error::Unimplemented)
  }

  fn visit_floating_point_literal_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitFloatingPointLiteralExpression
    Err(Error::Unimplemented)
  }

  fn visit_field_access_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitFieldAccessExpression
    Err(Error::Unimplemented)
  }

  fn visit_element_access_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitElementAccessExpression
    Err(Error::Unimplemented)
  }

  fn visit_dereference_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitDereferenceExpression
    Err(Error::Unimplemented)
  }

  fn visit_assignment_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitAssignmentExpression
    Err(Error::Unimplemented)
  }

  fn visit_increment_decrement_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitIncrementDecrementExpression
    Err(Error::Unimplemented)
  }

  fn visit_new_expression(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitNewExpression
    Err(Error::Unimplemented)
  }

  fn visit_assume_type_impossible_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitAssumeTypeImpossibleExpression
    Err(Error::Unimplemented)
  }

  fn visit_statement_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitStatementExpression
    Err(Error::Unimplemented)
  }

  fn visit_try_label_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitTryLabelExpression
    Err(Error::Unimplemented)
  }

  fn visit_basic_type_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitBasicTypeExpression
    Err(Error::Unimplemented)
  }

  fn visit_function_type_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitFunctionTypeExpression
    Err(Error::Unimplemented)
  }

  fn visit_precomputed_type_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitPrecomputedTypeExpression
    Err(Error::Unimplemented)
  }

  fn visit_union_type_expression(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitUnionTypeExpression
    Err(Error::Unimplemented)
  }

  fn visit_block_statement(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitBlockStatement
    Err(Error::Unimplemented)
  }

  fn visit_expression_statement(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitExpressionStatement
    Err(Error::Unimplemented)
  }

  fn visit_if_statement(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitIfStatement
    Err(Error::Unimplemented)
  }

  fn visit_while_statement(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitWhileStatement
    Err(Error::Unimplemented)
  }

  fn visit_typeswitch_statement(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitTypeswitchStatement
    Err(Error::Unimplemented)
  }

  fn visit_for_loop_statement(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitForLoopStatement
    Err(Error::Unimplemented)
  }

  fn visit_break_statement(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitBreakStatement
    Err(Error::Unimplemented)
  }

  fn visit_continue_statement(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitContinueStatement
    Err(Error::Unimplemented)
  }

  fn visit_return_statement(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitReturnStatement
    Err(Error::Unimplemented)
  }

  fn visit_debug_statement(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitDebugStatement
    Err(Error::Unimplemented)
  }

  fn visit_assert_statement(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitAssertStatement
    Err(Error::Unimplemented)
  }

  fn visit_tail_call_statement(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitTailCallStatement
    Err(Error::Unimplemented)
  }

  fn visit_var_declaration_statement(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitVarDeclarationStatement
    Err(Error::Unimplemented)
  }

  fn visit_goto_statement(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitGotoStatement
    Err(Error::Unimplemented)
  }

  fn visit_abstract_type_declaration(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitAbstractTypeDeclaration
    Err(Error::Unimplemented)
  }

  fn visit_type_alias_declaration(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitTypeAliasDeclaration
    Err(Error::Unimplemented)
  }

  fn visit_bit_field_struct_declaration(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitBitFieldStructDeclaration
    Err(Error::Unimplemented)
  }

  fn visit_class_declaration(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitClassDeclaration
    Err(Error::Unimplemented)
  }

  fn visit_struct_declaration(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitStructDeclaration
    Err(Error::Unimplemented)
  }

  fn visit_generic_callable_declaration(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitGenericCallableDeclaration
    Err(Error::Unimplemented)
  }

  fn visit_generic_type_declaration(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitGenericTypeDeclaration
    Err(Error::Unimplemented)
  }

  fn visit_specialization_declaration(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitSpecializationDeclaration
    Err(Error::Unimplemented)
  }

  fn visit_extern_const_declaration(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitExternConstDeclaration
    Err(Error::Unimplemented)
  }

  fn visit_namespace_declaration(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitNamespaceDeclaration
    Err(Error::Unimplemented)
  }

  fn visit_const_declaration(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitConstDeclaration
    Err(Error::Unimplemented)
  }

  fn visit_cpp_include_declaration(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitCppIncludeDeclaration
    Err(Error::Unimplemented)
  }

  fn visit_torque_macro_declaration(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitTorqueMacroDeclaration
    Err(Error::Unimplemented)
  }

  fn visit_torque_builtin_declaration(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitTorqueBuiltinDeclaration
    Err(Error::Unimplemented)
  }

  fn visit_external_macro_declaration(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitExternalMacroDeclaration
    Err(Error::Unimplemented)
  }

  fn visit_external_builtin_declaration(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitExternalBuiltinDeclaration
    Err(Error::Unimplemented)
  }

  fn visit_external_runtime_declaration(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitExternalRuntimeDeclaration
    Err(Error::Unimplemented)
  }

  fn visit_intrinsic_declaration(
    &mut self,
    _ast: &mut Ast,
    _node: NodeId,
  ) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitIntrinsicDeclaration
    Err(Error::Unimplemented)
  }

  fn visit_identifier(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitIdentifier
    Err(Error::Unimplemented)
  }

  fn visit_try_handler(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitTryHandler
    Err(Error::Unimplemented)
  }

  fn visit_class_body(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {
    // TODO(填体): 原版 TSAGenerator::VisitClassBody
    Err(Error::Unimplemented)
  }
}

// 原版 TSAGenerator 自有方法 (GenerateClass/GenerateStruct 等) 见 .cc 清单;
// 类内枚举/字段随填体上提。
// ── 原版 src/torque/tsa-generator.cc ──
// L67: explicit PerSourceData(SourceId source) : source(source)
// L70: explicit IndentationScope(TSAGenerator* generator) : generator_(generator)
// L73: ~IndentationScope()
// L79: explicit OutputBufferScope(TSAGenerator* generator) : generator_(generator)
// L84: ~OutputBufferScope()
// L88: std::stringstream& buffer()
// L89: std::string ToString() const
// L98: public: BlockBindings(TSAGenerator* generator, BindingsManager<T>* manager) : torque::BlockBindings<T>(manager), generator_(generator)
// L104: ~BlockBindings()
// L116: bool SourceShouldGenerateReducer(SourceId source, std::string& reducer_name)
// L134: public: void Run(Ast& ast, const std::string& output_directory)
// L310: bool IsTailReturning(TorqueMacro* macro)
// L326: void GenerateMacro(TorqueMacro* macro, SourceId source)
// L427: void GenerateBuiltin(Builtin* builtin, SourceId source)
// L501: void FailCallableLookup( const std::string& reason, const QualifiedName& name, const TypeVector& parameter_types, const std::vector<Binding<TSALabel>*>& labels, const std::vector<Signature>& candidates, const std::vector<std::pair<GenericCallable*, std::string>> inapplicable_generics)
// L535: TypeArgumentInference InferSpecializationTypes( GenericCallable* generic, const TypeVector& explicit_specialization_types, const TypeVector& explicit_arguments)
// L560: Callable* GetOrCreateSpecialization( const SpecializationKey<GenericCallable>& key)
// L572: Callable* TryLookupCallable(const QualifiedName& name, const TypeVector& parameter_types)
// L680: AstNode* VisitCallExpression(CallExpression* node)
// L682: AstNode* VisitCallMethodExpression(CallMethodExpression* node)
// L686: AstNode* VisitIntrinsicCallExpression(IntrinsicCallExpression* node)
// L690: AstNode* VisitStructExpression(StructExpression* node)
// L694: AstNode* VisitLogicalOrExpression(LogicalOrExpression* node)
// L698: AstNode* VisitLogicalAndExpression(LogicalAndExpression* node)
// L702: AstNode* VisitSpreadExpression(SpreadExpression* node)
// L706: AstNode* VisitConditionalExpression(ConditionalExpression* node)
// L711: AstNode* VisitIdentifierExpression(IdentifierExpression* node)
// L721: AstNode* VisitStringLiteralExpression(StringLiteralExpression* node)
// L725: AstNode* VisitIntegerLiteralExpression(IntegerLiteralExpression* node)
// L729: AstNode* VisitFloatingPointLiteralExpression( FloatingPointLiteralExpression* node)
// L734: AstNode* VisitFieldAccessExpression(FieldAccessExpression* node)
// L737: AstNode* VisitElementAccessExpression(ElementAccessExpression* node)
// L741: AstNode* VisitDereferenceExpression(DereferenceExpression* node)
// L745: AstNode* VisitAssignmentExpression(AssignmentExpression* node)
// L748: AstNode* VisitIncrementDecrementExpression( IncrementDecrementExpression* node)
// L753: AstNode* VisitNewExpression(NewExpression* node)
// L757: AstNode* VisitAssumeTypeImpossibleExpression( AssumeTypeImpossibleExpression* node)
// L762: AstNode* VisitStatementExpression(StatementExpression* node)
// L766: AstNode* VisitTryLabelExpression(TryLabelExpression* node)
// L771: AstNode* VisitBasicTypeExpression(BasicTypeExpression* node)
// L781: AstNode* VisitFunctionTypeExpression(FunctionTypeExpression* node)
// L785: AstNode* VisitPrecomputedTypeExpression(PrecomputedTypeExpression* node)
// L789: AstNode* VisitUnionTypeExpression(UnionTypeExpression* node)
// L794: AstNode* VisitBlockStatement(BlockStatement* node, bool emit_braces = true)
// L810: AstNode* VisitExpressionStatement(ExpressionStatement* node)
// L826: AstNode* VisitIfStatement(IfStatement* node)
// L857: AstNode* VisitWhileStatement(WhileStatement* node)
// L865: AstNode* VisitTypeswitchStatement(TypeswitchStatement* node)
// L895: AstNode* VisitForLoopStatement(ForLoopStatement* node)
// L899: AstNode* VisitBreakStatement(BreakStatement* node)
// L903: AstNode* VisitContinueStatement(ContinueStatement* node)
// L908: AstNode* VisitReturnStatement(ReturnStatement* node)
// L941: AstNode* VisitDebugStatement(DebugStatement* node)
// L951: AstNode* VisitAssertStatement(AssertStatement* node)
// L968: AstNode* VisitTailCallStatement(TailCallStatement* node)
// L973: std::string GetTypeName(const Type* type)
// L982: AstNode* VisitVarDeclarationStatement(VarDeclarationStatement* node)
// L1020: AstNode* VisitGotoStatement(GotoStatement* node)
// L1033: AstNode* VisitAbstractTypeDeclaration(AbstractTypeDeclaration* node)
// L1037: AstNode* VisitTypeAliasDeclaration(TypeAliasDeclaration* node)
// L1041: AstNode* VisitBitFieldStructDeclaration(BitFieldStructDeclaration* node)
// L1045: AstNode* VisitClassDeclaration(ClassDeclaration* node)
// L1049: AstNode* VisitStructDeclaration(StructDeclaration* node)
// L1053: AstNode* VisitGenericCallableDeclaration(GenericCallableDeclaration* node)
// L1057: AstNode* VisitGenericTypeDeclaration(GenericTypeDeclaration* node)
// L1061: AstNode* VisitSpecializationDeclaration(SpecializationDeclaration* node)
// L1065: AstNode* VisitExternConstDeclaration(ExternConstDeclaration* node)
// L1070: AstNode* VisitNamespaceDeclaration(NamespaceDeclaration* node)
// L1075: AstNode* VisitConstDeclaration(ConstDeclaration* node)
// L1079: AstNode* VisitCppIncludeDeclaration(CppIncludeDeclaration* node)
// L1084: AstNode* VisitTorqueMacroDeclaration(TorqueMacroDeclaration* node)
// L1088: AstNode* VisitTorqueBuiltinDeclaration(TorqueBuiltinDeclaration* node)
// L1092: AstNode* VisitExternalMacroDeclaration(ExternalMacroDeclaration* node)
// L1096: AstNode* VisitExternalBuiltinDeclaration(ExternalBuiltinDeclaration* node)
// L1100: AstNode* VisitExternalRuntimeDeclaration(ExternalRuntimeDeclaration* node)
// L1104: AstNode* VisitIntrinsicDeclaration(IntrinsicDeclaration* node)
// L1108: AstNode* VisitIdentifier(Identifier* node)
// L1112: AstNode* VisitTryHandler(TryHandler* node)
// L1116: AstNode* VisitClassBody(ClassBody* node)
// L1124: explicit ExpressionResult(const Type* type) : type(type)
// L1127: ExpressionResult VisitExpression(Expression* expr)
// L1139: ExpressionResult GenerateCall(const Callable* callable, const TypeVector& argument_types, std::vector<std::string> argument_strings, const std::vector<std::string>& label_names, const TypeVector& specialization_types)
// L1203: std::vector<Binding<TSALabel>*> LabelsFromIdentifiers( const std::vector<Identifier*>& names)
// L1214: ExpressionResult VisitExpression_CallExpression(CallExpression* expr)
// L1259: ExpressionResult VisitExpression_CallMethodExpression( CallMethodExpression* expr)
// L1263: ExpressionResult VisitExpression_IntrinsicCallExpression( IntrinsicCallExpression* expr)
// L1267: ExpressionResult VisitExpression_StructExpression(StructExpression* expr)
// L1270: ExpressionResult VisitExpression_LogicalOrExpression( LogicalOrExpression* expr)
// L1274: ExpressionResult VisitExpression_LogicalAndExpression( LogicalAndExpression* expr)
// L1278: ExpressionResult VisitExpression_SpreadExpression(SpreadExpression* expr)
// L1281: ExpressionResult VisitExpression_ConditionalExpression( ConditionalExpression* expr)
// L1286: std::optional<Binding<TSAValue>*> TryLookupLocalValue( const std::string& name)
// L1291: std::optional<Binding<TSALabel>*> TryLookupLabel(const std::string& name)
// L1295: Binding<TSALabel>* LookupLabel(const std::string& name)
// L1301: ExpressionResult VisitExpression_IdentifierExpression( IdentifierExpression* expr)
// L1382: ExpressionResult VisitExpression_StringLiteralExpression( StringLiteralExpression* expr)
// L1391: ExpressionResult VisitExpression_IntegerLiteralExpression( IntegerLiteralExpression* expr)
// L1395: ExpressionResult VisitExpression_FloatingPointLiteralExpression( FloatingPointLiteralExpression* expr)
// L1400: ExpressionResult VisitExpression_FieldAccessExpression( FieldAccessExpression* expr)
// L1431: ExpressionResult VisitExpression_ElementAccessExpression( ElementAccessExpression* expr)
// L1435: ExpressionResult VisitExpression_DereferenceExpression( DereferenceExpression* expr)
// L1440: ExpressionResult VisitExpression_AssignmentExpression( AssignmentExpression* expr)
// L1456: ExpressionResult VisitExpression_IncrementDecrementExpression( IncrementDecrementExpression* expr)
// L1460: ExpressionResult VisitExpression_NewExpression(NewExpression* expr)
// L1463: ExpressionResult VisitExpression_AssumeTypeImpossibleExpression( AssumeTypeImpossibleExpression* expr)
// L1467: ExpressionResult VisitExpression_StatementExpression( StatementExpression* expr)
// L1472: ExpressionResult VisitExpression_TryLabelExpression( TryLabelExpression* expr)
// L1563: static LocationResult VariableAccess(Binding<TSAValue>* variable)
// L1570: bool IsVariableAccess() const
// L1573: LocationResult VisitLocation(Expression* expr)
// L1583: LocationResult VisitLocation_IdentifierExpression( IdentifierExpression* expr)
// L1663: void BeginOutputToHFile(SourceId source, int indentation_levels)
// L1671: void BeginOutputToCCFile(SourceId source, int indentation_levels)
// L1679: void EndOutputToFile()
// L1686: std::stringstream* BeginOutputToBuffer(std::stringstream* stream)
// L1691: void EndOutputToBuffer(std::stringstream* stream, std::stringstream* previous)
// L1697: void WriteFile(const std::string& file_path, const std::string& content)
// L1701: PerSourceData& GetPerSourceData(SourceId source)
// L1707: std::stringstream& out()
// L1713: std::stringstream& out_i()
// L1718: std::string indentation(int diff = 0)
// L1738: void IncreaseIndentation(int levels = 1)
// L1740: void DecreaseIndentation(int levels = 1)
// L1745: void EnterBlockBindingsScope(BlockBindings<TSAValue>* bindings)
// L1749: void EnterBlockBindingsScope(BlockBindings<TSALabel>* bindings)
// L1753: void LeaveBlockBindingsScope(BlockBindings<TSAValue>* bindings)
// L1759: void LeaveBlockBindingsScope(BlockBindings<TSALabel>* bindings)
// L1765: BlockBindings<TSAValue>* current_block_bindings_values()
// L1778: const char* __(SpecialAssemblerFunctionPrefix prefix = SpecialAssemblerFunctionPrefix::kNoPrefix)
// L1803: void GenerateTSA(Ast& ast, const std::string& output_directory)
