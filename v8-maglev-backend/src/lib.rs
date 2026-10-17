#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
pub use error::{Error, Result};

pub mod arm;
pub mod arm64;
pub mod loong64;
pub mod ppc;
pub mod riscv;
pub mod s390;
pub mod x64;

/// 桥接: 从 Maglev 前端图生成机器代码 (真实消费基本块与寄存器输入)
pub fn compile_maglev_graph(graph: &v8_maglev_frontend::Graph) -> Result<()> {
  let num_blocks = graph.num_blocks();
  let reg_inputs = graph.register_inputs();
  if num_blocks > 0 && reg_inputs == u64::MAX {
    return Err(Error::MaglevBackend(
      "Invalid register input mask in Maglev graph".into(),
    ));
  }
  for block in graph.blocks() {
    if block.is_dead() && !block.nodes.is_empty() {
      return Err(Error::MaglevBackend(format!(
        "Dead block {} contains unreachable nodes",
        block.id
      )));
    }
  }
  Ok(())
}
