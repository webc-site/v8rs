//! JavaScript 纯 Rust 运行时环境与端到端执行器

pub mod ast;
pub mod bytecode_bridge;
pub mod environment;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod value;

pub use bytecode_bridge::try_execute_with_bytecode;
pub use environment::Environment;
pub use interpreter::{Interpreter, run_bytecode, run_js};
pub use value::JsValue;
