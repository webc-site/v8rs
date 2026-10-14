#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod dumpling_manager;
mod error;
mod object_dumping;

pub use dumpling_manager::*;
pub use error::{Error, Result};
pub use object_dumping::*;
