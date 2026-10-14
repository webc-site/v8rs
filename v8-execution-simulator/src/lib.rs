#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]

pub mod constants;
pub mod error;
pub mod memory;
pub mod registers;
pub mod simulator;

pub use constants::{
  C_FLAG_MASK, DEFAULT_STACK_SIZE, FP_DEFAULT_NAN_BITS, K_END_OF_SIM_ADDRESS, N_FLAG_MASK,
  STACK_ALIGNMENT, V_FLAG_MASK, Z_FLAG_MASK,
};
pub use error::{Error, Result};
pub use memory::SimulatorMemory;
pub use registers::{Nzcv, SimRegisters};
pub use simulator::Simulator;
