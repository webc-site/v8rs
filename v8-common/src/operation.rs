// 由 tool/src/common.js 从 v8/src/common/operation.h 生成; 修订就地改, 复跑覆盖
//
// 修订 (Developer): OPERATION_LIST X-macro 全量展开 (22 变体), 变体命名
// kFoo → Foo (登记); 判定函数与 operator<< 输出名 1:1。

use core::fmt;

/// 原版 enum class Operation : uint8_t (operation.h:48-52)
/// 顺序 = OPERATION_LIST: ARITHMETIC → UNARY → COMPARISON
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Operation {
  Add,
  Subtract,
  Multiply,
  Divide,
  Modulus,
  Exponentiate,
  BitwiseAnd,
  BitwiseOr,
  BitwiseXor,
  ShiftLeft,
  ShiftRight,
  ShiftRightLogical,
  BitwiseNot,
  Negate,
  Increment,
  Decrement,
  Equal,
  StrictEqual,
  LessThan,
  LessThanOrEqual,
  GreaterThan,
  GreaterThanOrEqual,
}

impl Operation {
  /// 原版 operator<< 输出名 (operation.h:94-103, #name 拼接)
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Add => "Add",
      Self::Subtract => "Subtract",
      Self::Multiply => "Multiply",
      Self::Divide => "Divide",
      Self::Modulus => "Modulus",
      Self::Exponentiate => "Exponentiate",
      Self::BitwiseAnd => "BitwiseAnd",
      Self::BitwiseOr => "BitwiseOr",
      Self::BitwiseXor => "BitwiseXor",
      Self::ShiftLeft => "ShiftLeft",
      Self::ShiftRight => "ShiftRight",
      Self::ShiftRightLogical => "ShiftRightLogical",
      Self::BitwiseNot => "BitwiseNot",
      Self::Negate => "Negate",
      Self::Increment => "Increment",
      Self::Decrement => "Decrement",
      Self::Equal => "Equal",
      Self::StrictEqual => "StrictEqual",
      Self::LessThan => "LessThan",
      Self::LessThanOrEqual => "LessThanOrEqual",
      Self::GreaterThan => "GreaterThan",
      Self::GreaterThanOrEqual => "GreaterThanOrEqual",
    }
  }

  /// 原版 IsBitwiseBinaryOperation (operation.h:54-64) —
  /// ARITHMETIC_BITWISE_OPERATION_LIST 成员
  pub const fn is_bitwise_binary(self) -> bool {
    matches!(
      self,
      Self::BitwiseAnd
        | Self::BitwiseOr
        | Self::BitwiseXor
        | Self::ShiftLeft
        | Self::ShiftRight
        | Self::ShiftRightLogical
    )
  }

  /// 原版 IsUnaryOperation (operation.h:66-76) — UNARY_OPERATION_LIST 成员
  pub const fn is_unary(self) -> bool {
    matches!(
      self,
      Self::BitwiseNot | Self::Negate | Self::Increment | Self::Decrement
    )
  }

  /// 原版 IsBinaryOperation (operation.h:78-80)
  pub const fn is_binary(self) -> bool {
    !self.is_unary()
  }

  /// 原版 IsComparisonOperation (operation.h:82-92) —
  /// COMPARISON_OPERATION_LIST 成员
  pub const fn is_comparison(self) -> bool {
    matches!(
      self,
      Self::Equal
        | Self::StrictEqual
        | Self::LessThan
        | Self::LessThanOrEqual
        | Self::GreaterThan
        | Self::GreaterThanOrEqual
    )
  }
}

impl fmt::Display for Operation {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}
