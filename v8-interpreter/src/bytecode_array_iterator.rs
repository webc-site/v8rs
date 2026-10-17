// 由 tool/src/interpreter/bytecode-array-iterator.h.js 从 v8/src/interpreter/bytecode-array-iterator.{h,cc} 生成; 修订就地改, 复跑覆盖

use crate::{
  bytecode_operands::{OperandScale, OperandType},
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
    Bytecode::decode(byte)
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

  /// 定位到指定偏移 (原版 SetOffset)
  ///
  /// 原版靠 `Reset + AdvanceTo` 逐条走到目标, 只为保证偏移落在指令边界上; 照搬过来后每次向后
  /// 跳转 (`JumpLoop`) 都要从 0 重扫整段循环体字节码, 循环体越长白费越多 —— 原版生成的汇编是
  /// 直接 `jmp` 到固定偏移的, 并没有这趟扫描。跳转目标出自本仓库的字节码生成器, 天然对齐指令
  /// 边界, 因此这里按原版 `SetOffsetUnchecked` 的做法 O(1) 落位。
  #[inline]
  pub fn set_offset(&mut self, offset: usize) -> error::Result<()> {
    if offset >= self.bytes.len() {
      return Err(Error::InvalidBytecodeOffset(offset));
    }
    self.cursor = offset;
    self.done = false;
    self.update_current_bytecode();
    if self.done {
      return Err(Error::InvalidBytecodeOffset(offset));
    }
    Ok(())
  }

  #[inline(always)]
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

  #[inline(always)]
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

    self.current_bytecode = Bytecode::decode(byte);
  }

  /// 操作数在字节码流里的位置: (起始绝对偏移, 字节宽度)
  ///
  /// `#[inline(always)]`: 表查询 (OPERAND_COUNT / OPERAND_SIZES / OPERAND_OFFSETS) 只有内联进
  /// 分发循环后, LLVM 才能把同一指令内多次取操作数的表读合并成一次
  #[inline(always)]
  fn operand_location(&self, operand_index: usize) -> error::Result<(usize, usize)> {
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
    let width = self.current_bytecode.operand_sizes(self.operand_scale)[operand_index] as usize;
    let offset = self.current_bytecode.operand_offsets(self.operand_scale)[operand_index] as usize;
    let start = self.cursor + offset;
    if start + width > self.bytes.len() {
      return Err(Error::UnexpectedEndOfInput);
    }
    Ok((start, width))
  }

  /// 取操作数所在字节的 4 字节小端窗口 (不足 4 字节时回落到逐字节拼装)
  #[inline(always)]
  fn operand_word(&self, start: usize) -> u32 {
    if start + 4 <= self.bytes.len() {
      let win = &self.bytes[start..start + 4];
      u32::from_le_bytes([win[0], win[1], win[2], win[3]])
    } else {
      let mut word = 0u32;
      for (i, &b) in self.bytes[start..].iter().enumerate() {
        word |= (b as u32) << (i * 8);
      }
      word
    }
  }

  #[inline(always)]
  pub fn get_unsigned_operand(&self, operand_index: usize) -> error::Result<u32> {
    let (start, width) = self.operand_location(operand_index)?;
    let word = self.operand_word(start);
    Ok(match width {
      1 => word & 0xff,
      2 => word & 0xffff,
      4 => word,
      _ => 0,
    })
  }

  #[inline(always)]
  pub fn get_signed_operand(&self, operand_index: usize) -> error::Result<i32> {
    let (start, width) = self.operand_location(operand_index)?;
    let word = self.operand_word(start);
    Ok(match width {
      1 => (word as u8) as i8 as i32,
      2 => (word as u16) as i16 as i32,
      4 => word as i32,
      _ => 0,
    })
  }

  #[inline(always)]
  pub fn get_flag8_operand(&self, operand_index: usize) -> error::Result<u8> {
    Ok(self.get_unsigned_operand(operand_index)? as u8)
  }
  #[inline(always)]
  pub fn get_flag16_operand(&self, operand_index: usize) -> error::Result<u16> {
    Ok(self.get_unsigned_operand(operand_index)? as u16)
  }
  #[inline(always)]
  pub fn get_immediate_operand(&self, operand_index: usize) -> error::Result<i32> {
    self.get_signed_operand(operand_index)
  }
  #[inline(always)]
  pub fn get_unsigned_immediate_operand(&self, operand_index: usize) -> error::Result<u32> {
    self.get_unsigned_operand(operand_index)
  }
  #[inline(always)]
  pub fn get_register_operand(&self, operand_index: usize) -> error::Result<Register> {
    let raw = self.get_signed_operand(operand_index)?;
    Ok(Register::from_operand(raw))
  }
  #[inline(always)]
  pub fn get_register_list_operand(&self, operand_index: usize) -> error::Result<RegisterList> {
    let first = self.get_register_operand(operand_index)?;
    let count = self.get_unsigned_operand(operand_index + 1)? as usize;
    Ok(RegisterList::from_range(first, count))
  }
  #[inline(always)]
  pub fn get_register_pair_operand(
    &self,
    operand_index: usize,
  ) -> error::Result<(Register, Register)> {
    let first = self.get_register_operand(operand_index)?;
    let second = Register::new(first.index() + 1);
    Ok((first, second))
  }
  #[inline(always)]
  pub fn get_star_target_register(&self) -> error::Result<Register> {
    if Bytecodes::is_short_star(self.current_bytecode) {
      Ok(Register::from_short_star(self.current_bytecode))
    } else if self.current_bytecode == Bytecode::Star {
      self.get_register_operand(0)
    } else {
      Err(Error::InvalidBytecode(self.current_bytecode as u8))
    }
  }
  #[inline(always)]
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

  /// `#[inline(always)]`: 采样里它在 Richards 直方图上仍是独立符号 (66 样本), 说明没被内联进
  /// 分发循环; 它全部由表读与小算术组成, 内联后跳转目标直接落进循环的寄存器分配里
  #[inline(always)]
  pub fn get_jump_target_offset(&self) -> error::Result<usize> {
    self.get_absolute_offset(self.get_relative_jump_target_offset()?)
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(JUMP_TABLE_TARGET_OFFSET_DECLARATIONS.len() == 2);
};
