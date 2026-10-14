// 由 tool/src/interpreter/bytecode-array-iterator.h.js 从 v8/src/interpreter/bytecode-array-iterator.{h,cc} 生成; 修订就地改, 复跑覆盖

use crate::{
  bytecode_operands::{OperandScale, OperandSize, OperandType},
  bytecode_register::{Register, RegisterList},
  bytecodes::{Bytecode, Bytecodes},
  error,
  error::Error,
};

/// 跳转表偏移项 (原版 struct JumpTableTargetOffset)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JumpTableTargetOffset {
  /// case 值
  pub case_value: i32,
  /// 目标字节码偏移
  pub target_offset: i32,
}

/// 原版 class/struct JumpTableTargetOffset 成员声明 (机械 1:1 提取, 2 条)
pub const JUMP_TABLE_TARGET_OFFSET_DECLARATIONS: &[&str] = &[
  "[private/field] int case_value",
  "[private/field] int target_offset",
];

/// 原版 class/struct JumpTableTargetOffsets 成员声明 (机械 1:1 提取, 9 条)
pub const JUMP_TABLE_TARGET_OFFSETS_DECLARATIONS: &[&str] = &[
  "[public/field] class V8_EXPORT_PRIVATE iterator final",
  "[public/method] JumpTableTargetOffsets(const BytecodeArrayIterator* iterator, int table_start, int table_size, int case_value_base)",
  "[public/method] iterator begin() const",
  "[public/method] iterator end() const",
  "[public/method] int size() const",
  "[private/field] const BytecodeArrayIterator* iterator_",
  "[private/field] int table_start_",
  "[private/field] int table_size_",
  "[private/field] int case_value_base_",
];

/// 原版 class/struct iterator 成员声明 (机械 1:1 提取, 10 条)
pub const JUMP_TABLE_OFFSETS_ITERATOR_DECLARATIONS: &[&str] = &[
  "[public/method] iterator(int case_value, int table_offset, int table_end, const BytecodeArrayIterator* iterator)",
  "[public/method] JumpTableTargetOffset operator*()",
  "[public/method] iterator& operator++()",
  "[public/method] bool operator!=(const iterator& other)",
  "[private/method] void UpdateAndAdvanceToValid()",
  "[private/field] const BytecodeArrayIterator* iterator_",
  "[private/field] Tagged<Smi> current_",
  "[private/field] int index_",
  "[private/field] int table_offset_",
  "[private/field] int table_end_",
];

