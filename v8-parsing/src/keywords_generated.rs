// 由 tool/src/parsing/keywords.js 从 v8/src/parsing/keywords.txt 生成; 修订就地改, 复跑覆盖
// 编译期完美散列关键字映射表

use phf::phf_map;

use crate::token_generated::Token;

/// 关键字完美散列查找表 (原版 PerfectKeywordHash, 共 52 个)
pub static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
  "accessor" => Token::Accessor,
  "async" => Token::Async,
  "await" => Token::Await,
  "break" => Token::Break,
  "case" => Token::Case,
  "catch" => Token::Catch,
  "class" => Token::Class,
  "const" => Token::Const,
  "continue" => Token::Continue,
  "debugger" => Token::Debugger,
  "default" => Token::Default,
  "delete" => Token::Delete,
  "do" => Token::Do,
  "else" => Token::Else,
  "enum" => Token::Enum,
  "export" => Token::Export,
  "extends" => Token::Extends,
  "false" => Token::FalseLiteral,
  "finally" => Token::Finally,
  "for" => Token::For,
  "function" => Token::Function,
  "get" => Token::Get,
  "if" => Token::If,
  "implements" => Token::FutureStrictReservedWord,
  "import" => Token::Import,
  "in" => Token::In,
  "instanceof" => Token::InstanceOf,
  "interface" => Token::FutureStrictReservedWord,
  "let" => Token::Let,
  "new" => Token::New,
  "null" => Token::NullLiteral,
  "of" => Token::Of,
  "package" => Token::FutureStrictReservedWord,
  "private" => Token::FutureStrictReservedWord,
  "protected" => Token::FutureStrictReservedWord,
  "public" => Token::FutureStrictReservedWord,
  "return" => Token::Return,
  "set" => Token::Set,
  "static" => Token::Static,
  "super" => Token::Super,
  "switch" => Token::Switch,
  "this" => Token::This,
  "throw" => Token::Throw,
  "true" => Token::TrueLiteral,
  "try" => Token::Try,
  "typeof" => Token::TypeOf,
  "using" => Token::Using,
  "var" => Token::Var,
  "void" => Token::Void,
  "while" => Token::While,
  "with" => Token::With,
  "yield" => Token::Yield,
};

/// 根据文本查找 Token, 若不是关键字则返回 None
#[inline]
pub fn lookup_keyword(name: &str) -> Option<Token> {
  KEYWORDS.get(name).copied()
}
