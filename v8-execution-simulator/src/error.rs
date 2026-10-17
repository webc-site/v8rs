use std::result;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
  #[error("Memory access out of bounds at 0x{address:x} (size: {size})")]
  MemoryOutOfBounds { address: u64, size: usize },

  #[error("Unaligned memory access at 0x{address:x} (alignment: {alignment})")]
  UnalignedAccess { address: u64, alignment: usize },

  #[error("Unimplemented instruction 0x{instruction:08x} at PC 0x{pc:x}")]
  UnimplementedInstruction { instruction: u32, pc: u64 },

  #[error("Simulator execution reached max cycle limit ({limit})")]
  CycleLimitExceeded { limit: u64 },

  #[error("Simulator execution halted: {0}")]
  Halted(String),
}

pub type Result<T> = result::Result<T, Error>;
