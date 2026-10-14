#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
mod lazy_compile_dispatcher;
mod optimizing_compile_dispatcher;

pub use error::*;
pub use lazy_compile_dispatcher::*;
pub use optimizing_compile_dispatcher::*;
