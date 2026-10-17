#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]

mod error;
pub use error::{Error, Result};

pub mod frame_layout;
pub use frame_layout::{
  INITIAL_BUFFER_SIZE, NUM_CACHED_REGISTERS, NUM_REGISTERS_TO_UNROLL, REGISTERS,
};
