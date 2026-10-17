// 由 tool/src/interpreter/bytecode-register.h.js 从 v8/src/interpreter/bytecode-register.{h,cc} 生成; 修订就地改, 复跑覆盖

use crate::{
  bytecode_operands::OperandSize,
  bytecodes::{Bytecode, SHORT_STAR_COUNT},
  error,
  error::Error,
};

/// 非法寄存器索引 (原版 kInvalidIndex = kMaxInt)
pub const INVALID_INDEX: i32 = i32::MAX;

/// 帧偏移 -> 寄存器索引 (原版 OffsetFromFPToRegisterIndex; kSystemPointerSize = 8)
#[inline]
#[must_use]
pub const fn offset_from_fp_to_register_index(offset: i32, register_file_from_fp: i32) -> i32 {
  (register_file_from_fp - offset) / 8
}

/// 寄存器索引 -> 字节码操作数编码 (原版 Register::ToOperand; register_file_start_offset =
/// OffsetFromFPToRegisterIndex(0))
#[inline]
#[must_use]
pub const fn to_operand(register_file_start_offset: i32, index: i32) -> i32 {
  register_file_start_offset - index
}

/// 操作数编码 -> 寄存器索引 (原版 Register::FromOperand)
#[inline]
#[must_use]
pub const fn from_operand(register_file_start_offset: i32, operand: i32) -> i32 {
  register_file_start_offset - operand
}

/// ShortStar 字节码 -> 寄存器索引 (原版 Register::FromShortStar; star0_repr = Bytecode::Star0)
#[inline]
#[must_use]
pub const fn from_short_star(star0_repr: u8, bytecode: Bytecode) -> i32 {
  star0_repr as i32 - bytecode as i32
}

/// 寄存器索引 -> ShortStar 字节码 (原版 Register::TryToShortStar; star0_repr = Bytecode::Star0)
#[inline]
#[must_use]
pub const fn try_to_short_star(star0_repr: u8, index: i32) -> Option<Bytecode> {
  if index >= 0 && index < SHORT_STAR_COUNT as i32 {
    Bytecode::from_repr((star0_repr as i32 - index) as u8)
  } else {
    None
  }
}

/// 操作数编码尺寸 (原版 Register::SizeOfOperand; i8/i16 阈值 -128/127/-32768/32767)
#[inline]
#[must_use]
pub const fn size_of_operand(operand: i32) -> OperandSize {
  use crate::bytecode_operands::OperandSize;
  if operand >= -128 && operand <= 127 {
    OperandSize::Byte
  } else if operand >= -32768 && operand <= 32767 {
    OperandSize::Short
  } else {
    OperandSize::Quad
  }
}

/// 解释器寄存器类型定义
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Register {
  index: i32,
}

impl Register {
  #[inline]
  pub const fn new(index: i32) -> Self {
    Self { index }
  }

  #[inline]
  pub const fn invalid_value() -> Self {
    Self {
      index: INVALID_INDEX,
    }
  }

  #[inline]
  pub const fn index(self) -> i32 {
    self.index
  }

  #[inline]
  pub const fn is_valid(self) -> bool {
    self.index != INVALID_INDEX
  }

  #[inline]
  pub const fn is_parameter(self) -> bool {
    self.index < 0
  }

  #[inline]
  pub const fn from_parameter_index(index: i32) -> Self {
    Self { index: -1 - index }
  }

  #[inline]
  pub const fn to_parameter_index(self) -> i32 {
    -1 - self.index
  }

  #[inline]
  pub const fn receiver() -> Self {
    Self::from_parameter_index(0)
  }

  #[inline]
  pub const fn is_receiver(self) -> bool {
    self.to_parameter_index() == 0
  }

  #[inline]
  pub const fn to_operand(self) -> i32 {
    -4 - self.index
  }

  #[inline]
  pub const fn from_operand(operand: i32) -> Self {
    Self {
      index: -4 - operand,
    }
  }

