#![cfg_attr(docsrs, feature(doc_cfg))]

//! rs8-capi: V8 C ABI 兼容接口层
//!
//! 提供 `extern "C"` 符号，实现与 C++ `v8.h` 对应 1:1 的 C ABI 兼容面。

mod error;
pub use error::Error;

pub type Result<T> = aok::Result<T>;

pub mod handles;
pub use handles::*;

pub mod initialization;
pub use initialization::*;

pub mod isolate;
pub use isolate::*;

pub mod context;
pub use context::*;

pub mod value;
pub use value::*;

pub mod platform;
pub use platform::*;

pub mod snapshot;
pub use snapshot::*;

pub mod script;
pub use script::*;
