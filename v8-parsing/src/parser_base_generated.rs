// 由 tool/src/parsing/parser-base.h.js 从 v8/src/parsing/parser-base.h 生成; 修订就地改, 复跑覆盖
// 解析器基类枚举与骨架

// 语义面: ParserBase<Impl> 模板方法 (263+ 处 Parse*) 登记跳过,
// 由 parser_generated.rs (parser.cc 侧) 与 Developer 修订补齐。

/// 函数名合法性检查模式 (原 enum FunctionNameValidity)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FunctionNameValidity {
  /// kFunctionNameIsStrictReserved
  FunctionNameIsStrictReserved = 0,
  /// kSkipFunctionNameCheck
  SkipFunctionNameCheck = 1,
  /// kFunctionNameValidityUnknown
  FunctionNameValidityUnknown = 2,
}

/// 带标号函数语句许可 (原 enum AllowLabelledFunctionStatement)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AllowLabelledFunctionStatement {
  /// kAllowLabelledFunctionStatement
  AllowLabelledFunctionStatement = 0,
  /// kDisallowLabelledFunctionStatement
  DisallowLabelledFunctionStatement = 1,
}

/// 箭头函数头判定 (原 enum ParsingArrowHeadFlag)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ParsingArrowHeadFlag {
  /// kCertainlyNotArrowHead
  CertainlyNotArrowHead = 0,
  /// kMaybeArrowHead
  MaybeArrowHead = 1,
}

/// 解析函数标志位集 (原 enum class ParseFunctionFlag : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ParseFunctionFlag {
  IsNormal = 0,
  IsGenerator = 1 << 0,
  IsAsync = 1 << 1,
}

/// 位集别名 (原 base::Flags<ParseFunctionFlag>)
pub type ParseFunctionFlags = u8;

/// 属性解析类别 (原 enum class ParsePropertyKind : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ParsePropertyKind {
  /// kAutoAccessorClassField
  AutoAccessorClassField = 0,
  /// kAccessorGetter
  AccessorGetter = 1,
  /// kAccessorSetter
  AccessorSetter = 2,
  /// kValue
  Value = 3,
  /// kShorthand
  Shorthand = 4,
  /// kAssign
  Assign = 5,
  /// kMethod
  Method = 6,
  /// kClassField
  ClassField = 7,
  /// kShorthandOrClassField
  ShorthandOrClassField = 8,
  /// kSpread
  Spread = 9,
  /// kNotSet
  NotSet = 10,
}

/// 变量声明上下文 (原 enum VariableDeclarationContext)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VariableDeclarationContext {
  /// kStatementListItem
  StatementListItem = 0,
  /// kStatement
  Statement = 1,
  /// kForStatement
  ForStatement = 2,
}

/// 循环目标类型 (原 enum TargetType)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TargetType {
  /// TARGET_FOR_ANONYMOUS
  TARGET_FOR_ANONYMOUS = 0,
  /// TARGET_FOR_NAMED_ONLY
  TARGET_FOR_NAMED_ONLY = 1,
}

/// 原 template <typename Impl> class ParserBase 骨架
pub struct ParserBase;

// —— 提取不变式断言 ——
const _: () = {
  assert!((ParseFunctionFlag::IsGenerator as u8) | (ParseFunctionFlag::IsAsync as u8) == 3);
};
