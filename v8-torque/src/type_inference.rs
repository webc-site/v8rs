// 由 tool/src/torque/declarable.js 从 v8/src/torque/type-inference.h 生成; 修订就地改, 复跑覆盖
// 类型实参推断

use std::collections::BTreeMap;

use crate::{
  ast::{GenericParameter, NodeId},
  error::{Error, Result},
  types::{TypeId, TypeVector},
};

// 原版 TypeArgumentInference 1:1 — 泛型实参推断 (见原版头注释):
// 形参类型表达式 (NodeId) 与实参型 (TypeId) 匹配; 显式给出的实参忽略不一致约束;
// 函数/联合形参不产生约束。推断在相应 scope 内实例化 (上下文 → 构造时传参)。
#[derive(Debug, Clone, Default)]
pub struct TypeArgumentInference {
  pub num_explicit: usize,
  pub type_parameter_from_name: BTreeMap<String, usize>,
  pub inferred: Vec<Option<TypeId>>,
  pub failure_reason: Option<String>,
}

impl TypeArgumentInference {
  /// 原版构造 (type_parameters: 泛型形参表, term_parameters: 项形参类型表达式)
  #[must_use]
  pub fn new(
    type_parameters: &[GenericParameter],
    explicit_type_arguments: &TypeVector,
    term_parameters: &[NodeId],
    term_argument_types: &[Option<TypeId>],
  ) -> Self {
    // TODO(填体): Match 循环
    let _ = (
      type_parameters,
      explicit_type_arguments,
      term_parameters,
      term_argument_types,
    );
    Self::default()
  }

  #[must_use]
  pub fn has_failed(&self) -> bool {
    self.failure_reason.is_some()
  }

  /// 原版 GetFailureReason (前置条件: HasFailed())
  #[must_use]
  pub fn failure_reason(&self) -> &str {
    self.failure_reason.as_deref().unwrap_or_default()
  }

  /// 原版 GetResult
  pub fn get_result(&self) -> Result<TypeVector> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }

  /// 原版 Fail
  pub fn fail(&mut self, reason: String) {
    self.failure_reason = Some(reason);
  }

  /// 原版 Match (私有)
  pub fn match_parameter(&mut self, parameter: NodeId, argument_type: Option<TypeId>) {
    // TODO(填体)
    let _ = (parameter, argument_type);
  }

  /// 原版 MatchGeneric (私有)
  pub fn match_generic(&mut self, parameter: NodeId, argument_type: Option<TypeId>) {
    // TODO(填体)
    let _ = (parameter, argument_type);
  }
}

// ── 原版 src/torque/type-inference.cc ──
// L11: TypeArgumentInference::TypeArgumentInference( const GenericParameters& type_parameters, const TypeVector& explicit_type_arguments, const std::vector<TypeExpression*>& term_parameters, const std::vector<std::optional<const Type*>>& term_argument_types) : num_explicit_(explicit_type_arguments.size()), type_parameter_from_name_(type_parameters.size()), inferred_(type_parameters.size())
// L50: TypeVector TypeArgumentInference::GetResult() const
// L59: void TypeArgumentInference::Match(TypeExpression* parameter, const Type* argument_type)
// L93: void TypeArgumentInference::MatchGeneric(BasicTypeExpression* parameter, const Type* argument_type)

// 填体提示: 推断结果经 GenericCallable::GetSpecialization (specializations 表) 消费。
