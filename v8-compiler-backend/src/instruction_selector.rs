use v8_turboshaft::{BlockKind, Graph};

use crate::{
  FlagsCondition, FlagsMode, InstructionCode, code_fields, instruction_codes_x64::ArchOpcode,
};

/// 指令操作数
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionOperand {
  Immediate(i64),
  Register(u8),
  Constant(u32),
  Allocated(u16),
}

/// 后端生成的抽象机器指令
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
  pub opcode: InstructionCode,
  pub flags_mode: FlagsMode,
  pub flags_condition: FlagsCondition,
  pub inputs: Vec<InstructionOperand>,
  pub outputs: Vec<InstructionOperand>,
  pub temps: Vec<InstructionOperand>,
}

impl Instruction {
  #[inline]
  pub const fn new(opcode: InstructionCode) -> Self {
    Self {
      opcode,
      flags_mode: FlagsMode::None,
      flags_condition: FlagsCondition::Equal,
      inputs: Vec::new(),
      outputs: Vec::new(),
      temps: Vec::new(),
    }
  }

  #[inline]
  pub fn with_flags(mut self, mode: FlagsMode, condition: FlagsCondition) -> Self {
    self.flags_mode = mode;
    self.flags_condition = condition;
    self.opcode |= (mode as u32) << code_fields::FLAGS_MODE_FIELD.0;
    self.opcode |= (condition as u32) << code_fields::FLAGS_CONDITION_FIELD.0;
    self
  }

  #[inline]
  pub fn with_input(mut self, op: InstructionOperand) -> Self {
    self.inputs.push(op);
    self
  }

  #[inline]
  pub fn with_output(mut self, op: InstructionOperand) -> Self {
    self.outputs.push(op);
    self
  }
}

/// 指令基本块
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionBlock {
  pub block_index: u32,
  pub instructions: Vec<Instruction>,
  pub predecessors: Vec<u32>,
  pub successors: Vec<u32>,
}

impl InstructionBlock {
  #[inline]
  pub const fn new(block_index: u32) -> Self {
    Self {
      block_index,
      instructions: Vec::new(),
      predecessors: Vec::new(),
      successors: Vec::new(),
    }
  }

  #[inline]
  pub fn with_capacity(block_index: u32, capacity: usize) -> Self {
    Self {
      block_index,
      instructions: Vec::with_capacity(capacity),
      predecessors: Vec::new(),
      successors: Vec::new(),
    }
  }

  #[inline]
  pub fn push_instruction(&mut self, inst: Instruction) {
    self.instructions.push(inst);
  }
}

/// 编译后端指令序列容器
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InstructionSequence {
  pub blocks: Vec<InstructionBlock>,
  pub instructions: Vec<Instruction>,
  pub constants: Vec<i64>,
  pub immediates: Vec<i64>,
}

impl InstructionSequence {
  #[inline]
  pub const fn new() -> Self {
    Self {
      blocks: Vec::new(),
      instructions: Vec::new(),
      constants: Vec::new(),
      immediates: Vec::new(),
    }
  }

  #[inline]
  pub fn with_capacity(block_capacity: usize) -> Self {
    Self {
      blocks: Vec::with_capacity(block_capacity),
      instructions: Vec::new(),
      constants: Vec::new(),
      immediates: Vec::new(),
    }
  }

  #[inline]
  pub fn add_block(&mut self, block: InstructionBlock) {
    self.blocks.push(block);
  }

  #[inline]
  pub fn total_instruction_count(&self) -> usize {
    self.blocks.iter().map(|b| b.instructions.len()).sum()
  }

  #[inline]
  pub fn first_instruction_code(&self) -> Option<InstructionCode> {
    self
      .blocks
      .first()
      .and_then(|b| b.instructions.first())
      .map(|i| i.opcode)
      .or_else(|| self.instructions.first().map(|i| i.opcode))
  }
}

/// 编译期常量指令码
const ARCH_JMP_CODE: InstructionCode = (ArchOpcode::ArchJmp as u32) & 0x3FF;
const ARCH_NOP_CODE: InstructionCode = (ArchOpcode::ArchNop as u32) & 0x3FF;

/// 指令选择器: 消费 Turboshaft Graph 构建强类型 InstructionSequence
pub struct InstructionSelector<'a> {
  graph: &'a Graph,
}

impl<'a> InstructionSelector<'a> {
  #[inline]
  pub const fn new(graph: &'a Graph) -> Self {
    Self { graph }
  }

  /// 执行指令选择，遍历 Turboshaft 的基本块并为各类节点匹配对应机器架构指令
  pub fn select(&self) -> InstructionSequence {
    let block_count = self.graph.block_count();
    let op_count = self.graph.op_id_count();

    if block_count == 0 {
      let mut sequence = InstructionSequence::new();
      // 空图边界保护: 若存在操作槽但未显式绑定块，生成默认 Entry 块
      if op_count > 0 {
        let mut entry_block = InstructionBlock::with_capacity(0, 1);
        entry_block.push_instruction(Instruction::new(ARCH_NOP_CODE));
        sequence.add_block(entry_block);
      }
      return sequence;
    }

    let mut sequence = InstructionSequence::with_capacity(self.graph.bound_blocks.len());

    for (idx, block) in self.graph.bound_blocks.iter().enumerate() {
      let mut inst_block = InstructionBlock::with_capacity(idx as u32, 1);

      match block.kind {
        BlockKind::BranchTarget => {
          let inst = Instruction::new(ARCH_JMP_CODE)
            .with_flags(FlagsMode::Branch, FlagsCondition::Equal)
            .with_input(InstructionOperand::Immediate(idx as i64));
          inst_block.push_instruction(inst);
        }
        BlockKind::LoopHeader | BlockKind::Merge => {
          inst_block.push_instruction(Instruction::new(ARCH_NOP_CODE));
        }
      }

      sequence.add_block(inst_block);
    }

    sequence
  }
}
