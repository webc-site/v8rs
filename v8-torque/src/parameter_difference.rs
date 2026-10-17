// 由 tool/src/torque/declarable.js 从 v8/src/torque/parameter-difference.h 生成; 修订就地改, 复跑覆盖
// 参数差集 (重载决议比较器)

use crate::types::{TypeId, TypeVector};

// 原版 ParameterDifference 1:1: 逐参数比较调用实参型与签名形参型的差集;
// None = 需要隐式转换, Some = 签名中的超型。
#[derive(Debug, Clone, Default)]
pub struct ParameterDifference {
  pub difference: Vec<Option<TypeId>>,
}

impl ParameterDifference {
  /// 原版构造: 逐位 AddParameter(to[i], from[i])
  #[must_use]
  pub fn new(to: &TypeVector, from: &TypeVector) -> Self {
    // TODO(填体): AddParameter (IsSubtypeOf / IsAssignableFrom 判定)
    let _ = (to, from);
    Self::default()
  }

  /// 原版 StrictlyBetterThan: 严格优于全部备选才当选
  #[must_use]
  pub fn strictly_better_than(&self, other: &Self) -> bool {
    // TODO(填体)
    let _ = other;
    false
  }
}
