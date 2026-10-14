use std::result;

use thiserror::Error;
use v8_common::MessageTemplate;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
  #[error("Missing argument to template {template:?} (expected {expected}, got {got})")]
  MissingTemplateArgument {
    template: MessageTemplate,
    expected: usize,
    got: usize,
  },
  #[error("Too many arguments to template {template:?} (expected {expected}, got {got})")]
  TooManyTemplateArguments {
    template: MessageTemplate,
    expected: usize,
    got: usize,
  },
  #[error("Missing argument to custom template (expected {expected}, got {got})")]
  MissingCustomTemplateArgument { expected: usize, got: usize },
  #[error("Too many arguments to custom template (expected {expected}, got {got})")]
  TooManyCustomTemplateArguments { expected: usize, got: usize },
}

pub type Result<T> = result::Result<T, Error>;
