#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod ast;
pub mod ast_generated;
pub mod ast_impl;
pub mod char_range;
pub mod error;
pub mod flags;
pub mod flags_generated;
pub mod parser;
pub mod parser_generated;
pub mod regexp_error_generated;
pub mod tables_generated;

pub use ast::{
  Alternative, Assertion, AstPrinter, Atom, BackReference, Capture, CharacterClassStrings,
  CharacterSet, ClassRanges, ClassSetExpression, ClassSetOperand, ClassSetOperation, ClassString,
  Disjunction, Empty, Group, Interval, K_INFINITY, Lookaround, LookaroundType, Quantifier,
  QuantifierType, RegExpTree, StackLimiter, Text, TextElement, TextElementType, Tree, Visitor,
  format_ast,
};
pub use ast_generated::{AssertionType, ClassRangesFlags, StandardCharacterSet, TreeKind};
pub use char_range::CharacterRange;
pub use error::{Error, RegexpError, Result};
pub use flags::{RegExpFlags, flags_to_string, try_flag_from_char};
pub use parser::{CompileData, Parser, parse, parse_utf16};
