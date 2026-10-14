use std::result;

use thiserror::Error;

use crate::gdb_server::PacketError;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
  #[error("Invalid break location: {0}")]
  InvalidBreakLocation(i32),

  #[error("Debug evaluation failed: {0}")]
  EvaluationFailed(String),

  #[error("GDB protocol packet error: {0:?}")]
  GdbPacketError(PacketError),

  #[error("Debug scope error: {0}")]
  ScopeError(String),

  /// v8: src/objects/module.cc JSModuleNamespace::GetExport — kAccessedUninitializedVariable
  #[error("ReferenceError: {name} is not initialized (TDZ)")]
  AccessedUninitializedVariable { name: String },

  /// v8: src/objects/module.cc JSModuleNamespace::GetPropertyAttributes — kNotDefined
  #[error("ReferenceError: {name} is not defined")]
  NotDefined { name: String },

  /// v8: src/objects/module.cc JSModuleNamespace::DefineOwnProperty — kRedefineDisallowed
  #[error("TypeError: cannot redefine property {key}")]
  RedefineDisallowed { key: String },
}

pub type Result<T> = result::Result<T, Error>;
