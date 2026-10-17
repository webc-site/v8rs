// 由 tool/src/baseline/bytecode-offset-iterator.h.js 从 v8/src/baseline/bytecode-offset-iterator.{h,cc} 生成; 修订就地改, 复跑覆盖

//! PC -> 字节码偏移映射表迭代器 (VLQ 无符号差分编码; 语义修订阶段接入 vlq 工具)

/// 原版 class/struct BytecodeOffsetIterator 成员声明 (机械 1:1 提取, 25 条)
pub const BYTECODE_OFFSET_ITERATOR_DECLARATIONS: &[&str] = &[
  "[public/method] explicit BytecodeOffsetIterator(Handle<TrustedByteArray> mapping_table, Handle<BytecodeArray> bytecodes)",
  "[public/method] explicit BytecodeOffsetIterator(Tagged<TrustedByteArray> mapping_table, Tagged<BytecodeArray> bytecodes)",
  "[public/method] ~BytecodeOffsetIterator()",
  "[public/method] inline void Advance()",
  "[public/method] inline void AdvanceToBytecodeOffset(int bytecode_offset)",
  "[public/method] inline void AdvanceToPCOffset(Address pc_offset)",
  "[public/method] inline bool done() const",
  "[public/method] inline Address current_pc_start_offset() const",
  "[public/method] inline Address current_pc_end_offset() const",
  "[public/method] inline int current_bytecode_offset() const",
  "[public/method] static void UpdatePointersCallback(void* iterator)",
  "[public/method] void UpdatePointers()",
  "[private/method] void Initialize()",
  "[private/method] inline uint32_t ReadPosition()",
  "[private/field] Handle<TrustedByteArray> mapping_table_",
  "[private/field] uint8_t* data_start_address_",
  "[private/field] uint32_t data_length_",
  "[private/field] uint32_t current_index_",
  "[private/field] Address current_pc_start_offset_",
  "[private/field] Address current_pc_end_offset_",
  "[private/field] int current_bytecode_offset_",
  "[private/field] Tagged<BytecodeArray> bytecode_handle_storage_",
  "[private/field] interpreter::BytecodeArrayIterator bytecode_iterator_",
  "[private/field] LocalHeap* local_heap_",
  "[private/field] std::optional<DisallowGarbageCollection> no_gc_",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(BYTECODE_OFFSET_ITERATOR_DECLARATIONS.len() == 25);
};

/// 字节码与 PC 映射条目
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BytecodeOffsetEntry {
  pub bytecode_offset: usize,
  pub pc_start_offset: usize,
  pub pc_end_offset: usize,
}

/// 字节码与 PC 映射表迭代器 (解码由 BytecodeOffsetTableBuilder 生成的 LEB128 序列)
#[derive(Debug, Clone)]
pub struct BytecodeOffsetIterator<'a> {
  mapping_table: &'a [u8],
  cursor: usize,
  current_pc_start_offset: usize,
  current_pc_end_offset: usize,
  current_bytecode_offset: usize,
  done: bool,
}

impl<'a> BytecodeOffsetIterator<'a> {
  #[inline]
  pub fn new(mapping_table: &'a [u8]) -> Self {
    let mut iter = Self {
      mapping_table,
      cursor: 0,
      current_pc_start_offset: 0,
      current_pc_end_offset: 0,
      current_bytecode_offset: 0,
      done: false,
    };
    iter.advance();
    iter
  }

  #[inline]
  #[must_use]
  pub const fn done(&self) -> bool {
    self.done
  }

  #[inline]
  #[must_use]
  pub const fn current_pc_start_offset(&self) -> usize {
    self.current_pc_start_offset
  }

  #[inline]
  #[must_use]
  pub const fn current_pc_end_offset(&self) -> usize {
    self.current_pc_end_offset
  }

  #[inline]
  #[must_use]
  pub const fn current_bytecode_offset(&self) -> usize {
    self.current_bytecode_offset
  }

  /// 读取一个变长无符号整数
  fn read_position(&mut self) -> Option<usize> {
    if self.cursor >= self.mapping_table.len() {
      return None;
    }
    let mut result: usize = 0;
    let mut shift: u32 = 0;
    while self.cursor < self.mapping_table.len() {
      let byte = self.mapping_table[self.cursor];
      self.cursor += 1;
      if shift < usize::BITS {
        result |= ((byte & 0x7f) as usize) << shift;
      }
      if (byte & 0x80) == 0 {
        return Some(result);
      }
      shift += 7;
    }
    Some(result)
  }

  /// 前进到下一个字节码映射项
  pub fn advance(&mut self) {
    if self.done {
      return;
    }
    if let Some(delta) = self.read_position() {
      self.current_pc_start_offset = self.current_pc_end_offset;
      self.current_pc_end_offset += delta;
      self.current_bytecode_offset += 1;
    } else {
      self.current_pc_start_offset = self.current_pc_end_offset;
      self.done = true;
    }
  }

  /// 前进到指定的字节码偏移
  pub fn advance_to_bytecode_offset(&mut self, target_offset: usize) {
    while !self.done() && self.current_bytecode_offset < target_offset {
      self.advance();
    }
  }

  /// 前进到指定的 PC 偏移
  pub fn advance_to_pc_offset(&mut self, target_pc: usize) {
    while !self.done() && self.current_pc_end_offset < target_pc {
      self.advance();
    }
  }
}

impl<'a> Iterator for BytecodeOffsetIterator<'a> {
  type Item = BytecodeOffsetEntry;

  fn next(&mut self) -> Option<Self::Item> {
    if self.done {
      None
    } else {
      let entry = BytecodeOffsetEntry {
        bytecode_offset: self.current_bytecode_offset,
        pc_start_offset: self.current_pc_start_offset,
        pc_end_offset: self.current_pc_end_offset,
      };
      self.advance();
      Some(entry)
    }
  }
}
