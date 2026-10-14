// 由 tool/src/interpreter/bytecode-node.h.js 从 v8/src/interpreter/bytecode-node.{h,cc} 生成; 修订就地改, 复跑覆盖

use crate::{
  bytecode_operands::OperandScale, bytecode_source_info::BytecodeSourceInfo, bytecodes::Bytecode,
  error, error::Error,
};

/// 单节点操作数容量 (原版 operands_[5], 对齐 Bytecodes::kMaxOperands)
pub const NODE_OPERAND_CAPACITY: usize = 5;

/// 原版 class/struct BytecodeNode 成员声明 (机械 1:1 提取, 25 条)
pub const BYTECODE_NODE_DECLARATIONS: &[&str] = &[
  "[public/method] V8_INLINE BytecodeNode(Bytecode bytecode, BytecodeSourceInfo source_info = BytecodeSourceInfo()) : bytecode_(bytecode), operand_count_(0), operand_scale_(OperandScale::kSingle), source_info_(source_info)",
  "[public/method] V8_INLINE BytecodeNode(Bytecode bytecode, uint32_t operand0, BytecodeSourceInfo source_info = BytecodeSourceInfo()) : bytecode_(bytecode), operand_count_(1), operand_scale_(OperandScale::kSingle), source_info_(source_info)",
  "[public/method] V8_INLINE BytecodeNode(Bytecode bytecode, uint32_t operand0, uint32_t operand1, BytecodeSourceInfo source_info = BytecodeSourceInfo()) : bytecode_(bytecode), operand_count_(2), operand_scale_(OperandScale::kSingle), source_info_(source_info)",
  "[public/method] V8_INLINE BytecodeNode(Bytecode bytecode, uint32_t operand0, uint32_t operand1, uint32_t operand2, BytecodeSourceInfo source_info = BytecodeSourceInfo()) : bytecode_(bytecode), operand_count_(3), operand_scale_(OperandScale::kSingle), source_info_(source_info)",
  "[public/method] V8_INLINE BytecodeNode(Bytecode bytecode, uint32_t operand0, uint32_t operand1, uint32_t operand2, uint32_t operand3, BytecodeSourceInfo source_info = BytecodeSourceInfo()) : bytecode_(bytecode), operand_count_(4), operand_scale_(OperandScale::kSingle), source_info_(source_info)",
  "[public/method] V8_INLINE BytecodeNode(Bytecode bytecode, uint32_t operand0, uint32_t operand1, uint32_t operand2, uint32_t operand3, uint32_t operand4, BytecodeSourceInfo source_info = BytecodeSourceInfo()) : bytecode_(bytecode), operand_count_(5), operand_scale_(OperandScale::kSingle), source_info_(source_info)",
  "[public/method] BYTECODE_LIST(DEFINE_BYTECODE_NODE_CREATOR, DEFINE_BYTECODE_NODE_CREATOR) void Print(std::ostream& os) const",
  "[public/method] Bytecode bytecode() const",
  "[public/method] uint32_t operand(int i) const",
  "[public/method] const uint32_t* operands() const",
  "[public/method] void update_operand0(uint32_t operand0)",
  "[public/method] int operand_count() const",
  "[public/method] OperandScale operand_scale() const",
  "[public/method] const BytecodeSourceInfo& source_info() const",
  "[public/method] void set_source_info(BytecodeSourceInfo source_info)",
  "[public/method] bool operator==(const BytecodeNode& other) const",
  "[public/method] bool operator!=(const BytecodeNode& other) const",
  "[private/method] V8_INLINE BytecodeNode(Bytecode bytecode, int operand_count, OperandScale operand_scale, BytecodeSourceInfo source_info, uint32_t operand0 = 0, uint32_t operand1 = 0, uint32_t operand2 = 0, uint32_t operand3 = 0, uint32_t operand4 = 0) : bytecode_(bytecode), operand_count_(operand_count), operand_scale_(operand_scale), source_info_(source_info)",
  "[private/method] V8_INLINE void UpdateScaleForOperand(int operand_index, uint32_t operand)",
  "[private/method] V8_INLINE void SetOperand(int operand_index, uint32_t operand)",
  "[private/field] Bytecode bytecode_",
  "[private/field] uint32_t operands_[Bytecodes::kMaxOperands]",
  "[private/field] int operand_count_",
  "[private/field] OperandScale operand_scale_",
  "[private/field] BytecodeSourceInfo source_info_",
];

