#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
pub use error::{Error, Result};

mod instruction_codes;
pub use instruction_codes::*;

pub mod instruction_codes_arm64;
pub mod instruction_codes_x64;

mod instruction_selector;
pub use instruction_selector::*;

/// 从 Turboshaft 优化图构建指令选择器并执行选择，返回首条指令的操作码（若无指令则返回 0）
pub fn select_instructions_from_turboshaft(graph: &v8_turboshaft::Graph) -> InstructionCode {
  let selector = InstructionSelector::new(graph);
  let sequence = selector.select();
  sequence.first_instruction_code().unwrap_or(0)
}
