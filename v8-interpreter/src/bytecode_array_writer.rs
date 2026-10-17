// 由 tool/src/interpreter/bytecode-array-writer.h.js 从 v8/src/interpreter/bytecode-array-writer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 字节码写出器与打包常数及真实发射逻辑

use std::mem::take;

use crate::{
  bytecode_label::{BytecodeLabel, BytecodeLoopHeader, INVALID_OFFSET},
  bytecode_node::BytecodeNode,
  bytecode_operands::{OperandScale, OperandSize},
  bytecodes::{Bytecode, MAX_OPERANDS},
};

/// 打包字节码最大尺寸 = 前缀 1 + 操作码 1 + 5 操作数 × 最大操作数尺寸 4 (原版 kMaxSizeOfPackedBytecode 公式)
pub const MAX_SIZE_OF_PACKED_BYTECODE: usize = 2 + MAX_OPERANDS * OperandSize::LAST as usize;

/// 8 位跳转占位 (原版 k8BitJumpPlaceholder)
pub const JUMP_PLACEHOLDER_8BIT: u32 = 0x7f;
/// 16 位跳转占位 (原版 k16BitJumpPlaceholder = 低 8 位拼接高 8 位)
pub const JUMP_PLACEHOLDER_16BIT: u32 = JUMP_PLACEHOLDER_8BIT | (JUMP_PLACEHOLDER_8BIT << 8);
/// 32 位跳转占位 (原版 k32BitJumpPlaceholder 同法)
pub const JUMP_PLACEHOLDER_32BIT: u32 = JUMP_PLACEHOLDER_16BIT | (JUMP_PLACEHOLDER_16BIT << 16);

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct LabelData {
  bound: bool,
  offset: usize,
  referrers: Vec<usize>,
}

/// 字节码真实写出器
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BytecodeArrayWriter {
  bytecodes: Vec<u8>,
  labels: Vec<LabelData>,
}

impl BytecodeArrayWriter {
  #[inline]
  #[must_use]
  pub const fn new() -> Self {
    Self {
      bytecodes: Vec::new(),
      labels: Vec::new(),
    }
  }

  /// 创建新跳转标签
  pub fn new_label(&mut self) -> BytecodeLabel {
    let id = self.labels.len();
    self.labels.push(LabelData {
      bound: false,
      offset: INVALID_OFFSET,
      referrers: Vec::new(),
    });
    BytecodeLabel::with_id(id)
  }

  #[inline]
  fn ensure_label(&mut self, label: BytecodeLabel) {
    while self.labels.len() <= label.id() {
      self.labels.push(LabelData {
        bound: false,
        offset: INVALID_OFFSET,
        referrers: Vec::new(),
      });
    }
  }

  #[inline]
  #[must_use]
  pub fn current_bytecode_size(&self) -> usize {
    self.bytecodes.len()
  }

  #[inline]
  #[must_use]
  pub fn bytecodes(&self) -> &[u8] {
    &self.bytecodes
  }

  #[inline]
  #[must_use]
  pub fn into_bytecodes(self) -> Vec<u8> {
    self.bytecodes
  }

  /// 写入字节码节点
  pub fn write(&mut self, node: &BytecodeNode) {
    let scale = node.operand_scale();
    match scale {
      OperandScale::Double => self.bytecodes.push(Bytecode::Wide as u8),
      OperandScale::Quadruple => self.bytecodes.push(Bytecode::ExtraWide as u8),
      OperandScale::Single => {}
    }

    self.bytecodes.push(node.bytecode() as u8);

    for i in 0..node.operand_count() {
      let op = node.operand(i).unwrap_or(0);
      let op_type = node.bytecode().get_operand_type(i);
      let op_size = op_type.size(scale);
      match op_size {
        OperandSize::None => {}
        OperandSize::Byte => self.bytecodes.push(op as u8),
        OperandSize::Short => self.bytecodes.extend_from_slice(&(op as u16).to_le_bytes()),
        OperandSize::Quad => self.bytecodes.extend_from_slice(&op.to_le_bytes()),
      }
    }
  }

  /// 写入前向跳转 (记录引用位置)
  pub fn write_jump(&mut self, node: &BytecodeNode, label: BytecodeLabel) {
    self.ensure_label(label);
    let jump_location = self.bytecodes.len();
    self.write(node);
    self.labels[label.id()].referrers.push(jump_location);
  }

  /// 写入循环回跳 (JumpLoop)
  pub fn write_jump_loop(&mut self, node: &BytecodeNode, _loop_header: &BytecodeLoopHeader) {
    self.write(node);
  }

  /// 绑定前向跳转标签并回填各跳转指令的操作数
  pub fn bind_label(&mut self, label: BytecodeLabel) {
    self.ensure_label(label);
    let target = self.bytecodes.len();
    self.labels[label.id()].bound = true;
    self.labels[label.id()].offset = target;

    let referrers = take(&mut self.labels[label.id()].referrers);
    for jump_location in referrers {
      let mut prefix_size = 0;
      let mut scale = OperandScale::Single;
      let first_byte = self.bytecodes[jump_location];

      if first_byte == Bytecode::Wide as u8 {
        prefix_size = 1;
        scale = OperandScale::Double;
      } else if first_byte == Bytecode::ExtraWide as u8 {
        prefix_size = 1;
        scale = OperandScale::Quadruple;
      }

      let op_offset = jump_location + prefix_size + 1;
      let relative_delta = target - (jump_location + prefix_size);

      match scale {
        OperandScale::Single => {
          self.bytecodes[op_offset] = relative_delta as u8;
        }
        OperandScale::Double => {
          let bytes = (relative_delta as u16).to_le_bytes();
          self.bytecodes[op_offset..op_offset + 2].copy_from_slice(&bytes);
        }
        OperandScale::Quadruple => {
          let bytes = (relative_delta as u32).to_le_bytes();
          self.bytecodes[op_offset..op_offset + 4].copy_from_slice(&bytes);
        }
      }
    }
  }