/// 字节码节点 (承载操作码、操作数列表、缩放尺度与源码位置)
#[derive(Debug, Clone, Copy)]
pub struct BytecodeNode {
  bytecode: Bytecode,
  operand_count: usize,
  operand_scale: OperandScale,
  source_info: BytecodeSourceInfo,
  operands: [u32; NODE_OPERAND_CAPACITY],
}

impl PartialEq for BytecodeNode {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    if self.bytecode != other.bytecode || self.source_info != other.source_info {
      return false;
    }
    for i in 0..self.operand_count {
      if self.operands[i] != other.operands[i] {
        return false;
      }
    }
    true
  }
}
impl Eq for BytecodeNode {}

impl Default for BytecodeNode {
  fn default() -> Self {
    Self {
      bytecode: Bytecode::Illegal,
      operand_count: 0,
      operand_scale: OperandScale::Single,
      source_info: BytecodeSourceInfo::none(),
      operands: [0; NODE_OPERAND_CAPACITY],
    }
  }
}

impl BytecodeNode {
  #[inline]
  pub fn new(bytecode: Bytecode, source_info: BytecodeSourceInfo) -> error::Result<Self> {
    let expected = bytecode.number_of_operands();
    if expected != 0 {
      return Err(Error::InvalidOperandCount { expected, got: 0 });
    }
    Ok(Self {
      bytecode,
      operand_count: 0,
      operand_scale: OperandScale::Single,
      source_info,
      operands: [0; NODE_OPERAND_CAPACITY],
    })
  }

  #[inline]
  pub fn with_operands(
    bytecode: Bytecode,
    operands: &[u32],
    source_info: BytecodeSourceInfo,
  ) -> error::Result<Self> {
    let expected = bytecode.number_of_operands();
    if expected != operands.len() {
      return Err(Error::InvalidOperandCount {
        expected,
        got: operands.len(),
      });
    }

    let mut node_operands = [0u32; NODE_OPERAND_CAPACITY];
    let mut scale = OperandScale::Single;

    for (i, &op) in operands.iter().enumerate() {
      node_operands[i] = op;
      let op_type = bytecode.get_operand_type(i);
      let op_scale = op_type.scale_for_operand(op);
      if (op_scale as u8) > (scale as u8) {
        scale = op_scale;
      }
    }

    Ok(Self {
      bytecode,
      operand_count: operands.len(),
      operand_scale: scale,
      source_info,
      operands: node_operands,
    })
  }

  #[inline]
  pub fn with_operands_and_scale(
    bytecode: Bytecode,
    operands: &[u32],
    min_scale: OperandScale,
    source_info: BytecodeSourceInfo,
  ) -> error::Result<Self> {
    let mut node = Self::with_operands(bytecode, operands, source_info)?;
    if (min_scale as u8) > (node.operand_scale as u8) {
      node.operand_scale = min_scale;
    }
    Ok(node)
  }

  #[inline]
  pub const fn bytecode(&self) -> Bytecode {
    self.bytecode
  }

  #[inline]
  pub const fn operand_count(&self) -> usize {
    self.operand_count
  }

  #[inline]
  pub const fn operand_scale(&self) -> OperandScale {
    self.operand_scale
  }

  #[inline]
  pub const fn source_info(&self) -> BytecodeSourceInfo {
    self.source_info
  }

  #[inline]
  pub fn set_source_info(&mut self, source_info: BytecodeSourceInfo) {
    self.source_info = source_info;
  }

  #[inline]
  pub fn operand(&self, index: usize) -> error::Result<u32> {
    if index < self.operand_count {
      Ok(self.operands[index])
    } else {
      Err(Error::OperandIndexOutOfRange {
        index,
        count: self.operand_count,
      })
    }
  }

  #[inline]
  pub fn set_operand(&mut self, index: usize, operand: u32) -> error::Result<()> {
    if index < self.operand_count {
      self.operands[index] = operand;
      let op_type = self.bytecode.get_operand_type(index);
      let op_scale = op_type.scale_for_operand(operand);
      if (op_scale as u8) > (self.operand_scale as u8) {
        self.operand_scale = op_scale;
      }
      Ok(())
    } else {
      Err(Error::OperandIndexOutOfRange {
        index,
        count: self.operand_count,
      })
    }
  }

  #[inline]
  pub fn update_operand0(&mut self, operand0: u32) -> error::Result<()> {
    self.set_operand(0, operand0)
  }

  #[inline]
  pub const fn operands(&self) -> &[u32; NODE_OPERAND_CAPACITY] {
    &self.operands
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(BYTECODE_NODE_DECLARATIONS.len() == 25);
  assert!(NODE_OPERAND_CAPACITY == 5);
};
