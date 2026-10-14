#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
pub use error::{Error, Result};

pub mod elements;
pub mod elements_impl;
pub mod elements_kind;
pub mod fixed_array;
pub mod fixed_array_base;
pub mod pod_array;
pub mod primitive_array;
pub mod struct_;
pub mod trusted;
