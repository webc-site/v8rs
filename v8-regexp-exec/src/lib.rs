#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
pub use error::{Error, Result};

pub mod experimental;
pub mod regexp_interpreter;
pub mod regexp_result_vector;
pub mod regexp_stack;
pub mod regexp_utils;
