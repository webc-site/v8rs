#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;
pub mod escape_table;
pub mod parser;
pub mod stringifier;
pub mod stringifier_api_generated;
pub mod token;
pub mod value;

pub use error::{Error, Result};
pub use parser::{DEFAULT_MAX_DEPTH, JsonParser};
pub use stringifier::JsonStringifier;
pub use stringifier_api_generated::stringifier_api_skel;
pub use token::{EscapeKind, JsonToken};
pub use value::JsonValue;

/// 解析 JSON 文本为 JsonValue
#[inline]
pub fn parse(source: &str) -> Result<JsonValue> {
  JsonParser::new(source).parse()
}

/// 解析 JSON 文本并指定最大递归深度限制
#[inline]
pub fn parse_with_depth(source: &str, max_depth: usize) -> Result<JsonValue> {
  JsonParser::with_max_depth(source, max_depth).parse()
}

/// 将 JsonValue 序列化为紧凑 JSON 字符串
#[inline]
pub fn stringify(value: &JsonValue) -> Result<String> {
  JsonStringifier::new().stringify(value)
}

/// 将 JsonValue 序列化为带指定缩进的 JSON 字符串
#[inline]
pub fn stringify_indent(value: &JsonValue, indent: &str) -> Result<String> {
  JsonStringifier::with_indent(Some(indent)).stringify(value)
}
