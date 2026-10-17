// v8-debug/src/lib.rs
// V8 调试器、作用域、上下文、模块与 GDB 远端协议

#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod call_site_info;
pub mod context_slot;
pub mod contexts;
mod core;
pub mod debug_objects;
mod error;
mod frames;
mod gdb_server;
pub mod module;
mod scopes;
pub mod source_text_module;
pub mod synthetic_module;

pub use core::{
  BreakLocation, BreakReason, CoverageMode, DebugBreakType, DebugScope, DisableBreak,
  EvaluateGlobalMode, ExceptionBreakState, ExceptionBreakType, GetSourceOffsetMode,
  NativeAccessorType, PrivateMemberFilter, StepAction,
};

pub use error::{Error, Result};
pub use frames::{
  Coverage, CoverageBlock, CoverageFunction, CoverageScript, K_NO_SOURCE_POSITION, RedirectMode,
  WasmValueFieldOffsets,
};
pub use gdb_server::{
  GdbPacket, K_TRANSPORT_BUF_SIZE, PacketError, ProcessPacketResult, TargetStatus, WasmAddr,
  WasmAddressType,
};
pub use scopes::{
  K_SCOPE_DETAILS_END_POSITION_INDEX, K_SCOPE_DETAILS_FUNCTION_INDEX, K_SCOPE_DETAILS_NAME_INDEX,
  K_SCOPE_DETAILS_OBJECT_INDEX, K_SCOPE_DETAILS_SIZE, K_SCOPE_DETAILS_START_POSITION_INDEX,
  K_SCOPE_DETAILS_TYPE_INDEX, PropertyIteratorPhase, ScopeIteratorMode, ScopeType, VariableInfo,
};