/// 原版 class/struct BytecodeArrayIterator 成员声明 (机械 1:1 提取, 75 条)
pub const BYTECODE_ARRAY_ITERATOR_DECLARATIONS: &[&str] = &[
  "[public/method] explicit BytecodeArrayIterator(Handle<BytecodeArray> bytecode_array, int initial_offset = 0)",
  "[public/method] BytecodeArrayIterator(Handle<BytecodeArray> bytecode_array, int initial_offset, DisallowGarbageCollection& no_gc V8_LIFETIME_BOUND)",
  "[public/method] ~BytecodeArrayIterator()",
  "[public/method] BytecodeArrayIterator(const BytecodeArrayIterator&) = delete",
  "[public/method] BytecodeArrayIterator& operator=(const BytecodeArrayIterator&) = delete",
  "[public/method] inline void Advance()",
  "[public/method] void AdvanceTo(int offset)",
  "[public/method] void SetOffset(int offset)",
  "[public/method] void Reset()",
  "[public/method] static bool IsValidOffset(Handle<BytecodeArray> bytecode_array, int offset)",
  "[public/method] static bool IsValidOSREntryOffset(Handle<BytecodeArray> bytecode_array, int offset)",
  "[public/method] bool CurrentBytecodeIsValidOSREntry() const",
  "[public/method] void ApplyDebugBreak()",
  "[public/method] inline Bytecode current_bytecode() const",
  "[public/method] int current_bytecode_size() const",
  "[public/method] int current_bytecode_size_without_prefix() const",
  "[public/method] int current_offset() const",
  "[public/method] int current_operand_offset(int operand_index) const",
  "[public/method] uint8_t* current_address() const",
  "[public/method] int next_offset() const",
  "[public/method] Bytecode next_bytecode() const",
  "[public/method] OperandScale current_operand_scale() const",
  "[public/method] DirectHandle<BytecodeArray> bytecode_array() const",
  "[public/method] uint32_t GetFlag8Operand(int operand_index) const",
  "[public/method] uint32_t GetFlag16Operand(int operand_index) const",
  "[public/method] uint32_t GetUnsignedImmediateOperand(int operand_index) const",
  "[public/method] int32_t GetImmediateOperand(int operand_index) const",
  "[public/method] uint32_t GetConstantPoolIndexOperand(int operand_index) const",
  "[public/method] uint32_t GetFeedbackSlotOperand(int operand_index) const",
  "[public/method] uint32_t GetContextSlotOperand(int operand_index) const",
  "[public/method] uint32_t GetCoverageSlotOperand(int operand_index) const",
  "[public/method] FeedbackSlot GetSlotOperand(int operand_index) const",
  "[public/method] Register GetParameter(int parameter_index) const",
  "[public/method] uint32_t GetRegisterCountOperand(int operand_index) const",
  "[public/method] Register GetRegisterOperand(int operand_index) const",
  "[public/method] Register GetStarTargetRegister() const",
  "[public/method] std::pair<Register, Register> GetRegisterPairOperand(int operand_index) const",
  "[public/method] RegisterList GetRegisterListOperand(int operand_index) const",
  "[public/method] uint32_t GetRegisterOperandRange(int operand_index) const",
  "[public/method] Runtime::FunctionId GetRuntimeIdOperand(int operand_index) const",
  "[public/method] Runtime::FunctionId GetIntrinsicIdOperand(int operand_index) const",
  "[public/method] uint32_t GetNativeContextIndexOperand(int operand_index) const",
  "[public/method] AbortReason GetAbortReasonOperand(int operand_index) const",
  "[public/method] uint8_t GetEmbeddedFeedback(int operand_index) const",
  "[public/method] Tagged<Object> GetConstantAtIndex(int offset) const",
  "[public/method] Handle<Object> GetConstantAtIndex(int offset, Isolate* isolate) const",
  "[public/method] Handle<Object> GetConstantAtIndex(int offset, LocalIsolate* isolate) const",
  "[public/method] Tagged<Smi> GetConstantAtIndexAsSmi(int offset) const",
  "[public/method] Handle<Object> GetConstantForOperand(int operand_index, Isolate* isolate) const",
  "[public/method] Handle<Object> GetConstantForOperand(int operand_index, LocalIsolate* isolate) const",
  "[public/method] int GetRelativeJumpTargetOffset() const",
  "[public/method] int GetJumpTargetOffset() const",
  "[public/method] std::pair<int, int> GetNextJumpOffsets()",
  "[public/method] JumpTableTargetOffsets GetJumpTableTargetOffsets() const",
  "[public/method] int GetAbsoluteOffset(int relative_offset) const",
  "[public/method] std::ostream& PrintCurrentBytecodeTo(std::ostream& os) const",
  "[public/method] static void UpdatePointersCallback(void* iterator)",
  "[public/method] void UpdatePointers()",
  "[public/method] CompareOperationHint GetEmbeddedCompareOperationHint()",
  "[public/method] int GetEmbeddedFeedbackOffset(int operand_index) const",
  "[public/method] inline bool done() const",
  "[public/method] bool operator==(const BytecodeArrayIterator& other) const",
  "[public/method] bool operator!=(const BytecodeArrayIterator& other) const",
  "[protected/method] void SetOffsetUnchecked(int offset)",
  "[private/method] uint32_t GetUnsignedOperand(int operand_index, OperandType operand_type) const",
  "[private/method] int32_t GetSignedOperand(int operand_index, OperandType operand_type) const",
  "[private/method] inline void UpdateCurrentBytecode()",
  "[private/field] Handle<BytecodeArray> bytecode_array_",
  "[private/field] uint8_t* start_",
  "[private/field] uint8_t* end_",
  "[private/field] uint8_t* cursor_",
  "[private/field] Bytecode current_bytecode_",
  "[private/field] OperandScale operand_scale_",
  "[private/field] int prefix_size_",
  "[private/field] LocalHeap* const local_heap_",
];

/// 字节码数组安全迭代器 (对应原版 BytecodeArrayIterator)
#[derive(Debug, Clone)]
pub struct BytecodeArrayIterator<'a> {
  bytes: &'a [u8],
  cursor: usize,
  current_offset: usize,
  current_bytecode: Bytecode,
  operand_scale: OperandScale,
  prefix_size: usize,
  done: bool,
}

impl<'a> BytecodeArrayIterator<'a> {
  pub fn new(bytes: &'a [u8]) -> Self {
    let mut it = Self {
      bytes,
      cursor: 0,
      current_offset: 0,
      current_bytecode: Bytecode::Illegal,
      operand_scale: OperandScale::Single,
      prefix_size: 0,
      done: bytes.is_empty(),
    };
    if !bytes.is_empty() {
      it.update_current_bytecode();
    }
    it
  }

  #[inline]
  pub const fn done(&self) -> bool {
    self.done
  }

  #[inline]
  pub const fn current_bytecode(&self) -> Bytecode {
    self.current_bytecode
  }