  #[inline]
  pub const fn from_short_star(bytecode: Bytecode) -> Self {
    Self {
      index: (Bytecode::Star0 as i32) - (bytecode as i32),
    }
  }

  #[inline]
  pub fn try_to_short_star(self) -> Option<Bytecode> {
    try_to_short_star(Bytecode::Star0 as u8, self.index)
  }

  #[inline]
  pub const fn size_of_operand(self) -> OperandSize {
    size_of_operand(self.to_operand())
  }
}

/// 连续寄存器列表
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RegisterList {
  first_reg_index: i32,
  register_count: usize,
}

impl RegisterList {
  #[inline]
  pub const fn new() -> Self {
    Self {
      first_reg_index: INVALID_INDEX,
      register_count: 0,
    }
  }

  #[inline]
  pub const fn single(reg: Register) -> Self {
    Self {
      first_reg_index: reg.index(),
      register_count: 1,
    }
  }

  #[inline]
  pub const fn from_range(first_reg: Register, count: usize) -> Self {
    Self {
      first_reg_index: first_reg.index(),
      register_count: count,
    }
  }

  #[inline]
  pub const fn register_count(&self) -> usize {
    self.register_count
  }

  #[inline]
  pub const fn is_empty(&self) -> bool {
    self.register_count == 0
  }

  #[inline]
  pub const fn first_register(&self) -> Register {
    if self.register_count == 0 {
      Register::new(0)
    } else {
      Register::new(self.first_reg_index)
    }
  }

  #[inline]
  pub const fn last_register(&self) -> Register {
    if self.register_count == 0 {
      Register::new(0)
    } else {
      Register::new(self.first_reg_index + (self.register_count as i32) - 1)
    }
  }

  #[inline]
  pub const fn get(&self, index: usize) -> Option<Register> {
    if index < self.register_count {
      Some(Register::new(self.first_reg_index + (index as i32)))
    } else {
      None
    }
  }

  #[inline]
  pub fn register_at(&self, index: usize) -> error::Result<Register> {
    if index < self.register_count {
      Ok(Register::new(self.first_reg_index + (index as i32)))
    } else {
      Err(Error::InvalidRegisterCount {
        expected: self.register_count,
        got: index,
      })
    }
  }

  #[inline]
  pub fn truncate(&self, new_count: usize) -> error::Result<Self> {
    if new_count <= self.register_count {
      Ok(Self {
        first_reg_index: self.first_reg_index,
        register_count: new_count,
      })
    } else {
      Err(Error::InvalidRegisterCount {
        expected: self.register_count,
        got: new_count,
      })
    }
  }

  #[inline]
  pub fn pop_left(&self) -> error::Result<Self> {
    if self.register_count > 0 {
      Ok(Self {
        first_reg_index: self.first_reg_index + 1,
        register_count: self.register_count - 1,
      })
    } else {
      Err(Error::EmptyRegisterList)
    }
  }
}

