use std::result;

use thiserror::Error;

/// JSON 解析与序列化错误
#[derive(Error, Debug, PartialEq)]
pub enum Error {
  /// 遇到非预期 Token
  #[error("Unexpected token '{token}' in JSON at position {position}")]
  UnexpectedToken {
    /// 触发错误的字符或描述
    token: String,
    /// 字符在输入中的字节/字符索引位置
    position: usize,
  },

  /// 非预期的输入终止
  #[error("Unexpected end of JSON input")]
  UnexpectedEndOfInput,

  /// 字符串中出现非法控制字符或未闭合
  #[error("Bad control character in string literal in JSON at position {position}")]
  BadControlCharacter {
    /// 发生错误的位置
    position: usize,
  },

  /// 非法 Unicode 转义序列
  #[error("Bad Unicode escape in JSON at position {position}")]
  BadUnicodeEscape {
    /// 发生错误的位置
    position: usize,
  },

  /// 非法转义字符
  #[error("Bad escaped character in JSON at position {position}")]
  BadEscapeCharacter {
    /// 发生错误的位置
    position: usize,
  },

  /// 数字格式错误（如非法前导零、单独的负号、缺少指数等）
  #[error("Unexpected number in JSON at position {position}")]
  UnexpectedNumber {
    /// 发生错误的位置
    position: usize,
  },

  /// 栈深度溢出（嵌套过深）
  #[error("Maximum call stack size exceeded (JSON nesting limit: {limit})")]
  StackOverflow {
    /// 触发限制的层级
    limit: usize,
  },

  /// 序列化时检测到循环引用
  #[error("Converting circular structure to JSON")]
  CircularReference,

  /// 其他自定义格式错误
  #[error("{0}")]
  Custom(String),
}

/// JSON 模块通用 Result
pub type Result<T> = result::Result<T, Error>;
