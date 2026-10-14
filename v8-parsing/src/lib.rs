#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod character_stream;
mod error;
pub mod keywords_generated;
pub mod literal_buffer;
pub mod literal_buffer_generated;
pub mod parsing_misc_generated;
pub mod scanner;
pub mod scanner_cc_generated;
pub mod scanner_generated;
pub mod token_generated;

pub use character_stream::{
  CharacterStream, OneByteCharacterStream, OwnedTwoByteCharacterStream, ScannerStream,
  SourceStream, TwoByteCharacterStream,
};
pub use error::{Error, Result};
pub use keywords_generated::{KEYWORDS, lookup_keyword};
pub use literal_buffer::LiteralBuffer;
pub use literal_buffer_generated::{
  K_GROWTH_FACTOR, K_INITIAL_CAPACITY, K_MAX_GROWTH, new_capacity,
};
pub use parsing_misc_generated::{
  FuncNameInferrer, ImportAttributes, ParseInfo, PendingCompilationErrorHandler, Rewriter,
  parsing_misc_skel,
};
pub use scanner::{
  BookmarkScope, MessageTemplate, Scanner, ScannerFlags, SourcePosition, TokenDesc,
};
pub use scanner_cc_generated::scanner_cc_skel;
pub use scanner_generated::{
  CHARACTER_SCAN_FLAGS, K_CHARACTER_LOOKAHEAD_BUFFER_SIZE, K_END_OF_INPUT, K_MAX_ASCII, Location,
  NumberKind, ONE_CHAR_TOKENS, scan_flags,
};
pub use token_generated::Token;