  /// 绑定循环头
  pub fn bind_loop_header(&mut self, loop_header: &mut BytecodeLoopHeader) {
    loop_header.bind_to(self.bytecodes.len());
  }
}

/// 原版 class BytecodeArrayWriter 成员声明 (机械 1:1 提取, 48 条)
pub const BYTECODE_ARRAY_WRITER_DECLARATIONS: &[&str] = &[
  "[public/method] BytecodeArrayWriter( Zone* zone, ConstantArrayBuilder* constant_array_builder, SourcePositionTableBuilder::RecordingMode source_position_mode)",
  "[public/method] BytecodeArrayWriter(const BytecodeArrayWriter&) = delete",
  "[public/method] BytecodeArrayWriter& operator=(const BytecodeArrayWriter&) = delete",
  "[public/method] void Write(BytecodeNode* node)",
  "[public/method] void WriteJump(BytecodeNode* node, BytecodeLabel* label)",
  "[public/method] void WriteJumpLoop(BytecodeNode* node, BytecodeLoopHeader* loop_header)",
  "[public/method] void WriteSwitch(BytecodeNode* node, BytecodeJumpTable* jump_table)",
  "[public/method] void BindLabel(BytecodeLabel* label)",
  "[public/method] void BindLoopHeader(BytecodeLoopHeader* loop_header)",
  "[public/method] void BindJumpTableEntry(BytecodeJumpTable* jump_table, int case_value)",
  "[public/method] void BindHandlerTarget(HandlerTableBuilder* handler_table_builder, int handler_id)",
  "[public/method] void BindTryRegionStart(HandlerTableBuilder* handler_table_builder, int handler_id)",
  "[public/method] void BindTryRegionEnd(HandlerTableBuilder* handler_table_builder, int handler_id)",
  "[public/method] void PatchJumpTableSize(BytecodeJumpTable* jump_table, int new_size)",
  "[public/method] void SetFunctionEntrySourcePosition(int position)",
  "[public/method] int CheckBytecodeMatches(Handle<BytecodeArray> bytecode)",
  "[public/method] bool RemainderOfBlockIsDead() const",
  "[public/method] size_t current_bytecode_size() const",
  "[private/const] static const size_t kMaxSizeOfPackedBytecode = 2 * sizeof(Bytecode) + Bytecodes::kMaxOperands * static_cast<size_t>(OperandSize::kLast)",
  "[private/field] const uint32_t k8BitJumpPlaceholder = 0x7f",
  "[private/method] const uint32_t k16BitJumpPlaceholder = k8BitJumpPlaceholder | (k8BitJumpPlaceholder << 8)",
  "[private/method] const uint32_t k32BitJumpPlaceholder = k16BitJumpPlaceholder | (k16BitJumpPlaceholder << 16)",
  "[private/method] void PatchJump(size_t jump_target, size_t jump_location)",
  "[private/method] void PatchJumpWith8BitOperand(size_t jump_location, int delta)",
  "[private/method] void PatchJumpWith16BitOperand(size_t jump_location, int delta)",
  "[private/method] void PatchJumpWith32BitOperand(size_t jump_location, int delta)",
  "[private/method] void EmitBytecode(const BytecodeNode* const node)",
  "[private/method] void EmitJump(BytecodeNode* node, BytecodeLabel* label)",
  "[private/method] void EmitJumpLoop(BytecodeNode* node, BytecodeLoopHeader* loop_header)",
  "[private/method] void EmitSwitch(BytecodeNode* node, BytecodeJumpTable* jump_table)",
  "[private/method] void UpdateSourcePositionTable(const BytecodeNode* const node)",
  "[private/method] void UpdateExitSeenInBlock(Bytecode bytecode)",
  "[private/method] void MaybeElideLastBytecode(Bytecode next_bytecode, bool has_source_info)",
  "[private/method] void InvalidateLastBytecode()",
  "[private/method] void StartBasicBlock()",
  "[private/method] ZoneVector<uint8_t>* bytecodes()",
  "[private/method] SourcePositionTableBuilder* source_position_table_builder()",
  "[private/method] ConstantArrayBuilder* constant_array_builder()",
  "[private/method] Zone* zone()",
  "[private/field] ZoneVector<uint8_t> bytecodes_",
  "[private/field] int unbound_jumps_",
  "[private/field] SourcePositionTableBuilder source_position_table_builder_",
  "[private/field] ConstantArrayBuilder* constant_array_builder_",
  "[private/field] Bytecode last_bytecode_",
  "[private/field] size_t last_bytecode_offset_",
  "[private/field] bool last_bytecode_had_source_info_",
  "[private/field] bool elide_noneffectful_bytecodes_",
  "[private/field] bool exit_seen_in_block_",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(BYTECODE_ARRAY_WRITER_DECLARATIONS.len() == 48);
  assert!(MAX_SIZE_OF_PACKED_BYTECODE == 22);
  assert!(JUMP_PLACEHOLDER_8BIT == 0x7f);
  assert!(JUMP_PLACEHOLDER_16BIT == 0x7f7f);
  assert!(JUMP_PLACEHOLDER_32BIT == 0x7f7f7f7f);
};
