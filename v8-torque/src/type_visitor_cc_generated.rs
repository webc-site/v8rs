// 由 tool/src/torque/type-visitor.cc.js 从 v8/src/torque/type-visitor.{h,cc} 生成; 修订就地改, 复跑覆盖
// TypeVisitor 访问器对数清单

/// TypeVisitor 访问器面 (原版 type-visitor.h; visit 族 5 项: ComputeTypeVector, ComputeType, VisitClassFieldsAndMethods, VisitStructMethods, ComputeTypeForStructExpression)
pub mod type_visitor_surface {
  pub const VISIT_METHODS: [&str; 5] = [
    "ComputeTypeVector",
    "ComputeType",
    "VisitClassFieldsAndMethods",
    "VisitStructMethods",
    "ComputeTypeForStructExpression",
  ];
  /// cc 侧 visitor 实现类 (1 项)
  pub const IMPL_CLASSES: [&str; 1] = ["TypeVisitor"];
}

/// type-visitor.cc 方法定义头清单 (对数基准)。
/// 类 → 方法定义数: TypeVisitor=11
/// const Type* TypeVisitor::ComputeType(TypeDeclaration* decl, MaybeSpecializationKey specialized_from, Scope* specialization_requester)
/// const Type* TypeVisitor::ComputeType(TypeAliasDeclaration* decl, MaybeSpecializationKey specialized_from)
/// const AbstractType* TypeVisitor::ComputeType( AbstractTypeDeclaration* decl, MaybeSpecializationKey specialized_from)
/// const BitFieldStructType* TypeVisitor::ComputeType( BitFieldStructDeclaration* decl, MaybeSpecializationKey specialized_from)
/// const StructType* TypeVisitor::ComputeType( StructDeclaration* decl, MaybeSpecializationKey specialized_from)
/// const ClassType* TypeVisitor::ComputeType( ClassDeclaration* decl, MaybeSpecializationKey specialized_from)
/// const Type* TypeVisitor::ComputeType(TypeExpression* type_expression)
/// Signature TypeVisitor::MakeSignature(const CallableDeclaration* declaration)
/// void TypeVisitor::VisitClassFieldsAndMethods( ClassType* class_type, const ClassDeclaration* class_declaration)
/// void TypeVisitor::VisitStructMethods( StructType* struct_type, const StructDeclaration* struct_declaration)
/// const Type* TypeVisitor::ComputeTypeForStructExpression( TypeExpression* type_expression, const std::vector<const Type*>& term_argument_types)
pub mod type_visitor_cc_skel {
  pub const DEF_COUNT: usize = 11;
}
