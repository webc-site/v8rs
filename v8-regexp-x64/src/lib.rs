#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]

mod error;
pub use error::{Error, Result};

pub mod frame_layout;
pub use frame_layout::{REGEXP_CODE_SIZE, REGISTERS};
