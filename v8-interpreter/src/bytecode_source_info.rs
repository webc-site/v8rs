// 由 tool/src/interpreter/bytecode-source-info.h.js 从 v8/src/interpreter/bytecode-source-info.{h,cc} 生成; 修订就地改, 复跑覆盖
// 字节码源码位置信息骨架

/// 位置类别 (原版私有 enum class PositionType : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum PositionType {
  #[default]
  None = 0,
  Expression = 1,
  Statement = 2,
}

/// 未初始化位置哨兵 (原版 kUninitializedPosition)
pub const UNINITIALIZED_POSITION: i32 = -1;

/// 原版 class BytecodeSourceInfo 成员声明 (机械 1:1 提取, 17 条)
pub const BYTECODE_SOURCE_INFO_DECLARATIONS: &[&str] = &[
  "[public/const] static const int kUninitializedPosition = -1",
  "[public/method] BytecodeSourceInfo() : position_type_(PositionType::kNone), source_position_(kUninitializedPosition), is_breakable_(true)",
  "[public/method] BytecodeSourceInfo(int source_position, bool is_statement, bool is_breakable = true) : position_type_(is_statement ? PositionType::kStatement : PositionType::kExpression), source_position_(source_position), is_breakable_(is_breakable)",
  "[public/method] void MakeStatementPosition(int source_position, bool is_breakable = true)",
  "[public/method] void MakeExpressionPosition(int source_position)",
  "[public/method] void ForceExpressionPosition(int source_position)",
  "[public/method] int source_position() const",
  "[public/method] bool is_statement() const",
  "[public/method] bool is_expression() const",
  "[public/method] bool is_valid() const",
  "[public/method] void set_invalid()",
  "[public/method] bool is_breakable() const",
  "[public/method] bool operator==(const BytecodeSourceInfo& other) const",
  "[public/method] bool operator!=(const BytecodeSourceInfo& other) const",
  "[private/field] PositionType position_type_",
  "[private/field] int source_position_",
  "[private/field] bool is_breakable_",
];

/// 源码位置信息 (原版 BytecodeSourceInfo)
#[derive(Debug, Clone, Copy, Default)]
pub struct BytecodeSourceInfo {
  position_type: PositionType,
  source_position: i32,
  is_breakable: bool,
}

impl PartialEq for BytecodeSourceInfo {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.position_type == other.position_type && self.source_position == other.source_position
  }
}
impl Eq for BytecodeSourceInfo {}

impl BytecodeSourceInfo {
  #[inline]
  pub const fn none() -> Self {
    Self {
      position_type: PositionType::None,
      source_position: UNINITIALIZED_POSITION,
      is_breakable: true,
    }
  }

  #[inline]
  pub const fn statement(source_position: i32, is_breakable: bool) -> Self {
    Self {
      position_type: PositionType::Statement,
      source_position,
      is_breakable,
    }
  }

  #[inline]
  pub const fn expression(source_position: i32) -> Self {
    Self {
      position_type: PositionType::Expression,
      source_position,
      is_breakable: true,
    }
  }

  #[inline]
  pub const fn source_position(self) -> i32 {
    self.source_position
  }

  #[inline]
  pub const fn is_statement(self) -> bool {
    matches!(self.position_type, PositionType::Statement)
  }

  #[inline]
  pub const fn is_expression(self) -> bool {
    matches!(self.position_type, PositionType::Expression)
  }

  #[inline]
  pub const fn is_valid(self) -> bool {
    self.source_position != UNINITIALIZED_POSITION
  }

  #[inline]
  pub fn set_invalid(&mut self) {
    self.position_type = PositionType::None;
    self.source_position = UNINITIALIZED_POSITION;
  }

  #[inline]
  pub fn make_statement_position(&mut self, source_position: i32, is_breakable: bool) {
    self.position_type = PositionType::Statement;
    self.source_position = source_position;
    self.is_breakable = is_breakable;
  }

  #[inline]
  pub fn make_expression_position(&mut self, source_position: i32) {
    self.position_type = PositionType::Expression;
    self.source_position = source_position;
    self.is_breakable = true;
  }

  #[inline]
  pub fn force_expression_position(&mut self, source_position: i32) {
    self.position_type = PositionType::Expression;
    self.source_position = source_position;
  }

  #[inline]
  pub const fn is_breakable(self) -> bool {
    self.is_breakable
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(BYTECODE_SOURCE_INFO_DECLARATIONS.len() == 17);
  assert!(UNINITIALIZED_POSITION == -1);
};