  #[inline]
  pub const fn current_offset(&self) -> usize {
    self.current_offset
  }

  #[inline]
  pub const fn current_operand_scale(&self) -> OperandScale {
    self.operand_scale
  }

  #[inline]
  pub fn current_bytecode_size_without_prefix(&self) -> usize {
    self.current_bytecode.size(self.operand_scale)
  }

  #[inline]
  pub fn current_bytecode_size(&self) -> usize {
    self.prefix_size + self.current_bytecode_size_without_prefix()
  }

  #[inline]
  pub fn next_offset(&self) -> usize {
    self.current_offset + self.current_bytecode_size()
  }

  pub fn next_bytecode(&self) -> Bytecode {
    let next_cursor = self.cursor + self.current_bytecode_size_without_prefix();
    if next_cursor >= self.bytes.len() {
      return Bytecode::Illegal;
    }
    let mut byte = self.bytes[next_cursor];
    if byte == (Bytecode::Wide as u8) || byte == (Bytecode::ExtraWide as u8) {
      if next_cursor + 1 < self.bytes.len() {
        byte = self.bytes[next_cursor + 1];
      } else {
        return Bytecode::Illegal;
      }
    }
    Bytecode::from_byte(byte).unwrap_or(Bytecode::Illegal)
  }

  pub fn reset(&mut self) {
    self.cursor = 0;
    self.current_offset = 0;
    self.operand_scale = OperandScale::Single;
    self.prefix_size = 0;
    self.done = self.bytes.is_empty();
    if !self.done {
      self.update_current_bytecode();
    }
  }

  pub fn advance_to(&mut self, offset: usize) -> error::Result<()> {
    if self.current_offset > offset {
      return Err(Error::InvalidBytecodeOffset(offset));
    }
    while !self.done && self.current_offset < offset {
      self.advance();
    }
    if !self.done && self.current_offset == offset {
      Ok(())
    } else {
      Err(Error::InvalidBytecodeOffset(offset))
    }
  }

  pub fn set_offset(&mut self, offset: usize) -> error::Result<()> {
    if self.current_offset > offset {
      self.reset();
    }
    self.advance_to(offset)
  }

  pub fn advance(&mut self) {
    if self.done {
      return;
    }
    let size_without = self.current_bytecode_size_without_prefix();
    self.cursor += size_without;
    if self.cursor >= self.bytes.len() {
      self.done = true;
    } else {
      self.update_current_bytecode();
    }
  }

  fn update_current_bytecode(&mut self) {
    if self.cursor >= self.bytes.len() {
      self.done = true;
      return;
    }
    self.current_offset = self.cursor;
    let mut byte = self.bytes[self.cursor];
    if byte == (Bytecode::Wide as u8) {
      self.operand_scale = OperandScale::Double;
      self.prefix_size = 1;
      self.cursor += 1;
      if self.cursor < self.bytes.len() {
        byte = self.bytes[self.cursor];
      } else {
        self.done = true;
        return;
      }
    } else if byte == (Bytecode::ExtraWide as u8) {
      self.operand_scale = OperandScale::Quadruple;
      self.prefix_size = 1;
      self.cursor += 1;
      if self.cursor < self.bytes.len() {
        byte = self.bytes[self.cursor];
      } else {
        self.done = true;
        return;
      }
    } else {
      self.operand_scale = OperandScale::Single;
      self.prefix_size = 0;
    }

    self.current_bytecode = match Bytecode::from_byte(byte) {
      Some(bc) => bc,
      None => Bytecode::Illegal,
    };
  }

  fn read_operand_bytes(
    &self,
    operand_index: usize,
  ) -> error::Result<(&'a [u8], OperandType, OperandSize)> {
    if self.done {
      return Err(Error::UnexpectedEndOfInput);
    }
    let num_ops = self.current_bytecode.number_of_operands();
    if operand_index >= num_ops {
      return Err(Error::OperandIndexOutOfRange {
        index: operand_index,
        count: num_ops,
      });
    }

    let op_type = self.current_bytecode.get_operand_type(operand_index);
    let op_size = op_type.size(self.operand_scale);
    let op_offset = self
      .current_bytecode
      .get_operand_offset(operand_index, self.operand_scale);
    let abs_start = self.cursor + op_offset;
    let abs_end = abs_start + (op_size as usize);

    if abs_end <= self.bytes.len() {
      Ok((&self.bytes[abs_start..abs_end], op_type, op_size))
    } else {
      Err(Error::UnexpectedEndOfInput)
    }
  }

