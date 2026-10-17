// 由 tool/src/torque/generators.js 从 v8/src/torque/torque-code-generator.h 生成; 修订就地改, 复跑覆盖
// 代码生成器基类 (公共字段 + 指令发射骨架)

use std::collections::BTreeMap;

use crate::{
  error::{Error, Result},
  instructions::{BlockId, DefinitionLocation, Instruction},
  source_positions::SourcePosition,
  utils::Stack,
};

// C++ 持 const ControlFlowGraph& + ostream* → Rust: cfg 经方法传入, 输出走 String
// 缓冲 (out/out_decls 合并由调用方拆分, 登记偏差)。
/// 原版 TorqueCodeGenerator 基类字段 1:1
#[derive(Debug, Default)]
pub struct TorqueCodeGenerator {
  /// 原版 out_ (生成代码缓冲)
  pub out: String,
  /// 原版 out_decls_ (声明区缓冲)
  pub out_decls: String,
  pub fresh_id: usize,
  pub previous_position: SourcePosition,
  pub location_map: BTreeMap<DefinitionLocation, String>,
}

impl TorqueCodeGenerator {
  /// 原版 DefinitionToVariable
  #[must_use]
  pub fn definition_to_variable(&self, location: &DefinitionLocation) -> Option<&str> {
    // 原版: phi → phi_bb{id}_{index}; 其余查表
    if location.is_phi() {
      return None; // TODO(填体): phi 命名需 cfg 块 id
    }
    self.location_map.get(location).map(String::as_str)
  }

  /// 原版 SetDefinitionVariable
  pub fn set_definition_variable(&mut self, definition: DefinitionLocation, str_: String) {
    self.location_map.insert(definition, str_);
  }

  /// 原版 FreshNodeName
  #[must_use]
  pub fn fresh_node_name(&mut self) -> String {
    let n = self.fresh_id;
    self.fresh_id += 1;
    format!("tmp{n}")
  }

  /// 原版 FreshCatchName
  #[must_use]
  pub fn fresh_catch_name(&mut self) -> String {
    let n = self.fresh_id;
    self.fresh_id += 1;
    format!("catch{n}")
  }

  /// 原版 FreshLabelName
  #[must_use]
  pub fn fresh_label_name(&mut self) -> String {
    let n = self.fresh_id;
    self.fresh_id += 1;
    format!("label{n}")
  }

  /// 原版 BlockName
  #[must_use]
  pub fn block_name(block: BlockId) -> String {
    format!("block{block}")
  }

  /// 原版 IsEmptyInstruction (静态)
  #[must_use]
  pub fn is_empty_instruction(instruction: &Instruction) -> bool {
    // TODO(填体)
    let _ = instruction;
    false
  }

  /// 原版 EmitInstruction (后端无关部分直接发射, 后端相关虚分派)
  pub fn emit_instruction(
    &mut self,
    instruction: &Instruction,
    stack: &mut Stack<String>,
  ) -> Result<()> {
    // TODO(填体): 后端无关 Peek/Poke/DeleteRange 直发; 其余转后端
    let _ = (instruction, stack);
    Err(Error::Unimplemented)
  }

  /// 原版 EmitSourcePosition (纯虚 → 后端实现)
  pub fn emit_source_position(&mut self, pos: SourcePosition, always_emit: bool) -> Result<()> {
    // TODO(填体)
    let _ = (pos, always_emit);
    Err(Error::Unimplemented)
  }
}

// ── 原版 src/torque/torque-code-generator.cc ──
// L13: bool TorqueCodeGenerator::IsEmptyInstruction(const Instruction& instruction)
// L27: void TorqueCodeGenerator::EmitInstruction(const Instruction& instruction, Stack<std::string>* stack)
// L45: void TorqueCodeGenerator::EmitInstruction(const PeekInstruction& instruction, Stack<std::string>* stack)
// L50: void TorqueCodeGenerator::EmitInstruction(const PokeInstruction& instruction, Stack<std::string>* stack)
// L56: void TorqueCodeGenerator::EmitInstruction( const DeleteRangeInstruction& instruction, Stack<std::string>* stack)
