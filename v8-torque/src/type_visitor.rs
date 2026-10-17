// 由 tool/src/torque/types.js 从 v8/src/torque/type-visitor.h 生成; 修订就地改, 复跑覆盖
// TypeVisitor: 类型计算访问器 (静态方法族 → 关联函数)

use crate::{
  ast::NodeId,
  declarable::DeclarableId,
  error::{Error, Result},
  type_oracle::TypeOracle,
  types::{MaybeSpecializationKey, Signature, TypeId, TypeVector},
};

// 原版 TypeVisitor 全静态 → Rust 关联函数; ComputeType 多重载 (按 decl 类型分派)
// 合一为 compute_type_decl (按节点 kind 分派, 登记偏差); 上下文 Scope → DeclarableId。
pub struct TypeVisitor;

impl TypeVisitor {
  /// 原版 ComputeType(TypeExpression*) — 分派入口
  pub fn compute_type(oracle: &TypeOracle, type_expression: NodeId) -> Result<TypeId> {
    // TODO(填体)
    let _ = (oracle, type_expression);
    Err(Error::Unimplemented)
  }

  /// 原版 ComputeTypeVector
  pub fn compute_type_vector(oracle: &TypeOracle, v: &[NodeId]) -> Result<TypeVector> {
    // TODO(填体)
    let _ = (oracle, v);
    Err(Error::Unimplemented)
  }

  /// 原版 ComputeType(decl, specialized_from, requester) — 五个 decl 重载合一
  pub fn compute_type_decl(
    oracle: &TypeOracle,
    decl: NodeId,
    specialized_from: MaybeSpecializationKey,
    specialization_requester: Option<DeclarableId>,
  ) -> Result<TypeId> {
    // TODO(填体): AbstractType/TypeAlias/BitFieldStruct/Struct/Class 五分支
    let _ = (oracle, decl, specialized_from, specialization_requester);
    Err(Error::Unimplemented)
  }

  /// 原版 VisitClassFieldsAndMethods
  pub fn visit_class_fields_and_methods(
    oracle: &TypeOracle,
    class_type: TypeId,
    class_declaration: NodeId,
  ) -> Result<()> {
    // TODO(填体)
    let _ = (oracle, class_type, class_declaration);
    Err(Error::Unimplemented)
  }

  /// 原版 VisitStructMethods
  pub fn visit_struct_methods(
    oracle: &TypeOracle,
    struct_type: TypeId,
    struct_declaration: NodeId,
  ) -> Result<()> {
    // TODO(填体)
    let _ = (oracle, struct_type, struct_declaration);
    Err(Error::Unimplemented)
  }

  /// 原版 MakeSignature
  pub fn make_signature(declaration: NodeId) -> Result<Signature> {
    // TODO(填体)
    let _ = declaration;
    Err(Error::Unimplemented)
  }

  /// 原版 ComputeTypeForStructExpression (返回 Struct 或 BitFieldStruct)
  pub fn compute_type_for_struct_expression(
    oracle: &TypeOracle,
    type_expression: NodeId,
    term_argument_types: &TypeVector,
  ) -> Result<TypeId> {
    // TODO(填体)
    let _ = (oracle, type_expression, term_argument_types);
    Err(Error::Unimplemented)
  }
}

// ── 原版 src/torque/type-visitor.h ──

// ── 原版 src/torque/type-visitor.cc ──
// L19: const Type* TypeVisitor::ComputeType(TypeDeclaration* decl, MaybeSpecializationKey specialized_from, Scope* specialization_requester)
// L54: const Type* TypeVisitor::ComputeType(TypeAliasDeclaration* decl, MaybeSpecializationKey specialized_from)
// L62: std::string ComputeGeneratesType(std::optional<std::string> opt_gen)
// L68: const AbstractType* TypeVisitor::ComputeType( AbstractTypeDeclaration* decl, MaybeSpecializationKey specialized_from)
// L100: void DeclareMethods(AggregateType* container_type, const std::vector<Declaration*>& methods)
// L125: const BitFieldStructType* TypeVisitor::ComputeType( BitFieldStructDeclaration* decl, MaybeSpecializationKey specialized_from)
// L189: const StructType* TypeVisitor::ComputeType( StructDeclaration* decl, MaybeSpecializationKey specialized_from)
// L234: const ClassType* TypeVisitor::ComputeType( ClassDeclaration* decl, MaybeSpecializationKey specialized_from)
// L321: const Type* TypeVisitor::ComputeType(TypeExpression* type_expression)
// L371: Signature TypeVisitor::MakeSignature(const CallableDeclaration* declaration)
// L392: void TypeVisitor::VisitClassFieldsAndMethods( ClassType* class_type, const ClassDeclaration* class_declaration)
// L481: void TypeVisitor::VisitStructMethods( StructType* struct_type, const StructDeclaration* struct_declaration)
// L486: const Type* TypeVisitor::ComputeTypeForStructExpression( TypeExpression* type_expression, const std::vector<const Type*>& term_argument_types)