  pub fn get_unsigned_operand(&self, operand_index: usize) -> error::Result<u32> {
    let (slice, _op_type, op_size) = self.read_operand_bytes(operand_index)?;
    match op_size {
      OperandSize::None => Ok(0),
      OperandSize::Byte => Ok(slice[0] as u32),
      OperandSize::Short => Ok(u16::from_le_bytes([slice[0], slice[1]]) as u32),
      OperandSize::Quad => Ok(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]])),
    }
  }

  pub fn get_signed_operand(&self, operand_index: usize) -> error::Result<i32> {
    let (slice, _op_type, op_size) = self.read_operand_bytes(operand_index)?;
    match op_size {
      OperandSize::None => Ok(0),
      OperandSize::Byte => Ok((slice[0] as i8) as i32),
      OperandSize::Short => Ok((i16::from_le_bytes([slice[0], slice[1]])) as i32),
      OperandSize::Quad => Ok(i32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]])),
    }
  }

  #[inline]
  pub fn get_flag8_operand(&self, operand_index: usize) -> error::Result<u8> {
    Ok(self.get_unsigned_operand(operand_index)? as u8)
  }

  #[inline]
  pub fn get_flag16_operand(&self, operand_index: usize) -> error::Result<u16> {
    Ok(self.get_unsigned_operand(operand_index)? as u16)
  }

  #[inline]
  pub fn get_immediate_operand(&self, operand_index: usize) -> error::Result<i32> {
    self.get_signed_operand(operand_index)
  }

  #[inline]
  pub fn get_unsigned_immediate_operand(&self, operand_index: usize) -> error::Result<u32> {
    self.get_unsigned_operand(operand_index)
  }

  #[inline]
  pub fn get_register_operand(&self, operand_index: usize) -> error::Result<Register> {
    let raw = self.get_signed_operand(operand_index)?;
    Ok(Register::from_operand(raw))
  }

  #[inline]
  pub fn get_register_list_operand(&self, operand_index: usize) -> error::Result<RegisterList> {
    let first = self.get_register_operand(operand_index)?;
    let count = self.get_unsigned_operand(operand_index + 1)? as usize;
    Ok(RegisterList::from_range(first, count))
  }

  #[inline]
  pub fn get_register_pair_operand(
    &self,
    operand_index: usize,
  ) -> error::Result<(Register, Register)> {
    let first = self.get_register_operand(operand_index)?;
    let second = Register::new(first.index() + 1);
    Ok((first, second))
  }

  #[inline]
  pub fn get_star_target_register(&self) -> error::Result<Register> {
    if Bytecodes::is_short_star(self.current_bytecode) {
      Ok(Register::from_short_star(self.current_bytecode))
    } else if self.current_bytecode == Bytecode::Star {
      self.get_register_operand(0)
    } else {
      Err(Error::InvalidBytecode(self.current_bytecode as u8))
    }
  }

  #[inline]
  pub fn get_register_operand_range(&self, operand_index: usize) -> error::Result<u32> {
    let num_ops = self.current_bytecode.number_of_operands();
    if operand_index >= num_ops {
      return Err(Error::OperandIndexOutOfRange {
        index: operand_index,
        count: num_ops,
      });
    }
    let op_type = self.current_bytecode.get_operand_type(operand_index);
    if op_type == OperandType::RegList || op_type == OperandType::RegOutList {
      self.get_unsigned_operand(operand_index + 1)
    } else if let Some(n) = Bytecodes::number_of_registers_represented_by(op_type) {
      Ok(n)
    } else {
      Err(Error::OperandIndexOutOfRange {
        index: operand_index,
        count: num_ops,
      })
    }
  }

  #[inline]
  pub fn get_runtime_id_operand(&self, operand_index: usize) -> error::Result<u32> {
    self.get_unsigned_operand(operand_index)
  }

  #[inline]
  pub fn get_parameter(&self, parameter_index: i32) -> Register {
    Register::from_parameter_index(parameter_index + 1)
  }

  #[inline]
  pub fn get_relative_jump_target_offset(&self) -> error::Result<i32> {
    let raw = self.get_unsigned_immediate_operand(0)? as i32;
    if self.current_bytecode == Bytecode::JumpLoop {
      Ok(-raw)
    } else {
      Ok(raw)
    }
  }

  #[inline]
  pub fn get_absolute_offset(&self, relative_offset: i32) -> error::Result<usize> {
    let target =
      (self.current_offset as isize) + (relative_offset as isize) + (self.prefix_size as isize);
    if target >= 0 {
      Ok(target as usize)
    } else {
      Err(Error::InvalidJumpTarget(target))
    }
  }

  #[inline]
  pub fn get_jump_target_offset(&self) -> error::Result<usize> {
    self.get_absolute_offset(self.get_relative_jump_target_offset()?)
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(JUMP_TABLE_TARGET_OFFSET_DECLARATIONS.len() == 2);
};
