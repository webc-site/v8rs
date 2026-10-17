#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
pub use error::{Error, Result};

pub mod js_regexp_string_iterator;
pub mod number_string_cache;
pub mod string;
pub mod string_comparator;
pub mod string_forwarding_table;
pub mod string_set;
pub mod string_table;