/// 原版 class/struct Register 成员声明 (机械 1:1 提取, 43 条)
pub const REGISTER_DECLARATIONS: &[&str] = &[
  "[public/const] constexpr explicit Register(int index = kInvalidIndex) : index_(index)",
  "[public/const] constexpr int index() const",
  "[public/const] constexpr bool is_parameter() const",
  "[public/const] constexpr bool is_valid() const",
  "[public/const] static constexpr Register FromParameterIndex(int index)",
  "[public/const] constexpr int ToParameterIndex() const",
  "[public/const] static constexpr Register receiver()",
  "[public/const] constexpr bool is_receiver() const",
  "[public/const] static constexpr Register invalid_value()",
  "[public/const] static constexpr Register function_closure()",
  "[public/const] constexpr bool is_function_closure() const",
  "[public/const] static constexpr Register current_context()",
  "[public/const] constexpr bool is_current_context() const",
  "[public/const] static constexpr Register bytecode_array()",
  "[public/const] constexpr bool is_bytecode_array() const",
  "[public/const] static constexpr Register bytecode_offset()",
  "[public/const] constexpr bool is_bytecode_offset() const",
  "[public/const] static constexpr Register feedback_vector()",
  "[public/const] constexpr bool is_feedback_vector() const",
  "[public/const] static constexpr Register argument_count()",
  "[public/const] static constexpr Register virtual_accumulator()",
  "[public/const] constexpr OperandSize SizeOfOperand() const",
  "[public/const] constexpr int32_t ToOperand() const",
  "[public/const] static constexpr Register FromOperand(int32_t operand)",
  "[public/const] static constexpr Register FromShortStar(Bytecode bytecode)",
  "[public/const] constexpr std::optional<Bytecode> TryToShortStar() const",
  "[public/method] std::string ToString() const",
  "[public/const] constexpr bool operator==(const Register& other) const",
  "[public/const] constexpr bool operator!=(const Register& other) const",
  "[public/const] constexpr bool operator<(const Register& other) const",
  "[public/const] constexpr bool operator<=(const Register& other) const",
  "[public/const] constexpr bool operator>(const Register& other) const",
  "[public/const] constexpr bool operator>=(const Register& other) const",
  "[private/const] static constexpr int kRegisterFileStartOffset = OffsetFromFPToRegisterIndex(0)",
  "[private/const] static constexpr int kFirstParamRegisterIndex = OffsetFromFPToRegisterIndex(InterpreterFrameConstants::kFirstParamFromFp)",
  "[private/const] static constexpr int kFunctionClosureRegisterIndex = OffsetFromFPToRegisterIndex(StandardFrameConstants::kFunctionOffset)",
  "[private/const] static constexpr int kCurrentContextRegisterIndex = OffsetFromFPToRegisterIndex(StandardFrameConstants::kContextOffset)",
  "[private/const] static constexpr int kBytecodeArrayRegisterIndex = OffsetFromFPToRegisterIndex( InterpreterFrameConstants::kBytecodeArrayFromFp)",
  "[private/const] static constexpr int kBytecodeOffsetRegisterIndex = OffsetFromFPToRegisterIndex( InterpreterFrameConstants::kBytecodeOffsetFromFp)",
  "[private/const] static constexpr int kFeedbackVectorRegisterIndex = OffsetFromFPToRegisterIndex( InterpreterFrameConstants::kFeedbackVectorFromFp)",
  "[private/const] static constexpr int kCallerPCOffsetRegisterIndex = OffsetFromFPToRegisterIndex(InterpreterFrameConstants::kCallerPCOffset)",
  "[private/const] static constexpr int kArgumentCountRegisterIndex = OffsetFromFPToRegisterIndex(InterpreterFrameConstants::kArgCOffset)",
  "[private/field] int index_",
];

/// 原版 class/struct RegisterList 成员声明 (机械 1:1 提取, 12 条)
pub const REGISTER_LIST_DECLARATIONS: &[&str] = &[
  "[public/method] RegisterList() : first_reg_index_(Register::invalid_value().index()), register_count_(0)",
  "[public/method] explicit RegisterList(Register r) : RegisterList(r.index(), 1)",
  "[public/method] const RegisterList Truncate(int new_count)",
  "[public/method] const RegisterList PopLeft() const",
  "[public/method] const Register operator[](size_t i) const",
  "[public/method] const Register first_register() const",
  "[public/method] const Register last_register() const",
  "[public/method] int register_count() const",
  "[private/method] RegisterList(int first_reg_index, int register_count) : first_reg_index_(first_reg_index), register_count_(register_count)",
  "[private/method] void IncrementRegisterCount()",
  "[private/field] int first_reg_index_",
  "[private/field] int register_count_",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(REGISTER_DECLARATIONS.len() == 43);
  assert!(REGISTER_LIST_DECLARATIONS.len() == 12);
  assert!(INVALID_INDEX == i32::MAX);
  assert!(size_of_operand(0) as u8 == OperandSize::Byte as u8);
  assert!(size_of_operand(1000) as u8 == OperandSize::Short as u8);
  assert!(size_of_operand(100000) as u8 == OperandSize::Quad as u8);
};
