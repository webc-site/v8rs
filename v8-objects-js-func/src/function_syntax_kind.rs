// 由 tool/src/objects/function-syntax-kind.h.js 从 v8/src/objects/function-syntax-kind.h 生成; 修订就地改, 复跑覆盖
// FunctionSyntaxKind 语法种类

/// 函数语法种类 (原版 enum class FunctionSyntaxKind : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum FunctionSyntaxKind {
  /// kAnonymousExpression
  AnonymousExpression = 0,
  /// kNamedExpression
  NamedExpression = 1,
  /// kDeclaration
  Declaration = 2,
  /// kAccessorOrMethod
  AccessorOrMethod = 3,
  /// kWrapped
  Wrapped = 4,
}

impl FunctionSyntaxKind {
  /// 原版 kLastFunctionSyntaxKind
  pub const LAST: Self = Self::Wrapped;

  /// 原版 FunctionSyntaxKind2String
  #[inline]
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
      Self::AnonymousExpression => "AnonymousExpression",
      Self::NamedExpression => "NamedExpression",
      Self::Declaration => "Declaration",
      Self::AccessorOrMethod => "AccessorOrMethod",
      Self::Wrapped => "Wrapped",
    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(FunctionSyntaxKind::LAST as u8 == 4);
  assert!(FunctionSyntaxKind::Declaration.to_str() == "Declaration");
};
