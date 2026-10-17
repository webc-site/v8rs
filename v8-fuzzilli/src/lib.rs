#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod cov;
mod error;
mod fuzzilli;

pub use cov::*;
pub use error::{Error, Result};
pub use fuzzilli::*;
